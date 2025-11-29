//! routes/ HTTP 入口，前端直接访问，负责：
//! 触发中间件（JWT、限流、日志）
//! 参数校验
//! 调 models 里的 DB 函数
//! 把结果包装成 JSON 返回给前端

pub mod articles;
pub mod auth;
pub mod comments;
pub mod health;
pub mod searches;
pub mod users;

// 路由聚合：
// - 将各路由模块组合在一起
// - 添加全局中间件（例如 CORS、Trace）
// - 将全局状态 `AppState` 注入，供提取器与 handler 使用

use crate::db::AppState;
use axum::{
    Router,
    routing::{delete, get, patch, post, put},
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
        .route("/api/register", post(auth::register))
        .route("/api/login", post(auth::login))
        .route("/api/users", get(users::get_users)) // debug route
        .route("/api/users/{user_id}", delete(users::delete_users))
        .route("/api/editAccount", put(users::edit_account))
        // articles
        .route("/articles", get(articles::articles))
        .route("/api/article", post(articles::handle_post_article))
        .route("/article/{id}", get(articles::handle_get_article))
        .route("/api/article/{id}", delete(articles::handle_delete_article))
        .route("/api/article/{id}", put(articles::handle_put_article))
        .route("/api/article/{id}", patch(articles::handle_patch_article))
        // comments
        .route("/comments/{id}", get(comments::handle_get_comments))
        .route("/api/comment", post(comments::handle_post_comment))
        .route(
            "/comment/{comment_id}",
            delete(comments::handle_delete_comment),
        )
        .route("/api/comment/like", put(comments::like_comment))
        // .route("/api/comments/like", get(comments::get_comments_like))
        // searches
        .route(
            "/suggestions/{keyword}",
            get(searches::handle_suggests_by_keys),
        )
        .with_state(state.clone());

    // 返回路由
    Router::new()
        .merge(api)
        .layer(TraceLayer::new_for_http()) // “监控膜”: 每个请求进来/出去时，自动打印日志。
        .layer(cors) // 跨域通行证
}
