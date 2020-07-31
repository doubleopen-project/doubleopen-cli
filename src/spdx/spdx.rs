// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use crate::{
    fossology::{
        fossology::Fossology,
        structs::{HashQueryInput, HashQueryResponse},
    },
    yocto::Package,
};
use chrono::{DateTime, Utc};
use fs::DirEntry;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

// TODO: Annotations.
/// # SPDX 2.2
///
/// Store information about files in SPDX files. Latest spec
/// is currently 2.2. Can be serialized to JSON.  
///     
/// Spec: https://spdx.github.io/spdx-spec/
#[derive(Serialize, Deserialize)]
pub struct SPDX {
    pub document_creation_information: DocumentCreationInformation,
    // Valid SPDX?
    pub package_information: Vec<PackageInformation>,
    pub other_licensing_information_detected: Vec<OtherLicensingInformationDetected>,
}

/// ## Document Creation Information
///
/// SPDX's [Document Creation Information](https://spdx.github.io/spdx-spec/2-document-creation-information/)
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

/// ## File Information
///
/// SPDX's [File Information](https://spdx.github.io/spdx-spec/4-file-information/)
#[derive(Serialize, Deserialize)]
pub struct FileInformation {
    pub file_name: String,
    pub file_spdx_identifier: String,
    pub file_type: Option<Vec<String>>,
    pub file_checksum: Vec<Checksum>,
    /// Store Fossology's license conclusion. Need a way to parse Fossology's
    /// output for policy engine.
    pub concluded_license: String,
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

#[derive(Serialize, Deserialize, PartialEq)]
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
            // Current version is 2.2. Might need to support more verisons
            // in the future.
            spdx_version: "SPDX-2.2".to_string(),
            data_license: "CC0-1.0".to_string(),
            spdx_identifier: "SPDXRef-DOCUMENT".to_string(),
            document_name: "NOASSERTION".to_string(),
            spdx_document_namespace: "NOASSERTION".to_string(),
            external_document_references: None,
            license_list_version: None,
            // TODO: Get tool name and version automatically.
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
    /// Create new SPDX struct.
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

    /// Get unique hashes for all files in all packages of the SPDX.
    pub fn get_unique_hashes(&self) -> Vec<String> {
        let mut unique_hashes: Vec<String> = Vec::new();

        for package_information in self.package_information.iter() {
            for file_information in package_information.file_information.iter() {
                if let Some(checksum) = file_information
                    .file_checksum
                    .iter()
                    .find(|checksum| checksum.algorithm == Algorithm::SHA256)
                {
                    unique_hashes.push(checksum.value.clone());
                }
            }
        }

        unique_hashes.sort();
        unique_hashes.dedup();

        unique_hashes
    }

    /// Get scanner results and license conclusions for the files in SPDX
    /// found on the Fossology instance.
    pub fn query_fossology_for_licenses(&mut self, fossology: &Fossology) {
        let hashes = self.get_unique_hashes();

        // Create input for the Fossology query.
        let input: Vec<HashQueryInput> = hashes
            .iter()
            .map(|hash| HashQueryInput {
                sha256: hash.to_string(),
            })
            .collect();

        let mut response = fossology.licenses_for_hashes(&input);

        self.process_fossology_response(&mut response);
    }

    /// Add information from Fossology response to the SPDX.
    pub fn process_fossology_response(&mut self, responses: &mut Vec<HashQueryResponse>) {
        println!("Processing Fossology response");
        let pb = ProgressBar::new(self.package_information.len() as u64);

        // Sort response by sha256 to enable binary search.
        responses.sort_by_key(|i| i.hash.sha256.clone().unwrap());

        // Loop over all the files in all packages.
        for package_information in &mut self.package_information {
            pb.inc(1);
            for file_information in &mut package_information.file_information {
                // Get sha256 of the file.
                if let Some(sha256) = file_information
                    .file_checksum
                    .iter()
                    .find(|checksum| checksum.algorithm == Algorithm::SHA256)
                {
                    // Find the corresponding item in response.
                    if let Ok(response) = responses
                        .binary_search_by_key(&sha256.value.to_uppercase(), |i| {
                            i.hash.sha256.clone().unwrap().to_uppercase()
                        })
                    {
                        let response = &responses[response];

                        // Add MD5 to the file in SPDX.
                        if let Some(md5) = &response.hash.md5 {
                            file_information.file_checksum.push(Checksum {
                                algorithm: Algorithm::MD5,
                                value: md5.to_string(),
                            })
                        }

                        // Add SHA1 to the file in SPDX.
                        if let Some(sha1) = &response.hash.sha1 {
                            file_information.file_checksum.push(Checksum {
                                algorithm: Algorithm::SHA1,
                                value: sha1.to_string(),
                            })
                        }

                        // Add license findings to the file in SPDX.
                        if let Some(findings) = &response.findings {
                            file_information.license_information_in_file = findings.scanner.clone();

                            if findings.conclusion.len() > 0 {
                                file_information.concluded_license = findings.conclusion.join(" ");
                            }
                        }
                    }
                }
            }
        }

        pb.finish();
    }

    /// Save serialized SPDX as json,
    pub fn save_as_json(&self, path: &str) {
        println!("Saving to json...");
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, json).expect("Unable to write file");
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

    /// Create SPDX packages from Yocto packages.
    // TODO: Should probably parse srclists first, then check
    // manifest for which packages are not used.
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

    pub fn new_from_pkglist(pkglist: &DirEntry, id: &mut i32) -> PackageInformation {
        *id += 1;
        let file_content = fs::read_to_string(pkglist.path()).unwrap();
        PackageInformation {
            package_name: pkglist
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            package_spdx_identifier: format!("SPDXRef-{}", id),
            package_comment: Some(file_content),
            ..Default::default()
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

impl Checksum {
    /// Create new checksum.
    pub fn new(algorithm: Algorithm, value: String) -> Self {
        Self { algorithm, value }
    }
}
