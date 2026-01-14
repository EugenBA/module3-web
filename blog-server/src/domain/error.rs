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
