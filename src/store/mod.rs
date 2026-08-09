use crate::{image::RemoteImage, prelude::*};

#[derive(Deserialize, Serialize, Default)]
pub struct Store {
    pub project_registry_digests: HashMap<String, Vec<[u8; 32]>>,
    pub project_remote_images: HashMap<String, Vec<RemoteImage>>,
}

impl Store {
    pub fn new() -> Result<Self> {
        let store = Self::default();

        let file_path = state_path()?;
        let parent_path = file_path
            .parent()
            .with_context(|| anyhow!("failed to retrieve state file dirname"))?;
        fs::create_dir_all(parent_path)?;
        fs::write(file_path, serde_json::to_string(&store)?)?;

        Ok(store)
    }

    pub fn save(&self) -> Result<()> {
        fs::write(state_path()?, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let file_path = state_path()?;
        if !file_path.exists() {
            return Self::new().with_context(|| anyhow!("failed to create new state file"));
        }
        let file_content = fs::read_to_string(file_path.clone())?;
        Ok(serde_json::from_str(file_content.as_str())?)
    }

    pub fn sync<S: FnOnce(&mut Self) -> Result<()>>(&mut self, setter: S) -> Result<()> {
        setter(self)?;
        self.save()?;
        Ok(())
    }

    pub fn remove_digest(&mut self, project_name: &str, digest: &[u8; 32]) -> Result<()> {
        self.sync(|store| {
            let digests = store
                .project_registry_digests
                .get_mut(project_name)
                .ok_or(anyhow!["Cannot find digest"])?;
            digests.retain(|d| d != digest);
            Ok(())
        })
    }

    pub fn push_digest(&mut self, project_name: &str, digest: &[u8; 32]) -> Result<()> {
        self.sync(|store| {
            let digests = store
                .project_registry_digests
                .get_mut(project_name)
                .ok_or(anyhow!["Cannot find digest"])?;
            digests.push(*digest);
            Ok(())
        })
    }

    pub fn remove_remote_image(
        &mut self,
        project_name: &str,
        remote_image: &RemoteImage,
    ) -> Result<()> {
        self.sync(|store| {
            let remote_images = store
                .project_remote_images
                .get_mut(project_name)
                .ok_or(anyhow!["Cannot find project"])?;
            remote_images.retain(|ri| &ri.digest != &remote_image.digest);
            Ok(())
        })
    }

    pub fn push_remote_image(
        &mut self,
        project_name: &str,
        remote_image: RemoteImage,
    ) -> Result<()> {
        self.sync(|store| {
            let remote_images = store
                .project_remote_images
                .get_mut(project_name)
                .ok_or(anyhow!["Cannot find project"])?;
            remote_images.push(remote_image);
            Ok(())
        })
    }
}
