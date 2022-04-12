// SPDX-FileCopyrightText: 2020 HH Par hash: (), findings: (), uploads: (), message: ()  hash: (), findings: (), uploads: (), message: () tners
//
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use doubleopen::gpl_or_later_conversion;
use fossology_rs::{
    license::get_license,
    upload::{filesearch, FilesearchResponse, Hash},
    Fossology,
};
use log::{debug, info};
use spdx_rs::models::{
    Algorithm, Checksum, OtherLicensingInformationDetected, RelationshipType, SpdxExpression, SPDX,
};
use spdx_toolkit::license_list::LicenseList;

use crate::doubleopen::{
    fossology_conclusions_to_spdx_expression, get_packages_with_closed_license, is_do_license,
};

pub mod commands;
mod doubleopen;
mod utilities;

/// Get scanner results and license conclusions for the files in SPDX
/// found on the Fossology instance.
pub fn populate_spdx_document_from_fossology(
    fossology: &Fossology,
    spdx: &mut spdx_rs::models::SPDX,
    license_list: &LicenseList,
) -> Result<(), anyhow::Error> {
    info!("Populating SPDX from Fossology.");

    // Update declared licenses to valid SPDX.
    for package in &mut spdx.package_information {
        let mut lic = package.declared_license.to_string();
        let identifiers = package.declared_license.identifiers();

        for identifier in identifiers {
            if license_list.includes_license(&identifier.replace('+', ""))
                || license_list.includes_exception(&identifier)
                || is_do_license(&identifier)
                || identifier.starts_with("LicenseRef-")
                || identifier == "Dual-license"
                || identifier == "NOASSERTION"
                || identifier == "NONE"
            {
                continue;
            } else {
                lic = lic.replace(&identifier, &format!("LicenseRef-{}", identifier));
            }
        }

        package.declared_license = SpdxExpression::parse(&lic)?;
    }

    let sha256_values = spdx.get_unique_hashes(Algorithm::SHA256);
    debug!(
        "Extracted {} unique hashes from the SPDX Document.",
        &sha256_values.len()
    );

    let closed_recipes = get_packages_with_closed_license(&spdx.package_information);

    let mut closed_file_hashes: HashSet<String> = HashSet::new();

    for recipe in closed_recipes {
        let recipe_contained_files = spdx.get_files_for_package(&recipe.package_spdx_identifier);
        let recipe_contained_hashes =
            recipe_contained_files
                .iter()
                .filter_map(|&(file, relationship)| {
                    if relationship.relationship_type == RelationshipType::Contains {
                        Some(file.checksum(Algorithm::SHA256))
                    } else {
                        None
                    }
                });

        for hash in recipe_contained_hashes.flatten() {
            closed_file_hashes.insert(hash.to_string());
        }
    }

    let sha256_values = sha256_values
        .iter()
        .filter(|&sha256| !closed_file_hashes.contains(sha256))
        .collect::<Vec<_>>();

    debug!("Filtered source files contained by CLOSED-recipes.");

    debug!(
        "Query fossology with {} unique hashes.",
        &sha256_values.len()
    );
    // Create input for the Fossology query.
    let input: Vec<Hash> = sha256_values
        .iter()
        .map(|hash| Hash {
            sha256: Some(hash.to_string()),
            ..Default::default()
        })
        .collect();
    let mut responses = Vec::new();

    const CHUNK_SIZE: usize = 2000;

    for (i, batch) in input.chunks(CHUNK_SIZE).enumerate() {
        info!(
            "Querying {} / {}.",
            ((i + 1) * CHUNK_SIZE).min(input.len()),
            input.len()
        );
        responses.extend(filesearch(fossology, batch, None)?);
    }

    process_fossology_responses(spdx, responses, license_list);
    add_license_texts_to_spdx(spdx, license_list, fossology);
    Ok(())
}

