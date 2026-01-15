use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: i64,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password_hash: String,
    pub(crate) created_at: i64,
}
#[derive(Debug, Deserialize)]
pub(crate) struct RegisterUser {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
}
#[derive(Debug, Deserialize)]
pub(crate) struct LoginUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl User {
    pub(crate) fn new(username: String, hash: String) -> Self {
        Self{
            id: 0,
            username,
            email: "".to_string(),
            password_hash: hash,
            created_at: 0,
        }
    }
}
