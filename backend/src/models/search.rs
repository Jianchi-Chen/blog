use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, query_as};


#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct TmpSuggest {
    pub title: String,
}

pub async fn get_suggests_by_keyword(
    pool: &SqlitePool,
    params: &str,
) -> Result<Vec<TmpSuggest>, sqlx::Error> {
    let bind_value = format!("%{}%", params);

    let res = query_as::<_, TmpSuggest>(
        r#"
        SELECT title FROM articles WHERE title LIKE ?
    "#,
    )
    .bind(&bind_value)
    .fetch_all(pool)
    .await?;

    // println!("{}", bind_value);
    // println!("{:?}", res);

    Ok(res)
}
