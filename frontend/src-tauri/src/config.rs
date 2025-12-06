//! 统一配置中心:开发时从 .env 读取，生产时使用持久化配置 + 默认值。

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
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
    /// 加载配置：开发时读 .env，生产时读持久化配置文件
    #[allow(unused_variables)]
    pub fn load(app_handle: Option<&tauri::AppHandle>) -> Result<Self> {
        // 开发模式：从 .env 读取
        #[cfg(debug_assertions)]
        {
            dotenvy::dotenv().ok();
            return Ok(Self::from_env());
        }

        // 生产模式：从应用数据目录读取配置文件
        #[cfg(not(debug_assertions))]
        {
            if let Some(handle) = app_handle {
                let config_path = Self::get_config_path(handle)?;

                // 如果配置文件存在，读取它
                if config_path.exists() {
                    let content = fs::read_to_string(&config_path)?;
                    let config: Config = serde_json::from_str(&content)?;
                    return Ok(config);
                }

                // 否则创建新配置并保存
                let config = Self::default();
                config.save(handle)?;
                return Ok(config);
            }

            // 如果没有 app_handle，返回默认配置
            Ok(Self::default())
        }
    }

    /// 从环境变量读取配置（开发时使用）
    fn from_env() -> Self {
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

    /// 保存配置到应用数据目录
    pub fn save(&self, app_handle: &tauri::AppHandle) -> Result<()> {
        let config_path = Self::get_config_path(app_handle)?;

        // 确保父目录存在
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// 获取配置文件路径
    fn get_config_path(app_handle: &tauri::AppHandle) -> Result<PathBuf> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| anyhow::anyhow!("Failed to get app data dir: {}", e))?;
        Ok(app_data_dir.join("config.json"))
    }

    /// 获取数据库路径（生产环境下使用应用数据目录）
    #[allow(unused_variables)]
    pub fn get_database_path(&self, app_handle: Option<&tauri::AppHandle>) -> String {
        #[cfg(not(debug_assertions))]
        {
            if let Some(handle) = app_handle {
                if let Ok(app_data_dir) = handle.path().app_data_dir() {
                    return app_data_dir.join("app.db").to_string_lossy().to_string();
                }
            }
        }

        self.database_url.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "./app.db".into(),
            host: "127.0.0.1".into(),
            port: 3000,
            // 生产环境首次启动时生成随机密钥
            jwt_secret: uuid::Uuid::new_v4().to_string(),
            jwt_ttl: 7 * 24 * 3600,
        }
    }
}
