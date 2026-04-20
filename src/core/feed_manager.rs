use crate::db::{articles, feeds, schema::Feed};
use crate::fetcher::{http::fetch_feed, parser::parse_feed};
use anyhow::Result;
use futures::stream::{FuturesUnordered, StreamExt};
use sqlx::SqlitePool;
use std::time::Duration;

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
        self.fetch_all_feeds_with_timeout(Duration::from_secs(30))
            .await
    }

    async fn fetch_all_feeds_with_timeout(&self, timeout_duration: Duration) -> Vec<Result<usize>> {
        let feeds = match self.get_all_feeds().await {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to get feeds: {}", e);
                return vec![];
            }
        };

        let feed_count = feeds.len();
        let mut results: Vec<Option<Result<usize>>> =
            std::iter::repeat_with(|| None).take(feed_count).collect();
        let mut tasks: FuturesUnordered<_> = feeds
            .into_iter()
            .enumerate()
            .map(|(index, feed)| {
                let pool = self.pool.clone();
                async move { (index, Self::fetch_single_feed(pool, feed).await) }
            })
            .collect();
        let deadline = tokio::time::Instant::now() + timeout_duration;

        while !tasks.is_empty() {
            let now = tokio::time::Instant::now();
            if now >= deadline {
                break;
            }

            match tokio::time::timeout(deadline - now, tasks.next()).await {
                Ok(Some((index, fetch_result))) => results[index] = Some(fetch_result),
                Ok(None) => break,
                Err(_) => break,
            }
        }

        let pending = results.iter().filter(|result| result.is_none()).count();
        if pending > 0 {
            tracing::error!(
                "Fetch all feeds timeout with {} feeds still pending",
                pending
            );
        }

        results
            .into_iter()
            .map(|result| {
                result.unwrap_or_else(|| {
                    Err(anyhow::anyhow!(
                        "Feed fetch timed out after {:?}",
                        timeout_duration
                    ))
                })
            })
            .collect()
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
            )
            .await
            {
                Ok(id) if id > 0 => inserted_count += 1,
                Ok(_) => {} // 文章已存在
                Err(e) => tracing::warn!("Failed to insert article: {}", e),
            }
        }

        // 更新拉取时间
        feeds::update_feed_fetch_time(&pool, feed.id).await?;

        tracing::info!(
            "Feed {} fetched: {} new articles",
            feed.title,
            inserted_count
        );
        Ok(inserted_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_feed_manager_basic() {
        let pool = create_pool(":memory:").await.unwrap();
        let manager = FeedManager::new(pool);

        let id = manager
            .add_feed("Test", "http://test.com", "test")
            .await
            .unwrap();
        assert!(id > 0);

        let feeds = manager.get_all_feeds().await.unwrap();
        assert_eq!(feeds.len(), 1);
    }

    #[tokio::test]
    async fn test_fetch_all_feeds_preserves_completed_results_on_timeout() {
        let pool = create_pool(":memory:").await.unwrap();
        let mock_server = MockServer::start().await;
        let fast_xml = r#"<?xml version="1.0"?>
        <rss version="2.0">
          <channel>
            <title>Test Feed</title>
            <item>
              <title>Fast Item</title>
              <link>https://example.com/fast</link>
              <description>Example content</description>
              <pubDate>Mon, 20 Apr 2026 00:00:00 GMT</pubDate>
            </item>
          </channel>
        </rss>"#;
        let slow_xml = r#"<?xml version="1.0"?>
        <rss version="2.0">
          <channel>
            <title>Test Feed</title>
            <item>
              <title>Slow Item</title>
              <link>https://example.com/slow</link>
              <description>Example content</description>
              <pubDate>Mon, 20 Apr 2026 00:00:00 GMT</pubDate>
            </item>
          </channel>
        </rss>"#;

        Mock::given(method("GET"))
            .and(path("/fast.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fast_xml))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/slow.xml"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(2))
                    .set_body_string(slow_xml),
            )
            .mount(&mock_server)
            .await;

        let manager = FeedManager::new(pool.clone());
        let fast_url = format!("{}/fast.xml", mock_server.uri());
        let slow_url = format!("{}/slow.xml", mock_server.uri());

        manager.add_feed("Fast", &fast_url, "test").await.unwrap();
        manager.add_feed("Slow", &slow_url, "test").await.unwrap();

        let results = manager
            .fetch_all_feeds_with_timeout(Duration::from_millis(250))
            .await;

        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
        assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
        assert_eq!(results.iter().filter(|r| r.is_err()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter_map(|r| r.as_ref().ok())
                .sum::<usize>(),
            1
        );

        let fast_feed = feeds::get_feed_by_url(&pool, &fast_url)
            .await
            .unwrap()
            .unwrap();
        let slow_feed = feeds::get_feed_by_url(&pool, &slow_url)
            .await
            .unwrap()
            .unwrap();
        assert!(fast_feed.last_fetched.is_some());
        assert!(slow_feed.last_fetched.is_none());

        let article_count = articles::get_all_articles(&pool, 10, 0)
            .await
            .unwrap()
            .len();
        assert_eq!(article_count, 1);
    }
}
