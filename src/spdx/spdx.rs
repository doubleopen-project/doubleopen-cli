// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use crate::fossology::{
    fossology::Fossology,
    structs::{HashQueryInput, HashQueryResponse},
};
use flate2::read::GzDecoder;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::BufReader,
    path::Path,
};
use tar::Archive;
use uuid::Uuid;
use walkdir::WalkDir;

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

    /// Counter for creating SPDXRefs.
    #[serde(skip_serializing)]
    spdx_ref_counter: i32,
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
            spdx_ref_counter: 0,
        }
    }

    /// Deserialize from file. Accepts json and yaml.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let file = fs::File::open(&path).expect("SPDX file not found");
        let reader = BufReader::new(file);

        match path.extension().unwrap().to_str() {
            Some("yml") => {
                serde_yaml::from_reader::<_, SPDX>(reader).unwrap()
            }
            Some("json") => {
                serde_json::from_reader::<_, SPDX>(reader).unwrap()
            }
            None | Some(_) => panic!(),
        }
    }

    /// Create next SPDXRef
    pub fn spdx_ref(&mut self) -> String {
        self.spdx_ref_counter += 1;
        format!("SPDXRef-{}", self.spdx_ref_counter)
    }

    pub fn find_file_by_spdx_id<'a>(
        file_informations: &'a Vec<FileInformation>,
        spdx_id: &str,
    ) -> Option<&'a FileInformation> {
        file_informations
            .iter()
            .find(|file| file.file_spdx_identifier.to_lowercase() == spdx_id.to_lowercase())
    }

    /// Add package from source archive to SPDX.
    pub fn add_package_from_archive<P: AsRef<Path>>(&mut self, path_to_archive: P) {
        let path = path_to_archive.as_ref();

        // Create a temporary directory and unpack the archive there.
        let temp_dir = tempfile::tempdir().unwrap();
        let file = File::open(&path).unwrap();
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack(&temp_dir.path()).unwrap();

        // Create PackageInformation based on the archive data.
        let mut package = PackageInformation {
            package_file_name: Some(path.file_name().unwrap().to_str().unwrap().to_owned()),
            package_name: path.file_name().unwrap().to_str().unwrap().to_owned(),
            package_spdx_identifier: self.spdx_ref(),
            ..Default::default()
        };

        // Create FileInformation for all files in the source archive.
        let mut source_files: Vec<FileInformation> = WalkDir::new(&temp_dir)
            .into_iter()
            .filter_map(|f| {
                let entry = f.unwrap();
                if entry.metadata().unwrap().is_file() {
                    Some(FileInformation::try_from_direntry(entry, self))
                } else {
                    None
                }
            })
            .collect();

        // Add SPDX Identifiers of the source files to the package.
        package.files = source_files
            .iter()
            .map(|file| file.file_spdx_identifier.clone())
            .collect();

        self.package_information.push(package);
        self.file_information.append(&mut source_files);
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
                            file_information.concluded_license =
                                SPDXExpression(findings.conclusion.join(" "));
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn adds_correct_package_metadata_from_archive() {
        let mut test_pkgdata_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_pkgdata_path.push("tests/examples/yocto/build/downloads/dbus-1.12.16.tar.gz");

        let mut spdx = SPDX::new("dbus");

        spdx.add_package_from_archive(&test_pkgdata_path.to_str().unwrap());

        let first_package = spdx.package_information.iter().next().clone().unwrap();

        assert_eq!(
            first_package.package_file_name,
            Some("dbus-1.12.16.tar.gz".into())
        );
        assert_eq!(first_package.package_name, "dbus-1.12.16.tar.gz");
        assert_eq!(first_package.package_spdx_identifier, "SPDXRef-1")
    }

    #[test]
    fn find_correct_file_by_spdx_id() {
        let mut spdx = SPDX::new("test");
        spdx.file_information.push(FileInformation {
            file_spdx_identifier: "SPDXRef-1".into(),
            ..Default::default()
        });
        spdx.file_information.push(FileInformation {
            file_spdx_identifier: "SPDXRef-2".into(),
            file_name: "Find this".into(),
            ..Default::default()
        });
        spdx.file_information.push(FileInformation {
            file_spdx_identifier: "SPDXRef-3".into(),
            ..Default::default()
        });

        let found_file = SPDX::find_file_by_spdx_id(&spdx.file_information, "SPDXRef-2").unwrap();

        assert_eq!(found_file.file_name, "Find this")
    }

    #[test]
    fn test_spdx_ref_generation() {
        let mut spdx = SPDX::new("test");

        let id_first = spdx.spdx_ref();
        let id_second = spdx.spdx_ref();
        let id_third = spdx.spdx_ref();

        assert_eq!(id_first, "SPDXRef-1");
        assert_eq!(id_second, "SPDXRef-2");
        assert_eq!(id_third, "SPDXRef-3");
    }

    #[test]
    fn deserialize_from_file() {
       todo!() 
    }
}
