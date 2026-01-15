use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::blog_repository::InDbPostRepository;
use crate::data::user_repository::InDbUserRepository;
use crate::domain::error::{BlogError, DomainError};
use crate::domain::post::{CreatePost, UpdatePost};
use crate::domain::user::{LoginUser, RegisterUser, TokenResponse};
use crate::presentation::auth::AuthenticatedUser;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Scope, delete, get, post, put, web, Responder};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tracing::info;

#[derive(Clone)]
pub(crate) struct RequestId(pub String);

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: DateTime<Utc>,
}

pub(crate) fn protected_scope() -> Scope {
    web::scope("")
        .service(create_post)
        .service(update_post)
        .service(delete_post)
}

pub(crate) fn public_scope() -> Scope {
    web::scope("")
        .route("/health", web::get().to(health))
        .route("/api/post/{id}", web::get().to(get_post))
        .service(register)
        .service(login)
}

fn ensure_owner(owner_id: i64, user: &AuthenticatedUser) -> Result<(), DomainError> {
    if owner_id != user.id {
        Err(DomainError::Unauthorized)
    } else {
        Ok(())
    }
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now(),
    })
}

#[post("/api/post")]
async fn create_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    blog: web::Data<BlogService<InDbPostRepository>>,
    payload: web::Json<CreatePost>,
) -> Result<HttpResponse, BlogError> {
    let post = blog
        .create_post(payload.title.clone(), payload.content.clone(), user.id)
        .await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        "post created"
    );

    Ok(HttpResponse::Created().json(post))
}

//#[get("/api/post/{id}")]
async fn get_post(
    req: HttpRequest,
    blog: web::Data<BlogService<InDbPostRepository>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, BlogError> {
    let post = blog.get_post(path.clone()).await?;

    info!(
        request_id = %request_id(&req),
        post_id = %path.into_inner(),
        "post created"
    );

    Ok(HttpResponse::Created().json(post))
}

#[put("/api/post/{id}")]
async fn update_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    blog: web::Data<BlogService<InDbPostRepository>>,
    payload: web::Json<UpdatePost>,
    path: web::Path<i64>,
) -> Result<HttpResponse, BlogError> {
    let post = blog
        .update_post(path.clone(), user.id, payload.clone())
        .await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %path.into_inner(),
        "post update"
    );
    Ok(HttpResponse::Ok().json(post))
}

#[delete("/api/post/{id}")]
async fn delete_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    blog: web::Data<BlogService<InDbPostRepository>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, BlogError> {
    blog.delete_post(path.clone(), user.id).await?;
    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %path.into_inner(),
        "post delete"
    );
    Ok(HttpResponse::NoContent().into())
}

#[post("/api/auth/register")]
async fn register(
    req: HttpRequest,
    auth: web::Data<AuthService<InDbUserRepository>>,
    payload: web::Json<RegisterUser>,
) -> Result<HttpResponse, BlogError> {
    let user = auth.register(payload.clone()).await?;
    info!(
        request_id = %request_id(&req),
        username = %payload.username,
        email = %payload.username,
        "register user"
    );
    Ok(HttpResponse::Created().json(user))
}

#[post("/api/auth/login")]
async fn login(
    req: HttpRequest,
    auth: web::Data<AuthService<InDbUserRepository>>,
    payload: web::Json<LoginUser>,
) -> Result<HttpResponse, BlogError> {
    let jwt = auth.login(payload.clone()).await?;
    info!(
        request_id = %request_id(&req),
        username= payload.username,
        "login user"
    );
    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: jwt,
        username: payload.username.clone(),
    }))
}

fn request_id(req: &HttpRequest) -> String {
    req.extensions()
        .get::<RequestId>()
        .map(|rid| rid.0.clone())
        .unwrap_or_else(|| "unknown".into())
}
