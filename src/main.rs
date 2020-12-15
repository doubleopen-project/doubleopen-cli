// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use analyze::yocto::Yocto;
use clap::{Clap, ValueHint};
use std::path::PathBuf;
mod analyze;
mod fossology;
use spdx::SPDX;
mod policy_engine;
mod spdx;
mod utilities;

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
    Analyze(Analyze),

    /// Use Fossology.
    #[clap(author, version)]
    Fossology(Fossology),

    /// Evaluate SPDX against a Policy.
    #[clap(author, version)]
    Evaluate(Evaluate),
}

#[derive(Clap, Debug)]
struct Analyze {
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    manifest: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
    build: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
    output: PathBuf,
}

#[derive(Clap, Debug)]
struct Fossology {
    #[clap(short, long, value_hint = ValueHint::Url)]
    uri: String,

    #[clap(short, long)]
    token: String,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath)]
    output: PathBuf,
}

#[derive(Clap, Debug)]
struct Evaluate {
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
        SubCommand::Analyze(analyze) => {
            // TODO: Don't unwrap.
            let mut yocto_build = Yocto::new(&analyze.build, &analyze.manifest).unwrap();
            yocto_build.analyze();
            let spdx: SPDX = yocto_build.into();
            spdx.save_as_json(&analyze.output);
        }
        SubCommand::Fossology(_) => {}
        SubCommand::Evaluate(evaluate) => {
            let policy = Policy::from_files(evaluate.policies, &evaluate.context);
            let policy_engine = PolicyEngine::new(policy);

            let spdx = SPDX::from_file(&evaluate.spdx);

            let result = policy_engine.evaluate_spdx(&spdx);
        }
    }
}
