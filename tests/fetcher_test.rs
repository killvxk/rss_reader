use rss_reader::fetcher::http::fetch_feed;
use rss_reader::fetcher::parser::{parse_feed, ParsedArticle, ParsedFeed};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

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
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(35)))
        .mount(&mock_server)
        .await;

    let url = format!("{}/slow", mock_server.uri());
    let result = fetch_feed(&url).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_parse_rss_feed() {
    let xml = include_str!("fixtures/sample_rss.xml");
    let result = parse_feed(xml);

    assert!(result.is_ok());
    let feed = result.unwrap();
    assert_eq!(feed.title, "Test Feed");
    assert_eq!(feed.articles.len(), 2);
    assert_eq!(feed.articles[0].title, "Test Article 1");
}

#[tokio::test]
async fn test_parse_atom_feed() {
    let xml = include_str!("fixtures/sample_atom.xml");
    let result = parse_feed(xml);

    assert!(result.is_ok());
    let feed = result.unwrap();
    assert_eq!(feed.title, "Test Atom Feed");
    assert_eq!(feed.articles.len(), 1);
}

#[tokio::test]
async fn test_parse_invalid_feed() {
    let result = parse_feed("<invalid>xml</invalid>");
    assert!(result.is_err());
}
