use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FileType {
    Source,
    Binary,
    Archive,
    Application,
    Audio,
    Image,
    Text,
    Video,
    Documentation,
    SPDX,
    Other,
}
