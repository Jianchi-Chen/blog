pub mod auth;
pub mod commands;
pub mod config;
pub mod db;
pub mod models;
pub mod repositories;
pub mod services;

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
            // 通用命令
            commands::http_request,
            // 用户管理
            commands::get_users,
            commands::delete_user,
            commands::edit_account,
            // 文章管理
            // 评论管理
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
