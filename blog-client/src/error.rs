
#[cfg(not(target_arch = "wasm32"))]
use reqwest::StatusCode;
use thiserror::Error;
#[cfg(not(target_arch = "wasm32"))]
use tonic::Status;
#[cfg(not(target_arch = "wasm32"))]
use tonic::codegen::http::uri::InvalidUri;

#[derive(Error, Debug)]
pub enum BlogClientError {
    #[cfg(not(target_arch = "wasm32"))]
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[cfg(target_arch = "wasm32")]
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] gloo_net::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("gRPC transport error: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("gRPC status error: {0}")]
    GrpcStatus(#[from] Status),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("gRPC status error: {0}")]
    GrpcUriError(#[from] InvalidUri),

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

    #[error("Create post error: {0}")]
    CreatePostError(String),
}

impl BlogClientError {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_http_status(status: StatusCode, message: String) -> Self {
        match status.as_u16() {
            404 => Self::NotFound(message),
            401 | 403 => Self::Unauthorized(message),
            400 => Self::InvalidRequest(message),
            409 => Self::AlreadyExists(message),
            422 => Self::Validation(message),
            _ => Self::Unknown(format!("HTTP {}: {}", status, message)),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_http_status(status: u16, message: String) -> Self {
        match status {
            404 => Self::NotFound(message),
            401 | 403 => Self::Unauthorized(message),
            400 => Self::InvalidRequest(message),
            409 => Self::AlreadyExists(message),
            422 => Self::Validation(message),
            _ => Self::Unknown(format!("HTTP {}: {}", status, message)),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
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
