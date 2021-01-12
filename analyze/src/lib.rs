// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

//! # Analyzer module
//! 
//! Module for analyzing Yocto images to produce their bill of materials.

pub mod package;
pub mod yocto;
pub use package::*;

/// Error encountered during analysis.
#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
    /// Errors related to file operations.
    #[error(transparent)]
    FileError(#[from] std::io::Error),

    /// Error while parsing some input.
    #[error("parsing failed")]
    ParseError(String),

    /// Errors related to directory traversal.
    #[error(transparent)]
    DirectoryError(#[from] walkdir::Error),
}
