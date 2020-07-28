// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use clap::{app_from_crate, App, Arg};
use std::fs;
mod fossology;
mod yocto;
use indicatif::ProgressBar;
use spdx::spdx::PackageInformation;
use yocto::{package_list::PackageList, srclist, Package};
mod spdx;
mod utilities;

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
                ),
        ])
        .get_matches();

    let mut packages: Vec<Package>;
    let packages_count: u64;
    let package_lists: Vec<PackageList>;

    // Process analyze subcommand.
    if let Some(ref matches) = matches.subcommand_matches("analyze") {
        if let (Some(manifest_path), Some(srclists_path)) = (
            matches.value_of("manifest"),
            matches.value_of("srclist folder"),
        ) {
            packages = Package::new(manifest_path);
            packages_count = packages.len() as u64;
            package_lists = srclist::process_srclists(srclists_path);
            println!("Finding srclists for packages...");
            let pb = ProgressBar::new(packages_count);
            for e in packages.iter_mut() {
                pb.inc(1);
                e.find_srclist(&package_lists);
            }
            pb.finish_with_message("done");

            let mut spdx = spdx::spdx::SPDX::new("Yocto");
            spdx.package_information = PackageInformation::from_yocto_packages(&packages);

            let hashes = spdx.get_unique_hashes();

            // Output to JSON
            if let Some(ref file) = matches.value_of("save to file") {
                println!("Saving to json...");
                let json = serde_json::to_string_pretty(&spdx).unwrap();

                fs::write(file, json).expect("Unable to write file");
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
    }
}
