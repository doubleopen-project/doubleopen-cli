use crate::spdx::{
    spdx::{Algorithm, FileInformation, PackageInformation, SPDX},
    Checksum, Relationship, RelationshipType,
};
use fs::File;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    path::Path,
    path::PathBuf,
};
use walkdir::WalkDir;

use self::{
    manifest_entry::ManifestEntry, runtime_reverse::RuntimeReverse,
    source_package::YoctoSourcePackage,
};

use super::AnalyzerError;

pub mod manifest_entry;
pub mod runtime_reverse;
pub mod source_package;

/// Remove files from packages in SPDX file if the files weren't used to build
/// the packages based on dwarfsrcfiles.
pub fn exclude_files_with_srclist<P: AsRef<Path>>(
    package: &mut PackageInformation,
    srclist_path: P,
    file_informations: &Vec<FileInformation>,
) {
    let hashes_in_srclist = hashes_from_srclist(srclist_path);

    package.files.retain(|file| {
        let file_information = SPDX::find_file_by_spdx_id(&file_informations, file).unwrap();

        hashes_in_srclist.iter().any(|hash| {
            hash.to_lowercase()
                == file_information
                    .file_checksum
                    .iter()
                    .find(|checksum| checksum.algorithm == Algorithm::SHA256)
                    .unwrap()
                    .value
                    .to_lowercase()
        })
    });
}

/// Get all unique hashes from a srclist file produced by Yocto.
pub fn hashes_from_srclist<P: AsRef<Path>>(path: P) -> Vec<String> {
    let srclist_content = fs::read_to_string(path).unwrap();
    let srclist: HashMap<String, Vec<HashMap<String, Option<String>>>> =
        serde_json::from_str(&srclist_content).unwrap();

    let mut hashes: Vec<String> = Vec::new();

    for i in srclist {
        for elf_file in i.1 {
            for source_file in elf_file {
                if let Some(value) = source_file.1 {
                    hashes.push(value);
                }
            }
        }
    }
    hashes.sort();
    hashes.dedup();
    hashes
}

#[derive(Debug)]
pub struct Yocto {
    pub image_name: String,
    pub architecture: String,
    pub build_directory: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest_entries: Vec<ManifestEntry>,
    pub source_packages: Vec<YoctoSourcePackage>,
}

impl Yocto {
    pub fn new<P: AsRef<Path>>(
        build_directory: P,
        manifest_path: P,
    ) -> Result<Self, AnalyzerError> {
        let image_name = manifest_path
            .as_ref()
            .file_stem()
            .ok_or_else(|| AnalyzerError::ParseError("No manifest file name.".into()))?
            .to_owned()
            .into_string()
            .map_err(|_| AnalyzerError::ParseError("test".into()))?;

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

        let mut yocto = Self {
            architecture,
            image_name,
            build_directory: build_directory.as_ref().to_path_buf(),
            manifest_path: manifest_path.as_ref().to_path_buf(),
            ..Default::default()
        };

        yocto.get_list_of_packages_from_manifest()?;
        yocto.create_source_packages()?;

        Ok(yocto)
    }

