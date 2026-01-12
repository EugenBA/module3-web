use serde::{Deserialize, Serialize};

//Реализуйте метод new для создания нового поста.
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id:  i64,
    title: String,
    content: String,
    author_id: i64,
    created_at: i64,
    updated_at: i64
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
    title: String,
    content: String
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePost {
    title: String,
    content: String
}