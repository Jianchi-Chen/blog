//! 鉴权模块：
//! - JWT 生成/校验（基于 jsonwebtoken）
//! - 密码哈希/校验（基于 argon2）
//! - 适配 Tauri 命令系统

use crate::config::Config;
use anyhow::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String, // user id
    pub message: String, // 用户名或其他信息
    pub exp: usize,      // 过期时间（秒）
    pub iat: usize,      // 签发时间（秒）
}

fn now_ts() -> usize {
    Utc::now().timestamp().max(0) as usize
}

/// 生成 JWT Token
pub fn generate_token(config: &Config, user_id: String, username: &str) -> Result<String> {
    let iat = now_ts();
    let exp = (Utc::now() + Duration::seconds(config.jwt_ttl))
        .timestamp()
        .max(0) as usize;

    let claims = Claims {
        user_id,
        message: username.to_string(),
        exp,
        iat,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;

    Ok(token)
}

/// 验证并解析 JWT Token
pub fn decode_token(config: &Config, token: &str) -> Result<Claims> {
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}

/// 统一封装密码哈希
pub fn hash_password(plain: &str) -> Result<String> {
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

/// 从 Bearer Token 中提取 token 字符串
pub fn extract_token_from_header(auth_header: &str) -> Option<String> {
    if auth_header.starts_with("Bearer ") {
        Some(auth_header[7..].to_string())
    } else {
        None
    }
}
