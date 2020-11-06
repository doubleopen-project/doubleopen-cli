use super::Checksum;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ExternalDocumentReference {
    pub id_string: String,
    pub spdx_document_uri: String,
    pub checksum: Checksum,
}
