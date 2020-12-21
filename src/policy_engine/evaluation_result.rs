use super::policy_violation::PolicyViolation;

pub struct EvaluationResult {
    pub policy_violations: Vec<PolicyViolation>,
}
