use std::path::Path;

use regex::Regex;
use spdx_rs::{license_list::LicenseList, PackageInformation, SPDXExpression};

// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

/// Parse list of Double Open's license conclusions from Fossology to an SPDX expression.
pub fn parse_doubleopen_license(licenses: Vec<String>) -> String {
    let mut or_operator_list: Vec<String> = Vec::new();
    let mut other_licenses_list: Vec<String> = Vec::new();

    for license in licenses {
        #[allow(clippy::branches_sharing_code)]
        if is_do_license(&license) && is_or_license(&license) {
            let license = dolicense_to_spdx(license);
            or_operator_list.push(license);
        } else {
            let license = dolicense_to_spdx(license);
            other_licenses_list.push(license);
        }
    }

    let or_licenses = or_operator_list.join(" ");
    let other_licenses = other_licenses_list.join(" AND ");
    if or_licenses.is_empty() {
        other_licenses
    } else {
        format!("{} {}", or_licenses, other_licenses)
    }
}

/// Convert Double Open's custom Fossology license to SPDX expression.
pub(crate) fn dolicense_to_spdx(license: String) -> String {
    if is_do_license(&license) {
        // Remove prefix.
        let license = license.strip_prefix("DOLicense-").expect("Always exists.");

        // Process parentheses.
        let license = license.replace("paro-", "(");
        let license = license.replace("-parc", ")");

        // Process -OR- and -AND-.
        let license = license.replace("-OR-", " OR ").replace("-AND-", " AND ");

        // Process -OR license.
        let license = if is_or_license(&license) {
            let license = license.strip_suffix("-OR").expect("Always exists.");
            format!("{} OR", license)
        } else {
            license
        };

        // Process DO Exceptions.
        let license = if is_do_exception_license(&license) {
            let license = license
                .strip_prefix("SPDXException-")
                .expect("Always exists.")
                .to_string();
            license.replace("-with-", " WITH ")
        } else {
            license
        };

        gpl_or_later_conversion(license)
    } else {
        license
    }
}

