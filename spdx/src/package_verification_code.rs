use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageVerificationCode {
    pub value: String,
    pub excludes: Option<String>,
}
