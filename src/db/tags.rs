use super::schema::{Article, Tag};
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn create_tag(pool: &SqlitePool, name: &str) -> Result<i64> {
    let result = sqlx::query("INSERT INTO tags (name) VALUES (?) ON CONFLICT(name) DO NOTHING")
        .bind(name)
        .execute(pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(result.last_insert_rowid())
    } else {
        // 标签已存在，查询 ID
        let tag: (i64,) = sqlx::query_as("SELECT id FROM tags WHERE name = ?")
            .bind(name)
            .fetch_one(pool)
            .await?;
        Ok(tag.0)
    }
}

pub async fn get_all_tags(pool: &SqlitePool) -> Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>("SELECT id, name FROM tags ORDER BY name")
        .fetch_all(pool)
        .await?;

    Ok(tags)
}

pub async fn add_tag_to_article(pool: &SqlitePool, article_id: i64, tag_id: i64) -> Result<()> {
    sqlx::query(
        "INSERT INTO article_tags (article_id, tag_id) VALUES (?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(article_id)
    .bind(tag_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_tag_from_article(
    pool: &SqlitePool,
    article_id: i64,
    tag_id: i64,
) -> Result<()> {
    sqlx::query("DELETE FROM article_tags WHERE article_id = ? AND tag_id = ?")
        .bind(article_id)
        .bind(tag_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_tags_for_article(pool: &SqlitePool, article_id: i64) -> Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT t.id, t.name
         FROM tags t
         JOIN article_tags at ON t.id = at.tag_id
         WHERE at.article_id = ?
         ORDER BY t.name",
    )
    .bind(article_id)
    .fetch_all(pool)
    .await?;

    Ok(tags)
}

pub async fn get_articles_by_tag(
    pool: &SqlitePool,
    tag_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT a.id, a.feed_id, a.title, a.link, a.content, a.published,
                a.is_read, a.is_bookmarked, a.created_at
         FROM articles a
         JOIN article_tags at ON a.id = at.article_id
         WHERE at.tag_id = ?
         ORDER BY a.published DESC
         LIMIT ? OFFSET ?",
    )
    .bind(tag_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;

    #[tokio::test]
    async fn test_tag_crud() {
        let pool = create_pool(":memory:").await.unwrap();

        let id = create_tag(&pool, "test").await.unwrap();
        assert!(id > 0);

        let tags = get_all_tags(&pool).await.unwrap();
        assert_eq!(tags.len(), 1);

        // 重复创建应返回相同 ID
        let id2 = create_tag(&pool, "test").await.unwrap();
        assert_eq!(id, id2);
    }
}
