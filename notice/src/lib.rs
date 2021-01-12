//! # Notice
//!
//! A module for generating notice files from SPDX documents.

use std::{fs::write, path::Path};

use handlebars::{Handlebars, RenderError, TemplateFileError};
use serde::Serialize;
use spdx::SPDX;

/// A struct to pass to the template for creating the notice. Based on SPDX,
/// but some additional fields are extracted from SPDX and added to help creating
/// custom notice files.
#[derive(Debug, Serialize)]
pub struct Notice<'a> {
    /// Full SPDX document.
    spdx: &'a SPDX,

    /// List of unique licenses found in the SPDX.
    licenses: Vec<NoticeLicense<'a>>,
}

impl<'a> Notice<'a> {
    /// Render the Notice with a Handlebars template file.
    fn render<P: AsRef<Path>>(&self, template_path: P) -> Result<String, NoticeError> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file("notice_template", template_path)?;
        let output = handlebars.render("notice_template", &self)?;
        Ok(output)
    }

    /// Render the Notice with a Handlebars template file to a file.
    pub fn render_notice_to_file<P: AsRef<Path>, O: AsRef<Path>>(
        &self,
        template_path: P,
        output_path: O,
    ) -> Result<(), NoticeError> {
        let notice = self.render(template_path)?;

        write(output_path, notice)?;

        Ok(())
    }
}

/// Error while creating notice.
#[derive(Debug, thiserror::Error)]
pub enum NoticeError {
    /// Error with file input or output.
    #[error(transparent)]
    FileError(#[from] std::io::Error),

    /// Error with template file.
    #[error(transparent)]
    TemplateFileError(#[from] TemplateFileError),

    /// Error while rendering the notice.
    #[error(transparent)]
    RenderError(#[from] RenderError),

    /// A license for the SPDX ID was not found for notice generation.
    #[error("License for the SPDX ID not found.")]
    LicenseNotFoundError(String),
}

impl<'a> From<&'a SPDX> for Notice<'a> {
    /// Create the notice struct from SPDX.
    fn from(spdx: &'a SPDX) -> Self {
        let mut notice_licenses: Vec<NoticeLicense> = Vec::new();

        // Get licenses and copyrights from all files in the SPDX.
        // TODO: Should probably also get data from packages.
        for file in &spdx.file_information {
            for license in file.concluded_license.licenses() {
                // Do not add NOASSERTION to the notice.
                if license == "NOASSERTION" {
                    continue;
                }

                // Check if the license is already encountered. If it is, add copyrights
                // for that licenses. If the license is new, create it and add copyrights.
                let idx = notice_licenses
                    .iter()
                    .position(|notice_license| notice_license.name == license);

                match idx {
                    Some(idx) => {
                        let mut copyrights: Vec<&str> = file
                            .copyright_text
                            .lines()
                            .filter(|&copyright| copyright != "NOASSERTION")
                            .collect();
                        notice_licenses[idx].copyrights.append(&mut copyrights);
                    }
                    None => {
                        let copyrights: Vec<&str> = file
                            .copyright_text
                            .lines()
                            .filter(|&copyright| copyright != "NOASSERTION")
                            .collect();
                        notice_licenses.push(NoticeLicense {
                            name: license,
                            copyrights,
                            text: "TEST".into(),
                        })
                    }
                }
            }
        }

        // Remove duplicate copyrights from licenses.
        for notice_license in &mut notice_licenses {
            notice_license.copyrights.sort_unstable();
            notice_license.copyrights.reverse();
            notice_license.copyrights.dedup();

            // Populate the notice license with its license text.
            notice_license
                .get_license_text(&spdx)
                .expect("Did not find the license text.");
        }

        Notice {
            spdx,
            licenses: notice_licenses,
        }
    }
}

/// Information aboute licenses encountered in SPDX to be provided for the template.
#[derive(Debug, Serialize)]
struct NoticeLicense<'a> {
    /// SPDX ID of the license.
    name: String,

    /// Full license text.
    text: String,

    /// Copyrights from all files licensed under the license.
    copyrights: Vec<&'a str>,
}

impl<'a> NoticeLicense<'a> {
    /// Populate the struct with the license text. Get the text from the SPDX
    /// license list in GitHub if the license is on the list, and from the SPDX
    /// file if it's not on the list.
    fn get_license_text(&mut self, spdx: &SPDX) -> Result<(), NoticeError> {
        // TODO: Might make more sense to first check the SPDX documents before
        // querying the SPDX list to reduce the number of requests. Probably a
        // minimal performance impact.
        let license_list_version = match &spdx.document_creation_information.license_list_version {
            Some(version) => version,
            None => "3.11",
        };
        let text_from_spdx_list = self.get_license_text_from_spdx_list(&license_list_version);

        // If no license found from list, get it from SPDX. If it doesn't include the
        // license either, produce error.
        match text_from_spdx_list {
            Ok(text) => {
                self.text = text;
            }
            Err(_) => {
                self.text = self.get_license_text_from_spdx_file(&spdx)?;
            }
        }

        Ok(())
    }

    /// Get license text from the provided SPDX file.
    fn get_license_text_from_spdx_file(&self, spdx: &SPDX) -> Result<String, NoticeError> {
        let text = spdx
            .other_licensing_information_detected
            .iter()
            .find(|&lic| lic.license_identifier == self.name)
            .ok_or_else(|| {
                NoticeError::LicenseNotFoundError(format!(
                    "License '{}' not found from SPDX file.",
                    &self.name
                ))
            })?;

        Ok(text.extracted_text.clone())
    }

