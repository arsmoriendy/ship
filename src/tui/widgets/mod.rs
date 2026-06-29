mod focusable;
mod footer;
mod image_list;
mod project_list;

use super::{
    actions::Action,
    component::Component,
    prelude::*,
    state::{AppState, Focus},
    widgets::{footer::Footer, image_list::ImageList, project_list::ProjectList},
};

pub struct RootWidget {
    project_list: ProjectList,
    image_list: ImageList,
    footer: Footer,
}

impl RootWidget {
    pub fn new() -> Self {
        Self {
            project_list: ProjectList {},
            image_list: ImageList {},
            footer: Footer {},
        }
    }
}

impl StatefulWidget for &mut RootWidget {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut AppState)
    where
        Self: Sized,
    {
        let [main, footer] = vertical![*=1, ==1].areas(area);
        let [projects, images] = horizontal![==50%,==50%].areas(main);

        self.project_list.render(projects, buf, state);
        self.image_list.render(images, buf, state);
        self.footer.render(footer, buf, state);
    }
}

impl Component<&mut AppState> for RootWidget {
    async fn handle_events(&mut self, event: &Event, state: &mut AppState) -> Action {
        let focused_act = match state.focus {
            Focus::Projects => self.project_list.handle_events(event, state).await,
            Focus::Images => self.image_list.handle_events(event, state).await,
        };

        let global_act = match event {
            Event::Key(ke) => self.handle_key_events(ke, state).await,
            _ => Action::Noop,
        };

        match focused_act {
            Action::Noop => global_act,
            _ => focused_act,
        }
    }

    async fn handle_key_events(&mut self, ke: &KeyEvent, _state: &mut AppState) -> Action {
        match ke.code {
            KeyCode::Char('j') | KeyCode::Down => Action::SelectDown,
            KeyCode::Char('k') | KeyCode::Up => Action::SelectUp,
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
            _ => Action::Noop,
        }
    }
}
