use super::Algorithm;
use serde::{Deserialize, Serialize};
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
}
