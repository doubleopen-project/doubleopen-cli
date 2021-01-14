// SPDX-FileCopyrightText: 2020-2021 HH Partners
//
// SPDX-License-Identifier: MIT

use super::policy_violation::PolicyViolation;

pub struct EvaluationResult {
    pub policy_violations: Vec<PolicyViolation>,
}
