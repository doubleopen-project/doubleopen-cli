// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

pub mod algorithm;
pub mod checksum;
pub mod document_creation_information;
pub mod external_document_reference;
pub mod file_information;
pub mod file_type;
pub mod other_licensing_information_detected;
pub mod package_information;
pub mod package_verification_code;
pub mod relationship;
pub mod spdx_expression;
pub use algorithm::*;
pub use checksum::*;
pub use document_creation_information::*;
pub use external_document_reference::*;
pub use file_information::*;
pub use file_type::*;
use fossology::{
    api_objects::{requests::HashQueryInput, responses::HashQueryResponse},
    Fossology, FossologyError,
};
use log::info;
pub use other_licensing_information_detected::*;
pub use package_information::*;
pub use package_verification_code::*;
pub use relationship::*;
use serde::{Deserialize, Serialize};
pub use spdx_expression::*;
use std::{
    fs::{self},
    io::BufReader,
    path::Path,
};
use uuid::Uuid;

use self::Relationship;

// TODO: Annotations.
/// # SPDX 2.2
///
/// Store information about files in SPDX files. Latest spec
/// is currently 2.2. Can be serialized to JSON.  
///     
/// Spec: https://spdx.github.io/spdx-spec/
#[derive(Serialize, Deserialize, Debug)]
pub struct SPDX {
    /// https://spdx.github.io/spdx-spec/2-document-creation-information/
    #[serde(flatten)]
    pub document_creation_information: DocumentCreationInformation,

    // TODO: Valid SPDX?
    /// https://spdx.github.io/spdx-spec/3-package-information/
    #[serde(rename = "packages")]
    pub package_information: Vec<PackageInformation>,

    /// https://spdx.github.io/spdx-spec/6-other-licensing-information-detected/
    #[serde(rename = "hasExtractedLicensingInfos")]
    pub other_licensing_information_detected: Vec<OtherLicensingInformationDetected>,

    /// https://spdx.github.io/spdx-spec/4-file-information/
    #[serde(rename = "files")]
    pub file_information: Vec<FileInformation>,

    /// https://spdx.github.io/spdx-spec/7-relationships-between-SPDX-elements/
    pub relationships: Vec<Relationship>,

