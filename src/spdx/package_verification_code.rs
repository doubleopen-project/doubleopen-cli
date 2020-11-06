use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PackageVerificationCode {
    pub value: String,
    pub excludes: Option<String>,
}
