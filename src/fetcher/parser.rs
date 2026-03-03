use feed_rs::parser;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};

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
    let feed = parser::parse(xml.as_bytes())
        .context("Failed to parse feed XML")?;

    let title = feed.title
        .map(|t| t.content)
        .unwrap_or_else(|| "Untitled Feed".to_string());

    let articles = feed.entries
        .into_iter()
        .map(|entry| {
            let title = entry.title
                .map(|t| t.content)
                .unwrap_or_else(|| "Untitled".to_string());

            let link = entry.links
                .first()
                .map(|l| l.href.clone())
                .unwrap_or_default();

            let content = entry.summary
                .map(|s| s.content)
                .or_else(|| entry.content.and_then(|c| c.body));

            let published = entry.published
                .or(entry.updated)
                .unwrap_or_else(Utc::now);

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
