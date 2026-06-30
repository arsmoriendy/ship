pub use crate::prelude::*;
pub use ratatui::{
    DefaultTerminal,
    crossterm::{
        ExecutableCommand,
        event::{self, Event, KeyCode, KeyEvent},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    macros::{horizontal, line as l, span, text, vertical},
    prelude::*,
    style::{Color, Style},
    widgets::{
        Block, List, ListState, StatefulWidget, Widget,
    },
};
pub use std::io::stdout;
pub const SPINNER_SEQUENCE: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
