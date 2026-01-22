// src/grpc_client.rs
use crate::{BlogClientError, Result, models::*};
use tonic::{transport::Channel, metadata::MetadataValue, Request};
use std::sync::Arc;
use tokio::sync::RwLock;

// Импортируем сгенерированные protobuf типы
use crate::proto::{
    blog_service_client::BlogServiceClient,
    CreateUserRequest as ProtoCreateUserRequest,
    LoginUserRequest as ProtoLoginUserRequest,
    CreatePostRequest as ProtoCreatePostRequest,
    UpdatePostRequest as ProtoUpdatePostRequest,
    DeletePostRequest as ProtoDeletePostRequest,
    GetPostRequest as ProtoGetPostRequest,
    ListPostsRequest as ProtoListPostsRequest,
    User as ProtoUser,
    Post as ProtoPost,
};

#[derive(Clone)]
pub struct GrpcClient {
    client: BlogServiceClient<Channel>,
    token: Arc<RwLock<Option<String>>>,
}

impl GrpcClient {
    pub async fn new(addr: &str) -> Result<Self> {
        let channel = Channel::from_shared(addr.to_string())?
            .connect()
            .await?;

        let client = BlogServiceClient::new(channel);

        Ok(Self {
            client,
            token: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn set_token(&self, token: Option<String>) {
        *self.token.write().await = token;
    }

    async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    async fn create_request<T>(&self, message: T) -> Result<Request<T>> {
        let mut request = Request::new(message);

        if let Some(token) = self.get_token().await {
            let header_value = format!("Bearer {}", token)
                .parse::<MetadataValue<_>>()
                .map_err(|e| BlogClientError::Unknown(e.to_string()))?;
            request.metadata_mut().insert("authorization", header_value);
        }

        Ok(request)
    }

    fn from_proto_user(user: ProtoUser) -> User {
        User {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: chrono::DateTime::from_timestamp(
                user.created_at.unwrap().seconds,
                user.created_at.unwrap().nanos as u32,
            ).unwrap_or_else(|| chrono::Utc::now()),
        }
    }

    fn from_proto_post(post: ProtoPost) -> Post {
        Post {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            created_at: chrono::DateTime::from_timestamp(
                post.created_at.unwrap().seconds,
                post.created_at.unwrap().nanos as u32,
            ).unwrap_or_else(|| chrono::Utc::now()),
            updated_at: post.updated_at.map(|ts| {
                chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
                    .unwrap_or_else(|| chrono::Utc::now())
            }),
        }
    }

    pub async fn register(&self, username: &str, email: &str, password: &str) -> Result<AuthResponse> {
        let request = ProtoCreateUserRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        let mut client = self.client.clone();
        let response = client.create_user(request).await?;
        let response = response.into_inner();

        let token = response.token;
        let user = response.user.unwrap();

        Ok(AuthResponse {
            user: Self::from_proto_user(user),
            token,
        })
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<AuthResponse> {
        let request = ProtoLoginUserRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let mut client = self.client.clone();
        let response = client.login_user(request).await?;
        let response = response.into_inner();

        let token = response.token;
        let user = response.user.unwrap();

        Ok(AuthResponse {
            user: Self::from_proto_user(user),
            token,
        })
    }

    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post> {
        let request = ProtoCreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        };

        let mut client = self.client.clone();
        let request = self.create_request(request).await?;
        let response = client.create_post(request).await?;
        let response = response.into_inner();

        Ok(Self::from_proto_post(response.post.unwrap()))
    }

    pub async fn get_post(&self, id: &str) -> Result<Post> {
        let request = ProtoGetPostRequest {
            id: id.to_string(),
        };

        let mut client = self.client.clone();
        let response = client.get_post(request).await?;
        let response = response.into_inner();

        Ok(Self::from_proto_post(response.post.unwrap()))
    }

    pub async fn update_post(&self, id: &str, title: &str, content: &str) -> Result<Post> {
        let request = ProtoUpdatePostRequest {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
        };

        let mut client = self.client.clone();
        let request = self.create_request(request).await?;
        let response = client.update_post(request).await?;
        let response = response.into_inner();

        Ok(Self::from_proto_post(response.post.unwrap()))
    }

    pub async fn delete_post(&self, id: &str) -> Result<()> {
        let request = ProtoDeletePostRequest {
            id: id.to_string(),
        };

        let mut client = self.client.clone();
        let request = self.create_request(request).await?;
        client.delete_post(request).await?;

        Ok(())
    }

    pub async fn list_posts(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Post>> {
        let request = ProtoListPostsRequest {
            offset: offset.map(|o| o as i32),
            limit: limit.map(|l| l as i32),
        };

        let mut client = self.client.clone();
        let response = client.list_posts(request).await?;
        let response = response.into_inner();

        let posts = response.posts
            .into_iter()
            .map(Self::from_proto_post)
            .collect();

        Ok(posts)
    }
}
