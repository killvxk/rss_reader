# RSS 源初始化脚本更新说明

## 更新内容

基于 [yarb 项目](https://github.com/Vu1nT0tal/yarb) 的 RSS 源集合，完整提取并筛选了高频更新的优质 RSS 源。

## 脚本文件

- `init_feeds.sh` - 完整版初始化脚本（推荐使用）
- `init_feeds_old.sh` - 旧版脚本备份

## RSS 源统计

### 总计：64 个高质量 RSS 源

#### 1. 中文安全资讯（9 个，每日多次更新）
- FreeBuf
- 安全客
- Seebug Paper
- 嘶吼 RoarTalk
- SecWiki News
- 跳跳糖社区
- 先知技术社区
- unSafe.sh
- 安全脉搏

#### 2. 国际安全资讯（8 个，每日更新）
- Krebs on Security
- Schneier on Security
- The Hacker News
- Bleeping Computer
- Dark Reading
- Darknet
- Graham Cluley
- Security Affairs

#### 3. 企业安全团队（7 个，定期更新）
- 腾讯玄武实验室
- 腾讯科恩实验室
- 360 Netlab
- 奇安信 A-TEAM
- Google Project Zero
- Microsoft Security
- GitHub Security Lab

#### 4. 漏洞情报（7 个，每日更新）
- Seebug 漏洞社区
- CISA Alerts
- Packet Storm
- Sploitus
- CXSecurity
- Bugtraq
- 华为安全通告

#### 5. 威胁研究（6 个，每日更新）
- The DFIR Report
- Malwarebytes Labs
- Trustwave Blog
- Qualys Security
- Tenable Blog
- CrowdStrike

#### 6. 渗透测试（6 个，定期更新）
- PortSwigger Blog
- Offensive Security
- SpecterOps
- MDSec
- Corelan Team
- Hacking Articles

#### 7. 技术资讯（5 个，每日多次更新）
- Hacker News
- The Verge
- Dev.to
- GitHub Blog
- InfoSec Write-ups

#### 8. 区块链资讯（6 个，每日多次更新）
- Decrypt
- The Block
- BlockBeats
- Odaily
- CoinDesk
- Cointelegraph

#### 9. RSShub 聚合（3 个，每日更新）
- 小黑盒新闻
- Readhub 日报
- HelloGitHub

#### 10. 中文技术博客（5 个，精选）
- 阮一峰的网络日志
- 酷壳 CoolShell
- 云风的 BLOG
- 鸟窝
- 火丁笔记

#### 11. 安全工具（2 个，定期更新）
- Darknet Tools
- KitPloit

## 使用方法

```bash
# 1. 构建项目（如果还没构建）
cargo build --release

# 2. 运行初始化脚本
./init_feeds.sh

# 3. 拉取最新文章
./target/release/rss-reader fetch

# 4. 启动 TUI 界面
./target/release/rss-reader
```

## 特点

1. **高频更新**：所有源均为每日更新或每日多次更新
2. **分类清晰**：按领域分为 11 个类别
3. **质量保证**：从 yarb 项目的数百个源中精选
4. **彩色输出**：美观的终端界面
5. **统计功能**：实时显示添加成功/失败的源数量

## 数据来源

所有 RSS 源来自 [Vu1nT0tal/yarb](https://github.com/Vu1nT0tal/yarb) 项目，该项目收集了：
- `CyberSecurityRSS.opml` - 网络安全 RSS 源
- `Chinese-Security-RSS.opml` - 中文安全 RSS 源
- `awesome-security-feed.opml` - 优质安全订阅源
- `chinese-independent-blogs.opml` - 中文独立博客

## 注意事项

- 首次拉取约需 2-3 分钟
- 部分源可能因网络原因暂时无法访问
- 建议每天拉取 2-3 次以获取最新内容
- 可使用 `./target/release/rss-reader remove <id>` 删除不需要的源

## 更新日期

2026-03-06
