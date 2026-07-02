use bil_verify::VerificationReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticExplanation {
    pub title: String,
    pub summary: String,
    pub affected_objects: Vec<String>,
    pub remediation_steps: Vec<String>,
    pub markdown: String,
}

pub fn explain(_report: &VerificationReport) -> DiagnosticExplanation {
    DiagnosticExplanation {
        title: "Verification Explanation".to_string(),
        summary: "Summary of findings".to_string(),
        affected_objects: vec![],
        remediation_steps: vec![],
        markdown: "Markdown explanation".to_string(),
    }
}
