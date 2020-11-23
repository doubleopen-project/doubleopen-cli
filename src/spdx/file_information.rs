use serde::{Deserialize, Serialize};

use super::{Checksum, FileType, SPDX, SPDXExpression};

/// ## File Information
///
/// SPDX's [File Information](https://spdx.github.io/spdx-spec/4-file-information/)
#[derive(Serialize, Deserialize)]
pub struct FileInformation {
    pub file_name: String,
    #[serde(rename = "SPDXID")]
    pub file_spdx_identifier: String,
    pub file_type: Option<Vec<FileType>>,
    pub file_checksum: Vec<Checksum>,
    /// Store Fossology's license conclusion. Need a way to parse Fossology's
    /// output for policy engine.
    pub concluded_license: SPDXExpression,
    /// Store Fossology's scan results.
    pub license_information_in_file: Vec<String>,
    pub comments_on_license: Option<String>,
    pub copyright_text: String,
    pub file_comment: Option<String>,
    pub file_notice: Option<String>,
    pub file_contributor: Option<Vec<String>>,
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

    /// Create FileInformation fron a walkdir direntry.
    pub fn try_from_direntry(direntry: walkdir::DirEntry, spdx: &mut SPDX) -> FileInformation {
        let sha256 = Checksum::try_sha256_from_path(direntry.path());
        let spdx_id = spdx.spdx_ref();

        FileInformation {
            file_name: direntry.file_name().to_str().unwrap().to_string(),
            file_checksum: vec![sha256],
            file_spdx_identifier: spdx_id,
            ..Default::default()
        }
    }
}
