//! /auth 相关路由：注册、登录、获取当前用户信息
//! 说明：演示如何组合 models + auth + error + state

use crate::auth::{JwtAuth, generate_token, hash_password, verify_password};
use crate::db::AppState;
use crate::error::{AppError, AppResult};
use crate::models::user::{
    NewUser, UserPublic, find_user_by_id, find_user_by_username, insert_user,
};
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
    // pub message: Option<String>,
}

/// POST /register
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<(StatusCode, Json<AuthResponse>)> {
    // 简单校验
    if payload.username.trim().is_empty() || payload.password.len() < 3 {
        return Err(AppError::BadRequest(
            "invalid username or password too short".into(),
        ));
    }

    // 检查是否已经存在
    if find_user_by_username(&state.pool, &payload.username)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("username already registered".into()));
    }

    let password_hash = hash_password(&payload.password)?;
    let new = NewUser {
        username: payload.username.clone(),
        password: password_hash,
    };
    let user = insert_user(&state.pool, &new).await?;
    let token = generate_token(&state, user.id, &user.username)?;
    let user_public = user.into();

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: user_public,
        }),
    ))
}

/// POST /login
// 签名解析：
// State(state) = “把全局 AppState 拿出来，变量叫 state”；
// Json(payload) = “把请求体 JSON 解析成 LoginPayload，变量叫 payload”。
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<Json<AuthResponse>> {
    let Some(user) = find_user_by_username(&state.pool, &payload.username).await? else {
        return Err(AppError::Unauthorized("invalid credentials".into()));
    };

    if !verify_password(&payload.password, &user.password) {
        return Err(AppError::Unauthorized("invalid credentials".into()));
    }

    let token = generate_token(&state, user.id, &user.username)?;
    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// GET /auth/me
pub async fn me(
    State(state): State<Arc<AppState>>,
    JwtAuth(claims): JwtAuth,
) -> AppResult<Json<UserPublic>> {
    let Some(user) = find_user_by_id(&state.pool, claims.sub).await? else {
        return Err(AppError::Unauthorized("user not found".into()));
    };
    Ok(Json(user.into()))
}
