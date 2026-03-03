use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub enum AppEvent {
    Quit,
    MoveUp,
    MoveDown,
    SwitchPanelLeft,
    SwitchPanelRight,
    ToggleRead,
    ToggleBookmark,
    OpenInBrowser,
    Refresh,
    Search,
    ToggleHelp,
    FilterAll,
    FilterUnread,
    FilterBookmarked,
    None,
}

pub fn handle_key_event(key: KeyEvent) -> AppEvent {
    match (key.code, key.modifiers) {
        // Quit
        (KeyCode::Char('q'), KeyModifiers::NONE) => AppEvent::Quit,
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => AppEvent::Quit,
        (KeyCode::Esc, _) => AppEvent::Quit,

        // Navigation
        (KeyCode::Char('j'), KeyModifiers::NONE) | (KeyCode::Down, _) => AppEvent::MoveDown,
        (KeyCode::Char('k'), KeyModifiers::NONE) | (KeyCode::Up, _) => AppEvent::MoveUp,
        (KeyCode::Char('h'), KeyModifiers::NONE) | (KeyCode::Left, _) => AppEvent::SwitchPanelLeft,
        (KeyCode::Char('l'), KeyModifiers::NONE) | (KeyCode::Right, _) => {
            AppEvent::SwitchPanelRight
        }

        // Actions
        (KeyCode::Enter, _) => AppEvent::OpenInBrowser,
        (KeyCode::Char(' '), KeyModifiers::NONE) => AppEvent::ToggleRead,
        (KeyCode::Char('b'), KeyModifiers::NONE) => AppEvent::ToggleBookmark,
        (KeyCode::Char('r'), KeyModifiers::NONE) => AppEvent::Refresh,
        (KeyCode::Char('/'), KeyModifiers::NONE) => AppEvent::Search,
        (KeyCode::Char('?'), KeyModifiers::NONE) => AppEvent::ToggleHelp,

        // Filters
        (KeyCode::Char('a'), KeyModifiers::NONE) => AppEvent::FilterAll,
        (KeyCode::Char('u'), KeyModifiers::NONE) => AppEvent::FilterUnread,
        (KeyCode::Char('m'), KeyModifiers::NONE) => AppEvent::FilterBookmarked,

        _ => AppEvent::None,
    }
}

pub fn poll_event(timeout: Duration) -> Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
