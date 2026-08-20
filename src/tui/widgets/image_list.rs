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
    tags: BTreeSet<String>,
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
        spans.push(Span::from(
            value.tags.into_iter().collect::<Vec<_>>().join(", "),
        ));

        spans
    }
}

fn check_span<'a>(checked: bool) -> Span<'a> {
    if checked {
        "✓".to_owned().light_green()
    } else {
        "⨯".to_owned().red()
    }
}

pub struct ImageList {}

impl StatefulWidget for &mut ImageList {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let project = &state.selected_project();
        let remote_images = state
            .store
            .project_remote_images
            .get(&project.name)
            .cloned()
            .unwrap_or(vec![]);
        let rows: Vec<Row> = project
            .images
            .iter()
            .map(|img| {
                let row: ImageListRow = img.into();

                let remote = check_span(match &row.digest {
                    Some(digest) => remote_images.iter().any(|ri| &ri.digest == digest),
                    None => false,
                });
                let local = check_span(matches!(img, Image::Local(_)));

                let mut spans: Vec<Span> = row.into();
                spans.insert(0, remote);
                spans.insert(0, local);

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
