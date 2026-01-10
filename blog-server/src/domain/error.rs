use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogError
{
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Post not found")]
    PostNotFound,
    #[error("Forbidden action")]
    Forbidden
}