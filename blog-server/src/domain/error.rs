use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum DomainError {
    #[error("User not found {0}")]
    UserNotFound(String),
    #[error("User {0} already exists ")]
    UserAlreadyExists(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Post not found")]
    PostNotFound,
    #[error("Forbidden action")]
    Forbidden,
    #[error("Unauthorized user")]
    Unauthorized,
    #[error("Internal error {0}")]
    Internal(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub(crate) enum BlogError {
    #[error("User not found {0}")]
    UserNotFound(String),
    #[error("User {0} already exists ")]
    UserAlreadyExists(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Post not found")]
    PostNotFound,
    #[error("Forbidden action")]
    Forbidden,
    #[error("Unauthorized user")]
    Unauthorized,
    #[error("Internal error {0}")]
    Internal(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("insufficient funds on account {0}")]
    InsufficientFunds(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("validation error: {0}")]
    Validation(String),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl  From<DomainError> for BlogError{
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::UserNotFound(e) => { BlogError::UserNotFound(e)}
            DomainError::UserAlreadyExists(e) => { BlogError::UserAlreadyExists(e)}
            DomainError::InvalidCredentials => { BlogError::InvalidCredentials}
            DomainError::PostNotFound => { BlogError::PostNotFound}
            DomainError::Forbidden => {BlogError::Forbidden}
            DomainError::Unauthorized => {BlogError::Unauthorized}
            DomainError::Internal(e) => { BlogError::Internal(e)}
            DomainError::DatabaseError(e) => { BlogError::DatabaseError(e.to_string())}
        }
    }
}

impl ResponseError for BlogError {
    fn status_code(&self) -> StatusCode {
        match self {
            BlogError::Validation(_) => StatusCode::BAD_REQUEST,
            BlogError::NotFound(_) => StatusCode::NOT_FOUND,
            BlogError::Unauthorized => StatusCode::UNAUTHORIZED,
            BlogError::InsufficientFunds(_) => StatusCode::BAD_REQUEST,
            BlogError::Internal(_) | _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let details = match self {
            BlogError::Validation(msg) => Some(json!({ "message": msg })),
            BlogError::NotFound(resource) => Some(json!({ "resource": resource })),
            BlogError::Unauthorized => None,
            BlogError::InsufficientFunds(account) => {
                Some(json!({ "account_id": account, "reason": "insufficient_funds" }))
            }
            BlogError::Internal(_) | _ => None,
        };
        let body = ErrorBody {
            error: &message,
            details,
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}
