use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct TmpSuggest {
    pub title: Option<String>,
    pub id: Option<String>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct SuggestRespond {
    pub item: Vec<TmpSuggest>,
}
