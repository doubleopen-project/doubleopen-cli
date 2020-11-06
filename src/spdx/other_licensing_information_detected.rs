use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OtherLicensingInformationDetected {
    pub license_identifier: String,
    pub extracted_text: String,
    pub license_name: String,
    pub license_cross_reference: Option<Vec<String>>,
    pub license_comment: Option<String>,
}
