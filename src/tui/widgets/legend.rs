use crate::tui::{actions::Action, prelude::*, state::AppState};

#[derive(new)]
pub struct Legend {
    actions: &'static [Action],
}

impl Legend {
    pub fn as_line<'a>(&self, state: &mut AppState) -> Line<'a> {
        let legend = self
            .actions
            .iter()
            .map(|a| {
                [
                    span![a].yellow(),
                    ": ".white(),
                    state
                        .config
                        .action_keymaps(a)
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
                        .white(),
                ]
            })
            .collect::<Vec<_>>()
            .join(&span![" | "].blue());
        Line::from(legend)
    }
}

impl StatefulWidget for &mut Legend {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.as_line(state).render(area, buf);
    }
}
