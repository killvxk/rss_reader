# RSS 阅读器设计文档

**项目名称**: Rust TUI RSS Reader
**创建日期**: 2026-03-03
**状态**: 已批准

## 1. 项目概述

### 1.1 目标
创建一个基于 Rust 的终端 TUI 模式 RSS 阅读器，支持快速浏览和信息管理功能。

### 1.2 核心需求
- **快速浏览**: 在终端快速扫描标题，标记感兴趣的文章，稍后在浏览器打开
- **信息管理**: 标记、分类、搜索、导出文章，作为个人知识库
- **默认 RSS 源**: https://gist.githubusercontent.com/discountry/80a05b0123f340014f98b6099a3ae5ab/raw/c119d890c71402ca2535fc8dce2743efe3302b25/2026-my-rss.md
- **RSS 源数量**: 28 个 feeds（rsshub: 15, tech: 8, blockchain: 5）

### 1.3 技术选型

| 组件 | 技术栈 | 理由 |
|------|--------|------|
| TUI 框架 | ratatui | 最活跃的 Rust TUI 库，支持复杂布局 |
| 异步运行时 | tokio | 高性能异步 I/O，适合并发拉取 RSS |
| RSS 解析 | feed-rs | 支持 RSS/Atom 多种格式 |
| 数据库 | sqlx + SQLite | 类型安全，支持 FTS5 全文搜索 |
| HTTP 客户端 | reqwest | 成熟的异步 HTTP 库 |

## 2. 系统架构

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                    TUI Layer (ratatui)                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Feed List    │  │ Article List │  │ Preview Pane │  │
│  │ Panel        │  │ Panel        │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │ Command Bar (search/filter/shortcuts)            │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                          ↕
┌─────────────────────────────────────────────────────────┐
│              Application Core (tokio)                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Feed Manager │  │ Article      │  │ Tag/Bookmark │  │
│  │              │  │ Manager      │  │ Manager      │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                          ↕
┌─────────────────────────────────────────────────────────┐
│              Data Layer (sqlx + SQLite)                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ feeds table  │  │ articles     │  │ tags/        │  │
│  │              │  │ table + FTS5 │  │ bookmarks    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                          ↕
┌─────────────────────────────────────────────────────────┐
│         External Services (reqwest + feed-rs)           │
│         RSS/Atom Feeds (28 sources)                     │
└─────────────────────────────────────────────────────────┘
```

### 2.2 模块划分

1. **UI 层** (`src/ui/`) - 负责渲染和用户交互
2. **业务逻辑层** (`src/core/`) - Feed 管理、文章处理、标签系统
3. **数据持久化层** (`src/db/`) - SQLite 存储 + 导出功能
4. **网络层** (`src/fetcher/`) - 异步拉取和解析 RSS

## 3. 数据模型

### 3.1 SQLite Schema

```sql
-- Feeds 表
CREATE TABLE feeds (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    category TEXT,  -- rsshub/tech/blockchain
    last_fetched TIMESTAMP,
    fetch_error TEXT
);

-- Articles 表
CREATE TABLE articles (
    id INTEGER PRIMARY KEY,
    feed_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    link TEXT UNIQUE NOT NULL,
    content TEXT,
    published TIMESTAMP,
    is_read BOOLEAN DEFAULT 0,
    is_bookmarked BOOLEAN DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (feed_id) REFERENCES feeds(id)
);

-- 全文搜索索引
CREATE VIRTUAL TABLE articles_fts USING fts5(
    title, content,
    content=articles,
    content_rowid=id
);

-- Tags 表
CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

