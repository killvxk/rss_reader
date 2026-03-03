use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use feed_rs::parser;
use regex::Regex;

/// 清理 HTML 标签，转换为纯文本
fn clean_html(html: &str) -> String {
    // 先用 html2text 转换
    let text = html2text::from_read(html.as_bytes(), 1000).unwrap_or_else(|_| html.to_string());

    // 再用正则表达式移除残留的 HTML 标签
    let re = Regex::new(r"<[^>]*>").unwrap();
    let cleaned = re.replace_all(&text, "");

    // 清理多余的空白字符
    let re_whitespace = Regex::new(r"\s+").unwrap();
    re_whitespace.replace_all(&cleaned, " ").trim().to_string()
}

#[derive(Debug, Clone)]
pub struct ParsedFeed {
    pub title: String,
    pub articles: Vec<ParsedArticle>,
}

#[derive(Debug, Clone)]
pub struct ParsedArticle {
    pub title: String,
    pub link: String,
    pub content: Option<String>,
    pub published: DateTime<Utc>,
}

pub fn parse_feed(xml: &str) -> Result<ParsedFeed> {
    let feed = parser::parse(xml.as_bytes()).context("Failed to parse feed XML")?;

    let title = feed
        .title
        .map(|t| t.content)
        .unwrap_or_else(|| "Untitled Feed".to_string());

    let articles = feed
        .entries
        .into_iter()
        .map(|entry| {
            let title = entry
                .title
                .map(|t| clean_html(&t.content))
                .unwrap_or_else(|| "Untitled".to_string());

            let link = entry
                .links
                .first()
                .map(|l| l.href.clone())
                .unwrap_or_default();

            let content = entry
                .summary
                .map(|s| clean_html(&s.content))
                .or_else(|| entry.content.and_then(|c| c.body.map(|b| clean_html(&b))));

            let published = entry.published.or(entry.updated).unwrap_or_else(Utc::now);

            ParsedArticle {
                title,
                link,
                content,
                published,
            }
        })
        .collect();

    Ok(ParsedFeed { title, articles })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_feed() {
        let xml = r#"<?xml version="1.0"?>
        <rss version="2.0">
          <channel>
            <title>Empty Feed</title>
          </channel>
        </rss>"#;

        let result = parse_feed(xml);
        assert!(result.is_ok());
        let feed = result.unwrap();
        assert_eq!(feed.articles.len(), 0);
    }
}
