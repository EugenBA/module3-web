//! Модуль для HTTP транспорта
//!
//! Предоставляет функциональность для взаимодействия с бэкэндом по HTTP

use crate::error::BlogClientError;
use crate::models::models::{
    CreatePostRequest, LoginRequest, RegisterUserRequest, Response, UpdatePostRequest,
};

use crate::transports::http_helpers::{
    HttpBuilder, HttpClientRequest, HttpRequest, HttpRequestMethod,
};
use core::time::Duration;
use serde_json::json;

pub(crate) struct HttpClient {
    client: HttpClientRequest,
    base_url: String,
}

impl HttpClient {
    pub(crate) async fn new(base_url: &str, timeout: Duration) -> Result<Self, BlogClientError> {
        let client = HttpClientRequest::new(timeout);
        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    async fn request<T>(
        &self,
        method: HttpRequestMethod,
        path: &str,
        body: Option<serde_json::Value>,
        token: Option<String>,
    ) -> Result<T, BlogClientError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.request(method, &url);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        // Добавляем тело если нужно
        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status = response.status();

        if self.client.status_ok(status) {
            if let Ok(data) = response.json::<T>().await {
                Ok(data)
            } else {
                Ok(T::default())
            }
        } else {
            let error_text = response.text().await?;
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

        self.request::<Response>(
            HttpRequestMethod::POST,
            "/api/auth/register",
            Some(body),
            None,
        )
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

        self.request::<Response>(HttpRequestMethod::POST, "/api/auth/login", Some(body), None)
            .await
    }

    pub(crate) async fn create_post(
        &self,
        title: &str,
        content: &str,
        token: Option<String>,
    ) -> Result<Response, BlogClientError> {
        let body = json!(CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        });

        self.request::<Response>(HttpRequestMethod::POST, "/api/posts", Some(body), token)
            .await
    }

    pub(crate) async fn get_post(&self, id: i64) -> Result<Response, BlogClientError> {
        self.request::<Response>(
            HttpRequestMethod::GET,
            &format!("/api/posts/{}", id),
            None,
            None,
        )
        .await
    }

    pub(crate) async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
        token: Option<String>,
    ) -> Result<Response, BlogClientError> {
        let body = json!(UpdatePostRequest {
            id,
            title: title.to_string(),
            content: content.to_string(),
        });

        self.request::<Response>(
            HttpRequestMethod::PUT,
            &format!("/api/posts/{}", id),
            Some(body),
            token,
        )
        .await
    }

    pub(crate) async fn delete_post(
        &self,
        id: i64,
        token: Option<String>,
    ) -> Result<Response, BlogClientError> {
        let response = self
            .request::<Response>(
                HttpRequestMethod::DELETE,
                &format!("/api/posts/{}", id),
                None,
                token,
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

        self.request::<Response>(HttpRequestMethod::GET, &url, None, None)
            .await
    }
}