    /// Counter for creating SPDXRefs. Is not part of the spec, so don't serialize.
    #[serde(skip)]
    pub spdx_ref_counter: i32,
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
            file_information: Vec::new(),
            relationships: Vec::new(),
            spdx_ref_counter: 0,
        }
    }

    /// Deserialize from file. Accepts json and yaml.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        info!("Deserializing SPDX from {}", path.as_ref().display());
        let path = path.as_ref();
        let file = fs::File::open(&path).expect("SPDX file not found");
        let reader = BufReader::new(file);

        match path.extension().unwrap().to_str() {
            Some("yml") => serde_yaml::from_reader::<_, SPDX>(reader).unwrap(),
            Some("json") => serde_json::from_reader::<_, SPDX>(reader).unwrap(),
            None | Some(_) => panic!(),
        }
    }

    /// Get unique hashes for all files in all packages of the SPDX.
    pub fn get_unique_hashes(&self, algorithm: Algorithm) -> Vec<String> {
        let mut unique_hashes: Vec<String> = Vec::new();

        for file_information in self.file_information.iter() {
            if let Some(checksum) = file_information
                .file_checksum
                .iter()
                .find(|checksum| checksum.algorithm == algorithm)
            {
                unique_hashes.push(checksum.value.clone());
            }
        }

        unique_hashes.sort();
        unique_hashes.dedup();

        unique_hashes
    }

    /// Get scanner results and license conclusions for the files in SPDX
    /// found on the Fossology instance.
    pub fn query_fossology_for_licenses(
        &mut self,
        fossology: &Fossology,
    ) -> Result<(), FossologyError> {
        let sha256_values = self.get_unique_hashes(Algorithm::SHA256);

        // Create input for the Fossology query.
        let input: Vec<HashQueryInput> = sha256_values
            .iter()
            .map(|hash| HashQueryInput {
                sha256: Some(hash.to_string()),
                ..Default::default()
            })
            .collect();

        let response = fossology.licenses_for_hashes(&input)?;

        self.process_fossology_response(response);

        // Add license texts to SPDX for licenses not on the SPDX license list.
        let licenses = self.get_license_ids();

        for license in licenses {
            if !is_in_spdx_license_list(&license) {
                let spdx_license = self
                    .other_licensing_information_detected
                    .iter()
                    .find(|&lic| lic.license_identifier == license);

                match spdx_license {
                    Some(_) => {}
                    None => {
                        let license_data = fossology.license_by_short_name(&license)?;
                        self.other_licensing_information_detected.push(
                            OtherLicensingInformationDetected {
                                license_identifier: license,
                                extracted_text: license_data.text.to_string(),
                                license_name: license_data.full_name.to_string(),
                                license_cross_reference: None,
                                license_comment: None,
                            },
                        )
                    }
                }
            }
        }

        Ok(())
    }

    /// Add information from Fossology response to the SPDX.
    pub fn process_fossology_response(&mut self, mut responses: Vec<HashQueryResponse>) {
        info!("Processing Fossology response");

        // Sort response by sha256 to enable binary search.
        responses.sort_unstable_by_key(|i| i.hash.sha256.clone().unwrap().to_uppercase());

        // Loop over all the files in all packages.
        for file_information in &mut self.file_information {
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

                        if !findings.conclusion.is_empty() {
                            // TODO: Transform Fossology output to SPDX expression.
                            file_information.concluded_license =
                                spdx_expression_from_api_licenses(findings.conclusion.clone());
                        };

                        if !findings.copyright.is_empty() {
                            file_information.copyright_text = findings.copyright.join("\n");
                        }
                    }
                }
            }
        }
    }

    /// Save serialized SPDX as json,
    pub fn save_as_json<P: AsRef<Path>>(&self, path: P) {
        println!("Saving to json...");
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, json).expect("Unable to write file");
    }

    /// Find related files of the package with the provided id.
    pub fn get_files_for_package(
        &self,
        package_spdx_id: &str,
    ) -> Vec<(&FileInformation, &Relationship)> {
        let relationships = self
            .relationships
            .iter()
            .filter(|relationship| relationship.spdx_element_id == package_spdx_id);

        relationships
            .map(|relationship| {
                let file = self
                    .file_information
                    .iter()
                    .find(|file| file.file_spdx_identifier == relationship.related_spdx_element)
                    .expect("File should always exist.");
                (file, relationship)
            })
            .collect::<Vec<_>>()
    }

    /// Get all license identifiers from the SPDX.
    pub fn get_license_ids(&self) -> Vec<String> {
        let mut license_ids = Vec::new();

        for file in &self.file_information {
            for license in &file.concluded_license.licenses() {
                if !license_ids.contains(license) && license != "NOASSERTION" && license != "NONE" {
                    license_ids.push(license.clone());
                }
            }
        }

        license_ids
    }
}

/// Transform a list of licenses returned by Fossology to an SPDX license expression.
/// Fossology's Dual-license tag doesn't allow accurate representation of OR licenses
/// with more than two licenses, so all license combinations with 3 or more licenses
/// are interpreted as AND licenses.
pub fn spdx_expression_from_api_licenses(mut fossology_licenses: Vec<String>) -> SPDXExpression {
    if fossology_licenses.len() == 3 && fossology_licenses.contains(&"Dual-license".into()) {
        let dual_license_position = fossology_licenses
            .iter()
            .position(|lic| lic == "Dual-license")
            .expect("Should always exist here");

        fossology_licenses.remove(dual_license_position);
        let expression = fossology_licenses.join(" OR ");
        SPDXExpression(expression)
    } else {
        let expression = fossology_licenses
            .iter()
            .filter(|&lic| lic != &"Dual-license".to_string())
            .cloned()
            .collect::<Vec<_>>()
            .join(" AND ");
        SPDXExpression(expression)
    }
}

