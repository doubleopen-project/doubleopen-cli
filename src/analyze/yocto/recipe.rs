use std::{collections::HashMap, fs::read_to_string, path::Path, process::Command};

use log::{debug, error};
use tempfile::TempDir;
use walkdir::WalkDir;

use crate::{
    analyze::{AnalyzerError, Package, SourceFile},
    utilities::{hash256_for_path, is_hidden},
};

#[derive(PartialEq)]
pub struct Recipe {
    pub name: String,
    pub version: String,
}

impl Recipe {
    /// Try to get a recipe for a package in manifest file.
    ///
    /// Manifest file includes a list of packages like `dbus-1 core2_64 1.12.16`,
    /// where the first element is the package name, second is the package architecture
    /// and the third is the package version.
    pub fn try_from_manifest_line(manifest_line: &str) -> Result<Self, AnalyzerError> {
        debug!("Parsing line {} from manifest file.", &manifest_line);

        // The elements of the package are separated by whitespace.
        let mut split = manifest_line.split_whitespace();

        // The package name is the first element of the line.
        let package_name = split
            .next()
            // TODO: Improve error.
            .ok_or_else(|| AnalyzerError::ParseError(manifest_line.into()))?
            .to_string();

        // Use Yocto's `oe-pkgdata-util` to determine the source recipe for the
        // package. This requires the utility to be in path, which is provided
        // by sourcing `oe-init-build-env`.
        let output = String::from_utf8(
            Command::new("oe-pkgdata-util")
                .arg("package-info")
                .arg(&package_name)
                .output()
                .expect("Error using 'oe-pkgdata-util'. Did you execute 'source oe-init-build-env'")
                .stdout,
        )
        .expect("Failed converting the output of 'oe-pkgdata-util' to a String.");

        let output = output.trim();

        // The output of `oe-pkgdata-util package-info` is separated by whitespace.
        let mut output = output.split_whitespace();

        // The name of the recipe is the 3rd and the version is the 4th element of the output.
        let recipe_name = output.nth(2);
        let recipe_version = output.next();

        // If the output included both the name and the version of the recipe, create the
        // Recipe struct.
        if let (Some(recipe_name), Some(recipe_version)) = (recipe_name, recipe_version) {
            debug!(
                "Recipe for {} is {} {}.",
                &package_name, &recipe_name, &recipe_version
            );
            Ok(Self {
                name: recipe_name.into(),
                version: recipe_version.into(),
            })
        } else {
            error!("No recipe for {}", &package_name);
            Err(AnalyzerError::ParseError(format!(
                "No recipe for {}",
                &package_name
            )))
        }
    }

    /// Analyze the source of the recipe and outputs a [Package](analyze::Package).
    ///
    /// Gets the source with Yocto's `devtool`.
    pub fn analyze_source<P: AsRef<Path>>(
        &self,
        build_directory: P,
        pkgdata_path: P,
    ) -> Result<Package, AnalyzerError> {
        debug!("Analyzing source for recipe {}.", &self.name);

        // Get the source for the recipe with Yocto's `devtool extract` and save it in
        // a temporary directory for analysis.
        debug!("Running devtool extract for {}.", &self.name);
        let mut command = Command::new("devtool");
        command.current_dir(build_directory.as_ref());
        let tempdir = TempDir::new()?;
        command
            .arg("extract")
            .arg(&self.name)
            .arg(tempdir.path())
            .output()?;
        debug!(
            "Devtool extract done, saved source of {} to{}.",
            &self.name,
            &tempdir.path().display()
        );

        // Find the srclist file for the package.
        debug!("Searching for srclist for {}", &self.name);
        let srclist = pkgdata_path.as_ref().read_dir()?.find(|entry| {
            let path = entry.as_ref().expect("Should always exist").path();
            let stem = path.file_stem();
            let extension = path.extension();

            match stem {
                Some(stem) => {
                    if stem.to_str().expect("Conversion should work") == self.name {
                        match extension {
                            Some(extension) => {
                                if extension.to_str().expect("Conversion should work.") == "srclist"
                                {
                                    return true;
                                } else {
                                    return false;
                                }
                            }
                            None => return false,
                        }
                    } else {
                        return false;
                    }
                }
                None => return false,
            }
        });

        let used_hashes = match srclist {
            Some(srclist) => {
                let srclist = srclist.expect("Should always unwrap.");
                let srclist_content = read_to_string(srclist.path()).expect("Should always exist.");
                let srclist: HashMap<String, Vec<HashMap<String, Option<String>>>> =
                    serde_json::from_str(&srclist_content).unwrap();

                let mut hashes: Vec<String> = Vec::new();

                for i in srclist {
                    for elf_file in i.1 {
                        for source_file in elf_file {
                            if let Some(value) = source_file.1 {
                                hashes.push(value.to_ascii_lowercase());
                            }
                        }
                    }
                }
                hashes.sort();
                hashes.dedup();
                debug!("Found hashlist for {}.", self.name);
                Some(hashes)
            }
            None => {
                debug!("Did not find srclist for {}.", self.name);
                None
            }
        };

        // Create a SourceFile for the source files of the recipe.
        debug!("Creating source files for {}.", &self.name);

        let source_files: Vec<SourceFile> = WalkDir::new(&tempdir.path())
            .min_depth(1)
            .into_iter()
            .filter_entry(|entry| !is_hidden(entry))
            .filter_map(|f| {
                let entry = f;
                match entry {
                    Ok(entry) => {
                        if entry.metadata().unwrap().is_file() {
                            let filename = entry
                                .path()
                                .strip_prefix(&tempdir.path())
                                .expect("Should always be extracted here.")
                                .to_string_lossy();
                            let sha256 = hash256_for_path(entry.path()).to_ascii_lowercase();
                            match &used_hashes {
                                Some(hashlist) => {
                                    let used_in_build = hashlist.binary_search(&sha256).is_ok();
                                    Some(SourceFile {
                                        name: filename.to_string(),
                                        sha256,
                                        used_in_build,
                                    })
                                }
                                None => Some(SourceFile {
                                    name: filename.to_string(),
                                    sha256,
                                    used_in_build: true,
                                }),
                            }
                        } else {
                            None
                        }
                    }
                    Err(_) => {
                        error!(
                            "No source for {} at {}",
                            &self.name,
                            tempdir.path().display()
                        );
                        None
                    }
                }
            })
            .collect();

        let used_source_files_count = &source_files
            .iter()
            .filter(|source_file| source_file.used_in_build == true)
            .count();

        debug!(
            "Found {} source files of which {} are used for recipe {}.",
            source_files.len(),
            used_source_files_count,
            &self.name
        );
        Ok(Package {
            name: self.name.clone(),
            version: self.version.clone(),
            source_files,
        })
    }
}
