//! Comment 模型定义

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct Comment {
    pub comment_id: String,
    pub article_id: Option<String>,
    pub user: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<String>,
    pub parent_id: Option<String>,
    pub like_count: Option<i64>,
}

#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct CommentWithLike {
    pub comment_id: String,
    pub article_id: Option<String>,
    pub user: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<String>,
    pub parent_id: Option<String>,
    pub like_count: Option<i64>,
    pub liked_by_me: Option<i64>,
}
