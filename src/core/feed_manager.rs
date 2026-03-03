use sqlx::SqlitePool;
use anyhow::Result;
use tokio::task::JoinHandle;
use std::time::Duration;
use crate::db::{feeds, articles, schema::Feed};
use crate::fetcher::{http::fetch_feed, parser::parse_feed};

pub struct FeedManager {
    pool: SqlitePool,
}

impl FeedManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_feed(&self, title: &str, url: &str, category: &str) -> Result<i64> {
        feeds::insert_feed(&self.pool, title, url, category).await
    }

    pub async fn get_all_feeds(&self) -> Result<Vec<Feed>> {
        feeds::get_all_feeds(&self.pool).await
    }

    pub async fn fetch_all_feeds(&self) -> Vec<Result<usize>> {
        let feeds = match self.get_all_feeds().await {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to get feeds: {}", e);
                return vec![];
            }
        };

        let tasks: Vec<JoinHandle<Result<usize>>> = feeds
            .into_iter()
            .map(|feed| {
                let pool = self.pool.clone();
                tokio::spawn(async move {
                    Self::fetch_single_feed(pool, feed).await
                })
            })
            .collect();

        // 等待所有任务完成，超时 30 秒
        let timeout_duration = Duration::from_secs(30);
        let results = tokio::time::timeout(
            timeout_duration,
            futures::future::join_all(tasks)
        ).await;

        match results {
            Ok(results) => results.into_iter().map(|r| r.unwrap_or_else(|e| {
                Err(anyhow::anyhow!("Task join error: {}", e))
            })).collect(),
            Err(_) => {
                tracing::error!("Fetch all feeds timeout");
                vec![]
            }
        }
    }

    async fn fetch_single_feed(pool: SqlitePool, feed: Feed) -> Result<usize> {
        tracing::info!("Fetching feed: {} ({})", feed.title, feed.url);

        // 拉取 feed
        let xml = match fetch_feed(&feed.url).await {
            Ok(xml) => xml,
            Err(e) => {
                let error_msg = format!("Fetch error: {}", e);
                tracing::error!("{}", error_msg);
                feeds::update_feed_error(&pool, feed.id, &error_msg).await?;
                return Err(e);
            }
        };

        // 解析 feed
        let parsed = match parse_feed(&xml) {
            Ok(p) => p,
            Err(e) => {
                let error_msg = format!("Parse error: {}", e);
                tracing::error!("{}", error_msg);
                feeds::update_feed_error(&pool, feed.id, &error_msg).await?;
                return Err(e);
            }
        };

        // 插入文章
        let mut inserted_count = 0;
        for article in parsed.articles {
            match articles::insert_article(
                &pool,
                feed.id,
                &article.title,
                &article.link,
                article.content.as_deref(),
                article.published,
            ).await {
                Ok(id) if id > 0 => inserted_count += 1,
                Ok(_) => {}, // 文章已存在
                Err(e) => tracing::warn!("Failed to insert article: {}", e),
            }
        }

        // 更新拉取时间
        feeds::update_feed_fetch_time(&pool, feed.id).await?;

        tracing::info!("Feed {} fetched: {} new articles", feed.title, inserted_count);
        Ok(inserted_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;

    #[tokio::test]
    async fn test_feed_manager_basic() {
        let pool = create_pool(":memory:").await.unwrap();
        let manager = FeedManager::new(pool);

        let id = manager.add_feed("Test", "http://test.com", "test")
            .await
            .unwrap();
        assert!(id > 0);

        let feeds = manager.get_all_feeds().await.unwrap();
        assert_eq!(feeds.len(), 1);
    }
}
