// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

//! # Yocto
//!
//! Module for analyzing builds produced by the
//! [Yocto build system](https://www.yoctoproject.org/). Heavily utilizes
//! development tools included with yocto by calling them as external processes,
//! so sourcing the build environment before analysis is required.

use log::{debug, info};
use rayon::prelude::*;
use std::{fs::read_to_string, path::Path, path::PathBuf};
use {
    fossology::Fossology,
    spdx::{
        relationship::Relationship, Algorithm, Checksum, FileInformation, PackageInformation,
        RelationshipType, SPDX,
    },
};

use self::recipe::Recipe;

use super::{AnalyzerError, Package};

mod recipe;

/// Representation of a Yocto image and its source. Includes all the relevant
/// information to store the bill of materials in an SPDX document.
#[derive(Debug)]
pub struct Yocto {
    pub image_name: String,
    pub architecture: String,
    pub build_directory: PathBuf,
    pub manifest_path: PathBuf,
    pub packages: Vec<Package>,
}

impl Yocto {
    /// Create a new Yocto build.
    ///
    /// Analyzing a Yocto build required access to Yocto developer tools, so the analyzer
    /// should be run with the build environment activated, `source oe-init-build-env` by
    /// default.
    ///
    /// Takes Yocto's build directory (`build/` by default) and the manifest file
    /// (`build/tmp/deploy/images/ARCH/IMAGE_NAME.manifest` as default) as arguments.
    pub fn new<P: AsRef<Path>>(
        build_directory: P,
        manifest_path: P,
    ) -> Result<Self, AnalyzerError> {
        info!(
            "Analyzing Yocto from {} and {}",
            &build_directory.as_ref().display(),
            &manifest_path.as_ref().display()
        );
        // Use file name of manifest as the name of the image.
        let image_name = manifest_path
            .as_ref()
            .file_stem()
            .ok_or_else(|| AnalyzerError::ParseError("No manifest file name.".into()))?
            .to_owned()
            .into_string()
            .map_err(|_| AnalyzerError::ParseError("test".into()))?;

        debug!("Analyzing image name {}", &image_name);

        // Get architecture from the folder path relative to the manifest file.
        let architecture = manifest_path
            .as_ref()
            .components()
            .rev()
            .nth(1)
            .ok_or_else(|| AnalyzerError::ParseError("No architecture in path.".into()))?
            .as_os_str()
            .to_str()
            .ok_or_else(|| AnalyzerError::ParseError("No architecture in path.".into()))?
            .to_string();

        let yocto = Self {
            architecture,
            image_name,
            build_directory: build_directory.as_ref().to_path_buf(),
            manifest_path: manifest_path.as_ref().to_path_buf(),
            ..Default::default()
        };

        debug!("Created Yocto build {}.", &yocto.image_name);
        Ok(yocto)
    }

    /// Analyze the Yocto build.
    ///
    /// Analyzing a Yocto build required access to Yocto developer tools, so the analyzer
    /// should be run with the build environment activated, `source oe-init-build-end` by
    /// default.
    pub fn analyze(&mut self) -> Result<(), AnalyzerError> {
        info!("Analyzing Yocto build {}", self.image_name);

        let recipes = self.recipes()?;
        let packages = recipes
            .par_iter()
            .map(|recipe| recipe.analyze_source(&self.build_directory, &self.pkgdata_path()));

        let (packages, _errors): (Vec<_>, Vec<_>) = packages.partition(Result::is_ok);
        let packages = packages.into_iter().map(Result::unwrap).collect::<Vec<_>>();
        self.packages = packages;
        Ok(())
    }

