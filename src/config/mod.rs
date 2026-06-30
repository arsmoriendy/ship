use crate::prelude::*;

#[derive(Deserialize, Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub project_registries: HashMap<String, String>,
    pub registry_commands: HashMap<String, RegistryCommands>,
    pub command_behaviours: CommandBehaviours,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegistryCommands {
    pub delete_image: String,
    pub list_digests: String,
}

#[derive(Deserialize, Serialize, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct CommandBehaviours {
    #[default(CommandBehaviour::Interactive)]
    pub push_image: CommandBehaviour,
    pub delete_image: CommandBehaviour,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum CommandBehaviour {
    #[default]
    Async,
    Interactive,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_file = config_path()?;

        if !config_file.exists() {
            let config_parent = config_file
                .parent()
                .with_context(|| anyhow!("failed to retrieve config file dirname"))?;
            fs::create_dir_all(config_parent)?;
            let config = Self::default();
            fs::write(config_file, serde_json::to_string_pretty(&config)?)?;
            return Ok(config);
        }

        let config_str =
            fs::read_to_string(config_file).with_context(|| "failed to read config file")?;
        serde_json::from_str(&config_str).with_context(|| "failed to parse config file")
    }
}
