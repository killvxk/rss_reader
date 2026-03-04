use crate::ui::state::{AppState, Panel};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use regex::Regex;

/// 清理 HTML 标签，转换为纯文本
fn clean_html(html: &str) -> String {
    // 移除 HTML 标签
    let re = Regex::new(r"<[^>]*>").unwrap();
    let cleaned = re.replace_all(html, "");

    // 清理多余的空白字符
    let re_whitespace = Regex::new(r"\s+").unwrap();
    re_whitespace.replace_all(&cleaned, " ").trim().to_string()
}

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.size());

    // Header
    draw_header(f, app, chunks[0]);

    // Main content (3 panels)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Feeds
            Constraint::Percentage(40), // Articles
            Constraint::Percentage(40), // Preview
        ])
        .split(chunks[1]);

    draw_feeds_panel(f, app, main_chunks[0]);
    draw_articles_panel(f, app, main_chunks[1]);
    draw_preview_panel(f, app, main_chunks[2]);

    // Footer
    draw_footer(f, app, chunks[2]);

    // Help overlay
    if app.show_help {
        draw_help_overlay(f, f.size());
    }
}

fn draw_header(f: &mut Frame, app: &AppState, area: Rect) {
    let title = format!(
        " RSS Reader - {} feeds | {} unread | {} bookmarked ",
        app.feeds.len(),
        app.unread_count(),
        app.bookmarked_count()
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan))
        .title(title);

    f.render_widget(block, area);
}

fn draw_feeds_panel(f: &mut Frame, app: &AppState, area: Rect) {
    let is_active = app.active_panel == Panel::Feeds;
    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = app
        .feeds
        .iter()
        .enumerate()
        .map(|(i, feed)| {
            let style = if i == app.selected_feed_index && is_active {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = format!("  {} ({})", feed.title, feed.category);
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Feeds ({}) ", app.feeds.len()))
            .border_style(border_style),
    );

    f.render_widget(list, area);
}

fn draw_articles_panel(f: &mut Frame, app: &AppState, area: Rect) {
    let is_active = app.active_panel == Panel::Articles;
    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = app
        .filtered_articles
        .iter()
        .enumerate()
        .map(|(i, article)| {
            let style = if i == app.selected_article_index && is_active {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if article.is_read {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };

            let read_marker = if article.is_read { " " } else { "●" };
            let bookmark_marker = if article.is_bookmarked { "⭐" } else { " " };
            let title = clean_html(&article.title);
            let content = format!("{} {} {}", read_marker, bookmark_marker, title);

            ListItem::new(content).style(style)
        })
        .collect();

    let title = match &app.filter_mode {
        crate::ui::state::FilterMode::All => {
            format!(" Articles ({}) ", app.filtered_articles.len())
        }
        crate::ui::state::FilterMode::Unread => {
            format!(" Unread ({}) ", app.filtered_articles.len())
        }
        crate::ui::state::FilterMode::Bookmarked => {
            format!(" Bookmarked ({}) ", app.filtered_articles.len())
        }
        crate::ui::state::FilterMode::ByFeed(_) => {
            format!(" Articles ({}) ", app.filtered_articles.len())
        }
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style),
    );

    f.render_widget(list, area);
}

fn draw_preview_panel(f: &mut Frame, app: &AppState, area: Rect) {
    let is_active = app.active_panel == Panel::Preview;
    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let content = if let Some(article) = app.selected_article() {
        let mut text = Vec::new();
        text.push(Line::from(vec![
            Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(clean_html(&article.title)),
        ]));
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled("Link: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(&article.link, Style::default().fg(Color::Blue)),
        ]));
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled("Published: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&article.published),
        ]));
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(if article.is_read { "Read" } else { "Unread" }),
            Span::raw(" | "),
            Span::raw(if article.is_bookmarked {
                "Bookmarked"
            } else {
                "Not bookmarked"
            }),
        ]));
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            "Content:",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        text.push(Line::from(""));

        if let Some(content) = &article.content {
            text.push(Line::from(clean_html(content)));
        } else {
            text.push(Line::from(Span::styled(
                "No content available",
                Style::default().fg(Color::DarkGray),
            )));
        }

        Text::from(text)
    } else {
        Text::from("No article selected")
    };

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Preview ")
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, app: &AppState, area: Rect) {
    let shortcuts = vec![
        "[j/k]Move",
        "[h/l]Panel",
        "[Enter]Open",
        "[Space]Read",
        "[b]Bookmark",
        "[r]Refresh",
        "[a]All",
        "[u]Unread",
        "[m]Marked",
        "[?]Help",
        "[q]Quit",
    ];

    let text = Line::from(
        shortcuts
            .iter()
            .map(|s| Span::raw(format!(" {} ", s)))
            .collect::<Vec<_>>(),
    );

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", app.status_message)),
    );

    f.render_widget(paragraph, area);
}

fn draw_help_overlay(f: &mut Frame, area: Rect) {
    let help_text = vec![
        "RSS Reader - Keyboard Shortcuts",
        "",
        "Navigation:",
        "  j / ↓       - Move down",
        "  k / ↑       - Move up",
        "  h / ←       - Switch to left panel",
        "  l / →       - Switch to right panel",
        "",
        "Actions:",
        "  Enter       - Open article in browser",
        "  Space       - Toggle read/unread",
        "  b           - Toggle bookmark",
        "  r           - Refresh all feeds",
        "",
        "Filters:",
        "  a           - Show all articles",
        "  u           - Show unread only",
        "  m           - Show bookmarked only",
        "",
        "Other:",
        "  ?           - Toggle this help",
        "  q / Esc     - Quit",
        "",
        "Press any key to close",
    ];

    let text: Vec<Line> = help_text.iter().map(|s| Line::from(*s)).collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Help ")
        .style(Style::default().bg(Color::Black).fg(Color::White));

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().bg(Color::Black).fg(Color::White));

    // Center the help overlay
    let popup_area = centered_rect(60, 80, area);

    // Clear the area first to prevent background content from showing through
    f.render_widget(Clear, popup_area);
    f.render_widget(paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
