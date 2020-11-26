// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

pub mod yocto;

#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
  #[error(transparent)]
  FileError(#[from] std::io::Error),

  #[error("parsing failed")]
  ParseError(String),
}