-- Article-Tag 关联表
CREATE TABLE article_tags (
    article_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (article_id, tag_id),
    FOREIGN KEY (article_id) REFERENCES articles(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
```

### 3.2 Rust 数据结构

```rust
// 核心数据结构
struct Feed {
    id: i64,
    title: String,
    url: String,
    category: String,
}

struct Article {
    id: i64,
    feed_id: i64,
    title: String,
    link: String,
    content: Option<String>,
    published: DateTime<Utc>,
    is_read: bool,
    is_bookmarked: bool,
}

// 应用状态
struct AppState {
    db: SqlitePool,
    feeds: Vec<Feed>,
    articles: Vec<Article>,
    selected_feed: Option<usize>,
    selected_article: Option<usize>,
    search_query: String,
    filter_mode: FilterMode,
}

enum FilterMode {
    All,
    Unread,
    Bookmarked,
    ByTag(String),
    ByFeed(i64),
}
```

## 4. UI 设计

### 4.1 主界面布局

```
┌─────────────────────────────────────────────────────────┐
│ RSS Reader - 28 feeds | 156 unread          [? Help]    │
├──────────────┬──────────────────────┬───────────────────┤
│ Feeds (28)   │ Articles (156)       │ Preview           │
│              │                      │                   │
│ ▸ rsshub(15) │ ● [HN] Show HN: ... │ Title: Show HN... │
│ ▾ tech (8)   │ ● [Verge] Apple ... │                   │
│   HackerNews │   [Wired] AI and... │ Published: 2h ago │
│   TheVerge   │   [Dev.to] Rust ... │ Source: HN        │
│   Wired      │                      │                   │
│ ▸ blockchain │                      │ Content:          │
│              │                      │ Lorem ipsum...    │
│              │                      │                   │
├──────────────┴──────────────────────┴───────────────────┤
│ [/]Search [f]Filter [t]Tag [b]Bookmark [r]Refresh [q]Quit│
└─────────────────────────────────────────────────────────┘
```

### 4.2 交互设计

**键盘快捷键:**
- `j/k` 或 `↑/↓`: 上下移动
- `h/l` 或 `←/→`: 左右切换面板
- `Enter`: 在浏览器打开文章
- `Space`: 标记已读/未读
- `b`: 添加/移除书签
- `t`: 添加标签
- `/`: 打开搜索
- `f`: 打开过滤器
- `r`: 刷新所有 feeds
- `e`: 导出文章
- `?`: 显示帮助
- `q`: 退出

**弹窗交互:**
- 搜索弹窗: 实时搜索，显示结果列表
- 标签弹窗: 输入标签名，支持自动补全
- 过滤弹窗: 选择过滤条件（未读/已读/书签/标签/日期）
- 导出弹窗: 选择格式（JSON/Markdown/HTML）

## 5. 关键流程

### 5.1 启动流程

```
1. 初始化 SQLite 数据库
   ↓
2. 加载 RSS 列表（从 URL 或本地缓存）
   ↓
3. 并发拉取所有 feeds (tokio::spawn × 28)
   ├─ 成功：解析并存入数据库
   └─ 失败：记录错误，继续其他 feeds
   ↓
4. 启动 TUI 界面
   ↓
5. 进入事件循环（键盘输入 + 定时刷新）
```

### 5.2 并发拉取策略

```rust
async fn fetch_all_feeds(feeds: Vec<Feed>) -> Vec<FetchResult> {
    let tasks: Vec<_> = feeds
        .into_iter()
        .map(|feed| tokio::spawn(fetch_single_feed(feed)))
        .collect();

    // 等待所有任务完成，超时 30 秒
    let results = timeout(
        Duration::from_secs(30),
        join_all(tasks)
    ).await;

    // 处理超时和错误
}
```

### 5.3 数据流

**文章更新流程:**
```
用户按 'r' 刷新
  ↓
显示加载指示器
  ↓
异步拉取所有 feeds
  ↓
解析新文章 → 去重（按 link）
  ↓
批量插入数据库
  ↓
更新 UI 显示
  ↓
显示通知："已更新 42 篇新文章"
```

**搜索流程:**
```
用户按 '/' 打开搜索框
  ↓
输入查询词（实时搜索）
  ↓
SQL: SELECT * FROM articles_fts WHERE articles_fts MATCH ?
  ↓
显示结果列表
  ↓
用户选择 → 跳转到文章详情
```

## 6. 错误处理

### 6.1 错误处理策略

| 错误类型 | 处理方式 |
|---------|---------|
| 网络超时 | 跳过该 feed，记录错误，不阻塞其他 feeds |
| RSS 解析失败 | 记录原始内容，标记为解析失败，继续 |
| 数据库写入失败 | 重试 3 次，失败则记录日志 |
| SQLite 锁定 | 使用 WAL 模式避免锁定 |
| 磁盘空间不足 | 提示用户清理旧文章 |

### 6.2 日志策略

- 使用 `tracing` 库记录日志
- 日志级别: ERROR/WARN/INFO/DEBUG
- 日志文件: `~/.local/share/rss-reader/logs/app.log`
- 日志轮转: 每天一个文件，保留 7 天

## 7. 测试策略

### 7.1 单元测试

**数据库层测试:**
```rust
#[tokio::test]
async fn test_insert_and_query_article() { }

#[tokio::test]
async fn test_fts_search() { }

#[tokio::test]
async fn test_tag_operations() { }

#[tokio::test]
async fn test_bookmark_filter() { }
```

**RSS 解析测试:**
```rust
#[tokio::test]
async fn test_parse_rss2() { }

#[tokio::test]
async fn test_parse_atom() { }

#[tokio::test]
async fn test_parse_rsshub() { }

#[tokio::test]
async fn test_malformed_feed() { }
```

**业务逻辑测试:**
```rust
#[tokio::test]
async fn test_filter_unread() { }

#[tokio::test]
async fn test_export_to_json() { }

#[tokio::test]
async fn test_export_to_markdown() { }
```

### 7.2 集成测试

```rust
#[tokio::test]
async fn test_full_workflow() {
    // 初始化 → 拉取 → 搜索 → 标记 → 导出
}

#[tokio::test]
async fn test_concurrent_fetch() {
    // 使用 mock HTTP server 测试并发拉取
}

#[tokio::test]
async fn test_error_recovery() {
    // 模拟网络失败，验证错误恢复
}
```

### 7.3 TUI 自动化测试

```rust
#[test]
fn test_render_feed_list() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    // 验证渲染输出
}

#[test]
fn test_keyboard_navigation() {
    // 模拟键盘输入，验证状态变化
}
```

### 7.4 Mock 和 Fixture

- 使用 `wiremock` 或 `mockito` 模拟 HTTP 请求
- 测试数据存放在 `tests/fixtures/` 目录
- 包含各种 RSS/Atom 格式样本

### 7.5 性能测试

```rust
fn benchmark_search(c: &mut Criterion) {
    c.bench_function("search 10k articles", |b| {
        b.iter(|| { /* 搜索逻辑 */ });
    });
}
```

### 7.6 覆盖率目标

- 数据库层: ≥ 90%
- RSS 解析: ≥ 85%
- 业务逻辑: ≥ 80%
- UI 层: ≥ 60%
- **总体: ≥ 80%**

### 7.7 CI/CD 自动化

```yaml
# .github/workflows/ci.yml
- name: Run tests with coverage
  run: cargo tarpaulin --out Xml

- name: Check coverage threshold
  run: |
    # 要求 >= 80% 覆盖率
```

## 8. 性能目标

- **启动时间**: < 10 秒（拉取 28 个 feeds）
- **UI 刷新率**: 60 FPS
- **搜索响应**: < 100ms（10,000 篇文章）
- **内存占用**: < 50MB（正常使用）

## 9. 功能清单

### 9.1 核心功能（MVP）

- [x] 从 URL 加载 RSS 列表
- [x] 并发拉取 28 个 feeds
- [x] 解析 RSS/Atom 格式
- [x] SQLite 存储文章
- [x] TUI 双栏布局
- [x] 文章列表浏览
- [x] 标记已读/未读
- [x] 在浏览器打开文章

### 9.2 信息管理功能

- [x] 全文搜索（FTS5）
- [x] 标签系统
- [x] 收藏/书签
- [x] 按 feed/日期/状态过滤
- [x] 导出为 JSON/Markdown

### 9.3 高级功能（可选）

- [ ] 定时自动刷新
- [ ] 文章去重（相似度检测）
- [ ] 阅读统计
- [ ] 主题配置
- [ ] 快捷键自定义

## 10. 项目结构

```
rss_reader/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── feed_list.rs
│   │   ├── article_list.rs
│   │   ├── preview.rs
│   │   └── popups.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── feed_manager.rs
│   │   ├── article_manager.rs
│   │   └── tag_manager.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── schema.rs
│   │   └── queries.rs
│   └── fetcher/
│       ├── mod.rs
│       ├── http.rs
│       └── parser.rs
├── tests/
│   ├── integration_test.rs
│   └── fixtures/
│       ├── rss2_sample.xml
│       ├── atom_sample.xml
│       └── feeds_list.md
└── docs/
    └── plans/
        └── 2026-03-03-rss-reader-design.md
```

## 11. 依赖清单

```toml
[dependencies]
ratatui = "0.26"
tokio = { version = "1.36", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
feed-rs = "1.4"
reqwest = { version = "0.11", features = ["json"] }
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
wiremock = "0.6"
criterion = "0.5"
tarpaulin = "0.27"
```

## 12. 下一步

1. 调用 `writing-plans` skill 创建详细实现计划
2. 按阶段实施开发
3. 每个阶段完成后运行测试并保存进度
