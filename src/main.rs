use rss_reader::db::create_pool;
use rss_reader::core::feed_manager::FeedManager;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    // 连接数据库
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:rss_reader.db".to_string());
    let pool = create_pool(&db_url).await?;
    let manager = FeedManager::new(pool.clone());

    match args[1].as_str() {
        "add" => {
            if args.len() < 5 {
                println!("Usage: rss-reader add <title> <url> <category>");
                return Ok(());
            }
            let title = &args[2];
            let url = &args[3];
            let category = &args[4];

            match manager.add_feed(title, url, category).await {
                Ok(id) => println!("✓ Feed added successfully (ID: {})", id),
                Err(e) => println!("✗ Failed to add feed: {}", e),
            }
        }
        "list" => {
            match manager.get_all_feeds().await {
                Ok(feeds) => {
                    if feeds.is_empty() {
                        println!("No feeds found. Add one with: rss-reader add <title> <url> <category>");
                    } else {
                        println!("\n📰 Feeds ({}):", feeds.len());
                        println!("{:-<80}", "");
                        for feed in feeds {
                            println!("  [{}] {} ({})", feed.id, feed.title, feed.category);
                            println!("      URL: {}", feed.url);
                            if let Some(last_fetched) = feed.last_fetched {
                                println!("      Last fetched: {}", last_fetched);
                            }
                            if let Some(error) = feed.fetch_error {
                                println!("      ⚠ Error: {}", error);
                            }
                            println!();
                        }
                    }
                }
                Err(e) => println!("✗ Failed to list feeds: {}", e),
            }
        }
        "fetch" => {
            println!("🔄 Fetching all feeds...\n");
            let results = manager.fetch_all_feeds().await;

            let mut success_count = 0;
            let mut total_articles = 0;

            for result in results {
                match result {
                    Ok(count) => {
                        success_count += 1;
                        total_articles += count;
                    }
                    Err(e) => println!("✗ Error: {}", e),
                }
            }

            println!("\n✓ Fetch complete:");
            println!("  - {} feeds fetched successfully", success_count);
            println!("  - {} new articles added", total_articles);
        }
        "articles" => {
            use rss_reader::db::articles;

            let limit = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

            match articles::get_all_articles(&pool, limit, 0).await {
                Ok(articles) => {
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
                Err(e) => println!("✗ Failed to list articles: {}", e),
            }
        }
        "search" => {
            if args.len() < 3 {
                println!("Usage: rss-reader search <query>");
                return Ok(());
            }

            use rss_reader::db::articles;
            let query = &args[2];

            match articles::search_articles(&pool, query, 20, 0).await {
                Ok(articles) => {
                    if articles.is_empty() {
                        println!("No articles found matching '{}'", query);
                    } else {
                        println!("\n🔍 Search Results for '{}' ({}):", query, articles.len());
                        println!("{:-<80}", "");
                        for article in articles {
                            println!("  • {}", article.title);
                            println!("    {}", article.link);
                            println!();
                        }
                    }
                }
                Err(e) => println!("✗ Search failed: {}", e),
            }
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!(r#"
RSS Reader - A simple RSS feed reader

USAGE:
    rss-reader <COMMAND> [OPTIONS]

COMMANDS:
    add <title> <url> <category>    Add a new RSS feed
    list                             List all feeds
    fetch                            Fetch all feeds and update articles
    articles [limit]                 Show latest articles (default: 10)
    search <query>                   Search articles by keyword
    help                             Show this help message

EXAMPLES:
    rss-reader add "Hacker News" "https://news.ycombinator.com/rss" "tech"
    rss-reader fetch
    rss-reader articles 20
    rss-reader search "rust"

ENVIRONMENT:
    DATABASE_URL    Database connection string (default: sqlite:rss_reader.db)
"#);
}

