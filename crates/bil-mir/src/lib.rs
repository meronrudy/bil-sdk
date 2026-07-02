use bil_core::{ActorRef, AuthorityRef, BilId, EventId, EvidenceRef, PolicyRef, ProfileId, SystemRef};
use bil_replay::ReplayState;
use serde::{Deserialize, Serialize};

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
