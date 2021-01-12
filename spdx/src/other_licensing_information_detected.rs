use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/
#[derive(Debug, Serialize, Deserialize)]
pub struct OtherLicensingInformationDetected {
    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#61-license-identifier
    pub license_identifier: String,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#62-extracted-text
    pub extracted_text: String,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#63-license-name
    pub license_name: String,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#64-license-cross-reference
    pub license_cross_reference: Option<Vec<String>>,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/#65-license-comment
    pub license_comment: Option<String>,
}
