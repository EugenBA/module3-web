use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::blog::proto_blog_service_server::ProtoBlogServiceServer;
use crate::data::blog_repository::InDbPostRepository;
use crate::data::user_repository::InDbUserRepository;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::database;
use crate::infrastructure::jwt::JwtService;
use crate::infrastructure::logging::init_logging;
use crate::presentation::grpc_service::BlogGrpcService;
use crate::presentation::http_handlers;
use crate::presentation::middleware::{JwtAuthMiddleware, RequestIdMiddleware};
use actix_cors::Cors;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpServer, web};
use anyhow::Error;
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, info};

pub(crate) async fn start_server() -> Result<(), Error> {
    init_logging();

    let config = AppConfig::from_env().expect("invalid configuration data base");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    // миграции
    database::run(&pool).await.expect("migrations failed");

    let user_repo = Arc::new(InDbUserRepository::new(pool.clone()));
    let blog_repo = Arc::new(InDbPostRepository::new(pool.clone()));

    let auth_service = AuthService::new(
        Arc::clone(&user_repo),
        JwtService::new(&config.secret.clone()),
    );
    let blog_service = BlogService::new(Arc::clone(&blog_repo));
    let config_data = config.clone();
    let http_handle = start_http_server(
        config_data.clone(),
        blog_service.clone(),
        auth_service.clone(),
    )
    .await?;
    let grpc_handle =
        start_grpc_server(config_data, blog_service.clone(), auth_service.clone()).await?;
    tokio::select! {
        grpc_result = grpc_handle => {
            error!("gRPC server stopped: {:?}", grpc_result);
            if let Err(e) = grpc_result {
                error!("gRPC server error: {}", e);
            }
        }
        http_result = http_handle => {
            error!("HTTP server stopped: {:?}", http_result);
            if let Err(e) = http_result {
                error!("HTTP server error: {}", e);
            }
        }
    }
    Ok(())
}

async fn start_http_server(
    config_data: AppConfig,
    blog_service: BlogService<InDbPostRepository>,
    auth_service: AuthService<InDbUserRepository>,
) -> Result<JoinHandle<()>, Error> {
    let config = config_data.clone();
    let http_server = HttpServer::new(move || {
        let cors = build_cors(&config);
        App::new()
            .wrap(Logger::default())
            .wrap(RequestIdMiddleware)
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("Referrer-Policy", "no-referrer"))
                    .add(("Permissions-Policy", "geolocation=()"))
                    .add(("Cross-Origin-Opener-Policy", "same-origin")),
            )
            .wrap(cors)
            .app_data(web::Data::new(blog_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .service(
                web::scope("/api")
                    // Аутентификация (публичная)
                    .route("/health", web::get().to(http_handlers::health))
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(http_handlers::register))
                            .route("/login", web::post().to(http_handlers::login)),
                    )
                    // Посты: публичные GET, защищенные другие методы
                    .service(
                        web::scope("/posts")
                            // Публичные
                            .route("", web::get().to(http_handlers::get_posts))
                            .route("/{id}", web::get().to(http_handlers::get_post))
                            // Защищенные (с middleware)
                            .service(
                                web::scope("")
                                    .wrap(JwtAuthMiddleware::new(auth_service.keys().clone()))
                                    .route("", web::post().to(http_handlers::create_post))
                                    .route("/{id}", web::put().to(http_handlers::update_post))
                                    .route("/{id}", web::delete().to(http_handlers::delete_post)),
                            ),
                    ),
            )
    })
    .bind((config_data.host.as_str(), config_data.port))?
    .run();
    info!("HTTP server start");
    let handle = tokio::spawn(async move {
        http_server.await.expect("Can't start http server");
    });
    Ok(handle)
}

async fn start_grpc_server(
    config_data: AppConfig,
    blog_service: BlogService<InDbPostRepository>,
    auth_service: AuthService<InDbUserRepository>,
) -> Result<JoinHandle<()>, Error> {
    let grpc_service = BlogGrpcService::new(blog_service, auth_service);

    let grpc_service_server = ProtoBlogServiceServer::new(grpc_service);
    let ip_addr: IpAddr = config_data.host.parse()?;
    let socket_add = SocketAddr::new(ip_addr, config_data.grpc_port);
    let server = tonic::transport::Server::builder()
        .add_service(grpc_service_server)
        .serve(socket_add);
    info!("gRPC server listening on {:?}", socket_add);
    let handle = tokio::spawn(async move {
        if let Err(e) = server.await {
            error!("gRPC server error: {}", e);
        }
    });
    Ok(handle)
}

fn build_cors(config: &AppConfig) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
        ])
        .supports_credentials()
        .max_age(3600);
    for origin in &config.origins {
        cors = cors.allowed_origin(origin);
    }
    cors
}
