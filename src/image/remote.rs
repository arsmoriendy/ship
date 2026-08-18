use crate::image::LocalImage;

use super::prelude::*;

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
