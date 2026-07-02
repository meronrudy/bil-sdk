use bil_core::{BilStatus, ProfileId, ReceiptId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub status: BilStatus,
    pub receipt_id: Option<ReceiptId>,
    pub profile: Option<ProfileId>,
    pub checks: Vec<VerificationCheck>,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCheck {
    pub kind: VerificationCheckKind,
    pub status: BilStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationCheckKind {
    SchemaValid,
    CanonicalEncodingValid,
    CommitmentHashValid,
    SignatureValid,
    SignerKnown,
    EvidenceRefsPresent,
    MerkleRootValid,
    MerkleProofsValid,
    AuthorityRefsPresent,
    AuthorityBindingValid,
    PolicyRefsPresent,
    ReplayDeterministic,
    TimestampValid,
    AssuranceLevelValid,
    ProfileChecksPassed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub priority: FindingPriority,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingPriority {
    P0, // invalid proof / cannot trust artifact
    P1, // incomplete evidence / governance risk
    P2, // warning / non-production / metadata gap
}
