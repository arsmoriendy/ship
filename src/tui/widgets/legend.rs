use crate::tui::{actions::Action, prelude::*, state::AppState};

#[derive(new)]
pub struct Legend {
    actions: &'static [Action],
}

impl StatefulWidget for &mut Legend {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let legend = self
            .actions
            .iter()
            .map(|a| {
                [
                    span![a].yellow(),
                    span![": "],
                    state
                        .config
                        .action_keymaps(&a)
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|km| {
                            format!(
                                "{}{}",
                                match km.modifiers {
                                    Some(m) => format!("{m}+"),
                                    None => "".to_owned(),
                                },
                                km.key
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                        .into(),
                ]
            })
            .collect::<Vec<_>>()
            .join(&span![" | "].blue());

        Line::from(legend).render(area, buf);
    }
}
