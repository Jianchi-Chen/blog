//! Article 模型定义

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::repositories::article::NewArticle;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ArticleModel {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub created_at: Option<String>,
    pub update_at: Option<String>,
    pub status: Option<String>,
    pub views: Option<i32>,
    pub tags: Option<String>,
    pub message: String,
}

impl From<ArticleModel> for NewArticle {
    fn from(v: ArticleModel) -> Self {
        Self {
            id: Some(v.id),
            title: v.title,
            content: v.content,
            summary: v.summary,
            status: v.status,
            tags: v.tags,
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PubArticles {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub created_at: String,
    pub status: String,
    pub views: i32,
    pub tags: String,
}
