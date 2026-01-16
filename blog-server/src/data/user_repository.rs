use crate::domain::{error::DomainError, user::User};
use sqlx::{PgPool, Row};
use tonic::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
    fn new(pool: PgPool) -> Self;
}

#[derive(Debug, Clone)]
pub(crate) struct InDbUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for InDbUserRepository {
    async fn create(&self, user: User) -> Result<User, DomainError> {
        if let Some(_) = self.find_by_name(user.username.as_str()).await? {
            return Err(DomainError::UserAlreadyExists(user.username));
        }
        let row = sqlx::query(
            r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password_hash, created_at
        "#,
        )
        .bind(user.username)
        .bind(user.email)
        .bind(user.password_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("username"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        })
    }

    async fn find_by_name(&self, username: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
        SELECT id, username, email, password_hash, created_at
        FROM users
        WHERE username = $1
        "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
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
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
        }))
    }

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl InDbUserRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
