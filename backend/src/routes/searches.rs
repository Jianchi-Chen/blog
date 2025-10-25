use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};

use crate::{
    db::AppState,
    error::AppResult,
    models::search::{TmpSuggest, get_suggests_by_keyword},
};

#[derive(Deserialize, Clone, Serialize)]
pub struct SuggestParams {
    pub keyword: String,
    pub limit: Option<String>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct SuggestRespond {
    pub item: Vec<TmpSuggest>,
}

// 获取关键词搜索建议
pub async fn handle_suggests_by_keys(
    State(state): State<Arc<AppState>>,
    Path(keyword): Path<String>,
) -> AppResult<Json<SuggestRespond>> {
    let trim_parmas = keyword.trim_matches('"');
    let item = get_suggests_by_keyword(&state.pool, trim_parmas).await?;

    Ok(Json(SuggestRespond { item }))
}
