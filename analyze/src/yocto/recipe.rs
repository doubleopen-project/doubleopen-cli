// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, fs::read_to_string, path::Path, process::Command};

use flate2::{write::GzEncoder, Compression};
use log::{debug, error, info};
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;

use {
    crate::{AnalyzerError, Package, SourceFile},
    fossology::Fossology,
    utilities::{hash256_for_path, is_hidden},
};

use crate::Binary;

use super::Yocto;

/// Representation of a recipe in Yocto.
#[derive(PartialEq, Debug)]
pub struct Recipe {
    /// Name of the recipe.
    pub name: String,

    /// Name of the recipe.
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

    /// Analyze the source of the recipe and outputs a [Package](crate::analyze::Package).
    ///
    /// Gets the source with Yocto's `devtool`.
    pub fn analyze_source<P: AsRef<Path>>(
        &self,
        build_directory: P,
        pkgdata_path: P,
    ) -> Result<Package, AnalyzerError> {
        debug!("Analyzing source for recipe {}.", &self.name);
        let tempdir = self.get_recipe_source(&build_directory)?;

        // Find the srclist file for the package.
        debug!("Searching for srclist for {}", &self.name);
        let srclist = pkgdata_path.as_ref().read_dir()?.find(|entry| {
            let path = entry.as_ref().expect("Should always exist").path();
            let stem = path.file_stem();
            let extension = path.extension();

            // Find the srclist by checking for files with a stem of the package name an extension of `.srclist`.
            match stem {
                Some(stem) => {
                    if stem.to_str().expect("Conversion should work") == self.name {
                        match extension {
                            Some(extension) => {
                                return extension.to_str().expect("Conversion should work.")
                                    == "srclist"
                            }
                            None => false,
                        }
                    } else {
                        false
                    }
                }
                None => false,
            }
        });

        let mut binaries: Vec<Binary> = Vec::new();

        // If an srclist file was found for the recipe, get the hashes of the files
        // that were used to build the binaries according to the debug utility.
        let used_hashes = match srclist {
            Some(srclist) => {
                let srclist = srclist.expect("Should always unwrap.");
                let srclist_content = read_to_string(srclist.path()).expect("Should always exist.");
                let used_hashes = Recipe::process_srclist_content(&srclist_content);
                debug!("Found hashlist for {}.", self.name);
                binaries = used_hashes.1;
                Some(used_hashes.0)
            }
            None => {
                debug!("Did not find srclist for {}.", self.name);
                None
            }
        };

        debug!("Creating source files for {}.", &self.name);

        // Create a SourceFile for the source files of the recipe.
        let source_files: Vec<SourceFile> = WalkDir::new(&tempdir.path())
            .follow_links(true)
            // Jump over the first level, as it's the tempdir itself.
            .min_depth(1)
            .into_iter()
            // Filter hidden files to exclude `.git/`.
            // TODO: May make more sense and be a little safer to specifically skip
            // problematic folders, such as `.git/`.
            .filter_entry(|entry| !is_hidden(entry))
            .filter_map(|f| {
                let entry = f;
                match entry {
                    Ok(entry) => {
                        // Only add files, not folders.
                        if entry.metadata().unwrap().is_file() {
                            // Get the path relative to the root of the source package.
                            let filename = entry
                                .path()
                                .strip_prefix(&tempdir.path())
                                .expect("Should always be extracted here.")
                                .to_string_lossy();

                            // Calculate the SHA256 checksum for the source file.
                            let sha256 = hash256_for_path(entry.path()).to_ascii_lowercase();

                            // If a srclist with a list of used files was found, try to find the
                            // checksum of the source file in it. If not found, mark the file as not
                            // used in build.
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
                            "{}: Error opening an entry in {}: {:?}",
                            &self.name,
                            tempdir.path().display(),
                            entry,
                        );
                        None
                    }
                }
            })
            .collect();

        let used_source_files_count = &source_files
            .iter()
            .filter(|source_file| source_file.used_in_build)
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
            binaries,
        })
    }

    /// Get the source for the recipe with Yocto's `devtool extract` and save it in
    /// a temporary directory for analysis.
    pub fn get_recipe_source<P: AsRef<Path>>(
        &self,
        build_directory: P,
    ) -> Result<TempDir, AnalyzerError> {
        info!("Extracting source for {}", &self.name);

        debug!("Running devtool extract for {}.", &self.name);
        let mut command = Command::new("devtool");
        command.current_dir(build_directory.as_ref());
        let tempdir = TempDir::new()?;

        let output = command
            .arg("extract")
            .arg(&self.name)
            .arg(tempdir.path())
            .output()?;

        if output.status.success() {
            debug!(
                "Devtool extract done, saved source of {} to {}.",
                &self.name,
                &tempdir.path().display()
            );
        } else {
            error!(
                "Devtool extract failed for {} with error: {:?}",
                &self.name,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Remove git directories from source, as devtool extract uses git and writes
        // new metadata. With the git directories included the extraction is not deterministic,
        // leading into different hashe for every extraction.
        for entry in WalkDir::new(&tempdir.path()).into_iter() {
            let entry = entry?;
            if entry.file_name().to_str().expect("Should always convert") == ".git"
                && entry.file_type().is_dir()
            {
                debug!("Deleting {}", entry.path().display());
                std::fs::remove_dir_all(entry.path()).expect("Should not error");
            }
        }

        Ok(tempdir)
    }

    /// Uploads source archive for the Yocto recipe to Fossology. Does not upload
    /// if an archive with the same hash has already been uploaded.
    pub fn upload_recipe_source_to_fossology(
        &self,
        yocto: &Yocto,
        fossology: &Fossology,
        folder_id: &i32,
    ) -> Result<(), AnalyzerError> {
        info!("Uploading source of recipe {} to Fossology.", &self.name);

        // Get a temporary directory with the source of the recipe.
        let source_directory = self.get_recipe_source(&yocto.build_directory)?;

        // Get the amount of files in the source directory.
        // TODO: Might want to filter if no files in source?
        let source_len = WalkDir::new(&source_directory).into_iter().count();
        debug!("{} includes {} files", &self.name, source_len);

        // Create a temporary directory for the archive and create an archive of
        // the source.
        let tempdir = tempdir()?;
        debug!("Created a temporary directory.");
        let tar_gz = std::fs::File::create(
            &tempdir
                .path()
                .join(format!("{}-{}.tar.gz", &self.name, &self.version)),
        )?;
        debug!("Created a tar gz.");
        let enc = GzEncoder::new(tar_gz, Compression::default());
        debug!("Created an encoder");
        let mut tar = tar::Builder::new(enc);
        // Deterministic mode required to produce same hash value.
        tar.mode(tar::HeaderMode::Deterministic);
        debug!("Created a tar builder.");
        tar.follow_symlinks(false);
        tar.append_dir_all("", &source_directory.path())?;
        tar.into_inner()?;
        debug!("Added files to tar");

        // Get sha256 value for the source archive.
        debug!("Calculating hash");
        let sha_256 = hash256_for_path(
            &tempdir
                .path()
                .join(format!("{}-{}.tar.gz", &self.name, &self.version)),
        );
        debug!("SHA256 for {} is {}.", self.name, sha_256);

        // If the source archive does not exist in Fossology based on the sha256
        // value, upload it. Don't upload duplicates.
        let exists = fossology.file_exists(&sha_256).unwrap();
        if !exists {
            info!("{} was not found on Fossology, uploading now.", self.name);
            fossology.upload(
                &tempdir
                    .path()
                    .join(format!("{}-{}.tar.gz", &self.name, &self.version)),
                &folder_id,
            );
        } else {
            info!(
                "{} was already found on Fossology, did not upload it again.",
                self.name
            );
        }

        Ok(())
    }

    /// Process srclist file content.
    /// The format of the srclist files is a little sketchy.
    ///
    /// Returns a vector of binary names with a related vector of source file hashes for each
    /// binary.
    pub fn process_srclist_content(srclist_content: &str) -> (Vec<String>, Vec<Binary>) {
        let srclist: HashMap<String, Vec<HashMap<String, Option<String>>>> =
            serde_json::from_str(&srclist_content).unwrap();

        let mut hashes: Vec<String> = Vec::new();
        let mut binaries: Vec<Binary> = Vec::new();

        for i in srclist {
            let mut binary: Binary = Binary {
                name: i.0,
                source_hashes: Vec::new(),
            };
            for elf_file in i.1 {
                for source_file in elf_file {
                    if let Some(value) = source_file.1 {
                        hashes.push(value.to_ascii_lowercase());
                        binary.source_hashes.push(value.to_ascii_lowercase());
                    }
                }
            }
            binaries.push(binary);
        }
        hashes.sort();
        hashes.dedup();
        (hashes, binaries)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn srclist_content_is_correctly_parsed() {
        let srclist_content = r#"
{
  "/home/mikko/doubleopen/poky/build/tmp/work/core2-64-poky-linux/mtdev/1.1.6-r0/package/usr/bin/mtdev-test": [
    {
      "/usr/src/debug/glibc/2.32-r0/git/sysdeps/x86_64/crti.S": null
    },
    {
      "/usr/src/debug/mtdev/1.1.6-r0/mtdev-1.1.6/test/mtdev-test.c": "89c8c3d10e5ae73055d991970efe34745bdb618027c668b46eb50ef07ec8725e"
    },
    {
      "/usr/src/debug/mtdev/1.1.6-r0/build/test/<built-in>": null
    } 
  ],
  "/home/mikko/doubleopen/poky/build/tmp/work/core2-64-poky-linux/mtdev/1.1.6-r0/package/usr/lib/libmtdev.so.1.0.0": [
    {
      "/usr/src/debug/glibc/2.32-r0/git/sysdeps/x86_64/crti.S": null
    },
    {
      "/usr/src/debug/mtdev/1.1.6-r0/mtdev-1.1.6/src/caps.c": "bead9a7a2e39e86804afd7129c71ced1835fda67c891a19ec4ee75ec04632cd7"
    },
    {
      "/usr/src/debug/mtdev/1.1.6-r0/mtdev-1.1.6/src/common.h": "b19da05381db8cf58a9ed6b36fb6fddec2ec64554cc21bd0e68c8477acd988a5"
    }
  ]
}
        "#;

        let binaries = Recipe::process_srclist_content(srclist_content);

        assert_eq!(binaries.0.len(), 3);
        assert_eq!(binaries.1.len(), 2);
    }
}
