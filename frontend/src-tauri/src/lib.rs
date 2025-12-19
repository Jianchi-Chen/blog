//! Tauri 应用库入口

pub mod auth;
pub mod commands;
pub mod config;
pub mod db;
pub mod models;
pub mod repositories;
pub mod tray;

use crate::tray::load_system_tray;
use config::Config;
use db::{new_pool, run_migrations, run_seeds};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            // 加载系统托盘
            load_system_tray(app)?;

            // 自动更新插件
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;

            // 加载配置
            let config = match Config::load(&app.handle()) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Failed to load configuration: {}", e);
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Configuration error: {}", e),
                    )));
                }
            };

            log::info!("Application started with config: {:?}", config);

            // 初始化数据库
            let database_url = config.get_database_path(&app.handle());
            log::info!("Database path: {}", database_url);

            let pool = match tauri::async_runtime::block_on(async {
                let pool = new_pool(&database_url).await?;
                run_migrations(&pool).await?;

                // 运行种子数据（仅在开发模式或首次运行时）
                if let Err(e) = run_seeds(&pool).await {
                    log::warn!("Failed to run seeds: {}", e);
                }

                Ok::<_, Box<dyn std::error::Error>>(pool)
            }) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Database initialization failed: {}", e);
                    log::error!("Database initialization failed: {}", e);
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Database error: {}", e),
                    )));
                }
            };

            log::info!("Database initialized successfully");

            // 将配置和连接池存储到状态中
            app.manage(config);
            app.manage(pool);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // HTTP 请求代理
            commands::http_request,
            // 认证
            commands::login,
            commands::register,
            commands::verify_token,
            commands::get_current_user,
            // 用户管理
            commands::get_users,
            commands::delete_user,
            commands::edit_account,
            // 文章
            commands::get_articles,
            commands::get_article_by_id,
            commands::create_article,
            commands::update_article,
            commands::delete_article,
            commands::toggle_article_status,
            // 评论
            commands::get_comments,
            commands::post_comment,
            commands::delete_comment,
            commands::like_comment,
            // 搜索
            commands::get_suggestions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
