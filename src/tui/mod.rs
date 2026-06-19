mod prelude;
mod widgets;

use crate::config::Config;
use crate::project::Project;
use crate::{prelude::*, relation};
use prelude::*;

#[derive(PartialEq)]
pub enum Focus {
    Projects,
    Images,
}

pub struct App {
    pub projects: Vec<Project>,
    pub config: Config,

    pub project_digests: HashMap<String, Vec<[u8; 32]>>,

    pub exit: bool,

    pub selected_project: usize,
    pub selected_image: usize,

    pub focus: Focus,
}

impl App {
    pub fn new() -> Result<Self> {
        let projects = Project::list()?;
        let config = Config::new()?;
        Ok(App {
            projects,
            config,
            project_digests: HashMap::new(),
            exit: false,
            selected_project: 0,
            selected_image: 0,
            focus: Focus::Projects,
        })
    }

    pub fn refresh(&mut self) -> Result<()> {
        let projects = Project::list()?;
        let config = Config::new()?;

        self.projects = projects;
        self.config = config;
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let should_clear = self.handle_events()?;
            if should_clear {
                terminal.clear()?;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_events(&mut self) -> Result<bool> {
        match event::read()? {
            Event::Key(ke) if ke.kind == KeyEventKind::Press => Ok(self.handle_key_events(ke)?),
            _ => Ok(false),
        }
    }

    fn handle_key_events(&mut self, ke: KeyEvent) -> Result<bool> {
        match self.focus {
            Focus::Projects => match ke.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.selected_project = self
                        .selected_project
                        .saturating_add(1)
                        .clamp(0, self.projects.len().saturating_sub(1).try_into()?);
                    self.selected_image = 0
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.selected_project = self
                        .selected_project
                        .saturating_sub(1)
                        .clamp(0, self.projects.len().saturating_sub(1).try_into()?);
                    self.selected_image = 0
                }
                KeyCode::Char('L') | KeyCode::Enter => self.focus = Focus::Images,
                KeyCode::Char('f') => {
                    let project = &self.projects[self.selected_project];
                    let Some(reg) = &self.config.project_registries.get(&project.name) else {
                        return Ok(false);
                    };
                    let Some(cmds) = self.config.registry_commands.get(reg.as_str()) else {
                        return Ok(false);
                    };
                    let project_registry_digests = cmds.list_digests(&project)?;
                    self.project_digests
                        .insert(project.name.clone(), project_registry_digests);
                }
                _ => {}
            },
            Focus::Images => match ke.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.selected_image = self.selected_image.saturating_add(1).clamp(
                        0,
                        self.projects[self.selected_project]
                            .images
                            .len()
                            .saturating_sub(1)
                            .try_into()?,
                    )
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.selected_image = self.selected_image.saturating_sub(1).clamp(
                        0,
                        self.projects[self.selected_project]
                            .images
                            .len()
                            .saturating_sub(1)
                            .try_into()?,
                    )
                }
                KeyCode::Char('H') | KeyCode::Backspace => self.focus = Focus::Projects,
                KeyCode::Char('P') => {
                    let project = &self.projects[self.selected_project];
                    let Some(reg) = &self.config.project_registries.get(&project.name) else {
                        return Ok(false);
                    };
                    relation::push(&project.images[self.selected_image], reg)?;
                    self.refresh()?;
                    return Ok(true);
                }
                KeyCode::Char('D') => {
                    let project = &self.projects[self.selected_project];
                    let image = &project.images[self.selected_image];
                    let Some(reg) = &self.config.project_registries.get(&project.name) else {
                        return Ok(false);
                    };
                    let Some(cmds) = self.config.registry_commands.get(reg.as_str()) else {
                        return Ok(false);
                    };
                    cmds.delete_image(image)?;
                    self.refresh()?;
                    return Ok(true);
                }
                _ => {}
            },
        }
        match ke.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => {
                self.refresh()?;
                return Ok(true);
            }
            _ => {}
        }
        Ok(false)
    }
}
