#[macro_use]
mod macros;

use crate::prelude::*;
use std::num::ParseIntError;

pub fn parse_prefixed_sha256(sha_str: &str) -> Result<[u8; 32]> {
    let vec: Vec<u8> = (7..sha_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&sha_str[i..i + 2], 16))
        .collect::<Result<Vec<u8>, ParseIntError>>()?;
    Ok(vec
        .try_into()
        .map_err(|_| anyhow!("failed parsing sha256"))?)
}
