use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::JwtAuth,
    db::AppState,
    error::{AppError, AppResult},
    models::{
        article::find_article_by_id,
        comment::{
            Comment, delete_comment_by_comment_id, fetch_comments_by_article_id,
            post_comment_by_article_id,
        },
        user::find_user_by_id,
    },
    routes::auth::get_ident_by_id,
};

#[derive(Serialize, Deserialize)]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentIncome {
    pub article_id: String,
    pub user_id: Option<String>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeleteCommentParams {
    pub comment_id: String,
    // pub article_id: String,
    pub message: Option<String>,
}

/// 获取评论
pub async fn handle_get_comments(
    State(state): State<Arc<AppState>>,
    Path(arti_id): Path<String>,
) -> AppResult<Json<CommentsResponse>> {
    let res;

    let arti_id = match find_article_by_id(&state.pool, &arti_id).await? {
        Some(v) => v.id,
        None => return Err(AppError::BadRequest("未找到文章".into())),
    };

    res = fetch_comments_by_article_id(&state.pool, &arti_id).await?;

    Ok(Json(CommentsResponse { comments: res }))
}

/// 发表评论
// 有个坑，jwt如果放在后面，axum提取器可能不会识别从而报错
pub async fn handle_post_comment(
    State(state): State<Arc<AppState>>,
    JwtAuth(auth): JwtAuth,
    Json(payload): Json<CommentIncome>,
) -> AppResult<Json<Comment>> {
    let mut username = "".to_string();
    if let Some(u) = find_user_by_id(&state.pool, auth.user_id).await? {
        if u.identity == "visitor" {
            // println!("{}", u.identity);
            return Err(AppError::Unauthorized("未登录".into()));
        }
        username = u.username;
    }

    let res;
    if let Some(_) = find_article_by_id(&state.pool, &payload.article_id).await? {
        res = post_comment_by_article_id(&state.pool, payload, &username).await?;
    } else {
        return Err(AppError::BadRequest("未找到文章".into()));
    };

    Ok(Json(res))
}

/// 删除评论
pub async fn handle_delete_comment(
    State(state): State<Arc<AppState>>,
    JwtAuth(auth): JwtAuth,
    Path(comment_id): Path<String>,
) -> AppResult<Json<DeleteCommentParams>> {
    // println!("{}", comment_id);
    if get_ident_by_id(&state.pool, &auth.user_id).await? != "admin" {
        return Err(AppError::Unauthorized("权限不足".into()));
    };

    if delete_comment_by_comment_id(&state.pool, &comment_id).await? != "done".to_string() {
        return Err(AppError::BadRequest("删除失败".into()));
    }
    // println!("asd");

    let res = DeleteCommentParams {
        comment_id,
        message: Some("deletion operation completed".to_string()),
    };

    Ok(Json(res))
}
