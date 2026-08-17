use crate::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawLocalImage {
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

pub type Digest = [u8; 32];
pub type Id = [u8; 32];

#[derive(Debug, Clone)]
pub struct LocalImage {
    pub containers: u64,
    pub digest: Option<Digest>,
    pub id: Id,
    pub tags: BTreeSet<String>,

    pub created_at: String,
    pub created_since: String,
    pub repository: String,
    pub shared_size: String,
    pub size: String,
    pub unique_size: String,
}

#[derive(Error, Debug)]
pub enum ParseImageError {
    #[error("failed parsing \"Containers\" field: {0}")]
    Containers(String),
    #[error("failed parsing \"Repository\" field: {0}")]
    Repository(String),
    #[error("failed parsing \"ID\" field: {0}")]
    Id(String),
    #[error("failed parsing \"Digest\" field: {0}")]
    Digest(String),
}

impl TryFrom<&RawLocalImage> for LocalImage {
    type Error = ParseImageError;
    fn try_from(raw: &RawLocalImage) -> Result<Self, Self::Error> {
        Ok(LocalImage {
            containers: raw
                .containers
                .parse()
                .map_err(|_| ParseImageError::Containers(raw.containers.clone()))?,
            digest: if raw.digest == "<none>" {
                None
            } else {
                Some(
                    parse_prefixed_sha256(&raw.digest)
                        .map_err(|_| ParseImageError::Digest(raw.digest.clone()))?,
                )
            },
            id: parse_prefixed_sha256(raw.id.as_str())
                .map_err(|_| ParseImageError::Id(raw.id.clone()))?,

            tags: BTreeSet::from([raw.tag.clone()]),
            created_at: raw.created_at.clone(),
            created_since: raw.created_since.clone(),
            repository: raw.repository.clone(),
            shared_size: raw.shared_size.clone(),
            size: raw.size.clone(),
            unique_size: raw.unique_size.clone(),
        })
    }
}

impl LocalImage {
    pub fn list() -> Result<Vec<LocalImage>> {
        let res = docker!(
            "image",
            "list",
            "--format",
            "json",
            "--no-trunc",
            "--all",
            "--digests"
        )
        .output()?;
        if !res.status.success() {
            String::from_utf8(res.stderr).with_context(|| "Failed parsing stderr")?;
        }
        let out = res.stdout;
        let out_str = String::from_utf8(out).with_context(|| "Failed parsing stdout")?;
        let mut img_strs = out_str.split("\n").peekable();

        let mut images: Vec<LocalImage> = vec![];
        loop {
            if let Some(img_str) = img_strs.next()
        // ignore last string, i.e, trailing "\n"
            && img_strs.peek().is_some()
            {
                let raw = serde_json::from_str::<RawLocalImage>(img_str)
                    .with_context(|| "Failed to parse image")?;
                let id =
                    parse_prefixed_sha256(&raw.id).with_context(|| "Failed to parse image id")?;
                if let Some(img) = images.iter_mut().find(|img| img.id == id) {
                    img.tags.insert(raw.tag);
                    if img.digest.is_none() && raw.digest != "<none>" {
                        img.digest = Some(parse_prefixed_sha256(raw.digest.as_str())?);
                    }
                } else {
                    let parsed: LocalImage =
                        (&raw).try_into().with_context(|| "Failed to parse image")?;
                    let pos = match images.binary_search_by(|img| img.id.cmp(&parsed.id)) {
                        Ok(p) => p,
                        Err(p) => p,
                    };
                    images.insert(pos, parsed);
                }
                continue;
            }
            break;
        }

        Ok(images)
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct RemoteImage {
    pub digest: Digest,
    pub tags: BTreeSet<String>,
}

#[derive(Deserialize, Debug)]
pub struct RawRemoteImage {
    pub digest: String,
    pub tags: BTreeSet<String>,
}

impl TryFrom<RawRemoteImage> for RemoteImage {
    type Error = ParseImageError;
    fn try_from(raw: RawRemoteImage) -> Result<Self, Self::Error> {
        Ok(RemoteImage {
            digest: parse_prefixed_sha256(&raw.digest)
                .map_err(|_| ParseImageError::Digest(raw.digest.clone()))?,
            tags: raw.tags,
        })
    }
}

impl TryFrom<&LocalImage> for RemoteImage {
    type Error = ParseImageError;
    fn try_from(image: &LocalImage) -> Result<Self, Self::Error> {
        Ok(RemoteImage {
            digest: image
                .digest
                .ok_or(ParseImageError::Digest(String::from("<none>")))?,
            tags: image.tags.clone(),
        })
    }
}

#[derive(Clone)]
pub enum Image {
    Local(LocalImage),
    Remote(RemoteImage),
}
