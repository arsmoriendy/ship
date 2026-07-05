use crate::{
    image::Image,
    project::Project,
    store::Store,
    tui::{
        config::{Config, RegistryCommands},
        prelude::*,
    },
};

pub struct AppState {
    pub projects: Vec<Project>,
    pub config: Config,
    pub store: Store,

    pub loading: Option<String>,
    pub spinner_frame: usize,
    pub exit: bool,

    pub project_table_state: TableState,
    pub image_table_state: TableState,

    pub popup: Option<PopupVariant>,

    pub focus: Focus,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Focus {
    Projects,
    Images,
    Popup(Box<Focus>),
}

pub enum PopupVariant {
    Info(String),
    Error(String),
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
