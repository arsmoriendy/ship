use crate::prelude::*;
use crate::tui::Focus;
use crate::tui::prelude::*;
use crate::tui::state::AppState;
use crate::tui::widgets::focusable::{focus_block, focus_list};

pub struct ImageList {}
impl StatefulWidget for ImageList {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let project = &state.projects[state.selected_project];
        let mut lines: Vec<Line> = vec![];
        for img in &project.images {
            let mut spans: Vec<Span> = vec![];
            let pushed = if let Some(digest) = img.digest
                && let Some(project_digests) =
                    state.store.project_registry_digests.get(&project.name)
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
        let is_focused = state.focus == Focus::Images;
        StatefulWidget::render(
            focus_list(is_focused)
                .items(lines)
                .block(focus_block(is_focused).title("Images")),
            area,
            buf,
            &mut ListState::default().with_selected(Some(state.selected_image)),
        );
    }
}
