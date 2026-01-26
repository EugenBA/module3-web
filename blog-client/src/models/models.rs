use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub username: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub total: Option<u64>,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPostsRequest {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub author_id: Option<String>,
}
