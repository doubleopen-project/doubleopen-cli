use std::{fs::write, path::Path};

use handlebars::{Handlebars, RenderError, TemplateFileError};
use serde::Serialize;
use spdx::SPDX;

#[derive(Debug, Serialize)]
pub struct Notice<'a> {
    spdx: &'a SPDX,
    licenses: Vec<NoticeLicense>,
}

impl<'a> Notice<'a> {
    /// Render the Notice with a Handlebars template file.
    fn render<P: AsRef<Path>>(&self, template_path: P) -> Result<String, NoticeError> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file("notice_template", template_path)?;
        let output = handlebars.render("notice_template", &self)?;
        Ok(output)
    }

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

#[derive(Debug, thiserror::Error)]
pub enum NoticeError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),

    #[error(transparent)]
    TemplateFileError(#[from] TemplateFileError),

    #[error(transparent)]
    RenderError(#[from] RenderError),
}

impl<'a> From<&'a SPDX> for Notice<'a> {
    fn from(spdx: &'a SPDX) -> Self {
        let mut notice_licenses: Vec<NoticeLicense> = Vec::new();

        for file in &spdx.file_information {
            for license in file.concluded_license.licenses() {
                if license == "NOASSERTION" {
                    continue;
                }
                let idx = notice_licenses
                    .iter()
                    .position(|notice_license| notice_license.name == license);

                match idx {
                    Some(idx) => {
                        let mut copyrights: Vec<String> = file
                            .copyright_text
                            .lines()
                            .map(|line| line.to_string())
                            .filter(|copyright| copyright != "NOASSERTION")
                            .collect();
                        notice_licenses[idx].copyrights.append(&mut copyrights);
                    }
                    None => {
                        let copyrights: Vec<String> = file
                            .copyright_text
                            .lines()
                            .map(|line| line.to_string())
                            .filter(|copyright| copyright != "NOASSERTION")
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

        for notice_license in &mut notice_licenses {
            notice_license.copyrights.sort();
            notice_license.copyrights.reverse();
            notice_license.copyrights.dedup();

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
#[derive(Debug, Serialize)]
struct NoticeLicense {
    name: String,
    text: String,
    copyrights: Vec<String>,
}

impl NoticeLicense {
    /// Populate the struct with the license text. Get the text from the SPDX
    /// license list in GitHub if the license is on the list, and from the SPDX
    /// file if it's not on the list.
    fn get_license_text(&mut self, spdx: &SPDX) -> Result<(), NoticeError> {
        let license_list_version = match &spdx.document_creation_information.license_list_version {
            Some(version) => version,
            None => "3.11",
        };
        let text_from_spdx_list = self.get_license_text_from_spdx_list(&license_list_version);

        match text_from_spdx_list {
            Some(text) => {
                self.text = text;
            }
            None => {
                self.text = self
                    .get_license_text_from_spdx_file(&spdx)
                    .expect("Should be found")
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
            .expect("Should be found.");

        Ok(text.extracted_text.clone())
    }

    /// Get the license text for an SPDX Identifier from the specified version of
    /// the SPDX license list. Gets the text from the SPDX license list GitHub repo.
    fn get_license_text_from_spdx_list(&self, spdx_license_list_version: &str) -> Option<String> {
        let url = format!(
            "https://raw.githubusercontent.com/spdx/license-list-data/v{}/text/{}.txt",
            spdx_license_list_version, self.name
        );
        let body = reqwest::blocking::get(&url).unwrap().text().unwrap();
        if body == "404: Not Found" {
            None
        } else {
            Some(body)
        }
    }
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

        let notice_license = NoticeLicense {
            name: "Beerware".to_string(),
            copyrights: Vec::new(),
            text: "NONE".to_string(),
        };
        notice_license
            .get_license_text_from_spdx_list("3.11")
            .unwrap();

        assert_eq!(expected_beerware, notice_license.text);
    }
}
