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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct CreatePost {
    pub(crate) title: String,
    pub(crate) content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UpdatePost {
    pub(crate) title: String,
    pub(crate) content: String,
}
impl CreatePost {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}
