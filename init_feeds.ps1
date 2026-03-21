# RSS Reader 完整初始化脚本 (PowerShell)
# 基于 yarb 项目精选的高频更新 RSS 源
# 来源: https://github.com/Vu1nT0tal/yarb

$ErrorActionPreference = "Continue"

$DB_URL = if ($env:DATABASE_URL) { $env:DATABASE_URL } else { "sqlite:rss_reader.db" }
$RSS_READER = ".\target\release\rss-reader.exe"

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Blue
Write-Host "  RSS Reader 完整初始化" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Blue
Write-Host "数据库: $DB_URL"
Write-Host ""

# 检查可执行文件
if (-not (Test-Path $RSS_READER)) {
    Write-Host "  找不到 rss-reader 可执行文件" -ForegroundColor Red
    Write-Host "请先运行: cargo build --release"
    exit 1
}

# 统计变量
$script:totalAdded = 0
$script:totalSkipped = 0

# 添加源的辅助函数
function Add-Feed {
    param(
        [string]$Name,
        [string]$Url,
        [string]$Category
    )

    $output = & $RSS_READER add $Name $Url $Category 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  + $Name" -ForegroundColor Green
        $script:totalAdded++
    } else {
        Write-Host "  ! $Name (已存在或失败)" -ForegroundColor Yellow
        $script:totalSkipped++
    }
}

Write-Host "  开始添加 RSS 源..." -ForegroundColor Cyan
Write-Host ""

# ============================================
# 中文安全资讯（每日多次更新）
# ============================================
Write-Host "--- 中文安全资讯 ---" -ForegroundColor Yellow
Add-Feed "FreeBuf" "https://www.freebuf.com/feed" "security"
Add-Feed "安全客" "https://api.anquanke.com/data/v1/rss" "security"
Add-Feed "Seebug Paper" "https://paper.seebug.org/rss" "security"
Add-Feed "嘶吼 RoarTalk" "https://www.4hou.com/feed" "security"
Add-Feed "SecWiki News" "https://www.sec-wiki.com/news/rss" "security"
Add-Feed "跳跳糖社区" "https://www.tttang.com/rss.xml" "security"
Add-Feed "先知技术社区" "https://xz.aliyun.com/feed" "security"
Add-Feed "unSafe.sh" "https://buaq.net/rss.xml" "security"
Add-Feed "安全脉搏" "https://www.secpulse.com/feed" "security"
Write-Host ""

# ============================================
# 国际安全资讯（每日更新）
# ============================================
Write-Host "--- 国际安全资讯 ---" -ForegroundColor Yellow
Add-Feed "Krebs on Security" "https://krebsonsecurity.com/feed/" "security"
Add-Feed "Schneier on Security" "https://www.schneier.com/feed/atom/" "security"
Add-Feed "The Hacker News" "https://feeds.feedburner.com/TheHackersNews" "security"
Add-Feed "Bleeping Computer" "https://www.bleepingcomputer.com/feed/" "security"
Add-Feed "Dark Reading" "https://www.darkreading.com/rss.xml" "security"
Add-Feed "Darknet" "http://feeds.feedburner.com/darknethackers" "security"
Add-Feed "Graham Cluley" "http://feeds.feedburner.com/GrahamCluleysBlog" "security"
Add-Feed "Security Affairs" "http://securityaffairs.co/wordpress/feed" "security"
Write-Host ""

# ============================================
# 企业安全团队（定期更新）
# ============================================
Write-Host "--- 企业安全团队 ---" -ForegroundColor Yellow
Add-Feed "腾讯玄武实验室" "https://xlab.tencent.com/cn/atom.xml" "security"
Add-Feed "腾讯科恩实验室" "https://keenlab.tencent.com/zh/atom.xml" "security"
Add-Feed "360 Netlab" "https://blog.netlab.360.com/rss" "security"
Add-Feed "奇安信 A-TEAM" "https://blog.ateam.qianxin.com/atom.xml" "security"
Add-Feed "Google Project Zero" "http://googleprojectzero.blogspot.com/feeds/posts/default" "security"
Add-Feed "Microsoft Security" "https://www.microsoft.com/security/blog/feed/" "security"
Add-Feed "GitHub Security Lab" "https://securitylab.github.com/research/feed.xml" "security"
Write-Host ""

