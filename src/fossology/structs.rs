use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HashQueryInput {
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HashQueryResponse {
    pub hash: Hash,
    findings: Option<Findings>,
    uploads: Option<Vec<i32>>,
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
struct Findings {
    scanner: Vec<String>,
    conclusion: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadPackageResponse {
    pub code: i32,
    pub message: i32,
    #[serde(rename = "type")]
    pub response_type: String,
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

#[cfg(test)]
mod tests {
    use super::HashQueryResponse;

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

        let _: Vec<HashQueryResponse> = serde_json::from_str(&response).unwrap();
    }

    #[test]
    fn hash_query_success() {
        let response = r#"
        [
            {
              "hash": {
                "sha1": "CD54520C6C3C42E53685DC706E28721742AA3FFF",
                "md5": "C80C5A8385011A0260DCE6BD0DA93DCE",
                "sha256": "767EC234AE7AA684695B3A735548224888132E063F92DB585759B422570621D4",
                "size": 487840
              },
              "findings": {
                "scanner": [],
                "conclusion": []
              },
              "uploads": [
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13
              ]
            }
        ]
        "#;

        let _: Vec<HashQueryResponse> = serde_json::from_str(&response).unwrap();
    }
}
