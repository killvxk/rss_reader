pub mod articles;
pub mod feeds;
pub mod schema;
pub mod tags;

use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    // 确保数据库文件会被自动创建
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("{}?mode=rwc", database_url))
        .await?;

    // 运行迁移
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let pool = create_pool(":memory:").await.unwrap();
        assert!(!pool.is_closed());
    }
}