    pub fn get_list_of_packages_from_manifest(&mut self) -> Result<(), AnalyzerError> {
        let runtime_reverse_dir_path = self
            .build_directory
            .join("tmp/pkgdata/")
            .join(&self.architecture)
            .join("runtime-reverse");
        let file = File::open(&self.manifest_path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        for line in lines {
            if let Ok(line) = line {
                let entry = ManifestEntry::new(&line, &runtime_reverse_dir_path)?;
                self.manifest_entries.push(entry);
            }
        }

        Ok(())
    }

    pub fn get_unique_reversed_packages(&self) -> Vec<&RuntimeReverse> {
        let mut runtime_reverses: Vec<&RuntimeReverse> = self
            .manifest_entries
            .iter()
            .map(|manifest_entry| &manifest_entry.runtime_reverse)
            .collect();

        runtime_reverses.sort_by_key(|&rr| &rr.package_name);
        runtime_reverses.dedup();
        runtime_reverses
    }

    fn create_source_packages(&mut self) -> Result<(), AnalyzerError> {
        let unique_reversed_packages = self.get_unique_reversed_packages();
        let work_directories: Vec<PathBuf> = WalkDir::new(&self.build_directory.join("tmp/work/"))
            .min_depth(3)
            .max_depth(3)
            .into_iter()
            .map(|entry| {
                entry
                    .as_ref()
                    .expect("should always be correct")
                    .clone()
                    .into_path()
            })
            .collect();

        let source_packages = unique_reversed_packages
            .iter()
            .filter_map(|reversed_package| {
                let source_archive_path =
                    &reversed_package.find_source_files(&work_directories).ok();
                match source_archive_path {
                    Some(path) => YoctoSourcePackage::new(
                        reversed_package.package_name.clone(),
                        reversed_package.version.clone(),
                        path.clone(),
                    )
                    .ok(),
                    None => {
                        println!(
                            "No source archive for package {}-{}",
                            &reversed_package.package_name, &reversed_package.version
                        );
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        for source_package in source_packages {
            self.source_packages.push(source_package);
        }

        Ok(())
    }

    pub fn get_work_directories(&self) -> Vec<PathBuf> {
        let work_directories: Vec<PathBuf> = WalkDir::new(&self.build_directory.join("tmp/work/"))
            .min_depth(3)
            .max_depth(3)
            .into_iter()
            .map(|entry| {
                entry
                    .as_ref()
                    .expect("should always be correct")
                    .clone()
                    .into_path()
            })
            .collect();
        work_directories
    }
}

impl Default for Yocto {
    fn default() -> Self {
        Self {
            architecture: "DEFAULT".into(),
            build_directory: "DEFAULT".into(),
            image_name: "DEFAULT".into(),
            manifest_entries: Vec::new(),
            source_packages: Vec::new(),
            manifest_path: "DEFAULT".into(),
        }
    }
}

impl From<Yocto> for SPDX {
    fn from(yocto: Yocto) -> SPDX {
        let mut spdx = SPDX::new(&yocto.image_name);

        for package in yocto.source_packages {
            let mut spdx_package =
                PackageInformation::new(&package.package_name, &mut spdx.spdx_ref_counter);
            spdx_package.package_version = Some(package.package_version);
            for file in package.source_files {
                let mut spdx_file =
                    FileInformation::new(&file.filename, &mut spdx.spdx_ref_counter);
                spdx_file
                    .file_checksum
                    .push(Checksum::new(Algorithm::SHA256, &file.sha256));

                let relationship = Relationship::new(
                    &spdx_package.package_spdx_identifier,
                    &spdx_file.file_spdx_identifier,
                    RelationshipType::Contains,
                    None,
                );
                spdx.file_information.push(spdx_file);
                spdx.relationships.push(relationship);
            }

            spdx.package_information.push(spdx_package)
        }

        spdx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn create_yocto() {
        let mut manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_path.push("tests/examples/yocto/build/tmp/deploy/images/qemux86-64/core-image-sato-qemux86-64.manifest");

        let mut build_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        build_path.push("tests/examples/yocto/build");

        let yocto = Yocto::new(&build_path, &manifest_path).unwrap();

        assert_eq!(yocto.image_name, "core-image-sato-qemux86-64");
        assert_eq!(yocto.architecture, "qemux86-64");
        assert!(yocto
            .build_directory
            .ends_with("tests/examples/yocto/build"));
        assert_eq!(yocto.manifest_entries.len(), 5);
        assert_eq!(
            yocto.manifest_entries[0].runtime_reverse.package_name,
            "adwaita-icon-theme".to_string()
        );
        assert_eq!(
            yocto.manifest_entries[0].runtime_reverse.version,
            "3.34.3".to_string()
        );
        assert_eq!(
            yocto.manifest_entries[1].runtime_reverse.package_name,
            "adwaita-icon-theme".to_string()
        );
        assert_eq!(
            yocto.manifest_entries[1].runtime_reverse.version,
            "3.34.3".to_string()
        );
        assert_eq!(
            yocto.manifest_entries[2].runtime_reverse.package_name,
            "alsa-utils".to_string()
        );
        assert_eq!(
            yocto.manifest_entries[2].runtime_reverse.version,
            "1.2.1".to_string()
        );

        assert_eq!(yocto.source_packages.len(), 3);
        assert_eq!(yocto.source_packages[0].package_name, "adwaita-icon-theme");
        assert_eq!(yocto.source_packages[0].source_files.len(), 3070);
    }

    #[test]
    fn get_all_unique_hashes_from_srclist() {
        let mut srclist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        srclist_path.push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/dbus.srclist");
        let hashes = hashes_from_srclist(srclist_path);

        // TODO: The correct amount has not been manually checked, may want to create
        // a proper test file.
        assert_eq!(hashes.len(), 240)
    }

    #[test]
    fn parse_manifest_for_packages() {
        let mut manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_path.push("tests/examples/yocto/build/tmp/deploy/images/qemux86-64/core-image-sato-qemux86-64.manifest");

        let mut build_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        build_directory.push("tests/examples/yocto/build/");

        let yocto = Yocto::new(build_directory, manifest_path).unwrap();

        let expected_packages = vec![
            ManifestEntry {
                package_name: "adwaita-icon-theme".into(),
                architecture: "noarch".into(),
                version: "3.34.3".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "adwaita-icon-theme".into(),
                    version: "3.34.3".into(),
                    package_revision: "r0".into(),
                },
            },
            ManifestEntry {
                package_name: "adwaita-icon-theme-symbolic".into(),
                architecture: "noarch".into(),
                version: "3.34.3".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "adwaita-icon-theme".into(),
                    version: "3.34.3".into(),
                    package_revision: "r0".into(),
                },
            },
            ManifestEntry {
                package_name: "alsa-utils-alsactl".into(),
                architecture: "core2_64".into(),
                version: "1.2.1".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "alsa-utils".into(),
                    version: "1.2.1".into(),
                    package_revision: "r0".into(),
                },
            },
            ManifestEntry {
                package_name: "alsa-utils-alsamixer".into(),
                architecture: "core2_64".into(),
                version: "1.2.1".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "alsa-utils".into(),
                    version: "1.2.1".into(),
                    package_revision: "r0".into(),
                },
            },
            ManifestEntry {
                package_name: "dbus-1".into(),
                architecture: "core2_64".into(),
                version: "1.12.16".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "dbus".into(),
                    version: "1.12.16".into(),
                    package_revision: "r0".into(),
                },
            },
        ];

        assert_eq!(yocto.manifest_entries, expected_packages)
    }

    #[test]
    fn parse_runtime_reverse() {
        let mut test_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_manifest_path
            .push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/dbus-1");

        let runtime_reverse = RuntimeReverse::new(&test_manifest_path).unwrap();

        let expected = RuntimeReverse {
            package_name: "dbus".into(),
            version: "1.12.16".into(),
            package_revision: "r0".into(),
        };

        assert_eq!(runtime_reverse, expected);
    }


}
