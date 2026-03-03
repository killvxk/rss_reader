use rss_reader::db::create_pool;
use rss_reader::db::feeds::{get_all_feeds, get_feed_by_url, insert_feed};

#[tokio::test]
async fn test_insert_and_query_feed() {
    let pool = create_pool(":memory:").await.unwrap();

    // 插入 feed
    let feed_id = insert_feed(
        &pool,
        "Hacker News",
        "https://news.ycombinator.com/rss",
        "tech",
    )
    .await
    .unwrap();

    assert!(feed_id > 0);

    // 查询所有 feeds
    let feeds = get_all_feeds(&pool).await.unwrap();
    assert_eq!(feeds.len(), 1);
    assert_eq!(feeds[0].title, "Hacker News");

    // 按 URL 查询
    let feed = get_feed_by_url(&pool, "https://news.ycombinator.com/rss")
        .await
        .unwrap();
    assert!(feed.is_some());
    assert_eq!(feed.unwrap().title, "Hacker News");
}
