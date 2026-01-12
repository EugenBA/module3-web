use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug)]
struct PostRow {
    id: i64,
    title: String,
    content: String,
    author_id: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

async fn add_post(
    pool: &PgPool,
    title: &str,
    content: &str,
    author_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO post (title, content, author_id)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(title)
    .bind(content)
    .bind(author_id)
    .execute(pool)
    .await?;
    Ok(())
}

async fn get_post(pool: &PgPool, id: i64) -> Result<Option<PostRow>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, title, content, author_id, created_at, updated_at
        FROM post
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| PostRow {
        id: r.get("id"),
        title: r.get("title"),
        content: r.get("content"),
        author_id: r.get("author_id"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

async fn update_post(
    pool: &PgPool,
    id: i64,
    title: &str,
    content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE post
        SET title = $2, content = $3, updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(title)
    .bind(content)
    .execute(pool)
    .await?;
    Ok(())
}

async fn delete_post(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE post
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
