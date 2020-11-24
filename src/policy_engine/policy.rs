use std::collections::HashMap;

use crate::spdx::{FileInformation, PackageInformation, SPDX};

use super::{evaluation_result::EvaluationResult, license::License, policy_violation::PolicyViolation};
use serde::{Deserialize, Serialize};

/// Struct for a license policy to be used with the Policy Engine.
#[derive(Serialize, Deserialize)]
pub struct Policy {
    /// Allowlisted licenses.
    pub licenses_allow: Vec<License>,

    /// Denylisted licenses.
    pub licenses_deny: Vec<License>,
}

impl Policy {
    pub fn evaluate_spdx(&self, spdx: &SPDX) -> EvaluationResult {
        let mut allowed_licenses: HashMap<String, bool> = HashMap::new();
        self.licenses_allow.iter().for_each(|license| {
            allowed_licenses.insert(license.spdx_expression.to_string(), true);
        });

        let mut violations: Vec<PolicyViolation> = Vec::new();
        
        for package in spdx.package_information.iter() {
            for file in package.find_files_for_package(&spdx.file_information).iter() {
                match Policy::evaluate_file(&allowed_licenses, file, &package) {
                    Some(violation) => { violations.push(violation)}
                    None => {}
                }
            }
        }
        todo!()
    }

    pub fn evaluate_file(allowed_licenses: &HashMap<String, bool>, file_information: &FileInformation, package_information: &PackageInformation) -> Option<PolicyViolation> {
        let expression = file_information.concluded_license.parse().unwrap();
        match expression.evaluate(allowed_licenses) {
            false => {
                let violation = PolicyViolation {
                    file_id: file_information.file_spdx_identifier.to_string(),
                    file_name: file_information.file_name.to_string(),
                    package_id: package_information.package_spdx_identifier.to_string(),
                    package_name: package_information.package_name.to_string(),
                    file_license: file_information.concluded_license.0.to_string()
                };
                Some(violation)
            },
            true => None
        }
    }
}
