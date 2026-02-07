use crate::domain::post::{CreatePost, UpdatePost};
use crate::domain::{error::DomainError, post::Post};
use sqlx::{PgPool, Row};
use tonic::async_trait;

#[async_trait]
pub(crate) trait BlogRepository: Send + Sync {
    async fn create(&self, author_id: i64, crete_post: CreatePost) -> Result<Post, DomainError>;
    async fn get_post(&self, post_id: i64) -> Result<Option<Post>, DomainError>;
    async fn update_post(
        &self,
        post_id: i64,
        author_id: i64,
        update_post: UpdatePost,
    ) -> Result<Post, DomainError>;
    async fn delete_post(&self, post_id: i64, author_id: i64) -> Result<(), DomainError>;
    async fn get_posts(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError>;
    async fn find_post(&self, post_id: i64, author_id: i64) -> Result<Option<Post>, DomainError>;
    fn new(pool: PgPool) -> Self;
}

#[derive(Debug, Clone)]
pub(crate) struct InDbPostRepository {
    pool: PgPool,
}

#[async_trait]
impl BlogRepository for InDbPostRepository {
    async fn create(&self, author_id: i64, create_post: CreatePost) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
        INSERT INTO posts (title, content, author_id)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, author_id, created_at, updated_at
        "#,
        )
        .bind(create_post.title)
        .bind(create_post.content)
        .bind(author_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(Post {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    async fn get_post(&self, post_id: i64) -> Result<Option<Post>, DomainError> {
        let row = sqlx::query(
            r#"
        SELECT id, title, content, author_id, created_at, updated_at
        FROM posts
        WHERE id = $1
        "#,
        )
        .bind(post_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Post {
            id: r.get("id"),
            title: r.get("title"),
            content: r.get("content"),
            author_id: r.get("author_id"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    async fn update_post(
        &self,
        post_id: i64,
        author_id: i64,
        update_post: UpdatePost,
    ) -> Result<Post, DomainError> {
        if let None = self.find_post(post_id, author_id).await? {
            return Err(DomainError::PostNotFound);
        }
        let row = sqlx::query(
            r#"
        UPDATE posts
        set title=$3, content=$4, updated_at=NOW()
        WHERE id=$1 and author_id=$2
        RETURNING id, title, content, author_id, created_at, updated_at
        "#,
        )
        .bind(post_id)
        .bind(author_id)
        .bind(update_post.title)
        .bind(update_post.content)
        .fetch_one(&self.pool)
        .await?;

        Ok(Post {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    async fn delete_post(&self, post_id: i64, author_id: i64) -> Result<(), DomainError> {
        if let None = self.find_post(post_id, author_id).await? {
            return Err(DomainError::PostNotFound);
        }
        sqlx::query(
            r#"
        DELETE FROM posts
        WHERE id = $1
        "#,
        )
        .bind(post_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_posts(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError> {
        let row = sqlx::query(
            r#"
        SELECT id, title, content, author_id, created_at, updated_at
        FROM posts
        ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(row
            .into_iter()
            .map(|r| Post {
                id: r.get("id"),
                title: r.get("title"),
                content: r.get("content"),
                author_id: r.get("author_id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .into_iter()
            .collect())
    }

    async fn find_post(&self, post_id: i64, author_id: i64) -> Result<Option<Post>, DomainError> {
        let row = sqlx::query(
            r#"
        SELECT id, title, content, author_id, created_at, updated_at
        FROM posts
        WHERE id=$1 and author_id=$2
        
        "#,
        )
        .bind(post_id)
        .bind(author_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Post {
            id: r.get("id"),
            title: r.get("title"),
            content: r.get("content"),
            author_id: r.get("author_id"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl InDbPostRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
