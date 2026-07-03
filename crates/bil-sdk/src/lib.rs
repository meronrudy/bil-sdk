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

#[derive(Debug)]
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
        let receipt = self.receipt()?;
        let report = Bil::verify(&receipt)?;
        let explanation = Bil::explain(&report);

        let mut markdown = String::new();
        markdown.push_str("# Assurance Memo\n\n");
        markdown.push_str(&format!("**Profile:** {:?}\n", self.profile));
        markdown.push_str(&format!(
            "**Receipt ID:** {}\n\n",
            receipt.preimage.receipt_id.0
        ));

        markdown.push_str("## Verification Summary\n\n");
        markdown.push_str(&explanation.markdown);

        Ok(AssuranceMemo { markdown })
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

    pub fn include_ai_assist(self, _include: bool) -> Self {
        // TODO: store in builder state
        self
    }

    pub fn include_human_review(self, _include: bool) -> Self {
        // TODO: store in builder state
        self
    }

    pub fn build(self) -> Result<BilMirGraph, BilError> {
        match self.profile {
            SyntheticProfile::BankBranch => {
                let config = bil_mock::BankBranchSyntheticConfig {
                    seed: self.seed.unwrap_or(0),
                    branch_id: "branch-001".to_string(),
                    include_ai_assist: true, // Hardcoded for now until builder state is updated
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

    pub fn issue(
        graph: BilMirGraph,
        capability: CapabilityCode,
    ) -> Result<IssuedArtifact, BilError> {
        use bil_canonical::BilCanonical;
        use bil_core::ReceiptId;
        use bil_ink::{InkReceiptPreimage, IssuerRef, SubjectRef};
        use bil_signers::{BilSigner, SoftwareDevSigner};

        // 1. Hash the MIR graph
        let mir_commitment = graph
            .commitment_hash()
            .map_err(|e| BilError::Failed(format!("Failed to hash MIR graph: {}", e)))?;

        // 2. Create a signer
        // For the mock, we use the public key as the signer ID so the verifier can check it
        let temp_signer = SoftwareDevSigner::new("temp".to_string());
        let pk_hex = temp_signer.public_key_ref().0;
        // We need to reuse the same keypair, so we can't just create a new one with the hex string.
        // Let's update SoftwareDevSigner to allow setting the ID, or just use the temp_signer
        // and we'll update the receipt's signer field directly.
        let signer = temp_signer;

        // 3. Construct the receipt preimage
        let evidence_root = bil_ink::merkle::MerkleTree::build(&graph.evidence)
            .map_err(|e| BilError::Failed(format!("Failed to build Merkle tree: {}", e)))?
            .map(|tree| tree.root);

        let preimage = InkReceiptPreimage {
            receipt_id: ReceiptId(format!("rcpt-{}", uuid::Uuid::new_v4())),
            capability,
            profile: graph.profile.clone(),
            issuer: IssuerRef("sdk-issuer-001".to_string()),
            subject: SubjectRef("subject-001".to_string()),
            mir_commitment,
            evidence_root,
            event_refs: graph.events.iter().map(|e| e.id.clone()).collect(),
            authority_refs: graph.authorities.iter().map(|a| a.id.clone()).collect(),
            policy_refs: graph.policies.iter().map(|p| p.id.clone()).collect(),
            signer: bil_ink::SignerRef(pk_hex),
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            assurance_level: signer.assurance_level(),
        };

        // 4. Hash the explicit receipt preimage. The commitment and signature
        // are intentionally outside this payload.
        let canonical_bytes = preimage
            .to_canonical_bytes()
            .map_err(|e| BilError::Failed(format!("Failed to encode preimage: {}", e)))?;
        let canonical_commitment = bil_canonical::Hash256::sha256(&canonical_bytes);

        // 5. Sign the preimage bytes, not just the commitment hash
        let signature = signer
            .sign(&canonical_bytes)
            .map_err(|e| BilError::Failed(format!("Failed to sign receipt: {}", e)))?;

        // 6. Construct the final receipt
        let receipt = InkReceipt {
            preimage,
            canonical_commitment,
            signature,
        };

        Ok(IssuedArtifact { receipt })
    }
}

pub mod prelude {
    pub use super::*;
}
