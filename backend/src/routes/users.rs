//! /users 相关路由：列表（受保护）

use crate::auth::JwtAuth;
use crate::db::AppState;
use crate::error::AppResult;
use crate::models::user::{UserPublic, delete_user_by_id, list_users};
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