    /// Get recipes used to build the image.
    ///
    /// The manifest file contains all of the packages that are included in the image.
    /// One recipe can result in multiple packages. Find the source recipe for all of
    /// the packages and return a list of unique recipes.
    pub fn recipes(&self) -> Result<Vec<Recipe>, AnalyzerError> {
        info!("Analyzing recipes for Yocto build {}", &self.image_name);

        let image_manifest = read_to_string(&self.manifest_path).map_err(|_| {
            AnalyzerError::ParseError(format!(
                "Manifest file not found in {}",
                &self.manifest_path.display()
            ))
        })?;

        // The packages are separated by newline in the manifest file.
        let lines = image_manifest.par_lines();
        info!(
            "The manifest file includes {} packages.",
            lines.clone().count()
        );

        // Try to create recipes from the manifest lines.
        let recipes = lines
            .map(|line| Recipe::try_from_manifest_line(&line))
            .collect::<Vec<_>>();
        info!(
            "Image {} contains {} packages.",
            &self.image_name,
            recipes.len()
        );

        // Separate recipes and errors.
        let (recipes, _errors): (Vec<_>, Vec<_>) = recipes.into_iter().partition(Result::is_ok);
        let mut recipes: Vec<_> = recipes.into_iter().map(Result::unwrap).collect();

        // Sort and remove duplicates
        recipes.sort_unstable_by_key(|recipe| recipe.name.clone());
        recipes.dedup();
        info!(
            "Image {} was built from {} recipes.",
            &self.image_name,
            recipes.len()
        );

        Ok(recipes)
    }

    /// Get the path to the pkgdata directory.
    pub fn pkgdata_path(&self) -> PathBuf {
        let mut tmp_directory = self.manifest_path.clone();
        tmp_directory.pop();
        tmp_directory.pop();
        tmp_directory.pop();
        tmp_directory.pop();
        let pkgdata = tmp_directory
            .join("pkgdata/")
            .join(self.architecture.clone());
        debug!("Packagedata: {:?}", pkgdata);
        pkgdata
    }

    /// Upload the source code of the image to Fossology.
    pub fn upload_source_to_fossology(
        &self,
        fossology: &Fossology,
        folder_id: &i32,
    ) -> Result<(), AnalyzerError> {
        debug!(
            "Uploading source of Yocto build {} to Fossology.",
            &self.image_name
        );

        let recipes = self.recipes()?;

        debug!(
            "Yocto build {} includes {} recipes.",
            &self.image_name,
            &recipes.len()
        );

        let _results = recipes
            .iter()
            .map(|recipe| -> Result<(), AnalyzerError> {
                recipe.upload_recipe_source_to_fossology(&self, &fossology, &folder_id)
            })
            .collect::<Vec<_>>();

        Ok(())
    }
}

impl Default for Yocto {
    fn default() -> Self {
        Self {
            architecture: "DEFAULT".into(),
            build_directory: "DEFAULT".into(),
            image_name: "DEFAULT".into(),
            manifest_path: "DEFAULT".into(),
            packages: Vec::new(),
        }
    }
}

impl From<Yocto> for SPDX {
    /// Create an SPDX document from a Yocto build.
    fn from(yocto: Yocto) -> SPDX {
        let mut spdx = SPDX::new(&yocto.image_name);

        // Add all packages used in the image to the SPDX.
        for package in yocto.packages {
            let mut spdx_package =
                PackageInformation::new(&package.name, &mut spdx.spdx_ref_counter);
            spdx_package.package_version = Some(package.version);

            // For each package, add their source files to the SPDX.
            for file in package.source_files {
                let mut spdx_file = FileInformation::new(&file.name, &mut spdx.spdx_ref_counter);
                spdx_file
                    .file_checksum
                    .push(Checksum::new(Algorithm::SHA256, &file.sha256));

                // Add the correct relationship to connect the package and the source
                // file in the SPDX. Differentiate between files used in the build
                // and files not used.
                let relationship = match file.used_in_build {
                    true => Relationship::new(
                        &spdx_package.package_spdx_identifier,
                        &spdx_file.file_spdx_identifier,
                        // TODO: Not sure if the correct relationship.
                        RelationshipType::Contains,
                        None,
                    ),
                    false => Relationship::new(
                        &spdx_package.package_spdx_identifier,
                        &spdx_file.file_spdx_identifier,
                        // TODO: Not sure if the correct relationship.
                        RelationshipType::Other,
                        None,
                    ),
                };
                spdx.file_information.push(spdx_file);
                spdx.relationships.push(relationship);
            }

            spdx.package_information.push(spdx_package)
        }

        spdx
    }
}
