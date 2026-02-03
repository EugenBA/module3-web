use crate::models::models::{Response};
use crate::{error::BlogClientError, transports::http_client::HttpClient};
use core::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(not(target_arch = "wasm32"))]
use crate::transports::grpc_client::grpc_client::GrpcClient;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};
#[cfg(target_arch = "wasm32")]
use crate::models::models::StorageUser;

#[derive(Debug, Clone)]
pub enum Transport {
    /// HTTP транспорт с указанием базового URL
    Http(String),
    /// gRPC транспорт с указанием адреса
    #[cfg(not(target_arch = "wasm32"))]
    Grpc(String),
}

impl Transport {
    /// Создает HTTP транспорт с указанием базового URL
    pub fn http(base_url: impl Into<String>) -> Self {
        Self::Http(base_url.into())
    }

    /// Создает gRPC транспорт с указанием адреса
    #[cfg(not(target_arch = "wasm32"))]
    pub fn grpc(addr: impl Into<String>) -> Self {
        Self::Grpc(addr.into())
    }
}

#[derive(Clone)]
pub struct BlogClient {
    transport: Transport,
    http_client: Option<Arc<HttpClient>>,
    #[cfg(not(target_arch = "wasm32"))]
    grpc_client: Option<Arc<GrpcClient>>,
    token: Arc<RwLock<Option<String>>>,
}

impl BlogClient {
    pub async fn new(transport: Transport, timeout: Duration) -> Result<Self, BlogClientError> {
        match &transport {
            Transport::Http(base_url) => {
                let http_client = HttpClient::new(base_url, timeout).await?;
                Ok(Self {
                    transport,
                    http_client: Some(Arc::new(http_client)),
                    #[cfg(not(target_arch = "wasm32"))]
                    grpc_client: None,
                    token: Arc::new(RwLock::new(None)),
                })
            }
            #[cfg(not(target_arch = "wasm32"))]
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

    pub async fn http(
        base_url: impl Into<String>,
        timeout: Duration,
    ) -> Result<Self, BlogClientError> {
        Self::new(Transport::Http(base_url.into()), timeout).await
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn grpc(addr: impl Into<String>, timeout: Duration) -> Result<Self, BlogClientError> {
        Self::new(Transport::Grpc(addr.into()), timeout).await
    }

    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token.clone();
        if let Some(http_client) = &self.http_client {
            http_client.set_token(token.clone()).await;
            if let Some(token) = token.clone() {
                let _ = self.save_token(&token);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(grpc_client) = &self.grpc_client {
            grpc_client.set_token(token).await;
        }
    }

    pub async fn get_token(&self) -> Option<String> {
        if cfg!(target_arch = "wasm32") {
            self.token.read().await.clone()
        } else {
            self.token.read().await.clone()
        }
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => {
                let response = client.register(username, email, password).await?;
                self.set_token(response.token.clone()).await;
                #[cfg(target_arch = "wasm32")]
                if let Some(id) = response.id {
                    self.save_username(StorageUser {
                        id,
                        username: username.to_string(),
                    })?;
                }
                Ok(response)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => {
                let response = client.register(username, email, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => {
                let response = client.login(username, password).await?;
                self.set_token(response.token.clone()).await;
                #[cfg(target_arch = "wasm32")]
                if let Some(id) = response.id {
                    self.save_username(StorageUser {
                        id,
                        username: username.to_string(),
                    })?;
                }
                Ok(response)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => {
                let response = client.login(username, password).await?;
                self.set_token(Some(response.token.clone())).await;
                Ok(response)
            }
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    pub async fn create_post(
        &self,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.create_post(title, content).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.create_post(title, content).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    pub async fn get_post(&self, id: i64) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.get_post(id).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.get_post(id).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.update_post(id, title, content).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.update_post(id, title, content).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    pub async fn delete_post(&self, id: i64) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.delete_post(id).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.delete_post(id).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    pub async fn list_posts(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Response, BlogClientError> {
        match self {
            Self {
                http_client: Some(client),
                ..
            } => client.get_posts(limit, offset).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self {
                grpc_client: Some(client),
                ..
            } => client.get_posts(limit, offset).await,
            _ => Err(BlogClientError::NoTransportConfigured),
        }
    }

    pub fn transport(&self) -> &Transport {
        &self.transport
    }

    pub async fn load_token(&self) -> Result<(), BlogClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let token_file = ".blog_token";
            if Path::new(token_file).exists() {
                let token = fs::read_to_string(token_file)?;
                self.set_token(Some(token)).await;
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let token = LocalStorage::get("blog_token")?;
            self.set_token(Some(token)).await;
        }
        Ok(())
    }

    pub fn save_token(&self, token: &str) -> Result<(), BlogClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            fs::write(".blog_token", token)?;
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        {
            LocalStorage::set("blog_token", token)?;
            self.set_token(Some(token.to_string()));
            Ok(())
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn save_username(&self, user: StorageUser) -> Result<(), BlogClientError> {
        LocalStorage::set("blog_username", &user)?;
        Ok(())
    }

    pub fn clear_token(&self) -> Result<(), BlogClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let token_file = ".blog_token";
            if Path::new(token_file).exists() {
                fs::remove_file(".blog_token")?;
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            LocalStorage::delete("blog_token");
            LocalStorage::delete("blog_username");
        }
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_user(&self) -> Option<StorageUser> {
        LocalStorage::get("blog_username").ok()
    }
}

pub struct BlogClientBuilder {
    transport: Option<Transport>,
    timeout: Option<Duration>,
    token: Option<String>,
}

impl BlogClientBuilder {
    /// Создает новый билдер
    pub fn new() -> Self {
        Self {
            transport: None,
            timeout: None,
            token: None,
        }
    }

    pub fn transport(mut self, transport: Transport) -> Self {
        self.transport = Some(transport);
        self
    }

    pub fn http(mut self, base_url: impl Into<String>) -> Self {
        self.transport = Some(Transport::Http(base_url.into()));
        self
    }

    /// Устанавливает gRPC транспорт
    #[cfg(not(target_arch = "wasm32"))]
    pub fn grpc(mut self, addr: impl Into<String>) -> Self {
        self.transport = Some(Transport::Grpc(addr.into()));
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub async fn build(self) -> Result<BlogClient, BlogClientError> {
        let transport = self
            .transport
            .ok_or_else(|| BlogClientError::Config("Transport not specified".to_string()))?;
        let timeout = self.timeout.unwrap_or_else(|| Duration::from_secs(30));
        let client = BlogClient::new(transport, timeout).await?;

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
