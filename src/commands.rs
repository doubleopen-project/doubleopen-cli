// SPDX-FileCopyrightText: 2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::path::Path;

use fossology_rs::{Fossology, FossologyError};
use log::{error, info};

use crate::utilities::hash256_for_path;

pub(crate) fn upload_missing_archives_to_fossology<P: AsRef<Path>>(
    source_archives: Vec<P>,
    fossology: &Fossology,
    fossolody_folder: &i32,
) -> Result<(), FossologyError> {
    info!("Uploading missing archives to Fossology.",);

    for file in source_archives {
        let sha256 = hash256_for_path(&file);
        if !fossology.file_exists(&sha256)? {
            match fossology.upload(&file, fossolody_folder) {
                Ok(_) => info!(
                    "Succesfully uploaded {} to Fossology.",
                    &file.as_ref().display()
                ),
                Err(err) => {
                    error!(
                        "Failed uploading {} to Fossology: {}",
                        &file.as_ref().display(),
                        err
                    )
                }
            }
        } else {
            info!(
                "{} exists on Fossology, did not upload again.",
                &file.as_ref().display()
            );
        }
    }
    Ok(())
}
