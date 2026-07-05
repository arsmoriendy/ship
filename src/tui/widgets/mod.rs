mod focusable;
mod footer;
mod image_list;
mod legend;
mod popup;
mod project_list;

use ratatui::text::ToText;

use crate::tui::{state::PopupVariant, widgets::popup::PopupComponent};

use super::{
    actions::Action,
    component::Component,
    prelude::*,
    state::{AppState, Focus},
    widgets::{footer::Footer, image_list::ImageList, popup::Popup, project_list::ProjectList},
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
        let [projects, images] = vertical![==50%,==50%].areas(main);
        let popup = Rect {
            x: 3,
            y: area.height / 2 - 3,
            width: area.width - 6,
            height: 6,
        };

        self.project_list.render(projects, buf, state);
        self.image_list.render(images, buf, state);
        self.footer.render(footer, buf, state);
        if let Some(pop) = &state.popup.clone() {
            let (title, content, style) = match pop {
                PopupVariant::Info(s) => ("Info", s.to_text(), Style::default()),
                PopupVariant::Error(s) => ("Error", s.to_text(), Style::default().red()),
            };
            Popup::default()
                .title(title)
                .content(content)
                .style(style)
                .render(popup, buf, state);
        }
    }
}

impl Component<&mut AppState> for RootWidget {
    async fn handle_events(&mut self, event: &Event, state: &mut AppState) -> Action {
        let focused_act = match state.focus {
            Focus::Projects => self.project_list.handle_events(event, state).await,
            Focus::Images => self.image_list.handle_events(event, state).await,
            Focus::Popup(_) => PopupComponent::default().handle_events(event, state).await,
        };

        if focused_act != Action::Noop {
            return focused_act;
        }

        let global_act = match event {
            Event::Key(ke) => self.handle_key_events(ke, state).await,
            _ => Action::Noop,
        };

        return global_act;
    }

    async fn handle_key_events(&mut self, ke: &KeyEvent, state: &mut AppState) -> Action {
        state
            .config
            .match_actions(ke, &[Action::SelectDown, Action::SelectUp, Action::Quit])
    }
}
