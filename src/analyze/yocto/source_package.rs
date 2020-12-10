use std::{convert::TryFrom, path::PathBuf, process::Command};

use crate::{analyze::AnalyzerError, utilities::hash256_for_path};
use log::{debug, error, info};
use tempfile::TempDir;
use walkdir::WalkDir;

use super::manifest_entry::ManifestEntry;

#[derive(Debug)]
pub struct YoctoSourcePackage {
    pub package_name: String,
    pub package_version: String,
    pub source_archive_path: PathBuf,
    pub source_files: Vec<YoctoSourceFile>,
}

impl YoctoSourcePackage {
    pub fn new(recipe: &str) -> Result<Self, AnalyzerError> {
        // Find source directory.
        let mut command = Command::new("devtool");
        command.current_dir("/home/hhpartners/yocto/build");
        let tempdir = TempDir::new()?;
        command
            .arg("extract")
            .arg(&recipe)
            .arg(tempdir.path())
            .output()?;

        // Create a temporary directory and unpack the archive there.
        let source_files: Vec<YoctoSourceFile> = WalkDir::new(&tempdir.path())
            .into_iter()
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
                            let sha256 = hash256_for_path(entry.path());
                            Some(YoctoSourceFile {
                                filename: filename.to_string(),
                                sha256,
                            })
                        } else {
                            None
                        }
                    }
                    Err(_) => {
                        error!("No source for {} at {}", recipe, tempdir.path().display());
                        None
                    }
                }
            })
            .collect();

        info!("Found {} source files for {}.", source_files.len(), recipe,);
        Ok(Self {
            package_name: "testi".into(),
            package_version: "testi".into(),
            source_archive_path: tempdir.into_path(),
            source_files,
        })
    }
}

impl TryFrom<&ManifestEntry> for YoctoSourcePackage {
    type Error = AnalyzerError;

    fn try_from(manifest_entry: &ManifestEntry) -> Result<Self, Self::Error> {
        // Find source directory.
        let mut command = Command::new("devtool");
        command.current_dir("/home/hhpartners/yocto/build");
        let tempdir = TempDir::new()?;
        command
            .arg("extract")
            .arg(&manifest_entry.recipe_name)
            .arg(tempdir.path())
            .output()?;

        // Create a temporary directory and unpack the archive there.
        let source_files: Vec<YoctoSourceFile> = WalkDir::new(&tempdir.path())
            .into_iter()
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
                            let sha256 = hash256_for_path(entry.path());
                            Some(YoctoSourceFile {
                                filename: filename.to_string(),
                                sha256,
                            })
                        } else {
                            None
                        }
                    }
                    Err(_) => {
                        error!("No source for {} at {}", &manifest_entry.recipe_name, tempdir.path().display());
                        None
                    }
                }
            })
            .collect();

        info!("Found {} source files for {}.", source_files.len(), &manifest_entry.recipe_name);
        Ok(Self {
            package_name: manifest_entry.recipe_name.clone(),
            package_version: manifest_entry.recipe_version.clone(),
            source_archive_path: tempdir.into_path(),
            source_files,
        })
    }
}

#[derive(Debug)]
pub struct YoctoSourceFile {
    pub filename: String,
    pub sha256: String,
}
// #[cfg(test)]
// mod test_super {
//     use super::*;

//     #[test]
//     fn archives_are_extracted() {
//         let mut source_archive = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         source_archive.push("tests/examples/yocto/build/downloads/dbus-1.12.16.tar.gz");
//         let package = YoctoSourcePackage::new("dbus".into(), "1.12.16".into(), source_archive)
//             .expect("tar.gz");
//         assert_eq!(package.source_files.len(), 537);

//         let mut source_archive = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         source_archive.push("tests/examples/yocto/build/downloads/alsa-utils-1.2.1.tar.bz2");
//         let package = YoctoSourcePackage::new("alsa-utils".into(), "1.2.1".into(), source_archive)
//             .expect("tar.xz");
//         assert_eq!(package.source_files.len(), 285);

//         let mut source_archive = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         source_archive.push("tests/examples/yocto/build/downloads/bison-3.5.3.tar.xz");
//         let package = YoctoSourcePackage::new("dbus".into(), "1.12.16".into(), source_archive)
//             .expect("tar.xz");
//         assert_eq!(package.source_files.len(), 1109);
//     }
// }
