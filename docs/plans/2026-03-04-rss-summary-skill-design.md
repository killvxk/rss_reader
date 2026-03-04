# RSS 摘要 Skill 设计文档

**日期：** 2026-03-04
**作者：** Claude Opus 4.6
**状态：** 设计完成，待实现

## 概述

创建一个 Claude Code skill，用于自动获取并总结 RSS 阅读器中的重要消息。该 skill 将调用 rss-reader CLI 工具，使用 AI 智能筛选重大新闻，并生成按类别分组的简短摘要。

## 需求

### 功能需求

1. **自动拉取最新数据**：调用 `rss-reader fetch` 获取最新 RSS 文章
2. **智能筛选**：使用 AI 分析文章重要性，过滤琐碎消息
3. **简短摘要**：每篇重要文章生成一句话概括
4. **分类展示**：按 RSS 源类别（tech, blockchain, rsshub）分组
5. **数量自适应**：由 AI 决定筛选出多少条消息

### 非功能需求

1. **性能优化**：只分析最新 50 篇文章，避免处理过多数据
2. **错误处理**：优雅处理各种失败场景
3. **用户体验**：显示进度提示，提供清晰的输出格式

## 架构设计

### 整体架构

```
用户调用 /rss-summary skill
    ↓
Skill 执行流程：
    1. 执行 rss-reader fetch（拉取最新数据）
    2. 执行 rss-reader articles --json 100（获取文章列表）
    3. 按时间排序，取最新 50 篇的 ID
    4. 执行 rss-reader articles --json --with-content --ids=<id列表>
    5. 将文章数据发送给 Claude 进行重要性分析
    6. Claude 筛选重要文章并生成摘要
    7. 按类别分组格式化输出
```

### 核心组件

#### 1. rss-reader CLI 扩展

需要添加以下新参数：

**`--json` 参数**
- 功能：输出 JSON 格式而非人类可读格式
- 用法：`rss-reader articles --json`
- 输出格式：
  ```json
  {
    "articles": [
      {
        "id": 123,
        "feed_id": 5,
        "feed_title": "Hacker News",
        "category": "tech",
        "title": "Article Title",
        "link": "https://...",
        "published": "2026-03-04T10:30:00Z",
        "is_read": false,
        "is_bookmarked": false
      }
    ],
    "total": 150
  }
  ```

**`--with-content` 参数**
- 功能：在 JSON 输出中包含文章完整内容
- 用法：`rss-reader articles --json --with-content`
- 说明：默认不包含 content 字段以减少数据量

**`--ids` 参数**
- 功能：只获取指定 ID 的文章
- 用法：`rss-reader articles --json --ids=123,124,125`
- 说明：用于第二阶段只获取需要的文章完整内容

#### 2. Claude Code Skill

**文件位置：** `.claude/skills/rss-summary.md`

**Skill 元数据：**
```yaml
name: rss-summary
description: 获取并总结今日重要 RSS 消息
```

**核心功能：**
1. 命令执行：使用 Bash tool 调用 rss-reader
2. JSON 解析：处理命令输出
3. AI 分析：构造提示词进行重要性判断
4. 格式化输出：生成用户友好的摘要

## 数据流设计

### 第零步：拉取最新数据

```bash
rss-reader fetch
```

**输出示例：**
```
🔄 Fetching all feeds...

✓ Fetch complete:
  - 16 feeds fetched successfully
  - 42 new articles added
```

**处理：**
- 显示进度提示："正在拉取最新 RSS..."
- 解析输出，提取成功数量
- 如果全部失败，提示用户检查网络

### 第一步：获取文章列表

```bash
rss-reader articles --json 100
```

**处理：**
- 解析 JSON 输出
- 按 `published` 字段排序（最新在前）
- 提取前 50 篇的 ID 列表

### 第二步：获取完整内容

```bash
rss-reader articles --json --with-content --ids=123,124,125,...
```

**处理：**
- 只获取 50 篇文章的完整内容
- 减少数据传输和处理量

### 第三步：AI 分析

**提示词模板：**
```
分析以下 RSS 文章，筛选出重要和重大的消息。

判断标准：
- 技术突破、产品发布、重大事件
- 行业影响力大的新闻
- 有实质性内容的深度文章
- 排除：日常更新、小修小补、个人博客琐事、重复内容

请返回筛选后的文章列表，每篇用一句话概括核心内容（20-30 字）。
按重要性排序，只保留真正重要的消息。

返回格式：
{
  "important_articles": [
    {
      "id": 123,
      "category": "tech",
      "feed_title": "Hacker News",
      "summary": "一句话概括"
    }
  ]
}

文章数据：
[JSON 数据]
```

### 第四步：格式化输出

**输出格式：**
```
📰 今日重要 RSS 摘要 (2026-03-04)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔧 Tech (3 条)

• Rust 1.78 发布，引入新的异步运行时优化
  来源: Rust Blog

• GitHub 推出 AI 代码审查功能，支持自动检测安全漏洞
  来源: GitHub Blog

• Meta 开源新一代 LLM 模型 Llama 3，性能超越 GPT-4
  来源: Hacker News

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💰 Blockchain (2 条)

• 比特币突破 $70,000，创历史新高
  来源: The Block

• 以太坊完成 Dencun 升级，Gas 费用降低 90%
  来源: Decrypt

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 RSShub (1 条)

• HelloGitHub 月刊发布，推荐 50 个优质开源项目
  来源: HelloGitHub

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

共筛选出 6 条重要消息（从 50 篇文章中）
```

