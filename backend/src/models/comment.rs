use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::routes::comments::CommentIncome;

#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct Comment {
    pub comment_id: String,
    pub article_id: Option<String>,
    pub user: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<String>,
}

/// 获取评论
pub async fn fetch_comments_by_article_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Vec<Comment>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Comment,
        r#"
            SELECT comment_id,
                article_id ,
                user,
                content ,
                created_at 
            FROM comments
            WHERE article_id = $1
        "#,
        id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("DB error: {:?}", e);
        e
    })?;

    Ok(rows)
}

/// 发布评论
pub async fn post_comment_by_article_id(
    pool: &SqlitePool,
    new: CommentIncome,
    username: &str,
) -> Result<Comment, sqlx::Error> {
    let c_id = Uuid::now_v7().to_string();
    let now = Local::now();
    let create_at = now.format("%Y::%m::%d").to_string();

    let res: Comment = sqlx::query_as::<_, Comment>(
        r#"INSERT INTO comments 
        (comment_id, article_id, user, content, created_at) 
        VALUES (?,?,?,?,?) 
        RETURNING *"#,
    )
    .bind(c_id)
    .bind(new.article_id)
    .bind(username)
    .bind(new.content)
    .bind(create_at)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        eprintln!("DB error: {:?}", e);
        e
    })?;

    Ok(res)
}

/// 删除评论
pub async fn delete_comment_by_comment_id(
    pool: &SqlitePool,
    comment_id: &str,
) -> Result<String, sqlx::Error> {
    // todo 待施工；目前无法删除

    println!("{}", comment_id);

    let _ = sqlx::query!(r#"DELETE FROM comments WHERE comment_id = ?"#, comment_id)
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("db delete error: {:?}", e);
            e
        })?;

    Ok("done".to_string())
}
