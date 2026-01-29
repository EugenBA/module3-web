use crate::error::BlogClientError;
use crate::models::models::{
    Response, CreatePostRequest, LoginRequest, RegisterUserRequest, UpdatePostRequest,
};
#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client; // Для нативных платформ

#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request as Client;
//use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub(crate) struct HttpClient {
    client: Client,
    base_url: String,
    token: Arc<RwLock<Option<String>>>,
}

impl HttpClient {
    pub(crate) async fn new(base_url: &str) -> Result<Self, BlogClientError> {
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

    pub(crate) async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token;
    }

    async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    async fn request<T>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T, BlogClientError>
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

    pub(crate) async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Response, BlogClientError> {
        let body = json!(RegisterUserRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        });

        self.request::<Response>(reqwest::Method::POST, "/api/auth/register", Some(body))
            .await
    }

    pub(crate) async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Response, BlogClientError> {
        let body = json!(LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        });

        self.request::<Response>(reqwest::Method::POST, "/api/auth/login", Some(body))
            .await
    }

    pub(crate) async fn create_post(
        &self,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        let body = json!(CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        });

        self.request::<Response>(reqwest::Method::POST, "/api/posts", Some(body))
            .await
    }

    pub(crate) async fn get_post(&self, id: i64) -> Result<Response, BlogClientError> {
        self.request::<Response>(reqwest::Method::GET, &format!("/api/posts/{}", id), None)
            .await
    }

    pub(crate) async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        let body = json!(UpdatePostRequest {
            id,
            title: title.to_string(),
            content: content.to_string(),
        });

        self.request::<Response>(
            reqwest::Method::PUT,
            &format!("/api/posts/{}", id),
            Some(body),
        )
        .await
    }

    pub(crate) async fn delete_post(&self, id: i64) -> Result<Response, BlogClientError> {
        let response = self
            .request::<Response>(
                reqwest::Method::DELETE,
                &format!("/api/posts/{}", id),
                None,
            )
            .await?;

        Ok(response)
    }

    pub(crate) async fn get_posts(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Response, BlogClientError> {
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

        self.request::<Response>(reqwest::Method::GET, &url, None)
            .await
    }
}
