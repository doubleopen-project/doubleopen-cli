// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use super::package_list::{ElfFile, PackageList, SourceFile};
use indicatif::ProgressBar;
use std::{collections::HashMap, fs, path::PathBuf};

pub fn process_srclists(srclist_directory: &str) -> Vec<PackageList> {
    println!("Processing srclists...");
    let files = fs::read_dir(srclist_directory).unwrap();

    let mut package_lists: Vec<PackageList> = vec![];

    let mut srclists: Vec<PathBuf> = vec![];

    for file in files {
        let file = file.unwrap();

        let path = file.path();

        let file_name = path.file_name().unwrap();

        if path.is_dir() {
            continue;
        }

        let file_type: FileType = match path.extension() {
            Some(extension) => match extension.to_str() {
                Some("srclist") => FileType::SrcList,
                _ => FileType::PackageList,
            },
            None => FileType::PackageList,
        };

        match file_type {
            FileType::SrcList => srclists.push(path.clone()),
            FileType::PackageList => package_lists.push(PackageList::new(
                file_name.to_str().unwrap().to_owned(),
                path.to_owned(),
            )),
        }
    }

    let package_list_count = package_lists.len() as u64;
    let pb = ProgressBar::new(package_list_count);
    for e in package_lists.iter_mut() {
        pb.inc(1);
        for srclist in &srclists {
            let srclistpath = format!("{}.srclist", e.name);
            if srclist.file_name().unwrap().to_str().unwrap() == srclistpath {
                e.srclist = Some(srclist.clone());

                let contents = fs::read_to_string(&srclist).unwrap();

                process_srclist_content(&contents, e);
            }
        }
        let contents = fs::read_to_string(&e.path).unwrap();
        let mut split = contents.split_whitespace();
        split.next();
        while let Some(item) = split.next() {
            e.packages.push(item.to_owned())
        }
    }
    pb.finish_with_message("done");

    package_lists
}

/// Add information from srclist to PackageList.
fn process_srclist_content(srclist_content: &str, package_list: &mut PackageList) {
    let source_list: HashMap<String, Vec<HashMap<String, Option<String>>>> =
        serde_json::from_str(srclist_content).unwrap();

    for elf_file in source_list.iter() {
        let elf_path = elf_file.0;
        let mut source_files: Vec<SourceFile> = vec![];
        for source_file in elf_file.1 {
            for source_file_2 in source_file {
                let sf: SourceFile = SourceFile {
                    path: source_file_2.0.to_owned(),
                    sha256: source_file_2.1.to_owned(),
                };
                source_files.push(sf);
            }
        }
        let elf: ElfFile = ElfFile {
            path: elf_path.to_owned(),
            source_files,
        };

        package_list.elf_files.push(elf);
    }
}

#[derive(Debug)]
pub enum FileType {
    SrcList,
    PackageList,
}

#[cfg(test)]
mod tests {
    use super::process_srclist_content;
    use crate::yocto::package_list::{ElfFile, PackageList, SourceFile};

    #[test]
    fn srclist_content_is_correctly_parsed() {
        let srclist_content: &str = r#"
        {
            "/path/to/elf/one": [
                {
                    "/path/to/null/source": null
                },
                {
                    "/path/to/found/source": "f045c15d062121e7902877a496feb8b9f3ba1351b62233aaa4dae38d874825ba"
                }
            ],
            "/path/to/elf/two": [
                {
                    "/source/1": "9ace381871f4a991e7da7a590bc0e20ee48d9b4df0469cc898d0158f442b7906"
                },
                {
                    "/source/2": "b7979a70aa1b61a7d0bda93f5cc1c04ce7755da35bd8eef37181f9fe19de4089"
                },
                {
                    "/source/3": "37515f78be386f2edad4746f2947c928f480728c56d9a94e25302ebbdaef3c96"
                }
            ]
        }"#;
        let mut package_list: PackageList = PackageList {
            name: "Test".to_string(),
            path: "/path/to/package-list".into(),
            elf_files: Vec::new(),
            srclist: Some("/path/to/srclist".into()),
            packages: vec!["package1".to_string(), "package2".to_string()],
        };

        let expected_package_list: PackageList = PackageList {
            name: "Test".to_string(),
            path: "/path/to/package-list".into(),
            elf_files: vec![
                ElfFile {
                    path: "/path/to/elf/one".into(),
                    source_files: vec![
                        SourceFile {
                            path: "/path/to/null/source".into(),
                            sha256: None,
                        },
                        SourceFile {
                            path: "/path/to/found/source".into(),
                            sha256: Some(
                                "f045c15d062121e7902877a496feb8b9f3ba1351b62233aaa4dae38d874825ba"
                                    .into(),
                            ),
                        },
                    ],
                },
                ElfFile {
                    path: "/path/to/elf/two".into(),
                    source_files: vec![
                        SourceFile {
                            path: "/source/1".into(),
                            sha256: Some(
                                "9ace381871f4a991e7da7a590bc0e20ee48d9b4df0469cc898d0158f442b7906"
                                    .into(),
                            ),
                        },
                        SourceFile {
                            path: "/source/2".into(),
                            sha256: Some(
                                "b7979a70aa1b61a7d0bda93f5cc1c04ce7755da35bd8eef37181f9fe19de4089"
                                    .into(),
                            ),
                        },
                        SourceFile {
                            path: "/source/3".into(),
                            sha256: Some(
                                "37515f78be386f2edad4746f2947c928f480728c56d9a94e25302ebbdaef3c96"
                                    .into(),
                            ),
                        },
                    ],
                },
            ],
            srclist: Some("/path/to/srclist".into()),
            packages: vec!["package1".to_string(), "package2".to_string()],
        };

        process_srclist_content(srclist_content, &mut package_list);

        assert_eq!(package_list.name, expected_package_list.name);
        assert_eq!(package_list.path, expected_package_list.path);
        assert_eq!(package_list.packages, expected_package_list.packages);
        assert_eq!(package_list.srclist, expected_package_list.srclist);
        assert!(package_list
            .elf_files
            .iter()
            .all(|x| expected_package_list.elf_files.contains(x)));
    }
}
