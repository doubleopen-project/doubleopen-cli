// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! Double Open Command Line Utility.

use analyze::{yocto::Yocto};
use clap::{Clap, ValueHint};
use fossology::Fossology;
use spdx::{license_list::LicenseList, SPDX};
use std::path::PathBuf;

// use policy_engine::PolicyEngine;

/// Command line options.
#[derive(Clap, Debug)]
#[clap(author, about, version)]
struct Opts {
    /// Subcommand to run.
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Analyze software projects for their bill of materials, get license and copyright data for them,
/// evaluate license compliance and build notice files.
#[derive(Clap, Debug)]
enum SubCommand {
    /// Analyze a Yocto project and save the bill of materials as an SPDX document.
    #[clap(author, version)]
    Analyze(AnalyzeArguments),

    /// Interact with Fossology.
    #[clap(author, version)]
    Fossology(FossologyArguments),
}

/// Arguments for the analyze subcommand.
#[derive(Clap, Debug)]
struct AnalyzeArguments {
    /// Manifest file of the Yocto build. Default location at
    /// `build/tmp/deploy/images/<arch>/<image>.manifest`.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    manifest: PathBuf,

    /// Build directory of the Yocto build. Default location at `build/`.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
    build: PathBuf,

    /// Path to output the SPDX document to.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
    output: PathBuf,
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
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
        source_dir_path: PathBuf,

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
        // Process analyze subcommand.
        SubCommand::Analyze(analyze_arguments) => {
            // TODO: Don't unwrap.
            let mut yocto_build =
                Yocto::new(&analyze_arguments.build, &analyze_arguments.manifest).unwrap();
            yocto_build.analyze().unwrap();
            let spdx: SPDX = yocto_build.into();
            spdx.save_as_json(&analyze_arguments.output);
        }

        // Process Fossology subcommand.
        SubCommand::Fossology(fossology_arguments) => match fossology_arguments.action {
            // Process upload subcommand of Fossology.
            FossologyAction::Upload {
                source_dir_path,
                folder,
            } => {
                let fossology =
                    Fossology::new(&fossology_arguments.uri, &fossology_arguments.token);

                fossology
                    .upload_files_in_dir(&source_dir_path, &folder)
                    .unwrap();
            }

            // Process query subcommand of Fossology.
            FossologyAction::Query { input, output } => {
                let mut spdx = SPDX::from_file(&input);
                let fossology =
                    Fossology::new(&fossology_arguments.uri, &fossology_arguments.token);
                let license_list = LicenseList::from_github();
                spdx.query_fossology_for_licenses(&fossology, &license_list)
                    .unwrap();
                spdx.save_as_json(&output);
            }
        },
    }
}
