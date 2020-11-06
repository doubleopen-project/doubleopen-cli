// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use crate::fossology::{
    fossology::Fossology,
    structs::{HashQueryInput, HashQueryResponse},
};
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

pub use super::*;

// TODO: Annotations.
/// # SPDX 2.2
///
/// Store information about files in SPDX files. Latest spec
/// is currently 2.2. Can be serialized to JSON.  
///     
/// Spec: https://spdx.github.io/spdx-spec/
#[derive(Serialize, Deserialize)]
pub struct SPDX {
    #[serde(flatten)]
    pub document_creation_information: DocumentCreationInformation,
    // Valid SPDX?
    #[serde(rename = "packages")]
    pub package_information: Vec<PackageInformation>,
    #[serde(rename = "hasExtractedLicensingInfos")]
    pub other_licensing_information_detected: Vec<OtherLicensingInformationDetected>,
    #[serde(rename = "files")]
    pub file_information: Vec<FileInformation>,
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
        }
    }

    /// Get unique hashes for all files in all packages of the SPDX.
    pub fn get_unique_hashes(&self) -> Vec<String> {
        let mut unique_hashes: Vec<String> = Vec::new();

        for file_information in self.file_information.iter() {
            if let Some(checksum) = file_information
                .file_checksum
                .iter()
                .find(|checksum| checksum.algorithm == Algorithm::SHA256)
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
        let pb = ProgressBar::new(self.file_information.len() as u64);

        // Sort response by sha256 to enable binary search.
        responses.sort_by_key(|i| i.hash.sha256.clone().unwrap());

        // Loop over all the files in all packages.
        for file_information in &mut self.file_information {
            pb.inc(1);
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

        pb.finish();
    }

    /// Save serialized SPDX as json,
    pub fn save_as_json(&self, path: &str) {
        println!("Saving to json...");
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, json).expect("Unable to write file");
    }
}
