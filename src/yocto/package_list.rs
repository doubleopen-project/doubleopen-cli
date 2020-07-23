// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PackageList {
    pub name: String,
    pub path: PathBuf,
    pub srclist: Option<PathBuf>,
    pub packages: Vec<String>,
    pub elf_files: Vec<ElfFile>,
}

impl PackageList {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            srclist: None,
            packages: vec![],
            elf_files: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ElfFile {
    pub path: String,
    pub source_files: Vec<SourceFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceFile {
    pub path: String,
    pub sha256: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::PackageList;

    #[test]
    fn package_list_is_created() {
        let expected_package_list = PackageList {
            name: "test".into(),
            path: "test".into(),
            srclist: None,
            packages: Vec::new(),
            elf_files: Vec::new(),
        };

        let package_list = PackageList::new("test".into(), "test".into());

        assert_eq!(expected_package_list, package_list);
    }
}
