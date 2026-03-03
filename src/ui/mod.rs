pub mod state;
pub mod events;
pub mod render;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

use crate::db::{feeds, articles};
use crate::core::feed_manager::FeedManager;
use state::{AppState, FilterMode};
use events::{handle_key_event, poll_event, AppEvent};

pub async fn run_tui(pool: sqlx::SqlitePool) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = AppState::new(pool.clone());
    let manager = FeedManager::new(pool.clone());

    // Load initial data
    app.status_message = "Loading feeds...".to_string();
    terminal.draw(|f| render::draw(f, &app))?;

    app.feeds = feeds::get_all_feeds(&pool).await?;
    app.articles = articles::get_all_articles(&pool, 1000, 0).await?;
    app.filtered_articles = app.articles.clone();
    app.status_message = "Ready".to_string();

    // Main loop
    let result = run_app(&mut terminal, &mut app, &manager).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut AppState,
    manager: &FeedManager,
) -> Result<()> {
    loop {
        terminal.draw(|f| render::draw(f, app))?;

        if let Some(event) = poll_event(Duration::from_millis(100))? {
            if let Event::Key(key) = event {
                let app_event = handle_key_event(key);

                match app_event {
                    AppEvent::Quit => {
                        return Ok(());
                    }
                    AppEvent::MoveUp => {
                        app.move_selection_up();
                    }
                    AppEvent::MoveDown => {
                        app.move_selection_down();
                    }
                    AppEvent::SwitchPanelLeft => {
                        app.switch_panel_left();
                    }
                    AppEvent::SwitchPanelRight => {
                        app.switch_panel_right();
                    }
                    AppEvent::ToggleRead => {
                        if let Some(article) = app.selected_article() {
                            let article_id = article.id;
                            let new_status = !article.is_read;
                            if let Err(e) = articles::mark_as_read(&app.pool, article_id, new_status).await {
                                app.status_message = format!("Error: {}", e);
                            } else {
                                // Reload articles
                                app.articles = articles::get_all_articles(&app.pool, 1000, 0).await?;
                                apply_filter(app).await?;
                                app.status_message = if new_status { "Marked as read" } else { "Marked as unread" }.to_string();
                            }
                        }
                    }
                    AppEvent::ToggleBookmark => {
                        if let Some(article) = app.selected_article() {
                            let article_id = article.id;
                            if let Err(e) = articles::toggle_bookmark(&app.pool, article_id).await {
                                app.status_message = format!("Error: {}", e);
                            } else {
                                // Reload articles
                                app.articles = articles::get_all_articles(&app.pool, 1000, 0).await?;
                                apply_filter(app).await?;
                                app.status_message = "Bookmark toggled".to_string();
                            }
                        }
                    }
                    AppEvent::OpenInBrowser => {
                        if let Some(article) = app.selected_article() {
                            let url = article.link.clone();
                            let article_id = article.id;
                            if let Err(e) = open::that(&url) {
                                app.status_message = format!("Failed to open browser: {}", e);
                            } else {
                                app.status_message = format!("Opened: {}", url);
                                // Mark as read
                                let _ = articles::mark_as_read(&app.pool, article_id, true).await;
                                app.articles = articles::get_all_articles(&app.pool, 1000, 0).await?;
                                apply_filter(app).await?;
                            }
                        }
                    }
                    AppEvent::Refresh => {
                        app.status_message = "Refreshing feeds...".to_string();
                        terminal.draw(|f| render::draw(f, app))?;

                        let results = manager.fetch_all_feeds().await;
                        let success_count = results.iter().filter(|r| r.is_ok()).count();
                        let total_articles: usize = results.iter().filter_map(|r| r.as_ref().ok()).sum();

                        app.feeds = feeds::get_all_feeds(&app.pool).await?;
                        app.articles = articles::get_all_articles(&app.pool, 1000, 0).await?;
                        apply_filter(app).await?;

                        app.status_message = format!(
                            "Refreshed {} feeds, {} new articles",
                            success_count, total_articles
                        );
                    }
                    AppEvent::ToggleHelp => {
                        app.show_help = !app.show_help;
                    }
                    AppEvent::FilterAll => {
                        app.filter_mode = FilterMode::All;
                        apply_filter(app).await?;
                        app.status_message = "Showing all articles".to_string();
                    }
                    AppEvent::FilterUnread => {
                        app.filter_mode = FilterMode::Unread;
                        apply_filter(app).await?;
                        app.status_message = "Showing unread articles".to_string();
                    }
                    AppEvent::FilterBookmarked => {
                        app.filter_mode = FilterMode::Bookmarked;
                        apply_filter(app).await?;
                        app.status_message = "Showing bookmarked articles".to_string();
                    }
                    AppEvent::Search => {
                        // TODO: Implement search input
                        app.status_message = "Search not yet implemented".to_string();
                    }
                    AppEvent::None => {}
                }
            }
        }
    }
}

async fn apply_filter(app: &mut AppState) -> Result<()> {
    app.filtered_articles = match &app.filter_mode {
        FilterMode::All => app.articles.clone(),
        FilterMode::Unread => app.articles.iter().filter(|a| !a.is_read).cloned().collect(),
        FilterMode::Bookmarked => app.articles.iter().filter(|a| a.is_bookmarked).cloned().collect(),
        FilterMode::ByFeed(feed_id) => app.articles.iter().filter(|a| a.feed_id == *feed_id).cloned().collect(),
    };

    // Reset selection if out of bounds
    if app.selected_article_index >= app.filtered_articles.len() {
        app.selected_article_index = app.filtered_articles.len().saturating_sub(1);
    }

    Ok(())
}
