use super::Checksum;
use serde::{Deserialize, Serialize};

/// https://spdx.github.io/spdx-spec/2-document-creation-information/#26-external-document-references
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalDocumentReference {
    /// Unique ID string of the reference.
    pub id_string: String,

    /// Unique ID for the external document.
    pub spdx_document_uri: String,

    /// Checksum of the external document following the checksum format defined
    /// in https://spdx.github.io/spdx-spec/4-file-information/#44-file-checksum.
    pub checksum: Checksum,
}
