// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

/// An abstraction layer between packages in different packaging systems and
/// SPDX packages. Not sure if needed, more relevant when adding support for
/// other systems than Yocto.
#[derive(Debug)]
pub struct Package {
    /// Name of the package.
    pub name: String,

    /// Version of the package.
    pub version: String,

    /// Source files inclded in the package.
    pub source_files: Vec<SourceFile>,

    /// Binaries provided by the package.
    pub binaries: Vec<Binary>,
}

/// An abstraction layer between source files in different packaging systems and
/// SPDX source files. Not sure if needed, more relevant when adding support for
/// other systems than Yocto.
#[derive(Debug)]
pub struct SourceFile {
    /// File path of the source file.
    pub name: String,

    /// SHA256 checksum of the source file.
    pub sha256: String,

    /// An indicator of whether the particular source file is actually used in the
    /// produced package. Can be produced for example with debug utility showing
    /// what source files were used to build the binaries of the packages, which
    /// should exclude e.g. tests.
    pub used_in_build: bool,
}

#[derive(Debug)]
pub struct Binary {
    pub name: String,

    pub source_hashes: Vec<String,>
}