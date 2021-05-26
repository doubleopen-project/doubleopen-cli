// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! Fossology API response and request payloads.

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
    }

    impl Default for HashQueryInput {
        fn default() -> Self {
            Self {
                sha1: None,
                md5: None,
                sha256: None,
            }
        }
    }

    /// # POST to `/jobs`.
    ///
    /// Schedule an analysis.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ScheduleJobsInput {
        analysis: Analysis,
        decider: Decider,
        // TODO: Reuse.
    }
    impl ScheduleJobsInput {
        /// Create the payload with default values.
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

    /// Decider jobs to schedule with POST to `/jobs`.
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

    /// Analysis jobs to schedule with POST to `/jobs`.
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

/// Response payloads for the Fossology API.
pub mod responses {
    use serde::{Deserialize, Serialize};

    /// Response for POST to `/filesearch`.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct HashQueryResponse {
        pub hash: Hash,
        pub findings: Option<Findings>,
        pub uploads: Option<Vec<i32>>,
        pub message: Option<String>,
    }

    /// Hash values in the response for POST to `/filesearch`.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Hash {
        pub sha1: Option<String>,
        pub md5: Option<String>,
        pub sha256: Option<String>,
        pub size: Option<i64>,
    }

    /// License and copyright findings in the response for POST to `/filesearch`.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Findings {
        pub scanner: Vec<String>,
        pub conclusion: Vec<String>,
        pub copyright: Vec<String>,
    }

    // TODO: The API's default response, should probably make more general.
    /// Response for POST to `/uploads`.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UploadPackageResponse {
        pub code: i32,
        pub message: i32,
        #[serde(rename = "type")]
        pub response_type: String,
    }

    /// Response for GET to `/uploads/{id}`.
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

    /// Hash values of upload detail from GET to `/uploads/{id}`.
    // TODO: Might be able to combine with the hash struct in filesearch.
    #[derive(Serialize, Deserialize, Debug)]
    struct Hashes {
        sha1: String,
        md5: String,
        sha256: String,
        size: i32,
    }

    /// Response for GET from `/license`.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetLicense {
        pub id: i32,
        pub short_name: String,
        pub full_name: String,
        pub text: String,
        pub risk: Option<i32>,
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

    #[test]
    fn license_search() {
        let response1 = r#"
        {
            "id": 126,
            "shortName": "MIT",
            "fullName": "MIT License",
            "text": "MIT License Copyright (c) <year> <copyright holders>\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and\/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.",
            "risk": null
        }
        "#;

        let response2 = r#"
        {
            "id": 126,
            "shortName": "MIT",
            "fullName": "MIT License",
            "text": "MIT License Copyright (c) <year> <copyright holders>\n\nPermission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and\/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\n\nThe above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.\n\nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.",
            "risk": 5
        }
        "#;

        let response1: responses::GetLicense = serde_json::from_str(&response1).unwrap();
        let response2: responses::GetLicense = serde_json::from_str(&response2).unwrap();

        assert_eq!(response1.full_name, "MIT License");
        assert_eq!(response1.risk, None);
        assert_eq!(response2.risk, Some(5));
    }
}
