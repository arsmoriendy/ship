use crate::prelude::*;

#[derive(Deserialize, Serialize, Default)]
pub struct State {
    pub project_registry_digests: HashMap<String, Vec<[u8; 32]>>,
}

impl State {
    pub fn new() -> Result<Self> {
        let state = Self::default();

        let state_file = state_path()?;
        let parent_path = state_file
            .parent()
            .with_context(|| anyhow!("failed to retrieve state file dirname"))?;
        fs::create_dir_all(parent_path)?;
        fs::write(state_file, serde_json::to_string(&state)?)?;

        return Ok(state);
    }

    pub fn save(&self) -> Result<()> {
        fs::write(state_path()?, serde_json::to_string(self)?)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let sd = state_path()?;
        if !sd.exists() {
            return Ok(Self::new().with_context(|| anyhow!("failed to create new state file"))?);
        }
        let file_content = fs::read_to_string(sd.clone())?;
        Ok(serde_json::from_str(file_content.as_str())?)
    }

    pub fn sync<S: FnOnce(&mut Self) -> ()>(&mut self, setter: S) -> Result<()> {
        setter(self);
        self.save()?;
        Ok(())
    }
}