# ============================================
# 漏洞情报（每日更新）
# ============================================
Write-Host "--- 漏洞情报 ---" -ForegroundColor Yellow
Add-Feed "Seebug 漏洞社区" "https://www.seebug.org/rss/new" "vulnerability"
Add-Feed "CISA Alerts" "https://www.cisa.gov/cybersecurity-advisories/all.xml" "vulnerability"
Add-Feed "Packet Storm" "https://rss.packetstormsecurity.com/files/" "vulnerability"
Add-Feed "Sploitus" "https://sploitus.com/rss" "vulnerability"
Add-Feed "CXSecurity" "https://cxsecurity.com/wlb/rss/all/" "vulnerability"
Add-Feed "Bugtraq" "http://seclists.org/rss/bugtraq.rss" "vulnerability"
Add-Feed "华为安全通告" "https://www.huawei.com/cn/rss-feeds/psirt/rss" "vulnerability"
Write-Host ""

# ============================================
# 威胁研究（每日更新）
# ============================================
Write-Host "--- 威胁研究 ---" -ForegroundColor Yellow
Add-Feed "The DFIR Report" "https://thedfirreport.com/feed/" "threat"
Add-Feed "Malwarebytes Labs" "http://blog.malwarebytes.org/feed/" "threat"
Add-Feed "Trustwave Blog" "https://www.trustwave.com/en-us/rss/trustwave-blog/" "threat"
Add-Feed "Qualys Security" "https://community.qualys.com/blogs/securitylabs/feeds/posts" "threat"
Add-Feed "Tenable Blog" "https://feeds.feedburner.com/tenable/qaXL" "threat"
Add-Feed "CrowdStrike" "https://www.crowdstrike.com/blog/feed" "threat"
Write-Host ""

# ============================================
# 渗透测试（定期更新）
# ============================================
Write-Host "--- 渗透测试 ---" -ForegroundColor Yellow
Add-Feed "PortSwigger Blog" "https://portswigger.net/blog/rss" "pentest"
Add-Feed "Offensive Security" "http://www.offensive-security.com/blog/feed/" "pentest"
Add-Feed "SpecterOps" "https://posts.specterops.io/feed" "pentest"
Add-Feed "MDSec" "https://www.mdsec.co.uk/category/penetration-testing/feed/" "pentest"
Add-Feed "Corelan Team" "https://www.corelan.be/index.php/feed/" "pentest"
Add-Feed "Hacking Articles" "http://www.hackingarticles.in/feed/" "pentest"
Write-Host ""

# ============================================
# 技术资讯（每日多次更新）
# ============================================
Write-Host "--- 技术资讯 ---" -ForegroundColor Yellow
Add-Feed "Hacker News" "https://hnrss.org/frontpage" "tech"
Add-Feed "The Verge" "https://www.theverge.com/rss/index.xml" "tech"
Add-Feed "Dev.to" "https://dev.to/feed" "tech"
Add-Feed "GitHub Blog" "https://github.blog/feed/" "tech"
Add-Feed "InfoSec Write-ups" "https://infosecwriteups.com/feed" "tech"
Write-Host ""

# ============================================
# 区块链资讯（每日多次更新）
# ============================================
Write-Host "--- 区块链资讯 ---" -ForegroundColor Yellow
Add-Feed "Decrypt" "https://decrypt.co/feed" "blockchain"
Add-Feed "The Block" "https://www.theblock.co/rss.xml" "blockchain"
Add-Feed "BlockBeats" "https://api.theblockbeats.news/v2/rss/all" "blockchain"
Add-Feed "Odaily" "https://rss.odaily.news/rss/newsflash" "blockchain"
Add-Feed "CoinDesk" "https://www.coindesk.com/arc/outboundfeeds/rss/" "blockchain"
Add-Feed "Cointelegraph" "https://cointelegraph.com/rss" "blockchain"
Write-Host ""

# ============================================
# RSShub 聚合（每日更新）
# ============================================
Write-Host "--- RSShub 聚合 ---" -ForegroundColor Yellow
Add-Feed "小黑盒新闻" "https://rsshub.umzzz.com/xiaoheihe/news" "rsshub"
Add-Feed "Readhub 日报" "https://rsshub.umzzz.com/readhub/daily" "rsshub"
Add-Feed "HelloGitHub" "https://rsshub.umzzz.com/hellogithub/home" "rsshub"
Write-Host ""

