//! 鉴权模块：
//! - JWT 生成/校验（基于 jsonwebtoken）
//! - 密码哈希/校验（基于 argon2）
//! - `JwtAuth` 提取器：从请求头解析 Bearer Token 并验证，向 handler 提供 Claims

use crate::db::AppState;
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,      // user id
    pub email: String, // 冗余字段，便于调试展示
    pub exp: usize,    // 过期时间（秒）
    pub iat: usize,    // 签发时间（秒）
}

fn now_ts() -> usize {
    Utc::now().timestamp().max(0) as usize
}

pub fn generate_token(state: &AppState, user_id: i64, username: &str) -> AppResult<String> {
    let iat = now_ts();
    let exp = (Utc::now() + Duration::seconds(state.cfg.jwt_ttl))
        .timestamp()
        .max(0) as usize;

    let claims = Claims {
        sub: user_id,
        email: username.to_string(),
        exp,
        iat,
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.cfg.jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

/// 统一封装密码哈希
pub fn hash_password(plain: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(plain.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

/// 校验密码
pub fn verify_password(plain: &str, hashed: &str) -> bool {
    if let Ok(parsed) = PasswordHash::new(hashed) {
        Argon2::default()
            .verify_password(plain.as_bytes(), &parsed)
            .is_ok()
    } else {
        false
    }
}

/// 基于 Bearer Token 的鉴权提取器
/// 用法：在 handler 参数中写 `JwtAuth(claims): JwtAuth`
pub struct JwtAuth(pub Claims);
// #[async_trait] // 同步trait
impl<S> axum::extract::FromRequestParts<S> for JwtAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 获取全局状态（Arc<AppState>）——由 Router::with_state 注入
        let state = parts
            .extensions
            .get::<Arc<AppState>>()
            .cloned()
            .ok_or_else(|| AppError::BadRequest("missing app state".into()))?;

        // 解析 Authorization: Bearer <token>
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|hv| hv.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("missing bearer token".into()))?;

        let claims = decode_token(&state, token)?;
        Ok(JwtAuth(claims))
    }
}

/// 用 state 里的密钥验证签名。
pub fn decode_token(state: &AppState, token: &str) -> AppResult<Claims> {
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.cfg.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
