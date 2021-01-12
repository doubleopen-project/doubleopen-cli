use serde::{Deserialize, Serialize};

use super::{Checksum, FileType, SPDXExpression};

/// ## File Information
///
/// SPDX's [File Information](https://spdx.github.io/spdx-spec/4-file-information/)
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInformation {
    /// https://spdx.github.io/spdx-spec/4-file-information/#41-file-name
    pub file_name: String,

    /// https://spdx.github.io/spdx-spec/4-file-information/#42-file-spdx-identifier
    #[serde(rename = "SPDXID")]
    pub file_spdx_identifier: String,

    /// https://spdx.github.io/spdx-spec/4-file-information/#43-file-type
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_type: Option<Vec<FileType>>,

    /// https://spdx.github.io/spdx-spec/4-file-information/#44-file-checksum
    pub file_checksum: Vec<Checksum>,

    /// https://spdx.github.io/spdx-spec/4-file-information/#45-concluded-license
    pub concluded_license: SPDXExpression,

    /// https://spdx.github.io/spdx-spec/4-file-information/#46-license-information-in-file
    pub license_information_in_file: Vec<String>,

    /// https://spdx.github.io/spdx-spec/4-file-information/#47-comments-on-license
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comments_on_license: Option<String>,

    /// https://spdx.github.io/spdx-spec/4-file-information/#48-copyright-text
    pub copyright_text: String,

    /// https://spdx.github.io/spdx-spec/4-file-information/#412-file-comment
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_comment: Option<String>,

    /// https://spdx.github.io/spdx-spec/4-file-information/#413-file-notice
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_notice: Option<String>,

    /// https://spdx.github.io/spdx-spec/4-file-information/#414-file-contributor
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file_contributor: Option<Vec<String>>,

    /// https://spdx.github.io/spdx-spec/4-file-information/#415-file-attribution-text
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