/// Convert declared license from meta-doubleopen to SPDX expression.
pub(crate) fn yocto_license_to_spdx(
    license: &SPDXExpression,
    license_list: &LicenseList,
) -> SPDXExpression {
    let inner = license.0.to_string();

    let inner = inner
        .replace(" & ", " AND ")
        .replace(" | ", " OR ")
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("_", "-");

    let inner = inner
        .split_ascii_whitespace()
        .map(|l| convert_special_yocto_licenses(l.to_string()))
        .map(|l| {
            if l == "AND"
                || l == "OR"
                || l == "NOASSERTION"
                || l == "NONE"
                || l == "("
                || l == ")"
                || license_list.includes_license(&l.replace("+", ""))
            {
                l
            } else {
                format!("LicenseRef-{}", l)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    SPDXExpression(inner)
}

fn convert_special_yocto_licenses(input: String) -> String {
    match input.as_str() {
        "AGPL-3" => "AGPL-3.0-only".to_string(),
        "AGPL-3+" => "AGPL-3.0-or-later".to_string(),
        "AGPLv3" => "AGPL-3.0-only".to_string(),
        "AGPLv3+" => "AGPL-3.0-or-later".to_string(),
        "AGPLv3.0" => "AGPL-3.0-only".to_string(),
        "AGPLv3.0+" => "AGPL-3.0-or-later".to_string(),
        "AGPL-3.0" => "AGPL-3.0-only".to_string(),
        "AGPL-3.0+" => "AGPL-3.0-or-later".to_string(),
        "BSD-0-Clause" => "0BSD".to_string(),
        "GPL-1" => "GPL-1.0-only".to_string(),
        "GPL-1+" => "GPL-1.0-or-later".to_string(),
        "GPLv1" => "GPL-1.0-only".to_string(),
        "GPLv1+" => "GPL-1.0-or-later".to_string(),
        "GPLv1.0" => "GPL-1.0-only".to_string(),
        "GPLv1.0+" => "GPL-1.0-or-later".to_string(),
        "GPL-1.0" => "GPL-1.0-only".to_string(),
        "GPL-1.0+" => "GPL-1.0-or-later".to_string(),
        "GPL-2" => "GPL-2.0-only".to_string(),
        "GPL-2+" => "GPL-2.0-or-later".to_string(),
        "GPLv2" => "GPL-2.0-only".to_string(),
        "GPLv2+" => "GPL-2.0-or-later".to_string(),
        "GPLv2.0" => "GPL-2.0-only".to_string(),
        "GPLv2.0+" => "GPL-2.0-or-later".to_string(),
        "GPL-2.0" => "GPL-2.0-only".to_string(),
        "GPL-2.0+" => "GPL-2.0-or-later".to_string(),
        "GPL-3" => "GPL-3.0-only".to_string(),
        "GPL-3+" => "GPL-3.0-or-later".to_string(),
        "GPLv3" => "GPL-3.0-only".to_string(),
        "GPLv3+" => "GPL-3.0-or-later".to_string(),
        "GPLv3.0" => "GPL-3.0-only".to_string(),
        "GPLv3.0+" => "GPL-3.0-or-later".to_string(),
        "GPL-3.0" => "GPL-3.0-only".to_string(),
        "GPL-3.0+" => "GPL-3.0-or-later".to_string(),
        "LGPLv2" => "LGPL-2.0-only".to_string(),
        "LGPLv2+" => "LGPL-2.0-or-later".to_string(),
        "LGPLv2.0" => "LGPL-2.0-only".to_string(),
        "LGPLv2.0+" => "LGPL-2.0-or-later".to_string(),
        "LGPL-2.0" => "LGPL-2.0-only".to_string(),
        "LGPL-2.0+" => "LGPL-2.0-or-later".to_string(),
        "LGPL2.1" => "LGPL-2.1-only".to_string(),
        "LGPL2.1+" => "LGPL-2.1-or-later".to_string(),
        "LGPLv2.1" => "LGPL-2.1-only".to_string(),
        "LGPLv2.1+" => "LGPL-2.1-or-later".to_string(),
        "LGPL-2.1" => "LGPL-2.1-only".to_string(),
        "LGPL-2.1+" => "LGPL-2.1-or-later".to_string(),
        "LGPLv3" => "LGPL-3.0-only".to_string(),
        "LGPLv3+" => "LGPL-3.0-or-later".to_string(),
        "LGPL-3.0" => "LGPL-3.0-only".to_string(),
        "LGPL-3.0+" => "LGPL-3.0-or-later".to_string(),
        "MPL-1" => "MPL-1.0".to_string(),
        "MPLv1" => "MPL-1.0".to_string(),
        "MPLv1.1" => "MPL-1.1".to_string(),
        "MPLv2" => "MPL-2.0".to_string(),
        "MIT-X" => "MIT".to_string(),
        "MIT-style" => "MIT".to_string(),
        "openssl" => "OpenSSL".to_string(),
        "PSF" => "PSF-2.0".to_string(),
        "PSFv2" => "PSF-2.0".to_string(),
        "Python-2" => "Python-2.0".to_string(),
        "Apachev2" => "Apache-2.0".to_string(),
        "Apache-2" => "Apache-2.0".to_string(),
        "Artisticv1" => "Artistic-1.0".to_string(),
        "Artistic-1" => "Artistic-1.0".to_string(),
        "AFL-2" => "AFL-2.0".to_string(),
        "AFL-1" => "AFL-1.2".to_string(),
        "AFLv2" => "AFL-2.0".to_string(),
        "AFLv1" => "AFL-1.2".to_string(),
        "CDDLv1" => "CDDL-1.0".to_string(),
        "CDDL-1" => "CDDL-1.0".to_string(),
        "EPLv1.0" => "EPL-1.0".to_string(),
        "FreeType" => "FTL".to_string(),
        "Nauman" => "Naumen".to_string(),
        "tcl" => "TCL".to_string(),
        "vim" => "Vim".to_string(),
        "SGIv1" => "SGI-1".to_string(),
        _ => input,
    }
}

/// Convert deprecated license ids.
pub fn gpl_or_later_conversion(license: String) -> String {
    license
        .replace("AGPL-1.0+", "AGPL-1.0-or-later")
        .replace("AGPL-3.0+", "AGPL-3.0-or-later")
        .replace("GFDL-1.1-invariants+", "GFDL-1.1-invariants-or-later")
        .replace("GFDL-1.1-no-invariants+", "GFDL-1.1-no-invariants-or-later")
        .replace("GFDL-1.1+", "GFDL-1-1-or-later")
        .replace("GFDL-1.2-invariants+", "GFDL-1.2-invariants-or-later")
        .replace("GFDL-1.2-no-invariants+", "GFDL-1.2-no-invariants-or-later")
        .replace("GFDL-1.2+", "GFDL-1-2-or-later")
        .replace("GFDL-1.3-invariants+", "GFDL-1.3-invariants-or-later")
        .replace("GFDL-1.3-no-invariants+", "GFDL-1.3-no-invariants-or-later")
        .replace("GFDL-1.3+", "GFDL-1-3-or-later")
        .replace("GPL-1.0+", "GPL-1.0-or-later")
        .replace("gpl-2.0+", "GPL-2.0-or-later")
        .replace("GPL-2.0+", "GPL-2.0-or-later")
        .replace("gpl-3.0+", "GPL-3.0-or-later")
        .replace("GPL-3.0+", "GPL-3.0-or-later")
        .replace("LGPL-2.0+", "LGPL-2.0-or-later")
        .replace("LGPL-2.1+", "LGPL-2.1-or-later")
        .replace("LGPL-3.0+", "LGPL-3.0-or-later")
}

/// Check if the string is Double Open's custom Fossology license.
pub fn is_do_license(license: &str) -> bool {
    license.starts_with("DOLicense-")
}

/// Check if the string is Double Open's OR license.
fn is_or_license(license: &str) -> bool {
    license.ends_with("-OR")
}

/// Check if the string is Double Open's license with SPDX exception.
fn is_do_exception_license(license: &str) -> bool {
    license.starts_with("SPDXException-")
}

/// Sanitize string to conform to SPDX license expression spec.
fn sanitize_spdx_expression(lic: String) -> String {
    lic.replace(&['(', ')', '[', ']'][..], "")
}

/// Convert Fossology's conclusions to SPDX Expression.
pub fn fossology_conclusions_to_spdx_expression(
    conclusions: Vec<String>,
    license_list: &LicenseList,
) -> SPDXExpression {
    // Convert all conclusions to be SPDX compliant.
    let conclusions: Vec<String> = conclusions
        .into_iter()
        .map(sanitize_spdx_expression)
        .map(gpl_or_later_conversion)
        .map(|lic| {
            if license_list.includes_license(&lic.replace("+", ""))
                || license_list.includes_exception(&lic)
                || is_do_license(&lic)
                || lic.starts_with("LicenseRef-")
                || lic == "Dual-license"
                || lic == "NOASSERTION"
                || lic == "NONE"
            {
                lic
            } else {
                format!("LicenseRef-{}", lic)
            }
        })
        .collect();

    // Join licenses with AND or OR.
    let expression = if (conclusions.len() == 2
        || (conclusions.len() == 3 && conclusions.contains(&"Dual-license".to_string())))
        && conclusions
            .iter()
            .any(|lic| license_list.includes_exception(lic))
    {
        let mut sorted_conclusions: Vec<String> = Vec::new();
        for lic in conclusions {
            if license_list.includes_exception(&lic) {
                sorted_conclusions.push(lic)
            } else {
                sorted_conclusions.insert(0, lic)
            }
        }
        filter_dual_license(sorted_conclusions).join(" WITH ")
    } else if conclusions.len() == 3 && conclusions.contains(&"Dual-license".to_string()) {
        let conclusions = filter_dual_license(conclusions);
        conclusions.join(" OR ")
    } else {
        let conclusions = filter_dual_license(conclusions);
        let conclusions = add_licenserefs(conclusions, license_list);
        parse_doubleopen_license(conclusions)
    };

    SPDXExpression(expression)
}

/// Filter Fossology's Dual-license from the list of licenses.
fn filter_dual_license(conclusions: Vec<String>) -> Vec<String> {
    conclusions
        .into_iter()
        .filter(|lic| lic != "Dual-license")
        .collect::<Vec<String>>()
}

/// Add SPDX's LicenseRef to a license if it's not on the SPDX license list.
fn add_licenserefs(conclusions: Vec<String>, license_list: &LicenseList) -> Vec<String> {
    conclusions
        .into_iter()
        .map(|lic| {
            if license_list.includes_license(&lic.replace("+", ""))
                || lic.starts_with("LicenseRef-")
                || is_do_license(&lic)
                || lic == "NOASSERTION"
                || lic == "NONE"
            {
                lic
            } else {
                format!("LicenseRef-{}", lic)
            }
        })
        .collect()
}

/// Check if the package contains a declared license with "CLOSED".
pub fn get_packages_with_closed_license(
    package_information: &[PackageInformation],
) -> Vec<&PackageInformation> {
    package_information
        .iter()
        .filter(|package| package.declared_license.0.contains("CLOSED"))
        .collect()
}

/// Check if the archive at path should not be uploaded.
pub fn skip_package_upload<P: AsRef<Path>>(
    archive_path: P,
    packages_to_skip: &[&PackageInformation],
) -> bool {
    let mut packages_to_skip_regex = packages_to_skip.iter().map(|package| {
        if let Some(version) = &package.package_version {
            Regex::new(&format!(
                "^{}.*{}.*",
                regex::escape(&package.package_name),
                regex::escape(version)
            ))
            .expect("Regex creation to succeed")
        } else {
            Regex::new(&format!("^{}.*", &package.package_name)).expect("Regex creation to succeed")
        }
    });

    packages_to_skip_regex.any(|package| {
        let file_name = archive_path.as_ref().file_name();

        if let Some(file_name) = file_name {
            let file_name = file_name.to_string_lossy();
            package.is_match(&file_name)
        } else {
            false
        }
    })
}

#[cfg(test)]
mod test_super {
    use spdx_rs::SPDX;

    use super::*;

    #[cfg(test)]
    mod conclusions_to_spdx_expression {
        use super::*;

        #[test]
        fn single_licenses_are_converted_correctly() {
            let input1 = vec!["MIT".to_string()];
            let input2 = vec!["CustomLicense".to_string()];
            let input3 = vec!["Autoconf-exception-2.0".to_string()];
            let input4 = vec!["NONE".to_string()];
            let input5 = vec!["NOASSERTION".to_string()];

            let license_list = LicenseList::from_github().unwrap();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);
            let result3 = fossology_conclusions_to_spdx_expression(input3, &license_list);
            let result4 = fossology_conclusions_to_spdx_expression(input4, &license_list);
            let result5 = fossology_conclusions_to_spdx_expression(input5, &license_list);

            assert_eq!(result1, SPDXExpression("MIT".to_string()));
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense".to_string())
            );
            assert_eq!(
                result3,
                SPDXExpression("LicenseRef-Autoconf-exception-2.0".to_string())
            );
            assert_eq!(result4, SPDXExpression("NONE".to_string()));
            assert_eq!(result5, SPDXExpression("NOASSERTION".to_string()));
        }

        #[test]
        fn simple_and_licenses_are_converted_correctly() {
            let input1 = vec!["MIT".to_string(), "Apache-2.0".to_string()];
            let input2 = vec!["CustomLicense".to_string(), "MIT".to_string()];
            let input3 = vec!["Autoconf-exception-2.0".to_string(), "MIT".to_string()];

            let license_list = LicenseList::from_github().unwrap();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);
            let result3 = fossology_conclusions_to_spdx_expression(input3, &license_list);

            assert_eq!(result1, SPDXExpression("MIT AND Apache-2.0".to_string()));
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense AND MIT".to_string())
            );
            assert_eq!(
                result3,
                SPDXExpression("MIT WITH Autoconf-exception-2.0".to_string())
            );
        }

        #[test]
        fn simple_or_licenses_are_converted_correctly() {
            let input1 = vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "Dual-license".to_string(),
            ];
            let input2 = vec![
                "CustomLicense".to_string(),
                "MIT".to_string(),
                "Dual-license".to_string(),
            ];
            let input3 = vec![
                "Autoconf-exception-2.0".to_string(),
                "MIT".to_string(),
                "Dual-license".to_string(),
            ];

            let license_list = LicenseList::from_github().unwrap();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);
            let result3 = fossology_conclusions_to_spdx_expression(input3, &license_list);

            assert_eq!(result1, SPDXExpression("MIT OR Apache-2.0".to_string()));
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense OR MIT".to_string())
            );
            assert_eq!(
                result3,
                SPDXExpression("MIT WITH Autoconf-exception-2.0".to_string())
            );
        }

        #[test]
        fn or_licenses_with_three_are_converted_to_and() {
            let input1 = vec![
                "MIT".to_string(),
                "Apache-2.0".to_string(),
                "ISC".to_string(),
                "Dual-license".to_string(),
            ];
            let input2 = vec![
                "CustomLicense".to_string(),
                "MIT".to_string(),
                "Dual-license".to_string(),
                "GPL-2.0-or-later".to_string(),
            ];

            let license_list = LicenseList::from_github().unwrap();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);

            assert_eq!(
                result1,
                SPDXExpression("MIT AND Apache-2.0 AND ISC".to_string())
            );
            assert_eq!(
                result2,
                SPDXExpression("LicenseRef-CustomLicense AND MIT AND GPL-2.0-or-later".to_string())
            );
        }

        #[test]
        fn doubleopen_license_is_converted_correctly() {
            let license_list = LicenseList::from_github().unwrap();

            let input_1 = vec![
                "DOLicense-LGPL-2.1-AND-Zlib-OR".to_string(),
                "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception".to_string(),
                "MIT".to_string(),
                "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR".to_string(),
            ];
            let expected_1 = SPDXExpression("LGPL-2.1 AND Zlib OR BSD-3-Clause AND GPL-2.0 OR GPL-2.0-or-later WITH Autoconf-exception AND MIT".to_string());
            assert_eq!(
                fossology_conclusions_to_spdx_expression(input_1, &license_list),
                expected_1
            );

            let input_2 = vec![
                "DOLicense-LGPL-2.1-OR".to_string(),
                "BSD-3-Clause".to_string(),
                "MIT".to_string(),
            ];
            let expected_2 = SPDXExpression("LGPL-2.1 OR BSD-3-Clause AND MIT".to_string());
            assert_eq!(
                fossology_conclusions_to_spdx_expression(input_2, &license_list),
                expected_2
            );

            let input_3 = vec![
                "DOLicense-paro-LGPL-2.1-OR-BSD-3-Clause-parc".to_string(),
                "MIT".to_string(),
            ];
            let expected_3 = SPDXExpression("(LGPL-2.1 OR BSD-3-Clause) AND MIT".to_string());
            assert_eq!(
                fossology_conclusions_to_spdx_expression(input_3, &license_list),
                expected_3
            );
        }

        #[test]
        fn with_licenses_are_converted_correctly() {
            let input1 = vec!["Bison-exception-2.2".to_string(), "GPL-3.0+".to_string()];
            let input2 = vec!["GPL-3.0+".to_string(), "Bison-exception-2.2".to_string()];

            let license_list = LicenseList::from_github().unwrap();

            let result1 = fossology_conclusions_to_spdx_expression(input1, &license_list);
            let result2 = fossology_conclusions_to_spdx_expression(input2, &license_list);

            assert_eq!(
                result1,
                SPDXExpression("GPL-3.0-or-later WITH Bison-exception-2.2".to_string())
            );
            assert_eq!(
                result2,
                SPDXExpression("GPL-3.0-or-later WITH Bison-exception-2.2".to_string())
            );
        }
    }

    #[test]
    fn is_do_license_works() {
        let input_1 = "DOLicense-LGPL-2.1-AND-Zlib-OR";
        let input_2 = "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception";
        let input_3 = "MIT";
        let input_4 = "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR";

        assert!(is_do_license(input_1));
        assert!(is_do_license(input_2));
        assert!(!is_do_license(input_3));
        assert!(is_do_license(input_4));
    }

    #[test]
    fn is_or_license_works() {
        let input_1 = "DOLicense-LGPL-2.1-AND-Zlib-OR";
        let input_2 = "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception";
        let input_3 = "MIT";
        let input_4 = "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR";

        assert!(is_or_license(input_1));
        assert!(!is_or_license(input_2));
        assert!(!is_or_license(input_3));
        assert!(is_or_license(input_4));
    }

    #[test]
    fn format_dolicense_to_spdx_works() {
        let input_1 = "DOLicense-paro-LGPL-2.1-OR-BSD-3-Clause-parc";
        let expected_1 = "(LGPL-2.1 OR BSD-3-Clause)";

        let input_2 = "DOLicense-LGPL-2.1-AND-Zlib-OR";
        let expected_2 = "LGPL-2.1 AND Zlib OR";

        let input_3 = "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception";
        let expected_3 = "GPL-2.0-or-later WITH Autoconf-exception";

        let input_4 = "DOLicense-LGPL-2.1-OR";
        let expected_4 = "LGPL-2.1 OR";

        assert_eq!(dolicense_to_spdx(input_1.into()), expected_1.to_string());
        assert_eq!(dolicense_to_spdx(input_2.into()), expected_2.to_string());
        assert_eq!(dolicense_to_spdx(input_3.into()), expected_3.to_string());
        assert_eq!(dolicense_to_spdx(input_4.into()), expected_4.to_string());
    }

    #[test]
    fn parse_doubleopen_license_works() {
        let input_1 = vec![
            "DOLicense-LGPL-2.1-AND-Zlib-OR".to_string(),
            "DOLicense-SPDXException-GPL-2.0+-with-Autoconf-exception".to_string(),
            "MIT".to_string(),
            "DOLicense-BSD-3-Clause-AND-GPL-2.0-OR".to_string(),
        ];
        let expected_1 = "LGPL-2.1 AND Zlib OR BSD-3-Clause AND GPL-2.0 OR GPL-2.0-or-later WITH Autoconf-exception AND MIT".to_string();
        assert_eq!(parse_doubleopen_license(input_1), expected_1);

        let input_2 = vec![
            "DOLicense-LGPL-2.1-OR".to_string(),
            "BSD-3-Clause".to_string(),
            "MIT".to_string(),
        ];
        let expected_2 = "LGPL-2.1 OR BSD-3-Clause AND MIT".to_string();
        assert_eq!(parse_doubleopen_license(input_2), expected_2);

        let input_3 = vec![
            "DOLicense-paro-LGPL-2.1-OR-BSD-3-Clause-parc".to_string(),
            "MIT".to_string(),
        ];
        let expected_3 = "(LGPL-2.1 OR BSD-3-Clause) AND MIT".to_string();
        assert_eq!(parse_doubleopen_license(input_3), expected_3);
    }

    #[test]
    fn get_packages_with_closed_source() {
        let mut spdx = SPDX::new("test_spdx");
        let mut packages: Vec<PackageInformation> = vec![
            PackageInformation {
                package_name: "nginx".to_string(),
                package_version: Some("1.16.1".to_string()),
                declared_license: SPDXExpression("MIT".to_string()),
                ..Default::default()
            },
            PackageInformation {
                package_name: "tzdata".to_string(),
                package_version: Some("2021a".to_string()),
                declared_license: SPDXExpression("CLOSED".to_string()),
                ..Default::default()
            },
            PackageInformation {
                package_name: "systemd".to_string(),
                package_version: Some("1_244.5".to_string()),
                declared_license: SPDXExpression("MIT AND CLOSED AND Apache-2.0".to_string()),
                ..Default::default()
            },
        ];

        spdx.package_information.append(&mut packages);

        let closed_packages: Vec<&PackageInformation> =
            get_packages_with_closed_license(&spdx.package_information);

        assert_eq!(closed_packages.len(), 2);
        assert!(closed_packages
            .iter()
            .any(|package| package.package_name == "tzdata"));
        assert!(closed_packages
            .iter()
            .any(|package| package.package_name == "systemd"));
        assert!(!closed_packages
            .iter()
            .any(|package| package.package_name == "nginx"));
    }

    #[test]
    fn skip_uploading_correct_packages() {
        let mut spdx = SPDX::new("test_spdx");
        let mut packages: Vec<PackageInformation> = vec![
            PackageInformation {
                package_name: "nginx".to_string(),
                package_version: Some("1.16.1".to_string()),
                declared_license: SPDXExpression("MIT".to_string()),
                ..Default::default()
            },
            PackageInformation {
                package_name: "tzdata".to_string(),
                package_version: Some("2021a".to_string()),
                declared_license: SPDXExpression("CLOSED".to_string()),
                ..Default::default()
            },
            PackageInformation {
                package_name: "systemd".to_string(),
                package_version: Some("1_244.5".to_string()),
                declared_license: SPDXExpression("MIT AND CLOSED AND Apache-2.0".to_string()),
                ..Default::default()
            },
            PackageInformation {
                package_name: "git_package".to_string(),
                package_version: Some("gitAUTOINC+123".to_string()),
                declared_license: SPDXExpression("MIT AND CLOSED AND Apache-2.0".to_string()),
                ..Default::default()
            },
        ];

        spdx.package_information.append(&mut packages);

        let closed_packages: Vec<&PackageInformation> =
            get_packages_with_closed_license(&spdx.package_information);

        let nginx_path = Path::new("nginx-1.16.1-40.tar.bz2");
        let systemd_path = Path::new("systemd-1_244.5-r0.tar");
        let tzdata_path = Path::new("tzdata-2021a-r0.tar.bz2");
        let git_package_path = Path::new("git_package-gitAUTOINC+123-r0.tar.bz2");

        assert!(!skip_package_upload(&nginx_path, &closed_packages));
        assert!(skip_package_upload(&systemd_path, &closed_packages));
        assert!(skip_package_upload(&tzdata_path, &closed_packages));
        assert!(skip_package_upload(&git_package_path, &closed_packages));
    }

    #[test]
    fn license_from_yocto_simple_spdx() {
        let license_list = LicenseList::from_github().unwrap();
        let input = SPDXExpression("Zlib".into());
        let converted = yocto_license_to_spdx(&input, &license_list);
        assert_eq!(converted, SPDXExpression("Zlib".into()));
    }

    #[test]
    fn license_from_yocto_gpl() {
        let license_list = LicenseList::from_github().unwrap();
        let input = SPDXExpression("GPLv2".into());
        let converted = yocto_license_to_spdx(&input, &license_list);
        assert_eq!(converted, SPDXExpression("GPL-2.0-only".into()));
    }

    #[test]
    fn license_from_yocto_closed() {
        let license_list = LicenseList::from_github().unwrap();
        let input = SPDXExpression("CLOSED".into());
        let converted = yocto_license_to_spdx(&input, &license_list);
        assert_eq!(converted, SPDXExpression("LicenseRef-CLOSED".into()));
    }

    #[test]
    fn license_from_yocto_dual_gpl() {
        let license_list = LicenseList::from_github().unwrap();
        let input = SPDXExpression("GPLv2 & GPLv2+".into());
        let converted = yocto_license_to_spdx(&input, &license_list);
        assert_eq!(
            converted,
            SPDXExpression("GPL-2.0-only AND GPL-2.0-or-later".into())
        );
    }
}
