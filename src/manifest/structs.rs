// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use crate::package_list::structs::PackageList;
use serde::Serialize;

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
