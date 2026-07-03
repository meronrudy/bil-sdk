use bil_canonical::{BilCanonical, Hash256};
use bil_core::{
    ActorRef, AuthorityRef, BilId, BilStatus, EventId, PolicyRef, ProfileId, ReceiptId, SystemRef,
};
use bil_ink::{CapabilityCode, InkReceipt};
use bil_mir::{
    AuthorityEdge, AuthorityNode, BilEvent, BilEventKind, BilMirGraph, EvidenceRefNode, PolicyNode,
    ReplayState,
};
use bil_verify::VerificationReport;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BilError {
    #[error("Operation failed: {0}")]
    Failed(String),
}

pub struct Bil;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoProfile {
    HumanOverride,
}

impl DemoProfile {
    pub fn from_name(name: &str) -> Option<Self> {
        match normalize_profile_name(name).as_str() {
            "human_override" | "generic" => Some(Self::HumanOverride),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntheticProfile {
    HumanOverride,
}

impl SyntheticProfile {
    pub fn from_name(name: &str) -> Option<Self> {
        match normalize_profile_name(name).as_str() {
            "human_override" | "generic" => Some(Self::HumanOverride),
            _ => None,
        }
    }
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
        let graph = Bil::mock(SyntheticProfile::HumanOverride)
            .with_seed(2026)
            .build()?;
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
    include_ai_assist: bool,
    include_human_review: bool,
}

impl MockBuilder {
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn include_ai_assist(mut self, include: bool) -> Self {
        self.include_ai_assist = include;
        self
    }

    pub fn include_human_review(mut self, include: bool) -> Self {
        self.include_human_review = include;
        self
    }

    pub fn build(self) -> Result<BilMirGraph, BilError> {
        match self.profile {
            SyntheticProfile::HumanOverride => Ok(generic_human_override_graph_with_options(
                self.seed.unwrap_or(0),
                self.include_ai_assist,
                self.include_human_review,
            )),
        }
    }
}

pub struct IssuedArtifact {
    pub receipt: InkReceipt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticExplanation {
    pub title: String,
    pub summary: String,
    pub affected_receipt_id: Option<ReceiptId>,
    pub remediation_steps: Vec<String>,
    pub markdown: String,
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
        explain_report(report)
    }

    pub fn mock(profile: SyntheticProfile) -> MockBuilder {
        MockBuilder {
            profile,
            seed: None,
            include_ai_assist: true,
            include_human_review: true,
        }
    }

    pub fn issue(
        graph: BilMirGraph,
        capability: CapabilityCode,
    ) -> Result<IssuedArtifact, BilError> {
        use bil_core::ReceiptId;
        use bil_ink::{InkReceiptPreimage, IssuerRef, SignerRef, SubjectRef};
        use bil_signers::{BilSigner, SoftwareDevSigner};

        let mir_commitment = graph
            .commitment_hash()
            .map_err(|e| BilError::Failed(format!("Failed to hash MIR graph: {}", e)))?;

        let signer = SoftwareDevSigner::new("software-dev".to_string());
        let signer_ref = SignerRef(signer.public_key_ref().0);

        let evidence_root = bil_ink::merkle::MerkleTree::build(&graph.evidence)
            .map_err(|e| BilError::Failed(format!("Failed to build Merkle tree: {}", e)))?
            .map(|tree| tree.root);

        let preimage = InkReceiptPreimage {
            receipt_id: ReceiptId(format!("rcpt-{}", uuid::Uuid::new_v4())),
            capability,
            profile: graph.profile.clone(),
            issuer: IssuerRef("bil-sdk".to_string()),
            subject: SubjectRef(graph.graph_id.0.clone()),
            mir_commitment,
            evidence_root,
            event_refs: graph.events.iter().map(|e| e.id.clone()).collect(),
            authority_refs: graph.authorities.iter().map(|a| a.id.clone()).collect(),
            policy_refs: graph.policies.iter().map(|p| p.id.clone()).collect(),
            signer: signer_ref,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| BilError::Failed(format!("System clock error: {}", e)))?
                .as_millis() as i64,
            assurance_level: signer.assurance_level(),
        };

        // The commitment and signature are intentionally outside the committed
        // payload. Only the explicit receipt preimage is encoded and signed.
        let canonical_bytes = preimage
            .to_canonical_bytes()
            .map_err(|e| BilError::Failed(format!("Failed to encode preimage: {}", e)))?;
        let canonical_commitment = bil_canonical::Hash256::sha256(&canonical_bytes);

        let signature = signer
            .sign(&canonical_bytes)
            .map_err(|e| BilError::Failed(format!("Failed to sign receipt: {}", e)))?;

        let receipt = InkReceipt {
            preimage,
            canonical_commitment,
            signature,
        };

        Ok(IssuedArtifact { receipt })
    }
}

pub fn generic_human_override_graph(seed: u64) -> BilMirGraph {
    generic_human_override_graph_with_options(seed, true, true)
}

fn generic_human_override_graph_with_options(
    seed: u64,
    include_ai_assist: bool,
    include_human_review: bool,
) -> BilMirGraph {
    let profile = ProfileId("human_override".to_string());
    let graph_id = BilId(format!("human-override-{}", seed));

    let mut events = vec![
        BilEvent {
            id: EventId(format!("evt_{}_workflow_started", seed)),
            kind: BilEventKind::WorkflowStarted,
        },
        BilEvent {
            id: EventId(format!("evt_{}_document_received", seed)),
            kind: BilEventKind::DocumentReceived,
        },
    ];

    if include_ai_assist {
        events.push(BilEvent {
            id: EventId(format!("evt_{}_evidence_extracted", seed)),
            kind: BilEventKind::EvidenceExtracted,
        });
    }

    events.push(BilEvent {
        id: EventId(format!("evt_{}_policy_evaluated", seed)),
        kind: BilEventKind::PolicyEvaluated,
    });

    if include_human_review {
        events.push(BilEvent {
            id: EventId(format!("evt_{}_human_reviewed", seed)),
            kind: BilEventKind::HumanReviewed,
        });
    }

    events.push(BilEvent {
        id: EventId(format!("evt_{}_decision_issued", seed)),
        kind: BilEventKind::DecisionIssued,
    });

    let evidence = events
        .iter()
        .map(|event| EvidenceRefNode {
            id: bil_core::EvidenceRef(format!("evd_{}", event.id.0)),
            hash: Hash256::sha256(format!("{}:{:?}:{}", graph_id.0, event.kind, seed).as_bytes()),
            kind: Some(format!("{:?}", event.kind)),
        })
        .collect();

    let policies = vec![
        PolicyNode {
            id: PolicyRef("POLICY_WORKFLOW_EVIDENCE_REQUIRED_V1".to_string()),
        },
        PolicyNode {
            id: PolicyRef("POLICY_HUMAN_OVERRIDE_REVIEW_V1".to_string()),
        },
    ];

    let authorities = vec![AuthorityNode {
        id: AuthorityRef("auth_generic_reviewer".to_string()),
        edge: AuthorityEdge {
            actor: ActorRef("actor_generic_reviewer".to_string()),
            system: Some(SystemRef("system_generic_workflow".to_string())),
            role: "reviewer".to_string(),
            scope: "human_override".to_string(),
            policy_ref: policies[1].id.clone(),
            valid_from_ms: 1783070400000,
            valid_until_ms: None,
        },
    }];

    let transition_hashes: Vec<Hash256> = events
        .iter()
        .map(|event| Hash256::sha256(format!("{}:{:?}", event.id.0, event.kind).as_bytes()))
        .collect();
    let mut final_state_preimage = Vec::new();
    for hash in &transition_hashes {
        final_state_preimage.extend_from_slice(&hash.0);
    }

    BilMirGraph {
        graph_id,
        profile,
        events,
        evidence,
        authorities,
        policies,
        replay: ReplayState {
            initial_state_hash: Hash256::zero(),
            transition_hashes,
            final_state_hash: Hash256::sha256(&final_state_preimage),
        },
    }
}

fn explain_report(report: &VerificationReport) -> DiagnosticExplanation {
    let title = format!("Verification status: {:?}", report.status);
    let affected_receipt_id = report.receipt_id.clone();
    let summary = if report.findings.is_empty() {
        "No verification findings were reported.".to_string()
    } else {
        format!(
            "{} verification finding(s) reported.",
            report.findings.len()
        )
    };

    let remediation_steps = match report.status {
        BilStatus::Pass => vec!["No remediation required.".to_string()],
        BilStatus::Warn => vec![
            "Review warning findings before relying on the receipt.".to_string(),
            "Upgrade non-production assurance metadata where required.".to_string(),
        ],
        BilStatus::Fail => vec![
            "Do not rely on this receipt as valid evidence.".to_string(),
            "Reissue the receipt from the canonical source graph.".to_string(),
            "Verify the signer reference and canonical commitment before retrying.".to_string(),
        ],
    };

    let mut markdown = String::new();
    markdown.push_str("# Verification Explanation\n\n");
    markdown.push_str(&format!("**Status:** {:?}\n", report.status));
    if let Some(receipt_id) = &report.receipt_id {
        markdown.push_str(&format!("**Receipt ID:** {}\n", receipt_id.0));
    }
    if let Some(profile) = &report.profile {
        markdown.push_str(&format!("**Profile:** {}\n", profile.0));
    }
    markdown.push('\n');

    markdown.push_str("## Summary\n\n");
    markdown.push_str(&summary);
    markdown.push_str("\n\n");

    markdown.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        markdown.push_str("- No findings.\n");
    } else {
        for finding in &report.findings {
            markdown.push_str(&format!("- [{:?}] {}\n", finding.priority, finding.message));
        }
    }
    markdown.push('\n');

    markdown.push_str("## Checks\n\n");
    for check in &report.checks {
        markdown.push_str(&format!("- {:?}: {:?}\n", check.kind, check.status));
    }
    markdown.push('\n');

    markdown.push_str("## Remediation\n\n");
    for step in &remediation_steps {
        markdown.push_str(&format!("- {}\n", step));
    }

    DiagnosticExplanation {
        title,
        summary,
        affected_receipt_id,
        remediation_steps,
        markdown,
    }
}

fn normalize_profile_name(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace('-', "_")
}

pub mod prelude {
    pub use super::*;
}
