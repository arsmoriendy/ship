use crate::tui::{
    Focus,
    actions::Action,
    component::Component,
    prelude::*,
    state::AppState,
    widgets::focusable::{focus_block, focus_list},
};

pub struct ImageList {}

impl StatefulWidget for &mut ImageList {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
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

impl Component<&mut AppState> for ImageList {
    async fn handle_key_events(&mut self, ke: &KeyEvent, _state: &mut AppState) -> Action {
        match ke.code {
            KeyCode::Char('H') => Action::Focus(Focus::Projects),
            KeyCode::Char('D') => Action::DeleteImage,
            KeyCode::Char('P') => Action::PushImage,
            _ => Action::Noop,
        }
    }
}
