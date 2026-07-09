//! Platform contracts for VLA proposals, safety decisions, and replay flows.

use crate::{EventId, MachineId, MonotonicMicros};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ActionCommand {
    pub linear_velocity: f32,
    pub angular_velocity: f32,
    pub emergency_stop: bool,
}

impl Default for ActionCommand {
    fn default() -> Self {
        Self {
            linear_velocity: 0.0,
            angular_velocity: 0.0,
            emergency_stop: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlannerStatus {
    Ready,
    Fallback,
    Rejected,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionProposal {
    pub proposal_id: EventId,
    pub machine_id: MachineId,
    pub timestamp: MonotonicMicros,
    pub planner: String,
    pub confidence: f32,
    pub command: ActionCommand,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyOutcome {
    Approved,
    Modified,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyDecision {
    pub machine_id: MachineId,
    pub timestamp: MonotonicMicros,
    pub original: ActionProposal,
    pub final_command: ActionCommand,
    pub outcome: SafetyOutcome,
    pub reasons: Vec<String>,
    pub latency_micros: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetySeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyEventType {
    ReplayStep,
    EmergencyStop,
    CollisionAvoidanceOverride,
    GeofenceOverride,
    StabilityReduction,
    LoadLimitStop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyEvent {
    pub event_id: EventId,
    pub machine_id: MachineId,
    pub timestamp: MonotonicMicros,
    pub event_type: SafetyEventType,
    pub severity: SafetySeverity,
    pub message: String,
}
