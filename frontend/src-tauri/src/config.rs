//! 统一配置中心：
//! - 开发环境：从 .env 文件加载配置
//! - 生产环境（Tauri）：从应用数据目录的 config.json 加载/创建配置

use serde::{Deserialize, Serialize};
use std::env;
use tauri::AppHandle;

#[cfg(not(debug_assertions))]
use tauri::Manager;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    /// JWT 过期秒数
    pub jwt_ttl: i64,
}

impl Config {
    /// 加载配置：开发模式使用环境变量，生产模式使用持久化配置
    #[allow(unused_variables)]
    pub fn load(app_handle: Option<&AppHandle>) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(debug_assertions)]
        {
            // 开发模式：从 .env 加载
            Ok(Self::from_env())
        }

        #[cfg(not(debug_assertions))]
        {
            // 生产模式：从 app data 加载或创建
            let app = app_handle.ok_or("AppHandle required in production mode")?;
            Self::from_app_data(app)
        }
    }

    /// 从环境变量加载（开发模式）
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

    /// 从应用数据目录加载或创建配置（生产模式）
    #[cfg(not(debug_assertions))]
    fn from_app_data(app: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        std::fs::create_dir_all(&app_data_dir)?;

        let config_path = app_data_dir.join("config.json");

        if config_path.exists() {
            // 读取现有配置
            let config_str = std::fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&config_str)?)
        } else {
            // 创建默认配置
            let config = Self {
                database_url: "app.db".into(), // 相对路径，稍后会转换为绝对路径
                host: "127.0.0.1".into(),
                port: 3000,
                jwt_secret: Self::generate_random_secret(),
                jwt_ttl: 7 * 24 * 3600,
            };

            // 保存配置
            let config_str = serde_json::to_string_pretty(&config)?;
            std::fs::write(&config_path, config_str)?;

            Ok(config)
        }
    }

    /// 生成随机 JWT secret
    #[allow(dead_code)]
    fn generate_random_secret() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// 获取数据库路径（开发环境使用相对路径，生产环境使用 app data 路径）
    #[allow(unused_variables)]
    pub fn get_database_path(&self, app_handle: Option<&AppHandle>) -> String {
        #[cfg(debug_assertions)]
        {
            // 开发模式：直接使用配置中的路径
            self.database_url.clone()
        }

        #[cfg(not(debug_assertions))]
        {
            // 生产模式：放在 app data 目录
            if let Some(app) = app_handle {
                if let Ok(app_data_dir) = app.path().app_data_dir() {
                    let db_path = app_data_dir.join(&self.database_url);
                    return format!("sqlite://{}", db_path.display());
                }
            }
            format!("sqlite://{}", self.database_url)
        }
    }
}
