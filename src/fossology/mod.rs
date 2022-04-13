// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

//! Functionality for interacting with Fossology

mod convert_licenses;
mod doubleopen_licenses;
mod populate_spdx;
mod queries;
mod upload_archives;

pub use populate_spdx::populate_spdx_document_from_fossology;
pub use upload_archives::upload_missing_archives_to_fossology;
