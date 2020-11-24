use super::policy_violation::PolicyViolation;

pub struct EvaluationResult {
  policy_violations: Vec<PolicyViolation>
}