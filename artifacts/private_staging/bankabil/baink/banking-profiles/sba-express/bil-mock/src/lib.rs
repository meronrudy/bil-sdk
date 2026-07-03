use bil_core::{AssuranceLevel, BilId, ProfileId};
use bil_mir::BilMirGraph;
use bil_replay::ReplayState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyntheticProfile {
    BankBranch,
    LoanDecision,
    AdverseAction,
    AiAssurance,
    ThirdPartyVendor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankBranchSyntheticConfig {
    pub seed: u64,
    pub branch_id: String,
    pub include_ai_assist: bool,
    pub include_human_review: bool,
    pub include_adverse_action: bool,
    pub signer_level: AssuranceLevel,
}

pub fn generate_bank_branch_mock(config: &BankBranchSyntheticConfig) -> BilMirGraph {
    use bil_core::{ActorRef, AuthorityRef, EventId, EvidenceRef, PolicyRef, SystemRef};
    use bil_mir::{AuthorityEdge, AuthorityNode, BilEvent, BilEventKind, EvidenceRefNode, PolicyNode};

    let mut events = vec![
        BilEvent {
            id: EventId(format!("evt-{}-1", config.seed)),
            kind: BilEventKind::BankBranchIntakeStarted,
        },
        BilEvent {
            id: EventId(format!("evt-{}-2", config.seed)),
            kind: BilEventKind::ConsentCaptured,
        },
        BilEvent {
            id: EventId(format!("evt-{}-3", config.seed)),
            kind: BilEventKind::DocumentReceived,
        },
    ];

    let mut evidence = vec![
        EvidenceRefNode {
            id: EvidenceRef(format!("evd-{}-doc1", config.seed)),
            hash: bil_canonical::Hash256::sha256(b"mock-doc-content"),
            kind: Some("application/pdf".to_string()),
        },
    ];

    let mut authorities = vec![
        AuthorityNode {
            id: AuthorityRef(format!("auth-{}-teller", config.seed)),
            edge: AuthorityEdge {
                actor: ActorRef("teller-001".to_string()),
                system: Some(SystemRef(config.branch_id.clone())),
                role: "BranchTeller".to_string(),
                scope: "intake".to_string(),
                policy_ref: PolicyRef("pol-intake-v1".to_string()),
                valid_from_ms: 1672531200000,
                valid_until_ms: None,
            },
        },
    ];

    let mut policies = vec![
        PolicyNode {
            id: PolicyRef("pol-intake-v1".to_string()),
        },
    ];

    if config.include_ai_assist {
        events.push(BilEvent {
            id: EventId(format!("evt-{}-ai", config.seed)),
            kind: BilEventKind::ModelInvoked,
        });
        evidence.push(EvidenceRefNode {
            id: EvidenceRef(format!("evd-{}-ai-summary", config.seed)),
            hash: bil_canonical::Hash256::sha256(b"mock-ai-summary-content"),
            kind: Some("text/markdown".to_string()),
        });
        authorities.push(AuthorityNode {
            id: AuthorityRef(format!("auth-{}-ai", config.seed)),
            edge: AuthorityEdge {
                actor: ActorRef("ai-assistant-v2".to_string()),
                system: Some(SystemRef("cloud-llm".to_string())),
                role: "Assistant".to_string(),
                scope: "summarization".to_string(),
                policy_ref: PolicyRef("pol-ai-v2".to_string()),
                valid_from_ms: 1672531200000,
                valid_until_ms: None,
            },
        });
        policies.push(PolicyNode {
            id: PolicyRef("pol-ai-v2".to_string()),
        });
    }

    if config.include_human_review {
        events.push(BilEvent {
            id: EventId(format!("evt-{}-review", config.seed)),
            kind: BilEventKind::HumanReviewed,
        });
        authorities.push(AuthorityNode {
            id: AuthorityRef(format!("auth-{}-manager", config.seed)),
            edge: AuthorityEdge {
                actor: ActorRef("manager-001".to_string()),
                system: Some(SystemRef(config.branch_id.clone())),
                role: "BranchManager".to_string(),
                scope: "review".to_string(),
                policy_ref: PolicyRef("pol-review-v1".to_string()),
                valid_from_ms: 1672531200000,
                valid_until_ms: None,
            },
        });
        policies.push(PolicyNode {
            id: PolicyRef("pol-review-v1".to_string()),
        });
    }

    events.push(BilEvent {
        id: EventId(format!("evt-{}-decision", config.seed)),
        kind: BilEventKind::DecisionIssued,
    });

    if config.include_adverse_action {
        events.push(BilEvent {
            id: EventId(format!("evt-{}-adverse", config.seed)),
            kind: BilEventKind::AdverseActionReasoned,
        });
    }

    BilMirGraph {
        graph_id: BilId(format!("mock-graph-{}", config.seed)),
        profile: ProfileId("bank_branch".to_string()),
        events,
        evidence,
        authorities,
        policies,
        replay: ReplayState {
            initial_state_hash: bil_canonical::Hash256([0; 32]),
            transition_hashes: vec![],
            final_state_hash: bil_canonical::Hash256([0; 32]),
        },
    }
}
