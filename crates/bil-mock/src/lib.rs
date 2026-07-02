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
    // Placeholder for mock generation logic
    BilMirGraph {
        graph_id: BilId(format!("mock-graph-{}", config.seed)),
        profile: ProfileId("bank_branch".to_string()),
        events: vec![],
        evidence: vec![],
        authorities: vec![],
        policies: vec![],
        replay: ReplayState {
            initial_state_hash: bil_canonical::Hash256([0; 32]),
            transition_hashes: vec![],
            final_state_hash: bil_canonical::Hash256([0; 32]),
        },
    }
}
