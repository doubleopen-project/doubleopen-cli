// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use fossology_rs::{license::get_license, upload::FilesearchResponse, Fossology};
use log::{debug, info};
use spdx_rs::models::{
    Algorithm, Checksum, FileInformation, OtherLicensingInformationDetected, RelationshipType,
    SpdxExpression, SPDX,
};
use spdx_toolkit::license_list::LicenseList;

use crate::fossology::{
    convert_licenses::update_license_to_valid_spdx,
    doubleopen_licenses::get_packages_with_closed_license,
    queries::filesearch_for_file_information,
};

use super::{
    convert_licenses::license_information_to_spdx_expressions,
    doubleopen_licenses::fossology_conclusions_to_spdx_expression,
};

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
        package.declared_license =
            update_license_to_valid_spdx(&package.declared_license, license_list)?;
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

    let files = &spdx
        .file_information
        .iter()
        .filter(|&file| match file.checksum(Algorithm::SHA256) {
            Some(checksum) => !closed_file_hashes.contains(checksum),
            None => true,
        })
        .cloned()
        .collect::<Vec<_>>();

    debug!("Filtered source files contained by CLOSED-recipes.");

    let responses = filesearch_for_file_information(files, fossology)?;

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

                add_file_response(file_information, response, license_list);
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
fn add_file_response(
    file: &mut FileInformation,
    response: &FilesearchResponse,
    license_list: &LicenseList,
) {
    // Add MD5 to the file in SPDX.
    if let Some(md5) = &response.hash.md5 {
        if file.checksum(Algorithm::MD5).is_none() {
            file.file_checksum.push(Checksum::new(Algorithm::MD5, md5))
        }
    }

    // Add SHA1 to the file in SPDX.
    if let Some(sha1) = &response.hash.sha1 {
        if file.checksum(Algorithm::SHA1).is_none() {
            file.file_checksum
                .push(Checksum::new(Algorithm::SHA1, sha1))
        }
    }

    // Add license findings to the file in SPDX.
    if let Some(findings) = &response.findings {
        // If scanner result is No_license_found and conlcusion is NOASSERTION
        // conclude as NONE.
        file.license_information_in_file =
            license_information_to_spdx_expressions(findings.scanner.clone(), license_list);

        if findings.scanner.len() == 1
            && findings.scanner.contains(&"No_license_found".to_string())
            && findings.conclusion.len() == 1
            && findings.conclusion.contains(&"NOASSERTION".to_string())
        {
            file.concluded_license = SpdxExpression::parse("NONE").expect("Should never fail");
        } else if !findings.conclusion.is_empty() {
            // TODO: Transform Fossology output to SPDX expression.
            file.concluded_license =
                fossology_conclusions_to_spdx_expression(findings.conclusion.clone(), license_list);
        }

        if !findings.copyright.is_empty() {
            file.copyright_text = findings.copyright.join("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::utilities::deserialize_spdx;

    use super::*;

    #[test]
    fn correctly_process_fossology_responses() {
        let mut spdx = deserialize_spdx("tests/data/fossology/test_spdx.json").unwrap();
        let license_list = LicenseList::from_github().unwrap();

        let mut response1: Vec<FilesearchResponse> =
            serde_json::from_str(&read_to_string("tests/data/fossology/response1.json").unwrap())
                .unwrap();

        let response2: Vec<FilesearchResponse> =
            serde_json::from_str(&read_to_string("tests/data/fossology/response2.json").unwrap())
                .unwrap();

        response1.extend(response2);

        process_fossology_responses(&mut spdx, response1, &license_list);

        let expected = deserialize_spdx("tests/data/fossology/expected.json").unwrap();

        assert_eq!(spdx, expected);
    }
}
