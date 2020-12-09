use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::analyze::AnalyzerError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeReverse {
    pub package_name: String,
    pub version: String,
    pub package_revision: String,
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

        let package_revision =
            lines
                .find(|line| line.starts_with("PR:"))
                .ok_or(AnalyzerError::ParseError(
                    "No PR in runtime reverse.".into(),
                ))?;
        Ok(Self {
            package_name: package_name[4..].to_string(),
            version: package_version[4..].to_string(),
            package_revision: package_revision[4..].to_string(),
        })
    }

    pub fn find_source_files<P: AsRef<Path>>(
        &self,
        work_directories: &Vec<P>,
    ) -> Result<PathBuf, AnalyzerError> {
        let name_version = format!("{}-{}", &self.package_name, &self.version);
        let version_revision = format!("{}-{}", &self.version, &self.package_revision);

        let archive = work_directories.iter().find(|entry| {
            let path = entry.as_ref();
            path.parent()
                .expect("should always have a parent")
                .ends_with(&self.package_name)
                && path.ends_with(&version_revision)
        });

        match archive {
            Some(direntry) => Ok(direntry.as_ref().to_path_buf().join(&name_version)),
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
            package_name: "alsa-utils".into(),
            version: "1.2.1".into(),
            package_revision: "r0".into(),
        };

        let archive = runtime_reverse
            .find_source_files(&work_directories)
            .unwrap();
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

        assert_eq!(runtime_reverse.package_name, "adwaita-icon-theme");
        assert_eq!(runtime_reverse.version, "3.34.3");
        assert_eq!(runtime_reverse.package_revision, "r0");

        let mut runtime_reverse_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        runtime_reverse_path
            .push("tests/examples/yocto/build/tmp/pkgdata/qemux86-64/runtime-reverse/db");
        let runtime_reverse = RuntimeReverse::new(&runtime_reverse_path).unwrap();

        assert_eq!(runtime_reverse.package_name, "db");
        assert_eq!(runtime_reverse.version, "5.3.28");
        assert_eq!(runtime_reverse.package_revision, "r0");
    }
}
