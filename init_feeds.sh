#!/bin/bash
# RSS Reader 初始化脚本 - 添加默认 RSS 源（28 个）

set -e

DB_URL="${DATABASE_URL:-sqlite:rss_reader.db}"
RSS_READER="./target/release/rss-reader"

echo "🚀 初始化 RSS Reader..."
echo "数据库: $DB_URL"
echo ""

# 检查可执行文件
if [ ! -f "$RSS_READER" ]; then
    echo "❌ 找不到 rss-reader 可执行文件"
    echo "请先运行: cargo build --release"
    exit 1
fi

echo "📰 添加默认 RSS 源（28 个）..."
echo ""

# RSShub Feeds (15 个)
echo "添加 RSShub 类 RSS (15 个)..."
$RSS_READER add "V2EX 热门" "https://rsshub.umzzz.com/v2ex/topics/hot" "rsshub"
$RSS_READER add "豆瓣实时热门" "https://rsshub.umzzz.com/douban/list/subject_real_time_hotest" "rsshub"
$RSS_READER add "掘金周榜" "https://rsshub.umzzz.com/juejin/trending/all/weekly" "rsshub"
$RSS_READER add "财新网" "https://rsshub.umzzz.com/caixin/article" "rsshub"
$RSS_READER add "知乎热榜" "https://rsshub.umzzz.com/zhihu/hot" "rsshub"
$RSS_READER add "少数派" "https://rsshub.umzzz.com/sspai/index" "rsshub"
$RSS_READER add "哔哩哔哩周刊" "https://rsshub.umzzz.com/bilibili/weekly" "rsshub"
$RSS_READER add "Dev.to 周榜" "https://rsshub.umzzz.com/dev.to/top/week" "rsshub"
$RSS_READER add "小黑盒新闻" "https://rsshub.umzzz.com/xiaoheihe/news" "rsshub"
$RSS_READER add "Readhub 日报" "https://rsshub.umzzz.com/readhub/daily" "rsshub"
$RSS_READER add "HelloGitHub" "https://rsshub.umzzz.com/hellogithub/home" "rsshub"
$RSS_READER add "RSShub 1" "https://rsshub.isrss.com/" "rsshub"
$RSS_READER add "RSShub 2" "https://rss.datuan.dev/" "rsshub"
$RSS_READER add "RSShub 3" "https://rsshub.cups.moe/" "rsshub"
$RSS_READER add "RSShub 4" "https://rss.spriple.org/" "rsshub"

# Tech Feeds (8 个)
echo "添加科技类 RSS (8 个)..."
$RSS_READER add "Hacker News" "https://hnrss.org/frontpage" "tech"
$RSS_READER add "The Verge" "https://www.theverge.com/rss/index.xml" "tech"
$RSS_READER add "Wired" "https://www.wired.com/feed/rss" "tech"
$RSS_READER add "Reddit Technology" "https://www.reddit.com/r/technology/top.rss?t=day" "tech"
$RSS_READER add "Simon Willison" "https://simonwillison.net/atom/everything/" "tech"
$RSS_READER add "Pragmatic Engineer" "https://blog.pragmaticengineer.com/rss/" "tech"
$RSS_READER add "CSS-Tricks" "https://css-tricks.com/feed/" "tech"
$RSS_READER add "UX Design" "https://uxdesign.cc/feed" "tech"

# Blockchain Feeds (5 个)
echo "添加区块链类 RSS (5 个)..."
$RSS_READER add "Decrypt" "https://decrypt.co/feed" "blockchain"
$RSS_READER add "The Block" "https://www.theblock.co/rss.xml" "blockchain"
$RSS_READER add "BlockBeats" "https://api.theblockbeats.news/v2/rss/all" "blockchain"
$RSS_READER add "PANews" "https://www.panewslab.com/rss.xml?lang=zh&type=NORMAL%2CNEWS%2CVIDEO&featured=true&in-depth=true" "blockchain"
$RSS_READER add "Odaily" "https://rss.odaily.news/rss/newsflash" "blockchain"

echo ""
echo "✅ 初始化完成！已添加 28 个 RSS 源"
echo ""
echo "📊 统计："
echo "   - RSShub: 15 个"
echo "   - Tech: 8 个"
echo "   - Blockchain: 5 个"
echo ""
echo "📋 查看所有 feeds:"
$RSS_READER list
echo ""
echo "🔄 拉取最新文章:"
echo "   $RSS_READER fetch"
echo ""
echo "🖥️  启动 TUI 界面:"
echo "   $RSS_READER"
echo ""
