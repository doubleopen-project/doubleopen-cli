// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

#![deny(clippy::all)]
use super::structs::{
    HashQueryInput, HashQueryResponse, ScheduleJobsInput, UploadDetailResponse,
    UploadPackageResponse,
};
use crate::utilities::hash256_for_path;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use reqwest::blocking::{multipart::Form, Client};
use serde::{Deserialize, Serialize};
use std::{fs, thread, time};
use time::Duration;

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

    /// Get version number for the Fossology instance.
    pub fn version(&self) {
        let body = self
            .client
            .get(&format!("{}/version", self.uri))
            .bearer_auth(&self.token)
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("Fossology version: {}", body);
    }

    /// Upload a package to Fossology.
    pub fn upload(&self, path_to_source: &str, folder_id: &i32) {
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

    pub fn upload_all_in_folder(&self, folder_path: &str) {
        let paths: Vec<String> = fs::read_dir(folder_path)
            .unwrap()
            .filter_map(|x| {
                let x = x.unwrap();
                if x.metadata().unwrap().is_file() {
                    Some(x.path().to_str().unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        let file_count = fs::read_dir(folder_path).unwrap().count() as u64;

        let pb = ProgressBar::new(file_count);

        println!("Calculating hashes for source packages.");
        let mut upload_objects: Vec<UploadObject> = paths
            .into_par_iter()
            .map(|i| {
                pb.inc(1);
                let hash = hash256_for_path(&i);
                UploadObject {
                    path: i.to_string(),
                    sha256: hash.clone(),
                    exists_in_fossology: false,
                }
            })
            .collect();

        pb.finish();

        // Check fossology for which of the packages are already uploaded.
        println!("Querying Fossology for existing packages.");
        let response: Vec<HashQueryResponse> = self
            .client
            .post(&format!("{}/filesearch", self.uri))
            .bearer_auth(&self.token)
            .json::<Vec<HashQueryInput>>(
                &upload_objects
                    .iter()
                    .map(|x| HashQueryInput {
                        sha256: x.sha256.to_string(),
                    })
                    .collect(),
            )
            .send()
            .unwrap()
            .json()
            .unwrap();

        // Match fossology response with the packages detected.
        for i in &mut upload_objects {
            if !i.path.ends_with(".done") {
                let includes = !&response.iter().any(|e| {
                    e.hash.sha1.is_none()
                        && e.hash.sha256.as_ref().map(String::as_str)
                            == Some(&i.sha256.to_uppercase())
                });
                if includes == true {
                    i.exists_in_fossology = true;
                }
            }
        }

        let style = ProgressStyle::default_bar().template("{wide_bar} {pos}/{len} {msg}");

        // Filter files that should not be uploaded.
        // TODO: Refactor to more modular function.
        upload_objects.retain(|x| !x.path.ends_with(".done"));

        // Filter files already in Fossology.
        upload_objects.retain(|x| x.exists_in_fossology == false);

        println!("Uploading source packages to Fossology.");
        let upload_objects_length = upload_objects.len() as u64;
        let pb = ProgressBar::new(upload_objects_length).with_style(style);
        upload_objects.iter().for_each(|x| {
            pb.set_message(&x.path);
            pb.inc(1);
            // TODO: Get input from user for the correct folder.
            self.upload(&x.path, &3);
        });
        pb.finish();
    }

    /// Get licenses for a list of hashes
    pub fn licenses_for_hashes(&self, hashes: &Vec<HashQueryInput>) -> Vec<HashQueryResponse> {
        let response: Vec<HashQueryResponse> = self
            .client
            .post(&format!("{}/filesearch", self.uri))
            .bearer_auth(&self.token)
            .json(&hashes)
            .send()
            .unwrap()
            .json()
            .unwrap();

        response
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
