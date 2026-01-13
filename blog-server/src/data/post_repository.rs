use sqlx::{PgPool, Row};
use tonic::async_trait;
use crate::domain::{post::Post, error::DomainError};


#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<Post, DomainError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError>;
    async fn update_post(&self, post: Post) -> Result<Post, DomainError>;
    async fn delete_post(&self, id: i64) -> Result<(), DomainError>;
}

#[derive(Debug)]
pub(crate)struct InDbPostRepository {
    pool: PgPool,
}


#[async_trait]
impl PostRepository for InDbPostRepository{
    async fn create(
        &self,
        post: Post,
    ) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
        INSERT INTO post (title, content, author_id)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, author_id, created_at, update_at
        "#,
        )
            .bind(post.title)
            .bind(post.content)
            .bind(post.author_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(Post {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("update_at")
        })
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError> {
        let row= sqlx::query(
            r#"
        SELECT id, title, content, author_id, created_at, updated_at
        FROM post
        WHERE id = $1
        "#,
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Post {
            id: r.get("id"),
            title: r.get("title"),
            content: r.get("content"),
            author_id: r.get("author_id"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at")
        }))
    }

    async fn update_post(&self, post: Post) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
        UPDATE post
        set title=#2, content=#3, updated_at=NOW()
        WHERE id = #1
        RETURNING id, title, content, author_id, created_at, updated_at
        "#,
        )
            .bind(post.id)
            .bind(post.title)
            .bind(post.content)
            .fetch_one(&self.pool)
            .await?;

        Ok(Post{
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        })
    }

    async fn delete_post(&self, id: i64) -> Result<(), DomainError> {
        sqlx::query(
            r#"
        DELETE post
        WHERE id = #1
        "#,
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(())
    }
}
impl InDbPostRepository {
    pub(crate)fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}



