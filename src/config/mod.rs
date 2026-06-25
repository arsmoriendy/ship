use crate::{image::Image, prelude::*, project::Project};

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
    delete_image: String,
    list_digests: String,
}

#[derive(Deserialize, Serialize, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct CommandBehaviours {
    #[default(CommandBehaviour::Interactive)]
    pub push_image: CommandBehaviour,
    pub delete_image: CommandBehaviour,
    pub list_digests: CommandBehaviour,
}

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum CommandBehaviour {
    #[default]
    Async,
    Interactive,
}

impl RegistryCommands {
    fn run_cmd<'a>(cmd: &'a str) -> Result<String> {
        let res = Command::new("sh").args(["-c", cmd]).output()?;
        if !res.status.success() {
            return Err(anyhow!("{}", String::from_utf8(res.stderr)?))
                .with_context(|| format!("failed to run command: \"{}\"", cmd));
        }
        Ok(String::from_utf8(res.stdout)?)
    }

    pub fn delete_image<'a>(&self, image: &'a Image) -> Result<()> {
        let digest = image.digest.ok_or(anyhow!("image has no digest"))?;
        let cmd = self
            .delete_image
            .replace("{id}", &encode_hex(image.id))
            .replace("{repository}", &image.repository)
            .replace("{digest}", &encode_hex(digest));
        Self::run_cmd(&cmd)?;
        Ok(())
    }

    pub fn list_digests<'a>(&self, project: &'a Project) -> Result<Vec<[u8; 32]>> {
        let cmd = self.list_digests.replace("{project}", &project.name);
        let res = Self::run_cmd(&cmd).with_context(|| "failed to list images")?;
        let prefixed_digests: Vec<String> = serde_json::from_str(res.as_str())?;
        let mut digests: Vec<[u8; 32]> = vec![];
        for pd in prefixed_digests {
            digests.push(parse_prefixed_sha256(pd.as_str())?);
        }
        Ok(digests)
    }
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
        Ok(serde_json::from_str(&config_str).with_context(|| "failed to parse config file")?)
    }
}
