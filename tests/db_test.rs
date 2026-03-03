use sqlx::SqlitePool;

#[tokio::test]
async fn test_database_schema_creation() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // 执行迁移
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // 验证表存在
    let result: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='feeds'")
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(result.0, 1);
}
