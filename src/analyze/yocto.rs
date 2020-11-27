use crate::spdx::spdx::{Algorithm, FileInformation, PackageInformation, SPDX};
use flate2::read::GzDecoder;
use fs::File;
use tar::Archive;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    path::Path,
    path::PathBuf,
};

use super::AnalyzerError;

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

pub fn get_list_of_packages_from_manifest<P: AsRef<Path>, R: AsRef<Path>>(
    manifest_path: &P,
    runtime_reverse_dir_path: &R,
) -> Result<Vec<ManifestEntry>, AnalyzerError> {
    let file = File::open(manifest_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut packages: Vec<ManifestEntry> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let entry = ManifestEntry::new(&line, &runtime_reverse_dir_path)?;
            packages.push(entry);
        }
    }

    Ok(packages)
}

#[derive(Debug)]
pub struct YoctoBuild<'yocto> {
    pub image_name: String,
    pub architecture: String,
    pub build_directory: PathBuf,
    pub manifest_entries: Vec<ManifestEntry>,
    pub source_packages: Vec<YoctoSourcePackage<'yocto>>,
}

impl<'yocto> YoctoBuild<'yocto> {
    pub fn new<P: AsRef<Path>>(
        build_directory: P,
        manifest_file: P,
    ) -> Result<Self, AnalyzerError> {
        let image_name = manifest_file
            .as_ref()
            .file_stem()
            .ok_or_else(|| AnalyzerError::ParseError("No manifest file name.".into()))?
            .to_owned()
            .into_string()
            .map_err(|_| AnalyzerError::ParseError("test".into()))?;

        let architecture = manifest_file
            .as_ref()
            .components()
            .rev()
            .nth(1)
            .ok_or_else(|| AnalyzerError::ParseError("No architecture in path.".into()))?
            .as_os_str()
            .to_str()
            .ok_or_else(|| AnalyzerError::ParseError("No architecture in path.".into()))?
            .to_string();

        let runtime_reverse_dir_path = build_directory
            .as_ref()
            .join("tmp/pkgdata/")
            .join(&architecture)
            .join("runtime-reverse");

        let manifest_entries =
            get_list_of_packages_from_manifest(&manifest_file, &runtime_reverse_dir_path)?;

        Ok(Self {
            image_name,
            architecture,
            build_directory: build_directory.as_ref().into(),
            manifest_entries,
            ..Default::default()
        })
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
}

impl<'yocto> Default for YoctoBuild<'yocto> {
    fn default() -> Self {
        Self {
            architecture: "DEFAULT".into(),
            build_directory: "DEFAULT".into(),
            image_name: "DEFAULT".into(),
            manifest_entries: Vec::new(),
            source_packages: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ManifestEntry {
    pub package_name: String,
    pub architecture: String,
    pub version: String,
    pub runtime_reverse: RuntimeReverse,
}

impl ManifestEntry {
    fn new<P: AsRef<Path>>(line: &str, build_path: P) -> Result<Self, AnalyzerError> {
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

        let runtime_reverse_path = build_path.as_ref().join(&package_name);

        let runtime_reverse = RuntimeReverse::new(runtime_reverse_path)?;

        Ok(Self {
            package_name,
            architecture,
            version,
            runtime_reverse,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeReverse {
    package_name: String,
    version: String,
}

impl RuntimeReverse {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, AnalyzerError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut package_name = lines.next().unwrap()?;
        if package_name.starts_with("PN:") {
            package_name.drain(..4);
        } else {
            return Err(AnalyzerError::ParseError("Error".into()));
        }

        let mut package_version = lines.next().unwrap()?;
        if package_version.starts_with("PV:") {
            package_version.drain(..4);
        } else {
            return Err(AnalyzerError::ParseError("Error".into()));
        }

        Ok(Self {
            package_name,
            version: package_version,
        })
    }

    pub fn find_source_archive<P: AsRef<Path>>(
        &self,
        build_directory: P,
    ) -> Result<PathBuf, AnalyzerError> {
        let downloads_directory = build_directory.as_ref().join("downloads/");

        let name_version = format!("{}-{}", &self.package_name, &self.version);

        let mut downloads = downloads_directory.read_dir()?.map(|x| x.unwrap());

        let archive = downloads.find(|download| {
            download
                .file_name()
                .to_str()
                .expect("Archive not found.")
                .starts_with(&self.package_name)
                && download
                    .path()
                    .extension()
                    .expect("Archive not found.")
                    .to_str()
                    .expect("Archive not found.")
                    != "done"
        });

        match archive {
            Some(direntry) => Ok(direntry.path()),
            None => Err(AnalyzerError::ParseError(
                "Can't find source archive.".into(),
            )),
        }
    }
}

#[derive(Debug)]
pub struct YoctoSourcePackage<'yocto> {
    package_name: &'yocto str,
    package_version: &'yocto str,
    source_archive_path: &'yocto Path,
    source_files: Vec<YoctoSourceFile<'yocto>>,
}

impl<'yocto> YoctoSourcePackage<'yocto> {
    pub fn new(
        package_name: &'yocto str,
        package_version: &'yocto str,
        source_archive_path: &'yocto Path,
    ) -> Self {
        // Create a temporary directory and unpack the archive there.
        let temp_dir = tempfile::tempdir().unwrap();
        let file = File::open(&source_archive_path).unwrap();
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack(&temp_dir.path()).unwrap();
        Self {
            package_name,
            package_version,
            source_archive_path,
            source_files,
        }
    }
}

#[derive(Debug)]
pub struct YoctoSourceFile<'yocto> {
    filename: &'yocto str,
    sha256: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spdx::spdx::SPDX;
    use std::path::PathBuf;

    #[test]
    fn create_yocto() {
        let mut manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_path.push("tests/examples/yocto/build/tmp/deploy/images/qemux86-64/core-image-sato-qemux86-64.manifest");

        let mut build_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        build_path.push("tests/examples/yocto/build");

        let yocto = YoctoBuild::new(&build_path, &manifest_path).unwrap();

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
    }

    #[test]
    fn find_source_archive() {
        let mut build_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        build_directory.push("tests/examples/yocto/build/");

        let runtime_reverse = RuntimeReverse {
            package_name: "alsa-utils".into(),
            version: "1.2.1".into(),
        };

        let archive = runtime_reverse
            .find_source_archive(&build_directory)
            .unwrap();
        let expected = build_directory.join("downloads/alsa-utils-1.2.1.tar.bz2");

        assert_eq!(archive, expected);
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
        let mut test_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_manifest_path.push("tests/examples/yocto/build/tmp/deploy/images/qemux86-64/core-image-sato-qemux86-64.manifest");

        let mut runtime_reverse_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        runtime_reverse_dir_path
            .push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/");

        let packages = super::get_list_of_packages_from_manifest(
            &test_manifest_path,
            &runtime_reverse_dir_path,
        )
        .unwrap();

        let expected_packages = vec![
            ManifestEntry {
                package_name: "adwaita-icon-theme".into(),
                architecture: "noarch".into(),
                version: "3.34.3".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "adwaita-icon-theme".into(),
                    version: "3.34.3".into(),
                },
            },
            ManifestEntry {
                package_name: "adwaita-icon-theme-symbolic".into(),
                architecture: "noarch".into(),
                version: "3.34.3".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "adwaita-icon-theme".into(),
                    version: "3.34.3".into(),
                },
            },
            ManifestEntry {
                package_name: "alsa-utils-alsactl".into(),
                architecture: "core2_64".into(),
                version: "1.2.1".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "alsa-utils".into(),
                    version: "1.2.1".into(),
                },
            },
            ManifestEntry {
                package_name: "alsa-utils-alsamixer".into(),
                architecture: "core2_64".into(),
                version: "1.2.1".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "alsa-utils".into(),
                    version: "1.2.1".into(),
                },
            },
            ManifestEntry {
                package_name: "dbus-1".into(),
                architecture: "core2_64".into(),
                version: "1.12.16".into(),
                runtime_reverse: RuntimeReverse {
                    package_name: "dbus".into(),
                    version: "1.12.16".into(),
                },
            },
        ];

        assert_eq!(packages, expected_packages)
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
        };

        assert_eq!(runtime_reverse, expected);
    }
}
