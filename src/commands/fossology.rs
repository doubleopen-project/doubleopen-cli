// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use clap::{Parser, ValueHint};
use fossology_rs::Fossology;
use spdx_toolkit::license_list::LicenseList;

use crate::{
    fossology::{populate_spdx_document_from_fossology, upload_missing_archives_to_fossology},
    utilities::{deserialize_spdx, serialize_spdx},
};

/// Arguments for the Fossology subcommand.
#[derive(Parser, Debug)]
pub struct FossologyArguments {
    /// URL of the Fossology instance to use.
    /// Example: `http://localhost/repo/api/v1`.
    #[clap(short, long, value_hint = ValueHint::Url)]
    uri: String,

    /// Access token for the Fossology instance.
    #[clap(short, long)]
    token: String,

    /// Action to do with Fossology.
    #[clap(subcommand)]
    action: FossologyAction,
}

/// Sub(sub)commands for the Fossology subcommands.
#[derive(Parser, Debug)]
enum FossologyAction {
    /// Upload source archives to Fossology.
    Upload(UploadArguments),

    /// Populate an SPDX file with license and copyritght information from Fossology.
    Query(QueryArguments),
}

pub fn fossology_command(arguments: FossologyArguments) -> anyhow::Result<()> {
    let fossology = Fossology::new(&arguments.uri, &arguments.token)?;

    match arguments.action {
        // Process upload subcommand of Fossology.
        FossologyAction::Upload(arguments) => {
            upload(arguments, &fossology)?;
        }

        // Process query subcommand of Fossology.
        FossologyAction::Query(arguments) => {
            query(arguments, &fossology)?;
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
pub struct UploadArguments {
    /// Source archives to upload to Fossology. Use pattern matching to upload multiple files.
    source_archive_paths: Vec<PathBuf>,

    /// ID of the folder in Fossology to upload the source to.
    #[clap(short, long)]
    folder: i32,

    /// Path to the SPDX Document to get the packages that should be skipped. Packages with
    /// CLOSED in their declared license won't be uploaded.
    #[clap(long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    spdx: PathBuf,

    /// Don't actually upload the packages, but print packages that would be uploaded and
    /// packages that would be skipped based on having a closed license.
    #[clap(long)]
    dry_run: bool,
}

pub fn upload(arguments: UploadArguments, fossology: &Fossology) -> anyhow::Result<()> {
    let spdx = deserialize_spdx(&arguments.spdx)?;

    upload_missing_archives_to_fossology(
        arguments.source_archive_paths,
        fossology,
        &arguments.folder,
        &spdx.package_information,
        arguments.dry_run,
    )?;

    Ok(())
}

#[derive(Parser, Debug)]
pub struct QueryArguments {
    /// Path to the input SPDX.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    input: PathBuf,

    /// Path to output the populated SPDX document to.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    output: PathBuf,
}

pub fn query(arguments: QueryArguments, fossology: &Fossology) -> anyhow::Result<()> {
    let mut spdx = deserialize_spdx(&arguments.input)?;

    let license_list = LicenseList::from_github(None)?;
    populate_spdx_document_from_fossology(fossology, &mut spdx, &license_list)?;

    serialize_spdx(arguments.output, &spdx)?;

    Ok(())
}
