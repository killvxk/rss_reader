# RSS 摘要 Skill 实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 创建 RSS 摘要 skill，自动拉取、筛选并总结重要 RSS 消息

**Architecture:** 扩展 rss-reader CLI 添加 JSON 输出支持，创建 Claude Code skill 调用 CLI 获取数据，使用 AI 分析重要性并生成分类摘要

**Tech Stack:** Rust (rss-reader CLI), Markdown (skill), Bash (命令执行), JSON (数据交换)

---

## Task 1: 添加数据库查询函数

**Files:**
- Modify: `src/db/articles.rs`
- Test: `tests/db_articles_test.rs`

**Step 1: 编写测试用例**

在 `tests/db_articles_test.rs` 添加：

```rust
#[tokio::test]
async fn test_get_articles_by_ids() {
    let pool = create_pool(":memory:").await.unwrap();

    // 插入测试数据
    let feed_id = insert_feed(&pool, "Test Feed", "https://test.com/rss", "tech")
        .await
        .unwrap();

    let article1_id = insert_article(
        &pool,
        feed_id,
        "Article 1",
        "https://test.com/1",
        Some("Content 1"),
        "2026-03-04T10:00:00Z",
    )
    .await
    .unwrap();

    let article2_id = insert_article(
        &pool,
        feed_id,
        "Article 2",
        "https://test.com/2",
        Some("Content 2"),
        "2026-03-04T11:00:00Z",
    )
    .await
    .unwrap();

    // 测试按 ID 查询
    let articles = get_articles_by_ids(&pool, &[article1_id, article2_id])
        .await
        .unwrap();

    assert_eq!(articles.len(), 2);
    assert_eq!(articles[0].title, "Article 2"); // 按时间倒序
    assert_eq!(articles[1].title, "Article 1");
}
```

**Step 2: 运行测试验证失败**

```bash
cargo test test_get_articles_by_ids
```

Expected: 编译错误 "cannot find function `get_articles_by_ids`"

**Step 3: 实现查询函数**

在 `src/db/articles.rs` 添加：

```rust
pub async fn get_articles_by_ids(
    pool: &SqlitePool,
    ids: &[i64],
) -> Result<Vec<Article>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = ids.iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query = format!(
        "SELECT * FROM articles WHERE id IN ({}) ORDER BY published DESC",
        placeholders
    );

    let mut query_builder = sqlx::query_as::<_, Article>(&query);
    for id in ids {
        query_builder = query_builder.bind(id);
    }

    query_builder
        .fetch_all(pool)
        .await
        .context("Failed to fetch articles by IDs")
}
```

**Step 4: 运行测试验证通过**

```bash
cargo test test_get_articles_by_ids
```

Expected: PASS

**Step 5: 提交**

