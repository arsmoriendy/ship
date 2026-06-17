use crate::prelude::*;
use crate::tui::App;
use crate::tui::Focus;
pub use crate::tui::prelude::*;
use crate::tui::widgets::focusable::{focus_block, focus_list};

impl App {
    pub fn image_list<'a>(&self) -> List<'a> {
        let mut lines: Vec<Line> = vec![];
        for img in &self.projects[self.selected_project].images {
            lines.push(Line::from(vec![
                encode_hex(img.id)[0..8].to_owned().light_green(),
                Span::from(" "),
                Span::from(img.tags.join(", ")),
            ]))
        }
        let is_focused = self.focus == Focus::Images;
        focus_list(is_focused)
            .items(lines)
            .block(focus_block(is_focused).title("Images"))
    }
}