# ============================================
# 中文技术博客（精选高质量）
# ============================================
Write-Host "--- 中文技术博客 ---" -ForegroundColor Yellow
Add-Feed "阮一峰的网络日志" "http://feeds.feedburner.com/ruanyifeng" "blog"
Add-Feed "酷壳 CoolShell" "http://coolshell.cn/feed" "blog"
Add-Feed "云风的 BLOG" "http://blog.codingnow.com/atom.xml" "blog"
Add-Feed "鸟窝" "https://colobu.com/atom.xml" "blog"
Add-Feed "火丁笔记" "http://huoding.com/feed" "blog"
Write-Host ""

# ============================================
# 安全工具（定期更新）
# ============================================
Write-Host "--- 安全工具 ---" -ForegroundColor Yellow
Add-Feed "Darknet Tools" "https://www.darknet.org.uk/feed/" "tools"
Add-Feed "KitPloit" "http://feeds.feedburner.com/PentestTools" "tools"
Write-Host ""

# ============================================
# AI 相关
# ============================================
Write-Host "--- AI 相关 ---" -ForegroundColor Yellow
Add-Feed "AI 开发者日报" "https://ainews.liduos.com/rss.xml" "ai"
Add-Feed "Founder Park" "https://wechat2rss.bestblogs.dev/feed/f940695505f2be1399d23cc98182297cadf6f90d.xml" "ai"
Add-Feed "Jina AI" "https://jina.ai/feed.rss" "ai"
Add-Feed "Last Week in AI" "https://lastweekin.ai/feed" "ai"
Add-Feed "Latent Space" "https://www.latent.space/feed" "ai"
Add-Feed "The Batch DeepLearning.AI" "https://rsshub.bestblogs.dev/deeplearning/the-batch" "ai"
Add-Feed "Turing Post" "https://rss.beehiiv.com/feeds/UJIoBuf5BX.xml" "ai"
Add-Feed "宝玉的分享" "https://s.baoyu.io/feed.xml" "ai"
Add-Feed "新智元" "https://wechat2rss.bestblogs.dev/feed/e531a18b21c34cf787b83ab444eef659d7a980de.xml" "ai"
Add-Feed "机器之心" "https://wechat2rss.bestblogs.dev/feed/8d97af31b0de9e48da74558af128a4673d78c9a3.xml" "ai"
Add-Feed "量子位" "https://www.qbitai.com/feed" "ai"
Write-Host ""

# ============================================
# 个人博客
# ============================================
Write-Host "--- 个人博客 ---" -ForegroundColor Yellow
Add-Feed "antirez" "https://antirez.com/rss" "blog"
Add-Feed "Andy Stewart" "https://manateelazycat.github.io/feed.xml" "blog"
Add-Feed "blog.jsbarretto.com" "https://blog.jsbarretto.com/rss.xml" "blog"
Add-Feed "Phodal" "https://www.phodal.com/blog/feeds/rss/" "blog"
Add-Feed "Brendan Gregg" "https://www.brendangregg.com/blog/rss.xml" "blog"
Add-Feed "CatCoding" "https://catcoding.me/atom.xml" "blog"
Add-Feed "Coding Horror" "https://blog.codinghorror.com/rss/" "blog"
Add-Feed "Joel on Software" "https://www.joelonsoftware.com/feed/" "blog"
Add-Feed "Matt Might" "https://matt.might.net/articles/feed.rss" "blog"
Add-Feed "Simon Willison" "https://simonwillison.net/atom/everything/" "blog"
Add-Feed "码农真经的博客" "https://blog.mzh.ren/zh/index.xml" "blog"
Add-Feed "老苏的blog" "https://laosu.tech/atom.xml" "blog"
Add-Feed "肖恩聊技术" "https://shawnxie.top/rss.xml" "blog"
Add-Feed "軟體考古學家" "https://blog.brachiosoft.com/index.xml" "blog"
Write-Host ""

