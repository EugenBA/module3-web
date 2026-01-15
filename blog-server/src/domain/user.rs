use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: i64,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password_hash: String,
    pub(crate) created_at: i64,
}
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RegisterUser {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
}
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct LoginUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct TokenResponse {
    pub(crate) access_token: String,
    pub(crate) username: String,
}

impl User {
    pub(crate) fn new(username: String, email: String, hash: String) -> Self {
        Self {
            id: 0,
            username,
            email,
            password_hash: hash,
            created_at: 0,
        }
    }
}
