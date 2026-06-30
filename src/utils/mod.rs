#[macro_use]
mod macros;

use directories::ProjectDirs;
use std::{num::ParseIntError, path::PathBuf};

use crate::prelude::*;

pub fn parse_prefixed_sha256(sha_str: &str) -> Result<[u8; 32]> {
    let vec: Vec<u8> = (7..sha_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&sha_str[i..i + 2], 16))
        .collect::<Result<Vec<u8>, ParseIntError>>()?;
    vec
        .try_into()
        .map_err(|_| anyhow!("failed parsing sha256"))
}

pub fn project_dirs() -> Result<ProjectDirs> {
    directories::ProjectDirs::from("top", "nugs", "ship")
        .ok_or(anyhow!("failed to parse project directories"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(project_dirs()?.config_dir().join("config.json"))
}

pub fn state_path() -> Result<PathBuf> {
    Ok(project_dirs()?
        .state_dir()
        .ok_or(anyhow!("failed to locate state directory"))?
        .join("state.json"))
}
