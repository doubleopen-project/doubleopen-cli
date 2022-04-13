// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

use fossology_rs::{
    upload::{filesearch, FilesearchResponse, Hash},
    Fossology,
};
use log::{debug, info};
use spdx_rs::models::FileInformation;

/// Query Fossology for [`FilesearchResponse`] based on SHA256 values of the input
/// [`FileInformation`].
pub fn filesearch_for_file_information(
    files: &[FileInformation],
    fossology: &Fossology,
) -> anyhow::Result<Vec<FilesearchResponse>> {
    const CHUNK_SIZE: usize = 2000;

    let hashes = files
        .iter()
        .filter_map(|file| {
            file.checksum(spdx_rs::models::Algorithm::SHA256)
                .map(Hash::from_sha256)
        })
        .collect::<HashSet<_>>();

    let hashes = hashes.into_iter().collect::<Vec<_>>();

    debug!("Query fossology with {} unique hashes.", &hashes.len());

    let mut responses = Vec::new();

    for (i, batch) in hashes.chunks(CHUNK_SIZE).enumerate() {
        info!(
            "Querying {} / {}.",
            ((i + 1) * CHUNK_SIZE).min(hashes.len()),
            hashes.len()
        );
        responses.extend(filesearch(fossology, batch, None)?);
    }

    Ok(responses)
}
