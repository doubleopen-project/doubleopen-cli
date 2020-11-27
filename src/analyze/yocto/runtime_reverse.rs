use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::analyze::AnalyzerError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeReverse {
    pub package_name: String,
    pub version: String,
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

#[cfg(test)]
mod test_super {
    use super::*;

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
}
