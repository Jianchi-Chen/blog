//! /users 相关路由：列表（受保护）

use crate::auth::JwtAuth;
use crate::db::AppState;
use crate::error::AppResult;
use crate::models::user::{UserPublic, list_users};
use axum::{Json, extract::State};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ListUsersResponse {
    users: Vec<UserPublic>,
}

// HTTP层函数，对外接口
pub async fn list(
    State(state): State<Arc<AppState>>,
    _auth: JwtAuth, // 需要登录
) -> AppResult<Json<ListUsersResponse>> {
    // panic!("pool: {:?}", &state.pool);
    // println!("1");
    let users = list_users(&state.pool, 100).await?;
    Ok(Json(ListUsersResponse { users }))
}