**格式说明：**
- 标题显示日期
- 分类图标：tech 🔧, blockchain 💰, rsshub 📊
- 每条消息：一句话摘要 + 来源
- 分隔线清晰区分类别
- 底部显示统计信息

## 错误处理

### 错误场景与处理策略

| 错误场景 | 检测方式 | 处理策略 |
|---------|---------|---------|
| rss-reader 不存在 | 检查文件是否存在 | 提示用户构建项目，退出 skill |
| fetch 部分失败 | 解析输出中的成功数量 | 显示警告，继续执行 |
| fetch 完全失败 | 0 篇新文章 | 提示检查网络，退出 skill |
| 数据库为空 | articles 返回空列表 | 提示可能是首次运行，建议检查 feeds |
| JSON 解析失败 | JSON.parse 异常 | 显示原始输出，提示版本不兼容 |
| AI 筛选后为空 | 返回空列表 | 提示"今日暂无重大消息" |

### 用户体验优化

1. **进度提示**
   - "正在拉取最新 RSS..."（fetch 阶段）
   - "正在分析文章重要性..."（AI 分析阶段）

2. **超时处理**
   - fetch 超过 5 分钟自动终止
   - 提示用户可能网络问题或源过多

3. **优雅降级**
   - 如果部分步骤失败，尽量输出可用结果
   - 例如：fetch 部分失败，仍然分析已有数据

## 实现细节

### CLI 命令实现（src/main.rs）

```rust
"articles" => {
    // 解析参数
    let limit = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
    let json_output = args.contains(&"--json".to_string());
    let with_content = args.contains(&"--with-content".to_string());
    let ids = args.iter()
        .find(|arg| arg.starts_with("--ids="))
        .map(|arg| arg.strip_prefix("--ids=").unwrap())
        .map(|ids_str| {
            ids_str.split(',')
                .filter_map(|id| id.parse::<i64>().ok())
                .collect::<Vec<_>>()
        });

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
        output_articles_human(&articles)?;
    }
}
```

### 需要添加的数据库函数

在 `src/db/articles.rs` 中添加：

```rust
pub async fn get_articles_by_ids(
    pool: &SqlitePool,
    ids: &[i64],
) -> Result<Vec<Article>> {
    let ids_str = ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let query = format!(
        "SELECT a.*, f.title as feed_title, f.category
         FROM articles a
         JOIN feeds f ON a.feed_id = f.id
         WHERE a.id IN ({})
         ORDER BY a.published DESC",
        ids_str
    );

    sqlx::query_as(&query)
        .fetch_all(pool)
        .await
        .context("Failed to fetch articles by IDs")
}
```

### Skill 实现伪代码

```markdown
---
name: rss-summary
description: 获取并总结今日重要 RSS 消息
---

1. 检查 rss-reader 是否存在
   - 如果不存在，提示构建并退出

2. 执行 fetch
   - 显示进度："正在拉取最新 RSS..."
   - 运行 `rss-reader fetch`
   - 解析输出，检查是否成功

3. 获取文章列表
   - 运行 `rss-reader articles --json 100`
   - 解析 JSON
   - 按时间排序，取前 50 篇 ID

4. 获取完整内容
   - 构造 ID 列表字符串
   - 运行 `rss-reader articles --json --with-content --ids=<ids>`
   - 解析 JSON

5. AI 分析
   - 显示进度："正在分析文章重要性..."
   - 构造提示词
   - 调用 Claude 分析
   - 解析返回的重要文章列表

6. 格式化输出
   - 按 category 分组
   - 添加图标和分隔线
   - 输出格式化的摘要
```

## 测试计划

### 单元测试

1. **CLI 参数解析**
   - 测试 `--json` 输出格式正确
   - 测试 `--with-content` 包含内容字段
   - 测试 `--ids` 只返回指定文章

2. **JSON 序列化**
   - 测试所有字段正确序列化
   - 测试特殊字符转义

### 集成测试

1. **完整流程测试**
   - 从 fetch 到输出的完整流程
   - 验证输出格式正确

2. **错误场景测试**
   - 数据库为空
   - fetch 失败
   - JSON 解析失败

### 手动测试

1. **真实数据测试**
   - 使用实际 RSS 源
   - 验证 AI 筛选质量
   - 检查输出可读性

2. **性能测试**
   - 测试 50 篇文章的处理时间
   - 验证 fetch 超时机制

## 后续优化方向

1. **缓存机制**
   - 在数据库中添加 `importance_score` 字段
   - 批量分析并缓存结果
   - 减少重复分析

2. **个性化筛选**
   - 支持用户自定义关键词
   - 支持排除特定主题
   - 支持调整重要性阈值

3. **多语言支持**
   - 支持中英文摘要
   - 自动检测文章语言

4. **通知集成**
   - 定时运行并发送通知
   - 集成 Slack/Email/Webhook

## 总结

本设计采用混合方案（方案 C），平衡了性能和准确性：
- 通过两阶段查询减少数据传输
- 只分析最新 50 篇文章，避免过载
- 使用 AI 智能筛选，确保质量
- 提供清晰的输出格式和错误处理

实现后，用户只需运行 `/rss-summary` 即可获得今日重要新闻的简洁摘要。
