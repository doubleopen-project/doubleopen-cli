// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! Double Open Command Line Utility.

use analyze::yocto::Yocto;
use clap::{Clap, ValueHint};
use fossology::Fossology;
use notice::Notice;
use spdx::SPDX;
use std::path::PathBuf;

use policy_engine::policy::Policy;
use policy_engine::PolicyEngine;

/// Command line options.
#[derive(Clap, Debug)]
#[clap(author, about, version)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Subcommands for the CLI.
#[derive(Clap, Debug)]
enum SubCommand {
    /// Analyze a project and save the bill of materials as an SPDX document.
    #[clap(author, version)]
    Analyze(AnalyzeArguments),

    /// Use Fossology.
    #[clap(author, version)]
    Fossology(FossologyArguments),

    /// Evaluate the license compliance of an SPDX file with a provided policy.
    #[clap(author, version)]
    Evaluate(EvaluateArguments),

    /// Create a notice file from SPDX.
    #[clap(author, version)]
    Notice(NoticeArguments),
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
    /// Upload the source for a Yocto build to Fossology.
    Upload {
        /// Manifest file of the Yocto build. Default location at
        /// `build/tmp/deploy/images/<arch>/<image>.manifest`.
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        manifest: PathBuf,

        /// Build directory of the Yocto build. Default location at `build/`.
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
        build: PathBuf,

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

/// Arguments for the evaluate subcommand.
#[derive(Clap, Debug)]
struct EvaluateArguments {
    /// Path to the input SPDX.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    spdx: PathBuf,

    /// List of policies to use in the evaluation.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    policies: Vec<PathBuf>,

    /// The context from the policies to use.
    #[clap(short, long)]
    context: String,
}

/// Arguments for the notice subcommand.
#[derive(Clap, Debug)]
struct NoticeArguments {
    /// Path to the input SPDX.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    input: PathBuf,

    /// Path to output the notice file to.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    output: PathBuf,

    /// Path to a template for the notice.
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    template: PathBuf,
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
                manifest,
                build,
                folder,
            } => {
                let yocto = Yocto::new(&build, &manifest).unwrap();
                let fossology =
                    Fossology::new(&fossology_arguments.uri, &fossology_arguments.token);

                yocto
                    .upload_source_to_fossology(&fossology, &folder)
                    .unwrap();
            }

            // Process query subcommand of Fossology.
            FossologyAction::Query { input, output } => {
                let mut spdx = SPDX::from_file(&input);
                let fossology =
                    Fossology::new(&fossology_arguments.uri, &fossology_arguments.token);
                spdx.query_fossology_for_licenses(&fossology).unwrap();
                spdx.save_as_json(&output);
            }
        },

        // Process evaluate subcommand.
        SubCommand::Evaluate(evaluate_arguments) => {
            let policy =
                Policy::from_files(evaluate_arguments.policies, &evaluate_arguments.context);
            let policy_engine = PolicyEngine::new(policy);

            let spdx = SPDX::from_file(&evaluate_arguments.spdx);

            let _result = policy_engine.evaluate_spdx(&spdx);
        }

        // Process notice subcommand.
        SubCommand::Notice(notice_arguments) => {
            let spdx = SPDX::from_file(&notice_arguments.input);
            let notice = Notice::from(&spdx);
            notice
                .render_notice_to_file(&notice_arguments.template, &notice_arguments.output)
                .expect("Error rendering notice.");
        }
    }
}
