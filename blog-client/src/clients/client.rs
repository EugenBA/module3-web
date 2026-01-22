// src/client.rs
use crate::{
    BlogClientError, Result,
    models::*,
    http_client::HttpClient,
    grpc_client::GrpcClient,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Тип транспорта для клиента
#[derive(Debug, Clone)]
pub enum Transport {
    /// HTTP транспорт с указанием базового URL
    Http(String),
    /// gRPC транспорт с указанием адреса
    Grpc(String),
}

impl Transport {
    /// Создает HTTP транспорт с указанием базового URL
    pub fn http(base_url: impl Into<String>) -> Self {
        Self::Http(base_url.into())
    }

    /// Создает gRPC транспорт с указанием адреса
    pub fn grpc(addr: impl Into<String>) -> Self {
        Self::Grpc(addr.into())
    }
}

/// Основной клиент Blog API
#[derive(Clone)]
pub struct BlogClient {
    transport: Transport,
    http_client: Option<Arc<HttpClient>>,
    grpc_client: Option<Arc<GrpcClient>>,
    token: Arc<RwLock<Option<String>>>,
}

impl BlogClient {
    /// Создает новый клиент с указанным транспортом
    pub async fn new(transport: Transport) -> Result<Self> {
        match &transport {
            Transport::Http(base_url) => {
                let http_client = HttpClient::new(base_url).await?;
                Ok(Self {
                    transport,
                    http_client: Some(Arc::new(http_client)),
                    grpc_client: None,
                    token: Arc::new(RwLock::new(None)),
                })
            }
            Transport::Grpc(addr) => {
                let grpc_client = GrpcClient::new(addr).await?;
                Ok(Self {
                    transport,
                    http_client: None,
                    grpc_client: Some(Arc::new(grpc_client)),
                    token: Arc::new(RwLock::new(None)),
                })
            }
        }
    }

    /// Создает HTTP клиент с указанным базовым URL
    pub async fn http(base_url: impl Into<String>) -> Result<Self> {
        Self::new(Transport::Http(base_url.into())).await
    }

    /// Создает gRPC клиент с указанным адресом
    pub async fn grpc(addr: impl Into<String>) -> Result<Self> {
        Self::new(Transport::Grpc(addr.into())).await
    }

    /// Устанавливает JWT токен
    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token.clone();

        if let Some(http_client) = &self.http_client {
            http_client.set_token(token.clone()).await;
        }
        if let Some(grpc_client) = &self.grpc_client {
            grpc_client.set_token(token).await;
        }
    }

    /// Возвращает текущий токен
    pub async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    /// Регистрация нового пользователя
    pub async fn register(&self, username: &str, email: &str, password: &str) -> Result<AuthResponse> {
        match self {
            Self { http_client: Some(client), .. } => {
                let response = client.register(username, email, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            Self { grpc_client: Some(client), .. } => {
                let response = client.register(username, email, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// Вход в систему
    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse> {
        match self {
            Self { http_client: Some(client), .. } => {
                let response = client.login(email, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            Self { grpc_client: Some(client), .. } => {
                let response = client.login(email, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// Создание поста
    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post> {
        match self {
            Self { http_client: Some(client), .. } => {
                client.create_post(title, content).await
            }
            Self { grpc_client: Some(client), .. } => {
                client.create_post(title, content).await
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// Получение поста по ID
    pub async fn get_post(&self, id: &str) -> Result<Post> {
        match self {
            Self { http_client: Some(client), .. } => client.get_post(id).await,
            Self { grpc_client: Some(client), .. } => client.get_post(id).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// Обновление поста
    pub async fn update_post(&self, id: &str, title: &str, content: &str) -> Result<Post> {
        match self {
            Self { http_client: Some(client), .. } => {
                client.update_post(id, title, content).await
            }
            Self { grpc_client: Some(client), .. } => {
                client.update_post(id, title, content).await
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// Удаление поста
    pub async fn delete_post(&self, id: &str) -> Result<()> {
        match self {
            Self { http_client: Some(client), .. } => client.delete_post(id).await,
            Self { grpc_client: Some(client), .. } => client.delete_post(id).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// Список постов с пагинацией
    pub async fn list_posts(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Post>> {
        match self {
            Self { http_client: Some(client), .. } => {
                client.list_posts(limit, offset).await
            }
            Self { grpc_client: Some(client), .. } => {
                client.list_posts(limit, offset).await
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    /// Возвращает тип транспорта
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
}

/// Билдер для BlogClient
pub struct BlogClientBuilder {
    transport: Option<Transport>,
    token: Option<String>,
}

impl BlogClientBuilder {
    /// Создает новый билдер
    pub fn new() -> Self {
        Self {
            transport: None,
            token: None,
        }
    }

    /// Устанавливает транспорт
    pub fn transport(mut self, transport: Transport) -> Self {
        self.transport = Some(transport);
        self
    }

    /// Устанавливает HTTP транспорт
    pub fn http(mut self, base_url: impl Into<String>) -> Self {
        self.transport = Some(Transport::Http(base_url.into()));
        self
    }

    /// Устанавливает gRPC транспорт
    pub fn grpc(mut self, addr: impl Into<String>) -> Self {
        self.transport = Some(Transport::Grpc(addr.into()));
        self
    }

    /// Устанавливает токен
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Строит BlogClient
    pub async fn build(self) -> Result<BlogClient> {
        let transport = self.transport
            .ok_or_else(|| BlogClientError::Config("Transport not specified".to_string()))?;

        let client = BlogClient::new(transport).await?;

        if let Some(token) = self.token {
            client.set_token(Some(token)).await;
        }

        Ok(client)
    }
}

impl Default for BlogClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}