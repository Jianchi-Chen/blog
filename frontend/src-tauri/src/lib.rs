//! Tauri 应用库入口

pub mod auth;
pub mod commands;
pub mod config;
pub mod db;
pub mod models;
pub mod repositories;

use config::Config;
use db::{new_pool, run_migrations};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // 日志插件（仅开发模式）
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 加载配置
            let config = Config::load(Some(&app.handle())).expect("Failed to load configuration");

            log::info!("Application started with config: {:?}", config);

            // 初始化数据库（异步操作需要在 tokio runtime 中执行）
            let database_url = config.get_database_path(Some(&app.handle()));
            let pool = tauri::async_runtime::block_on(async {
                let pool = new_pool(&database_url)
                    .await
                    .expect("Failed to create database pool");
                run_migrations(&pool)
                    .await
                    .expect("Failed to run migrations");
                pool
            });

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
