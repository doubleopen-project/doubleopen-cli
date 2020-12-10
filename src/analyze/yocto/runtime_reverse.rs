use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use log::debug;
use walkdir::WalkDir;

use crate::analyze::AnalyzerError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeReverse {
    pub name: String,
    pub version: String,
    pub revision: String,
    pub edition: Option<String>,
}

impl RuntimeReverse {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, AnalyzerError> {
        let file = read_to_string(&path)?;
        let lines = file.lines().collect::<Vec<_>>();
        let package_name =
            lines
                .iter()
                .find(|line| line.starts_with("PN:"))
                .ok_or(AnalyzerError::ParseError(
                    "No PN in runtime reverse.".into(),
                ))?;

        let package_version =
            lines
                .iter()
                .find(|line| line.starts_with("PV:"))
                .ok_or(AnalyzerError::ParseError(
                    "No PV in runtime reverse.".into(),
                ))?;

        let package_revision =
            lines
                .iter()
                .find(|line| line.starts_with("PR:"))
                .ok_or(AnalyzerError::ParseError(
                    "No PR in runtime reverse.".into(),
                ))?;

        let package_edition = lines.iter().find(|line| line.starts_with("PE:"));

        Ok(Self {
            name: package_name[4..].to_string(),
            version: package_version[4..].to_string(),
            revision: package_revision[4..].to_string(),
            edition: match package_edition {
                Some(edition) => Some(edition[4..].to_string()),
                None => None,
            },
        })
    }

    pub fn find_work_folder<P: AsRef<Path>>(
        &self,
        work_directories: &Vec<P>,
    ) -> Result<PathBuf, AnalyzerError> {
        let name_version = format!("{}-{}", &self.name, &self.version);
        let version_revision = match &self.edition {
            Some(edition) => format!("{}_{}-{}", edition, &self.version, &self.revision),
            None => format!("{}-{}", &self.version, &self.revision),
        };

        let archive = work_directories.iter().find(|entry| {
            let path = entry.as_ref();
            path.parent()
                .expect("should always have a parent")
                .ends_with(&self.name)
                && path.ends_with(&version_revision)
        });

        match archive {
            Some(direntry) => Ok(direntry.as_ref().to_path_buf()),
            None => Err(AnalyzerError::ParseError(format!(
                "Can't find source archive for {}",
                &name_version
            ))),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn find_source_archive() {
        let build_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0");

        let work_directories = vec![build_directory.clone()];

        let runtime_reverse = RuntimeReverse {
            name: "alsa-utils".into(),
            version: "1.2.1".into(),
            revision: "r0".into(),
            edition: None,
        };

        let archive = runtime_reverse.find_work_folder(&work_directories).unwrap();
        let expected = &build_directory.join("tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0");

        assert!(archive.ends_with("tmp/work/core2-64-poky-linux/alsa-utils/1.2.1-r0"));
    }

    #[test]
    fn create_runtime_reverse() {
        let mut runtime_reverse_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        runtime_reverse_path.push(
            "tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/adwaita-icon-theme",
        );
        let runtime_reverse = RuntimeReverse::new(&runtime_reverse_path).unwrap();

        assert_eq!(runtime_reverse.name, "adwaita-icon-theme");
        assert_eq!(runtime_reverse.version, "3.34.3");
        assert_eq!(runtime_reverse.revision, "r0");

        let mut runtime_reverse_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        runtime_reverse_path
            .push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/db");
        let runtime_reverse = RuntimeReverse::new(&runtime_reverse_path).unwrap();

        assert_eq!(runtime_reverse.name, "db");
        assert_eq!(runtime_reverse.version, "5.3.28");
        assert_eq!(runtime_reverse.revision, "r0");
    }
}
