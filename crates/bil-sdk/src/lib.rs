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

pub struct DoctorReport {
    pub is_healthy: bool,
    pub messages: Vec<String>,
}

pub trait VerificationInput {}

pub struct MockBuilder {
    profile: SyntheticProfile,
    seed: Option<u64>,
}

impl MockBuilder {
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn build(self) -> Result<BilMirGraph, BilError> {
        match self.profile {
            SyntheticProfile::BankBranch => {
                let config = bil_mock::BankBranchSyntheticConfig {
                    seed: self.seed.unwrap_or(0),
                    branch_id: "branch-001".to_string(),
                    include_ai_assist: true,
                    include_human_review: true,
                    include_adverse_action: false,
                    signer_level: bil_core::AssuranceLevel::L0SoftwareDev,
                };
                Ok(bil_mock::generate_bank_branch_mock(&config))
            }
            _ => Err(BilError::Failed("Profile not yet supported".to_string())),
        }
    }
}

pub struct IssuedArtifact {
    pub receipt: InkReceipt,
}

impl Bil {
    pub fn demo(profile: DemoProfile) -> Result<DemoRun, BilError> {
        Ok(DemoRun { profile })
    }

    pub fn doctor() -> Result<DoctorReport, BilError> {
        Ok(DoctorReport {
            is_healthy: true,
            messages: vec!["BIL SDK core is healthy".to_string()],
        })
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

    pub fn mock(profile: SyntheticProfile) -> MockBuilder {
        MockBuilder {
            profile,
            seed: None,
        }
    }

    pub fn issue(_graph: BilMirGraph, _capability: CapabilityCode) -> Result<IssuedArtifact, BilError> {
        Err(BilError::Failed("Not implemented".to_string()))
    }
}

pub mod prelude {
    pub use super::*;
}
