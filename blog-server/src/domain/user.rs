use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    password_hash: String,
    created_at: i64
}
#[derive(Debug, Serialize, Deserialize)]
struct RegisterUser {
    username: String,
    email: String,
    password: String
}
#[derive(Debug, Serialize, Deserialize)]
struct LoginUser {
    username: String,
    password: String
}

