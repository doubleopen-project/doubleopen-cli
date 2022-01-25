// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! Double Open Command Line Utility.

use clap::{Parser, ValueHint};
use doubleopen_cli::{
    commands::upload_missing_archives_to_fossology, populate_spdx_document_from_fossology,
};
use env_logger::Env;
use fossology_rs::Fossology;
use spdx_rs::{license_list::LicenseList, SPDX};
use std::path::PathBuf;

/// Command line options.
#[derive(Parser, Debug)]
#[clap(author, about, version)]
struct Opts {
    /// Subcommand to run.
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Interact with Fossology and process SPDX document with the data.
#[derive(Parser, Debug)]
enum SubCommand {
    /// Interact with Fossology.
    #[clap(author, version)]
    Fossology(FossologyArguments),
}

/// Arguments for the Fossology subcommand.
#[derive(Parser, Debug)]
struct FossologyArguments {
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
    Upload {
        /// Source archives to upload to Fossology. Use pattern matching to upload multiple files.
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
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
    },

    /// Populate an SPDX file with license and copyritght information from Fossology.
    Query {
        /// Path to the input SPDX.
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        input: PathBuf,

        /// Path to output the populated SPDX document to.
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    // Initialize logging.
    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    // Get the command line arguments.
    let opts: Opts = Opts::parse();

    // Process subcommands.
    match opts.subcmd {
        // Process Fossology subcommand.
        SubCommand::Fossology(fossology_arguments) => {
            let fossology = Fossology::new(&fossology_arguments.uri, &fossology_arguments.token)?;

            match fossology_arguments.action {
                // Process upload subcommand of Fossology.
                FossologyAction::Upload {
                    source_archive_paths,
                    folder,
                    spdx,
                    dry_run,
                } => {
                    let spdx = SPDX::from_file(&spdx)?;
                    upload_missing_archives_to_fossology(
                        source_archive_paths,
                        &fossology,
                        &folder,
                        &spdx.package_information,
                        dry_run,
                    )?;
                }

                // Process query subcommand of Fossology.
                FossologyAction::Query { input, output } => {
                    let mut spdx = SPDX::from_file(&input)?;
                    let license_list = LicenseList::from_github()?;
                    populate_spdx_document_from_fossology(&fossology, &mut spdx, &license_list)?;
                    spdx.save_as_json(&output)?;
                }
            }
        }
    }
    Ok(())
}
