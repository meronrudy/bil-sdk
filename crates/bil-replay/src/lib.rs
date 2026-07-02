use bil_canonical::Hash256;
use bil_core::EventId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayState {
    pub initial_state_hash: Hash256,
    pub transition_hashes: Vec<Hash256>,
    pub final_state_hash: Hash256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResult {
    pub status: ReplayStatus,
    pub expected_final_state: Hash256,
    pub actual_final_state: Hash256,
    pub divergent_event: Option<EventId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplayStatus {
    Deterministic,
    Divergent,
}
