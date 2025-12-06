//! 用户管理相关命令

use crate::auth::{decode_token, hash_password};
use crate::config::Config;
use crate::models::user::UserPublic;
use crate::repositories::user::{delete_user_by_id, edit_user_account, get_ident_by_id, list_users, AdminEditAccountPayload};
use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

#[derive(Serialize)]
pub struct ListUsersResponse {
    users: Vec<UserPublic>,
}

/// 获取用户列表（需要管理员权限）
#[tauri::command]
pub async fn get_users(
    token: String,
    limit: Option<i32>,
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<ListUsersResponse, String> {
    // 验证 token
    let claims = decode_token(&config, &token).map_err(|e| format!("Invalid token: {}", e))?;

    // 检查用户身份
    let identity = get_ident_by_id(pool.inner(), &claims.user_id)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if identity != "admin" {
        return Err("Only admin can view users".to_string());
    }

    let default_limit = limit.unwrap_or(10);
    let users = list_users(pool.inner(), default_limit)
        .await
        .map_err(|e| format!("Failed to fetch users: {}", e))?;

    Ok(ListUsersResponse { users })
}

/// 删除用户（需要管理员权限）
#[tauri::command]
pub async fn delete_user(
    token: String,
    user_id: String,
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<(), String> {
    // 验证 token
    let claims = decode_token(&config, &token).map_err(|e| format!("Invalid token: {}", e))?;

    // 检查用户身份
    let identity = get_ident_by_id(pool.inner(), &claims.user_id)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if identity != "admin" {
        return Err("Only admin can delete users".to_string());
    }

    delete_user_by_id(pool.inner(), &user_id)
        .await
        .map_err(|e| format!("Failed to delete user: {}", e))?;

    Ok(())
}

/// 编辑用户账号（仅管理员可用）
#[tauri::command]
pub async fn edit_account(
    token: String,
    payload: AdminEditAccountPayload,
    pool: State<'_, SqlitePool>,
    config: State<'_, Config>,
) -> Result<(), String> {
    // 验证 token
    let claims = decode_token(&config, &token).map_err(|e| format!("Invalid token: {}", e))?;

    // 检查用户身份
    let identity = get_ident_by_id(pool.inner(), &claims.user_id)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    if identity != "admin" {
        return Err("Only admin can edit user accounts".to_string());
    }

    // 防止更改超管的权限
    if payload.edited_identity.as_ref().map(|s| s.as_str()) == Some("admin")
        && payload.edited_id == "1"
    {
        return Err("Cannot change superadmin identity".to_string());
    }

    // 将传递的密码转为 hash
    let new_password = if let Some(ref password) = payload.edited_password {
        if !password.is_empty() {
            Some(
                hash_password(password)
                    .map_err(|e| format!("Failed to hash password: {}", e))?,
            )
        } else {
            None
        }
    } else {
        None
    };

    let updated_payload = AdminEditAccountPayload {
        edited_id: payload.edited_id,
        edited_username: payload.edited_username,
        edited_password: new_password,
        edited_identity: payload.edited_identity,
    };

    edit_user_account(pool.inner(), updated_payload)
        .await
        .map_err(|e| format!("Failed to edit account: {}", e))?;

    Ok(())
}
