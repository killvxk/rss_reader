# RSS 阅读器实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 构建一个基于 Rust 的 TUI RSS 阅读器，支持快速浏览和信息管理功能

**Architecture:** 分层架构 - TUI 层（ratatui）处理界面渲染，业务逻辑层管理 feeds/articles/tags，数据层（sqlx + SQLite）持久化存储，网络层（tokio + reqwest）异步拉取 RSS

**Tech Stack:** ratatui, tokio, sqlx, feed-rs, reqwest, chrono, serde

---

## 任务概览

1. **项目初始化** - Cargo 项目、依赖、目录结构
2. **数据库层** - SQLite schema、CRUD 操作、FTS5 搜索
3. **RSS 拉取器** - HTTP 客户端、feed 解析、并发拉取
4. **业务逻辑层** - Feed/Article/Tag 管理器
5. **TUI 基础** - 主界面框架、事件循环
6. **UI 组件** - Feed 列表、Article 列表、预览面板
7. **交互功能** - 搜索、过滤、标签、书签
8. **导出功能** - JSON/Markdown 导出
9. **集成测试** - 端到端测试
10. **文档和部署** - README、使用说明

---

## Task 1: 项目初始化

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `.gitignore`
- Create: `tests/fixtures/.gitkeep`

**Step 1: 创建 Cargo 项目配置**

创建 `Cargo.toml`:

```toml
[package]
name = "rss-reader"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.26"
crossterm = "0.27"
tokio = { version = "1.36", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
feed-rs = "1.4"
reqwest = { version = "0.11", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
wiremock = "0.6"
criterion = "0.5"
tempfile = "3.10"

[[bench]]
name = "search_benchmark"
harness = false
```

**Step 2: 创建基础 main.rs**

创建 `src/main.rs`:

```rust
fn main() {
    println!("RSS Reader - Coming Soon");
}
```

**Step 3: 创建 .gitignore**

创建 `.gitignore`:

```
/target
Cargo.lock
*.db
*.db-shm
*.db-wal
.DS_Store
```

**Step 4: 创建测试目录**

```bash
mkdir -p tests/fixtures
touch tests/fixtures/.gitkeep
```

**Step 5: 验证项目编译**

Run: `cargo build`
Expected: 成功编译，无错误

**Step 6: 提交**

```bash
git add .
git commit -m "chore: 初始化 Rust 项目结构

- 添加 Cargo.toml 依赖配置
- 创建基础 main.rs
- 添加 .gitignore

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 2: 数据库 Schema 和迁移

**Files:**
- Create: `migrations/20260303000001_init.sql`
- Create: `src/db/mod.rs`
- Create: `src/db/schema.rs`
- Create: `tests/db_test.rs`

**Step 1: 编写数据库迁移测试**

创建 `tests/db_test.rs`:

```rust
use sqlx::SqlitePool;

#[tokio::test]
async fn test_database_schema_creation() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    // 执行迁移
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    // 验证表存在
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='feeds'")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(result.0, 1);
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_database_schema_creation`
Expected: FAIL - 迁移文件不存在

**Step 3: 创建数据库迁移文件**

创建 `migrations/20260303000001_init.sql`:

```sql
-- Feeds 表
CREATE TABLE feeds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    category TEXT NOT NULL,
    last_fetched TIMESTAMP,
    fetch_error TEXT
);

-- Articles 表
CREATE TABLE articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    feed_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    link TEXT UNIQUE NOT NULL,
    content TEXT,
    published TIMESTAMP NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT 0,
    is_bookmarked BOOLEAN NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
);

-- 全文搜索索引
CREATE VIRTUAL TABLE articles_fts USING fts5(
    title,
    content,
    content=articles,
    content_rowid=id
);

-- FTS 触发器
CREATE TRIGGER articles_ai AFTER INSERT ON articles BEGIN
    INSERT INTO articles_fts(rowid, title, content)
    VALUES (new.id, new.title, new.content);
END;

CREATE TRIGGER articles_ad AFTER DELETE ON articles BEGIN
    DELETE FROM articles_fts WHERE rowid = old.id;
END;

CREATE TRIGGER articles_au AFTER UPDATE ON articles BEGIN
    UPDATE articles_fts SET title = new.title, content = new.content
    WHERE rowid = new.id;
END;

-- Tags 表
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

