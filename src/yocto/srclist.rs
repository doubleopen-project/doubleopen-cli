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
