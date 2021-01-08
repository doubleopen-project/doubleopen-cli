use std::path::Path;

use fossology::Fossology;
use serde::{Deserialize, Serialize};
use spdx::SPDX;

#[derive(Debug)]
pub struct Notice {
    licenses: Vec<NoticeLicense>,
}

impl Notice {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NoticeError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
}

impl Default for Notice {
    fn default() -> Self {
        Self {
            licenses: Vec::new(),
        }
    }
}

impl From<&SPDX> for Notice {
    fn from(spdx: &SPDX) -> Self {
        let mut notice_licenses: Vec<NoticeLicense> = Vec::new();

        for file in &spdx.file_information {
            for license in file.concluded_license.licenses() {
                let idx = notice_licenses
                    .iter()
                    .position(|notice_license| notice_license.name == license);

                match idx {
                    Some(idx) => {
                        let copyrights = file.copyright_text.lines();
                        notice_licenses[idx]
                            .copyrights
                            .push(file.copyright_text.clone());
                    }
                    None => notice_licenses.push(NoticeLicense {
                        name: license,
                        copyrights: vec![file.copyright_text.clone()],
                        text: "TEST".into(),
                    }),
                }
            }
        }

        Notice {
            licenses: notice_licenses,
        }
    }
}
#[derive(Debug)]
struct NoticeLicense {
    name: String,
    text: String,
    copyrights: Vec<String>,
}

/// Struct for storing license texts from SPDX license list and Fossology.
#[derive(Debug, Serialize, Deserialize)]
struct License {
    spdx_id: String,
    text: String,
}

impl License {
    /// Create a new license.
    pub fn new_from_name(
        name: &str,
        spdx_license_list_version: &str,
        fossology: &Fossology,
    ) -> Result<Self, NoticeError> {
        let license_from_spdx_list =
            License::get_license_text_from_spdx_list(&name, &spdx_license_list_version);

        match license_from_spdx_list {
            Some(license) => Ok(Self {
                spdx_id: name.to_string(),
                text: license,
            }),
            None => {
                let license_text =
                    License::get_license_text_from_fossology(fossology, &name).expect("Failed");
                Ok(Self {
                    spdx_id: name.to_string(),
                    text: license_text,
                })
            }
        }
    }

    /// Get the license text for an SPDX Identifier from the specified version of
    /// the SPDX license list. Gets the text from the SPDX license list GitHub repo.
    fn get_license_text_from_spdx_list(
        spdx_id: &str,
        spdx_license_list_version: &str,
    ) -> Option<String> {
        let url = format!(
            "https://raw.githubusercontent.com/spdx/license-list-data/v{}/text/{}.txt",
            spdx_license_list_version, spdx_id
        );
        let body = reqwest::blocking::get(&url).unwrap().text().unwrap();
        if body == "404: Not Found" {
            None
        } else {
            Some(body)
        }
    }

    fn get_license_text_from_fossology(fossology: &Fossology, spdx_id: &str) -> Option<String> {
        Some(
            fossology
                .license_by_short_name(&spdx_id)
                .expect("Failed getting the license from Fossology.")
                .text,
        )
    }
}

pub fn license_file_from_spdx<P: AsRef<Path>>(
    spdx: &SPDX,
    fossology: &Fossology,
    output_path: P,
) -> Result<(), NoticeError> {
    let licenses = spdx.get_license_ids();
    let licenses = licenses
        .iter()
        .map(|id| License::new_from_name(&id, "3.11", fossology).expect("Notice error"))
        .collect::<Vec<_>>();

    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&licenses).expect("Error serializing licenses."),
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_license_text_for_spdx_id() {
        let expected_beerware = r#""THE BEER-WARE LICENSE" (Revision 42):  <phk@FreeBSD.ORG> wrote this file.
As long as you retain this notice you  can do whatever you want with this
stuff. If we meet some day, and you think  this stuff is worth it, you can
buy me a beer in return Poul-Henning Kamp
"#;

        let result_beerware = License::get_license_text_from_spdx_list("Beerware", "3.11").unwrap();

        assert_eq!(expected_beerware, result_beerware);
    }
}
