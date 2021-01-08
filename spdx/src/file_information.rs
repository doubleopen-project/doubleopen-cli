use serde::{Deserialize, Serialize};

use super::{Checksum, FileType, SPDXExpression};

/// ## File Information
///
/// SPDX's [File Information](https://spdx.github.io/spdx-spec/4-file-information/)
#[derive(Serialize, Deserialize)]
pub struct FileInformation {
    pub file_name: String,
    #[serde(rename = "SPDXID")]
    pub file_spdx_identifier: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_type: Option<Vec<FileType>>,
    pub file_checksum: Vec<Checksum>,
    /// Store Fossology's license conclusion. Need a way to parse Fossology's
    /// output for policy engine.
    pub concluded_license: SPDXExpression,
    /// Store Fossology's scan results.
    pub license_information_in_file: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comments_on_license: Option<String>,
    pub copyright_text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_notice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_contributor: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_attribution_text: Option<Vec<String>>,
    // TODO: Snippet Information.
}

impl Default for FileInformation {
    fn default() -> Self {
        Self {
            file_name: "NOASSERTION".to_string(),
            file_spdx_identifier: "NOASSERTION".to_string(),
            file_type: None,
            file_checksum: Vec::new(),
            concluded_license: SPDXExpression("NOASSERTION".to_string()),
            license_information_in_file: Vec::new(),
            comments_on_license: None,
            copyright_text: "NOASSERTION".to_string(),
            file_comment: None,
            file_notice: None,
            file_contributor: None,
            file_attribution_text: None,
        }
    }
}

impl FileInformation {
    /// Create new file.
    pub fn new(name: &str, id: &mut i32) -> Self {
        *id += 1;
        Self {
            file_name: name.to_string(),
            file_spdx_identifier: format!("SPDXRef-{}", id),
            ..Default::default()
        }
    }
}