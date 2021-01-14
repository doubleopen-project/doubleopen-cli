// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Representation of allowlisted or denylisted license in license policy.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct License {
    /// SPDX Expression of the license.
    pub spdx_expression: String,

    /// Message attached to the policy violation for the license.
    pub message: Option<String>,
}
