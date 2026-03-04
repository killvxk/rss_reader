#!/bin/bash
# RSS Reader 初始化脚本 - 添加默认 RSS 源

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

echo "📰 添加默认 RSS 源..."
echo ""

# RSShub Feeds (有效的源)
echo "添加 RSShub 类 RSS..."
$RSS_READER add "小黑盒新闻" "https://rsshub.umzzz.com/xiaoheihe/news" "rsshub"
$RSS_READER add "Readhub 日报" "https://rsshub.umzzz.com/readhub/daily" "rsshub"
$RSS_READER add "HelloGitHub" "https://rsshub.umzzz.com/hellogithub/home" "rsshub"
$RSS_READER add "RSShub 官方" "https://rsshub.isrss.com/" "rsshub"
$RSS_READER add "RSShub Mirror 1" "https://rsshub.cups.moe/" "rsshub"
$RSS_READER add "RSShub Mirror 2" "https://rss.spriple.org/" "rsshub"

# Tech Feeds (有效的源)
echo "添加科技类 RSS..."
$RSS_READER add "Hacker News" "https://hnrss.org/frontpage" "tech"
$RSS_READER add "The Verge" "https://www.theverge.com/rss/index.xml" "tech"
$RSS_READER add "UX Design" "https://uxdesign.cc/feed" "tech"
$RSS_READER add "Rust Blog" "https://blog.rust-lang.org/feed.xml" "tech"
$RSS_READER add "GitHub Blog" "https://github.blog/feed/" "tech"
$RSS_READER add "Dev.to" "https://dev.to/feed" "tech"

# Blockchain Feeds (有效的源)
echo "添加区块链类 RSS..."
$RSS_READER add "Decrypt" "https://decrypt.co/feed" "blockchain"
$RSS_READER add "The Block" "https://www.theblock.co/rss.xml" "blockchain"
$RSS_READER add "BlockBeats" "https://api.theblockbeats.news/v2/rss/all" "blockchain"
$RSS_READER add "Odaily" "https://rss.odaily.news/rss/newsflash" "blockchain"

echo ""
echo "✅ 初始化完成！已添加 16 个有效 RSS 源"
echo ""
echo "📊 统计："
echo "   - RSShub: 6 个"
echo "   - Tech: 6 个"
echo "   - Blockchain: 4 个"
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
