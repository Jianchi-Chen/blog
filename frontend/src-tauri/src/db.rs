//! 数据库模块：提供连接池、迁移与全局状态。
//! 说明：将 `pool` 和 `config` 放入 `AppState`，方便在 handler 与提取器中访问。

use crate::config::Config;
use anyhow::Ok;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::env;
use std::path::Path;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub cfg: Config,
}

impl AppState {
    pub fn new(pool: SqlitePool, cfg: Config) -> Self {
        Self { pool, cfg }
    }
}

// 连接数据库
pub async fn new_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let db_file = database_url.strip_prefix("sqlite://").unwrap();
    let db_path = env::current_dir()?.join(db_file);

    SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(
            SqliteConnectOptions::new()
                .filename(&db_path)
                .create_if_missing(true),
        )
        .await
}

/// 执行迁移（embed 方式，编译期打包）
pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;

    run_seeds(pool, Path::new("./seeds")).await?;
    Ok(())
}

/// 执行 seeds 目录下的所有 .sql 文件
async fn run_seeds(pool: &SqlitePool, dir: &Path) -> anyhow::Result<()> {
    // 1. 收集所有 .sql 文件并按文件名排序, 异步环境下使用tokio::fs
    let mut entries = tokio::fs::read_dir(dir).await?;
    let mut files = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "sql") {
            files.push(path);
        }
    }
    files.sort();

    // 2. 依次执行
    for file in files {
        let sql = tokio::fs::read_to_string(&file).await?;
        sqlx::query(&sql).execute(pool).await?;
    }
    Ok(())
}
