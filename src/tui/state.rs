use crate::{
    image::{Image, LocalImage},
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

#[derive(Clone)]
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

    pub fn find_project_with_name(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|proj| proj.name == name)
    }

    pub fn try_find_project_with_name(&self, name: &str) -> Result<&Project> {
        self.find_project_with_name(name)
            .ok_or(anyhow!("Cannot find project \"{name}\""))
    }

    pub fn try_find_local_image_with_id(
        &self,
        project_name: &str,
        id: &crate::image::Id,
    ) -> Result<&LocalImage> {
        self.try_find_project_with_name(project_name)?
            .try_find_local_image_with_id(id)
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
        self.sync_store_images();
        Ok(())
    }

    pub fn sync_project_images_from_store(&mut self, project_name: &str) -> Result<()> {
        let Some(store_images) = self.store.project_remote_images.get(project_name) else {
            return Err(anyhow!("Cannot find project \"{project_name}\" in store"));
        };

        let Some(project) = self
            .projects
            .iter_mut()
            .find(|proj| proj.name == project_name)
        else {
            return Err(anyhow!("Cannot find project \"{project_name}\" in state"));
        };

        for store_img in store_images.iter() {
            project.merge_remote_image(store_img);
        }

        Ok(())
    }

    pub fn sync_store_images(&mut self) {
        for (project_name, store_images) in &self.store.project_remote_images {
            let Some(project) = self
                .projects
                .iter_mut()
                .find(|proj| proj.name == project_name.as_str())
            else {
                continue;
            };

            for store_img in store_images.iter() {
                project.merge_remote_image(store_img);
            }
        }
    }
}