/// Test if license is in the SPDX license list.
pub fn is_in_spdx_license_list(spdx_id: &str) -> bool {
    let url = format!(
        "https://raw.githubusercontent.com/spdx/license-list-data/master/text/{}.txt",
        spdx_id
    );
    let body = reqwest::blocking::get(&url).unwrap().text().unwrap();
    body != "404: Not Found"
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn deserialize_simple_spdx() {
        let spdx_file = SPDX::from_file("../tests/examples/spdx/simple.spdx.json");
        assert_eq!(
            spdx_file.document_creation_information.document_name,
            "test_package_document".to_string()
        );
    }

    #[test]
    fn find_related_files_for_package() {
        let spdx_file = SPDX::from_file("../tests/examples/spdx/simple.spdx.json");

        let package_1_files = spdx_file.get_files_for_package("SPDXRef-1");
        let package_2_files = spdx_file.get_files_for_package("SPDXRef-2");

        assert_eq!(package_1_files.len(), 2);
        assert_eq!(package_2_files.len(), 3);

        let file = package_1_files
            .iter()
            .find(|package_and_relationship| package_and_relationship.0.file_name == *"file2.txt")
            .expect("Should always be found");

        assert_eq!(file.0.file_spdx_identifier, "SPDXRef-4");
        assert_eq!(file.1.relationship_type, RelationshipType::Contains);

        let file = package_2_files
            .iter()
            .find(|package_and_relationship| package_and_relationship.0.file_name == *"file5.txt")
            .expect("Should always be found");

        assert_eq!(
            file.0.concluded_license,
            SPDXExpression("GPL-2.0+ AND BSD-3-Clause".into())
        );
    }

    #[test]
    fn test_spdx_expression_from_fossology() {
        let input_1 = vec![
            "MIT".to_string(),
            "Dual-license".to_string(),
            "ISC".to_string(),
        ];

        let expected_1 = SPDXExpression("MIT OR ISC".into());

        assert_eq!(expected_1, spdx_expression_from_api_licenses(input_1));

        let input_2 = vec!["MIT".to_string(), "ISC".to_string()];

        let expected_2 = SPDXExpression("MIT AND ISC".into());

        assert_eq!(expected_2, spdx_expression_from_api_licenses(input_2));

        let input_3 = vec![
            "MIT".to_string(),
            "Dual-license".to_string(),
            "ISC".to_string(),
            "GPL-2.0-only".to_string(),
        ];

        let expected_3 = SPDXExpression("MIT AND ISC AND GPL-2.0-only".into());

        assert_eq!(expected_3, spdx_expression_from_api_licenses(input_3));
    }

    #[test]
    fn get_all_licenses_from_spdx() {
        let spdx_file = SPDX::from_file("../tests/examples/spdx/simple.spdx.json");

        let mut actual = spdx_file.get_license_ids();
        actual.sort();

        let mut expected: Vec<String> =
            vec!["LGPL-2.1+".into(), "GPL-2.0+".into(), "BSD-3-Clause".into()];
        expected.sort();

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_if_license_is_in_spdx_list() {
        let not_listed_1 = is_in_spdx_license_list("GPL-2.0+");
        let not_listed_2 = is_in_spdx_license_list("DOESNOT");
        let listed_1 = is_in_spdx_license_list("MIT");
        let listed_2 = is_in_spdx_license_list("GPL-2.0-or-later");

        assert_eq!(not_listed_1, false);
        assert_eq!(not_listed_2, false);
        assert_eq!(listed_1, true);
        assert_eq!(listed_2, true);
    }
}