```bash
git add src/db/articles.rs tests/db_articles_test.rs
git commit -m "feat: 添加按 ID 查询文章的数据库函数

- 实现 get_articles_by_ids 函数
- 支持批量查询指定 ID 的文章
- 按发布时间倒序排列
- 添加单元测试验证功能

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 2: 添加 JSON 输出支持

**Files:**
- Modify: `src/main.rs`
- Modify: `Cargo.toml`

**Step 1: 添加 serde_json 依赖**

```bash
cargo add serde_json
```

**Step 2: 创建 JSON 输出函数**

在 `src/main.rs` 添加（在 `print_usage()` 函数之前）：

```rust
fn output_articles_json(
    articles: &[rss_reader::db::schema::Article],
    with_content: bool,
) -> anyhow::Result<()> {
    use serde_json::json;

    let articles_json: Vec<serde_json::Value> = articles
        .iter()
        .map(|article| {
            let mut obj = json!({
                "id": article.id,
                "feed_id": article.feed_id,
                "title": article.title,
                "link": article.link,
                "published": article.published,
                "is_read": article.is_read,
                "is_bookmarked": article.is_bookmarked,
            });

            if with_content {
                if let Some(content) = &article.content {
                    obj["content"] = json!(content);
                }
            }

            obj
        })
        .collect();

    let output = json!({
        "articles": articles_json,
        "total": articles.len(),
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
```

**Step 3: 修改 articles 命令支持参数**

在 `src/main.rs` 的 `"articles"` 分支中修改：

```rust
"articles" => {
    use rss_reader::db::articles;

    // 解析参数
    let mut limit = 10;
    let mut json_output = false;
    let mut with_content = false;
    let mut ids: Option<Vec<i64>> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => json_output = true,
            "--with-content" => with_content = true,
            arg if arg.starts_with("--ids=") => {
                let ids_str = arg.strip_prefix("--ids=").unwrap();
                ids = Some(
                    ids_str
                        .split(',')
                        .filter_map(|id| id.parse::<i64>().ok())
                        .collect()
                );
            }
            arg => {
                if let Ok(num) = arg.parse::<usize>() {
                    limit = num;
                }
            }
        }
        i += 1;
    }

    // 查询文章
    let articles = if let Some(ids) = ids {
        articles::get_articles_by_ids(&pool, &ids).await?
    } else {
        articles::get_all_articles(&pool, limit, 0).await?
    };

    // 输出
    if json_output {
        output_articles_json(&articles, with_content)?;
    } else {
        // 原有的人类可读格式输出
        if articles.is_empty() {
            println!("No articles found. Fetch feeds first with: rss-reader fetch");
        } else {
            println!("\n📄 Latest Articles ({}):", articles.len());
            println!("{:-<80}", "");
            for article in articles {
                let read_marker = if article.is_read { "✓" } else { " " };
                let bookmark_marker = if article.is_bookmarked { "⭐" } else { " " };
                println!("  [{}{}] {}", read_marker, bookmark_marker, article.title);
                println!("      {}", article.link);
                println!("      Published: {}", article.published);
                println!();
            }
        }
    }
}
```

**Step 4: 测试 JSON 输出**

```bash
cargo build --release
./target/release/rss-reader articles --json 5
```

Expected: 输出 JSON 格式的文章列表

**Step 5: 测试带内容的 JSON 输出**

```bash
./target/release/rss-reader articles --json --with-content 5
```

Expected: JSON 中包含 content 字段

**Step 6: 测试按 ID 查询**

```bash
./target/release/rss-reader articles --json --ids=1,2,3
```

Expected: 只返回指定 ID 的文章

**Step 7: 运行 fmt 和 clippy**

```bash
cargo fmt
cargo clippy --all-targets --all-features
```

Expected: 无警告

**Step 8: 提交**

```bash
git add src/main.rs Cargo.toml
git commit -m "feat: 添加 articles 命令的 JSON 输出支持

- 添加 --json 参数输出 JSON 格式
- 添加 --with-content 参数包含文章内容
- 添加 --ids 参数按 ID 查询文章
- 保持向后兼容，默认人类可读格式

使用示例：
  rss-reader articles --json 10
  rss-reader articles --json --with-content --ids=1,2,3

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 3: 创建 RSS 摘要 Skill

**Files:**
- Create: `.claude/skills/rss-summary.md`

**Step 1: 创建 skill 文件**

创建 `.claude/skills/rss-summary.md`：

```markdown
---
name: rss-summary
description: 获取并总结今日重要 RSS 消息，使用 AI 智能筛选重大新闻
---

# RSS 摘要 Skill

自动拉取最新 RSS 文章，使用 AI 分析重要性，生成按类别分组的简短摘要。

## 使用方法

直接调用：`/rss-summary`

## 执行流程

### 步骤 1：检查 rss-reader 可执行文件

检查 `./target/release/rss-reader` 是否存在。

如果不存在，输出：
```
❌ 找不到 rss-reader 可执行文件
请先构建项目：cd /root/rss_reader && cargo build --release
```

然后退出。

### 步骤 2：拉取最新数据

输出进度提示：
```
🔄 正在拉取最新 RSS...
```

执行命令：
```bash
cd /root/rss_reader && ./target/release/rss-reader fetch
```

解析输出，提取成功数量。如果完全失败（0 篇新文章），输出：
```
❌ 拉取失败，请检查网络连接
```

然后退出。

### 步骤 3：获取文章列表

执行命令：
```bash
cd /root/rss_reader && ./target/release/rss-reader articles --json 100
```

解析 JSON 输出，按 `published` 字段排序，取最新 50 篇的 ID。

如果文章列表为空，输出：
```
📭 数据库中没有文章
建议：检查 RSS 源是否正确配置
运行：./target/release/rss-reader list
```

然后退出。

### 步骤 4：获取完整内容

构造 ID 列表字符串（逗号分隔）。

执行命令：
```bash
cd /root/rss_reader && ./target/release/rss-reader articles --json --with-content --ids=<id列表>
```

解析 JSON 输出。

### 步骤 5：AI 分析重要性

输出进度提示：
```
🤖 正在分析文章重要性...
```

使用以下提示词分析文章：

```
分析以下 RSS 文章，筛选出重要和重大的消息。

判断标准：
- 技术突破、产品发布、重大事件
- 行业影响力大的新闻
- 有实质性内容的深度文章
- 排除：日常更新、小修小补、个人博客琐事、重复内容

请返回筛选后的文章列表，每篇用一句话概括核心内容（20-30 字）。
按重要性排序，只保留真正重要的消息。

返回 JSON 格式：
{
  "important_articles": [
    {
      "id": 123,
      "category": "tech",
      "title": "原标题",
      "summary": "一句话概括"
    }
  ]
}

文章数据：
<文章 JSON 数据>
```

解析 AI 返回的 JSON。

如果筛选后为空，输出：
```
📭 今日暂无重大消息
```

然后退出。

### 步骤 6：格式化输出

按 `category` 分组，生成格式化输出：

```
📰 今日重要 RSS 摘要 (YYYY-MM-DD)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔧 Tech (N 条)

• <摘要>
  来源: <title>

• <摘要>
  来源: <title>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💰 Blockchain (N 条)

• <摘要>
  来源: <title>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 RSShub (N 条)

• <摘要>
  来源: <title>

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

共筛选出 N 条重要消息（从 50 篇文章中）
```

类别图标映射：
- tech: 🔧
- blockchain: 💰
- rsshub: 📊
- 其他: 📰

## 错误处理

- 所有命令执行失败：显示错误信息并退出
- JSON 解析失败：显示原始输出，提示版本不兼容
- 超时（5 分钟）：自动终止并提示

## 注意事项

- 使用 Bash tool 执行所有命令
- 所有命令都在 /root/rss_reader 目录下执行
- JSON 解析使用标准 JSON 库
- 日期格式：YYYY-MM-DD
```

**Step 2: 测试 skill**

在 Claude Code 中测试：
```
/rss-summary
```

验证：
1. 能够正确拉取数据
2. 能够解析 JSON
3. AI 分析返回合理结果
4. 输出格式正确

**Step 3: 提交**

```bash
git add .claude/skills/rss-summary.md
git commit -m "feat: 创建 RSS 摘要 skill

功能：
- 自动拉取最新 RSS 文章
- AI 智能筛选重要消息
- 按类别分组生成摘要
- 完善的错误处理和进度提示

使用方式：
  /rss-summary

输出格式：
- 按类别分组（tech, blockchain, rsshub）
- 每条消息一句话概括
- 显示来源和统计信息

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 4: 更新文档

**Files:**
- Modify: `README.md`

**Step 1: 添加 skill 使用说明**

在 `README.md` 的"使用指南"部分添加：

```markdown
### RSS 摘要 Skill

使用 Claude Code skill 自动获取并总结重要 RSS 消息：

```bash
/rss-summary
```

功能：
- 自动拉取最新 RSS 文章
- AI 智能筛选重要消息（过滤琐碎内容）
- 按类别分组生成简短摘要
- 显示来源和统计信息

输出示例：
```
📰 今日重要 RSS 摘要 (2026-03-04)

🔧 Tech (3 条)
• Rust 1.78 发布，引入新的异步运行时优化
  来源: Rust Blog
...

共筛选出 6 条重要消息（从 50 篇文章中）
```
```

**Step 2: 更新 CLI 命令文档**

在 `README.md` 的"命令参考"部分更新 `articles` 命令：

```markdown
    articles [limit] [--json] [--with-content] [--ids=<ids>]
                                 Show latest articles (default: 10)
                                 --json: Output in JSON format
                                 --with-content: Include article content
                                 --ids: Query specific article IDs
```

**Step 3: 提交**

```bash
git add README.md
git commit -m "docs: 添加 RSS 摘要 skill 使用说明

- 添加 /rss-summary skill 使用文档
- 更新 articles 命令参数说明
- 添加输出示例

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 5: 版本更新和发布

**Files:**
- Modify: `Cargo.toml`

**Step 1: 更新版本号**

在 `Cargo.toml` 中：

```toml
[package]
version = "0.3.0"
```

**Step 2: 运行完整测试**

```bash
cargo test
cargo clippy --all-targets --all-features
cargo build --release
```

Expected: 所有测试通过，无警告

**Step 3: 提交并推送**

```bash
git add Cargo.toml
git commit -m "chore: 更新版本号至 0.3.0

新功能：
- 添加 articles 命令 JSON 输出支持
- 添加按 ID 查询文章功能
- 创建 RSS 摘要 skill

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"

git push origin main
```

**Step 4: 创建并推送 tag**

```bash
git tag v0.3.0
git push origin v0.3.0
```

Expected: GitHub Actions 自动构建 release

---

## 验收标准

### 功能验收

- [ ] `rss-reader articles --json` 输出正确的 JSON 格式
- [ ] `rss-reader articles --json --with-content` 包含文章内容
- [ ] `rss-reader articles --json --ids=1,2,3` 只返回指定文章
- [ ] `/rss-summary` skill 能够成功执行
- [ ] AI 筛选出的消息确实重要且相关
- [ ] 输出格式清晰易读，按类别分组

### 质量验收

- [ ] 所有单元测试通过
- [ ] 无 clippy 警告
- [ ] 代码格式化正确
- [ ] 文档完整准确

### 性能验收

- [ ] fetch 操作在 5 分钟内完成
- [ ] JSON 解析无性能问题
- [ ] AI 分析在合理时间内完成（< 30 秒）

---

## 后续优化建议

1. **缓存机制**：在数据库中添加 `importance_score` 字段，避免重复分析
2. **个性化配置**：支持用户自定义关键词和筛选规则
3. **定时任务**：支持定时运行并发送通知
4. **多语言支持**：支持中英文摘要切换
