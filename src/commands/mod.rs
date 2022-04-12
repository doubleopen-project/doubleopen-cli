// SPDX-FileCopyrightText: 2022 HH Partners
//
// SPDX-License-Identifier: MIT

mod fossology;

use clap::Parser;

use self::fossology::{fossology_command, FossologyArguments};

/// Command line options.
#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
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

pub fn main_command(opts: Opts) -> anyhow::Result<()> {
    // Process subcommands.
    match opts.subcmd {
        // Process Fossology subcommand.
        SubCommand::Fossology(arguments) => fossology_command(arguments)?,
    }

    Ok(())
}
