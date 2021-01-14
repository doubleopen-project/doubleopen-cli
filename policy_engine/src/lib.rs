// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use policy::Policy;

use spdx::{FileInformation, PackageInformation, SPDX};

use self::{evaluation_result::EvaluationResult, policy_violation::PolicyViolation};

pub mod evaluation_result;
pub mod license;
pub mod policy;
mod policy_file;
pub mod policy_violation;
pub mod resolution;

/// Policy Engine is used to evaluate license conclusions of files against a provided policy.
pub struct PolicyEngine {
    /// The Policy used for evaluation.
    _policy: Policy,

    /// List of allowed licenses in correct format for evaluation.
    allowed_licenses: HashMap<String, bool>,
}

impl PolicyEngine {
    /// Create new Engine based on Policy. Creates the required HashMap for evaluation
    pub fn new(policy: Policy) -> Self {
        let mut allowed_licenses: HashMap<String, bool> = HashMap::new();

        policy.licenses_allow.iter().for_each(|license| {
            allowed_licenses.insert(license.spdx_expression.to_string(), true);
        });

        Self {
            _policy: policy,
            allowed_licenses,
        }
    }

    /// Evaluate the whole SPDX against the policy in the Engine.
    pub fn evaluate_spdx(&self, spdx: &SPDX) -> EvaluationResult {
        let mut violations: Vec<PolicyViolation> = Vec::new();

        for package in spdx.package_information.iter() {
            for file in package
                .find_files_for_package(&spdx.file_information)
                .iter()
            {
                if let Some(violation) = self.evaluate_file(file, &package) {
                    violations.push(violation)
                }
            }
        }

        EvaluationResult {
            policy_violations: violations,
        }
    }

    /// Evaluate a single file against the policy in the Engine.
    pub fn evaluate_file(
        &self,
        file_information: &FileInformation,
        package_information: &PackageInformation,
    ) -> Option<PolicyViolation> {
        let expression = file_information.concluded_license.parse().unwrap();
        match expression.evaluate(&self.allowed_licenses) {
            false => {
                let violation = PolicyViolation {
                    file_id: file_information.file_spdx_identifier.to_string(),
                    file_name: file_information.file_name.to_string(),
                    package_id: package_information.package_spdx_identifier.to_string(),
                    package_name: package_information.package_name.to_string(),
                    file_license: file_information.concluded_license.0.to_string(),
                };
                Some(violation)
            }
            true => None,
        }
    }
}

#[cfg(test)]
mod test_policy_engine {
    use std::collections::HashMap;

    use spdx::{FileInformation, PackageInformation, SPDXExpression};

    use super::{
        license::License, policy::Policy, policy_violation::PolicyViolation, PolicyEngine,
    };

    #[test]
    fn create_engine() {
        let allowed_licenses: Vec<License> = vec![License {
            spdx_expression: "MIT".into(),
            message: Some("Allowed license.".into()),
        }];

        let policy = Policy {
            licenses_allow: allowed_licenses,
            licenses_deny: vec![],
            resolutions: vec![],
        };

        let engine = PolicyEngine::new(policy.clone());

        let mut expected_hashmap: HashMap<String, bool> = HashMap::new();
        expected_hashmap.insert("MIT".into(), true);

        assert_eq!(engine.allowed_licenses, expected_hashmap);
        assert_eq!(engine._policy, policy);
    }

    #[test]
    fn evaluates_file_correctly() {
        let mut id = 1;
        let package = PackageInformation::new("test_package", &mut id);
        let mut file = FileInformation::new("test_file", &mut id);

        file.concluded_license = SPDXExpression("MIT".into());

        let allowed_licenses: Vec<License> = vec![License {
            spdx_expression: "MIT".into(),
            message: Some("Allowed license.".into()),
        }];

        let policy = Policy {
            licenses_allow: allowed_licenses,
            licenses_deny: vec![],
            resolutions: vec![],
        };

        let engine = PolicyEngine::new(policy);

        let result = engine.evaluate_file(&file, &package);
        assert!(result.is_none());

        file.concluded_license = SPDXExpression("MIT AND GPL-2.0-only".into());

        let result = engine.evaluate_file(&file, &package).unwrap();
        let expected_violation = PolicyViolation {
            file_id: "SPDXRef-3".into(),
            file_license: "MIT AND GPL-2.0-only".into(),
            file_name: "test_file".into(),
            package_id: "SPDXRef-2".into(),
            package_name: "test_package".into(),
        };

        assert_eq!(result, expected_violation);
    }
}
