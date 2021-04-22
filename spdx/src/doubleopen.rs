pub fn parse_doubleopen_license(licenses: Vec<String>) -> String {
    let mut or_operator_list: Vec<String> = Vec::new();
    let mut other_licenses_list: Vec<String> = Vec::new();

    for license in licenses {
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

fn dolicense_to_spdx(license: String) -> String {
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

pub fn gpl_or_later_conversion(license: String) -> String {
    license
        .replace("GPL-2.0+", "GPL-2.0-or-later")
        .replace("GPL-3.0+", "GPL-3.0-or-later")
}

pub fn is_do_license(license: &str) -> bool {
    license.starts_with("DOLicense-")
}

fn is_or_license(license: &str) -> bool {
    license.ends_with("-OR")
}

fn is_do_exception_license(license: &str) -> bool {
    license.starts_with("SPDXException-")
}

#[cfg(test)]
mod test_super {
    use super::*;

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
        let expected_3 = "GPL-2.0+ WITH Autoconf-exception";

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
        let expected_1 = "LGPL-2.1 AND Zlib OR BSD-3-Clause AND GPL-2.0 OR GPL-2.0+ WITH Autoconf-exception AND MIT".to_string();
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
}
