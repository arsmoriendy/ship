use crate::tui::{actions::GLOBAL_ACTIONS, prelude::*, state::AppState, widgets::legend::Legend};

pub struct Footer {}

impl StatefulWidget for &mut Footer {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        let [keymaps, loading_indicator] = horizontal![*=1, ==24].areas(area);

        Legend::new(&GLOBAL_ACTIONS).render(keymaps, buf, state);
        if let Some(loading) = &state.loading {
            l![
                span![Color::Green; SPINNER_SEQUENCE[state.spinner_frame]],
                " ",
                span![loading]
            ]
            .alignment(Alignment::Right)
            .render(loading_indicator, buf);
        }
    }
}
