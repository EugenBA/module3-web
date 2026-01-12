use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User not found {0}")]
    UserNotFound(String),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Post not found")]
    PostNotFound,
    #[error("Forbidden action")]
    Forbidden,
    #[error("Unauthorized user")]
    Unauthorized,
    #[error("Internal error {0}")]
    Internal(String)
}
