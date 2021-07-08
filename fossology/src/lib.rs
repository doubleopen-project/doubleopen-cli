// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! # Fossology
//!
//! Module for communicating with Fossology's REST API.

#![deny(clippy::all)]
use self::api_objects::{requests::*, responses::*};
use api_objects::responses;
use log::{debug, error, info};
use reqwest::blocking::{multipart::Form, Client};
use serde::{Deserialize, Serialize};
use std::{fs::read_dir, path::Path, thread, time};
use time::Duration;
use utilities::hash256_for_path;
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

/// Error when interacting with Fossology.
#[derive(Debug, thiserror::Error)]
pub enum FossologyError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

// TODO: Can be deleted.
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
    pub fn upload<P: AsRef<Path>>(
        &self,
        path_to_source: P,
        folder_id: &i32,
    ) -> Result<(), FossologyError> {
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
            .send()?
            .json()?;

        // Wait for unpacker to finish.
        while !self.upload_exists_by_id(&response.message) {
            thread::sleep(time::Duration::from_secs(10));
        }

        // Schedule scanner jobs for the upload. Loop until succesful.
        loop {
            match self.schedule_jobs(folder_id, &response.message) {
                Ok(_) => {
                    debug!("Scheduling job succeeded.");
                    break;
                }
                Err(err) => error!("Scheduling job failed, trying again: {}", err),
            }
        }

        Ok(())
    }

    /// Schedule all Fossology jobs for an upload.
    fn schedule_jobs(&self, folder_id: &i32, upload_id: &i32) -> Result<(), FossologyError> {
        let input = ScheduleJobsInput::new();
        let _response = self
            .client
            .post(&format!("{}/jobs", self.uri))
            .bearer_auth(&self.token)
            .header("folderId", folder_id.to_string())
            .header("uploadId", upload_id.to_string())
            .json(&input)
            .send()?
            .text()?;

        Ok(())
    }

    /// Check if an upload exists on Fossology by upload id.
    fn upload_exists_by_id(&self, upload_id: &i32) -> bool {
        // TODO: Currently returns false if the request for example times out.
        // Handle with Error instead.

        // If an upload with a specified id does not exist, Fossology returns a
        // 503 error. We query the api with an upload id and try to deserialize the
        // response as a successful query. If the deserialization is successful,
        // an upload with the specified id exists.
        let response = self
            .client
            .get(&format!("{}/uploads/{}", self.uri, upload_id))
            .bearer_auth(&self.token)
            .send();

        match response {
            Ok(res) => {
                let response: Result<UploadDetailResponse, reqwest::Error> = res.json();
                match response {
                    Ok(_res) => true,
                    Err(_err) => false,
                }
            }
            Err(_) => false,
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
            .timeout(Duration::from_secs(1800))
            .bearer_auth(&self.token)
            .json(&hashes)
            .send()?
            .json()?;

        Ok(response)
    }

    /// Get license details by short name.
    pub fn license_by_short_name(
        &self,
        short_name: &str,
    ) -> Result<responses::GetLicense, FossologyError> {
        let short_name = if short_name.starts_with("LicenseRef-") {
            short_name
                .strip_prefix("LicenseRef-")
                .expect("Should always exist.")
        } else {
            short_name
        };

        let response: responses::GetLicense = self
            .client
            .get(&format!("{}/license", self.uri))
            .bearer_auth(&self.token)
            .header("shortName", short_name)
            .send()?
            .json()?;

        Ok(response)
    }

    pub fn upload_files_in_dir<P: AsRef<Path>>(
        &self,
        path_to_dir: P,
        folder_id: &i32,
    ) -> Result<(), FossologyError> {
        let files_in_dir = read_dir(path_to_dir).expect("Error reading directory.");
        for file in files_in_dir {
            let path = file.unwrap().path();
            if path.to_str().unwrap().ends_with("tar.bz2") {
                let sha256 = hash256_for_path(&path);
                if !self.file_exists(&sha256)? {
                    info!("Uploading {}", &path.display());
                    match self.upload(&path, folder_id) {
                        Ok(_) => info!("Succesfully uploaded {} to Fossology.", &path.display()),
                        Err(err) => {
                            error!("Failed uploading {} to Fossology: {}", &path.display(), err)
                        }
                    }
                } else {
                    info!(
                        "{} exist on Fossology, did not upload again.",
                        &path.display()
                    );
                }
            }
        }
        Ok(())
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
