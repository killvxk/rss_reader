#!/bin/bash
# RSS Reader 初始化脚本 - 添加高频更新的优质 RSS 源
# 只包含更新频繁、活跃度高的源

set -e

DB_URL="${DATABASE_URL:-sqlite:rss_reader.db}"
RSS_READER="./target/release/rss-reader"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 初始化 RSS Reader...${NC}"
echo "数据库: $DB_URL"
echo ""

# 检查可执行文件
if [ ! -f "$RSS_READER" ]; then
    echo -e "${RED}❌ 找不到 rss-reader 可执行文件${NC}"
    echo "请先运行: cargo build --release"
    exit 1
fi

# 添加源的辅助函数
add_feed() {
    local name="$1"
    local url="$2"
    local category="$3"

    if $RSS_READER add "$name" "$url" "$category" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} $name"
    else
        echo -e "${YELLOW}⚠${NC} $name (可能已存在或添加失败)"
    fi
}

echo -e "${BLUE}📰 添加高频更新 RSS 源...${NC}"
echo ""

# ============================================
# RSShub Feeds - 每日更新
# ============================================
echo -e "${YELLOW}[RSShub 聚合 - 每日更新]${NC}"
add_feed "小黑盒新闻" "https://rsshub.umzzz.com/xiaoheihe/news" "rsshub"
add_feed "Readhub 日报" "https://rsshub.umzzz.com/readhub/daily" "rsshub"
add_feed "HelloGitHub" "https://rsshub.umzzz.com/hellogithub/home" "rsshub"
echo ""

# ============================================
# Tech Feeds - 每日多次更新
# ============================================
echo -e "${YELLOW}[技术资讯 - 每日多次更新]${NC}"
add_feed "Hacker News" "https://hnrss.org/frontpage" "tech"
add_feed "The Verge" "https://www.theverge.com/rss/index.xml" "tech"
add_feed "Dev.to" "https://dev.to/feed" "tech"
add_feed "GitHub Trending" "https://mshibanami.github.io/GitHubTrendingRSS/daily/all.xml" "tech"
echo ""

# ============================================
# Security - 安全资讯（中文，每日更新）
# ============================================
echo -e "${YELLOW}[安全资讯 - 中文每日更新]${NC}"
add_feed "FreeBuf" "https://www.freebuf.com/feed" "security"
add_feed "安全客" "https://api.anquanke.com/data/v1/rss" "security"
add_feed "Seebug Paper" "https://paper.seebug.org/rss" "security"
add_feed "嘶吼 RoarTalk" "https://www.4hou.com/feed" "security"
add_feed "SecWiki News" "https://www.sec-wiki.com/news/rss" "security"
add_feed "跳跳糖社区" "https://tttang.com/rss.xml" "security"
add_feed "先知技术社区" "https://xz.aliyun.com/feed" "security"
echo ""

# ============================================
# Security - 国际安全资讯（每日更新）
# ============================================
echo -e "${YELLOW}[国际安全资讯 - 每日更新]${NC}"
add_feed "Krebs on Security" "https://krebsonsecurity.com/feed/" "security"
add_feed "Schneier on Security" "https://www.schneier.com/feed/atom/" "security"
add_feed "The Hacker News" "https://feeds.feedburner.com/TheHackersNews" "security"
add_feed "Bleeping Computer" "https://www.bleepingcomputer.com/feed/" "security"
add_feed "Dark Reading" "https://www.darkreading.com/rss.xml" "security"
echo ""

# ============================================
# Blockchain - 区块链资讯（每日多次更新）
# ============================================
echo -e "${YELLOW}[区块链资讯 - 每日多次更新]${NC}"
add_feed "Decrypt" "https://decrypt.co/feed" "blockchain"
add_feed "The Block" "https://www.theblock.co/rss.xml" "blockchain"
add_feed "BlockBeats" "https://api.theblockbeats.news/v2/rss/all" "blockchain"
add_feed "Odaily" "https://rss.odaily.news/rss/newsflash" "blockchain"
add_feed "CoinDesk" "https://www.coindesk.com/arc/outboundfeeds/rss/" "blockchain"
add_feed "Cointelegraph" "https://cointelegraph.com/rss" "blockchain"
echo ""

# ============================================
# 漏洞情报（每日更新）
# ============================================
echo -e "${YELLOW}[漏洞情报 - 每日更新]${NC}"
add_feed "Seebug 漏洞社区" "https://www.seebug.org/rss/new" "vulnerability"
add_feed "CISA Alerts" "https://www.cisa.gov/cybersecurity-advisories/all.xml" "vulnerability"
add_feed "Packet Storm" "https://rss.packetstormsecurity.com/files/" "vulnerability"
add_feed "Sploitus" "https://sploitus.com/rss" "vulnerability"
add_feed "CXSecurity" "https://cxsecurity.com/wlb/rss/all/" "vulnerability"
add_feed "Bugtraq" "http://seclists.org/rss/bugtraq.rss" "vulnerability"
echo ""

# ============================================
# 威胁研究与分析（每日更新）
# ============================================
echo -e "${YELLOW}[威胁研究 - 每日更新]${NC}"
add_feed "Google Project Zero" "http://googleprojectzero.blogspot.com/feeds/posts/default" "threat"
add_feed "Microsoft Security" "https://www.microsoft.com/security/blog/feed/" "threat"
add_feed "The DFIR Report" "https://thedfirreport.com/feed/" "threat"
add_feed "Malwarebytes Labs" "http://blog.malwarebytes.org/feed/" "threat"
add_feed "Trustwave Blog" "https://www.trustwave.com/en-us/rss/trustwave-blog/" "threat"
add_feed "Qualys Security" "https://community.qualys.com/blogs/securitylabs/feeds/posts" "threat"
echo ""

# ============================================
# 中文安全社区（高频更新）
# ============================================
echo -e "${YELLOW}[中文安全社区 - 高频更新]${NC}"
add_feed "unSafe.sh" "https://buaq.net/rss.xml" "security"
add_feed "腾讯玄武实验室" "https://xlab.tencent.com/cn/atom.xml" "security"
add_feed "360 Netlab" "https://blog.netlab.360.com/rss" "security"
echo ""

echo ""
echo -e "${GREEN}✅ 初始化完成！${NC}"
echo ""
echo -e "${BLUE}📊 统计（仅包含高频更新源）：${NC}"
echo "   - RSShub: 3 个（每日更新）"
echo "   - 技术资讯: 4 个（每日多次更新）"
echo "   - 安全资讯（中文）: 10 个（每日更新）"
echo "   - 国际安全资讯: 5 个（每日更新）"
echo "   - 区块链: 6 个（每日多次更新）"
echo "   - 漏洞情报: 6 个（每日更新）"
echo "   - 威胁研究: 6 个（每日更新）"
echo "   ${GREEN}总计: 40 个高活跃度 RSS 源${NC}"
echo ""
echo -e "${BLUE}📋 查看所有 feeds:${NC}"
echo "   $RSS_READER list"
echo ""
echo -e "${BLUE}🔄 拉取最新文章:${NC}"
echo "   $RSS_READER fetch"
echo ""
echo -e "${BLUE}🖥️  启动 TUI 界面:${NC}"
echo "   $RSS_READER"
echo ""
echo -e "${YELLOW}💡 提示：${NC}"
echo "   - 所有源均为高频更新，建议每天拉取 2-3 次"
echo "   - 首次拉取可能需要 1-2 分钟"
echo "   - 可使用 '$RSS_READER remove <id>' 删除不需要的源"
echo "   - 可使用 '$RSS_READER add <名称> <URL> <分类>' 添加自定义源"
echo ""
