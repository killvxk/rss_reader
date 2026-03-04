---
name: rss
description: This skill should be used when the user asks to "总结 RSS", "RSS 摘要", "今日重要新闻", "获取 RSS 消息", or mentions wanting a summary of RSS articles.
---

# RSS 摘要 Skill

## Overview

自动拉取最新 RSS 文章，使用 AI 分析重要性，生成按类别分组的简短摘要。核心原则：AI 筛选 + 分类展示。

## When to Use

**触发条件：**
- 用户想要快速了解今日重要新闻
- 需要从大量 RSS 文章中筛选重点
- 希望按类别查看技术/区块链/其他新闻

**使用方式：**
直接调用：`/rss`

## When NOT to Use

- 需要查看所有文章（使用 `rss-reader articles` 命令）
- 需要搜索特定关键词（使用 `rss-reader search` 命令）
- 需要手动筛选文章（使用 TUI 界面）

## Implementation

### 1. 检查可执行文件

检查 `./target/release/rss-reader` 是否存在。

若不存在，输出以下信息并退出：
```
❌ 找不到 rss-reader 可执行文件
请先构建项目：cd /root/rss_reader && cargo build --release
```

### 2. 拉取最新数据

输出进度提示：
```
🔄 正在拉取最新 RSS...
```

执行命令：
```bash
cd /root/rss_reader && ./target/release/rss-reader fetch
```

解析输出，提取成功数量。若完全失败（0 篇新文章），输出以下信息并退出：
```
❌ 拉取失败，请检查网络连接
```

### 3. 获取文章列表

执行命令：
```bash
cd /root/rss_reader && ./target/release/rss-reader articles --json 100
```

解析 JSON 输出，按 `published` 字段排序，取最新 50 篇的 ID。

若文章列表为空，输出以下信息并退出：
```
📭 数据库中没有文章
建议：检查 RSS 源是否正确配置
运行：./target/release/rss-reader list
```

### 4. 获取完整内容

构造 ID 列表字符串（逗号分隔）。

执行命令：
```bash
cd /root/rss_reader && ./target/release/rss-reader articles --json --with-content --ids=<id列表>
```

解析 JSON 输出。

### 5. 分析文章重要性

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

若筛选后为空，输出以下信息并退出：
```
📭 今日暂无重大消息
```

### 6. 格式化输出

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

- **工具使用**：使用 Bash tool 执行所有命令
- **工作目录**：所有命令都在 /root/rss_reader 目录下执行
- **权限要求**：需要读取权限访问 `./target/release/rss-reader` 可执行文件
- **网络访问**：fetch 命令需要网络连接访问 RSS 源
- **数据解析**：JSON 解析使用标准 JSON 库
- **日期格式**：YYYY-MM-DD
- **超时控制**：整个流程应在 5 分钟内完成，超时自动终止并提示

## Common Mistakes

**错误 1：跳过错误检查**
- 问题：直接执行命令不检查可执行文件是否存在
- 修复：始终先检查 `./target/release/rss-reader` 是否存在

**错误 2：不处理空结果**
- 问题：文章列表为空时继续执行导致错误
- 修复：每个步骤都检查结果，空结果时提前退出并给出提示

**错误 3：忽略 JSON 解析失败**
- 问题：假设 JSON 总是有效的
- 修复：捕获解析错误，显示原始输出并提示版本不兼容
