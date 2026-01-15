use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Scope};
use tracing::info;

use crate::domain::error::{BlogError, DomainError};
use crate::presentation::auth::AuthenticatedUser;
use crate::application::blog_service::BlogService;
use crate::data::blog_repository::InDbPostRepository;
use crate::domain::post::{CreatePost, UpdatePost};

#[derive(Clone)]
pub struct RequestId(pub String);

pub fn scope() -> Scope {
    web::scope("")
        .service(create_post)
}

fn ensure_owner(owner_id: i64, user: &AuthenticatedUser) -> Result<(), DomainError> {
    if owner_id != user.id {
        Err(DomainError::Unauthorized)
    } else {
        Ok(())
    }
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

#[get("/api/post/{id}")]
async fn get_post(
    req: HttpRequest,
    blog: web::Data<BlogService<InDbPostRepository>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, BlogError> {
    let post = blog
        .get_post(path.clone())
        .await?;

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

fn request_id(req: &HttpRequest) -> String {
    req.extensions()
        .get::<RequestId>()
        .map(|rid| rid.0.clone())
        .unwrap_or_else(|| "unknown".into())
}

