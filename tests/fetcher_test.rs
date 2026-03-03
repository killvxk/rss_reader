use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use rss_reader::fetcher::http::fetch_feed;

#[tokio::test]
async fn test_fetch_feed_success() {
    // 启动 mock server
    let mock_server = MockServer::start().await;

    // 读取测试 fixture
    let xml_content = include_str!("fixtures/sample_rss.xml");

    // 配置 mock 响应
    Mock::given(method("GET"))
        .and(path("/feed.xml"))
        .respond_with(ResponseTemplate::new(200).set_body_string(xml_content))
        .mount(&mock_server)
        .await;

    // 测试拉取
    let url = format!("{}/feed.xml", mock_server.uri());
    let result = fetch_feed(&url).await;

    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.contains("<title>Test Feed</title>"));
}

#[tokio::test]
async fn test_fetch_feed_timeout() {
    use std::time::Duration;

    let mock_server = MockServer::start().await;

    // 配置延迟响应（超过超时时间）
    Mock::given(method("GET"))
        .and(path("/slow"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_secs(35))
        )
        .mount(&mock_server)
        .await;

    let url = format!("{}/slow", mock_server.uri());
    let result = fetch_feed(&url).await;

    assert!(result.is_err());
}
