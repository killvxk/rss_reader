use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

pub async fn fetch_feed(url: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("RSS-Reader/0.1.0")
        .build()?;

    let response = client
        .get(url)
        .send()
        .await
        .context(format!("Failed to fetch feed from {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP error: {}", response.status());
    }

    let content = response
        .text()
        .await
        .context("Failed to read response body")?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_invalid_url() {
        let result = fetch_feed("http://invalid-domain-that-does-not-exist-12345.com").await;
        assert!(result.is_err());
    }
}
