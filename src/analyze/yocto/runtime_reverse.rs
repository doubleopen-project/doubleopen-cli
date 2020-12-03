use std::{
    fs::read_to_string,
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
        let file = read_to_string(&path)?;
        let mut lines = file.lines();
        let package_name =
            lines
                .find(|line| line.starts_with("PN:"))
                .ok_or(AnalyzerError::ParseError(
                    "No PN in runtime reverse.".into(),
                ))?;

        let package_version =
            lines
                .find(|line| line.starts_with("PV:"))
                .ok_or(AnalyzerError::ParseError(
                    "No PV in runtime reverse.".into(),
                ))?;

        Ok(Self {
            package_name: package_name[4..].to_string(),
            version: package_version[4..].to_string(),
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
                .starts_with(&name_version)
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
                format!("Can't find source archive for {}", &name_version)
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

    #[test]
    fn create_runtime_reverse() {
        let mut runtime_reverse_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        runtime_reverse_path.push(
            "tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/adwaita-icon-theme",
        );
        let runtime_reverse = RuntimeReverse::new(&runtime_reverse_path).unwrap();

        assert_eq!(runtime_reverse.package_name, "adwaita-icon-theme");
        assert_eq!(runtime_reverse.version, "3.34.3");

        let mut runtime_reverse_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        runtime_reverse_path
            .push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/db");
        let runtime_reverse = RuntimeReverse::new(&runtime_reverse_path).unwrap();

        assert_eq!(runtime_reverse.package_name, "db");
        assert_eq!(runtime_reverse.version, "5.3.28");
    }
}
