use std::{collections::HashMap, path::Path};

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

impl Policy {
    pub fn from_files<P: AsRef<Path>>(files: Vec<P>, context: &str) -> Self {
       todo!() 
    }
}

