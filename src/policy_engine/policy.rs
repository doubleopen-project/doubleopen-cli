use std::collections::HashMap;

use crate::spdx::{FileInformation, PackageInformation, SPDX};

use super::{evaluation_result::EvaluationResult, license::License, policy_violation::PolicyViolation};
use serde::{Deserialize, Serialize};

/// Struct for a license policy to be used with the Policy Engine.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Policy {
    /// Allowlisted licenses.
    pub licenses_allow: Vec<License>,

    /// Denylisted licenses.
    pub licenses_deny: Vec<License>,
}

