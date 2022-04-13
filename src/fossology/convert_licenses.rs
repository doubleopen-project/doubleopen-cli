// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

use spdx_toolkit::license_list::LicenseList;

use super::doubleopen_licenses::gpl_or_later_conversion;

/// Convert scanner hits from Fossology to vec of SPDX expressions.
pub fn license_information_to_spdx_expressions(
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
