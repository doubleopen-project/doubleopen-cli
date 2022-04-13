// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

use std::{path::Path, thread::sleep, time::Duration};

use fossology_rs::{
    job::{schedule_analysis, ScheduleAgents},
    upload::{filesearch, get_upload_by_id, new_upload_from_file, Hash},
    Fossology, FossologyError,
};
use log::info;
use spdx_rs::models::PackageInformation;

use crate::{
    fossology::doubleopen_licenses::{get_packages_with_closed_license, skip_package_upload},
    utilities::hash256_for_path,
};

pub fn upload_missing_archives_to_fossology<P: AsRef<Path>>(
    source_archives: Vec<P>,
    fossology: &Fossology,
    fossolody_folder: &i32,
    spdx_packages: &[PackageInformation],
    dry_run: bool,
) -> anyhow::Result<()> {
    info!("Uploading missing archives to Fossology.",);

    let packages_to_skip = get_packages_with_closed_license(spdx_packages);

    let (paths_to_skip, paths_to_upload): (Vec<_>, Vec<_>) = source_archives
        .iter()
        .partition(|archive| skip_package_upload(archive, &packages_to_skip));

    if !dry_run {
        for path in paths_to_skip {
            let display = path.as_ref().display();
            info!("Will not upload {} based on its license.", display);
        }

        for file in paths_to_upload {
            let sha256 = hash256_for_path(&file)?;
            let input = vec![Hash::from_sha256(&sha256)];

            let not_on_fossology = filesearch(fossology, &input, None).unwrap().is_empty();

            if not_on_fossology {
                let upload = new_upload_from_file(fossology, *fossolody_folder, &file)?;

                let ununpack_in_progress = |fossology, upload_id| -> Result<bool, FossologyError> {
                    get_upload_by_id(fossology, upload_id).map_or_else(
                        |err| match err {
                            FossologyError::Other(error_string) => {
                                if error_string.contains("Ununpack job not started") {
                                    Ok(true)
                                } else {
                                    Err(FossologyError::Other(error_string))
                                }
                            }
                            err => Err(err),
                        },
                        |res| Ok(res.is_none()),
                    )
                };

                while ununpack_in_progress(fossology, upload.upload_id)? {
                    info!(
                        "Waiting for {} to be unarchived on Fossology.",
                        &file.as_ref().display()
                    );
                    sleep(Duration::from_secs(10));
                }

                let mut analysis_input = ScheduleAgents::default();
                analysis_input.analysis.bucket = true;
                analysis_input.analysis.copyright_email_author = true;
                analysis_input.analysis.ecc = true;
                analysis_input.analysis.keyword = true;
                analysis_input.analysis.mime = true;
                analysis_input.analysis.monk = true;
                analysis_input.analysis.nomos = true;
                analysis_input.analysis.ojo = true;
                analysis_input.analysis.package = true;

                analysis_input.decider.new_scanner = true;
                analysis_input.decider.nomos_monk = true;
                analysis_input.decider.ojo_decider = true;

                schedule_analysis(
                    fossology,
                    *fossolody_folder,
                    upload.upload_id,
                    None,
                    &analysis_input,
                )?;
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
