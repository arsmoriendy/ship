use crate::tui::{actions::Action, prelude::*};

pub trait Component<S> {
    async fn handle_events(&mut self, event: &Event, state: S) -> Action {
        match event {
            Event::Key(ke) => self.handle_key_events(ke, state).await,
            _ => Action::Noop,
        }
    }

    async fn handle_key_events(&mut self, _ke: &KeyEvent, _state: S) -> Action {
        Action::Noop
    }
}
