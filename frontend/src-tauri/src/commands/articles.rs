//! 文章相关命令

use crate::auth::decode_token;
use crate::config::Config;
use crate::models::article::{ArticleModel, PubArticles};
use crate::repositories::article;
use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

#[derive(Serialize, Debug)]
pub struct ArticleResponse {
    articles: Vec<PubArticles>,
}

/// 获取文章列表
#[tauri::command]
pub async fn get_articles(
    identity: String,
    condition: Option<String>,
    pool: State<'_, SqlitePool>,
) -> Result<ArticleResponse, String> {
    log::info!("get_articles");
    let params = article::GetArticlesParams {
        identity,
        condition,
    };

    let articles = article::get_articles(pool.inner(), params)
        .await
        .map_err(|e| format!("Failed to fetch articles: {}", e))?;

    log::info!("success get_articles");
    Ok(ArticleResponse { articles })
}

/// 获取单篇文章
#[tauri::command]
pub async fn get_article_by_id(
    id: String,
    pool: State<'_, SqlitePool>,
) -> Result<ArticleModel, String> {
    let result = article::find_article_by_id(pool.inner(), &id)
        .await
        .map_err(|e| format!("Failed to fetch article: {}", e))?
        .ok_or("Article not found")?;

    Ok(result)
}

/// 新建文章
#[tauri::command]
pub async fn create_article(
    token: String,
    article_data: article::NewArticle,
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<ArticleModel, String> {
    // 验证 token
    let _claims = decode_token(&config, &token).map_err(|e| format!("Invalid token: {}", e))?;

    let result = article::post_article(pool.inner(), &article_data)
        .await
        .map_err(|e| format!("Failed to create article: {}", e))?;

    Ok(result)
}

/// 更新文章
#[tauri::command]
pub async fn update_article(
    token: String,
    id: String,
    article_data: article::NewArticle,
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<ArticleModel, String> {
    // 验证 token
    let _claims = decode_token(&config, &token).map_err(|e| format!("Invalid token: {}", e))?;

    let result = article::put_article_by_id(pool.inner(), &id, article_data)
        .await
        .map_err(|e| format!("Failed to update article: {}", e))?;

    Ok(result)
}

/// 删除文章
#[tauri::command]
pub async fn delete_article(
    token: String,
    id: String,
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<(), String> {
    // 验证 token
    let _claims = decode_token(&config, &token).map_err(|e| format!("Invalid token: {}", e))?;

    article::delete_article_by_id(pool.inner(), &id)
        .await
        .map_err(|e| format!("Failed to delete article: {}", e))?;

    Ok(())
}

/// 更改文章状态
#[tauri::command]
pub async fn toggle_article_status(
    token: String,
    id: String,
    status: article::NewStatus,
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<ArticleModel, String> {
    // 验证 token
    let _claims = decode_token(&config, &token).map_err(|e| format!("Invalid token: {}", e))?;

    let result = article::patch_article_by_id(pool.inner(), &id, status)
        .await
        .map_err(|e| format!("Failed to toggle article status: {}", e))?;

    Ok(result)
}
