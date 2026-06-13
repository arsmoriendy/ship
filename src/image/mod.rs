use std::num::ParseIntError;

use crate::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawImage {
    pub containers: String,
    pub created_at: String,
    pub created_since: String,
    pub digest: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub repository: String,
    pub shared_size: String,
    pub size: String,
    pub tag: String,
    pub unique_size: String,
}

pub struct Image {
    pub containers: u64,
    pub id: [u8; 32],
    pub tags: Vec<String>,

    pub created_at: String,
    pub created_since: String,
    pub digest: String,
    pub repository: String,
    pub shared_size: String,
    pub size: String,
    pub unique_size: String,
}

#[derive(Error, Debug)]
pub enum ParseImageError {
    #[error("failed parsing \"Containers\" field: {0}")]
    ParseContainersError(String),
    #[error("failed parsing \"Repository\" field: {0}")]
    ParseRepositoryError(String),
    #[error("failed parsing \"ID\" field: {0}")]
    ParseIdError(String),
}

impl TryFrom<&RawImage> for Image {
    type Error = ParseImageError;
    fn try_from(raw: &RawImage) -> Result<Self, Self::Error> {
        Ok(Image {
            containers: raw
                .containers
                .parse()
                .map_err(|_| ParseImageError::ParseContainersError(raw.containers.clone()))?,
            id: Image::parse_id(raw.id.as_str())
                .map_err(|_| ParseImageError::ParseIdError(raw.id.clone()))?,

            tags: vec![raw.tag.clone()],
            created_at: raw.created_at.clone(),
            created_since: raw.created_since.clone(),
            digest: raw.digest.clone(),
            repository: raw.repository.clone(),
            shared_size: raw.shared_size.clone(),
            size: raw.size.clone(),
            unique_size: raw.unique_size.clone(),
        })
    }
}

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("error calling the docker command, status: {0:?}, stderr: \"{1}\"")]
    CommandError(Option<i32>, String),
}

impl Image {
    fn parse_id(id: &str) -> Result<[u8; 32]> {
        let vec: Vec<u8> = (7..id.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&id[i..i + 2], 16))
            .collect::<Result<Vec<u8>, ParseIntError>>()?;
        Ok(vec
            .try_into()
            .map_err(|_| ParseImageError::ParseIdError(id.to_owned()))?)
    }

    pub fn list() -> Result<Vec<Image>> {
        let res = docker!("image", "list", "--format", "json", "--no-trunc", "--all");
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
                let raw = serde_json::from_str::<RawImage>(img_str)
                    .with_context(|| "Failed to parse image")?;
                let id = Image::parse_id(&raw.id).with_context(|| "Failed to parse image id")?;
                if let Some(img) = images.iter_mut().find(|img| img.id == id) {
                    img.tags.push(raw.tag);
                } else {
                    images.push((&raw).try_into().with_context(|| "Failed to parse image")?)
                }
                continue;
            }
            break;
        }

        Ok(images)
    }
}
