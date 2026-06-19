use crate::prelude::*;
use crate::tui::App;
use crate::tui::Focus;
pub use crate::tui::prelude::*;
use crate::tui::widgets::focusable::{focus_block, focus_list};

impl App {
    pub fn image_list<'a>(&self) -> List<'a> {
        let project = &self.projects[self.selected_project];
        let mut lines: Vec<Line> = vec![];
        for img in &project.images {
            let mut spans: Vec<Span> = vec![];
            let pushed = if let Some(digest) = img.digest
                && let Some(project_digests) = self.project_digests.get(&project.name)
                && project_digests.contains(&digest)
            {
                true
            } else {
                false
            };
            spans.push(if pushed { "✔ " } else { "  " }.light_green());
            spans.push(encode_hex(img.id)[0..8].to_owned().light_green());
            spans.push(Span::from(" "));
            spans.push(Span::from(img.tags.join(", ")));
            lines.push(Line::from(spans))
        }
        let is_focused = self.focus == Focus::Images;
        focus_list(is_focused)
            .items(lines)
            .block(focus_block(is_focused).title("Images"))
    }
}
