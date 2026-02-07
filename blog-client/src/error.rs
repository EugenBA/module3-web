//! Модуль для реализации обработки ошибок
//!
//! Предоставляет функциональность по обработке ошибок чтения, конвертации и записи.
#[cfg(not(target_arch = "wasm32"))]
use reqwest::StatusCode;
use thiserror::Error;
#[cfg(not(target_arch = "wasm32"))]
use tonic::Status;
#[cfg(not(target_arch = "wasm32"))]
use tonic::codegen::http::uri::InvalidUri;

/// Перчсисление для ошибок
/// Ошибки взаимодействия по HTTP
/// Ошибки взаимодействия по gRPC
/// Ошибки ввода-вывода
#[derive(Error, Debug)]
pub enum BlogClientError {
    /// Ошибка запроса HTTP
    #[cfg(not(target_arch = "wasm32"))]
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),
    /// Ошибка запроса HTTP
    #[cfg(target_arch = "wasm32")]
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] gloo_net::Error),
    /// Ошибка транспорта gRPC
    #[cfg(not(target_arch = "wasm32"))]
    #[error("gRPC transport error: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),
    /// Ошибка статуса gRPC
    #[cfg(not(target_arch = "wasm32"))]
    #[error("gRPC status error: {0}")]
    GrpcStatus(#[from] Status),
    /// Ошибка URI gRPC
    #[cfg(not(target_arch = "wasm32"))]
    #[error("gRPC status error: {0}")]
    GrpcUriError(#[from] InvalidUri),
    /// Ошибка запроса ресурса
    #[error("Resource not found: {0}")]
    NotFound(String),
    /// Ошибка аторизации
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    /// Ошибочный запрос
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    ///Ошибка - ресурс уже существует
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    /// Ошибка валидации
    #[error("Validation error: {0}")]
    Validation(String),
    /// Ошибка конфигурации транспорта
    #[error("No transport configured")]
    NoTransportConfigured,
    /// Ошибка сериализации
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    /// Ошибка ввода/вывода
    #[cfg(not(target_arch = "wasm32"))]
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Ошибка ввода/вывода
    #[cfg(target_arch = "wasm32")]
    #[error("IO error: {0}")]
    Io(#[from] gloo_storage::errors::StorageError),
    /// Ошибка конфигурации
    #[error("Configuration error: {0}")]
    Config(String),
    /// Неисзвестная ошибка
    #[error("Unknown error: {0}")]
    Unknown(String),
    /// Ошибка создания поста
    #[error("Create post error: {0}")]
    CreatePostError(String),
}

impl BlogClientError {
    ///
    ///  Конверитирует возврат статусов клиента в ошибки BlogClientError не для архитектуры wasm32
    ///
    ///  - `404`: Maps to `Self::NotFound`
    ///  - `401` or `403`: Maps to `Self::Unauthorized`
    ///  - `400`: Maps to `Self::InvalidRequest`
    ///  - `409`: Maps to `Self::AlreadyExists`
    ///  - `422`: Maps to `Self::Validation`
    ///
    ///  # Параметры
    ///
    ///  * `status` - HTTP код статуса
    ///  * `message` - описание ошибки
    ///
    ///  # Возврат
    ///
    ///  Инстанс с преобразованной ошибкой
    ///
    ///  # пример
    ///```rust
    ///  let status = StatusCode::NOT_FOUND;
    ///  let message = String::from("Resource not found.");
    ///  let error = EnumType::from_http_status(status, message);
    ///  assert_eq!(error, EnumType::NotFound("Resource not found."));
    ///
    ///
    /// let status = StatusCode::UNAUTHORIZED;
    /// let message = String::from("Access denied.");
    /// let error = EnumType::from_http_status(status, message);
    /// assert_eq!(error, EnumType::Unauthorized("Access denied."));
    /// ```
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
    ///
    /// Конверитирует возврат статусов клиента в ошибки BlogClientError для архитектуры wasm32
    ///
    ///  - `404`: Maps to `Self::NotFound`
    ///  - `401` or `403`: Maps to `Self::Unauthorized`
    ///  - `400`: Maps to `Self::InvalidRequest`
    ///  - `409`: Maps to `Self::AlreadyExists`
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
    ///
    /// Конвертация ошибок gRPC в ошибки BlogClientError
    ///
    /// # Параметры
    /// - `status`: gRPC статус
    ///
    /// # Возврат
    /// Инстанс с преобразованной ошибкой
    ///
    ///
    /// # Пример
    /// ```rust
    /// # use tonic::Status;
    /// # use tonic::Code;
    /// # fn example() {
    /// let grpc_status = Status::new(Code::NotFound, "Resource not found");
    /// let error = ImplementingType::from_grpc_status(grpc_status);
    /// // `error` will be an instance of `ImplementingType::NotFound`
    /// # }
    /// ```
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
/// Алиас для возращаемого типа
pub type Result<T> = std::result::Result<T, BlogClientError>;
