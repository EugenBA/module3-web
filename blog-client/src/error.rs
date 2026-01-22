// src/error.rs
use thiserror::Error;
use reqwest::StatusCode;
use tonic::Status;

#[derive(Error, Debug)]
pub enum BlogClientError {
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("gRPC transport error: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),

    #[error("gRPC status error: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("No transport configured")]
    NoTransportConfigured,

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl BlogClientError {
    pub fn from_http_status(status: StatusCode, message: String) -> Self {
        match status {
            StatusCode::NOT_FOUND => Self::NotFound(message),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Self::Unauthorized(message),
            StatusCode::BAD_REQUEST => Self::InvalidRequest(message),
            StatusCode::CONFLICT => Self::AlreadyExists(message),
            StatusCode::UNPROCESSABLE_ENTITY => Self::Validation(message),
            _ => Self::Unknown(format!("HTTP {}: {}", status, message)),
        }
    }

    pub fn from_grpc_status(status: Status) -> Self {
        match status.code() {
            tonic::Code::NotFound => Self::NotFound(status.message().to_string()),
            tonic::Code::Unauthenticated => Self::Unauthorized(status.message().to_string()),
            tonic::Code::InvalidArgument => Self::InvalidRequest(status.message().to_string()),
            tonic::Code::AlreadyExists => Self::AlreadyExists(status.message().to_string()),
            tonic::Code::FailedPrecondition => Self::Validation(status.message().to_string()),
            _ => Self::GrpcStatus(status),
        }
    }
}

pub type Result<T> = std::result::Result<T, BlogClientError>;