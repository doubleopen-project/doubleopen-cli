// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use analyze::yocto::Yocto;
use clap::{Clap, ValueHint};
use fossology::Fossology;
use std::path::PathBuf;
use spdx::SPDX;

use policy_engine::policy::Policy;
use policy_engine::PolicyEngine;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Analyze project.
    #[clap(author, version)]
    Analyze(AnalyzeArguments),

    /// Use Fossology.
    #[clap(author, version)]
    Fossology(FossologyArguments),

    /// Evaluate SPDX against a Policy.
    #[clap(author, version)]
    Evaluate(EvaluateArguments),
}

#[derive(Clap, Debug)]
struct AnalyzeArguments {
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    manifest: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
    build: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
    output: PathBuf,
}

#[derive(Clap, Debug)]
struct FossologyArguments {
    #[clap(short, long, value_hint = ValueHint::Url)]
    uri: String,

    #[clap(short, long)]
    token: String,

    #[clap(subcommand)]
    action: FossologyAction,
}

#[derive(Clap, Debug)]
enum FossologyAction {
    /// Upload the source for a Yocto build to Fossology.
    Upload {
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        manifest: PathBuf,

        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
        build: PathBuf,

        /// ID of the folder in Fossology to upload the source to.
        #[clap(short, long)]
        folder: i32,
    },

    /// Populate an SPDX file with license information from Fossology.
    Query {
        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        input: PathBuf,

        #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
        output: PathBuf,
    },
}

#[derive(Clap, Debug)]
struct EvaluateArguments {
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    spdx: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    policies: Vec<PathBuf>,

    #[clap(short, long)]
    context: String,
}
fn main() {
    env_logger::init();
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Analyze(analyze_arguments) => {
            // TODO: Don't unwrap.
            let mut yocto_build =
                Yocto::new(&analyze_arguments.build, &analyze_arguments.manifest).unwrap();
            yocto_build.analyze().unwrap();
            let spdx: SPDX = yocto_build.into();
            spdx.save_as_json(&analyze_arguments.output);
        }
        SubCommand::Fossology(fossology_arguments) => match fossology_arguments.action {
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
            FossologyAction::Query { input, output } => {
                let mut spdx = SPDX::from_file(&input);
                let fossology =
                    Fossology::new(&fossology_arguments.uri, &fossology_arguments.token);
                spdx.query_fossology_for_licenses(&fossology).unwrap();
                spdx.save_as_json(&output);
            }
        },
        SubCommand::Evaluate(evaluate_arguments) => {
            let policy =
                Policy::from_files(evaluate_arguments.policies, &evaluate_arguments.context);
            let policy_engine = PolicyEngine::new(policy);

            let spdx = SPDX::from_file(&evaluate_arguments.spdx);

            let _result = policy_engine.evaluate_spdx(&spdx);
        }
    }
}
