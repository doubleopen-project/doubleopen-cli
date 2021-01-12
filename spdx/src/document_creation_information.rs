use super::ExternalDocumentReference;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/// ## Document Creation Information
///
/// SPDX's [Document Creation Information](https://spdx.github.io/spdx-spec/2-document-creation-information/)
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentCreationInformation {
    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#21-spdx-version
    pub spdx_version: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#22-data-license
    pub data_license: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#23-spdx-identifier
    #[serde(rename = "SPDXID")]
    pub spdx_identifier: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#24-document-name
    pub document_name: String,

    ///https://spdx.github.io/spdx-spec/2-document-creation-information/#25-spdx-document-namespace
    pub spdx_document_namespace: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#26-external-document-references
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub external_document_references: Option<Vec<ExternalDocumentReference>>,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#27-license-list-version
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub license_list_version: Option<String>,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#28-creator
    pub creator: String,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#29-created
    pub created: DateTime<Utc>,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#210-creator-comment
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub creator_comment: Option<String>,

    /// https://spdx.github.io/spdx-spec/2-document-creation-information/#211-document-comment
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub document_comment: Option<String>,
}

impl Default for DocumentCreationInformation {
    fn default() -> Self {
        Self {
            // Current version is 2.2. Might need to support more verisons
            // in the future.
            spdx_version: "SPDX-2.2".to_string(),
            data_license: "CC0-1.0".to_string(),
            spdx_identifier: "SPDXRef-DOCUMENT".to_string(),
            document_name: "NOASSERTION".to_string(),
            spdx_document_namespace: "NOASSERTION".to_string(),
            external_document_references: None,
            license_list_version: None,
            // TODO: Get tool name and version automatically.
            creator:
                "Person: Jane Doe () Organization: ExampleCodeInspect () Tool: LicenseFind-1.0"
                    .to_string(),
            created: chrono::offset::Utc::now(),
            creator_comment: None,
            document_comment: None,
        }
    }
}
