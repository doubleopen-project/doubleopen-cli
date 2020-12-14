use super::AnalyzerError;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source_files: Vec<SourceFile>,
}

#[derive(Debug)]
pub struct SourceFile {
    pub name: String,
    pub sha256: String,
    pub used_in_build: bool,
}

impl Package {
    pub fn exlude_unused_source_files(&mut self, used_source_hashes: Vec<String>) -> Result<(), AnalyzerError> {
        for source_file in &mut self.source_files {
            if used_source_hashes.iter().any(|hash| hash.to_ascii_lowercase() == source_file.sha256.to_ascii_lowercase()) {
                source_file.used_in_build = true
            } else {
                source_file.used_in_build = false
            }
        };

        Ok(())
    }
}