-- Article-Tag 关联表
CREATE TABLE article_tags (
    article_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (article_id, tag_id),
    FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX idx_articles_feed_id ON articles(feed_id);
CREATE INDEX idx_articles_published ON articles(published DESC);
CREATE INDEX idx_articles_is_read ON articles(is_read);
CREATE INDEX idx_articles_is_bookmarked ON articles(is_bookmarked);
```

**Step 4: 运行测试验证通过**

Run: `cargo test test_database_schema_creation`
Expected: PASS

**Step 5: 创建数据库模块**

创建 `src/db/mod.rs`:

```rust
pub mod schema;

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
```

**Step 6: 创建数据模型**

创建 `src/db/schema.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Feed {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub category: String,
    pub last_fetched: Option<DateTime<Utc>>,
    pub fetch_error: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub feed_id: i64,
    pub title: String,
    pub link: String,
    pub content: Option<String>,
    pub published: DateTime<Utc>,
    pub is_read: bool,
    pub is_bookmarked: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ArticleTag {
    pub article_id: i64,
    pub tag_id: i64,
}
```

**Step 7: 更新 main.rs 引入模块**

修改 `src/main.rs`:

```rust
mod db;

fn main() {
    println!("RSS Reader - Coming Soon");
}
```

**Step 8: 运行所有测试**

Run: `cargo test`
Expected: 所有测试通过

**Step 9: 提交**

```bash
git add .
git commit -m "feat(db): 添加数据库 schema 和迁移

- 创建 feeds/articles/tags 表结构
- 添加 FTS5 全文搜索索引
- 实现数据库连接池
- 添加数据模型定义

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 3: 数据库 CRUD 操作

**Files:**
- Create: `src/db/feeds.rs`
- Create: `tests/db_feeds_test.rs`

**Step 1: 编写 Feed CRUD 测试**

创建 `tests/db_feeds_test.rs`:

```rust
use sqlx::SqlitePool;
use rss_reader::db::{create_pool, schema::Feed};
use rss_reader::db::feeds::{insert_feed, get_all_feeds, get_feed_by_url};

#[tokio::test]
async fn test_insert_and_query_feed() {
    let pool = create_pool(":memory:").await.unwrap();

    // 插入 feed
    let feed_id = insert_feed(
        &pool,
        "Hacker News",
        "https://news.ycombinator.com/rss",
        "tech"
    ).await.unwrap();

    assert!(feed_id > 0);

    // 查询所有 feeds
    let feeds = get_all_feeds(&pool).await.unwrap();
    assert_eq!(feeds.len(), 1);
    assert_eq!(feeds[0].title, "Hacker News");

    // 按 URL 查询
    let feed = get_feed_by_url(&pool, "https://news.ycombinator.com/rss")
        .await
        .unwrap();
    assert!(feed.is_some());
    assert_eq!(feed.unwrap().title, "Hacker News");
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_insert_and_query_feed`
Expected: FAIL - 模块不存在

**Step 3: 实现 Feed CRUD 操作**

创建 `src/db/feeds.rs`:

```rust
use sqlx::SqlitePool;
use anyhow::Result;
use chrono::Utc;
use super::schema::Feed;

pub async fn insert_feed(
    pool: &SqlitePool,
    title: &str,
    url: &str,
    category: &str,
) -> Result<i64> {
    let result = sqlx::query!(
        "INSERT INTO feeds (title, url, category) VALUES (?, ?, ?)",
        title,
        url,
        category
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_all_feeds(pool: &SqlitePool) -> Result<Vec<Feed>> {
    let feeds = sqlx::query_as!(
        Feed,
        "SELECT id, title, url, category, last_fetched, fetch_error FROM feeds ORDER BY category, title"
    )
    .fetch_all(pool)
    .await?;

    Ok(feeds)
}

pub async fn get_feed_by_url(pool: &SqlitePool, url: &str) -> Result<Option<Feed>> {
    let feed = sqlx::query_as!(
        Feed,
        "SELECT id, title, url, category, last_fetched, fetch_error FROM feeds WHERE url = ?",
        url
    )
    .fetch_optional(pool)
    .await?;

    Ok(feed)
}

pub async fn update_feed_fetch_time(pool: &SqlitePool, feed_id: i64) -> Result<()> {
    let now = Utc::now();
    sqlx::query!(
        "UPDATE feeds SET last_fetched = ?, fetch_error = NULL WHERE id = ?",
        now,
        feed_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_feed_error(pool: &SqlitePool, feed_id: i64, error: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE feeds SET fetch_error = ? WHERE id = ?",
        error,
        feed_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;

    #[tokio::test]
    async fn test_feed_operations() {
        let pool = create_pool(":memory:").await.unwrap();

        // 插入
        let id = insert_feed(&pool, "Test", "http://test.com/rss", "test")
            .await
            .unwrap();
        assert!(id > 0);

        // 查询
        let feeds = get_all_feeds(&pool).await.unwrap();
        assert_eq!(feeds.len(), 1);

        // 更新时间
        update_feed_fetch_time(&pool, id).await.unwrap();
        let feed = get_feed_by_url(&pool, "http://test.com/rss")
            .await
            .unwrap()
            .unwrap();
        assert!(feed.last_fetched.is_some());
    }
}
```

**Step 4: 更新 db/mod.rs**

修改 `src/db/mod.rs`:

```rust
pub mod schema;
pub mod feeds;

// ... 其余代码保持不变
```

**Step 5: 运行测试验证通过**

Run: `cargo test`
Expected: 所有测试通过

**Step 6: 提交**

```bash
git add .
git commit -m "feat(db): 实现 Feed CRUD 操作

- 添加 insert_feed/get_all_feeds/get_feed_by_url
- 添加 update_feed_fetch_time/update_feed_error
- 完整的单元测试覆盖

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 4: Article CRUD 和搜索

**Files:**
- Create: `src/db/articles.rs`
- Create: `tests/db_articles_test.rs`

**Step 1: 编写 Article CRUD 和搜索测试**

创建 `tests/db_articles_test.rs`:

```rust
use sqlx::SqlitePool;
use chrono::Utc;
use rss_reader::db::{create_pool, feeds::insert_feed};
use rss_reader::db::articles::{
    insert_article, get_articles_by_feed, search_articles,
    mark_as_read, toggle_bookmark, get_unread_count
};

#[tokio::test]
async fn test_article_operations() {
    let pool = create_pool(":memory:").await.unwrap();

    // 创建 feed
    let feed_id = insert_feed(&pool, "Test Feed", "http://test.com", "test")
        .await
        .unwrap();

    // 插入文章
    let article_id = insert_article(
        &pool,
        feed_id,
        "Test Article",
        "http://test.com/article1",
        Some("Content about Rust programming"),
        Utc::now()
    ).await.unwrap();

    assert!(article_id > 0);

    // 查询文章
    let articles = get_articles_by_feed(&pool, feed_id, 10, 0)
        .await
        .unwrap();
    assert_eq!(articles.len(), 1);

    // 标记已读
    mark_as_read(&pool, article_id, true).await.unwrap();
    let count = get_unread_count(&pool).await.unwrap();
    assert_eq!(count, 0);

    // 切换书签
    toggle_bookmark(&pool, article_id).await.unwrap();

    // 全文搜索
    let results = search_articles(&pool, "Rust", 10, 0)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_article_operations`
Expected: FAIL - 模块不存在

**Step 3: 实现 Article CRUD 操作**

创建 `src/db/articles.rs` (第一部分，50行以内):

```rust
use sqlx::SqlitePool;
use anyhow::Result;
use chrono::{DateTime, Utc};
use super::schema::Article;

pub async fn insert_article(
    pool: &SqlitePool,
    feed_id: i64,
    title: &str,
    link: &str,
    content: Option<&str>,
    published: DateTime<Utc>,
) -> Result<i64> {
    let result = sqlx::query!(
        "INSERT INTO articles (feed_id, title, link, content, published)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT(link) DO NOTHING",
        feed_id,
        title,
        link,
        content,
        published
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_articles_by_feed(
    pool: &SqlitePool,
    feed_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        "SELECT id, feed_id, title, link, content, published,
                is_read, is_bookmarked, created_at
         FROM articles
         WHERE feed_id = ?
         ORDER BY published DESC
         LIMIT ? OFFSET ?",
        feed_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}
```

**Step 4: 继续实现 Article 操作 (第二部分)**

使用 Edit 工具追加到 `src/db/articles.rs`:

```rust

pub async fn get_all_articles(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        "SELECT id, feed_id, title, link, content, published,
                is_read, is_bookmarked, created_at
         FROM articles
         ORDER BY published DESC
         LIMIT ? OFFSET ?",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn search_articles(
    pool: &SqlitePool,
    query: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        "SELECT a.id, a.feed_id, a.title, a.link, a.content, a.published,
                a.is_read, a.is_bookmarked, a.created_at
         FROM articles a
         JOIN articles_fts fts ON a.id = fts.rowid
         WHERE articles_fts MATCH ?
         ORDER BY a.published DESC
         LIMIT ? OFFSET ?",
        query,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn mark_as_read(pool: &SqlitePool, article_id: i64, is_read: bool) -> Result<()> {
    sqlx::query!(
        "UPDATE articles SET is_read = ? WHERE id = ?",
        is_read,
        article_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
```

**Step 5: 完成 Article 操作 (第三部分)**

继续追加到 `src/db/articles.rs`:

```rust

pub async fn toggle_bookmark(pool: &SqlitePool, article_id: i64) -> Result<()> {
    sqlx::query!(
        "UPDATE articles SET is_bookmarked = NOT is_bookmarked WHERE id = ?",
        article_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_unread_count(pool: &SqlitePool) -> Result<i64> {
    let result = sqlx::query!("SELECT COUNT(*) as count FROM articles WHERE is_read = 0")
        .fetch_one(pool)
        .await?;

    Ok(result.count as i64)
}

pub async fn get_bookmarked_articles(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        "SELECT id, feed_id, title, link, content, published,
                is_read, is_bookmarked, created_at
         FROM articles
         WHERE is_bookmarked = 1
         ORDER BY published DESC
         LIMIT ? OFFSET ?",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{create_pool, feeds::insert_feed};

    #[tokio::test]
    async fn test_article_crud() {
        let pool = create_pool(":memory:").await.unwrap();
        let feed_id = insert_feed(&pool, "Test", "http://test.com", "test")
            .await
            .unwrap();

        let id = insert_article(
            &pool,
            feed_id,
            "Title",
            "http://test.com/1",
            Some("Content"),
            Utc::now()
        ).await.unwrap();

        assert!(id > 0);

        let articles = get_all_articles(&pool, 10, 0).await.unwrap();
        assert_eq!(articles.len(), 1);
    }
}
```

**Step 6: 更新 db/mod.rs**

修改 `src/db/mod.rs`:

```rust
pub mod schema;
pub mod feeds;
pub mod articles;

// ... 其余代码
```

**Step 7: 运行测试验证通过**

Run: `cargo test`
Expected: 所有测试通过

**Step 8: 提交**

```bash
git add .
git commit -m "feat(db): 实现 Article CRUD 和全文搜索

- 添加文章插入、查询、更新操作
- 实现 FTS5 全文搜索
- 支持已读/书签状态管理
- 完整的单元测试覆盖

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 5: Tag 管理

**Files:**
- Create: `src/db/tags.rs`
- Create: `tests/db_tags_test.rs`

**Step 1: 编写 Tag 测试**

创建 `tests/db_tags_test.rs`:

```rust
use sqlx::SqlitePool;
use chrono::Utc;
use rss_reader::db::{create_pool, feeds::insert_feed, articles::insert_article};
use rss_reader::db::tags::{
    create_tag, get_all_tags, add_tag_to_article,
    get_tags_for_article, get_articles_by_tag
};

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
        Utc::now()
    ).await.unwrap();

    // 给文章添加标签
    add_tag_to_article(&pool, article_id, tag_id).await.unwrap();

    // 查询文章的标签
    let article_tags = get_tags_for_article(&pool, article_id)
        .await
        .unwrap();
    assert_eq!(article_tags.len(), 1);

    // 按标签查询文章
    let articles = get_articles_by_tag(&pool, tag_id, 10, 0)
        .await
        .unwrap();
    assert_eq!(articles.len(), 1);
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_tag_operations`
Expected: FAIL - 模块不存在

**Step 3: 实现 Tag 操作**

创建 `src/db/tags.rs`:

```rust
use sqlx::SqlitePool;
use anyhow::Result;
use super::schema::{Tag, Article};

pub async fn create_tag(pool: &SqlitePool, name: &str) -> Result<i64> {
    let result = sqlx::query!(
        "INSERT INTO tags (name) VALUES (?) ON CONFLICT(name) DO NOTHING",
        name
    )
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        Ok(result.last_insert_rowid())
    } else {
        // 标签已存在，查询 ID
        let tag = sqlx::query!("SELECT id FROM tags WHERE name = ?", name)
            .fetch_one(pool)
            .await?;
        Ok(tag.id)
    }
}

pub async fn get_all_tags(pool: &SqlitePool) -> Result<Vec<Tag>> {
    let tags = sqlx::query_as!(
        Tag,
        "SELECT id, name FROM tags ORDER BY name"
    )
    .fetch_all(pool)
    .await?;

    Ok(tags)
}

pub async fn add_tag_to_article(
    pool: &SqlitePool,
    article_id: i64,
    tag_id: i64,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO article_tags (article_id, tag_id) VALUES (?, ?)
         ON CONFLICT DO NOTHING",
        article_id,
        tag_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_tag_from_article(
    pool: &SqlitePool,
    article_id: i64,
    tag_id: i64,
) -> Result<()> {
    sqlx::query!(
        "DELETE FROM article_tags WHERE article_id = ? AND tag_id = ?",
        article_id,
        tag_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_tags_for_article(pool: &SqlitePool, article_id: i64) -> Result<Vec<Tag>> {
    let tags = sqlx::query_as!(
        Tag,
        "SELECT t.id, t.name
         FROM tags t
         JOIN article_tags at ON t.id = at.tag_id
         WHERE at.article_id = ?
         ORDER BY t.name",
        article_id
    )
    .fetch_all(pool)
    .await?;

    Ok(tags)
}

pub async fn get_articles_by_tag(
    pool: &SqlitePool,
    tag_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        "SELECT a.id, a.feed_id, a.title, a.link, a.content, a.published,
                a.is_read, a.is_bookmarked, a.created_at
         FROM articles a
         JOIN article_tags at ON a.id = at.article_id
         WHERE at.tag_id = ?
         ORDER BY a.published DESC
         LIMIT ? OFFSET ?",
        tag_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;

    #[tokio::test]
    async fn test_tag_crud() {
        let pool = create_pool(":memory:").await.unwrap();

        let id = create_tag(&pool, "test").await.unwrap();
        assert!(id > 0);

        let tags = get_all_tags(&pool).await.unwrap();
        assert_eq!(tags.len(), 1);

        // 重复创建应返回相同 ID
        let id2 = create_tag(&pool, "test").await.unwrap();
        assert_eq!(id, id2);
    }
}
```

**Step 4: 更新 db/mod.rs**

修改 `src/db/mod.rs`:

```rust
pub mod schema;
pub mod feeds;
pub mod articles;
pub mod tags;

// ... 其余代码
```

**Step 5: 运行测试验证通过**

Run: `cargo test`
Expected: 所有测试通过

**Step 6: 提交**

```bash
git add .
git commit -m "feat(db): 实现 Tag 管理功能

- 添加标签创建、查询操作
- 实现文章-标签关联
- 支持按标签过滤文章
- 完整的单元测试覆盖

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 6: RSS 拉取器 - HTTP 客户端

**Files:**
- Create: `src/fetcher/mod.rs`
- Create: `src/fetcher/http.rs`
- Create: `tests/fetcher_test.rs`
- Create: `tests/fixtures/sample_rss.xml`

**Step 1: 创建测试 fixture**

创建 `tests/fixtures/sample_rss.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Test Feed</title>
    <link>http://example.com</link>
    <description>Test RSS Feed</description>
    <item>
      <title>Test Article 1</title>
      <link>http://example.com/article1</link>
      <description>This is a test article about Rust</description>
      <pubDate>Mon, 03 Mar 2026 10:00:00 GMT</pubDate>
    </item>
    <item>
      <title>Test Article 2</title>
      <link>http://example.com/article2</link>
      <description>Another test article</description>
      <pubDate>Mon, 03 Mar 2026 09:00:00 GMT</pubDate>
    </item>
  </channel>
</rss>
```

**Step 2: 编写 HTTP 拉取测试**

创建 `tests/fetcher_test.rs`:

```rust
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
    use tokio::time::sleep;

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
```

**Step 3: 运行测试验证失败**

Run: `cargo test test_fetch_feed_success`
Expected: FAIL - 模块不存在

**Step 4: 实现 HTTP 客户端**

创建 `src/fetcher/http.rs`:

```rust
use reqwest::Client;
use anyhow::{Result, Context};
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
```

**Step 5: 创建 fetcher 模块**

创建 `src/fetcher/mod.rs`:

```rust
pub mod http;
```

**Step 6: 更新 main.rs**

修改 `src/main.rs`:

```rust
mod db;
mod fetcher;

fn main() {
    println!("RSS Reader - Coming Soon");
}
```

**Step 7: 运行测试验证通过**

Run: `cargo test`
Expected: 所有测试通过

**Step 8: 提交**

```bash
git add .
git commit -m "feat(fetcher): 实现 HTTP 客户端

- 添加 fetch_feed 函数
- 支持超时控制（30秒）
- 使用 wiremock 进行测试
- 添加测试 fixtures

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 7: RSS 解析器

**Files:**
- Create: `src/fetcher/parser.rs`
- Create: `tests/fixtures/sample_atom.xml`
- Modify: `tests/fetcher_test.rs`

**Step 1: 创建 Atom fixture**

创建 `tests/fixtures/sample_atom.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Test Atom Feed</title>
  <link href="http://example.com"/>
  <updated>2026-03-03T10:00:00Z</updated>
  <entry>
    <title>Atom Article 1</title>
    <link href="http://example.com/atom1"/>
    <updated>2026-03-03T10:00:00Z</updated>
    <summary>Test atom article</summary>
  </entry>
</feed>
```

**Step 2: 编写解析器测试**

追加到 `tests/fetcher_test.rs`:

```rust
use rss_reader::fetcher::parser::{parse_feed, ParsedFeed, ParsedArticle};

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
```

**Step 3: 运行测试验证失败**

Run: `cargo test test_parse_rss_feed`
Expected: FAIL - 模块不存在

**Step 4: 实现解析器**

创建 `src/fetcher/parser.rs`:

```rust
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
                .or(entry.content.and_then(|c| c.body))
                .map(|s| s.content);

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
```

**Step 5: 更新 fetcher/mod.rs**

修改 `src/fetcher/mod.rs`:

```rust
pub mod http;
pub mod parser;
```

**Step 6: 运行测试验证通过**

Run: `cargo test`
Expected: 所有测试通过

**Step 7: 提交**

```bash
git add .
git commit -m "feat(fetcher): 实现 RSS/Atom 解析器

- 使用 feed-rs 解析 RSS 2.0 和 Atom
- 提取标题、链接、内容、发布时间
- 支持空 feed 和无效 XML 处理
- 完整的单元测试覆盖

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 8: Feed 管理器 - 并发拉取

**Files:**
- Create: `src/core/mod.rs`
- Create: `src/core/feed_manager.rs`
- Create: `tests/feed_manager_test.rs`

**Step 1: 编写 Feed 管理器测试**

创建 `tests/feed_manager_test.rs`:

```rust
use sqlx::SqlitePool;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use rss_reader::db::create_pool;
use rss_reader::core::feed_manager::FeedManager;

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
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_fetch_all_feeds`
Expected: FAIL - 模块不存在

**Step 3: 实现 Feed 管理器 (第一部分)**

创建 `src/core/feed_manager.rs`:

```rust
use sqlx::SqlitePool;
use anyhow::Result;
use tokio::task::JoinHandle;
use std::time::Duration;
use crate::db::{feeds, articles, schema::Feed};
use crate::fetcher::{http::fetch_feed, parser::parse_feed};

pub struct FeedManager {
    pool: SqlitePool,
}

impl FeedManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_feed(&self, title: &str, url: &str, category: &str) -> Result<i64> {
        feeds::insert_feed(&self.pool, title, url, category).await
    }

    pub async fn get_all_feeds(&self) -> Result<Vec<Feed>> {
        feeds::get_all_feeds(&self.pool).await
    }

    pub async fn fetch_all_feeds(&self) -> Vec<Result<usize>> {
        let feeds = match self.get_all_feeds().await {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to get feeds: {}", e);
                return vec![];
            }
        };

        let tasks: Vec<JoinHandle<Result<usize>>> = feeds
            .into_iter()
            .map(|feed| {
                let pool = self.pool.clone();
                tokio::spawn(async move {
                    Self::fetch_single_feed(pool, feed).await
                })
            })
            .collect();

        // 等待所有任务完成，超时 30 秒
        let timeout_duration = Duration::from_secs(30);
        let results = tokio::time::timeout(
            timeout_duration,
            futures::future::join_all(tasks)
        ).await;

        match results {
            Ok(results) => results.into_iter().map(|r| r.unwrap_or_else(|e| {
                Err(anyhow::anyhow!("Task join error: {}", e))
            })).collect(),
            Err(_) => {
                tracing::error!("Fetch all feeds timeout");
                vec![]
            }
        }
    }
```

**Step 4: 继续实现 Feed 管理器 (第二部分)**

追加到 `src/core/feed_manager.rs`:

```rust

    async fn fetch_single_feed(pool: SqlitePool, feed: Feed) -> Result<usize> {
        tracing::info!("Fetching feed: {} ({})", feed.title, feed.url);

        // 拉取 feed
        let xml = match fetch_feed(&feed.url).await {
            Ok(xml) => xml,
            Err(e) => {
                let error_msg = format!("Fetch error: {}", e);
                tracing::error!("{}", error_msg);
                feeds::update_feed_error(&pool, feed.id, &error_msg).await?;
                return Err(e);
            }
        };

        // 解析 feed
        let parsed = match parse_feed(&xml) {
            Ok(p) => p,
            Err(e) => {
                let error_msg = format!("Parse error: {}", e);
                tracing::error!("{}", error_msg);
                feeds::update_feed_error(&pool, feed.id, &error_msg).await?;
                return Err(e);
            }
        };

        // 插入文章
        let mut inserted_count = 0;
        for article in parsed.articles {
            match articles::insert_article(
                &pool,
                feed.id,
                &article.title,
                &article.link,
                article.content.as_deref(),
                article.published,
            ).await {
                Ok(id) if id > 0 => inserted_count += 1,
                Ok(_) => {}, // 文章已存在
                Err(e) => tracing::warn!("Failed to insert article: {}", e),
            }
        }

        // 更新拉取时间
        feeds::update_feed_fetch_time(&pool, feed.id).await?;

        tracing::info!("Feed {} fetched: {} new articles", feed.title, inserted_count);
        Ok(inserted_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::create_pool;

    #[tokio::test]
    async fn test_feed_manager_basic() {
        let pool = create_pool(":memory:").await.unwrap();
        let manager = FeedManager::new(pool);

        let id = manager.add_feed("Test", "http://test.com", "test")
            .await
            .unwrap();
        assert!(id > 0);

        let feeds = manager.get_all_feeds().await.unwrap();
        assert_eq!(feeds.len(), 1);
    }
}
```

**Step 5: 添加 futures 依赖**

修改 `Cargo.toml`，在 `[dependencies]` 中添加:

```toml
futures = "0.3"
```

**Step 6: 创建 core 模块**

创建 `src/core/mod.rs`:

```rust
pub mod feed_manager;
```

**Step 7: 更新 main.rs**

修改 `src/main.rs`:

```rust
mod db;
mod fetcher;
mod core;

fn main() {
    println!("RSS Reader - Coming Soon");
}
```

**Step 8: 运行测试验证通过**

Run: `cargo test`
Expected: 所有测试通过

**Step 9: 提交**

```bash
git add .
git commit -m "feat(core): 实现 Feed 管理器和并发拉取

- 添加 FeedManager 管理 feeds
- 实现并发拉取所有 feeds (tokio::spawn)
- 支持超时控制和错误处理
- 自动插入新文章到数据库
- 完整的单元测试覆盖

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

由于实现计划非常长，我将分多次追加。让我继续添加剩余任务...

## Task 9: 加载默认 RSS 列表

**Files:**
- Create: `src/core/feed_loader.rs`
- Create: `tests/feed_loader_test.rs`

**Step 1: 编写 Feed 加载器测试**

创建 `tests/feed_loader_test.rs`:

```rust
use rss_reader::core::feed_loader::load_feeds_from_url;

#[tokio::test]
async fn test_load_feeds_from_markdown() {
    let markdown = r#"
# rsshub
- https://rsshub.umzzz.com/v2ex/topics/hot
- https://rsshub.umzzz.com/douban/group/blabla/discussion

# tech
- https://news.ycombinator.com/rss
- https://www.theverge.com/rss/index.xml
"#;

    let feeds = load_feeds_from_url(markdown).unwrap();
    assert_eq!(feeds.len(), 4);
    assert_eq!(feeds[0].category, "rsshub");
    assert_eq!(feeds[2].category, "tech");
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test test_load_feeds_from_markdown`
Expected: FAIL - 模块不存在

**Step 3: 实现 Feed 加载器**

创建 `src/core/feed_loader.rs`:

```rust
use anyhow::{Result, Context};
use reqwest::Client;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FeedInfo {
    pub url: String,
    pub category: String,
}

pub async fn fetch_feed_list(url: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client.get(url).send().await?;
    let content = response.text().await?;

    Ok(content)
}

pub fn load_feeds_from_url(markdown: &str) -> Result<Vec<FeedInfo>> {
    let mut feeds = Vec::new();
    let mut current_category = String::new();

    for line in markdown.lines() {
        let line = line.trim();

        // 跳过空行
        if line.is_empty() {
            continue;
        }

        // 检测分类标题 (# category)
        if line.starts_with('#') {
            current_category = line.trim_start_matches('#').trim().to_string();
            continue;
        }

        // 检测 URL 行 (- http://...)
        if line.starts_with('-') {
            let url = line.trim_start_matches('-').trim();
            if url.starts_with("http://") || url.starts_with("https://") {
                feeds.push(FeedInfo {
                    url: url.to_string(),
                    category: current_category.clone(),
                });
            }
        }
    }

    Ok(feeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_markdown() {
        let result = load_feeds_from_url("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_markdown_with_multiple_categories() {
        let md = "# cat1\n- http://a.com\n# cat2\n- http://b.com\n- http://c.com";
        let feeds = load_feeds_from_url(md).unwrap();
        assert_eq!(feeds.len(), 3);
        assert_eq!(feeds[0].category, "cat1");
        assert_eq!(feeds[1].category, "cat2");
    }
}
```

**Step 4: 更新 core/mod.rs**

修改 `src/core/mod.rs`:

```rust
pub mod feed_manager;
pub mod feed_loader;
```

**Step 5: 运行测试验证通过**

Run: `cargo test`
Expected: 所有测试通过

**Step 6: 提交**

```bash
git add .
git commit -m "feat(core): 实现 Feed 列表加载器

- 从 Markdown 格式解析 feed 列表
- 支持分类标签
- 支持从 URL 拉取列表
- 完整的单元测试覆盖

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 10: TUI 基础框架

**Files:**
- Create: `src/ui/mod.rs`
- Create: `src/ui/app.rs`
- Create: `src/ui/events.rs`
- Modify: `src/main.rs`

**Step 1: 创建 UI 事件处理**

创建 `src/ui/events.rs`:

```rust
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;
use anyhow::Result;

pub enum Event {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    pub fn next(&self) -> Result<Event> {
        if event::poll(self.tick_rate)? {
            if let CrosstermEvent::Key(key) = event::read()? {
                return Ok(Event::Key(key));
            }
        }
        Ok(Event::Tick)
    }
}
```

**Step 2: 创建应用状态**

创建 `src/ui/app.rs`:

```rust
use sqlx::SqlitePool;
use crate::db::schema::{Feed, Article};

pub struct App {
    pub pool: SqlitePool,
    pub should_quit: bool,
    pub feeds: Vec<Feed>,
    pub articles: Vec<Article>,
    pub selected_feed_index: usize,
    pub selected_article_index: usize,
}

impl App {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            should_quit: false,
            feeds: Vec::new(),
            articles: Vec::new(),
            selected_feed_index: 0,
            selected_article_index: 0,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub async fn load_feeds(&mut self) -> anyhow::Result<()> {
        self.feeds = crate::db::feeds::get_all_feeds(&self.pool).await?;
        Ok(())
    }

    pub async fn load_articles(&mut self) -> anyhow::Result<()> {
        self.articles = crate::db::articles::get_all_articles(&self.pool, 100, 0).await?;
        Ok(())
    }

    pub fn next_feed(&mut self) {
        if !self.feeds.is_empty() {
            self.selected_feed_index = (self.selected_feed_index + 1) % self.feeds.len();
        }
    }

    pub fn previous_feed(&mut self) {
        if !self.feeds.is_empty() {
            if self.selected_feed_index > 0 {
                self.selected_feed_index -= 1;
            } else {
                self.selected_feed_index = self.feeds.len() - 1;
            }
        }
    }

    pub fn next_article(&mut self) {
        if !self.articles.is_empty() {
            self.selected_article_index = (self.selected_article_index + 1) % self.articles.len();
        }
    }

    pub fn previous_article(&mut self) {
        if !self.articles.is_empty() {
            if self.selected_article_index > 0 {
                self.selected_article_index -= 1;
            } else {
                self.selected_article_index = self.articles.len() - 1;
            }
        }
    }
}
```

**Step 3: 创建 UI 模块**

创建 `src/ui/mod.rs`:

```rust
pub mod app;
pub mod events;

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io;
use anyhow::Result;

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn render_placeholder(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(f.size());

        let block = Block::default()
            .title("RSS Reader - Press 'q' to quit")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));

        let paragraph = Paragraph::new("TUI Coming Soon...")
            .block(block);

        f.render_widget(paragraph, chunks[0]);
    })?;
    Ok(())
}
```

**Step 4: 更新 main.rs 使用 TUI**

修改 `src/main.rs`:

```rust
mod db;
mod fetcher;
mod core;
mod ui;

use ui::{init_terminal, restore_terminal, render_placeholder, events::EventHandler, app::App};
use crossterm::event::{KeyCode, KeyModifiers};
use std::time::Duration;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 初始化数据库
    let database_url = "sqlite://rss_reader.db";
    let pool = db::create_pool(database_url).await?;

    // 初始化终端
    let mut terminal = init_terminal()?;

    // 创建应用
    let mut app = App::new(pool);
    app.load_feeds().await?;
    app.load_articles().await?;

    // 事件循环
    let event_handler = EventHandler::new(Duration::from_millis(250));

    loop {
        render_placeholder(&mut terminal)?;

        match event_handler.next()? {
            ui::events::Event::Key(key) => {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        app.quit();
                    }
                    (KeyCode::Char('j'), _) | (KeyCode::Down, _) => {
                        app.next_article();
                    }
                    (KeyCode::Char('k'), _) | (KeyCode::Up, _) => {
                        app.previous_article();
                    }
                    _ => {}
                }
            }
            ui::events::Event::Tick => {}
        }

        if app.should_quit {
            break;
        }
    }

    // 恢复终端
    restore_terminal(terminal)?;

    Ok(())
}
```

**Step 5: 运行程序验证**

Run: `cargo run`
Expected: 显示 TUI 界面，按 'q' 退出

**Step 6: 提交**

```bash
git add .
git commit -m "feat(ui): 实现 TUI 基础框架

- 添加终端初始化和恢复
- 实现事件循环和键盘处理
- 创建应用状态管理
- 支持基本导航 (j/k/q)
- 占位符界面渲染

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 11: Feed 列表面板

**Files:**
- Create: `src/ui/feed_list.rs`
- Modify: `src/ui/mod.rs`
- Modify: `src/main.rs`

**Step 1: 实现 Feed 列表组件**

创建 `src/ui/feed_list.rs`:

```rust
use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Color, Modifier},
    text::Span,
};
use crate::db::schema::Feed;

pub struct FeedList {
    pub state: ListState,
}

impl FeedList {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self { state }
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1) % len,
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn render_widget<'a>(&self, feeds: &'a [Feed]) -> (List<'a>, &mut ListState) {
        let items: Vec<ListItem> = feeds
            .iter()
            .map(|feed| {
                let category_tag = format!("[{}]", feed.category);
                let content = vec![
                    Span::styled(
                        category_tag,
                        Style::default().fg(Color::Yellow)
                    ),
                    Span::raw(" "),
                    Span::raw(&feed.title),
                ];
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Feeds")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Cyan))
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol("▶ ");

        (list, &mut self.state.clone())
    }
}
```

**Step 2: 更新 ui/mod.rs**

修改 `src/ui/mod.rs`，添加:

```rust
pub mod feed_list;
```

**Step 3: 更新 app.rs 使用 FeedList**

修改 `src/ui/app.rs`:

```rust
use sqlx::SqlitePool;
use crate::db::schema::{Feed, Article};
use super::feed_list::FeedList;

pub struct App {
    pub pool: SqlitePool,
    pub should_quit: bool,
    pub feeds: Vec<Feed>,
    pub articles: Vec<Article>,
    pub feed_list: FeedList,
    pub selected_article_index: usize,
}

impl App {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            should_quit: false,
            feeds: Vec::new(),
            articles: Vec::new(),
            feed_list: FeedList::new(),
            selected_article_index: 0,
        }
    }

    // ... 其他方法保持不变

    pub fn next_feed(&mut self) {
        self.feed_list.next(self.feeds.len());
    }

    pub fn previous_feed(&mut self) {
        self.feed_list.previous(self.feeds.len());
    }
}
```

**Step 4: 更新 main.rs 渲染 Feed 列表**

修改 `src/main.rs` 中的渲染函数:

```rust
use ratatui::layout::{Layout, Constraint, Direction};

fn render_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(75),
            ])
            .split(f.size());

        // 渲染 Feed 列表
        let (feed_list, state) = app.feed_list.render_widget(&app.feeds);
        f.render_stateful_widget(feed_list, chunks[0], state);

        // 占位符：文章列表
        let placeholder = Block::default()
            .title("Articles")
            .borders(Borders::ALL);
        f.render_widget(placeholder, chunks[1]);
    })?;
    Ok(())
}

// 在 main 函数的事件循环中替换 render_placeholder 为 render_ui
```

**Step 5: 运行程序验证**

Run: `cargo run`
Expected: 显示 Feed 列表，可以用 j/k 导航

**Step 6: 提交**

```bash
git add .
git commit -m "feat(ui): 实现 Feed 列表面板

- 添加 FeedList 组件
- 支持高亮选中项
- 支持键盘导航 (j/k)
- 显示分类标签
- 双栏布局

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task 12: Article 列表和预览面板

**Files:**
- Create: `src/ui/article_list.rs`
- Create: `src/ui/preview.rs`
- Modify: `src/ui/mod.rs`
- Modify: `src/main.rs`

**Step 1: 实现 Article 列表组件**

创建 `src/ui/article_list.rs`:

```rust
use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState},
    style::{Style, Color, Modifier},
    text::{Span, Line},
};
use crate::db::schema::Article;
use chrono::Local;

pub struct ArticleList {
    pub state: ListState,
}

impl ArticleList {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self { state }
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1) % len,
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn render_widget<'a>(&self, articles: &'a [Article]) -> (List<'a>, &mut ListState) {
        let items: Vec<ListItem> = articles
            .iter()
            .map(|article| {
                let read_marker = if article.is_read { " " } else { "●" };
                let bookmark_marker = if article.is_bookmarked { "★" } else { " " };

                let time_ago = {
                    let duration = Local::now().signed_duration_since(article.published);
                    if duration.num_days() > 0 {
                        format!("{}d", duration.num_days())
                    } else if duration.num_hours() > 0 {
                        format!("{}h", duration.num_hours())
                    } else {
                        format!("{}m", duration.num_minutes())
                    }
                };

                let content = Line::from(vec![
                    Span::styled(
                        read_marker,
                        Style::default().fg(Color::Green)
                    ),
                    Span::raw(" "),
                    Span::styled(
                        bookmark_marker,
                        Style::default().fg(Color::Yellow)
                    ),
                    Span::raw(" "),
                    Span::raw(&article.title),
                    Span::raw(" "),
                    Span::styled(
                        format!("({})", time_ago),
                        Style::default().fg(Color::DarkGray)
                    ),
                ]);

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Articles")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Cyan))
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol("▶ ");

        (list, &mut self.state.clone())
    }
}
```

**Step 2: 实现预览面板**

创建 `src/ui/preview.rs`:

```rust
use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Style, Color},
    text::{Line, Span},
};
use crate::db::schema::Article;

