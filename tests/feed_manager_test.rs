use rss_reader::core::feed_manager::FeedManager;
use rss_reader::db::create_pool;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_fetch_all_feeds() {
    let pool = create_pool(":memory:").await.unwrap();
    let mock_server = MockServer::start().await;

    let xml = include_str!("fixtures/sample_rss.xml");

    Mock::given(method("GET"))
        .and(path("/feed1.xml"))
        .respond_with(ResponseTemplate::new(200).set_body_string(xml))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/feed2.xml"))
        .respond_with(ResponseTemplate::new(200).set_body_string(xml))
        .mount(&mock_server)
        .await;

    let manager = FeedManager::new(pool.clone());

    // 添加测试 feeds
    let url1 = format!("{}/feed1.xml", mock_server.uri());
    let url2 = format!("{}/feed2.xml", mock_server.uri());

    manager.add_feed("Feed 1", &url1, "test").await.unwrap();
    manager.add_feed("Feed 2", &url2, "test").await.unwrap();

    // 并发拉取
    let results = manager.fetch_all_feeds().await;

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.is_ok()));
}
