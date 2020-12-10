use std::{path::Path, process::Command};

use log::{debug, error};

use crate::analyze::AnalyzerError;

use super::runtime_reverse::RuntimeReverse;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ManifestEntry {
    pub package_name: String,
    pub architecture: String,
    pub version: String,
    // pub runtime_reverse: RuntimeReverse,
    pub recipe_name: String,
    pub recipe_version: String,
}

impl ManifestEntry {
    pub fn new(line: &str) -> Result<Self, AnalyzerError> {
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

        debug!("Finding recipe for {}.", &package_name);
        let output = String::from_utf8(
            Command::new("oe-pkgdata-util")
                .arg("package-info")
                .arg(&package_name)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        let output = output.trim();

        let mut output = output.split_whitespace();

        let recipe_name = output.nth(2);
        let recipe_version = output.next();

        if let (Some(recipe_name), Some(recipe_version)) = (recipe_name, recipe_version) {
            debug!(
                "Recipe for {} is {} {}.",
                &package_name, &recipe_name, &recipe_version
            );
            Ok(Self {
                package_name,
                architecture,
                version,
                // runtime_reverse,
                recipe_name: recipe_name.into(),
                recipe_version: recipe_version.into(),
            })
        } else {
            error!("No recipe for {}", &package_name);
            Err(AnalyzerError::ParseError(format!("No recipe for {}", &package_name)))
        }

        // let runtime_reverse_path = runtime_reverse_path.as_ref().join(&package_name);

        //let runtime_reverse = RuntimeReverse::new(runtime_reverse_path)?;
    }
}
