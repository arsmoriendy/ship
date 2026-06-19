use crate::{image::Image, prelude::*, project::Project};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub project_registries: HashMap<String, String>,
    pub registry_commands: HashMap<String, RegistryCommands>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryCommands {
    delete_image: String,
    list_digests: String,
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
        let cmd = self
            .delete_image
            .replace("{id}", &encode_hex(image.id))
            .replace("{repository}", &image.repository)
            .replace(
                "{digest}",
                &image
                    .digest
                    .map(|dig| encode_hex(dig))
                    .unwrap_or("<none>".to_owned()),
            );
        Self::run_cmd(&cmd).with_context(|| "failed to delete image")?;
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
        let project_dirs = directories::ProjectDirs::from("top", "nugs", "ship")
            .ok_or(anyhow!("failed to parse project directories"))?;
        let config_str = fs::read_to_string(project_dirs.config_dir().join("config.json"))
            .with_context(|| "failed to read config file")?;
        Ok(serde_json::from_str(&config_str).with_context(|| "failed to parse config file")?)
    }
}
