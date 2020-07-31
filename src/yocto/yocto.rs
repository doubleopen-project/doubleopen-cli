use crate::spdx::spdx::{Algorithm, Checksum, FileInformation, PackageInformation, SPDX};
use fs::DirEntry;
use std::{collections::HashMap, fs};

/// Create SPDX struct from a Yocto pkgdata folder.
pub fn spdx_from_pkgdata(path: &str, name: &str) -> SPDX {
    // Create SPDX struct.
    let mut spdx = SPDX::new(name);

    // Parse pkgdata folder.
    let files = fs::read_dir(path).unwrap();
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
                        if package
                            .file_information
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

                            package.file_information.push(sf);
                        }
                    }
                }
            }
        }
    }

    spdx.package_information = packages;

    spdx
}

#[cfg(test)]
mod tests {
    use super::spdx_from_pkgdata;
    use crate::spdx::spdx::SPDX;
    use std::path::PathBuf;

    fn setup_spdx() -> SPDX {
        let mut test_pkgdata_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_pkgdata_path.push("tests/examples/yocto/pkgdata");

        let spdx = spdx_from_pkgdata(test_pkgdata_path.to_str().unwrap(), "test_spdx");

        spdx
    }

    #[test]
    fn correct_amount_of_packages_is_created() {
        let spdx = setup_spdx();

        assert_eq!(spdx.package_information.len(), 3);
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

        let dbus_wait = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "dbus-wait")
            .unwrap();

        let xset = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "xset")
            .unwrap();

        assert_eq!(dbus_wait.package_comment.as_ref().unwrap(), "PACKAGES: dbus-wait-src dbus-wait-dbg dbus-wait-staticdev dbus-wait-dev dbus-wait-doc dbus-wait-locale dbus-wait");

        assert_eq!(
            xset.package_comment.as_ref().unwrap(),
            "PACKAGES: xset-src xset-dbg xset-staticdev xset-dev xset-doc xset-locale xset"
        );
    }

    #[test]
    fn correct_amount_of_files() {
        let spdx = setup_spdx();

        let dbus_wait = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "dbus-wait")
            .unwrap();

        assert_eq!(dbus_wait.file_information.len(), 23)
    }

    #[test]
    fn duplicate_files_are_filtered() {
        let spdx = setup_spdx();

        let mtdev = spdx
            .package_information
            .iter()
            .find(|pkg| pkg.package_name == "mtdev")
            .unwrap();

        assert_eq!(mtdev.file_information.len(), 40);
    }
}
