use std::path::Path;

use super::Algorithm;
use serde::{Deserialize, Serialize};
use crate:: utilities::hash256_for_path;

#[derive(Serialize, Deserialize)]
pub struct Checksum {
    pub algorithm: Algorithm,
    pub value: String,
}

impl Checksum {
    /// Create new checksum.
    pub fn new(algorithm: Algorithm, value: String) -> Self {
        Self { algorithm, value }
    }

    pub fn try_sha256_from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            algorithm: Algorithm::SHA256,
            value: hash256_for_path(path),
        }
    }
}
