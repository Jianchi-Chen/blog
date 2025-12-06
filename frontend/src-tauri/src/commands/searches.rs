//! 搜索相关命令

use crate::repositories::search::*;
use sqlx::SqlitePool;
use tauri::State;

/// 获取搜索建议
#[tauri::command]
pub async fn get_suggestions(
    keyword: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<TmpSuggest>, String> {
    let suggestions = get_suggests_by_keyword(pool.inner(), &keyword)
        .await
        .map_err(|e| format!("Failed to fetch suggestions: {}", e))?;

    Ok(suggestions)
}
