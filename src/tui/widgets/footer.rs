use crate::tui::{prelude::*, state::AppState};

pub struct Footer {}

impl StatefulWidget for Footer {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        const SPINNER_SEQUENCE: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let [keymaps, loading_indicator] = horizontal![*=1, ==24].areas(area);

        state.spinner_frame = if state.spinner_frame == SPINNER_SEQUENCE.len() - 1 {
            0
        } else {
            state.spinner_frame + 1
        };

        text!["j: down, k: up, H: focus projects, L: focus images"].render(keymaps, buf);
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
