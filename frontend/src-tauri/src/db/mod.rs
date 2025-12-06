use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::PathBuf;

/// 创建新的数据库连接池
pub async fn new_pool(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // 处理绝对路径：如果是 sqlite:// 开头，确保数据库文件存在
    if db_url.starts_with("sqlite://") {
        let path = db_url.trim_start_matches("sqlite://");
        let db_path = PathBuf::from(path);
        
        // 确保父目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create database directory: {}", e)
                )))?;
        }
    }

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
}

/// 运行数据库迁移
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
}

/// 运行数据库种子数据（仅开发环境）
#[allow(dead_code)]
pub async fn run_seeds(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let seed_dir = PathBuf::from("./seeds");
    
    if !seed_dir.exists() {
        return Ok(());
    }

    let mut entries = std::fs::read_dir(&seed_dir)
        .map_err(|e| sqlx::Error::Io(e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| sqlx::Error::Io(e))?;
    
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            let sql = std::fs::read_to_string(&path)
                .map_err(|e| sqlx::Error::Io(e))?;
            
            sqlx::raw_sql(&sql).execute(pool).await?;
            
            println!("Executed seed: {}", path.display());
        }
    }

    Ok(())
}
