use crate::blog::proto_blog_service_client::ProtoBlogServiceClient;
use crate::blog::*;
use crate::error::BlogClientError;
use crate::models::models::{Response, Post, User};
use std::sync::Arc;
use std::vec;
use tokio::sync::RwLock;
use tonic::{Request, metadata::MetadataValue, transport::Channel};

#[derive(Clone)]
pub(crate) struct GrpcClient {
    client: ProtoBlogServiceClient<Channel>,
    token: Arc<RwLock<Option<String>>>,
}

impl GrpcClient {
    pub(crate) async fn new(addr: &str) -> Result<Self, BlogClientError> {
        let channel = Channel::from_shared(addr.to_string())?.connect().await?;
        let client = ProtoBlogServiceClient::new(channel);
        Ok(Self {
            client,
            token: Arc::new(RwLock::new(None)),
        })
    }

    pub(crate) async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token;
    }

    async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    async fn create_request<T>(&self, message: T) -> Result<Request<T>, BlogClientError> {
        let mut request = Request::new(message);

        if let Some(token) = self.get_token().await {
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
        let username = response.username;

        Ok(Response {post: None, username, token })
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
        let username = response.username;

        Ok(Response { post: None, username, token })
    }

    pub(crate) async fn create_post(
        &self,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        let request = CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        };

        let mut client = self.client.clone();
        let request = self.create_request(request).await?;
        let response = client.create_post(request).await?;
        let response = response.into_inner();
        if let Some(post) = response.post {
            Ok(Response { post: Some(vec![Post::from(post)]), username: "".to_string(),
                token: "".to_string()})
        }
        else {
            Err(BlogClientError::CreatePostError("No post returned".to_string()))
        }
    }

    pub(crate) async fn get_post(&self, id: i64) -> Result<Response, BlogClientError> {
        let request = GetPostRequest { id };

        let mut client = self.client.clone();
        let response = client.get_post(request).await?;
        let response = response.into_inner();
        if let Some(post) = response.post {
            Ok(Response { post: Some(vec![Post::from(post)]), username: "".to_string(), 
                token: "".to_string() })
        }
        else {
            Err(BlogClientError::NotFound("No post returned".to_string()))
        }
    }

    pub(crate) async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Response, BlogClientError> {
        let request = UpdatePostRequest {
            id,
            title: title.to_string(),
            content: content.to_string(),
        };

        let mut client = self.client.clone();
        let request = self.create_request(request).await?;
        let response = client.update_post(request).await?;
        let response = response.into_inner();
        if let Some(post) = response.post {
            Ok(Response { post: Some(vec![Post::from(post)]), username: "".to_string(), 
                token: "".to_string() })
        }
        else {
            Err(BlogClientError::NotFound("No post returned".to_string()))
        }
    }

    pub(crate) async fn delete_post(&self, id: i64) -> Result<Response, BlogClientError> {
        let request = DeletePostRequest { id };

        let mut client = self.client.clone();
        let request = self.create_request(request).await?;
        client.delete_post(request).await?;
        Ok(Response{
            post: None,
            username: "".to_string(),
            token: "".to_string(),
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
        Ok(Response{ post: Some(posts), username: "".to_string(), token: "".to_string() })
    }
}
