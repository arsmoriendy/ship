use crate::tui::{
    Focus,
    actions::{Action, IMAGE_ACTIONS},
    component::Component,
    prelude::*,
    state::AppState,
    widgets::{
        focusable::{focus_block, focus_table},
        legend::Legend,
    },
};

pub struct ImageList {}

impl StatefulWidget for &mut ImageList {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let project = &state.selected_project();
        let rows: Vec<Row> = project
            .images
            .iter()
            .map(|img| {
                let mut spans: Vec<Span> = vec![];

                let pushed = if let Some(digest) = img.digest
                    && let Some(remote_images) =
                        state.store.project_remote_images.get(&project.name)
                    && remote_images.iter().any(|ri| ri.digest == digest)
                {
                    "✓".to_owned().light_green()
                } else {
                    "⨯".to_owned().red()
                };

                let digest =
                    span![img.digest.map(encode_hex).unwrap_or("-".to_owned())].light_yellow();

                spans.push(pushed);
                spans.push(encode_hex(img.id).light_green());
                spans.push(digest);
                spans.push(Span::from(img.tags.join(", ")));

                Row::new(spans)
            })
            .collect();
        let is_focused = state.focus == Focus::Images;
        let table = focus_table(is_focused)
            .block(
                focus_block(is_focused)
                    .title("Images")
                    .title_top(Legend::new(&IMAGE_ACTIONS).as_line(state).right_aligned()),
            )
            .widths(constraints![==6, ==8, ==8, *=1])
            .header(Row::new(["Pushed", "Id", "Digest", "Tags"]).bold())
            .rows(rows);

        StatefulWidget::render(table, area, buf, &mut state.image_table_state);
    }
}

impl Component<&mut AppState> for ImageList {
    async fn handle_key_events(&mut self, ke: &KeyEvent, state: &mut AppState) -> Action {
        state.config.match_actions(
            ke,
            &[
                Action::FocusProjects,
                Action::DeleteImage,
                Action::PushImage,
            ],
        )
    }
}
