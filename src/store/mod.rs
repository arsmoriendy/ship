use crate::prelude::*;

#[derive(Deserialize, Serialize, Default)]
pub struct Store {
    pub project_registry_digests: HashMap<String, Vec<[u8; 32]>>,
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

        return Ok(store);
    }

    pub fn save(&self) -> Result<()> {
        fs::write(state_path()?, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let file_path = state_path()?;
        if !file_path.exists() {
            return Ok(Self::new().with_context(|| anyhow!("failed to create new state file"))?);
        }
        let file_content = fs::read_to_string(file_path.clone())?;
        Ok(serde_json::from_str(file_content.as_str())?)
    }

    pub fn sync<S: FnOnce(&mut Self) -> ()>(&mut self, setter: S) -> Result<()> {
        setter(self);
        self.save()?;
        Ok(())
    }
}
