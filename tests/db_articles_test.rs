use chrono::Utc;
use rss_reader::db::articles::{
    get_articles_by_feed, get_unread_count, insert_article, mark_as_read, search_articles,
    toggle_bookmark,
};
use rss_reader::db::{create_pool, feeds::insert_feed};
use sqlx::SqlitePool;

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
