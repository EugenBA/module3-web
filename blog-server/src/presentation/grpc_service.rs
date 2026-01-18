// presentation/grpc_service.rs
use tonic::{Request, Response, Status};
use tracing::{info, warn};
use std::sync::Arc;

use crate::{
    application::{
        blog_service::BlogService,
        auth_service::AuthService,
    },
    infrastructure::jwt::JwtService,
    domain::{user::{RegisterUser,LoginUser},
             post::{CreatePost, UpdatePost},
        error::BlogError,
    },
};

// Импортируем сгенерированные protobuf-типы
use blog::proto::{
    blog_service_server::BlogService as ProtoBlogService,
    CreateUserRequest, CreateUserResponse,
    LoginUserRequest, LoginUserResponse,
    CreatePostRequest, CreatePostResponse,
    UpdatePostRequest, UpdatePostResponse,
    DeletePostRequest, DeletePostResponse,
    GetPostRequest, GetPostResponse,
    ListPostsRequest, ListPostsResponse,
    User as ProtoUser,
    Post as ProtoPost,
};

pub struct BlogGrpcService {
    blog_service: Arc<dyn BlogService>,
    auth_service: Arc<dyn AuthService>,
    jwt_service: Arc<dyn JwtService>,
}

impl BlogGrpcService {
    pub fn new(
        blog_service: Arc<dyn BlogService>,
        auth_service: Arc<dyn AuthService>,
        jwt_service: Arc<dyn JwtService>,
    ) -> Self {
        Self {
            blog_service,
            auth_service,
            jwt_service,
        }
    }

    fn extract_token(&self, request: &Request<()>) -> Result<String, Status> {
        let metadata = request.metadata();
        let auth_header = metadata
            .get("authorization")
            .ok_or_else(|| {
                warn!("Missing authorization header");
                Status::unauthenticated("Missing authorization token")
            })?
            .to_str()
            .map_err(|_| {
                warn!("Invalid authorization header format");
                Status::unauthenticated("Invalid authorization header format")
            })?;

        if !auth_header.starts_with("Bearer ") {
            warn!("Invalid authorization header format - missing Bearer prefix");
            return Err(Status::unauthenticated("Invalid authorization header format"));
        }

        Ok(auth_header[7..].to_string())
    }

    fn get_user_id_from_token(&self, token: &str) -> Result<String, Status> {
        let claims = self.jwt_service
            .verify_token(token)
            .map_err(|e| {
                warn!("Invalid JWT token: {}", e);
                Status::unauthenticated("Invalid token")
            })?;

        Ok(claims.sub)
    }

    fn authenticate_request(&self, request: &Request<()>) -> Result<String, Status> {
        let token = self.extract_token(request)?;
        self.get_user_id_from_token(&token)
    }

    fn map_app_error_to_status(&self, error: AppError) -> Status {
        match error {
            AppError::NotFound(_) => Status::not_found(error.to_string()),
            AppError::Unauthorized(_) => Status::unauthenticated(error.to_string()),
            AppError::Validation(_) => Status::invalid_argument(error.to_string()),
            AppError::AlreadyExists(_) => Status::already_exists(error.to_string()),
            AppError::DatabaseError(_) => Status::internal("Database error"),
            AppError::JwtError(_) => Status::unauthenticated("Authentication error"),
            _ => Status::internal("Internal server error"),
        }
    }

    fn to_proto_user(&self, user: crate::domain::models::User) -> ProtoUser {
        ProtoUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: Some(prost_types::Timestamp {
                seconds: user.created_at.timestamp(),
                nanos: user.created_at.timestamp_subsec_nanos() as i32,
            }),
        }
    }

    fn to_proto_post(&self, post: crate::domain::models::Post) -> ProtoPost {
        ProtoPost {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            created_at: Some(prost_types::Timestamp {
                seconds: post.created_at.timestamp(),
                nanos: post.created_at.timestamp_subsec_nanos() as i32,
            }),
            updated_at: post.updated_at.map(|dt| prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }),
        }
    }
}

#[tonic::async_trait]
impl ProtoBlogService for BlogGrpcService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        info!("CreateUser gRPC request received");

        let req = request.into_inner();
        let command = CreateUserCommand {
            username: req.username,
            email: req.email,
            password: req.password,
        };

        match self.auth_service.register(command).await {
            Ok(user) => {
                let response = CreateUserResponse {
                    user: Some(self.to_proto_user(user)),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("CreateUser error: {}", e);
                Err(self.map_app_error_to_status(e))
            }
        }
    }

    async fn login_user(
        &self,
        request: Request<LoginUserRequest>,
    ) -> Result<Response<LoginUserResponse>, Status> {
        info!("LoginUser gRPC request received");

        let req = request.into_inner();
        let command = LoginCommand {
            email: req.email,
            password: req.password,
        };

        match self.auth_service.login(command).await {
            Ok((user, token)) => {
                let response = LoginUserResponse {
                    user: Some(self.to_proto_user(user)),
                    token,
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("LoginUser error: {}", e);
                Err(self.map_app_error_to_status(e))
            }
        }
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        info!("CreatePost gRPC request received");

        let user_id = self.authenticate_request(request.metadata())?;
        let req = request.into_inner();

        let command = CreatePostCommand {
            title: req.title,
            content: req.content,
            author_id: user_id,
        };

        match self.blog_service.create_post(command).await {
            Ok(post) => {
                let response = CreatePostResponse {
                    post: Some(self.to_proto_post(post)),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("CreatePost error: {}", e);
                Err(self.map_app_error_to_status(e))
            }
        }
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<UpdatePostResponse>, Status> {
        info!("UpdatePost gRPC request received");

        let user_id = self.authenticate_request(request.metadata())?;
        let req = request.into_inner();

        let command = UpdatePostCommand {
            id: req.id,
            title: req.title,
            content: req.content,
            author_id: user_id,
        };

        match self.blog_service.update_post(command).await {
            Ok(post) => {
                let response = UpdatePostResponse {
                    post: Some(self.to_proto_post(post)),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("UpdatePost error: {}", e);
                Err(self.map_app_error_to_status(e))
            }
        }
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        info!("DeletePost gRPC request received");

        let user_id = self.authenticate_request(request.metadata())?;
        let req = request.into_inner();

        match self.blog_service.delete_post(req.id, &user_id).await {
            Ok(_) => {
                let response = DeletePostResponse { success: true };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("DeletePost error: {}", e);
                Err(self.map_app_error_to_status(e))
            }
        }
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        info!("GetPost gRPC request received");

        let req = request.into_inner();

        match self.blog_service.get_post(req.id).await {
            Ok(post) => {
                let response = GetPostResponse {
                    post: Some(self.to_proto_post(post)),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("GetPost error: {}", e);
                Err(self.map_app_error_to_status(e))
            }
        }
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        info!("ListPosts gRPC request received");

        // Опционально: можем извлечь токен, но не требуем его для этого метода
        let user_id = self.extract_token(request.metadata()).ok()
            .and_then(|token| self.get_user_id_from_token(&token).ok());

        let req = request.into_inner();
        let offset = req.offset.unwrap_or(0) as usize;
        let limit = req.limit.unwrap_or(20) as usize;

        match self.blog_service.list_posts(offset, limit, user_id.as_deref()).await {
            Ok(posts) => {
                let proto_posts = posts
                    .into_iter()
                    .map(|post| self.to_proto_post(post))
                    .collect();

                let response = ListPostsResponse { posts: proto_posts };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("ListPosts error: {}", e);
                Err(self.map_app_error_to_status(e))
            }
        }
    }
}
