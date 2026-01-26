
use crate::error::BlogClientError;
use reqwest::{Client};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::models::{AuthResponse, Post};

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
    token: Arc<RwLock<Option<String>>>,
}

impl HttpClient {
    pub async fn new(base_url: &str) -> Result<Self, BlogClientError> {
        let client = Client::builder()
            .user_agent(format!("blog-client/{}", env!("CARGO_PKG_VERSION")))
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            token: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token;
    }

    async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    async fn request<T>(&self, method: reqwest::Method,
                        path: &str,
                        body: Option<serde_json::Value>) -> Result<T, BlogClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.request(method, &url);

        // Добавляем токен если есть
        if let Some(token) = self.get_token().await {
            request = request.bearer_auth(token);
        }

        // Добавляем тело если нужно
        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let data = response.json::<T>().await?;
            Ok(data)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(BlogClientError::from_http_status(status, error_text))
        }
    }

    pub async fn register(&self, username: &str, email: &str, password: &str) -> Result<AuthResponse, BlogClientError> {
        let body = json!({
            "username": username,
            "email": email,
            "password": password,
        });

        self.request::<AuthResponse>(
            reqwest::Method::POST,
            "/api/auth/register",
            Some(body),
        ).await
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse, BlogClientError> {
        let body = json!({
            "email": email,
            "password": password,
        });

        self.request::<AuthResponse>(
            reqwest::Method::POST,
            "/api/auth/login",
            Some(body),
        ).await
    }

    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        let body = json!({
            "title": title,
            "content": content,
        });

        self.request::<Post>(
            reqwest::Method::POST,
            "/api/posts",
            Some(body),
        ).await
    }

    pub async fn get_post(&self, id: &str) -> Result<Post, BlogClientError> {
        self.request::<Post>(
            reqwest::Method::GET,
            &format!("/api/posts/{}", id),
            None,
        ).await
    }

    pub async fn update_post(&self, id: &str, title: &str, content: &str) -> Result<Post, BlogClientError> {
        let body = json!({
            "title": title,
            "content": content,
        });

        self.request::<Post>(
            reqwest::Method::PUT,
            &format!("/api/posts/{}", id),
            Some(body),
        ).await
    }

    pub async fn delete_post(&self, id: &str) -> Result<(), BlogClientError> {
        let response = self.request::<serde_json::Value>(
            reqwest::Method::DELETE,
            &format!("/api/posts/{}", id),
            None,
        ).await?;

        Ok(())
    }

    pub async fn list_posts(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Post>, BlogClientError> {
        let mut url = "/api/posts".to_string();
        let mut params = vec![];

        if let Some(offset) = offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        self.request::<Vec<Post>>(
            reqwest::Method::GET,
            &url,
            None,
        ).await
    }
}
