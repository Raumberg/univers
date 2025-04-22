pub mod effects;
pub mod prelude;
pub mod terminal;
pub mod star_system;

use ratatui::layout::Rect;
use crossterm::event::KeyCode;

pub enum AppEvent {
    Tick,
    KeyPressed(KeyCode),
    Resize(Rect),
    RefreshAufBuffer,
}