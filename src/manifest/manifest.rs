// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use super::structs::*;
use crate::package_list::structs::PackageList;
use std::fs::File;
use std::io::{prelude::*, BufReader};

impl Package {
    pub fn find_srclist(&mut self, package_lists: &Vec<PackageList>) {
        for ref mut package_list in package_lists {
            let packages = &package_list.packages;
            for package in packages {
                if package == &self.name {
                    self.package_list = Some(package_list.clone());
                }
            }
        }
    }
}

pub fn process_manifest(manifest: &str) -> Vec<Package> {
    println!("Processing manifest...");
    let file = File::open(manifest).expect("No such file");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut packages: Vec<Package> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let mut split = line.split_whitespace();
            let name: String = split.next().expect("error").to_string();
            let architecture: String = split.next().expect("error").to_string();
            let version: String = split.next().expect("error").to_string();
            let package: Package = Package {
                name,
                architecture,
                version,
                ..Default::default()
            };
            packages.push(package);
        }
    }

    packages
}
