// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Struct for a license policy to be used with the Policy Engine.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Policy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    license_rules: Vec<LicenseRule>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    package_rules: Vec<PackageRule>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LicenseRule {
    #[serde(flatten)]
    pub target: TargetLicense,

    pub license_source: LicenseSource,

    pub policy_rule: PolicyRule,
}

impl LicenseRule {
    pub fn new(
        target: TargetLicense,
        license_source: LicenseSource,
        policy_rule: PolicyRule,
    ) -> Self {
        Self {
            target,
            license_source,
            policy_rule,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TargetLicense {
    #[serde(rename = "grouping")]
    Grouping(String),
    SPDXId(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PackageRule {
    pub package_name: String,

    pub package_version: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,

    pub policy_rule: PolicyRule,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_licenses: Vec<TargetLicense>,
}

impl PackageRule {
    pub fn new(
        package_name: &str,
        package_version: &str,
        files: Option<String>,
        policy_rule: PolicyRule,
        target_licenses: Vec<TargetLicense>,
    ) -> Self {
        Self {
            package_name: package_name.to_string(),
            package_version: package_version.to_string(),
            files,
            policy_rule,
            target_licenses,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum LicenseSource {
    Concluded,
    Scanner,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PolicyRule {
    Allowed,
    Denied,
}

impl Policy {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            license_rules: Vec::new(),
            package_rules: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn create_example_policy() {
        let mut example_policy = Policy::new();
        example_policy.license_rules.push(LicenseRule::new(
            TargetLicense::Grouping("Permissive".into()),
            LicenseSource::Concluded,
            PolicyRule::Allowed,
        ));
        example_policy.license_rules.push(LicenseRule::new(
            TargetLicense::SPDXId("MPL-2.0".into()),
            LicenseSource::Concluded,
            PolicyRule::Allowed,
        ));
        example_policy.package_rules.push(PackageRule::new(
            "bash",
            "1.22",
            None,
            PolicyRule::Allowed,
            vec![
                TargetLicense::Grouping("Copyleft".into()),
                TargetLicense::Grouping("Copyleft Limited".into()),
                TargetLicense::Grouping("Permissive".into()),
            ],
        ));
        example_policy.package_rules.push(PackageRule::new(
            "bash",
            "1.*",
            None,
            PolicyRule::Allowed,
            vec![
                TargetLicense::Grouping("Copyleft".into()),
                TargetLicense::SPDXId("GPL-2.0-or-later".into()),
            ],
        ));
        example_policy.package_rules.push(PackageRule::new(
            "bash",
            "1.22",
            Some("build/*.tar".into()),
            PolicyRule::Denied,
            vec![],
        ));

        let json = serde_json::to_string_pretty(&example_policy).unwrap();
        fs::write("../tests/examples/policy_engine/example_policy.json", json)
            .expect("Unable to write file");
        let yaml = serde_yaml::to_string(&example_policy).unwrap();
        fs::write("../tests/examples/policy_engine/example_policy.yml", yaml)
            .expect("Unable to write file");

        let json_string =
            fs::read_to_string("../tests/examples/policy_engine/example_policy.json").unwrap();
        let de_json: Policy = serde_json::from_str(&json_string).unwrap();
        let yaml_string =
            fs::read_to_string("../tests/examples/policy_engine/example_policy.yml").unwrap();
        let de_yaml: Policy = serde_yaml::from_str(&yaml_string).unwrap();

        assert_eq!(de_json, example_policy);
        assert_eq!(de_yaml, example_policy);
    }
}
