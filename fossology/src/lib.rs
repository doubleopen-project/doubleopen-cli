// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

#![deny(clippy::all)]
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
            sha256: Some(sha_256.into()),
            ..Default::default()
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

#[cfg(test)]
mod tests {
    use super::Fossology;
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
}
