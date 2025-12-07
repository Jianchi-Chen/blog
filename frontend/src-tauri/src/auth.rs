//! 鉴权模块：
//! - JWT 生成/校验（基于 jsonwebtoken）
//! - 密码哈希/校验（基于 argon2）

use crate::config::Config;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String, // user id
    pub message: String, // 冗余字段，便于调试展示
    pub exp: usize,      // 过期时间（秒）
    pub iat: usize,      // 签发时间（秒）
}

/// 获取当前时间戳（秒）
fn now_ts() -> usize {
    Utc::now().timestamp().max(0) as usize
}

pub fn generate_token(config: &Config, user_id: String, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
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

/// 统一封装密码哈希
pub fn hash_password(plain: &str) -> Result<String, argon2::password_hash::Error> {
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

/// 解码并验证 JWT Token
pub fn decode_token(config: &Config, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
