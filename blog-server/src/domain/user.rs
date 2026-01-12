use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: i64,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password_hash: String,
    pub(crate) created_at: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RegisterUser {
    username: String,
    pub(crate) email: String,
    pub(crate) password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginUser {
    pub(crate) username: String,
    pub(crate) password: String,
}
