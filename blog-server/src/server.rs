use std::sync::Arc;
use actix_cors::Cors;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::user_repository::InDbUserRepository;
use crate::data::blog_repository::InDbPostRepository;
use crate::infrastructure::config::{Config, JwtConfig, CorsConfig};
use crate::infrastructure::database;
use crate::infrastructure::logging::init_logging;
use crate::infrastructure::jwt::JwtService;
use crate::presentation::http_handlers;
use crate::presentation::middleware::{JwtAuthMiddleware};



pub(crate) async fn start_server() -> std::io::Result<()> {
    init_logging();

    let config = Config::from_env().expect("invalid configuration data base");
    let jwt_config = JwtConfig::from_env().expect("invalid configuration jwt keys");
    let cors_config = CorsConfig::from_env().expect("invalid configuration CORS");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    // миграции
    database::run(&pool).await.expect("migrations failed");

    let user_repo = Arc::new(InDbUserRepository::new(pool.clone()));
    let blog_repo = Arc::new(InDbPostRepository::new(pool.clone()));

    let auth_service = AuthService::new(Arc::clone(&user_repo),
                                        JwtService::new(&jwt_config.secret.clone()));
    let blog_service = BlogService::new(Arc::clone(&blog_repo));

    let config_data = cors_config.clone();

    HttpServer::new(move || {
        let cors = build_cors(&config_data);
        App::new()
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("Referrer-Policy", "no-referrer"))
                .add(("Permissions-Policy", "geolocation=()"))
                .add(("Cross-Origin-Opener-Policy", "same-origin")))
            .wrap(cors)
            .app_data(web::Data::new(blog_service.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .service(
                web::scope("/api")
                    .service(http_handlers::public_scope())
                    .service(
                        web::scope("")
                            .wrap(JwtAuthMiddleware::new(auth_service.keys().clone()))
                            .service(http_handlers::protected_scope())
                    )
            )
    })
        .bind((config.host.as_str(), config.port))?
        .run()
        .await
}

fn build_cors(config: &CorsConfig) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
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