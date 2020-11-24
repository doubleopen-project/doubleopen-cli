use serde::{Deserialize, Serialize};

/// Policy violations are emitted when Policy Engine determines that a file doesn't conform with the provided policy.
#[derive(Serialize, Deserialize)]
pub struct PolicyViolation {
    /// SPDX ID of the package that the violating file is a part of.
    pub package_id: String,

    /// Name of the package that the violating file is a part of.
    pub package_name: String,

    /// SPDX ID of the violating file.
    pub file_id: String,

    /// Name of the violating file.
    pub file_name: String,

    /// SPDX license expression of the violating file.
    pub file_license: String,
}
