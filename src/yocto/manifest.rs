// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use super::package_list::PackageList;
use serde::Serialize;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Serialize)]
pub struct Package {
    pub name: String,
    pub architecture: String,
    pub version: String,
    pub package_list: Option<PackageList>,
}

impl Default for Package {
    fn default() -> Self {
        Package {
            name: String::from("DEFAULT"),
            architecture: String::from("DEFAULT"),
            version: String::from("DEFAULT"),
            package_list: None,
        }
    }
}

impl Package {
    pub fn new(manifest_path: &str) -> Vec<Package> {
        println!("Processing manifest...");
        let file = File::open(manifest_path).expect("No such file");
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
