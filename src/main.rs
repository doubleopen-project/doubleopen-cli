// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use clap::{app_from_crate, App, Arg};
use std::{fs, io::BufReader};
mod analyze;
mod fossology;
use spdx::SPDX;
mod spdx;
mod utilities;
mod policy_engine;

fn main() {
    let matches = app_from_crate!()
        .subcommands(vec![
            App::new("analyze")
                .arg(
                    Arg::new("manifest")
                        .short('m')
                        .long("manifest")
                        .value_name("FILE")
                        .about("Manifest file of the image")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("srclist folder")
                        .short('f')
                        .long("folder")
                        .value_name("FOLDER")
                        .about("Folder including srclist files")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("save to file")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .about("Save package info to file")
                        .required(false)
                        .takes_value(true),
                ),
            App::new("fossology")
                .arg(
                    Arg::new("fossology")
                        .value_name("URI")
                        .about("Fossology instance")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("fossology token")
                        .long("token")
                        .short('t')
                        .value_name("TOKEN")
                        .about("Access token for Fossology API")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("upload")
                        .long("upload")
                        .short('u')
                        .value_name("DIR")
                        .about("Directory to upload to Fossology")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("spdx file")
                        .long("spdx")
                        .short('s')
                        .value_name("FILE")
                        .about("SPDX file to get licenses for (JSON).")
                        .takes_value(true),
                ),
        ])
        .get_matches();

    // Process analyze subcommand.
    if let Some(ref matches) = matches.subcommand_matches("analyze") {
        if let (Some(manifest_path), Some(srclists_path)) = (
            matches.value_of("manifest"),
            matches.value_of("srclist folder"),
        ) {
            let spdx = analyze::yocto::spdx_from_pkgdata(srclists_path, manifest_path, "Yocto");

            // Output to JSON
            if let Some(ref file) = matches.value_of("save to file") {
                spdx.save_as_json(file);
            }
        }
    }

    // Process Fossology subcommand.
    if let Some(ref matches) = matches.subcommand_matches("fossology") {
        // Setup Fossology.
        let fossology_uri = matches.value_of("fossology").unwrap();
        let token = matches.value_of("fossology token").unwrap();
        let fossology = fossology::fossology::Fossology::new(fossology_uri, token);

        fossology.version();

        // Upload package.
        if let Some(source_path) = matches.value_of("upload") {
            fossology.upload_all_in_folder(&source_path);
        }

        // Get licenses from Fossology for spdx.
        if let Some(spdx) = matches.value_of("spdx file") {
            let file = fs::File::open(&spdx).expect("SPDX file not found");
            let reader = BufReader::new(file);
            let mut spdx: SPDX = serde_json::from_reader(reader).unwrap();

            spdx.query_fossology_for_licenses(&fossology);

            spdx.save_as_json("test.spdx.json");
        }
    }
}
