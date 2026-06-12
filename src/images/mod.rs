use crate::prelude::*;
use std::process::Command;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub repository: String,
    pub tag: String,
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("error calling the docker command, status: {0:?}, stderr: \"{1}\"")]
    CommandError(Option<i32>, String),
}

pub fn list() -> Result<Vec<Image>> {
    let res = Command::new("docker")
        .args(["image", "list", "--format", "json", "--no-trunc"])
        .output()
        .with_context(|| "Failed spawning docker, make sure docker is installed")?;
    if !res.status.success() {
        DockerError::CommandError(
            res.status.code(),
            String::from_utf8(res.stderr).with_context(|| "Failed parsing stderr")?,
        );
    }
    let out = res.stdout;
    let out_str = String::from_utf8(out).with_context(|| "Failed parsing stdout")?;
    let mut img_strs = out_str.split("\n").peekable();

    let mut images: Vec<Image> = vec![];
    loop {
        if let Some(img_str) = img_strs.next()
        // ignore last string, i.e, trailing "\n"
            && img_strs.peek().is_some()
        {
            let img =
                serde_json::from_str::<Image>(img_str).with_context(|| "Failed to parse image")?;
            images.push(img);
            continue;
        }
        break;
    }
    Ok(images)
}