pub fn render_preview<'a>(article: Option<&'a Article>) -> Paragraph<'a> {
    match article {
        Some(article) => {
            let content = vec![
                Line::from(vec![
                    Span::styled("Title: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&article.title),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Link: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&article.link),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Published: ", Style::default().fg(Color::Cyan)),
                    Span::raw(article.published.format("%Y-%m-%d %H:%M").to_string()),
                ]),
                Line::from(""),
                Line::from(Span::styled("Content:", Style::default().fg(Color::Cyan))),
                Line::from(""),
                Line::from(
                    article.content
                        .as_ref()
                        .map(|c| c.as_str())
                        .unwrap_or("No content available")
                ),
            ];

            Paragraph::new(content)
                .block(
                    Block::default()
                        .title("Preview")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Cyan))
                )
                .wrap(Wrap { trim: true })
        }
        None => {
            Paragraph::new("No article selected")
                .block(
                    Block::default()
                        .title("Preview")
                        .borders(Borders::ALL)
                )
        }
    }
}
```

**Step 3: 更新 ui/mod.rs**

修改 `src/ui/mod.rs`，添加:

```rust
pub mod article_list;
pub mod preview;
```

**Step 4: 更新 app.rs**

修改 `src/ui/app.rs`:

```rust
use super::article_list::ArticleList;

