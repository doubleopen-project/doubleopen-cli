use std::{convert::TryFrom, fs::File, path::PathBuf};

use compress_tools::{uncompress_archive, Ownership};
use nom::{IResult, bytes::complete::is_not, character::complete::char, error::ErrorKind, error::ParseError, sequence::delimited, bytes::complete::tag};
use walkdir::WalkDir;

use crate::{analyze::AnalyzerError, utilities::hash256_for_path};

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
        source_archive_path: PathBuf,
    ) -> Result<Self, AnalyzerError> {
        // Create a temporary directory and unpack the archive there.
        let temp_dir = tempfile::tempdir().unwrap();
        let file = File::open(&source_archive_path).unwrap();
        uncompress_archive(file, temp_dir.path(), Ownership::Ignore)?;
        let source_files: Vec<YoctoSourceFile> = WalkDir::new(&temp_dir)
            .into_iter()
            .filter_map(|f| {
                let entry = f.unwrap();
                if entry.metadata().unwrap().is_file() {
                    let filename = entry.file_name().to_string_lossy();
                    let sha256 = hash256_for_path(entry.path());
                    Some(YoctoSourceFile {
                        filename: filename.to_string(),
                        sha256,
                    })
                } else {
                    None
                }
            })
            .collect();
        Ok(Self {
            package_name,
            package_version,
            source_archive_path,
            source_files,
        })
    }
}

pub struct SrcURI {
    locations: Vec<SourceLocation>,
}

impl SrcURI {
    pub fn parse(input: &str) -> Result<Self, AnalyzerError> {
        fn parse_multiline<'a, E: ParseError<&'a str>>(
            input: &'a str,
        ) -> IResult<&'a str, &'a str, E> {
            delimited(tag(r#"""#), is_not("\""), tag(r#"""#))(input)
        }

        let inner = parse_multiline::<(&str, ErrorKind)>(input)
            .map_err(|err| AnalyzerError::ParseError(err.to_string()))?;

        dbg!(inner);
        todo!()
    }
}

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

impl TryFrom<&str> for SourceLocation {
    type Error = AnalyzerError;

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
    use super::*;

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
"#;

        let expected_locations = vec![
            SourceLocation::HTTPS(
                "dbus.freedesktop.org/releases/dbus/dbus-${PV}.tar.gz".to_string(),
            ),
            SourceLocation::File("tmpdir.patch".into()),
            SourceLocation::File("dbus-1.init".into()),
            SourceLocation::File("clear-guid_from_server-if-send_negotiate_unix_f.patch".into()),
            SourceLocation::File("CVE-2020-12049.patch".into()),
        ];

        let parsed = SrcURI::parse(src_uri).unwrap();
    }

    #[test]
    fn parse_inner() {
        let src_uri = r#"https://dbus.freedesktop.org/releases/dbus/dbus-${PV}.tar.gz \
           file://tmpdir.patch \
           file://dbus-1.init \
           file://clear-guid_from_server-if-send_negotiate_unix_f.patch \
           file://CVE-2020-12049.patch \
"#;
        let parsed = SrcURI::parse(src_uri).unwrap();
    }
}
