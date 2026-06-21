mod prelude;
mod state;
mod widgets;

use std::time::Duration;

use crate::config::Config;
use crate::project::Project;
use crate::store::Store;
use crate::tui::state::{AppState, Focus};
use crate::tui::widgets::RootWidget;
use crate::{prelude::*, relation};
use anyhow::Ok;
use prelude::*;

pub struct App {
    state: Arc<Mutex<AppState>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let projects = Project::list()?;
        let config = Config::new()?;
        let state = Arc::new(Mutex::new(AppState {
            projects,
            config,
            store: Store::load()?,
            exit: false,
            selected_project: 0,
            selected_image: 0,
            focus: Focus::Projects,
        }));
        Ok(App { state })
    }

    pub async fn run(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            let mut state = self.state.lock().await;

            if state.exit {
                break;
            };

            terminal.draw(|frame| {
                frame.render_stateful_widget(RootWidget {}, frame.area(), &mut state);
            })?;

            drop(state);

            self.handle_events().await?;
        }
        Ok(())
    }

    async fn exit(&self) {
        let mut state = self.state.lock().await;
        state.exit = true;
    }

    async fn refresh(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        let projects = Project::list()?;
        let config = Config::new()?;

        state.projects = projects;
        state.config = config;
        Ok(())
    }

    async fn handle_events(&self) -> Result<()> {
        if event::poll(Duration::from_millis(1000))? {
            match event::read()? {
                Event::Key(ke) if ke.kind == KeyEventKind::Press => {
                    Ok(self.handle_key_events(ke).await?)
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    async fn handle_key_events(&self, ke: KeyEvent) -> Result<()> {
        let mut state = self.state.lock().await;
        match state.focus {
            Focus::Projects => match ke.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    state.selected_project = state
                        .selected_project
                        .saturating_add(1)
                        .clamp(0, state.projects.len().saturating_sub(1).try_into()?);
                    state.selected_image = 0
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    state.selected_project = state
                        .selected_project
                        .saturating_sub(1)
                        .clamp(0, state.projects.len().saturating_sub(1).try_into()?);
                    state.selected_image = 0
                }
                KeyCode::Char('L') | KeyCode::Enter => state.focus = Focus::Images,
                KeyCode::Char('f') => {
                    let mutex = self.state.clone();
                    tokio::task::spawn(async move {
                        let state = mutex.lock().await;

                        let project = state.projects[state.selected_project].clone();
                        let Some(reg) = state.config.project_registries.get(&project.name) else {
                            return Ok(());
                        };

                        let cmds = state.config.registry_commands.clone();
                        let Some(project_cmds) = cmds.get(reg.as_str()) else {
                            return Ok(());
                        };

                        drop(state);

                        let project_registry_digests = project_cmds.list_digests(&project)?;

                        let mut state = mutex.lock().await;
                        state.store.sync(|store| {
                            store
                                .project_registry_digests
                                .insert(project.name.clone(), project_registry_digests);
                        })?;

                        Ok(())
                    });
                }
                _ => {}
            },
            Focus::Images => match ke.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    state.selected_image = state.selected_image.saturating_add(1).clamp(
                        0,
                        state.projects[state.selected_project]
                            .images
                            .len()
                            .saturating_sub(1)
                            .try_into()?,
                    )
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    state.selected_image = state.selected_image.saturating_sub(1).clamp(
                        0,
                        state.projects[state.selected_project]
                            .images
                            .len()
                            .saturating_sub(1)
                            .try_into()?,
                    )
                }
                KeyCode::Char('H') | KeyCode::Backspace => state.focus = Focus::Projects,
                KeyCode::Char('P') => {
                    let project = &state.projects[state.selected_project];
                    let Some(reg) = &state.config.project_registries.get(&project.name) else {
                        return Ok(());
                    };
                    relation::push(&project.images[state.selected_image], reg)?;
                    drop(state);
                    self.refresh().await?;
                    return Ok(());
                }
                KeyCode::Char('D') => {
                    let project = &state.projects[state.selected_project];
                    let image = &project.images[state.selected_image];
                    let Some(reg) = &state.config.project_registries.get(&project.name) else {
                        return Ok(());
                    };
                    let Some(cmds) = state.config.registry_commands.get(reg.as_str()) else {
                        return Ok(());
                    };
                    cmds.delete_image(image)?;
                    drop(state);
                    self.refresh().await?;
                    return Ok(());
                }
                _ => {}
            },
        }
        match ke.code {
            KeyCode::Char('q') => {
                drop(state);
                self.exit().await;
                return Ok(());
            }
            KeyCode::Char('r') => {
                drop(state);
                self.refresh().await?;
                return Ok(());
            }
            _ => {}
        }
        Ok(())
    }
}
