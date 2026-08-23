mod actions;
mod component;
mod config;
mod external_command;
mod prelude;
mod state;
mod widgets;

use crate::{
    prelude::*,
    project::Project,
    store::Store,
    tui::{
        actions::Action,
        component::Component,
        config::Config,
        state::{AppState, Focus},
        widgets::RootWidget,
    },
};
use anyhow::Ok;
use prelude::*;
use std::time::Duration;

pub struct App {
    root_component: RootWidget,
    state: Arc<Mutex<AppState>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let projects = Project::list()?;
        let config = Config::new()?;
        let store = Store::load()?;

        let mut state = AppState {
            projects,
            config,
            store,

            loading: None,
            spinner_frame: 0,
            exit: false,

            image_table_state: TableState::default().with_selected(0),
            project_table_state: TableState::default().with_selected(0),

            popup: None,

            focus: Focus::Projects,
        };

        state.sync_store_images();

        Ok(App {
            root_component: RootWidget::new(),
            state: Arc::new(Mutex::new(state)),
        })
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            let mut state = self.state.lock().await;

            if state.exit {
                break;
            };

            state.spinner_frame = if state.spinner_frame == SPINNER_SEQUENCE.len() - 1 {
                0
            } else {
                state.spinner_frame + 1
            };

            terminal.draw(|frame| {
                frame.render_stateful_widget(&mut self.root_component, frame.area(), &mut state);
            })?;

            drop(state);

            self.handle_events(terminal).await?;
        }
        Ok(())
    }

    async fn handle_events(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            let mut mtx = self.state.lock().await;
            let action = self
                .root_component
                .handle_events(&event::read()?, &mut mtx)
                .await
                .clone();
            drop(mtx);

            if let Err(e) = self.handle_action(action, terminal).await {
                let mut mtx = self.state.lock().await;
                mtx.focus = Focus::Popup(Box::new(mtx.focus.clone()));
                mtx.popup = Some(state::PopupVariant::Error(e.to_string()))
            };
        }
        Ok(())
    }

    async fn handle_action(
        &mut self,
        action: Action,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        match action {
            Action::SelectUp => self.select_up().await?,
            Action::SelectDown => self.select_down().await?,
            Action::FocusImages => self.focus_images().await?,
            Action::FocusProjects => self.focus_projects().await?,
            Action::ClosePopup => self.close_popup().await?,
            Action::PushImage => self.push_image(terminal).await?,
            Action::PullImage => self.pull_image(terminal).await?,
            Action::DeleteImage => self.delete_image(terminal).await?,
            Action::ListImages => self.list_images().await?,
            Action::PruneImages => self.prune_images(terminal).await?,
            Action::Quit => self.quit().await?,
            _ => {}
        };
        Ok(())
    }
}