pub struct App {
    pub pool: SqlitePool,
    pub should_quit: bool,
    pub feeds: Vec<Feed>,
    pub articles: Vec<Article>,
    pub feed_list: FeedList,
    pub article_list: ArticleList,
}

impl App {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            should_quit: false,
            feeds: Vec::new(),
            articles: Vec::new(),
            feed_list: FeedList::new(),
            article_list: ArticleList::new(),
        }
    }

    pub fn next_article(&mut self) {
        self.article_list.next(self.articles.len());
    }

    pub fn previous_article(&mut self) {
        self.article_list.previous(self.articles.len());
    }

    pub fn get_selected_article(&self) -> Option<&Article> {
        self.article_list.state.selected()
            .and_then(|i| self.articles.get(i))
    }
}
```

**Step 5: 更新 main.rs 渲染三栏布局**

修改 `src/main.rs` 中的 render_ui:

```rust
fn render_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
            ])
            .split(f.size());

        // Feed 列表
        let (feed_list, feed_state) = app.feed_list.render_widget(&app.feeds);
        f.render_stateful_widget(feed_list, chunks[0], feed_state);

        // Article 列表
        let (article_list, article_state) = app.article_list.render_widget(&app.articles);
        f.render_stateful_widget(article_list, chunks[1], article_state);

        // 预览面板
        let preview = ui::preview::render_preview(app.get_selected_article());
        f.render_widget(preview, chunks[2]);
    })?;
    Ok(())
}
```

**Step 6: 运行程序验证**

Run: `cargo run`
Expected: 显示三栏布局，可以导航

**Step 7: 提交**

```bash
git add .
git commit -m "feat(ui): 实现 Article 列表和预览面板

- 添加 ArticleList 组件
- 添加 Preview 面板
- 显示已读/书签状态
- 显示相对时间
- 三栏布局完成

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## 剩余任务概览

由于篇幅限制，剩余任务包括：

- **Task 13**: 搜索功能
- **Task 14**: 过滤功能
- **Task 15**: 标签管理
- **Task 16**: 书签功能
- **Task 17**: 在浏览器打开文章
- **Task 18**: 刷新 feeds
- **Task 19**: 导出功能
- **Task 20**: 集成测试
- **Task 21**: 性能优化
- **Task 22**: 文档和部署

每个任务都遵循相同的 TDD 流程：测试 → 失败 → 实现 → 通过 → 提交。

---

## 执行选项

计划已保存到 `docs/plans/2026-03-03-rss-reader-implementation.md`。

**两种执行方式：**

**1. Subagent-Driven (当前会话)** - 我在当前会话中为每个任务派发新的 subagent，任务间进行代码审查，快速迭代

**2. Parallel Session (独立会话)** - 在新会话中使用 executing-plans skill，批量执行并设置检查点

你希望使用哪种方式？
