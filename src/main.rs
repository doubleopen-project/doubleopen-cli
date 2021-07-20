// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! Double Open Command Line Utility.

use clap::{Clap, ValueHint};
use commands::upload_missing_archives_to_fossology;
use doubleopen_cli::populate_spdx_document_from_fossology;
use fossology_rs::Fossology;
use spdx_rs::{license_list::LicenseList, SPDX};
use std::path::PathBuf;

// use policy_engine::PolicyEngine;
mod commands;
mod utilities;

/// Command line options.
#[derive(Clap, Debug)]
#[clap(author, about, version)]
struct Opts {
    /// Subcommand to run.
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Interact with Fossology and process SPDX document with the data.
#[derive(Clap, Debug)]
enum SubCommand {
    /// Interact with Fossology.
    #[clap(author, version)]
    Fossology(FossologyArguments),
}

/// Arguments for the Fossology subcommand.
#[derive(Clap, Debug)]
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
#[derive(Clap, Debug)]
enum FossologyAction {
    /// Upload source archives in a directory to Fossology.
    Upload {
        /// Directory containing the source archives to upload.
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        source_dir_path: Vec<PathBuf>,

        /// ID of the folder in Fossology to upload the source to.
        #[clap(short, long)]
        folder: i32,
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

fn main() {
    // Initialize logging.
    env_logger::init();

    // Get the command line arguments.
    let opts: Opts = Opts::parse();

    // Process subcommands.
    match opts.subcmd {
        // Process Fossology subcommand.
        SubCommand::Fossology(fossology_arguments) => match fossology_arguments.action {
            // Process upload subcommand of Fossology.
            FossologyAction::Upload {
                source_dir_path,
                folder,
            } => {
                let fossology =
                    Fossology::new(&fossology_arguments.uri, &fossology_arguments.token);

                upload_missing_archives_to_fossology(source_dir_path, &fossology, &folder)
                    .expect("upload to work");
            }

            // Process query subcommand of Fossology.
            FossologyAction::Query { input, output } => {
                let mut spdx = SPDX::from_file(&input).unwrap();
                let fossology =
                    Fossology::new(&fossology_arguments.uri, &fossology_arguments.token);
                let license_list = LicenseList::from_github().unwrap();
                populate_spdx_document_from_fossology(&fossology, &mut spdx, &license_list)
                    .unwrap();
                spdx.save_as_json(&output).unwrap();
            }
        },
    }
}
