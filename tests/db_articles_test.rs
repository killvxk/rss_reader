use chrono::Utc;
use rss_reader::db::articles::{
    get_articles_by_feed, get_articles_by_ids, get_unread_count, insert_article, mark_as_read,
    search_articles, toggle_bookmark,
};
use rss_reader::db::{create_pool, feeds::insert_feed};

#[tokio::test]
async fn test_article_operations() {
    let pool = create_pool(":memory:").await.unwrap();

    // 创建 feed
    let feed_id = insert_feed(&pool, "Test Feed", "http://test.com", "test")
        .await
        .unwrap();

    // 插入文章
    let article_id = insert_article(
        &pool,
        feed_id,
        "Test Article",
        "http://test.com/article1",
        Some("Content about Rust programming"),
        Utc::now(),
    )
    .await
    .unwrap();

    assert!(article_id > 0);

    // 查询文章
    let articles = get_articles_by_feed(&pool, feed_id, 10, 0).await.unwrap();
    assert_eq!(articles.len(), 1);

    // 标记已读
    mark_as_read(&pool, article_id, true).await.unwrap();
    let count = get_unread_count(&pool).await.unwrap();
    assert_eq!(count, 0);

    // 切换书签
    toggle_bookmark(&pool, article_id).await.unwrap();

    // 全文搜索
    let results = search_articles(&pool, "Rust", 10, 0).await.unwrap();
    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn test_get_articles_by_ids() {
    let pool = create_pool(":memory:").await.unwrap();

    // 插入测试数据
    let feed_id = insert_feed(&pool, "Test Feed", "https://test.com/rss", "tech")
        .await
        .unwrap();

    let article1_id = insert_article(
        &pool,
        feed_id,
        "Article 1",
        "https://test.com/1",
        Some("Content 1"),
        "2026-03-04T10:00:00Z".parse().unwrap(),
    )
    .await
    .unwrap();

    let article2_id = insert_article(
        &pool,
        feed_id,
        "Article 2",
        "https://test.com/2",
        Some("Content 2"),
        "2026-03-04T11:00:00Z".parse().unwrap(),
    )
    .await
    .unwrap();

    // 测试按 ID 查询
    let articles = get_articles_by_ids(&pool, &[article1_id, article2_id])
        .await
        .unwrap();

    assert_eq!(articles.len(), 2);
    assert_eq!(articles[0].title, "Article 2"); // 按时间倒序
    assert_eq!(articles[1].title, "Article 1");
}
