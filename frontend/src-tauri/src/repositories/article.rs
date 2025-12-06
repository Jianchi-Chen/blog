use crate::models::article::{ArticleModel, GetArticlesParams, NewArticle, NewStatus, PubArticles};
use anyhow::Result;
use chrono::Local;
use reqwest::StatusCode;
use sqlx::SqlitePool;
use uuid::Uuid;

/// 获取文章列表
pub async fn get_articles(
    pool: &SqlitePool,
    params: GetArticlesParams,
) -> Result<Vec<PubArticles>> {
    let rows = if params.identity == "admin" {
        sqlx::query_as::<_, PubArticles>(
            r#"
            SELECT id, title, summary, created_at, status, views, tags FROM articles
            "#,
        )
        .fetch_all(pool)
        .await
        .inspect_err(|e| {
            eprintln!("DB error: {:?}", e);
        })?
    } else {
        sqlx::query_as::<_, PubArticles>(
            r#"
            SELECT id, title, summary, created_at, status, views, tags FROM articles
            WHERE status = ?
            "#,
        )
        .bind("published") // 需要uuid的feature
        .fetch_all(pool)
        .await
        .inspect_err(|e| {
            eprintln!("DB error: {:?}", e);
        })?
    };

    // 再进行一次关键词搜索
    if let Some(keyword) = params.condition {
        let filtered_rows: Vec<PubArticles> = rows
            .into_iter()
            .filter(|c| c.title.contains(&keyword))
            .collect();
        return Ok(filtered_rows);
    }

    Ok(rows)
}

/// 新增文章
pub async fn post_article(pool: &SqlitePool, new: &NewArticle) -> Result<ArticleModel> {
    let id = Uuid::now_v7().to_string();
    let now = Local::now();
    let create_at = now.format("%Y::%m::%d").to_string();
    let status = "draft".to_string();

    sqlx::query!(
        r#"
        INSERT INTO articles (id, title, content, summary, created_at, status, tags)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        new.title,
        new.content,
        new.summary,
        create_at,
        status,
        new.tags
    )
    .execute(pool) //执行语句，并 等待一行结果
    .await
    .map_err(|e| {
        eprintln!("DB error: {:?}", e);
        anyhow::anyhow!(e)
    })?;

    sqlx::query_as::<_, ArticleModel>(r#"SELECT * FROM articles WHERE id = ?"#)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("DB error: {:?}", e);
            anyhow::anyhow!(e)
        })
}

// 一般函数能给&str给&str，需要所有权时才给String
/// 查找文章
pub async fn find_article_by_id(pool: &SqlitePool, id: &str) -> Result<Option<ArticleModel>> {
    sqlx::query_as::<_, ArticleModel>(r#"select * from articles where id = ?"#)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("DB error: {:?}", e);
            e.into()
        })
}

/// 删除文章
pub async fn delete_article_by_id(pool: &SqlitePool, id: &str) -> Result<StatusCode> {
    // 使用query / execute 代替 query_as::<> / fetch_*
    let res = sqlx::query(r#"DELETE FROM articles where id = ?"#)
        .bind(id)
        .execute(pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(anyhow::anyhow!("未找到对应文章"));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// 修改文章
pub async fn put_article_by_id(
    pool: &SqlitePool,
    id: &str,
    new: NewArticle,
) -> Result<ArticleModel> {
    let now = Local::now();
    let update_at = now.format("%Y::%m::%d").to_string();

    // 不带宏是运行期再检查
    // 使用宏是编译期检查
    let _ = sqlx::query!(
        r#"
            UPDATE articles
            SET title = ?,
                content = ?,
                summary = ?,
                update_at = ?,
                tags = ?,
                status = ?
            WHERE id = ?
    "#,
        new.title,
        new.content,
        new.summary,
        update_at,
        new.tags,
        "draft",
        id
    )
    .execute(pool)
    .await?;

    Ok(
        sqlx::query_as::<_, ArticleModel>(r#"SELECT * FROM articles WHERE id = ?"#)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                eprintln!("DB select error: {:?}", e);
                e
            })?,
    )
}

/// 更变文章状态
pub async fn patch_article_by_id(
    pool: &SqlitePool,
    id: &str,
    new: NewStatus,
) -> Result<ArticleModel> {
    let _ = sqlx::query(
        r#"
        UPDATE articles SET status = ?
        WHERE id = ?
        "#,
    )
    .bind(&new.toggle)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| {
        eprintln!("DB update error: {:?}", e);
        e
    })?;

    Ok(
        sqlx::query_as::<_, ArticleModel>(r#"SELECT * FROM articles WHERE id = ?"#)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                eprintln!("DB select error: {:?}", e);
                e
            })?,
    )
}
