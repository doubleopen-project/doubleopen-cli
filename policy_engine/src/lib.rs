// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use policy::Policy;

use spdx::{FileInformation, PackageInformation, SPDX};

use self::{evaluation_result::EvaluationResult, policy_violation::PolicyViolation};

pub mod evaluation_result;
pub mod policy;
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
        let allowed_licenses: HashMap<String, bool> = HashMap::new();

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
