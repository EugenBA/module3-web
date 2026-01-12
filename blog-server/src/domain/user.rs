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
pub(crate) struct RegisterUser {
    username: String,
    pub(crate) email: String,
    pub(crate) password: String
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

