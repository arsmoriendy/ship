use crate::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub project_registries: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let project_dirs = directories::ProjectDirs::from("top", "nugs", "ship")
            .ok_or(anyhow!("failed to parse project directories"))?;
        let config_str = fs::read_to_string(project_dirs.config_dir().join("config.json"))
            .with_context(|| "failed to read config file")?;
        Ok(serde_json::from_str(&config_str).with_context(|| "failed to parse config file")?)
    }
}
