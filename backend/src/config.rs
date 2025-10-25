//! 统一配置中心：从环境变量读取配置，便于测试与部署解耦。

use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    /// JWT 过期秒数
    pub jwt_ttl: i64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "./app.db".into()),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "please-change-me".into()),
            jwt_ttl: env::var("JWT_TTL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(7 * 24 * 3600),
        }
    }
}
