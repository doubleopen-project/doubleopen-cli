// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use crate::yocto::Package;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Annotations.
#[derive(Serialize, Deserialize)]
pub struct SPDX {
    pub document_creation_information: DocumentCreationInformation,
    // Valid SPDX?
    pub package_information: Vec<PackageInformation>,
    pub other_licensing_information_detected: Vec<OtherLicensingInformationDetected>,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentCreationInformation {
    pub spdx_version: String,
    pub data_license: String,
    pub spdx_identifier: String,
    pub document_name: String,
    pub spdx_document_namespace: String,
    pub external_document_references: Option<Vec<ExternalDocumentReference>>,
    pub license_list_version: Option<String>,
    pub creator: String,
    pub created: DateTime<Utc>,
    pub creator_comment: Option<String>,
    pub document_comment: Option<String>,
}

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

#[derive(Serialize, Deserialize)]
pub struct FileInformation {
    pub file_name: String,
    pub file_spdx_identifier: String,
    pub file_type: Option<Vec<String>>,
    pub file_checksum: Vec<Checksum>,
    pub concluded_license: String,
    pub license_information_in_file: Vec<String>,
    pub comments_on_license: Option<String>,
    pub copyright_text: String,
    pub file_comment: Option<String>,
    pub file_notice: Option<String>,
    pub file_contributor: Option<Vec<String>>,
    pub file_attribution_text: Option<Vec<String>>,
    // TODO: Snippet Information.
}

#[derive(Serialize, Deserialize)]
pub struct OtherLicensingInformationDetected {
    pub license_identifier: String,
    pub extracted_text: String,
    pub license_name: String,
    pub license_cross_reference: Option<Vec<String>>,
    pub license_comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageVerificationCode {
    pub value: String,
    pub excludes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ExternalDocumentReference {
    pub id_string: String,
    pub spdx_document_uri: String,
    pub checksum: Checksum,
}

#[derive(Serialize, Deserialize)]
pub struct Checksum {
    pub algorithm: Algorithm,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub enum Algorithm {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    MD2,
    MD4,
    MD5,
    MD6,
}

impl Default for DocumentCreationInformation {
    fn default() -> Self {
        Self {
            spdx_version: "SPDX-2.2".to_string(),
            data_license: "CC0-1.0".to_string(),
            spdx_identifier: "SPDXRef-DOCUMENT".to_string(),
            document_name: "NOASSERTION".to_string(),
            spdx_document_namespace: "NOASSERTION".to_string(),
            external_document_references: None,
            license_list_version: None,
            creator:
                "Person: Jane Doe () Organization: ExampleCodeInspect () Tool: LicenseFind-1.0"
                    .to_string(),
            created: chrono::offset::Utc::now(),
            creator_comment: None,
            document_comment: None,
        }
    }
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

impl Default for FileInformation {
    fn default() -> Self {
        Self {
            file_name: "NOASSERTION".to_string(),
            file_spdx_identifier: "NOASSERTION".to_string(),
            file_type: None,
            file_checksum: Vec::new(),
            concluded_license: "NOASSERTION".to_string(),
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

impl SPDX {
    pub fn new(name: &str) -> Self {
        Self {
            document_creation_information: DocumentCreationInformation {
                document_name: name.to_string(),
                spdx_document_namespace: format!(
                    "http://spdx.org/spdxdocs/{}-{}",
                    name.to_string(),
                    Uuid::new_v4()
                ),
                ..Default::default()
            },
            package_information: Vec::new(),
            other_licensing_information_detected: Vec::new(),
        }
    }
}

impl PackageInformation {
    pub fn new(name: &str, id: &mut i32) -> Self {
        *id += 1;
        Self {
            package_name: name.to_string(),
            package_spdx_identifier: format!("SPDXRef-{}", id),
            ..Default::default()
        }
    }

    pub fn from_yocto_packages(packages: &Vec<Package>) -> Vec<Self> {
        let mut package_informations: Vec<Self> = Vec::new();
        let mut package_names: Vec<String> = Vec::new();
        let mut package_id = 0;
        let mut file_id = 0;

        for package in packages.iter() {
            if let Some(package_list) = package.package_list.clone() {
                if !package_names.contains(&package_list.name) {
                    package_names.push(package_list.name.clone());
                    let mut file_informations: Vec<FileInformation> = Vec::new();
                    for elf_file in package_list.elf_files {
                        for source_file in elf_file.source_files {
                            let mut file_information =
                                FileInformation::new(&source_file.path, &mut file_id);
                            if let Some(sha256) = source_file.sha256 {
                                file_information.file_checksum.push(Checksum {
                                    algorithm: Algorithm::SHA256,
                                    value: sha256,
                                });
                            }
                            file_informations.push(file_information);
                        }
                    }
                    let mut package_information =
                        PackageInformation::new(&package_list.name, &mut package_id);
                    package_information.file_information = file_informations;
                    package_informations.push(package_information);
                }
            }
        }

        package_informations
    }
}

impl FileInformation {
    pub fn new(name: &str, id: &mut i32) -> Self {
        *id += 1;
        Self {
            file_name: name.to_string(),
            file_spdx_identifier: format!("SPDXRef-{}", id),
            ..Default::default()
        }
    }
}
