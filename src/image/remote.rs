use crate::image::{Image, LocalImage};

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

macro_rules! merge_tags {
    ($lhs:expr, $rhs:expr) => {
        for tag in &$rhs.tags {
            $lhs.tags.insert(tag.clone());
        }
    };
}

impl RemoteImage {
    pub fn merge_with(&self, img: &mut Image) {
        match img {
            Image::Local(img) => {
                img.digest = Some(self.digest);
                merge_tags!(img, self);
            }
            Image::Remote(img) => {
                img.digest = self.digest;
                merge_tags!(img, self);
            }
        };
    }
}
