use super::prelude::*;

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
