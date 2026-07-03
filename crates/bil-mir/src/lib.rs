use bil_core::{ActorRef, AuthorityRef, BilId, EventId, EvidenceRef, PolicyRef, ProfileId, SystemRef};
use bil_replay::ReplayState;
use serde::{Deserialize, Serialize};

use bil_canonical::{BilCanonical, BilValue, CanonicalError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilMirGraph {
    pub graph_id: BilId,
    pub profile: ProfileId,
    pub events: Vec<BilEvent>,
    pub evidence: Vec<EvidenceRefNode>,
    pub authorities: Vec<AuthorityNode>,
    pub policies: Vec<PolicyNode>,
    pub replay: ReplayState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilEvent {
    pub id: EventId,
    pub kind: BilEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BilEventKind {
    ConsentCaptured,
    BankBranchIntakeStarted,
    DocumentReceived,
    EvidenceExtracted,
    ModelInvoked,
    PolicyEvaluated,
    AuthorityBound,
    HumanReviewed,
    DecisionIssued,
    AdverseActionReasoned,
    AuditBundleExported,
    VendorRouteObserved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityEdge {
    pub actor: ActorRef,
    pub system: Option<SystemRef>,
    pub role: String,
    pub scope: String,
    pub policy_ref: PolicyRef,
    pub valid_from_ms: i64,
    pub valid_until_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRefNode {
    pub id: EvidenceRef,
    pub hash: bil_canonical::Hash256,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityNode {
    pub id: AuthorityRef,
    pub edge: AuthorityEdge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyNode {
    pub id: PolicyRef,
}

impl BilCanonical for BilMirGraph {
    fn to_canonical_value(&self) -> Result<BilValue, CanonicalError> {
        // For now, we just serialize to JSON and then to BilValue
        // In a real implementation, we would map the struct fields directly to BilValue
        let json = serde_json::to_value(self)
            .map_err(|e| CanonicalError::Encoding(e.to_string()))?;
        BilValue::try_from(&json)
    }
}
