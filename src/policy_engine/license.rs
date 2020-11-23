use serde::{Serialize, Deserialize};

/// Representation of allowlisted or denylisted license in license policy.
#[derive(Serialize, Deserialize)]
pub struct License {
  /// SPDX Expression of the license.
  pub spdx_expression: String,

  /// Message attached to the policy violation for the license.
  pub message: String,
}