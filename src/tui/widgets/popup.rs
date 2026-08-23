use crate::tui::{
    actions::{Action, POPUP_ACTIONS},
    component::Component,
    prelude::*,
    state::AppState,
    widgets::legend::Legend,
};

#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl StatefulWidget for Popup<'_> {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_top(Legend::new(&POPUP_ACTIONS).as_line(state).right_aligned())
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

#[derive(Default)]
pub struct PopupComponent {}

impl Component<&mut AppState> for PopupComponent {
    async fn handle_key_events(&mut self, ke: &KeyEvent, state: &mut AppState) -> Action {
        state.config.match_actions(ke, &POPUP_ACTIONS)
    }
}
