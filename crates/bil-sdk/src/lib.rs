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
        let receipt = self.receipt()?;
        Bil::verify(&receipt)
    }

    pub fn receipt(&self) -> Result<InkReceipt, BilError> {
        let builder = Bil::mock(SyntheticProfile::BankBranch).with_seed(2026);
        let graph = builder.build()?;
        let artifact = Bil::issue(graph, bil_ink::CapabilityCode("demo.receipt".to_string()))?;
        Ok(artifact.receipt)
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

    pub fn verify(receipt: &InkReceipt) -> Result<VerificationReport, BilError> {
        Ok(bil_verify::VerificationEngine::verify_receipt(receipt))
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

    pub fn issue(graph: BilMirGraph, capability: CapabilityCode) -> Result<IssuedArtifact, BilError> {
        use bil_canonical::BilCanonical;
        use bil_core::ReceiptId;
        use bil_ink::{IssuerRef, SubjectRef};
        use bil_signers::{BilSigner, SoftwareDevSigner};

        // 1. Hash the MIR graph
        let mir_commitment = graph
            .commitment_hash()
            .map_err(|e| BilError::Failed(format!("Failed to hash MIR graph: {}", e)))?;

        // 2. Create a signer
        let signer = SoftwareDevSigner::new("sdk-issuer-001".to_string());

        // 3. Construct the receipt envelope
        let mut receipt = InkReceipt {
            receipt_id: ReceiptId(format!("rcpt-{}", uuid::Uuid::new_v4())),
            capability,
            profile: graph.profile.clone(),
            issuer: IssuerRef("sdk-issuer-001".to_string()),
            subject: SubjectRef("subject-001".to_string()),
            mir_commitment,
            evidence_root: None, // TODO: Merkle tree
            event_refs: graph.events.iter().map(|e| e.id.clone()).collect(),
            authority_refs: graph.authorities.iter().map(|a| a.id.clone()).collect(),
            policy_refs: graph.policies.iter().map(|p| p.id.clone()).collect(),
            canonical_commitment: bil_canonical::Hash256([0; 32]), // Placeholder, will be updated
            signer: signer.signer_id(),
            signature: bil_ink::SignatureBytes(vec![]), // Placeholder
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            assurance_level: signer.assurance_level(),
        };

        // 4. Hash the receipt envelope (excluding signature)
        // For now, we just serialize the receipt to JSON and hash that as a placeholder
        // In a real implementation, we would have a specific canonical representation for the receipt
        let receipt_json = serde_json::to_value(&receipt).unwrap();
        let receipt_bil_value = bil_canonical::BilValue::try_from(&receipt_json).unwrap();
        let canonical_commitment = bil_canonical::encode_canonical(&receipt_bil_value)
            .map(|bytes| bil_canonical::Hash256::sha256(&bytes))
            .map_err(|e| BilError::Failed(format!("Failed to hash receipt: {}", e)))?;
        
        receipt.canonical_commitment = canonical_commitment.clone();

        // 5. Sign the commitment
        let signature = signer
            .sign(&canonical_commitment.0)
            .map_err(|e| BilError::Failed(format!("Failed to sign receipt: {}", e)))?;
        
        receipt.signature = signature;

        Ok(IssuedArtifact { receipt })
    }
}

pub mod prelude {
    pub use super::*;
}
