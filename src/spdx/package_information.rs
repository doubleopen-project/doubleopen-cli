use serde::{Deserialize, Serialize};

use super::{Checksum, FileInformation, PackageVerificationCode};

/// ## Package Information
///
/// SPDX's [Package Information](https://spdx.github.io/spdx-spec/3-package-information/).
#[derive(Serialize, Deserialize)]
pub struct PackageInformation {
    pub package_name: String,
    pub package_spdx_identifier: String,
    pub package_version: Option<String>,
    pub package_file_name: Option<String>,
    pub package_supplier: Option<String>,
    pub package_originator: Option<String>,
    pub package_download_location: String,
    pub files_analyzed: Option<bool>,
    pub package_verification_code: Option<PackageVerificationCode>,
    pub package_checksum: Option<Vec<Checksum>>,
    pub package_home_page: Option<String>,
    pub source_information: Option<String>,
    pub concluded_license: String,
    pub all_licenses_information_from_files: Option<Vec<String>>,
    pub declared_license: String,
    pub comments_on_license: Option<String>,
    pub copyright_text: String,
    pub package_summary_description: Option<String>,
    pub package_detailed_description: Option<String>,
    pub package_comment: Option<String>,
    // TODO: Create Struct if needed.
    pub external_reference: Option<String>,
    // Should probably be included in ExternalReference struct.
    pub external_reference_comment: Option<String>,
    pub package_attribution_text: Option<Vec<String>>,
    // Valid SPDX?
    pub file_information: Vec<FileInformation>,
}

impl Default for PackageInformation {
    fn default() -> Self {
        Self {
            package_name: "NOASSERTION".to_string(),
            package_spdx_identifier: "SPDXRef-1".to_string(),
            package_version: None,
            package_file_name: None,
            package_supplier: None,
            package_originator: None,
            package_download_location: "NOASSERTION".to_string(),
            files_analyzed: None,
            package_verification_code: None,
            package_checksum: None,
            package_home_page: None,
            source_information: None,
            concluded_license: "NOASSERTION".to_string(),
            all_licenses_information_from_files: None,
            declared_license: "NOASSERTION".to_string(),
            comments_on_license: None,
            copyright_text: "NOASSERTION".to_string(),
            package_summary_description: None,
            package_detailed_description: None,
            package_comment: None,
            external_reference: None,
            external_reference_comment: None,
            package_attribution_text: None,
            file_information: Vec::new(),
        }
    }
}

impl PackageInformation {
    /// Create new package.
    pub fn new(name: &str, id: &mut i32) -> Self {
        *id += 1;
        Self {
            package_name: name.to_string(),
            package_spdx_identifier: format!("SPDXRef-{}", id),
            ..Default::default()
        }
    }
}
