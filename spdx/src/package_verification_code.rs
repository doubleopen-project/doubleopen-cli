use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/3-package-information/#39-package-verification-code
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageVerificationCode {
    /// Value of the verification code.
    pub value: String,

    /// Files that were excluded when calculating the verification code.
    pub excludes: Option<String>,
}
