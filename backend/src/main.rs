//! 程序入口：
//! - 加载环境与配置
//! - 初始化日志
//! - 建立数据库连接并执行迁移
//! - 构建路由并启动 HTTP 服务器

mod auth;
mod config;
mod db;
mod error;
mod models;
mod routes;

use crate::config::Config;
use crate::db::{AppState, new_pool, run_migrations};
use crate::routes::create_router;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // 配置“日志收集器”（Subscriber），初始化 tracing 日志（支持 RUST_LOG 环境变量），复用性非常高
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env();
    let pool = new_pool(&cfg.database_url).await?;
    run_migrations(&pool).await?;

    let state = Arc::new(AppState::new(pool, cfg.clone()));

    let app = create_router(state.clone());

    let addr = SocketAddr::new(cfg.host.parse()?, cfg.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("🚀 listening on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
