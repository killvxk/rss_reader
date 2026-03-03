use sqlx::SqlitePool;
use anyhow::Result;
use chrono::Utc;
use super::schema::Feed;

pub async fn insert_feed(
    pool: &SqlitePool,
    title: &str,
    url: &str,
    category: &str,
) -> Result<i64> {
    let result = sqlx::query(
        "INSERT INTO feeds (title, url, category) VALUES (?, ?, ?)"
    )
    .bind(title)
    .bind(url)
    .bind(category)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_all_feeds(pool: &SqlitePool) -> Result<Vec<Feed>> {
    let feeds = sqlx::query_as::<_, Feed>(
        "SELECT id, title, url, category, last_fetched, fetch_error FROM feeds ORDER BY category, title"
    )
    .fetch_all(pool)
    .await?;

    Ok(feeds)
}

pub async fn get_feed_by_url(pool: &SqlitePool, url: &str) -> Result<Option<Feed>> {
    let feed = sqlx::query_as::<_, Feed>(
        "SELECT id, title, url, category, last_fetched, fetch_error FROM feeds WHERE url = ?"
    )
    .bind(url)
    .fetch_optional(pool)
    .await?;

    Ok(feed)
}

pub async fn update_feed_fetch_time(pool: &SqlitePool, feed_id: i64) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE feeds SET last_fetched = ?, fetch_error = NULL WHERE id = ?"
    )
    .bind(now)
    .bind(feed_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_feed_error(pool: &SqlitePool, feed_id: i64, error: &str) -> Result<()> {
    sqlx::query(
        "UPDATE feeds SET fetch_error = ? WHERE id = ?"
    )
    .bind(error)
    .bind(feed_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;

    #[tokio::test]
    async fn test_feed_operations() {
        let pool = create_pool(":memory:").await.unwrap();

        // 插入
        let id = insert_feed(&pool, "Test", "http://test.com/rss", "test")
            .await
            .unwrap();
        assert!(id > 0);

        // 查询
        let feeds = get_all_feeds(&pool).await.unwrap();
        assert_eq!(feeds.len(), 1);

        // 更新时间
        update_feed_fetch_time(&pool, id).await.unwrap();
        let feed = get_feed_by_url(&pool, "http://test.com/rss")
            .await
            .unwrap()
            .unwrap();
        assert!(feed.last_fetched.is_some());
    }
}
