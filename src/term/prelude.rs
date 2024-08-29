#[allow(unused_imports)]

pub use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Offset, Rect},
    prelude::*,
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Circle, Line, Paragraph, Widget, Clear},
    buffer::Buffer,
    text::{Line, Span},
    Frame,
};

pub use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub use tachyonfx::widget::{EffectTimeline, EffectTimelineRects};
pub use tachyonfx::{BufferRenderer, Effect, Shader};

pub use std::rc::Rc;
pub use std::sync::mpsc;
pub use std::time::Duration;
pub use std::{io, panic, thread};

pub use terminal::TerminalInterface;