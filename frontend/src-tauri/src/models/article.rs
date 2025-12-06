use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ArticleModel {
    pub id: String, // 如果数据库的字段可为NULL，rust的字段也必须要可为None
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub created_at: Option<String>,
    pub update_at: Option<String>,
    pub status: Option<String>,
    pub views: Option<i32>,
    pub tags: Option<String>,
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

#[derive(Serialize, Debug)]
pub struct ArticleResponse {
    articles: Vec<PubArticles>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct NewArticle {
    pub id: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub status: Option<String>,
    pub tags: Option<String>,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct NewStatus {
    pub toggle: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Params {
    pub id: String,
}

// 获取文章的参数
#[derive(Deserialize, Debug)]
pub struct GetArticlesParams {
    pub identity: String,
    pub condition: Option<String>,
}
