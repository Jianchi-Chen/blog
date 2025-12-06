//! Search 模型定义

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct TmpSuggest {
    pub title: Option<String>,
    pub id: Option<String>,
}
