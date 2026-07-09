//! Deterministic VLA boundary for MVP platform integration.

use contracts::{
    ActionCommand, ActionProposal, EventId, MachineId, MonotonicMicros, PlannerStatus,
};
use serde::{Deserialize, Serialize};
use shared::MachineState;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannerConfig {
    pub max_linear_velocity: f32,
    pub cautious_linear_velocity: f32,
    pub max_angular_velocity: f32,
}

impl Default for PlannerConfig {
    fn default() -> Self {
        Self {
            max_linear_velocity: 8.0,
            cautious_linear_velocity: 2.0,
            max_angular_velocity: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannerInput {
    pub machine_id: MachineId,
    pub timestamp: MonotonicMicros,
    pub state: MachineState,
    pub requested_command: ActionCommand,
    pub obstacle_count: usize,
    pub workers_nearby: usize,
    pub route_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlaResult {
    pub status: PlannerStatus,
    pub proposal: Option<ActionProposal>,
    pub explanation: String,
}

#[derive(Debug, Error)]
pub enum VlaError {
    #[error("invalid planner request: {0}")]
    InvalidRequest(String),
}

#[derive(Debug, Clone)]
pub struct DeterministicPlanner {
    config: PlannerConfig,
}

impl DeterministicPlanner {
    pub fn new(config: PlannerConfig) -> Self {
        Self { config }
    }

    pub fn plan(&self, input: &PlannerInput) -> Result<VlaResult, VlaError> {
        if input.route_label.trim().is_empty() {
            return Err(VlaError::InvalidRequest(
                "route_label must not be empty".to_string(),
            ));
        }

        let mut command = input.requested_command;
        let (status, explanation) = if input.requested_command.emergency_stop {
            command = ActionCommand {
                emergency_stop: true,
                ..ActionCommand::default()
            };
            (
                PlannerStatus::Fallback,
                "planner honored explicit emergency stop".to_string(),
            )
        } else if input.workers_nearby > 0 || input.obstacle_count > 0 {
            command.linear_velocity = command.linear_velocity.clamp(
                -self.config.cautious_linear_velocity,
                self.config.cautious_linear_velocity,
            );
            command.angular_velocity = command.angular_velocity.clamp(
                -self.config.max_angular_velocity,
                self.config.max_angular_velocity,
            );
            (
                PlannerStatus::Fallback,
                format!(
                    "planner switched to cautious mode for {} workers and {} obstacles",
                    input.workers_nearby, input.obstacle_count
                ),
            )
        } else {
            command.linear_velocity = command.linear_velocity.clamp(
                -self.config.max_linear_velocity,
                self.config.max_linear_velocity,
            );
            command.angular_velocity = command.angular_velocity.clamp(
                -self.config.max_angular_velocity,
                self.config.max_angular_velocity,
            );
            (
                PlannerStatus::Ready,
                format!("planner approved route {}", input.route_label),
            )
        };

        Ok(VlaResult {
            status,
            proposal: Some(ActionProposal {
                proposal_id: EventId::default(),
                machine_id: input.machine_id,
                timestamp: input.timestamp,
                planner: "deterministic-alpha-planner".to_string(),
                confidence: if status == PlannerStatus::Ready {
                    0.85
                } else {
                    0.55
                },
                command,
                rationale: explanation.clone(),
            }),
            explanation,
        })
    }
}

impl Default for DeterministicPlanner {
    fn default() -> Self {
        Self::new(PlannerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> PlannerInput {
        PlannerInput {
            machine_id: MachineId::test_id(1),
            timestamp: MonotonicMicros::new(1),
            state: MachineState::default(),
            requested_command: ActionCommand {
                linear_velocity: 6.0,
                angular_velocity: 0.3,
                emergency_stop: false,
            },
            obstacle_count: 0,
            workers_nearby: 0,
            route_label: "alpha-route".to_string(),
        }
    }

    #[test]
    fn ready_plan_generates_proposal() {
        let result = DeterministicPlanner::default().plan(&input()).unwrap();
        assert_eq!(result.status, PlannerStatus::Ready);
        assert!(result.proposal.is_some());
    }

    #[test]
    fn cautious_mode_reduces_speed() {
        let mut planner_input = input();
        planner_input.workers_nearby = 2;
        let result = DeterministicPlanner::default()
            .plan(&planner_input)
            .unwrap();
        assert_eq!(result.status, PlannerStatus::Fallback);
        assert!(
            result.proposal.unwrap().command.linear_velocity.abs()
                <= PlannerConfig::default().cautious_linear_velocity
        );
    }
}
