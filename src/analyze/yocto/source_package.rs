use std::{fs::File, path::PathBuf};

use compress_tools::{uncompress_archive, Ownership};
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

#[derive(Debug)]
pub struct YoctoSourceFile {
    filename: String,
    sha256: String,
}