fn process_fossology_responses(
    spdx: &mut SPDX,
    mut responses: Vec<FilesearchResponse>,
    license_list: &LicenseList,
) {
    info!("Processing Fossology response");

    // Sort response by sha256 to enable binary search.
    responses.sort_unstable_by_key(|i| i.hash.sha256.clone().unwrap().to_uppercase());

    // Loop over all the files in all packages.
    for file_information in &mut spdx.file_information {
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
                    if file_information.checksum(Algorithm::MD5).is_none() {
                        file_information
                            .file_checksum
                            .push(Checksum::new(Algorithm::MD5, md5))
                    }
                }

                // Add SHA1 to the file in SPDX.
                if let Some(sha1) = &response.hash.sha1 {
                    if file_information.checksum(Algorithm::SHA1).is_none() {
                        file_information
                            .file_checksum
                            .push(Checksum::new(Algorithm::SHA1, sha1))
                    }
                }

                // Add license findings to the file in SPDX.
                if let Some(findings) = &response.findings {
                    // If scanner result is No_license_found and conlcusion is NOASSERTION
                    // conclude as NONE.
                    file_information.license_information_in_file =
                        license_information_to_spdx_expressions(
                            findings.scanner.clone(),
                            license_list,
                        );

                    if findings.scanner.len() == 1
                        && findings.scanner.contains(&"No_license_found".to_string())
                        && findings.conclusion.len() == 1
                        && findings.conclusion.contains(&"NOASSERTION".to_string())
                    {
                        file_information.concluded_license =
                            SpdxExpression::parse("NONE").expect("Should never fail");
                    } else if !findings.conclusion.is_empty() {
                        // TODO: Transform Fossology output to SPDX expression.
                        file_information.concluded_license =
                            fossology_conclusions_to_spdx_expression(
                                findings.conclusion.clone(),
                                license_list,
                            );
                    }

                    if !findings.copyright.is_empty() {
                        file_information.copyright_text = findings.copyright.join("\n");
                    }
                }
            }
        }
    }
}

fn add_license_texts_to_spdx(spdx: &mut SPDX, license_list: &LicenseList, fossology: &Fossology) {
    // Add license texts to SPDX for licenses not on the SPDX license list.
    let licenses = spdx.get_license_ids();

    for license in licenses {
        if !license_list.includes_license(&license.replace('+', "")) {
            let spdx_license = spdx
                .other_licensing_information_detected
                .iter()
                .find(|&lic| lic.license_identifier == license);

            match spdx_license {
                Some(_) => {}
                None => {
                    let license_data = get_license(fossology, &license, None);
                    match license_data {
                        Ok(license_data) => {
                            let license_text = if !license_data.text.is_empty() {
                                license_data.text
                            } else {
                                "NOASSERTION".into()
                            };
                            spdx.other_licensing_information_detected.push(
                                OtherLicensingInformationDetected {
                                    license_identifier: license,
                                    extracted_text: license_text,
                                    license_name: license_data.full_name.to_string(),
                                    license_cross_reference: Vec::new(),
                                    license_comment: None,
                                },
                            )
                        }
                        Err(_) => spdx.other_licensing_information_detected.push(
                            OtherLicensingInformationDetected {
                                license_identifier: license,
                                extracted_text: "NOASSERTION".into(),
                                license_name: "NOASSERTION".into(),
                                license_cross_reference: Vec::new(),
                                license_comment: None,
                            },
                        ),
                    }
                }
            }
        }
    }
}

/// Convert scanner hits from Fossology to vec of SPDX expressions.
fn license_information_to_spdx_expressions(
    license_information: Vec<String>,
    license_list: &LicenseList,
) -> Vec<String> {
    license_information
        .into_iter()
        .filter(|lic| !lic.starts_with("DOLicense"))
        // Remove No_license_found
        .filter(|lic| lic != "No_license_found")
        // Remove Dual-license
        .filter(|lic| lic != "Dual-license")
        .map(gpl_or_later_conversion)
        // Sanitize characters
        .map(sanitize_spdx_expression)
        // Remove `+` signs as they're not currently used, and are problematic in later steps.
        .map(|lic| lic.replace('+', ""))
        // Add LicenseRefs
        .map(|lic| {
            if license_list.includes_license(&lic.replace('+', ""))
                || lic.starts_with("LicenseRef-")
            {
                lic
            } else {
                format!("LicenseRef-{}", lic)
            }
        })
        .collect()
}

/// Sanitize string to conform to SPDX license expression spec.
fn sanitize_spdx_expression(lic: String) -> String {
    lic.replace(&['(', ')', '[', ']'][..], "")
}

#[cfg(test)]
mod test_super {
    use fossology_rs::upload::Findings;
    use spdx_rs::models::FileInformation;

    use super::*;

