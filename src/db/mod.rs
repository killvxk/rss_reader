pub mod schema;
pub mod feeds;
pub mod articles;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use anyhow::Result;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // 运行迁移
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let pool = create_pool(":memory:").await.unwrap();
        assert!(pool.is_closed() == false);
    }
}
