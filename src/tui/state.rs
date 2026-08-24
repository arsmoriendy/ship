use crate::{
    image::{Image, LocalImage, RawLocalImage},
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

    pub fn try_get_selected_registry(&self) -> Result<&String> {
        self.selected_registry()
            .ok_or(anyhow!("Selected project has no configured registry"))
    }

    pub fn selected_cmds(&self) -> Option<RegistryCommands> {
        let project = self.selected_project();
        let reg = self.config.project_registries.get(&project.name)?;
        let cmds = &self.config.registry_commands;
        Some(cmds.get(reg.as_str())?.clone())
    }

    pub fn refresh_projects(&mut self) -> Result<()> {
        self.projects = self.list_projects()?;
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

    pub fn list_images(&self) -> Result<Vec<LocalImage>> {
        let res = cmd!(
            self.get_oci_cmd(),
            "image",
            "list",
            "--format",
            "json",
            "--no-trunc",
            "--all",
            "--digests"
        )
        .output()?;
        if !res.status.success() {
            String::from_utf8(res.stderr).with_context(|| "Failed parsing stderr")?;
        }
        let out = res.stdout;
        let out_str = String::from_utf8(out).with_context(|| "Failed parsing stdout")?;
        let mut img_strs = out_str.split("\n").peekable();

        let mut images: Vec<LocalImage> = vec![];
        loop {
            if let Some(img_str) = img_strs.next()
        // ignore last string, i.e, trailing "\n"
            && img_strs.peek().is_some()
            {
                let raw = serde_json::from_str::<RawLocalImage>(img_str)
                    .with_context(|| "Failed to parse image")?;
                let id =
                    parse_prefixed_sha256(&raw.id).with_context(|| "Failed to parse image id")?;
                if let Some(img) = images.iter_mut().find(|img| img.id == id) {
                    img.tags.insert(raw.tag);
                    if img.digest.is_none() && raw.digest != "<none>" {
                        img.digest = Some(parse_prefixed_sha256(raw.digest.as_str())?);
                    }
                } else {
                    let parsed: LocalImage =
                        (&raw).try_into().with_context(|| "Failed to parse image")?;
                    let pos = match images.binary_search_by(|img| img.id.cmp(&parsed.id)) {
                        Ok(p) => p,
                        Err(p) => p,
                    };
                    images.insert(pos, parsed);
                }
                continue;
            }
            break;
        }

        Ok(images)
    }

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let mut projects: Vec<Project> = vec![];
        let images = self.list_images()?;
        for img in images {
            let repo = img.repository.clone();
            let project_name = Project::get_project_name(repo.as_str())?;
            if let Some(project) = projects.iter_mut().find(|p| p.name == project_name) {
                project.images.push(Image::Local(img))
            } else {
                let mut new_project = Project::new(project_name);
                new_project.images.push(Image::Local(img));
                projects.push(new_project);
            }
        }
        Ok(projects)
    }

    pub fn get_oci_cmd(&self) -> &str {
        &self.config.oci_cmd
    }
}
