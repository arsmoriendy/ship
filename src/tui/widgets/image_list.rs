use crate::tui::{
    Focus,
    actions::Action,
    component::Component,
    prelude::*,
    state::AppState,
    widgets::focusable::{focus_block, focus_table},
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
        let rows: Vec<Row> = state
            .selected_project()
            .images
            .iter()
            .map(|img| {
                let mut spans: Vec<Span> = vec![];

                let pushed = if let Some(digest) = img.digest
                    && let Some(project_digests) =
                        state.store.project_registry_digests.get(&project.name)
                    && project_digests.contains(&digest)
                {
                    "✓".to_owned().light_green()
                } else {
                    "⨯".to_owned().red()
                };

                let digest = span![
                    img.digest
                        .map(|dig| encode_hex(dig))
                        .unwrap_or("-".to_owned())
                ]
                .light_yellow();

                spans.push(pushed);
                spans.push(encode_hex(img.id).light_green());
                spans.push(digest);
                spans.push(Span::from(img.tags.join(", ")));

                Row::new(spans)
            })
            .collect();
        let is_focused = state.focus == Focus::Images;
        let table = focus_table(is_focused)
            .block(focus_block(is_focused).title("Images"))
            .widths(constraints![==6, ==8, ==8, *=1])
            .header(Row::new(["Pushed", "Id", "Digest", "Tags"]).bold())
            .rows(rows);

        StatefulWidget::render(
            table,
            area,
            buf,
            &mut TableState::default().with_selected(state.selected_image),
        );
    }
}

impl Component<&mut AppState> for ImageList {
    async fn handle_key_events(&mut self, ke: &KeyEvent, _state: &mut AppState) -> Action {
        match ke.code {
            KeyCode::Char('K') => Action::Focus(Focus::Projects),
            KeyCode::Char('D') => Action::DeleteImage,
            KeyCode::Char('P') => Action::PushImage,
            _ => Action::Noop,
        }
    }
}
