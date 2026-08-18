pub use crate::prelude::*;

pub type Digest = [u8; 32];
pub type Id = [u8; 32];

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
