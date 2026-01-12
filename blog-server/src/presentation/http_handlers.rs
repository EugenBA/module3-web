use crate::data::user_repository;
use crate::domain::user::{LoginUser, RegisterUser};
use crate::infrastructure::{config::Config, jwt, hash};
use actix_web::{HttpResponse, Responder, get, post, web};
use sqlx::PgPool;
use crate::infrastructure::jwt::JwtService;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status":"ok"}))
}

#[post("/register")]
async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterUser>,
) -> actix_web::Result<impl Responder> {
    let email = body.email.trim().to_lowercase();
    if email.is_empty() || body.password.len() < 6 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "invalid input"
        })));
    }

    let pw_hash = hash::hash_password(&body.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("hash error"))?;

    let user_id = uuid::Uuid::new_v4();

    let res = user_repository::create_user(&pool, user_id, &email, &pw_hash).await;
    match res {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Ok(HttpResponse::Conflict().json(serde_json::json!({"error":"email taken"})))
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("db error")),
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,
    cfg: web::Data<Config>,
    body: web::Json<LoginUser>,
) -> actix_web::Result<impl Responder> {
    let username = body.username.trim().to_lowercase();

    let user = match user_repository::find_user(&pool, &username).await {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(HttpResponse::Unauthorized().finish()),
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("db error")),
    };

    let ok = hash::verify_password(&body.password, &user.password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("verify error"))?;

    if !ok {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let token = JwtService::generate_jwt(&cfg.jwt_secret, user.id)
        .map_err(|_| actix_web::error::ErrorInternalServerError("jwt error"))?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token,
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health).service(register).service(login);
}
