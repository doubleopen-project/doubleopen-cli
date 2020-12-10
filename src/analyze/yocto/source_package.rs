use std::{
    convert::TryFrom,
    fs::{read_to_string, File},
    path::{Path, PathBuf},
    process::Command,
};

use compress_tools::{uncompress_archive, Ownership};
use log::{debug, error, info};
use walkdir::WalkDir;
extern crate pest;
use crate::{analyze::AnalyzerError, utilities::hash256_for_path};
use pest::{iterators::Pair, Parser};

#[derive(Debug)]
pub struct YoctoSourcePackage {
    pub package_name: String,
    pub package_version: String,
    pub source_archive_path: PathBuf,
    pub source_files: Vec<YoctoSourceFile>,
}

impl YoctoSourcePackage {
    pub fn new(
        package_name: String,
        package_version: String,
        work_directory: PathBuf,
    ) -> Result<Self, AnalyzerError> {
        // Find source directory.
        let mut command = Command::new("bitbake");
        command.current_dir("/home/hhpartners/yocto/build");
        command.arg("-e").arg(&package_name);
        let output = command.output().unwrap();
        let output = std::str::from_utf8(&output.stdout).unwrap();
        let source = output
            .lines()
            .find(|line| line.starts_with("S="))
            .unwrap()
            .replace("S=", "")
            .replace('"', "");

        debug!(
            "Source for {}-{} is in {}",
            &package_name, &package_version, &source
        );

        // Create a temporary directory and unpack the archive there.
        let source_files: Vec<YoctoSourceFile> = WalkDir::new(&source)
            .into_iter()
            .filter_map(|f| {
                let entry = f;
                match entry {
                    Ok(entry) => {
                        if entry.metadata().unwrap().is_file() {
                            let filename = entry
                                .path()
                                .strip_prefix(&source)
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
                        error!(
                            "No source for {}-{} at {}",
                            package_name,
                            package_version,
                            source
                        );
                        None
                    }
                }
            })
            .collect();

        info!(
            "Found {} source files for {}-{}.",
            source_files.len(),
            package_name,
            package_version
        );
        Ok(Self {
            package_name,
            package_version,
            source_archive_path: source.into(),
            source_files,
        })
    }
}

#[derive(Debug)]
pub struct Recipe {
    pub src_uri: SrcURI,
    pub package_name: String,
    pub package_version: String,
}

impl Recipe {
    pub fn parse(
        input: &str,
        package_name: &str,
        package_version: &str,
    ) -> Result<Recipe, AnalyzerError> {
        let file: Pair<Rule> = RecipeParser::parse(Rule::file, &input)
            .expect("Unsuccesful parse")
            .next()
            .unwrap();

        let mut locations: Vec<SourceLocation> = Vec::new();

        for rule in file.into_inner() {
            match rule.as_rule() {
                Rule::src_uris => {
                    for inner_rule in rule.into_inner() {
                        locations.push(SourceLocation::try_from(
                            inner_rule
                                .as_str()
                                .replace("${PN}", package_name)
                                .replace("${PV}", package_version)
                                .as_str(),
                        )?)
                    }
                }
                _ => {}
            }
        }

        Ok(Recipe {
            package_name: package_name.to_owned(),
            package_version: package_version.to_owned(),
            src_uri: SrcURI { locations },
        })
    }

    pub fn collect_source_files(&self) {
        for location in &self.src_uri.locations {
            dbg!(location);
        }
    }
}

#[derive(Debug)]
pub struct SrcURI {
    pub locations: Vec<SourceLocation>,
}

#[derive(Parser)]
#[grammar = "analyze/yocto/recipe.pest"]
pub struct RecipeParser {}

/// Locations for sources in SRC_URI in a Yocto recipe. Reference https://docs.yoctoproject.org/ref-manual/ref-variables.html#term-SRC_URI.
#[derive(Debug, PartialEq)]
pub enum SourceLocation {
    File(String),
    BZR(String),
    Git(String),
    OSC(String),
    Repo(String),
    CCRC(String),
    HTTP(String),
    HTTPS(String),
    FTP(String),
    CVS(String),
    HG(String),
    P4(String),
    SSH(String),
    SVN(String),
    NPM(String),
}

impl SourceLocation {
    pub fn path_to_source<P: AsRef<Path>>(&self, path_to_recipe: P, package_name: &str) -> PathBuf {
        let path_to_recipe = path_to_recipe.as_ref();
        let path_to_source = match self {
            SourceLocation::File(path) => path_to_recipe
                .parent()
                .expect("Should always have a parent.")
                .join(package_name)
                .join(path),
            _ => unreachable!(),
        };
        path_to_source
    }
}

impl TryFrom<&str> for SourceLocation {
    type Error = AnalyzerError;

    /// Try to parse SourceLocation from a URI in a recipe file.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split("://");
        let protocol = split.next().ok_or(AnalyzerError::ParseError(format!(
            "No protocol found in {}",
            &value
        )))?;
        let location = split.next().ok_or(AnalyzerError::ParseError(format!(
            "No location found in {}",
            &value
        )))?;

