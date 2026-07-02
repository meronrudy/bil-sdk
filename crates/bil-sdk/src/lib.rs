use bil_core::BilStatus;
use bil_explain::DiagnosticExplanation;
use bil_ink::{CapabilityCode, InkReceipt};
use bil_mir::BilMirGraph;
use bil_mock::SyntheticProfile;
use bil_verify::VerificationReport;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BilError {
    #[error("Operation failed: {0}")]
    Failed(String),
}

pub struct Bil;

pub enum DemoProfile {
    BankBranch,
    LoanDecision,
    AiAssurance,
}

pub struct DemoRun {
    pub profile: DemoProfile,
}

impl DemoRun {
    pub fn verify(&self) -> Result<VerificationReport, BilError> {
        Ok(VerificationReport {
            status: BilStatus::Pass,
            receipt_id: None,
            profile: None,
            checks: vec![],
            findings: vec![],
        })
    }

    pub fn receipt(&self) -> Result<InkReceipt, BilError> {
        // Placeholder for generating a receipt from a demo run
        Err(BilError::Failed("Not implemented".to_string()))
    }

    pub fn memo(&self) -> Result<AssuranceMemo, BilError> {
        // Placeholder for generating a memo from a demo run
        Ok(AssuranceMemo {
            markdown: "# Assurance Memo\n\nThis is a placeholder memo.".to_string(),
        })
    }
}

pub struct AssuranceMemo {
    pub markdown: String,
}

impl AssuranceMemo {
    pub fn write_markdown(&self, path: &str) -> Result<(), BilError> {
        std::fs::write(path, &self.markdown)
            .map_err(|e| BilError::Failed(format!("Failed to write memo: {}", e)))
    }
}

pub struct DoctorReport;

pub trait VerificationInput {}

pub struct MockBuilder;

pub struct IssuedArtifact;

impl Bil {
    pub fn demo(profile: DemoProfile) -> Result<DemoRun, BilError> {
        Ok(DemoRun { profile })
    }

    pub fn doctor() -> Result<DoctorReport, BilError> {
        Ok(DoctorReport)
    }

    pub fn verify(_input: impl VerificationInput) -> Result<VerificationReport, BilError> {
        Ok(VerificationReport {
            status: BilStatus::Pass,
            receipt_id: None,
            profile: None,
            checks: vec![],
            findings: vec![],
        })
    }

    pub fn explain(report: &VerificationReport) -> DiagnosticExplanation {
        bil_explain::explain(report)
    }

    pub fn mock(_profile: SyntheticProfile) -> MockBuilder {
        MockBuilder
    }

    pub fn issue(_graph: BilMirGraph, _capability: CapabilityCode) -> Result<IssuedArtifact, BilError> {
        Ok(IssuedArtifact)
    }
}

pub mod prelude {
    pub use super::*;
}
