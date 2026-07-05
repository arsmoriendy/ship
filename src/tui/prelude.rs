pub use crate::prelude::*;
pub use crate::tui::state::PopupVariant::Error as ErrorPopup;
pub use ratatui::{
    DefaultTerminal,
    crossterm::{
        ExecutableCommand,
        event::{self, Event, KeyEvent},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    macros::{constraints, horizontal, line as l, span, vertical},
    prelude::*,
    style::{Color, Style},
    widgets::{
        Block, Borders, Clear, Paragraph, Row, StatefulWidget, Table, TableState, Widget, Wrap,
    },
};
pub use std::io::stdout;
pub const SPINNER_SEQUENCE: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
