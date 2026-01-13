use serde::{Deserialize, Serialize};

//Реализуйте метод new для создания нового поста.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) author_id: i64,
    pub(crate) created_at: i64,
    pub(crate) updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    title: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePost {
    title: String,
    content: String,
}
