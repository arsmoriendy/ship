use crate::{
    image::{Digest, Image, LocalImage, RemoteImage},
    tui::{
        Focus,
        actions::{Action, IMAGE_ACTIONS},
        component::Component,
        prelude::*,
        state::AppState,
        widgets::{
            focusable::{focus_block, focus_table},
            legend::Legend,
        },
    },
};

struct ImageListRow {
    id: Option<[u8; 32]>,
    digest: Option<Digest>,
    tags: Vec<String>,
}

impl From<&Image> for ImageListRow {
    fn from(image: &Image) -> Self {
        match image {
            Image::Local(li) => li.into(),
            Image::Remote(ri) => ri.into(),
        }
    }
}

impl From<&LocalImage> for ImageListRow {
    fn from(image: &LocalImage) -> Self {
        Self {
            digest: image.digest,
            id: Some(image.id),
            tags: image.tags.clone(),
        }
    }
}

impl From<&RemoteImage> for ImageListRow {
    fn from(value: &RemoteImage) -> Self {
        Self {
            digest: Some(value.digest),
            id: None,
            tags: value.tags.clone(),
        }
    }
}

impl From<ImageListRow> for Vec<Span<'_>> {
    fn from(value: ImageListRow) -> Self {
        let mut spans: Vec<Span> = vec![];

        let id = value
            .id
            .map(encode_hex)
            .unwrap_or("-".to_owned())
            .light_green();
        let digest = value
            .digest
            .map(encode_hex)
            .unwrap_or("-".to_owned())
            .light_yellow();

        spans.push(id);
        spans.push(digest);
        spans.push(Span::from(value.tags.join(", ")));

        spans
    }
}

pub struct ImageList {}

impl StatefulWidget for &mut ImageList {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let project = &state.selected_project();
        let mut remote_images = state
            .store
            .project_remote_images
            .get(&project.name)
            .cloned()
            .unwrap_or(vec![]);
        let mut rows: Vec<Row> = project
            .images
            .iter()
            .map(|img| {
                let row: ImageListRow = img.into();

                let remote = if let Some(digest) = &row.digest
                    && let Some(i) = remote_images.iter().position(|ri| &ri.digest == digest)
                {
                    remote_images.swap_remove(i);
                    "✓".to_owned().light_green()
                } else {
                    "⨯".to_owned().red()
                };

                let mut spans: Vec<Span> = row.into();
                spans.insert(0, remote);
                spans.insert(0, "✓".to_owned().light_green());

                Row::new(spans)
            })
            .collect();

        remote_images.iter().for_each(|ri| {
            let row: ImageListRow = ri.into();
            let mut spans: Vec<Span> = row.into();

            spans.insert(0, "✓".to_owned().light_green());
            spans.insert(0, "⨯".to_owned().red());

            rows.push(Row::new(spans));
        });

        let is_focused = state.focus == Focus::Images;
        let table = focus_table(is_focused)
            .block(
                focus_block(is_focused)
                    .title("Images")
                    .title_top(Legend::new(&IMAGE_ACTIONS).as_line(state).right_aligned()),
            )
            .widths(constraints![==5, ==6, ==8, ==8, *=1])
            .header(Row::new(["Local", "Remote", "Id", "Digest", "Tags"]).bold())
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
