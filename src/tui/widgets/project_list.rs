use crate::tui::{
    Focus,
    actions::{Action, PROJECT_ACTIONS},
    component::Component,
    prelude::*,
    state::AppState,
    widgets::{
        focusable::{focus_block, focus_table},
        legend::Legend,
    },
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
                    spans.push(registry.to_string().fg(Color::Yellow))
                }
                Row::new(spans)
            })
            .collect();

        let table = focus_table(is_focused)
            .block(
                focus_block(is_focused)
                    .title("Projects")
                    .title_top(Legend::new(&PROJECT_ACTIONS).as_line(state).right_aligned()),
            )
            .header(Row::new(["Name", "Images", "Registry"]).bold())
            .widths(constraints![*=1, ==10, *=1])
            .rows(rows);

        StatefulWidget::render(table, area, buf, &mut state.project_table_state);
    }
}

impl Component<&mut AppState> for ProjectList {
    async fn handle_key_events(&mut self, ke: &KeyEvent, state: &mut AppState) -> Action {
        state.config.match_actions(ke, &PROJECT_ACTIONS)
    }
}
