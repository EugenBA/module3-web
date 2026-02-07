//! Модуль для gRPC транспорта
//!
//! Предоставляет функциональность для взаимодействия с бэкэндом по gRPC
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod grpc_client {
use crate::blog::proto_blog_service_client::ProtoBlogServiceClient;
use crate::blog::*;
use crate::error::BlogClientError;
use crate::models::models::{Response, Post};
use tonic::{Request, metadata::MetadataValue, transport::Channel};

#[derive(Clone)]
pub(crate) struct GrpcClient {
    client: ProtoBlogServiceClient<Channel>,
}

impl GrpcClient {
    pub(crate) async fn new(addr: &str) -> Result<Self, BlogClientError> {
        let channel = Channel::from_shared(addr.to_string())?.connect().await?;
        let client = ProtoBlogServiceClient::new(channel);
        Ok(Self {
            client,
        })
    }

    async fn create_request<T>(&self, message: T, token: Option<String>) -> Result<Request<T>, BlogClientError> {
        let mut request = Request::new(message);

        if let Some(token) = token {
            let header_value = format!("Bearer {}", token)
                .parse::<MetadataValue<_>>()
                .map_err(|e| BlogClientError::Unknown(e.to_string()))?;
            request.metadata_mut().insert("authorization", header_value);
        }

        Ok(request)
    }

    pub(crate) async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Response, BlogClientError> {
        let request = RegisterUserRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        let mut client = self.client.clone();
        let response = client.register_user(request).await?;
        let response = response.into_inner();

        let token = response.token;
        let user = response.username;

        Ok(Response { 
            posts: None, 
            id: None, 
            token: Some(token), 
            title: None, 
            user: Some(user), 
            content: None })
    }

    pub(crate) async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Response, BlogClientError> {
        let request = LoginUserRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let mut client = self.client.clone();
        let response = client.login_user(request).await?;
        let response = response.into_inner();

        let token = response.token;
        let user = response.username;

        Ok(Response { posts: None, 
            id: None, 
            token: Some(token), 
            title: None, 
            user: Some(user), 
            content: None })
    }

    pub(crate) async fn create_post(
        &self,
        title: &str,
        content: &str,
        token: Option<String>
    ) -> Result<Response, BlogClientError> {
        let request = CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        };

        let mut client = self.client.clone();
        let request = self.create_request(request, token).await?;
        let response = client.create_post(request).await?;
        let response = response.into_inner();
        if let Some(post) = response.post {
            Ok(Response::from(post))
        } else {
            Err(BlogClientError::CreatePostError("No post returned".to_string()))
        }
    }

    pub(crate) async fn get_post(&self, id: i64) -> Result<Response, BlogClientError> {
        let request = GetPostRequest { id };

        let mut client = self.client.clone();
        let response = client.get_post(request).await?;
        let response = response.into_inner();
        if let Some(post) = response.post {
            Ok(Response::from(post))
        } else {
            Err(BlogClientError::NotFound("No post returned".to_string()))
        }
    }

    pub(crate) async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
        token: Option<String>
    ) -> Result<Response, BlogClientError> {
        let request = UpdatePostRequest {
            id,
            title: title.to_string(),
            content: content.to_string(),
        };

        let mut client = self.client.clone();
        let request = self.create_request(request, token).await?;
        let response = client.update_post(request).await?;
        let response = response.into_inner();
        if let Some(post) = response.post {
            Ok(Response::from(post))
        } else {
            Err(BlogClientError::NotFound("No post returned".to_string()))
        }
    }

    pub(crate) async fn delete_post(&self, id: i64, token: Option<String>) -> Result<Response, BlogClientError> {
        let request = DeletePostRequest { id };

        let mut client = self.client.clone();
        let request = self.create_request(request, token).await?;
        client.delete_post(request).await?;
        Ok(Response {
            posts: None,
            id: Some(id),
            token: None,
            title: None,
            user: None,
            content: None,
        })
    }

    pub(crate) async fn get_posts(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Response, BlogClientError> {
        let request = GetPostsRequest { offset, limit };

        let mut client = self.client.clone();
        let response = client.get_posts(request).await?;
        let response = response.into_inner();

        let posts = response
            .posts
            .into_iter()
            .map(Post::from)
            .collect();
        Ok(Response {
            id: None,
            posts: Some(posts),
            token: None,
            title: None,
            user: None,
            content: None,
        })
    }
}
}
