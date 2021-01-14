// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::{fs, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

/// Struct for deserializing policy files for turning into Policy.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PolicyFile {
    #[serde(default)]
    pub licenses: Vec<PolicyFileLicense>,
    #[serde(default)]
    pub resolutions: Vec<PolicyFileResolution>,
}

impl PolicyFile {
    /// Parse a yaml policy file to PolicyFile struct.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let file = fs::File::open(&path).expect("Policy file not found");
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    }

    /// Add a policy with higher priority to a policy, adding rules from the new
    /// policy that did not exist in the old one and overwriting conflicting rules.
    pub fn add_overriding_policy(&mut self, policy: &PolicyFile) {
        // Loop over the licenses in the new file.
        for new_license in &policy.licenses {
            // Find the new license from licenses in old policy.
            let old_license = self
                .licenses
                .iter_mut()
                .find(|old| old.spdx_id == new_license.spdx_id);

            // Process based on whether the license was in the old policy.
            match old_license {
                Some(old) => {
                    // The license was found in old policy. Loop over all of the allowed contexts in the license in new policy.
                    for new_allowed_context in new_license.allowed_contexts.iter() {
                        // Find the new context from the list of allowed contexts in old policy.
                        let old_allowed_context = old
                            .allowed_contexts
                            .iter()
                            .find(|ctx| ctx.context == new_allowed_context.context);

                        // Process based on whether the new context was in allowed license in the old policy.
                        match old_allowed_context {
                            // If the old allowed contexts already included the new allowed context, do nothing.
                            Some(_) => {}
                            // If the new context was not allowed, add it.
                            None => {
                                old.allowed_contexts.push(new_allowed_context.clone());
                            }
                        }

                        // Find the new context from the list of denied contexts in old policy. Return the index.
                        let old_denied_context = old
                            .denied_contexts
                            .iter()
                            .position(|ctx| ctx.context == new_allowed_context.context);

                        // Process based on whether the new allowed context was in denied licenses in the old policy.
                        if let Some(index) = old_denied_context {
                            old.denied_contexts.remove(index);
                        }
                    }

                    // The license was found in old policy. Loop over all of the allowed contexts in the license in new policy.
                    for new_denied_context in new_license.denied_contexts.iter() {
                        // Find the new context from the list of denied contexts in old policy.
                        let old_denied_context = old
                            .denied_contexts
                            .iter()
                            .find(|ctx| ctx.context == new_denied_context.context);

                        // Process based on whether the new context was in denied license in the old policy.
                        match old_denied_context {
                            // If the old allowed contexts already included the new denied context, do nothing.
                            Some(_) => {}
                            // If the new context was not denied, add it.
                            None => {
                                old.denied_contexts.push(new_denied_context.clone());
                            }
                        }

                        // Find the new context from the list of allowed contexts in old policy. Return the index.
                        let old_allowed_context = old
                            .allowed_contexts
                            .iter()
                            .position(|ctx| ctx.context == new_denied_context.context);

                        // Process based on whether the new denied context was in allowed contexts in the old policy.
                        if let Some(index) = old_allowed_context {
                            old.allowed_contexts.remove(index);
                        }
                    }
                }
                None => {
                    self.licenses.push(new_license.clone());
                }
            }
        }
    }

    /// Create the struct from multiple files, with the last one being the most
    /// prioritized.
    pub fn from_multiple_files<P: AsRef<Path>>(paths: &[P]) -> Self {
        let mut policies: Vec<PolicyFile> = paths
            .iter()
            .map(|path| PolicyFile::from_file(&path))
            .collect();

        let initial_policy = PolicyFile {
            licenses: vec![],
            resolutions: vec![],
        };

        policies.iter_mut().fold(initial_policy, |mut old, new| {
            old.add_overriding_policy(new);
            old
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PolicyFileLicense {
    pub spdx_id: String,
    #[serde(default)]
    pub allowed_contexts: Vec<PolicyFileContext>,
    #[serde(default)]
    pub denied_contexts: Vec<PolicyFileContext>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PolicyFileContext {
    pub context: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PolicyFileResolution {
    pub package: String,
    #[serde(default)]
    pub contexts: Vec<String>,
    pub spdx_id: String,
    #[serde(default)]
    pub description: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_combine() {
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

        let mut expected_licenses: Vec<PolicyFileLicense> = vec![
            PolicyFileLicense {
                spdx_id: "MIT".into(),
                allowed_contexts: vec![
                    PolicyFileContext {
                        context: "saas".into(),
                        description: None,
                    },
                    PolicyFileContext {
                        context: "internal".into(),
                        description: None,
                    },
                    PolicyFileContext {
                        context: "consumer software".into(),
                        description: Some("Is good".into()),
                    },
                ],
                denied_contexts: vec![],
            },
            PolicyFileLicense {
                spdx_id: "GPL-2.0-only".into(),
                allowed_contexts: vec![PolicyFileContext {
                    context: "internal".into(),
                    description: None,
                }],
                denied_contexts: vec![
                    PolicyFileContext {
                        context: "saas".into(),
                        description: Some("No GPL in saas for this project".into()),
                    },
                    PolicyFileContext {
                        context: "consumer software".into(),
                        description: Some("Not good".into()),
                    },
                ],
            },
        ];
        expected_licenses.sort_by_key(|lic| lic.spdx_id.to_string());
        for license in &mut expected_licenses {
            license
                .allowed_contexts
                .sort_by_key(|lic| lic.context.to_string());
            license
                .denied_contexts
                .sort_by_key(|lic| lic.context.to_string());
        }

        let mut expected_resolutions: Vec<PolicyFileResolution> = vec![PolicyFileResolution {
            package: "application-1.0.0".into(),
            contexts: vec!["consumer software".into()],
            description: "Licensed by the author separately.".into(),
            spdx_id: "GPL-2.0-only".into(),
        }];
        expected_resolutions.sort_by_key(|res| res.package.to_string());

        let expected_policy = PolicyFile {
            licenses: expected_licenses,
            resolutions: expected_resolutions,
        };

        policy1.add_overriding_policy(&policy2);
        policy1.licenses.sort_by_key(|lic| lic.spdx_id.to_string());
        policy1
            .resolutions
            .sort_by_key(|res| res.package.to_string());
        for license in &mut policy1.licenses {
            license
                .allowed_contexts
                .sort_by_key(|lic| lic.context.to_string());
            license
                .denied_contexts
                .sort_by_key(|lic| lic.context.to_string());
        }

        assert_eq!(policy1, expected_policy);
    }
}
