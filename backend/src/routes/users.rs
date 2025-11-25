//! /users 相关路由：列表（受保护）

use crate::auth::JwtAuth;
use crate::db::AppState;
use crate::error::{AppError, AppResult};
use crate::models::user::{UserPublic, delete_user_by_id, edit_user_account, list_users};
use crate::routes::auth::get_ident_by_id;
use axum::extract::{Path, Query};
use axum::{Json, extract::State};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct ListUsersResponse {
    users: Vec<UserPublic>,
}

// HTTP层函数，对外接口
pub async fn get_users(
    State(state): State<Arc<AppState>>,
    Query(query): Query<UsersQuery>,
) -> AppResult<Json<ListUsersResponse>> {
    tracing::info!("Received query: {:?}", query);

    let default_limit = query.limit.unwrap_or(10);

    let users = list_users(&state.pool, default_limit).await?;
    tracing::info!("Fetched users: {:?}", users);
    Ok(Json(ListUsersResponse { users }))
}

pub async fn delete_users(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    JwtAuth(auth): JwtAuth,
) -> AppResult<StatusCode> {
    tracing::info!("Received request to delete user: {:?}", user_id);

    // 验证前端传来的token
    if get_ident_by_id(&state.pool, &auth.user_id).await? != "admin" {
        tracing::warn!("Unauthorized delete attempt by user: {:?}", auth.user_id);
        return Err(crate::error::AppError::Unauthorized(
            "only admin can delete users".into(),
        ));
    }

    delete_user_by_id(&state.pool, &user_id).await?;
    tracing::info!("Deleted user: {:?}", user_id);

    Ok(StatusCode::NO_CONTENT.into())
}

#[derive(Deserialize, Debug)]
pub struct UsersQuery {
    pub limit: Option<i32>,
}

pub async fn edit_account(
    State(state): State<Arc<AppState>>,
    JwtAuth(auth): JwtAuth,
    Json(payload): Json<AdminEditAccountPayload>,
) -> AppResult<StatusCode> {
    tracing::info!(
        "AdminEditAccountPayload() Received request to edit account: {:?}",
        auth.user_id
    );

    // 验证前端传来的token是否为空
    if payload.current_token.clone().is_none() {
        tracing::warn!(
            "No current_token provided in edit account request by user: {:?}",
            auth.user_id
        );
        return Err(AppError::BadRequest("current_token is required".into()));
    }

    // 仅管理员编辑
    if get_ident_by_id(&state.pool, &auth.user_id).await? != "admin" {
        tracing::warn!(
            "Unauthorized edit attempt by user: {:?}, current identity: {:?}",
            auth.user_id,
            payload.edited_identity
        );
        return Err(AppError::Unauthorized(
            "cannot edit other user's account".into(),
        ));
    }

    // 防止更改超管的权限
    if payload.edited_identity.clone().unwrap_or_default() == "admin" && &payload.edited_id == "1" {
        tracing::warn!(
            "Attempt to change identity to superadmin by user: {:?}",
            auth.user_id
        );
        return Err(AppError::Unauthorized(
            "cannot change identity to superadmin".into(),
        ));
    }

    edit_user_account(&state.pool, payload).await?;

    tracing::info!("Edited account for user: {:?}", auth.user_id);

    Ok(StatusCode::OK.into())
}

#[derive(Deserialize, Debug)]
pub struct AdminEditAccountPayload {
    pub current_token: Option<String>,
    pub edited_id: String,
    pub edited_username: Option<String>,
    pub edited_password: Option<String>,
    pub edited_identity: Option<String>,
}