# ============================================
# 技术博客
# ============================================
Write-Host "--- 技术博客 ---" -ForegroundColor Yellow
Add-Feed "AWS Architecture Blog" "https://aws.amazon.com/blogs/architecture/feed/" "tech-blog"
Add-Feed "AWS Machine Learning Blog" "https://aws.amazon.com/blogs/machine-learning/feed/" "tech-blog"
Add-Feed "LlamaIndex Blog" "https://www.llamaindex.ai/blog/feed" "tech-blog"
Add-Feed "ByteByteGo Newsletter" "https://blog.bytebytego.com/feed" "tech-blog"
Add-Feed "Docker" "https://www.docker.com/feed/" "tech-blog"
Add-Feed "Engineering at Meta" "https://engineering.fb.com/feed/" "tech-blog"
Add-Feed "Engineering at Slack" "https://slack.engineering/feed/" "tech-blog"
Add-Feed "Grafana Labs Engineering" "https://grafana.com/categories/engineering/index.xml" "tech-blog"
Add-Feed "freeCodeCamp" "https://www.freecodecamp.org/news/rss/" "tech-blog"
Add-Feed "Hugging Face Blog" "https://huggingface.co/blog/feed.xml" "tech-blog"
Add-Feed "IntelliJ IDEA Blog" "https://blog.jetbrains.com/idea/feed" "tech-blog"
Add-Feed "Java Code Geeks" "http://feeds.feedburner.com/JavaCodeGeeks" "tech-blog"
Add-Feed "LangChain Blog" "https://blog.langchain.com/rss/" "tech-blog"
Add-Feed "Martin Fowler" "https://martinfowler.com/feed.atom" "tech-blog"
Add-Feed "Microservice architecture" "https://microservices.io/feed.xml" "tech-blog"
Add-Feed "Microsoft Azure Blog" "https://azure.microsoft.com/en-us/blog/feed/" "tech-blog"
Add-Feed "Microsoft Research Blog" "https://www.microsoft.com/en-us/research/blog/feed/" "tech-blog"
Add-Feed "Netflix TechBlog" "https://netflixtechblog.com/feed" "tech-blog"
Add-Feed "Redis" "https://redis.io/feed/" "tech-blog"
Add-Feed "Sagyam Blog" "https://blog.sagyamthapa.com.np/rss.xml" "tech-blog"
Add-Feed "Spring" "https://spring.io/blog.atom" "tech-blog"
Add-Feed "Stack Overflow Blog" "http://blog.stackoverflow.com/feed/" "tech-blog"
Add-Feed "Apache Software Foundation" "https://news.apache.org/feed" "tech-blog"
Add-Feed "The Cloudflare Blog" "https://blog.cloudflare.com/rss" "tech-blog"
Add-Feed "The New Stack" "https://thenewstack.io/feed/" "tech-blog"
Add-Feed "Uber Engineering Blog" "https://www.uber.com/blog/engineering/rss/" "tech-blog"
Add-Feed "美团技术团队" "https://tech.meituan.com/feed/" "tech-blog"
Write-Host ""

# ============================================
# 技术周刊
# ============================================
Write-Host "--- 技术周刊 ---" -ForegroundColor Yellow
Add-Feed "1Link.Fun 科技周刊" "https://techhub.social/users/1link.rss" "weekly"
Add-Feed "54321 Weekly" "https://54321.versun.me/feed/" "weekly"
Add-Feed "AIGC Weekly" "https://quaily.com/op7418/feed/atom" "weekly"
Add-Feed "Better Dev Link" "https://betterdev.link/rss.xml" "weekly"
Add-Feed "HelloGitHub 月刊" "https://hellogithub.com/rss" "weekly"
Add-Feed "不死鸟" "https://iui.su/feed/" "weekly"
Add-Feed "二丫讲梵-学习周刊" "https://wiki.eryajf.net/rss.xml" "weekly"
Add-Feed "大橘和朋友们的周刊" "https://rrorangeandfriends.de/feed.xml" "weekly"
Add-Feed "月球背面" "https://moonvy.com/blog/rss.xml" "weekly"
Add-Feed "潮流周刊" "https://weekly.tw93.fun/rss.xml" "weekly"
Add-Feed "粥里有勺糖" "https://sugarat.top/feed.rss" "weekly"
Add-Feed "老胡的周刊" "https://weekly.howie6879.com/rss/rss.xml" "weekly"
Add-Feed "肖恩技术周刊" "https://weekly.shawnxie.top/rss.xml" "weekly"
Add-Feed "让小产品的独立变现更简单" "https://www.ezindie.com/feed/rss.xml" "weekly"
Add-Feed "豌豆花下猫" "https://pythoncat.top/rss.xml" "weekly"
Add-Feed "阿猫的博客" "https://ameow.xyz/feed.xml" "weekly"
Add-Feed "龙爪槐守望者" "https://www.ftium4.com/rss.xml" "weekly"
Write-Host ""

