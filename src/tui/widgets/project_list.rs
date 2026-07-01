use crate::tui::{
    Focus,
    actions::Action,
    component::Component,
    prelude::*,
    state::AppState,
    widgets::focusable::{focus_block, focus_table},
};

pub struct ProjectList {}

impl StatefulWidget for &mut ProjectList {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let is_focused = state.focus == Focus::Projects;
        let rows: Vec<Row> = state
            .projects
            .iter()
            .map(|proj| {
                let mut spans = vec![
                    span![proj.name],
                    format!("{} images ", proj.images.len()).fg(Color::LightGreen),
                ];
                if let Some(registry) = state.config.project_registries.get(&proj.name) {
                    spans.push(format!("{}", registry).fg(Color::Yellow))
                }
                Row::new(spans)
            })
            .collect();

        let table = focus_table(is_focused)
            .block(focus_block(is_focused).title("Projects"))
            .header(Row::new(["Name", "Images", "Registry"]).bold())
            .widths(constraints![*=1, ==10, *=1])
            .rows(rows);

        StatefulWidget::render(
            table,
            area,
            buf,
            &mut TableState::default().with_selected(state.selected_project),
        );
    }
}

impl Component<&mut AppState> for ProjectList {
    async fn handle_key_events(&mut self, ke: &KeyEvent, _state: &mut AppState) -> Action {
        match ke.code {
            KeyCode::Char('J') => Action::Focus(Focus::Images),
            KeyCode::Char('f') => Action::FetchDigests,
            _ => Action::Noop,
        }
    }
}
