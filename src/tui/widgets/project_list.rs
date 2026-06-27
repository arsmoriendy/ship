use crate::tui::{
    Focus,
    prelude::*,
    state::AppState,
    widgets::focusable::{focus_block, focus_list},
};

pub struct ProjectList {}
impl StatefulWidget for ProjectList {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut lines: Vec<Line> = vec![];
        for proj in &state.projects {
            let mut spans = vec![
                Span::from(proj.name.clone()),
                format!(" ({} images) ", proj.images.len()).fg(Color::LightGreen),
            ];
            if let Some(registry) = state.config.project_registries.get(&proj.name) {
                spans.push(format!("[{}]", registry).fg(Color::Yellow))
            }
            lines.push(Line::from(spans))
        }
        let is_focused = state.focus == Focus::Projects;
        StatefulWidget::render(
            focus_list(is_focused)
                .items(lines)
                .block(focus_block(is_focused).title("Projects")),
            area,
            buf,
            &mut ListState::default().with_selected(Some(state.selected_project)),
        );
    }
}
