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

pub fn explain(report: &VerificationReport) -> DiagnosticExplanation {
    let mut affected_objects = Vec::new();
    let mut remediation_steps = Vec::new();
    let mut markdown = String::new();

    let title = match report.status {
        bil_core::BilStatus::Pass => "Verification Passed".to_string(),
        bil_core::BilStatus::Warn => "Verification Passed with Warnings".to_string(),
        bil_core::BilStatus::Fail => "Verification Failed".to_string(),
    };

    let summary = format!("Found {} issues during verification.", report.findings.len());

    markdown.push_str(&format!("# {}\n\n{}\n\n", title, summary));

    if !report.findings.is_empty() {
        markdown.push_str("## Findings\n\n");
        for finding in &report.findings {
            markdown.push_str(&format!("* **[{:?}]** {}\n", finding.priority, finding.message));
            
            // Basic heuristic for remediation steps based on message content
            if finding.message.contains("Signature verification failed") {
                remediation_steps.push("Ensure the receipt was signed with the correct private key.".to_string());
                remediation_steps.push("Verify the canonical encoding of the receipt matches the signed bytes.".to_string());
            } else if finding.message.contains("L0SoftwareDev") {
                remediation_steps.push("Use a production-grade signer (e.g., KMS or HSM) for production receipts.".to_string());
            } else if finding.message.contains("no event references") {
                remediation_steps.push("Ensure the MIR graph contains events before issuing a receipt.".to_string());
            } else if finding.message.contains("no authority references") {
                remediation_steps.push("Ensure the MIR graph contains authority bindings before issuing a receipt.".to_string());
            } else if finding.message.contains("no policy references") {
                remediation_steps.push("Ensure the MIR graph contains policy references before issuing a receipt.".to_string());
            }
        }
    }

    if let Some(receipt_id) = &report.receipt_id {
        affected_objects.push(receipt_id.0.clone());
    }

    if !remediation_steps.is_empty() {
        markdown.push_str("\n## Remediation Steps\n\n");
        for (i, step) in remediation_steps.iter().enumerate() {
            markdown.push_str(&format!("{}. {}\n", i + 1, step));
        }
    }

    DiagnosticExplanation {
        title,
        summary,
        affected_objects,
        remediation_steps,
        markdown,
    }
}
