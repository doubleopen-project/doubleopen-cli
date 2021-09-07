// SPDX-FileCopyrightText: 2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::path::Path;

use fossology_rs::{Fossology, FossologyError};
use log::{error, info};
use spdx_rs::PackageInformation;

use crate::{
    doubleopen::{get_packages_with_closed_license, skip_package_upload},
    utilities::hash256_for_path,
};

pub(crate) fn upload_missing_archives_to_fossology<P: AsRef<Path>>(
    source_archives: Vec<P>,
    fossology: &Fossology,
    fossolody_folder: &i32,
    spdx_packages: &[PackageInformation],
    dry_run: bool,
) -> Result<(), FossologyError> {
    info!("Uploading missing archives to Fossology.",);

    let packages_to_skip = get_packages_with_closed_license(spdx_packages);

    let (paths_to_skip, paths_to_upload): (Vec<_>, Vec<_>) = source_archives
        .iter()
        .partition(|archive| skip_package_upload(archive, &packages_to_skip));

    if !dry_run {
        for file in paths_to_upload {
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
    } else {
        info!("DRY_RUN: Real run would UPLOAD the following packages:");
        for path in paths_to_upload {
            let display = path.as_ref().display();
            info!("UPLOAD: {}", display);
        }

        info!("DRY_RUN: Real run would SKIP the following packages:");
        for path in paths_to_skip {
            let display = path.as_ref().display();
            info!("SKIP: {}", display);
        }
    }
    Ok(())
}
