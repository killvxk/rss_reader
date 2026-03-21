#!/bin/bash
# RSS Reader 完整初始化脚本
# 基于 yarb 项目精选的高频更新 RSS 源
# 来源: https://github.com/Vu1nT0tal/yarb

# 不要在错误时退出，因为某些源可能已存在
set +e

DB_URL="${DATABASE_URL:-sqlite:rss_reader.db}"

# 跨平台：检测 Windows 并追加 .exe 后缀
if [[ "$OS" == *"Windows"* ]] || [[ "$(uname -s 2>/dev/null)" == *"MINGW"* ]] || [[ "$(uname -s 2>/dev/null)" == *"MSYS"* ]]; then
    RSS_READER="./target/release/rss-reader.exe"
else
    RSS_READER="./target/release/rss-reader"
fi

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}🚀 RSS Reader 完整初始化${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "数据库: $DB_URL"
echo ""

# 检查可执行文件
if [ ! -f "$RSS_READER" ]; then
    echo -e "${RED}❌ 找不到 rss-reader 可执行文件${NC}"
    echo "请先运行: cargo build --release"
    exit 1
fi

# 统计变量
total_added=0
total_skipped=0

# 添加源的辅助函数
add_feed() {
    local name="$1"
    local url="$2"
    local category="$3"

    if $RSS_READER add "$name" "$url" "$category" 2>/dev/null; then
        echo -e "  ${GREEN}✓${NC} $name"
        ((total_added++))
    else
        echo -e "  ${YELLOW}⚠${NC} $name (已存在或失败)"
        ((total_skipped++))
    fi
}

echo -e "${CYAN}📰 开始添加 RSS 源...${NC}"
echo ""

# ============================================
# 中文安全资讯（每日多次更新）
# ============================================
echo -e "${YELLOW}━━━ 中文安全资讯 ━━━${NC}"
add_feed "FreeBuf" "https://www.freebuf.com/feed" "security"
add_feed "安全客" "https://api.anquanke.com/data/v1/rss" "security"
add_feed "Seebug Paper" "https://paper.seebug.org/rss" "security"
add_feed "嘶吼 RoarTalk" "https://www.4hou.com/feed" "security"
add_feed "SecWiki News" "https://www.sec-wiki.com/news/rss" "security"
add_feed "跳跳糖社区" "https://www.tttang.com/rss.xml" "security"
add_feed "先知技术社区" "https://xz.aliyun.com/feed" "security"
add_feed "unSafe.sh" "https://buaq.net/rss.xml" "security"
add_feed "安全脉搏" "https://www.secpulse.com/feed" "security"
echo ""

# ============================================
# 国际安全资讯（每日更新）
# ============================================
echo -e "${YELLOW}━━━ 国际安全资讯 ━━━${NC}"
add_feed "Krebs on Security" "https://krebsonsecurity.com/feed/" "security"
add_feed "Schneier on Security" "https://www.schneier.com/feed/atom/" "security"
add_feed "The Hacker News" "https://feeds.feedburner.com/TheHackersNews" "security"
add_feed "Bleeping Computer" "https://www.bleepingcomputer.com/feed/" "security"
add_feed "Dark Reading" "https://www.darkreading.com/rss.xml" "security"
add_feed "Darknet" "http://feeds.feedburner.com/darknethackers" "security"
add_feed "Graham Cluley" "http://feeds.feedburner.com/GrahamCluleysBlog" "security"
add_feed "Security Affairs" "http://securityaffairs.co/wordpress/feed" "security"
echo ""

# ============================================
# 企业安全团队（定期更新）
# ============================================
echo -e "${YELLOW}━━━ 企业安全团队 ━━━${NC}"
add_feed "腾讯玄武实验室" "https://xlab.tencent.com/cn/atom.xml" "security"
add_feed "腾讯科恩实验室" "https://keenlab.tencent.com/zh/atom.xml" "security"
add_feed "360 Netlab" "https://blog.netlab.360.com/rss" "security"
add_feed "奇安信 A-TEAM" "https://blog.ateam.qianxin.com/atom.xml" "security"
add_feed "Google Project Zero" "http://googleprojectzero.blogspot.com/feeds/posts/default" "security"
add_feed "Microsoft Security" "https://www.microsoft.com/security/blog/feed/" "security"
add_feed "GitHub Security Lab" "https://securitylab.github.com/research/feed.xml" "security"
echo ""

# ============================================
# 漏洞情报（每日更新）
# ============================================
echo -e "${YELLOW}━━━ 漏洞情报 ━━━${NC}"
add_feed "Seebug 漏洞社区" "https://www.seebug.org/rss/new" "vulnerability"
add_feed "CISA Alerts" "https://www.cisa.gov/cybersecurity-advisories/all.xml" "vulnerability"
add_feed "Packet Storm" "https://rss.packetstormsecurity.com/files/" "vulnerability"
add_feed "Sploitus" "https://sploitus.com/rss" "vulnerability"
add_feed "CXSecurity" "https://cxsecurity.com/wlb/rss/all/" "vulnerability"
add_feed "Bugtraq" "http://seclists.org/rss/bugtraq.rss" "vulnerability"
add_feed "华为安全通告" "https://www.huawei.com/cn/rss-feeds/psirt/rss" "vulnerability"
echo ""

# ============================================
# 威胁研究（每日更新）
# ============================================
echo -e "${YELLOW}━━━ 威胁研究 ━━━${NC}"
add_feed "The DFIR Report" "https://thedfirreport.com/feed/" "threat"
add_feed "Malwarebytes Labs" "http://blog.malwarebytes.org/feed/" "threat"
add_feed "Trustwave Blog" "https://www.trustwave.com/en-us/rss/trustwave-blog/" "threat"
add_feed "Qualys Security" "https://community.qualys.com/blogs/securitylabs/feeds/posts" "threat"
add_feed "Tenable Blog" "https://feeds.feedburner.com/tenable/qaXL" "threat"
add_feed "CrowdStrike" "https://www.crowdstrike.com/blog/feed" "threat"
echo ""

# ============================================
# 渗透测试（定期更新）
# ============================================
echo -e "${YELLOW}━━━ 渗透测试 ━━━${NC}"
add_feed "PortSwigger Blog" "https://portswigger.net/blog/rss" "pentest"
add_feed "Offensive Security" "http://www.offensive-security.com/blog/feed/" "pentest"
add_feed "SpecterOps" "https://posts.specterops.io/feed" "pentest"
add_feed "MDSec" "https://www.mdsec.co.uk/category/penetration-testing/feed/" "pentest"
add_feed "Corelan Team" "https://www.corelan.be/index.php/feed/" "pentest"
add_feed "Hacking Articles" "http://www.hackingarticles.in/feed/" "pentest"
echo ""

# ============================================
# 技术资讯（每日多次更新）
# ============================================
echo -e "${YELLOW}━━━ 技术资讯 ━━━${NC}"
add_feed "Hacker News" "https://hnrss.org/frontpage" "tech"
add_feed "The Verge" "https://www.theverge.com/rss/index.xml" "tech"
add_feed "Dev.to" "https://dev.to/feed" "tech"
add_feed "GitHub Blog" "https://github.blog/feed/" "tech"
add_feed "InfoSec Write-ups" "https://infosecwriteups.com/feed" "tech"
echo ""

# ============================================
# 区块链资讯（每日多次更新）
# ============================================
echo -e "${YELLOW}━━━ 区块链资讯 ━━━${NC}"
add_feed "Decrypt" "https://decrypt.co/feed" "blockchain"
add_feed "The Block" "https://www.theblock.co/rss.xml" "blockchain"
add_feed "BlockBeats" "https://api.theblockbeats.news/v2/rss/all" "blockchain"
add_feed "Odaily" "https://rss.odaily.news/rss/newsflash" "blockchain"
add_feed "CoinDesk" "https://www.coindesk.com/arc/outboundfeeds/rss/" "blockchain"
add_feed "Cointelegraph" "https://cointelegraph.com/rss" "blockchain"
echo ""

# ============================================
# RSShub 聚合（每日更新）
# ============================================
echo -e "${YELLOW}━━━ RSShub 聚合 ━━━${NC}"
add_feed "小黑盒新闻" "https://rsshub.umzzz.com/xiaoheihe/news" "rsshub"
add_feed "Readhub 日报" "https://rsshub.umzzz.com/readhub/daily" "rsshub"
add_feed "HelloGitHub" "https://rsshub.umzzz.com/hellogithub/home" "rsshub"
echo ""

# ============================================
# 中文技术博客（精选高质量）
# ============================================
echo -e "${YELLOW}━━━ 中文技术博客 ━━━${NC}"
add_feed "阮一峰的网络日志" "http://feeds.feedburner.com/ruanyifeng" "blog"
add_feed "酷壳 CoolShell" "http://coolshell.cn/feed" "blog"
add_feed "云风的 BLOG" "http://blog.codingnow.com/atom.xml" "blog"
add_feed "鸟窝" "https://colobu.com/atom.xml" "blog"
add_feed "火丁笔记" "http://huoding.com/feed" "blog"
echo ""

# ============================================
# 安全工具（定期更新）
# ============================================
echo -e "${YELLOW}━━━ 安全工具 ━━━${NC}"
add_feed "Darknet Tools" "https://www.darknet.org.uk/feed/" "tools"
add_feed "KitPloit" "http://feeds.feedburner.com/PentestTools" "tools"
echo ""

# ============================================
# AI 相关（每日更新）
# ============================================
echo -e "${YELLOW}━━━ AI 相关 ━━━${NC}"
add_feed "AI 开发者日报" "https://ainews.liduos.com/rss.xml" "ai"
add_feed "Founder Park" "https://wechat2rss.bestblogs.dev/feed/f940695505f2be1399d23cc98182297cadf6f90d.xml" "ai"
add_feed "Jina AI" "https://jina.ai/feed.rss" "ai"
add_feed "Last Week in AI" "https://lastweekin.ai/feed" "ai"
add_feed "Latent Space" "https://www.latent.space/feed" "ai"
add_feed "The Batch DeepLearning.AI" "https://rsshub.bestblogs.dev/deeplearning/the-batch" "ai"
add_feed "Turing Post" "https://rss.beehiiv.com/feeds/UJIoBuf5BX.xml" "ai"
add_feed "宝玉的分享" "https://s.baoyu.io/feed.xml" "ai"
add_feed "新智元" "https://wechat2rss.bestblogs.dev/feed/e531a18b21c34cf787b83ab444eef659d7a980de.xml" "ai"
add_feed "机器之心" "https://wechat2rss.bestblogs.dev/feed/8d97af31b0de9e48da74558af128a4673d78c9a3.xml" "ai"
add_feed "量子位" "https://www.qbitai.com/feed" "ai"
echo ""

# ============================================
# 个人博客（精选）
# ============================================
echo -e "${YELLOW}━━━ 个人博客 ━━━${NC}"
add_feed "antirez" "https://antirez.com/rss" "blog"
add_feed "Andy Stewart" "https://manateelazycat.github.io/feed.xml" "blog"
add_feed "blog.jsbarretto.com" "https://blog.jsbarretto.com/rss.xml" "blog"
add_feed "Phodal" "https://www.phodal.com/blog/feeds/rss/" "blog"
add_feed "Brendan Gregg" "https://www.brendangregg.com/blog/rss.xml" "blog"
add_feed "CatCoding" "https://catcoding.me/atom.xml" "blog"
add_feed "Coding Horror" "https://blog.codinghorror.com/rss/" "blog"
add_feed "Joel on Software" "https://www.joelonsoftware.com/feed/" "blog"
add_feed "Matt Might" "https://matt.might.net/articles/feed.rss" "blog"
add_feed "Simon Willison" "https://simonwillison.net/atom/everything/" "blog"
add_feed "码农真经的博客" "https://blog.mzh.ren/zh/index.xml" "blog"
add_feed "老苏的blog" "https://laosu.tech/atom.xml" "blog"
add_feed "肖恩聊技术" "https://shawnxie.top/rss.xml" "blog"
add_feed "軟體考古學家" "https://blog.brachiosoft.com/index.xml" "blog"
echo ""

# ============================================
# 技术博客（企业 & 社区）
# ============================================
echo -e "${YELLOW}━━━ 技术博客 ━━━${NC}"
add_feed "AWS Architecture Blog" "https://aws.amazon.com/blogs/architecture/feed/" "tech-blog"
add_feed "AWS Machine Learning Blog" "https://aws.amazon.com/blogs/machine-learning/feed/" "tech-blog"
add_feed "LlamaIndex Blog" "https://www.llamaindex.ai/blog/feed" "tech-blog"
add_feed "ByteByteGo Newsletter" "https://blog.bytebytego.com/feed" "tech-blog"
add_feed "Docker" "https://www.docker.com/feed/" "tech-blog"
add_feed "Engineering at Meta" "https://engineering.fb.com/feed/" "tech-blog"
add_feed "Engineering at Slack" "https://slack.engineering/feed/" "tech-blog"
add_feed "Grafana Labs Engineering" "https://grafana.com/categories/engineering/index.xml" "tech-blog"
add_feed "freeCodeCamp" "https://www.freecodecamp.org/news/rss/" "tech-blog"
add_feed "Hugging Face Blog" "https://huggingface.co/blog/feed.xml" "tech-blog"
add_feed "IntelliJ IDEA Blog" "https://blog.jetbrains.com/idea/feed" "tech-blog"
add_feed "Java Code Geeks" "http://feeds.feedburner.com/JavaCodeGeeks" "tech-blog"
add_feed "LangChain Blog" "https://blog.langchain.com/rss/" "tech-blog"
add_feed "Martin Fowler" "https://martinfowler.com/feed.atom" "tech-blog"
add_feed "Microservice architecture" "https://microservices.io/feed.xml" "tech-blog"
add_feed "Microsoft Azure Blog" "https://azure.microsoft.com/en-us/blog/feed/" "tech-blog"
add_feed "Microsoft Research Blog" "https://www.microsoft.com/en-us/research/blog/feed/" "tech-blog"
add_feed "Netflix TechBlog" "https://netflixtechblog.com/feed" "tech-blog"
add_feed "Redis" "https://redis.io/feed/" "tech-blog"
add_feed "Sagyam Blog" "https://blog.sagyamthapa.com.np/rss.xml" "tech-blog"
add_feed "Spring" "https://spring.io/blog.atom" "tech-blog"
add_feed "Stack Overflow Blog" "http://blog.stackoverflow.com/feed/" "tech-blog"
add_feed "Apache Software Foundation" "https://news.apache.org/feed" "tech-blog"
add_feed "The Cloudflare Blog" "https://blog.cloudflare.com/rss" "tech-blog"
add_feed "The New Stack" "https://thenewstack.io/feed/" "tech-blog"
add_feed "Uber Engineering Blog" "https://www.uber.com/blog/engineering/rss/" "tech-blog"
add_feed "美团技术团队" "https://tech.meituan.com/feed/" "tech-blog"
echo ""

# ============================================
# 技术周刊（每周更新）
# ============================================
echo -e "${YELLOW}━━━ 技术周刊 ━━━${NC}"
add_feed "1Link.Fun 科技周刊" "https://techhub.social/users/1link.rss" "weekly"
add_feed "54321 Weekly" "https://54321.versun.me/feed/" "weekly"
add_feed "AIGC Weekly" "https://quaily.com/op7418/feed/atom" "weekly"
add_feed "Better Dev Link" "https://betterdev.link/rss.xml" "weekly"
add_feed "HelloGitHub 月刊" "https://hellogithub.com/rss" "weekly"
add_feed "不死鸟" "https://iui.su/feed/" "weekly"
add_feed "二丫讲梵-学习周刊" "https://wiki.eryajf.net/rss.xml" "weekly"
add_feed "大橘和朋友们的周刊" "https://rrorangeandfriends.de/feed.xml" "weekly"
add_feed "月球背面" "https://moonvy.com/blog/rss.xml" "weekly"
add_feed "潮流周刊" "https://weekly.tw93.fun/rss.xml" "weekly"
add_feed "粥里有勺糖" "https://sugarat.top/feed.rss" "weekly"
add_feed "老胡的周刊" "https://weekly.howie6879.com/rss/rss.xml" "weekly"
add_feed "肖恩技术周刊" "https://weekly.shawnxie.top/rss.xml" "weekly"
add_feed "让小产品的独立变现更简单" "https://www.ezindie.com/feed/rss.xml" "weekly"
add_feed "豌豆花下猫" "https://pythoncat.top/rss.xml" "weekly"
add_feed "阿猫的博客" "https://ameow.xyz/feed.xml" "weekly"
add_feed "龙爪槐守望者" "https://www.ftium4.com/rss.xml" "weekly"
echo ""

# ============================================
# 热点资讯（每日更新）
# ============================================
echo -e "${YELLOW}━━━ 热点资讯 ━━━${NC}"
add_feed "36氪" "https://36kr.com/feed" "news"
add_feed "InfoQ" "https://feed.infoq.com/" "news"
add_feed "InfoQ 推荐" "https://plink.anyfeeder.com/infoq/recommend" "news"
add_feed "MIT 科技评论" "https://rsshub.bestblogs.dev/mittrchina/hot" "news"
add_feed "TechCrunch" "https://techcrunch.com/feed/" "news"
add_feed "SuperTechFans" "https://www.supertechfans.com/cn/index.xml" "news"
add_feed "奇客Solidot" "https://www.solidot.org/index.rss" "news"
add_feed "少数派" "https://sspai.com/feed" "news"
add_feed "晚点" "https://rsshub.bestblogs.dev/latepost" "news"
add_feed "极客公园" "https://www.geekpark.net/rss" "news"
add_feed "爱范儿" "https://www.ifanr.com/feed" "news"
add_feed "蓝点网" "https://www.landiannews.com/feed" "news"
echo ""

# ============================================
# 社区（每日更新）
# ============================================
echo -e "${YELLOW}━━━ 社区 ━━━${NC}"
add_feed "LINUX DO 最新话题" "https://linux.do/latest.rss" "community"
add_feed "LINUX DO 热门话题" "https://linux.do/top.rss" "community"
add_feed "V2EX" "https://v2ex.com/index.xml" "community"
echo ""

# ============================================
# 资源（定期更新）
# ============================================
echo -e "${YELLOW}━━━ 资源 ━━━${NC}"
add_feed "GitHub Weekly Trending" "https://mshibanami.github.io/GitHubTrendingRSS/weekly/all.xml" "resources"
add_feed "Mac玩儿法" "https://www.waerfa.com/feed" "resources"
add_feed "小众软件" "https://www.appinn.com/feed/" "resources"
add_feed "异次元软件世界" "https://feed.iplaysoft.com/" "resources"
echo ""

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ 初始化完成！${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${CYAN}📊 统计信息：${NC}"
echo "   ✓ 成功添加: ${GREEN}$total_added${NC} 个源"
echo "   ⚠ 跳过/失败: ${YELLOW}$total_skipped${NC} 个源"
echo ""
echo -e "${CYAN}📋 分类统计：${NC}"
echo "   • 中文安全资讯: 9 个"
echo "   • 国际安全资讯: 8 个"
echo "   • 企业安全团队: 7 个"
echo "   • 漏洞情报: 7 个"
echo "   • 威胁研究: 6 个"
echo "   • 渗透测试: 6 个"
echo "   • 技术资讯: 5 个"
echo "   • 区块链资讯: 6 个"
echo "   • RSShub 聚合: 3 个"
echo "   • 中文技术博客: 5 个"
echo "   • 安全工具: 2 个"
echo "   • AI 相关: 11 个"
echo "   • 个人博客: 14 个"
echo "   • 技术博客: 27 个"
echo "   • 技术周刊: 17 个"
echo "   • 热点资讯: 12 个"
echo "   • 社区: 3 个"
echo "   • 资源: 4 个"
echo "   ${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "   ${GREEN}总计: 152 个 RSS 源${NC}"
echo ""
echo -e "${CYAN}🔧 常用命令：${NC}"
echo "   查看所有源:  $RSS_READER list"
echo "   拉取文章:    $RSS_READER fetch"
echo "   启动 TUI:    $RSS_READER"
echo "   删除源:      $RSS_READER remove <id>"
echo ""
echo -e "${YELLOW}💡 提示：${NC}"
echo "   • 所有源均为高频更新，建议每天拉取 2-3 次"
echo "   • 首次拉取约需 2-3 分钟，请耐心等待"
echo "   • 部分源可能因网络原因暂时无法访问"
echo "   • 可根据需要删除不感兴趣的源"
echo ""
