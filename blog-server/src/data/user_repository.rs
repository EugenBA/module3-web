use std::ops::Deref;
use std::sync::Arc;
use sqlx::{PgPool, Row};
use tonic::async_trait;
use crate::domain::{user::User, error::DomainError};


#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
}

#[derive(Debug)]
pub(crate)struct InDbUserRepository {
    pool: Arc<PgPool>,
}


#[async_trait]
impl UserRepository for InDbUserRepository{
    async fn create(
        &self,
        user: User,
    ) -> Result<User, DomainError> {
        let result_user = user.clone();
        if let Some(_) = self.find_by_name(user.username.as_str()).await?{
            return Err(DomainError::UserAlreadyExists(result_user.username));
        }
        sqlx::query(
            r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        "#,
        )
            .bind(user.username)
            .bind(user.email)
            .bind(user.password_hash)
            .execute(self.pool.deref())
            .await?;
        Ok(result_user)
    }

    async fn find_by_name(&self, username: &str) -> Result<Option<User>, DomainError> {
        let row= sqlx::query(
            r#"
        SELECT id, username, email, password_hash, created_at
        FROM users
        WHERE username = $1
        "#,
        )
            .bind(username)
            .fetch_optional(self.pool.deref())
            .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
        }))
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
        SELECT id, username, email, password_hash, created_at
        FROM users
        WHERE id = $1
        "#,
        )
            .bind(id)
            .fetch_optional(self.pool.deref())
            .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
        }))
    }
}
impl InDbUserRepository {
    pub(crate)fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}



