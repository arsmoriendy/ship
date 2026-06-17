pub use crate::tui::prelude::*;
use crate::tui::{
    App, Focus,
    widgets::focusable::{focus_block, focus_list},
};

impl App {
    pub fn project_list<'a>(&'a self) -> List<'a> {
        let mut lines: Vec<Line> = vec![];
        for proj in &self.projects {
            let mut spans = vec![
                Span::from(&proj.name),
                format!(" ({} images) ", proj.images.len()).fg(Color::LightGreen),
            ];
            if let Some(registry) = self.config.project_registries.get(&proj.name) {
                spans.push(format!("[{}]", registry).fg(Color::Magenta))
            }
            lines.push(Line::from(spans))
        }
        let is_focused = self.focus == Focus::Projects;
        focus_list(is_focused)
            .items(lines)
            .block(focus_block(is_focused).title("Projects"))
    }
}
