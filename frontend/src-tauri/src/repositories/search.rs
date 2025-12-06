//! Search Repository - 搜索数据访问层

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct TmpSuggest {
    pub title: Option<String>,
    pub id: Option<String>,
}

/// 根据关键词获取搜索建议
pub async fn get_suggests_by_keyword(
    pool: &SqlitePool,
    keyword: &str,
) -> Result<Vec<TmpSuggest>, sqlx::Error> {
    let bind_value = format!("%{}%", keyword);

    sqlx::query_as::<_, TmpSuggest>(
        r#"
        SELECT id, title 
        FROM articles 
        WHERE status = 'published' AND title LIKE ?
        "#,
    )
    .bind(&bind_value)
    .fetch_all(pool)
    .await
}
