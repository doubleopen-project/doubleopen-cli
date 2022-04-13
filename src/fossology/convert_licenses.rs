// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

use spdx_rs::models::{SimpleExpression, SpdxExpression};
use spdx_toolkit::license_list::LicenseList;

use super::doubleopen_licenses::{gpl_or_later_conversion, is_do_license};

/// Convert scanner hits from Fossology to vec of SPDX expressions.
pub fn license_information_to_spdx_expressions(
    license_information: &[String],
    license_list: &LicenseList,
) -> anyhow::Result<Vec<SimpleExpression>> {
    license_information
        .iter()
        .cloned()
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
        .map(|lic| SimpleExpression::parse(&lic).map_err(anyhow::Error::new))
        .collect::<anyhow::Result<Vec<SimpleExpression>>>()
}

/// Sanitize string to conform to SPDX license expression spec.
fn sanitize_spdx_expression(lic: String) -> String {
    lic.replace(&['(', ')', '[', ']'][..], "")
}

/// Update [`SpdxExpression`] to be valid SPDX license.
pub fn update_license_to_valid_spdx(
    license: &SpdxExpression,
    license_list: &LicenseList,
) -> anyhow::Result<SpdxExpression> {
    let mut lic = license.to_string();
    let identifiers = license.identifiers();

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

    Ok(SpdxExpression::parse(&lic)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_licenseref_to_valid_spdx() {
        let original = SpdxExpression::parse("CLOSED").unwrap();

        let license_list = LicenseList::from_github().unwrap();
        let actual = update_license_to_valid_spdx(&original, &license_list).unwrap();

        let expected = SpdxExpression::parse("LicenseRef-CLOSED").unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn do_not_update_valid_spdx() {
        let original = SpdxExpression::parse("MIT").unwrap();

        let license_list = LicenseList::from_github().unwrap();
        let actual = update_license_to_valid_spdx(&original, &license_list).unwrap();

        let expected = SpdxExpression::parse("MIT").unwrap();

        assert_eq!(actual, expected);
    }
}
