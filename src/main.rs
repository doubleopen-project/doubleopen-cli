// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! Double Open Command Line Utility.

use clap::StructOpt;
use doubleopen_cli::commands::{main_command, Opts};
use env_logger::Env;

fn main() -> anyhow::Result<()> {
    // Initialize logging.
    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    // Get the command line arguments.
    let opts: Opts = Opts::parse();

    main_command(opts)?;

    Ok(())
}
