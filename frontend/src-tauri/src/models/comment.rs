use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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

#[derive(Serialize, Deserialize)]
pub struct CommentsResponse {
    pub comments: Vec<CommentWithLike>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentIncome {
    pub article_id: String,
    pub user_id: Option<String>,
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeleteCommentParams {
    pub comment_id: String,
    // pub article_id: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[warn(non_snake_case)]
pub struct LikeCommentPayload {
    pub comment_id: String,
}

#[derive(Serialize)]
pub struct CommentsLikeResponse {
    pub comment_id: String,
    pub like_or_unlike: String,
}
