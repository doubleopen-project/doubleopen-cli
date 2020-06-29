// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use clap::{app_from_crate, Arg};
use std::fs;
mod manifest;
mod srclist;
use indicatif::ProgressBar;

fn main() {
    let matches = app_from_crate!()
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
        )
        .get_matches();

    let mut packages = manifest::process_manifest(matches.value_of("manifest").unwrap());

    let package_lists = srclist::process_srclists(matches.value_of("srclist folder").unwrap());

    let packages_count = packages.len() as u64;
    println!("Finding srclists for packages...");
    let pb = ProgressBar::new(packages_count);
    for e in packages.iter_mut() {
        pb.inc(1);
        e.find_srclist(&package_lists);
    }
    pb.finish_with_message("done");

    if let Some(ref file) = matches.value_of("save to file") {
        println!("Saving to json...");
        let json = serde_json::to_string_pretty(&packages).unwrap();

        fs::write(file, json).expect("Unable to write file");
    }
}
