use serde::{Deserialize, Serialize};

use super::{Checksum, FileInformation, PackageVerificationCode, SPDXExpression};

/// ## Package Information
///
/// SPDX's [Package Information](https://spdx.github.io/spdx-spec/3-package-information/).
#[derive(Serialize, Deserialize)]
pub struct PackageInformation {
    pub package_name: String,
    pub package_spdx_identifier: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_supplier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_originator: Option<String>,
    pub package_download_location: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub files_analyzed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_verification_code: Option<PackageVerificationCode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_checksum: Option<Vec<Checksum>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_home_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source_information: Option<String>,
    pub concluded_license: SPDXExpression,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub all_licenses_information_from_files: Option<Vec<String>>,
    pub declared_license: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comments_on_license: Option<String>,
    pub copyright_text: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_summary_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_detailed_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_comment: Option<String>,
    // TODO: Create Struct if needed.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub external_reference: Option<String>,
    // Should probably be included in ExternalReference struct.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub external_reference_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub package_attribution_text: Option<Vec<String>>,
    // Valid SPDX?
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub files: Vec<String>,
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
            concluded_license: SPDXExpression("NOASSERTION".to_string()),
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
            files: Vec::new(),
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

    /// Find all files of the package.
    pub fn find_files_for_package<'a>(
        &'a self,
        files: &'a [FileInformation],
    ) -> Vec<&'a FileInformation> {
        self.files
            .iter()
            .map(|file| {
                files
                    .iter()
                    .find(|file_information| &file_information.file_spdx_identifier == file)
                    // Unwrap, the file should always exist in files.
                    // TODO: Proper error handling.
                    .unwrap()
            })
            .collect()
    }
}
