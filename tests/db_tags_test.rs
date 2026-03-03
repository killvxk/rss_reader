use chrono::Utc;
use rss_reader::db::tags::{
    add_tag_to_article, create_tag, get_all_tags, get_articles_by_tag, get_tags_for_article,
};
use rss_reader::db::{articles::insert_article, create_pool, feeds::insert_feed};
use sqlx::SqlitePool;

#[tokio::test]
async fn test_tag_operations() {
    let pool = create_pool(":memory:").await.unwrap();

    // 创建标签
    let tag_id = create_tag(&pool, "rust").await.unwrap();
    assert!(tag_id > 0);

    // 查询所有标签
    let tags = get_all_tags(&pool).await.unwrap();
    assert_eq!(tags.len(), 1);

    // 创建文章
    let feed_id = insert_feed(&pool, "Test", "http://test.com", "test")
        .await
        .unwrap();
    let article_id = insert_article(
        &pool,
        feed_id,
        "Rust Article",
        "http://test.com/1",
        None,
        Utc::now(),
    )
    .await
    .unwrap();

    // 给文章添加标签
    add_tag_to_article(&pool, article_id, tag_id).await.unwrap();

    // 查询文章的标签
    let article_tags = get_tags_for_article(&pool, article_id).await.unwrap();
    assert_eq!(article_tags.len(), 1);

    // 按标签查询文章
    let articles = get_articles_by_tag(&pool, tag_id, 10, 0).await.unwrap();
    assert_eq!(articles.len(), 1);
}
