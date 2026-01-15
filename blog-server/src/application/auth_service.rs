use crate::data::user_repository::UserRepository;
use crate::domain::user::{LoginUser, RegisterUser};
use crate::domain::{error::DomainError, user::User};
use crate::infrastructure::hash::{hash_password, verify_password};
use crate::infrastructure::jwt::JwtService;
use std::sync::Arc;
use tracing::instrument;

#[derive(Clone)]
pub struct AuthService<R: UserRepository + 'static> {
    repo: Arc<R>,
    keys: JwtService,
}

impl<R> AuthService<R>
where
    R: UserRepository + 'static,
{
    pub fn new(repo: Arc<R>, keys: JwtService) -> Self {
        Self { repo, keys }
    }

    pub fn keys(&self) -> &JwtService {
        &self.keys
    }

    #[instrument(skip(self))]
    pub async fn register(&self, register_user: RegisterUser) -> Result<User, DomainError> {
        let hash = hash_password(&register_user.password)
            .map_err(|err| DomainError::Internal(err.to_string()))?;
        let user = User::new(
            register_user.username.to_lowercase(),
            register_user.email,
            hash,
        );
        self.repo.create(user).await.map_err(DomainError::from)
    }

    #[instrument(skip(self))]
    pub async fn login(&self, login_user: LoginUser) -> Result<String, DomainError> {
        let user = self
            .repo
            .find_by_name(&login_user.username.to_lowercase())
            .await
            .map_err(DomainError::from)?
            .ok_or_else(|| DomainError::Unauthorized)?;

        let is_valid = verify_password(&login_user.password, &user.password_hash)
            .map_err(|_| DomainError::Unauthorized)?;
        if !is_valid {
            return Err(DomainError::Unauthorized);
        }

        self.keys
            .generate_token(user.id, user.username.as_str())
            .map_err(|err| DomainError::Internal(err.to_string()))
    }

    pub async fn get_user(&self, user_id: i64) -> Result<User, DomainError> {
        self.repo
            .find_by_id(user_id)
            .await
            .map_err(DomainError::from)?
            .ok_or_else(|| DomainError::UserNotFound(format!("user {}", user_id)))
    }
}
