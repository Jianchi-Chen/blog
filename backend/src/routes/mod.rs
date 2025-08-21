//! routes/ HTTP 入口，前端直接访问，负责：
//! 触发中间件（JWT、限流、日志）
//! 参数校验
//! 调 models 里的 DB 函数
//! 把结果包装成 JSON 返回给前端

pub mod auth;
pub mod health;
pub mod users;

// 路由聚合：
// - 将各路由模块组合在一起
// - 添加全局中间件（例如 CORS、Trace）
// - 将全局状态 `AppState` 注入，供提取器与 handler 使用

use crate::db::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    // 浏览器跨域请求，一律放行，不拦。
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 路由只负责匹配路径和方法，参数由框架自动提取
    let api = Router::new()
        .route("/health", get(health::health))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    // 返回路由
    Router::new()
        .merge(api)
        .layer(TraceLayer::new_for_http()) // “监控膜”: 每个请求进来/出去时，自动打印日志。
        .layer(cors) // 跨域通行证
        .with_state(state)
}
