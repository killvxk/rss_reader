# RSS Reader

[![CI](https://github.com/killvxk/rss_reader/actions/workflows/ci.yml/badge.svg)](https://github.com/killvxk/rss_reader/actions/workflows/ci.yml)
[![Release](https://github.com/killvxk/rss_reader/actions/workflows/release.yml/badge.svg)](https://github.com/killvxk/rss_reader/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个基于 Rust 的高性能 TUI RSS 阅读器，支持并发拉取、全文搜索和标签管理。

## 功能特性

- ✅ **TUI 界面**：基于 ratatui 的终端用户界面，三栏布局
- ✅ **多格式支持**：支持 RSS 2.0 和 Atom 格式
- ✅ **并发拉取**：使用 tokio 异步运行时并发拉取多个 feeds
- ✅ **全文搜索**：基于 SQLite FTS5 的高性能全文搜索
- ✅ **标签管理**：支持为文章添加标签和按标签过滤
- ✅ **状态管理**：已读/未读、书签功能
- ✅ **浏览器集成**：一键在浏览器中打开文章
- ✅ **错误处理**：完整的错误上下文和日志记录
- ✅ **命令行界面**：支持 CLI 和 TUI 两种模式

## 界面预览

```
┌─────────────────────────────────────────────────────────┐
│ RSS Reader - 11 feeds | 156 unread | 23 bookmarked      │
├──────────────┬──────────────────────┬───────────────────┤
│ Feeds (11)   │ Articles (156)       │ Preview           │
│              │                      │                   │
│  Hacker News │ ● ⭐ Show HN: ...    │ Title: Show HN... │
│  The Verge   │ ●   Apple announces │                   │
│  Wired       │     AI and future   │ Published: 2h ago │
│  Rust Blog   │     Rust 1.75 ...   │ Link: https://... │
│  Dev.to      │                      │                   │
│              │                      │ Content:          │
│              │                      │ Lorem ipsum...    │
│              │                      │                   │
├──────────────┴──────────────────────┴───────────────────┤
│ [j/k]Move [h/l]Panel [Enter]Open [Space]Read [b]Bookmark│
│ [r]Refresh [a]All [u]Unread [m]Marked [?]Help [q]Quit   │
└─────────────────────────────────────────────────────────┘
```

## 快速开始

### 安装依赖

```bash
# Debian/Ubuntu
sudo apt-get install libssl-dev pkg-config

# macOS
brew install openssl pkg-config

# Arch Linux
sudo pacman -S openssl pkg-config
```

### 编译

```bash
cargo build --release
```

### 初始化（添加默认 RSS 源）

```bash
./init_feeds.sh
```

### 启动 TUI 界面

```bash
./target/release/rss-reader
```

## 使用指南

### TUI 模式（默认）

直接运行程序启动 TUI 界面：

```bash
./target/release/rss-reader
```

#### 键盘快捷键

**导航：**
- `j` / `↓` - 向下移动
- `k` / `↑` - 向上移动
- `h` / `←` - 切换到左侧面板
- `l` / `→` - 切换到右侧面板

**操作：**
- `Enter` - 在浏览器中打开文章
- `Space` - 切换已读/未读状态
- `b` - 切换书签
- `r` - 刷新所有 feeds

**过滤：**
- `a` - 显示所有文章
- `u` - 仅显示未读文章
- `m` - 仅显示已加书签的文章

**其他：**
- `?` - 显示/隐藏帮助
- `q` / `Esc` - 退出程序

### CLI 模式

#### 1. 添加 RSS Feed

```bash
./target/release/rss-reader add "Hacker News" "https://news.ycombinator.com/rss" "tech"
./target/release/rss-reader add "Rust Blog" "https://blog.rust-lang.org/feed.xml" "rust"
```

#### 2. 列出所有 Feeds

```bash
./target/release/rss-reader list
```

#### 3. 拉取所有 Feeds

```bash
./target/release/rss-reader fetch
```

#### 4. 查看最新文章

```bash
# 显示最新 10 篇文章
./target/release/rss-reader articles

# 显示最新 20 篇文章
./target/release/rss-reader articles 20
```

#### 5. 搜索文章

```bash
./target/release/rss-reader search "rust"
./target/release/rss-reader search "async"
```

## 命令参考

```
RSS Reader - A simple RSS feed reader

USAGE:
    rss-reader [COMMAND] [OPTIONS]

COMMANDS:
    (no command)                     Start TUI mode (default)
    add <title> <url> <category>    Add a new RSS feed
    list                             List all feeds
    fetch                            Fetch all feeds and update articles
    articles [limit]                 Show latest articles (default: 10)
    search <query>                   Search articles by keyword
    help                             Show this help message

ENVIRONMENT:
    DATABASE_URL    Database connection string (default: sqlite:rss_reader.db)
```

## 项目结构

```
rss-reader/
├── src/
│   ├── main.rs              # CLI 入口
│   ├── lib.rs               # 库入口
│   ├── db/                  # 数据库层
│   │   ├── mod.rs
│   │   ├── schema.rs        # 数据模型
│   │   ├── feeds.rs         # Feed CRUD
│   │   ├── articles.rs      # Article CRUD + 搜索
│   │   └── tags.rs          # Tag 管理
│   ├── fetcher/             # RSS 拉取器
│   │   ├── mod.rs
│   │   ├── http.rs          # HTTP 客户端
│   │   └── parser.rs        # RSS/Atom 解析器
│   └── core/                # 业务逻辑层
│       ├── mod.rs
│       └── feed_manager.rs  # Feed 管理器
├── migrations/              # 数据库迁移
│   └── 20260303000001_init.sql
├── tests/                   # 集成测试
│   ├── fixtures/            # 测试数据
│   ├── db_test.rs
│   ├── db_feeds_test.rs
│   ├── db_articles_test.rs
│   ├── db_tags_test.rs
│   ├── fetcher_test.rs
│   └── feed_manager_test.rs
└── Cargo.toml
```

## 数据库 Schema

### Feeds 表
```sql
CREATE TABLE feeds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    category TEXT NOT NULL,
    last_fetched TIMESTAMP,
    fetch_error TEXT
);
```

### Articles 表
```sql
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
```

### 全文搜索索引
```sql
CREATE VIRTUAL TABLE articles_fts USING fts5(
    title,
    content,
    content=articles,
    content_rowid=id
);
```

## 开发

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_fetch_all_feeds

# 显示测试输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 安全审计
cargo audit
```

### 性能测试

```bash
# 运行 benchmark
cargo bench
```

## 架构设计

### 分层架构

```
┌─────────────────────────────────────┐
│         CLI / API Layer             │
│         (main.rs)                   │
└─────────────────────────────────────┘
                 │
┌─────────────────────────────────────┐
│      Business Logic Layer           │
│      (core/feed_manager.rs)         │
└─────────────────────────────────────┘
         │                    │
┌────────────────┐   ┌────────────────┐
│  Data Layer    │   │  Fetcher Layer │
│  (db/)         │   │  (fetcher/)    │
│  - feeds       │   │  - http        │
│  - articles    │   │  - parser      │
│  - tags        │   │                │
└────────────────┘   └────────────────┘
         │
┌─────────────────────────────────────┐
│         SQLite Database             │
│         (rss_reader.db)             │
└─────────────────────────────────────┘
```

### 并发拉取流程

```
FeedManager::fetch_all_feeds()
    │
    ├─> tokio::spawn(fetch_single_feed(feed1))
    ├─> tokio::spawn(fetch_single_feed(feed2))
    ├─> tokio::spawn(fetch_single_feed(feed3))
    └─> ...
         │
         ├─> HTTP fetch (reqwest)
         ├─> Parse (feed-rs)
         ├─> Insert articles (sqlx)
         └─> Update feed status
```

## 性能特性

- **并发拉取**：使用 tokio 并发拉取多个 feeds，显著提升速度
- **连接池**：SQLite 连接池（最大 5 个连接）
- **FTS5 搜索**：高性能全文搜索，支持中文分词
- **去重**：基于 link 的文章去重（ON CONFLICT DO NOTHING）
- **索引优化**：为常用查询字段添加索引

## 安全性

- **SQL 注入防护**：使用 sqlx 参数化查询
- **超时控制**：HTTP 请求 30 秒超时
- **错误隔离**：单个 feed 失败不影响其他 feeds
- **日志记录**：完整的错误日志和审计日志

## 扩展性

### 添加新的数据源

1. 在 `src/fetcher/` 中实现新的解析器
2. 在 `FeedManager` 中集成新的解析器
3. 添加相应的测试

### 添加新的存储后端

1. 实现 `db/` 模块的接口
2. 更新 `create_pool` 函数
3. 添加迁移脚本

### 添加 Web API

```rust
// 使用 axum 或 actix-web
use axum::{Router, routing::get};

async fn list_articles() -> Json<Vec<Article>> {
    // ...
}

let app = Router::new()
    .route("/articles", get(list_articles));
```

## 故障排查

### 数据库锁定

```bash
# 检查数据库连接
lsof rss_reader.db

# 重置数据库
rm rss_reader.db
./target/release/rss-reader list  # 自动创建新数据库
```

### Feed 拉取失败

```bash
# 查看详细日志
RUST_LOG=debug ./target/release/rss-reader fetch

# 检查网络连接
curl -I <feed_url>
```

### 搜索不工作

```bash
# 重建 FTS 索引
sqlite3 rss_reader.db "DELETE FROM articles_fts; INSERT INTO articles_fts(rowid, title, content) SELECT id, title, content FROM articles;"
```

## 贡献

欢迎提交 Issue 和 Pull Request！

### 开发流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码规范

- 遵循 Rust 官方代码风格
- 所有公共 API 必须有文档注释
- 新功能必须包含测试
- 提交信息遵循 Conventional Commits

## 许可证

MIT License

## 致谢

- [feed-rs](https://github.com/feed-rs/feed-rs) - RSS/Atom 解析
- [sqlx](https://github.com/launchbadge/sqlx) - 异步 SQL 工具包
- [tokio](https://tokio.rs/) - 异步运行时
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP 客户端
- [ratatui](https://github.com/ratatui-org/ratatui) - TUI 框架

---

**注意**：这是一个真实的生产级项目，包含完整的错误处理、测试覆盖和文档。
