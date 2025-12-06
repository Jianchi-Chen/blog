use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    /// 为简化与 SQLite datetime TEXT 的映射，这里使用 String
    pub identity: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserPublic {
    pub id: String,
    pub username: String,
    pub identity: String,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            identity: u.identity,
        }
    }
}

/// 用户传来的数据
#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub identity: String,
}

#[derive(Serialize)]
pub struct ListUsersResponse {
    users: Vec<UserPublic>,
}

#[derive(Deserialize, Debug)]
pub struct AdminEditAccountPayload {
    pub current_token: Option<String>,
    pub edited_id: String,
    pub edited_username: Option<String>,
    pub edited_password: Option<String>,
    pub edited_identity: Option<String>,
}
