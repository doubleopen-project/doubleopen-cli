// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

#![deny(clippy::all)]
use crate::spdx::SPDXExpression;

use self::api_objects::{requests::*, responses::*};
use reqwest::blocking::{multipart::Form, Client};
use serde::{Deserialize, Serialize};
use std::{path::Path, thread, time};
use time::Duration;
pub mod api_objects;

/// Fossology instance.
#[derive(Debug)]
pub struct Fossology {
    /// API base uri.
    uri: String,

    /// Access token for Fossology.
    token: String,

    /// Reqwest client.
    client: Client,
}

#[derive(Debug, thiserror::Error)]
pub enum FossologyError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

/// Objects in downloads-folder to be uploaded to Fossology.
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadObject {
    path: String,
    sha256: String,
    exists_in_fossology: bool,
}

impl Fossology {
    /// Initialize Fossology with URI and token.
    pub fn new(uri: &str, token: &str) -> Self {
        Self {
            uri: uri.to_owned(),
            token: token.to_owned(),
            client: Client::new(),
        }
    }

    /// Upload a package to Fossology.
    pub fn upload<P: AsRef<Path>>(&self, path_to_source: P, folder_id: &i32) {
        // Get the file in multipart form.
        let form = Form::new().file("fileInput", path_to_source).unwrap();

        // Upload the file to Fossology.
        let response: UploadPackageResponse = self
            .client
            .post(&format!("{}/uploads", self.uri))
            .timeout(Duration::from_secs(600))
            .bearer_auth(&self.token)
            .header("folderId", folder_id.to_string())
            .multipart(form)
            .send()
            .unwrap()
            .json()
            .unwrap();

        // Wait for unpacker to finish.
        while !self.upload_exists_by_id(&response.message) {
            thread::sleep(time::Duration::from_secs(10));
        }

        // Schedule scanner jobs for the upload.
        self.schedule_jobs(folder_id, &response.message);
    }

    /// Schedule all Fossology jobs for an upload.
    fn schedule_jobs(&self, folder_id: &i32, upload_id: &i32) {
        let input = ScheduleJobsInput::new();
        let _response = self
            .client
            .post(&format!("{}/jobs", self.uri))
            .bearer_auth(&self.token)
            .header("folderId", folder_id.to_string())
            .header("uploadId", upload_id.to_string())
            .json(&input)
            .send()
            .unwrap()
            .text()
            .unwrap();
    }

    /// Check if an upload exists on Fossology by upload id.
    fn upload_exists_by_id(&self, upload_id: &i32) -> bool {
        // If an upload with a specified id does not exist, Fossology returns a
        // 503 error. We query the api with an upload id and try to deserialize the
        // response as a successful query. If the deserialization is successful,
        // an upload with the specified id exists.
        let response: Result<UploadDetailResponse, reqwest::Error> = self
            .client
            .get(&format!("{}/uploads/{}", self.uri, upload_id))
            .bearer_auth(&self.token)
            .send()
            .unwrap()
            .json();

        match response {
            Ok(_res) => true,
            Err(_err) => false,
        }
    }

    /// Check if the file exist in Fossology based on sha256 value.
    pub fn file_exists(&self, sha_256: &str) -> Result<bool, FossologyError> {
        let body = HashQueryInput {
            sha256: sha_256.into(),
        };
        let response: Vec<HashQueryResponse> = self
            .client
            .post(&format!("{}/filesearch", self.uri))
            .bearer_auth(&self.token)
            .json(&vec![body])
            .send()?
            .json()?;

        if response[0].message == Some("Not found".into()) {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Get licenses for a list of hashes
    pub fn licenses_for_hashes(
        &self,
        hashes: &[HashQueryInput],
    ) -> Result<Vec<HashQueryResponse>, FossologyError> {
        let response: Vec<HashQueryResponse> = self
            .client
            .post(&format!("{}/filesearch", self.uri))
            .timeout(Duration::from_secs(600))
            .bearer_auth(&self.token)
            .json(&hashes)
            .send()?
            .json()?;

        Ok(response)
    }
}

/// Transform a list of licenses returned by Fossology to an SPDX license expression.
/// Fossology's Dual-license tag doesn't allow accurate representation of OR licenses
/// with more than two licenses, so all license combinations with 3 or more licenses
/// are interpreted as AND licenses.
pub fn spdx_expression_from_api_licenses(mut fossology_licenses: Vec<String>) -> SPDXExpression {
    if fossology_licenses.len() == 3 && fossology_licenses.contains(&"Dual-license".into()) {
        let dual_license_position = fossology_licenses
            .iter()
            .position(|lic| lic == "Dual-license")
            .expect("Should always exist here");

        fossology_licenses.remove(dual_license_position);
        let expression = fossology_licenses.join(" OR ");
        return SPDXExpression(expression);
    } else {
        let expression = fossology_licenses
            .iter()
            .filter(|&lic| lic != &"Dual-license".to_string())
            .cloned()
            .collect::<Vec<_>>()
            .join(" AND ");
        return SPDXExpression(expression);
    }
}

#[cfg(test)]
mod tests {
    use crate::spdx::SPDXExpression;

    use super::{spdx_expression_from_api_licenses, Fossology};
    use reqwest::blocking::Client;

    #[test]
    fn fossology_is_created() {
        let expected_fossology = Fossology {
            token: "token".into(),
            uri: "uri".into(),
            client: Client::new(),
        };

        let fossology = Fossology::new("uri", "token");

        assert_eq!(fossology.token, expected_fossology.token);
        assert_eq!(fossology.uri, expected_fossology.uri);
    }

    #[test]
    fn test_spdx_expression_from_fossology() {
        let input_1 = vec![
            "MIT".to_string(),
            "Dual-license".to_string(),
            "ISC".to_string(),
        ];

        let expected_1 = SPDXExpression("MIT OR ISC".into());

        assert_eq!(expected_1, spdx_expression_from_api_licenses(input_1));

        let input_2 = vec!["MIT".to_string(), "ISC".to_string()];

        let expected_2 = SPDXExpression("MIT AND ISC".into());

        assert_eq!(expected_2, spdx_expression_from_api_licenses(input_2));

        let input_3 = vec![
            "MIT".to_string(),
            "Dual-license".to_string(),
            "ISC".to_string(),
            "GPL-2.0-only".to_string(),
        ];

        let expected_3 = SPDXExpression("MIT AND ISC AND GPL-2.0-only".into());

        assert_eq!(expected_3, spdx_expression_from_api_licenses(input_3));
    }
}
