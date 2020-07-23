// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use super::package_list::PackageList;
use serde::Serialize;
use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Represents a package from image's manifest file.
#[derive(Debug, Serialize, PartialEq)]
pub struct Package {
    pub name: String,
    pub architecture: String,
    pub version: String,
    /// Package_list file where the package is found.
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
    /// Create a vector of packages from a manifest file.
    pub fn new(manifest_path: &str) -> Vec<Package> {
        println!("Processing manifest...");
        let file = File::open(manifest_path).expect("No such file");
        let reader = BufReader::new(file);
        let lines = reader.lines();
        let mut packages: Vec<Package> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                packages.push(Self::new_from_line(line));
            }
        }

        packages
    }

    /// Create a package from a line of manifest file.
    fn new_from_line(line: String) -> Package {
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
        package
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

#[cfg(test)]
mod tests {
    use super::Package;
    use crate::yocto::package_list::PackageList;
    use std::io::Write;

    #[test]
    fn manifest_line_is_parsed_correctly() {
        let line = "xf86-video-vesa core2_64 2.4.0".to_string();

        let expected_package = Package {
            name: "xf86-video-vesa".to_string(),
            architecture: "core2_64".to_string(),
            version: "2.4.0".to_string(),
            ..Default::default()
        };

        let created_package = Package::new_from_line(line);

        assert_eq!(expected_package, created_package);
    }

    #[test]
    fn manifest_file_is_parsed_correctly() {
        let mut manifest_file = tempfile::NamedTempFile::new().unwrap();

        writeln!(manifest_file, "package_1 core2_64 2.4.0").unwrap();
        writeln!(manifest_file, "package_2 core2_64 2.4.1").unwrap();

        let packages_expected = vec![
            Package {
                name: "package_1".to_string(),
                architecture: "core2_64".to_string(),
                version: "2.4.0".to_string(),
                ..Default::default()
            },
            Package {
                name: "package_2".to_string(),
                architecture: "core2_64".to_string(),
                version: "2.4.1".to_string(),
                ..Default::default()
            },
        ];

        let packages = Package::new(manifest_file.path().to_str().unwrap());

        assert_eq!(packages_expected, packages);
    }

    #[test]
    fn correct_package_list_is_found() {
        let package_lists = vec![
            PackageList {
                name: "Package List 1".to_string(),
                path: "path1".into(),
                srclist: Some("source/list/1".into()),
                elf_files: vec![],
                packages: vec!["xf86-video-vesa".to_string()],
            },
            PackageList {
                name: "Package List 2".to_string(),
                path: "path2".into(),
                srclist: Some("source/list/2".into()),
                elf_files: vec![],
                packages: vec!["xf86-audio-vesa".to_string()],
            },
        ];

        let mut package = Package {
            name: "xf86-video-vesa".to_string(),
            architecture: "core2_64".to_string(),
            version: "2.4.0".to_string(),
            ..Default::default()
        };

        let expected_package_with_package_list = Package {
            name: "xf86-video-vesa".to_string(),
            architecture: "core2_64".to_string(),
            version: "2.4.0".to_string(),
            package_list: Some(PackageList {
                name: "Package List 1".to_string(),
                path: "path1".into(),
                srclist: Some("source/list/1".into()),
                elf_files: vec![],
                packages: vec!["xf86-video-vesa".to_string()],
            }),
        };

        package.find_srclist(&package_lists);

        assert_eq!(package, expected_package_with_package_list);
    }
}
