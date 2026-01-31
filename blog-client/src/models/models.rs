#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use crate::blog::ProtoPost;

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
#[cfg(not(target_arch = "wasm32"))]
impl Post{
    fn timestamp_to_chrono(timestamp: Option<prost_types::Timestamp>) -> DateTime<Utc>{
        if let Some(ts) = timestamp {
            let system_time = std::time::UNIX_EPOCH
                + Duration::from_secs(ts.seconds as u64)
                + Duration::from_nanos(ts.nanos as u64);
            DateTime::<Utc>::from(system_time)
        }
        else {
            DateTime::<Utc>::default()
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl From<ProtoPost> for Post{
    fn from(value: ProtoPost) -> Self {
        Self{ id: value.id,
            title: value.title,
            content: value.content,
            author_id: value.author_id,
            created_at: Post::timestamp_to_chrono(value.created_at),
            updated_at: Some(Post::timestamp_to_chrono(value.updated_at))
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub post: Option<Vec<Post>>,
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
