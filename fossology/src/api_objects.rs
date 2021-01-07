// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

/// Request payloads for the Fossology API.
pub mod requests {
    use serde::{Deserialize, Serialize};

    // TODO: Add an enclosing array.
    /// # POST to `/filesearch`.
    ///
    /// List of file hashes to fetch
    #[derive(Serialize, Deserialize, Debug)]
    pub struct HashQueryInput {
        pub sha1: Option<String>,
        pub md5: Option<String>,
        pub sha256: Option<String>,
        pub size: Option<i32>,
    }

    impl Default for HashQueryInput {
        fn default() -> Self {
            Self {
                sha1: None,
                md5: None,
                sha256: None,
                size: None,
            }
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ScheduleJobsInput {
        analysis: Analysis,
        decider: Decider,
    }
    impl ScheduleJobsInput {
        pub fn new() -> Self {
            Self {
                analysis: Analysis {
                    ..Default::default()
                },
                decider: Decider {
                    ..Default::default()
                },
            }
        }
    }

    impl Default for ScheduleJobsInput {
        fn default() -> Self {
            Self {
                analysis: Analysis {
                    ..Default::default()
                },
                decider: Decider {
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Decider {
        nomos_monk: bool,
        /// Needs to be false for the other deciders to work:
        /// https://github.com/fossology/fossology/issues/1639
        bulk_reused: bool,
        new_scanner: bool,
        ojo_decider: bool,
    }

    impl Default for Decider {
        fn default() -> Self {
            Self {
                bulk_reused: false,
                new_scanner: true,
                nomos_monk: true,
                ojo_decider: true,
            }
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Analysis {
        bucket: bool,
        copyright_email_author: bool,
        ecc: bool,
        keyword: bool,
        mime: bool,
        monk: bool,
        nomos: bool,
        ojo: bool,
        package: bool,
    }

    impl Default for Analysis {
        fn default() -> Self {
            Self {
                bucket: true,
                copyright_email_author: true,
                ecc: true,
                keyword: true,
                mime: true,
                monk: true,
                nomos: true,
                ojo: true,
                package: true,
            }
        }
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HashQueryResponse {
        pub hash: Hash,
        pub findings: Option<Findings>,
        pub uploads: Option<Vec<i32>>,
        pub message: Option<String>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Hash {
        pub sha1: Option<String>,
        pub md5: Option<String>,
        pub sha256: Option<String>,
        pub size: Option<i64>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Findings {
        pub scanner: Vec<String>,
        pub conclusion: Vec<String>,
        pub copyright: Vec<String>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UploadPackageResponse {
        pub code: i32,
        pub message: i32,
        #[serde(rename = "type")]
        pub response_type: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UploadDetailResponse {
        #[serde(rename = "folderid")]
        folder_id: i32,
        #[serde(rename = "foldername")]
        folder_name: String,
        id: i32,
        description: String,
        #[serde(rename = "uploadname")]
        upload_name: String,
        #[serde(rename = "uploaddate")]
        upload_date: String,
        hash: Hashes,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Hashes {
        sha1: String,
        md5: String,
        sha256: String,
        size: i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_query_failure() {
        let response = r#"
        [
            {
              "hash": {
                "sha1": null,
                "md5": null,
                "sha256": "767ec234ae7aa684695b3a735548224888132e063f92db585759b422570621d4x",
                "size": null
              },
              "message": "Not found"
            }
        ]
        "#;

        let responses: Vec<responses::HashQueryResponse> = serde_json::from_str(&response).unwrap();

        assert_eq!(responses[0].message, Some("Not found".to_string()));
    }

    #[test]
    fn hash_query_success() {
        let response = r#"
        [
            {
                "hash": {
                "sha1": "04295F2D04DA21B646A314983A2C1C811E8DB72A",
                "md5": "1D1545D41BC2B900F353D7841CCA44EE",
                "sha256": "3845FEEAAE1AA266A89F9C3FDFA24674D8FED96639631CC3D718D4B15CC28696",
                "size": 2063
                },
                "findings": {
                "scanner": [
                    "HPND-sell-variant"
                ],
                "conclusion": [
                    "HPND-sell-variant"
                ],
                "copyright": [
                    "Copyright 2005 Red Hat, Inc."
                ]
                },
                "uploads": []
            }
        ]
        "#;

        let responses: Vec<responses::HashQueryResponse> = serde_json::from_str(&response).unwrap();

        assert_eq!(
            responses[0].hash.md5,
            Some("1D1545D41BC2B900F353D7841CCA44EE".into())
        );
    }
}
