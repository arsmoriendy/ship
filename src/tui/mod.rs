mod external_command;
mod prelude;
mod state;
mod widgets;

use crate::{
    config::{CommandBehaviour, Config},
    prelude::*,
    project::Project,
    relation,
    store::Store,
    tui::{
        external_command::ExternalCommand,
        state::{AppState, Focus},
        widgets::RootWidget,
    },
};
use anyhow::Ok;
use prelude::*;
use std::time::Duration;

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

            loading: None,
            spinner_frame: 0,
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

            self.handle_events(terminal).await?;
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

    async fn handle_events(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(ke) if ke.kind == KeyEventKind::Press => {
                    Ok(self.handle_key_events(ke, terminal).await?)
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    async fn handle_key_events(&self, ke: KeyEvent, terminal: &mut DefaultTerminal) -> Result<()> {
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
                    let project = state.projects[state.selected_project].clone();
                    let Some(reg) = state.config.project_registries.get(&project.name) else {
                        return Ok(());
                    };
                    let cmds = &state.config.registry_commands;
                    let Some(project_cmds) = cmds.get(reg.as_str()) else {
                        return Ok(());
                    };

                    match state.config.command_behaviours.list_digests {
                        CommandBehaviour::Async => {
                            let mutex = self.state.clone();
                            let project_cmds = project_cmds.clone();
                            state.loading = Some("Fetching images...".to_owned());
                            drop(state);
                            tokio::task::spawn(async move {
                                let project_registry_digests =
                                    project_cmds.list_digests(&project)?;

                                let mut state = mutex.lock().await;
                                state.store.sync(|store| {
                                    store
                                        .project_registry_digests
                                        .insert(project.name.clone(), project_registry_digests);
                                })?;
                                state.loading = None;

                                Ok(())
                            });
                        }
                        CommandBehaviour::Interactive => {
                            let _cmd = ExternalCommand::init(terminal);
                            let project_registry_digests = project_cmds.list_digests(&project)?;

                            state.store.sync(|store| {
                                store
                                    .project_registry_digests
                                    .insert(project.name.clone(), project_registry_digests);
                            })?;
                        }
                    }
                    return Ok(());
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
                    let Some(reg) = state.config.project_registries.get(&project.name) else {
                        return Ok(());
                    };
                    let reg = reg.clone();
                    let image = &project.images[state.selected_image];

                    match state.config.command_behaviours.push_image {
                        CommandBehaviour::Async => {
                            let mutex = self.state.clone();
                            let image = image.clone();

                            state.loading = Some("Pushing image...".to_owned());
                            drop(state);

                            tokio::task::spawn(async move {
                                relation::push(&image, &reg)?;

                                let mut state = mutex.lock().await;
                                state.loading = None;
                                drop(state);

                                Ok(())
                            });

                            return Ok(());
                        }
                        CommandBehaviour::Interactive => {
                            let _cmd = ExternalCommand::init(terminal)?;
                            relation::push(&image, &reg)?;
                        }
                    }

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

                    match state.config.command_behaviours.delete_image {
                        CommandBehaviour::Async => {
                            let mutex = self.state.clone();
                            let project_cmds = cmds.clone();
                            let image = image.clone();

                            state.loading = Some("Deleting image...".to_owned());
                            drop(state);

                            tokio::task::spawn(async move {
                                project_cmds.delete_image(&image)?;

                                let mut state = mutex.lock().await;
                                state.loading = None;
                                drop(state);

                                Ok(())
                            });

                            return Ok(());
                        }
                        CommandBehaviour::Interactive => {
                            let _cmd = ExternalCommand::init(terminal)?;
                            cmds.delete_image(image)?;
                        }
                    }
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
