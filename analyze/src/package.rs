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