    #[test]
    fn correctly_process_fossology_responses() {
        let mut spdx = SPDX::new("Test SPDX");
        let license_list = LicenseList::from_github().unwrap();

        spdx.file_information.push(FileInformation {
            file_name: "test_file_1".into(),
            file_spdx_identifier: "SPDXRef-File-1".into(),
            file_checksum: vec![Checksum {
                algorithm: Algorithm::SHA256,
                value: "checksum1".into(),
            }],
            ..Default::default()
        });

        spdx.file_information.push(FileInformation {
            file_name: "test_file_2".into(),
            file_spdx_identifier: "SPDXRef-File-2".into(),
            file_checksum: vec![Checksum {
                algorithm: Algorithm::SHA256,
                value: "checksum2".into(),
            }],
            ..Default::default()
        });

        spdx.file_information.push(FileInformation {
            file_name: "test_file_3".into(),
            file_spdx_identifier: "SPDXRef-File-3".into(),
            file_checksum: vec![Checksum {
                algorithm: Algorithm::SHA256,
                value: "checksum3".into(),
            }],
            ..Default::default()
        });

        let response_1 = FilesearchResponse {
            hash: Hash {
                sha1: Some("sha1".into()),
                md5: Some("md5".into()),
                sha256: Some("checksum1".into()),
                size: Some(100),
            },
            findings: Some(Findings {
                scanner: vec!["MIT".into(), "GPL-2.0-only".into()],
                conclusion: vec!["MIT".into(), "GPL-2.0-only".into()],
                copyright: vec!["test copyright 1".into()],
            }),
            uploads: Vec::new(),
            message: None,
        };

        let response_2 = FilesearchResponse {
            hash: Hash {
                sha1: Some("sha1".into()),
                md5: Some("md5".into()),
                sha256: Some("checksum2".into()),
                size: Some(100),
            },
            findings: Some(Findings {
                scanner: vec!["MIT".into(), "ISC".into(), "Dual-license".into()],
                conclusion: vec!["MIT".into(), "ISC".into(), "Dual-license".into()],
                copyright: vec!["test copyright 2".into()],
            }),
            uploads: Vec::new(),
            message: None,
        };

        let response_3 = FilesearchResponse {
            hash: Hash {
                sha1: Some("sha1".into()),
                md5: Some("md5".into()),
                sha256: Some("checksum3".into()),
                size: Some(100),
            },
            findings: Some(Findings {
                scanner: vec![
                    "DOLicense-SPDXException-GPL-2.0-or-later-with-Autoconf-exception-2.0".into(),
                    "GPL-2.0-or-later".into(),
                ],
                conclusion: vec![
                    "DOLicense-SPDXException-GPL-2.0-or-later-with-Autoconf-exception-2.0".into(),
                ],
                copyright: vec!["test copyright 3".into()],
            }),
            uploads: Vec::new(),
            message: None,
        };

        let fossology_responses: Vec<FilesearchResponse> = vec![response_1, response_2, response_3];

        process_fossology_responses(&mut spdx, fossology_responses, &license_list);

        let file_1 = spdx
            .file_information
            .iter()
            .find(|file| file.file_spdx_identifier == "SPDXRef-File-1")
            .unwrap();

        let file_2 = spdx
            .file_information
            .iter()
            .find(|file| file.file_spdx_identifier == "SPDXRef-File-2")
            .unwrap();

        let file_3 = spdx
            .file_information
            .iter()
            .find(|file| file.file_spdx_identifier == "SPDXRef-File-3")
            .unwrap();

        assert_eq!(
            file_1.license_information_in_file,
            vec!["MIT", "GPL-2.0-only"]
        );
        assert_eq!(
            file_1.concluded_license,
            SpdxExpression::parse("MIT AND GPL-2.0-only").unwrap()
        );
        assert_eq!(file_2.license_information_in_file, vec!["MIT", "ISC"]);
        assert_eq!(
            file_2.concluded_license,
            SpdxExpression::parse("MIT OR ISC").unwrap()
        );
        assert_eq!(
            file_3.license_information_in_file,
            vec!["GPL-2.0-or-later".to_string()]
        );
        assert_eq!(
            file_3.concluded_license,
            SpdxExpression::parse("GPL-2.0-or-later WITH Autoconf-exception-2.0").unwrap()
        );
    }
}
