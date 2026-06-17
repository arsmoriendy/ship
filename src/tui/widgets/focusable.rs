use crate::prelude::*;
use crate::tui::prelude::*;

pub fn focus_bg(is_focused: bool) -> Style {
    let focused_style = Style::default().bg(Color::Blue);
    if is_focused {
        focused_style
    } else {
        Style::default()
    }
}

pub fn focus_fg(is_focused: bool) -> Style {
    let focused_style = Style::default().fg(Color::Green);
    if is_focused {
        focused_style
    } else {
        Style::default()
    }
}

pub fn focus_block<'a>(is_focused: bool) -> Block<'a> {
    Block::bordered().border_style(focus_fg(is_focused))
}

pub fn focus_list<'a>(is_focused: bool) -> List<'a> {
    List::default().highlight_style(focus_bg(is_focused))
}
