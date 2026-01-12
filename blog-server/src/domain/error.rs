use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
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
