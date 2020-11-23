use super::license::License;
use serde::{Deserialize, Serialize};

/// Struct for a license policy to be used with the Policy Engine.
#[derive(Serialize, Deserialize)]
pub struct Policy {
    /// Allowlisted licenses.
    pub licenses_allow: Vec<License>,

    /// Denylisted licenses.
    pub licenses_deny: Vec<License>,
}
