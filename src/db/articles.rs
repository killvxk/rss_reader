use super::schema::Article;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

pub async fn insert_article(
    pool: &SqlitePool,
    feed_id: i64,
    title: &str,
    link: &str,
    content: Option<&str>,
    published: DateTime<Utc>,
) -> Result<i64> {
    let published_str = published.to_rfc3339();
    let result = sqlx::query(
        "INSERT INTO articles (feed_id, title, link, content, published)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT(link) DO NOTHING",
    )
    .bind(feed_id)
    .bind(title)
    .bind(link)
    .bind(content)
    .bind(published_str)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_articles_by_feed(
    pool: &SqlitePool,
    feed_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, feed_id, title, link, content, published,
                is_read, is_bookmarked, created_at
         FROM articles
         WHERE feed_id = ?
         ORDER BY published DESC
         LIMIT ? OFFSET ?",
    )
    .bind(feed_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn get_all_articles(pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<Article>> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, feed_id, title, link, content, published,
                is_read, is_bookmarked, created_at
         FROM articles
         ORDER BY published DESC
         LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn search_articles(
    pool: &SqlitePool,
    query: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT a.id, a.feed_id, a.title, a.link, a.content, a.published,
                a.is_read, a.is_bookmarked, a.created_at
         FROM articles a
         JOIN articles_fts fts ON a.id = fts.rowid
         WHERE articles_fts MATCH ?
         ORDER BY a.published DESC
         LIMIT ? OFFSET ?",
    )
    .bind(query)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn mark_as_read(pool: &SqlitePool, article_id: i64, is_read: bool) -> Result<()> {
    sqlx::query("UPDATE articles SET is_read = ? WHERE id = ?")
        .bind(is_read)
        .bind(article_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn toggle_bookmark(pool: &SqlitePool, article_id: i64) -> Result<()> {
    sqlx::query("UPDATE articles SET is_bookmarked = NOT is_bookmarked WHERE id = ?")
        .bind(article_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_unread_count(pool: &SqlitePool) -> Result<i64> {
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM articles WHERE is_read = 0")
        .fetch_one(pool)
        .await?;

    Ok(result.0)
}

pub async fn get_bookmarked_articles(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, feed_id, title, link, content, published,
                is_read, is_bookmarked, created_at
         FROM articles
         WHERE is_bookmarked = 1
         ORDER BY published DESC
         LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{create_pool, feeds::insert_feed};

    #[tokio::test]
    async fn test_article_crud() {
        let pool = create_pool(":memory:").await.unwrap();
        let feed_id = insert_feed(&pool, "Test", "http://test.com", "test")
            .await
            .unwrap();

        let id = insert_article(
            &pool,
            feed_id,
            "Title",
            "http://test.com/1",
            Some("Content"),
            Utc::now(),
        )
        .await
        .unwrap();

        assert!(id > 0);

        let articles = get_all_articles(&pool, 10, 0).await.unwrap();
        assert_eq!(articles.len(), 1);
    }
}
