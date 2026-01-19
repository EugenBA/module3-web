
use crate::blog::proto_blog_service_server::ProtoBlogService;
use crate::domain::post::Post;
use crate::domain::user::User;
use tonic::{Request, Response, Status, metadata::MetadataMap};
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
use crate::blog::*;
use crate::data::blog_repository::BlogRepository;
use crate::data::user_repository::UserRepository;

pub(crate) struct BlogGrpcService<R: BlogRepository + 'static, S: UserRepository + 'static> {
    blog_service: Arc<BlogService<R>>,
    auth_service: Arc<AuthService<S>>,
    jwt_service: Arc<JwtService>,
}

impl<R:BlogRepository, S:UserRepository> BlogGrpcService<R, S> {
    pub fn new(
        blog_service: Arc<BlogService<R>>,
        auth_service: Arc<AuthService<S>>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            blog_service,
            auth_service,
            jwt_service,
        }
    }

    fn extract_token(&self, metadata: &MetadataMap) -> Result<String, Status> {
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

    fn get_user_id_from_token(&self, token: &str) -> Result<i64, Status> {
        let claims = self.jwt_service
            .verify_token(token)
            .map_err(|e| {
                warn!("Invalid JWT token: {}", e);
                Status::unauthenticated("Invalid token")
            })?;

        Ok(claims.user_id)
    }

    fn authenticate_request(&self, metadata: &MetadataMap) -> Result<i64, Status> {
        let token = self.extract_token(metadata)?;
        self.get_user_id_from_token(&token)
    }

    fn map_app_error_to_status(&self, error: BlogError) -> Status {
        match error {
            BlogError::NotFound(_) => Status::not_found(error.to_string()),
            BlogError::Unauthorized => Status::unauthenticated("Unauthorized user"),
            BlogError::Validation(_) => Status::invalid_argument(error.to_string()),
            BlogError::UserAlreadyExists(_) => Status::already_exists(error.to_string()),
            BlogError::DatabaseError(_) => Status::internal("Database error"),
            BlogError::InvalidCredentials => Status::unauthenticated("Authentication error"),
            _ => Status::internal("Internal server error"),
        }
    }

    fn to_proto_user(&self, user: User) -> ProtoUser {
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

    fn to_proto_post(&self, post: Post) -> ProtoPost {
        ProtoPost {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            created_at: Some(prost_types::Timestamp {
                seconds: post.created_at.timestamp(),
                nanos: post.created_at.timestamp_subsec_nanos() as i32,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: post.updated_at.timestamp(),
                nanos: post.updated_at.timestamp_subsec_nanos() as i32,
            }),
        }
    }
}

#[tonic::async_trait]
impl<R:BlogRepository, S:UserRepository> ProtoBlogService for BlogGrpcService<R, S>{
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        info!("CreateUser gRPC request received");

        let req = request.into_inner();
        let register_user = RegisterUser {
            username: req.username.clone(),
            email: req.email,
            password: req.password,
        };

        match self.auth_service.register(register_user).await {
            Ok(token) => {
                let response = TokenResponse {
                    token,
                    username: req.username
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("CreateUser error: {}", e);
                Err(self.map_app_error_to_status(e.into()))
            }
        }
    }

    async fn login_user(
        &self,
        request: Request<LoginUserRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        info!("LoginUser gRPC request received");

        let req = request.into_inner();
        let login_user = LoginUser {
            username: req.username.clone(),
            password: req.password,
        };

        match self.auth_service.login(login_user).await {
            Ok((token)) => {
                let response = TokenResponse {
                    username: req.username,
                    token,
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                warn!("LoginUser error: {}", e);
                Err(self.map_app_error_to_status(e.into()))
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

        let create_post = CreatePost {
            title: req.title,
            content: req.content,
        };

        match self.blog_service.create_post(create_post.title,
                                            create_post.content, user_id).await {
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

        let update_post = UpdatePost {
            title: req.title,
            content: req.content,
        };

        match self.blog_service.update_post(req.id, user_id, update_post).await {
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

        match self.blog_service.delete_post(req.id, user_id).await {
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
                if let Some(post) = post{
                    let response = GetPostResponse {
                        post: Some(self.to_proto_post(post)),
                    };
                    Ok(Response::new(response))
                }
                else {
                    warn!("GetPost error: empty posts");
                    Err(self.map_app_error_to_status(BlogError::PostNotFound))
                }
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
