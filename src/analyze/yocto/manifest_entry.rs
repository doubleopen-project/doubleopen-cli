use std::path::Path;

use crate::analyze::AnalyzerError;

use super::runtime_reverse::RuntimeReverse;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ManifestEntry {
    pub package_name: String,
    pub architecture: String,
    pub version: String,
    pub runtime_reverse: RuntimeReverse,
}

impl ManifestEntry {
    pub fn new<P: AsRef<Path>>(line: &str, runtime_reverse_path: P) -> Result<Self, AnalyzerError> {
        let mut split = line.split_whitespace();
        let package_name = split
            .next()
            .ok_or_else(|| AnalyzerError::ParseError(line.into()))?
            .to_string();

        let architecture = split
            .next()
            .ok_or_else(|| AnalyzerError::ParseError(line.into()))?
            .to_string();

        let version = split
            .next()
            .ok_or_else(|| AnalyzerError::ParseError(line.into()))?
            .to_string();

        let runtime_reverse_path = runtime_reverse_path.as_ref().join(&package_name);

        let runtime_reverse = RuntimeReverse::new(runtime_reverse_path)?;

        Ok(Self {
            package_name,
            architecture,
            version,
            runtime_reverse,
        })
    }
}
