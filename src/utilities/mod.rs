// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use sha2::{Digest, Sha256};
use std::{fs::File, io, path::Path};
use walkdir::DirEntry;

pub fn hash256_for_path<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256).unwrap();
    let hash: sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize> =
        sha256.finalize();

    hex::encode_upper(hash)
}

// Helper function to skip hidden files
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;
    use walkdir::WalkDir;

    use super::*;

    #[test]
    fn skip_hidden_files() {
        let temp_dir = tempdir().unwrap();
        std::fs::write(&temp_dir.path().join("not_hidden"), "not hidden contents").unwrap();
        std::fs::write(&temp_dir.path().join(".hidden"), "hidden contents").unwrap();

        let filtered_files = WalkDir::new(temp_dir.path())
            .min_depth(1)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect::<Vec<_>>();

        assert_eq!(filtered_files.len(), 1);
        assert_eq!(filtered_files[0], "not_hidden");
    }
}