    /// Get the license text for an SPDX Identifier from the specified version of
    /// the SPDX license list. Gets the text from the SPDX license list GitHub repo.
    fn get_license_text_from_spdx_list(
        &self,
        spdx_license_list_version: &str,
    ) -> Result<String, NoticeError> {
        let url = format!(
            "https://raw.githubusercontent.com/spdx/license-list-data/v{}/text/{}.txt",
            spdx_license_list_version, self.name
        );
        let body = reqwest::blocking::get(&url).unwrap().text().unwrap();

        // Github returns "404: Not Found" if the file is not found.
        if body == "404: Not Found" {
            Err(NoticeError::LicenseNotFoundError(format!(
                "License '{}' not found from SPDX list.",
                &self.name
            )))
        } else {
            Ok(body)
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn get_correct_license_text_from_spdx_list() {
        let expected_beerware = r#""THE BEER-WARE LICENSE" (Revision 42):  <phk@FreeBSD.ORG> wrote this file.
As long as you retain this notice you  can do whatever you want with this
stuff. If we meet some day, and you think  this stuff is worth it, you can
buy me a beer in return Poul-Henning Kamp
"#;

        let mut notice_license = NoticeLicense {
            name: "Beerware".to_string(),
            copyrights: Vec::new(),
            text: "NONE".to_string(),
        };
        notice_license.text = notice_license
            .get_license_text_from_spdx_list("3.11")
            .unwrap();

        assert_eq!(expected_beerware, notice_license.text);
    }

    #[test]
    fn get_correct_license_text_from_spdx_file() {
        let spdx = read_to_string("../tests/examples/spdx/simple.spdx.json").unwrap();
        let spdx: SPDX = serde_json::from_str(&spdx).unwrap();

        let expected_license = r#"CMU License

          Mach Operating System
          Copyright © 1991,1990,1989 Carnegie Mellon University
          All Rights Reserved.
Permission to use, copy, modify and distribute this software and its documentation is hereby granted, provided that both the copyright notice and this permission notice appear in all copies of the software, derivative works or modified versions, and any portions thereof, and that both notices appear in supporting documentation.

carnegie mellon allows free use of this software in its “as is” condition. carnegie mellon disclaims any liability of any kind for any damages whatsoever resulting from the use of this software.

Carnegie Mellon requests users of this software to return to

           Software Distribution Coordinator
           School of Computer Science
           Carnegie Mellon University
           Pittsburgh PA 15213-3890
or Software.Distribution@CS.CMU.EDU any improvements or extensions that they make and grant Carnegie Mellon the rights to redistribute these changes."#;

        let mut notice_license = NoticeLicense {
            name: "CMU".into(),
            text: "NONE".into(),
            copyrights: Vec::new(),
        };

        notice_license.text = notice_license
            .get_license_text_from_spdx_file(&spdx)
            .unwrap();

        assert_eq!(notice_license.text, expected_license);
    }

    #[test]
    fn get_correct_license_text() {
        let spdx = read_to_string("../tests/examples/spdx/simple.spdx.json").unwrap();
        let spdx: SPDX = serde_json::from_str(&spdx).unwrap();

        let mut license_from_spdx_list = NoticeLicense {
            name: "Beerware".into(),
            text: "NONE".into(),
            copyrights: Vec::new(),
        };

        let expected_beerware = r#""THE BEER-WARE LICENSE" (Revision 42):  <phk@FreeBSD.ORG> wrote this file.
As long as you retain this notice you  can do whatever you want with this
stuff. If we meet some day, and you think  this stuff is worth it, you can
buy me a beer in return Poul-Henning Kamp
"#;

        let mut license_from_spdx_file = NoticeLicense {
            name: "CMU".into(),
            text: "NONE".into(),
            copyrights: Vec::new(),
        };

        let expected_cmu = r#"CMU License

          Mach Operating System
          Copyright © 1991,1990,1989 Carnegie Mellon University
          All Rights Reserved.
Permission to use, copy, modify and distribute this software and its documentation is hereby granted, provided that both the copyright notice and this permission notice appear in all copies of the software, derivative works or modified versions, and any portions thereof, and that both notices appear in supporting documentation.

carnegie mellon allows free use of this software in its “as is” condition. carnegie mellon disclaims any liability of any kind for any damages whatsoever resulting from the use of this software.

Carnegie Mellon requests users of this software to return to

           Software Distribution Coordinator
           School of Computer Science
           Carnegie Mellon University
           Pittsburgh PA 15213-3890
or Software.Distribution@CS.CMU.EDU any improvements or extensions that they make and grant Carnegie Mellon the rights to redistribute these changes."#;

        let mut license_not_found = NoticeLicense {
            name: "ERROR".into(),
            text: "NONE".into(),
            copyrights: Vec::new(),
        };

        license_from_spdx_list.get_license_text(&spdx).unwrap();
        license_from_spdx_file.get_license_text(&spdx).unwrap();
        license_not_found.get_license_text(&spdx).unwrap_err();

        assert_eq!(license_from_spdx_list.text, expected_beerware);
        assert_eq!(license_from_spdx_file.text, expected_cmu);
    }
}
