use crate::db::schema::{Feed, Article};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Feeds,
    Articles,
    Preview,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterMode {
    All,
    Unread,
    Bookmarked,
    ByFeed(i64),
}

pub struct AppState {
    pub pool: SqlitePool,
    pub feeds: Vec<Feed>,
    pub articles: Vec<Article>,
    pub filtered_articles: Vec<Article>,
    pub selected_feed_index: usize,
    pub selected_article_index: usize,
    pub active_panel: Panel,
    pub filter_mode: FilterMode,
    pub search_query: String,
    pub show_help: bool,
    pub status_message: String,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            feeds: Vec::new(),
            articles: Vec::new(),
            filtered_articles: Vec::new(),
            selected_feed_index: 0,
            selected_article_index: 0,
            active_panel: Panel::Feeds,
            filter_mode: FilterMode::All,
            search_query: String::new(),
            show_help: false,
            status_message: String::from("Press ? for help"),
        }
    }

    pub fn selected_feed(&self) -> Option<&Feed> {
        self.feeds.get(self.selected_feed_index)
    }

    pub fn selected_article(&self) -> Option<&Article> {
        self.filtered_articles.get(self.selected_article_index)
    }

    pub fn move_selection_up(&mut self) {
        match self.active_panel {
            Panel::Feeds => {
                if self.selected_feed_index > 0 {
                    self.selected_feed_index -= 1;
                }
            }
            Panel::Articles | Panel::Preview => {
                if self.selected_article_index > 0 {
                    self.selected_article_index -= 1;
                }
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        match self.active_panel {
            Panel::Feeds => {
                if self.selected_feed_index < self.feeds.len().saturating_sub(1) {
                    self.selected_feed_index += 1;
                }
            }
            Panel::Articles | Panel::Preview => {
                if self.selected_article_index < self.filtered_articles.len().saturating_sub(1) {
                    self.selected_article_index += 1;
                }
            }
        }
    }

    pub fn switch_panel_left(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Articles => Panel::Feeds,
            Panel::Preview => Panel::Articles,
            Panel::Feeds => Panel::Feeds,
        };
    }

    pub fn switch_panel_right(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Feeds => Panel::Articles,
            Panel::Articles => Panel::Preview,
            Panel::Preview => Panel::Preview,
        };
    }

    pub fn unread_count(&self) -> usize {
        self.articles.iter().filter(|a| !a.is_read).count()
    }

    pub fn bookmarked_count(&self) -> usize {
        self.articles.iter().filter(|a| a.is_bookmarked).count()
    }
}
