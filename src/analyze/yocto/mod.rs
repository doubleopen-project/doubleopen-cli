use crate::{
    fossology::Fossology,
    spdx::{
        spdx::{Algorithm, FileInformation, PackageInformation, SPDX},
        Checksum, Relationship, RelationshipType,
    },
    utilities::hash256_for_path,
};
use flate2::{write::GzEncoder, Compression};
use log::{debug, info};
use rayon::prelude::*;
use std::{fs::read_to_string, path::Path, path::PathBuf};
use tempfile::tempdir;

use self::recipe::Recipe;

use super::{AnalyzerError, Package};

mod recipe;

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

        let (packages, errors): (Vec<_>, Vec<_>) = packages.partition(Result::is_ok);
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
        let (recipes, errors): (Vec<_>, Vec<_>) = recipes.into_iter().partition(Result::is_ok);
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

    pub fn pkgdata_path(&self) -> PathBuf {
        self.build_directory
            .join("tmp/pkgdata/")
            .join(self.architecture.clone())
    }

    pub fn upload_source_to_fossology(&self, fossology: &Fossology) -> Result<(), AnalyzerError> {
        debug!(
            "Uploading source of Yocto build {} to Fossology.",
            &self.image_name
        );

        let recipes = self.recipes()?;
        debug!(
            "Yocto build {} includes {} recipes.",
            &self.image_name,
            recipes.len()
        );

        for recipe in recipes {
            debug!("Uploading source of recipe {} to Fossology.", &recipe.name);
            let source_directory =
                recipe.get_recipe_source(&self.build_directory, &self.pkgdata_path())?;
            debug!("Creating a temporary dir");
            let tempdir = tempdir()?;
            let tar_gz = std::fs::File::create(
                &tempdir
                    .path()
                    .join(format!("{}-{}.tar.gz", &recipe.name, &recipe.version)),
            )?;
            debug!("Created a tar gz.");
            let enc = GzEncoder::new(tar_gz, Compression::default());
            debug!("Created an encoder");
            let mut tar = tar::Builder::new(enc);
            debug!("Created a tar builder.");
            tar.append_dir_all("", &source_directory.path())?;
            debug!("Added files to tar");
            tar.finish();

            debug!("Calculating hash");
            let sha256 = hash256_for_path(
                &tempdir
                    .path()
                    .join(format!("{}-{}.tar.gz", &recipe.name, &recipe.version)),
            );
            debug!("SHA256 for {} is {}.", recipe.name, sha256);

            fossology.upload(
                &tempdir
                    .path()
                    .join(format!("{}-{}.tar.gz", &recipe.name, &recipe.version)),
                &3,
            );
        }

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
    fn from(yocto: Yocto) -> SPDX {
        let mut spdx = SPDX::new(&yocto.image_name);

        for package in yocto.packages {
            let mut spdx_package =
                PackageInformation::new(&package.name, &mut spdx.spdx_ref_counter);
            spdx_package.package_version = Some(package.version);
            for file in package.source_files {
                let mut spdx_file = FileInformation::new(&file.name, &mut spdx.spdx_ref_counter);
                spdx_file
                    .file_checksum
                    .push(Checksum::new(Algorithm::SHA256, &file.sha256));

                let relationship = match file.used_in_build {
                    true => Relationship::new(
                        &spdx_package.package_spdx_identifier,
                        &spdx_file.file_spdx_identifier,
                        RelationshipType::Contains,
                        None,
                    ),
                    false => Relationship::new(
                        &spdx_package.package_spdx_identifier,
                        &spdx_file.file_spdx_identifier,
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::PathBuf;

//     #[test]
//     fn create_yocto() {
//         let mut manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         manifest_path.push("tests/examples/yocto/build/tmp/deploy/images/qemux86-64/core-image-sato-qemux86-64.manifest");

//         let mut build_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         build_path.push("tests/examples/yocto/build");

//         let yocto = Yocto::new(&build_path, &manifest_path).unwrap();

//         assert_eq!(yocto.image_name, "core-image-sato-qemux86-64");
//         assert_eq!(yocto.architecture, "qemux86-64");
//         assert!(yocto
//             .build_directory
//             .ends_with("tests/examples/yocto/build"));
//         assert_eq!(yocto.manifest_entries.len(), 5);
//         assert_eq!(
//             yocto.manifest_entries[0].runtime_reverse.name,
//             "adwaita-icon-theme".to_string()
//         );
//         assert_eq!(
//             yocto.manifest_entries[0].runtime_reverse.version,
//             "3.34.3".to_string()
//         );
//         assert_eq!(
//             yocto.manifest_entries[1].runtime_reverse.name,
//             "adwaita-icon-theme".to_string()
//         );
//         assert_eq!(
//             yocto.manifest_entries[1].runtime_reverse.version,
//             "3.34.3".to_string()
//         );
//         assert_eq!(
//             yocto.manifest_entries[2].runtime_reverse.name,
//             "alsa-utils".to_string()
//         );
//         assert_eq!(
//             yocto.manifest_entries[2].runtime_reverse.version,
//             "1.2.1".to_string()
//         );

//         assert_eq!(yocto.source_packages.len(), 3);
//         assert_eq!(yocto.source_packages[0].package_name, "adwaita-icon-theme");
//         assert_eq!(yocto.source_packages[0].source_files.len(), 3070);
//     }

//     #[test]
//     fn get_all_unique_hashes_from_srclist() {
//         let mut srclist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         srclist_path.push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/dbus.srclist");
//         let hashes = hashes_from_srclist(srclist_path);

//         // TODO: The correct amount has not been manually checked, may want to create
//         // a proper test file.
//         assert_eq!(hashes.len(), 240)
//     }

//     // #[test]
//     // fn parse_manifest_for_packages() {
//     //     let mut manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     //     manifest_path.push("tests/examples/yocto/build/tmp/deploy/images/qemux86-64/core-image-sato-qemux86-64.manifest");

//     //     let mut build_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     //     build_directory.push("tests/examples/yocto/build/");

//     //     let yocto = Yocto::new(build_directory, manifest_path).unwrap();

//     //     let expected_packages = vec![
//     //         ManifestEntry {
//     //             package_name: "adwaita-icon-theme".into(),
//     //             architecture: "noarch".into(),
//     //             version: "3.34.3".into(),
//     //             runtime_reverse: RuntimeReverse {
//     //                 name: "adwaita-icon-theme".into(),
//     //                 version: "3.34.3".into(),
//     //                 revision: "r0".into(),
//     //                 edition: None,
//     //             },
//     //         },
//     //         ManifestEntry {
//     //             package_name: "adwaita-icon-theme-symbolic".into(),
//     //             architecture: "noarch".into(),
//     //             version: "3.34.3".into(),
//     //             runtime_reverse: RuntimeReverse {
//     //                 name: "adwaita-icon-theme".into(),
//     //                 version: "3.34.3".into(),
//     //                 revision: "r0".into(),
//     //                 edition: None,
//     //             },
//     //         },
//     //         ManifestEntry {
//     //             package_name: "alsa-utils-alsactl".into(),
//     //             architecture: "core2_64".into(),
//     //             version: "1.2.1".into(),
//     //             runtime_reverse: RuntimeReverse {
//     //                 name: "alsa-utils".into(),
//     //                 version: "1.2.1".into(),
//     //                 revision: "r0".into(),
//     //                 edition: None,
//     //             },
//     //         },
//     //         ManifestEntry {
//     //             package_name: "alsa-utils-alsamixer".into(),
//     //             architecture: "core2_64".into(),
//     //             version: "1.2.1".into(),
//     //             runtime_reverse: RuntimeReverse {
//     //                 name: "alsa-utils".into(),
//     //                 version: "1.2.1".into(),
//     //                 revision: "r0".into(),
//     //                 edition: None,
//     //             },
//     //         },
//     //         ManifestEntry {
//     //             package_name: "dbus-1".into(),
//     //             architecture: "core2_64".into(),
//     //             version: "1.12.16".into(),
//     //             runtime_reverse: RuntimeReverse {
//     //                 name: "dbus".into(),
//     //                 version: "1.12.16".into(),
//     //                 revision: "r0".into(),
//     //                 edition: None,
//     //             },
//     //         },
//     //     ];

//     //     assert_eq!(yocto.manifest_entries, expected_packages)
//     // }

//     #[test]
//     fn parse_runtime_reverse() {
//         let mut test_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         test_manifest_path
//             .push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/dbus-1");

//         let runtime_reverse = RuntimeReverse::new(&test_manifest_path).unwrap();

//         let expected = RuntimeReverse {
//             name: "dbus".into(),
//             version: "1.12.16".into(),
//             revision: "r0".into(),
//             edition: None,
//         };

//         assert_eq!(runtime_reverse, expected);
//     }
// }
