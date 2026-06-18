use std::io::Read;

use crate::{image::Image, prelude::*};

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
}

impl RegistryCommands {
    fn run_cmd<'a>(cmd: &'a str) -> Result<()> {
        let mut res = Command::new("sh").args(["-c", cmd]).spawn()?;
        let status = res.wait()?;
        if !status.success() {
            let mut err_msg = String::new();
            if let Some(mut stderr) = res.stderr {
                stderr.read_to_string(&mut err_msg)?;
            } else {
                err_msg.push_str("non zero exit status");
            }
            return Err(anyhow!(err_msg))
                .with_context(|| format!("failed to run command: \"{}\"", cmd));
        }
        Ok(())
    }

    pub fn delete_image<'a>(&self, image: &'a Image, tag: Option<&'a str>) -> Result<()> {
        let mut del_cmd = self
            .delete_image
            .replace("{id}", &encode_hex(image.id))
            .replace("{repository}", &image.repository);
        if let Some(tag) = tag {
            del_cmd = del_cmd.replace("{tag}", tag)
        };
        Self::run_cmd(&del_cmd).with_context(|| "failed to delete image")?;
        Ok(())
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