        match protocol {
            "file" => Ok(SourceLocation::File(location.to_string())),
            "bzr" => Ok(SourceLocation::BZR(location.to_string())),
            "git" => Ok(SourceLocation::Git(location.to_string())),
            "osc" => Ok(SourceLocation::OSC(location.to_string())),
            "repo" => Ok(SourceLocation::Repo(location.to_string())),
            "ccrc" => Ok(SourceLocation::CCRC(location.to_string())),
            "http" => Ok(SourceLocation::HTTP(location.to_string())),
            "https" => Ok(SourceLocation::HTTPS(location.to_string())),
            "ftp" => Ok(SourceLocation::FTP(location.to_string())),
            "cvs" => Ok(SourceLocation::CVS(location.to_string())),
            "hg" => Ok(SourceLocation::HG(location.to_string())),
            "p4" => Ok(SourceLocation::P4(location.to_string())),
            "ssh" => Ok(SourceLocation::SSH(location.to_string())),
            "svn" => Ok(SourceLocation::SVN(location.to_string())),
            "npm" => Ok(SourceLocation::NPM(location.to_string())),
            _ => Err(AnalyzerError::ParseError(format!(
                "Unknown protocol in {}",
                &value
            ))),
        }
    }
}

#[derive(Debug)]
pub struct YoctoSourceFile {
    pub filename: String,
    pub sha256: String,
}
#[cfg(test)]
mod test_super {
    use std::fs::read_to_string;

    use super::*;
    use crate::analyze::yocto::source_package::pest::Parser;

    #[test]
    fn archives_are_extracted() {
        let mut source_archive = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        source_archive.push("tests/examples/yocto/build/downloads/dbus-1.12.16.tar.gz");
        let package = YoctoSourcePackage::new("dbus".into(), "1.12.16".into(), source_archive)
            .expect("tar.gz");
        assert_eq!(package.source_files.len(), 537);

        let mut source_archive = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        source_archive.push("tests/examples/yocto/build/downloads/alsa-utils-1.2.1.tar.bz2");
        let package = YoctoSourcePackage::new("alsa-utils".into(), "1.2.1".into(), source_archive)
            .expect("tar.xz");
        assert_eq!(package.source_files.len(), 285);

        let mut source_archive = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        source_archive.push("tests/examples/yocto/build/downloads/bison-3.5.3.tar.xz");
        let package = YoctoSourcePackage::new("dbus".into(), "1.12.16".into(), source_archive)
            .expect("tar.xz");
        assert_eq!(package.source_files.len(), 1109);
    }

    #[test]
    fn source_location_is_parsed() {
        assert_eq!(
            SourceLocation::try_from(
                "https://dbus.freedesktop.org/releases/dbus/dbus-${PV}.tar.gz"
            )
            .unwrap(),
            SourceLocation::HTTPS(
                "dbus.freedesktop.org/releases/dbus/dbus-${PV}.tar.gz".to_string()
            )
        );
        assert_eq!(
            SourceLocation::try_from(
                "file://clear-guid_from_server-if-send_negotiate_unix_f.patch"
            )
            .unwrap(),
            SourceLocation::File(
                "clear-guid_from_server-if-send_negotiate_unix_f.patch".to_string()
            )
        );
    }

    #[test]
    fn parse_src_uri() {
        let src_uri = r#"SRC_URI = "https://dbus.freedesktop.org/releases/dbus/dbus-${PV}.tar.gz \
           file://tmpdir.patch \
           file://dbus-1.init \
           file://clear-guid_from_server-if-send_negotiate_unix_f.patch \
           file://CVE-2020-12049.patch \
""#;

        let expected_locations = vec![
            SourceLocation::HTTPS(
                "dbus.freedesktop.org/releases/dbus/dbus-${PV}.tar.gz".to_string(),
            ),
            SourceLocation::File("tmpdir.patch".into()),
            SourceLocation::File("dbus-1.init".into()),
            SourceLocation::File("clear-guid_from_server-if-send_negotiate_unix_f.patch".into()),
            SourceLocation::File("CVE-2020-12049.patch".into()),
        ];

        let parsed = RecipeParser::parse(Rule::src_uris, &src_uri).unwrap();
        let mut locations: Vec<SourceLocation> = vec![];
        for pair in parsed {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::uri => {
                        locations.push(SourceLocation::try_from(inner_pair.as_str()).unwrap())
                    }
                    _ => error!("Should not happen."),
                }
            }
        }

        assert_eq!(locations, expected_locations);
    }

    #[test]
    fn parse_recipe() {
        let mut recipe_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        recipe_path.push("tests/examples/yocto/meta/recipes-core/dbus/dbus_1.12.16.bb");

        let input = read_to_string(&recipe_path).unwrap();

        let recipe = Recipe::parse(&input, "dbus", "1-12-16").unwrap();
        dbg!(recipe);
    }

    #[test]
    fn find_source_location() {
        let mut recipe_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        recipe_path.push("tests/examples/yocto/meta/recipes-core/dbus/dbus_1.12.16.bb");

        let source_location = SourceLocation::File("tmpdir.patch".into());
        assert!(source_location
            .path_to_source(recipe_path, "dbus")
            .ends_with("meta/recipes-core/dbus/dbus/tmpdir.patch"));
    }

    #[test]
    fn collect_source_files_of_recipe() {
        let mut recipe_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        recipe_path.push("tests/examples/yocto/meta/recipes-core/dbus/dbus_1.12.16.bb");
        let input = read_to_string(&recipe_path).unwrap();
        let recipe = Recipe::parse(&input, "dbus", "1.12.16").unwrap();

        recipe.collect_source_files();
    }
}
