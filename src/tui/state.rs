use crate::{
    config::{Config, RegistryCommands},
    image::Image,
    project::Project,
    store::Store,
    tui::prelude::*,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Focus {
    Projects,
    Images,
}

pub struct AppState {
    pub projects: Vec<Project>,
    pub config: Config,
    pub store: Store,

    pub loading: Option<String>,
    pub spinner_frame: usize,
    pub exit: bool,

    pub project_table_state: TableState,
    pub image_table_state: TableState,

    pub focus: Focus,
}

impl AppState {
    pub fn selected_project(&self) -> &Project {
        &self.projects[self.project_table_state.selected().unwrap()]
    }

    pub fn selected_image(&self) -> &Image {
        &self.selected_project().images[self.image_table_state.selected().unwrap()]
    }

    pub fn selected_registry(&self) -> Option<&String> {
        let project = self.selected_project();
        self.config.project_registries.get(&project.name)
    }

    pub fn selected_cmds(&self) -> Option<RegistryCommands> {
        let project = self.selected_project();
        let reg = self.config.project_registries.get(&project.name)?;
        let cmds = &self.config.registry_commands;
        Some(cmds.get(reg.as_str())?.clone())
    }

    pub fn refresh_projects(&mut self) -> Result<()> {
        self.projects = Project::list()?;
        Ok(())
    }
}
