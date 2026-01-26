use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) author_id: i64,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ListPosts {
    pub(crate) posts: Option<Vec<Post>>,
    pub(crate) total: usize,
    pub(crate) limit: i64,
    pub(crate) offset: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct CreatePost {
    pub(crate) title: String,
    pub(crate) content: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct UpdatePost {
    pub(crate) title: String,
    pub(crate) content: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GetPaginationPost {
    pub(crate) limit: i64,
    pub(crate) offset: i64,
}

impl CreatePost {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}
