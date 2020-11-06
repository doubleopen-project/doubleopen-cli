use crate::spdx::spdx::{Algorithm, Checksum, FileInformation, PackageInformation, SPDX};
use fs::{DirEntry, File};
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
};

/// Create SPDX struct from a Yocto pkgdata folder.
pub fn spdx_from_pkgdata(pkgdata_path: &str, manifest_path: &str, name: &str) -> SPDX {
    // Create SPDX struct.
    let mut spdx = SPDX::new(name);

    // Parse pkgdata folder.
    let files = fs::read_dir(pkgdata_path).unwrap();
    let mut srclists: Vec<DirEntry> = Vec::new();
    let mut pkglists: Vec<DirEntry> = Vec::new();

    for file in files {
        let file = file.unwrap();

        let path = file.path();

        // Skip if directory.
        if path.is_dir() {
            continue;
        }

        match path.extension() {
            Some(extension) => match extension.to_str() {
                Some("srclist") => srclists.push(file),
                _ => continue,
            },
            None => pkglists.push(file),
        }
    }

    let mut package_id = 0;

    let mut packages: Vec<PackageInformation> = pkglists
        .iter()
        .map(|pkglist| {
            let mut package = PackageInformation::new(
                pkglist.path().file_stem().unwrap().to_str().unwrap(),
                &mut package_id,
            );
            package.package_comment = Some(
                fs::read_to_string(pkglist.path())
                    .unwrap()
                    .trim()
                    .to_string(),
            );
            package
        })
        .collect();

    let mut file_id = 0;

    let mut files: Vec<FileInformation> = Vec::new();

    // TEMPORARY SOLUTION
    for package in &mut packages {
        if let Some(srclist) = srclists.iter().find(|srclist| {
            srclist.path().file_stem().unwrap().to_str().unwrap() == package.package_name
        }) {
            let srclist_content = fs::read_to_string(srclist.path()).unwrap();

            let srclist: HashMap<String, Vec<HashMap<String, Option<String>>>> =
                serde_json::from_str(&srclist_content).unwrap();

            for i in srclist {
                for elf_file in i.1 {
                    for source_file in elf_file {
                        // TODO: add to list if another file exists with same name but different hash.
                        if files
                            .iter()
                            .find(|x| x.file_name == source_file.0)
                            .is_none()
                            && !source_file.0.contains("<built-in>")
                        {
                            let mut sf = FileInformation::new(source_file.0.as_str(), &mut file_id);

                            if let Some(value) = source_file.1 {
                                sf.file_checksum
                                    .push(Checksum::new(Algorithm::SHA256, value))
                            }

                            files.push(sf);
                        }
                    }
                }
            }
        }
    }

    spdx.package_information = packages;

    filter_packages_with_manifest(&mut spdx, manifest_path);

    spdx
}

pub fn filter_packages_with_manifest(spdx: &mut SPDX, manifest_path: &str) {
    let original_count = &spdx.package_information.len();
    let manifest_packages = get_list_of_packages_from_manifest(manifest_path);
    spdx.package_information.retain(|e| {
        parse_comment_for_packages(&e.package_comment.clone().unwrap())
            .iter()
            .any(|n| manifest_packages.contains(n))
    });
    let final_count = &spdx.package_information.len();

    println!(
        "Filtered {} packages of original {} based on manifest. Final package count: {}",
        original_count - final_count,
        original_count,
        final_count
    )
}

pub fn parse_comment_for_packages(comment: &str) -> Vec<String> {
    let comment = comment.split_whitespace();

    comment
        .filter_map(|word| {
            if word.to_lowercase() == "PACKAGES:".to_lowercase() {
                None
            } else {
                Some(word.to_string())
            }
        })
        .collect()
}

pub fn get_list_of_packages_from_manifest(manifest_path: &str) -> Vec<String> {
    let file = File::open(manifest_path).expect("No such file");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut packages: Vec<String> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let mut split = line.split_whitespace();
            let name: String = split.next().expect("error").to_string();
            packages.push(name);
        }
    }

    packages
}

#[cfg(test)]
mod tests {
    use super::spdx_from_pkgdata;
    use crate::spdx::spdx::SPDX;
    use std::path::PathBuf;

    fn setup_spdx() -> SPDX {
        let mut test_pkgdata_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_pkgdata_path.push("tests/examples/yocto/pkgdata");

        let mut test_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_manifest_path.push("tests/examples/yocto/manifest.manifest");

        let spdx = spdx_from_pkgdata(
            test_pkgdata_path.to_str().unwrap(),
            test_manifest_path.to_str().unwrap(),
            "test_spdx",
        );

        spdx
    }

    #[test]
    fn correct_amount_of_packages_is_created() {
        let spdx = setup_spdx();

        assert_eq!(spdx.package_information.len(), 2);
    }

    #[test]
    fn spdx_is_created_correctly() {
        let spdx = setup_spdx();

        assert_eq!(
            spdx.document_creation_information.document_name,
            "test_spdx"
        );
    }

    #[test]
    fn subpackages_are_in_comments() {
        let spdx = setup_spdx();

        let mtdev = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "mtdev")
            .unwrap();

        let xset = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "xset")
            .unwrap();

        assert_eq!(
            mtdev.package_comment.as_ref().unwrap(),
            "PACKAGES: mtdev-src mtdev-dbg mtdev-staticdev mtdev-dev mtdev-doc mtdev-locale mtdev"
        );

        assert_eq!(
            xset.package_comment.as_ref().unwrap(),
            "PACKAGES: xset-src xset-dbg xset-staticdev xset-dev xset-doc xset-locale xset"
        );
    }

    #[test]
    fn correct_amount_of_files() {
        let spdx = setup_spdx();

        let xset = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "xset")
            .unwrap();

        assert_eq!(xset.files.len(), 30)
    }

    #[test]
    fn duplicate_files_are_filtered() {
        let spdx = setup_spdx();

        let mtdev = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "mtdev")
            .unwrap();

        assert_eq!(mtdev.files.len(), 40);
    }

    #[test]
    fn parse_comment_for_packages() {
        let comment =
            "PACKAGES: mtdev-src mtdev-dbg mtdev-staticdev mtdev-dev mtdev-doc mtdev-locale mtdev";

        let packages = super::parse_comment_for_packages(comment);

        let expected_packages: Vec<String> = vec![
            "mtdev-src".to_string(),
            "mtdev-dbg".to_string(),
            "mtdev-staticdev".to_string(),
            "mtdev-dev".to_string(),
            "mtdev-doc".to_string(),
            "mtdev-locale".to_string(),
            "mtdev".to_string(),
        ];

        assert_eq!(packages, expected_packages);
    }

    #[test]
    fn parse_manifest_for_packages() {
        let mut test_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_manifest_path.push("tests/examples/yocto/manifest.manifest");

        let packages =
            super::get_list_of_packages_from_manifest(test_manifest_path.to_str().unwrap());

        let expected_packages: Vec<String> = vec!["mtdev".to_string(), "xset".to_string()];

        assert_eq!(packages, expected_packages)
    }

    #[test]
    fn filter_spdx_with_manifest() {
        let mut spdx = setup_spdx();

        let mut test_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_manifest_path.push("tests/examples/yocto/manifest.manifest");

        super::filter_packages_with_manifest(&mut spdx, test_manifest_path.to_str().unwrap());

        assert_eq!(spdx.package_information.len(), 2)
    }
}
