use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Auth {}

#[derive(Deserialize, Debug)]
pub struct DockerConfig {
    auths: HashMap<String, Auth>,
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("cannot find home directory")]
    HomeDirError,
}

pub fn parse() -> Result<DockerConfig> {
    let mut cfg_path = std::env::home_dir().ok_or(ParseError::HomeDirError)?;
    cfg_path.push(".docker");
    cfg_path.push("config.json");
    let cfg_str = fs::read_to_string(cfg_path.canonicalize()?)?;
    let cfg: DockerConfig = serde_json::from_str(&cfg_str)?;
    Ok(cfg)
}
