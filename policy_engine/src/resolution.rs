use serde::{Deserialize, Serialize};

/// Resolutions can be used to resolve rule violations.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Resolution {
    /// Package that the violation is resolved for.
    pub package: String,
    /// License that is violated.
    pub license: String,
    /// Why the violation is resolved.
    pub reason: String,
}
