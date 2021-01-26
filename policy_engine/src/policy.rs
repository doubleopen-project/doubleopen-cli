// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::path::Path;

use super::{license::License, policy_file::PolicyFile, resolution::Resolution};
use serde::{Deserialize, Serialize};

/// Struct for a license policy to be used with the Policy Engine.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Policy {
    /// Allowlisted licenses.
    pub licenses_allow: Vec<License>,

    /// Denylisted licenses.
    pub licenses_deny: Vec<License>,

    /// Resolutions for rule violations.
    pub resolutions: Vec<Resolution>,
}

impl Policy {
    /// Creates a policy from an undefined amount of Policy Files. Provide context
    /// of the application.
    pub fn from_files<P: AsRef<Path>>(files: &[P], context: &str) -> Self {
        let combined_files = PolicyFile::from_multiple_files(&files);
        Self::from_policy_file(combined_files, context)
    }

    /// Creates A policy from single PolicyFile struct based on context..
    pub fn from_policy_file(policy_file: PolicyFile, context: &str) -> Self {
        let allowed_licensens: Vec<License> = policy_file
            .licenses
            .iter()
            .filter_map(|lic| {
                let find_context = lic
                    .allowed_contexts
                    .iter()
                    .find(|ctx| ctx.context == context);
                match find_context {
                    Some(ctx) => Some(License {
                        message: ctx.description.clone(),
                        spdx_expression: lic.spdx_id.clone(),
                    }),
                    None => None,
                }
            })
            .collect();

        let denied_licensens: Vec<License> = policy_file
            .licenses
            .iter()
            .filter_map(|lic| {
                let find_context = lic
                    .denied_contexts
                    .iter()
                    .find(|ctx| ctx.context == context);
                match find_context {
                    Some(ctx) => Some(License {
                        message: ctx.description.clone(),
                        spdx_expression: lic.spdx_id.clone(),
                    }),
                    None => None,
                }
            })
            .collect();

        let resolutions: Vec<Resolution> = policy_file
            .resolutions
            .iter()
            .filter_map(|res| {
                let find_context = res.contexts.iter().find(|ctx| ctx.as_str() == context);
                match find_context {
                    Some(_) => Some(Resolution {
                        license: res.spdx_id.clone(),
                        package: res.package.clone(),
                        reason: res.description.clone(),
                    }),
                    None => None,
                }
            })
            .collect();
        Policy {
            licenses_allow: allowed_licensens,
            licenses_deny: denied_licensens,
            resolutions,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_policy_from_two_files() {
        let policy_1 = r#"
          licenses:
          - spdx_id: "MIT"
            allowed_contexts: 
            - context: "saas"
            - context: "consumer software"
              description: "Is good"
            - context: "internal"
          - spdx_id: "GPL-2.0-only"
            allowed_contexts:
            - context: "saas"
            - context: "internal"
            denied_contexts:
            - context: "consumer software"
              description: "Not good"
          resolutions:
          - package: "application-1.0.0"
            contexts:
            - "consumer software"
            spdx_id: GPL-2.0-only
            description: "Licensed by the author separately."
          - package: "application-2.0.0"
            contexts:
            - "saas"
            spdx_id: GPL-2.0-only
            description: "Licensed by the author separately."
       "#;

        let policy_2 = r#"
          licenses:
          - spdx_id: "GPL-2.0-only"
            denied_contexts:
            - context: "saas"
              description: "No GPL in saas for this project"
       "#;

        let mut policy1: PolicyFile = serde_yaml::from_str(policy_1).unwrap();
        let policy2: PolicyFile = serde_yaml::from_str(policy_2).unwrap();
        policy1.add_overriding_policy(&policy2);

        let policy = Policy::from_policy_file(policy1, "saas");

        let expected_policy = Policy {
            licenses_allow: vec![License {
                spdx_expression: "MIT".into(),
                message: None,
            }],
            licenses_deny: vec![License {
                spdx_expression: "GPL-2.0-only".into(),
                message: Some("No GPL in saas for this project".into()),
            }],
            resolutions: vec![Resolution {
                license: "GPL-2.0-only".into(),
                package: "application-2.0.0".into(),
                reason: "Licensed by the author separately.".into(),
            }],
        };

        assert_eq!(policy, expected_policy);
    }
}