# ============================================
# 热点资讯
# ============================================
Write-Host "--- 热点资讯 ---" -ForegroundColor Yellow
Add-Feed "36氪" "https://36kr.com/feed" "news"
Add-Feed "InfoQ" "https://feed.infoq.com/" "news"
Add-Feed "InfoQ 推荐" "https://plink.anyfeeder.com/infoq/recommend" "news"
Add-Feed "MIT 科技评论" "https://rsshub.bestblogs.dev/mittrchina/hot" "news"
Add-Feed "TechCrunch" "https://techcrunch.com/feed/" "news"
Add-Feed "SuperTechFans" "https://www.supertechfans.com/cn/index.xml" "news"
Add-Feed "奇客Solidot" "https://www.solidot.org/index.rss" "news"
Add-Feed "少数派" "https://sspai.com/feed" "news"
Add-Feed "晚点" "https://rsshub.bestblogs.dev/latepost" "news"
Add-Feed "极客公园" "https://www.geekpark.net/rss" "news"
Add-Feed "爱范儿" "https://www.ifanr.com/feed" "news"
Add-Feed "蓝点网" "https://www.landiannews.com/feed" "news"
Write-Host ""

# ============================================
# 社区
# ============================================
Write-Host "--- 社区 ---" -ForegroundColor Yellow
Add-Feed "LINUX DO 最新话题" "https://linux.do/latest.rss" "community"
Add-Feed "LINUX DO 热门话题" "https://linux.do/top.rss" "community"
Add-Feed "V2EX" "https://v2ex.com/index.xml" "community"
Write-Host ""

# ============================================
# 资源
# ============================================
Write-Host "--- 资源 ---" -ForegroundColor Yellow
Add-Feed "GitHub Weekly Trending" "https://mshibanami.github.io/GitHubTrendingRSS/weekly/all.xml" "resources"
Add-Feed "Mac玩儿法" "https://www.waerfa.com/feed" "resources"
Add-Feed "小众软件" "https://www.appinn.com/feed/" "resources"
Add-Feed "异次元软件世界" "https://feed.iplaysoft.com/" "resources"
Write-Host ""

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Blue
Write-Host "  初始化完成！" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Blue
Write-Host ""
Write-Host "  统计信息：" -ForegroundColor Cyan
Write-Host "   + 成功添加: $($script:totalAdded) 个源" -ForegroundColor Green
Write-Host "   ! 跳过/失败: $($script:totalSkipped) 个源" -ForegroundColor Yellow
Write-Host ""
Write-Host "  分类统计：" -ForegroundColor Cyan
Write-Host "   - 中文安全资讯: 9 个"
Write-Host "   - 国际安全资讯: 8 个"
Write-Host "   - 企业安全团队: 7 个"
Write-Host "   - 漏洞情报: 7 个"
Write-Host "   - 威胁研究: 6 个"
Write-Host "   - 渗透测试: 6 个"
Write-Host "   - 技术资讯: 5 个"
Write-Host "   - 区块链资讯: 6 个"
Write-Host "   - RSShub 聚合: 3 个"
Write-Host "   - 中文技术博客: 5 个"
Write-Host "   - 安全工具: 2 个"
Write-Host "   - AI 相关: 11 个"
Write-Host "   - 个人博客: 14 个"
Write-Host "   - 技术博客: 27 个"
Write-Host "   - 技术周刊: 17 个"
Write-Host "   - 热点资讯: 12 个"
Write-Host "   - 社区: 3 个"
Write-Host "   - 资源: 4 个"
Write-Host "   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "   总计: 152 个 RSS 源" -ForegroundColor Green
Write-Host ""
Write-Host "  常用命令：" -ForegroundColor Cyan
Write-Host "   查看所有源:  $RSS_READER list"
Write-Host "   拉取文章:    $RSS_READER fetch"
Write-Host "   启动 TUI:    $RSS_READER"
Write-Host "   删除源:      $RSS_READER remove <id>"
Write-Host ""
Write-Host "  提示：" -ForegroundColor Yellow
Write-Host "   - 建议每天拉取 2-3 次"
Write-Host "   - 首次拉取约需 3-5 分钟，请耐心等待"
Write-Host "   - 部分源可能因网络原因暂时无法访问"
Write-Host "   - 可根据需要删除不感兴趣的源"
Write-Host ""
