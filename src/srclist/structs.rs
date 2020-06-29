// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileType {
    SrcList,
    PackageList,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageList {
    pub name: String,
    pub path: PathBuf,
    pub srclist: Option<PathBuf>,
    pub packages: Vec<String>,
    pub elf_files: Vec<ElfFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElfFile {
    pub path: String,
    pub source_files: Vec<SourceFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub path: String,
    pub sha256: Option<String>,
}
