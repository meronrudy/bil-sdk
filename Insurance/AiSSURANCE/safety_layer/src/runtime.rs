//! Replay runtime for deterministic safety evaluation.

use crate::{
    config::SafetyConfig,
    constraints::SafetyConstraints,
    controllers::{
        CollisionAvoidanceController, EmergencyStopController, GeofenceController,
        LoadChartController, SafetyController, StabilityController,
    },
    SafetyError, SafetyState,
};
use contracts::{
    ActionProposal, EventId, SafetyDecision, SafetyEvent, SafetyEventType, SafetyOutcome,
    SafetySeverity,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayStep {
    pub state: SafetyState,
    pub proposal: ActionProposal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayReport {
    pub decisions: Vec<SafetyDecision>,
    pub events: Vec<SafetyEvent>,
    pub max_latency: Duration,
}

pub struct SafetyRuntime {
    controllers: Vec<Box<dyn SafetyController>>,
    constraints: SafetyConstraints,
    config: SafetyConfig,
}

impl SafetyRuntime {
    pub fn new() -> Self {
        Self::with_config(SafetyConfig::default())
    }

    pub fn with_config(config: SafetyConfig) -> Self {
        Self {
            controllers: vec![
                Box::new(EmergencyStopController),
                Box::new(CollisionAvoidanceController),
                Box::new(GeofenceController),
                Box::new(StabilityController),
                Box::new(LoadChartController),
            ],
            constraints: SafetyConstraints::new(),
            config,
        }
    }

    pub fn evaluate(
        &self,
        state: &SafetyState,
        proposal: &ActionProposal,
    ) -> Result<(SafetyDecision, Vec<SafetyEvent>), SafetyError> {
        let start = Instant::now();

        self.constraints
            .check_envelope(&state.position, &state.velocity)?;
        self.constraints.check_human_proximity(
            &state.position,
            &state
                .workers
                .iter()
                .map(|worker| worker.position.clone())
                .collect::<Vec<_>>(),
        )?;

        let mut final_command = proposal.command;
        let mut reasons = Vec::new();
        let mut events = vec![SafetyEvent {
            event_id: EventId::default(),
            machine_id: proposal.machine_id,
            timestamp: proposal.timestamp,
            event_type: SafetyEventType::ReplayStep,
            severity: SafetySeverity::Info,
            message: "evaluated proposal in replay runtime".to_string(),
        }];
        let mut outcome = SafetyOutcome::Approved;

        for controller in &self.controllers {
            if let Some(controller_override) = controller.check_and_override(state, proposal)? {
                final_command = controller_override.command;
                reasons.push(controller_override.reason.clone());
                events.push(SafetyEvent {
                    event_id: EventId::default(),
                    machine_id: proposal.machine_id,
                    timestamp: proposal.timestamp,
                    event_type: controller_override.event_type,
                    severity: match controller_override.event_type {
                        SafetyEventType::EmergencyStop => SafetySeverity::Critical,
                        _ => SafetySeverity::Warning,
                    },
                    message: controller_override.reason,
                });
                outcome = if final_command.emergency_stop {
                    SafetyOutcome::Rejected
                } else {
                    SafetyOutcome::Modified
                };
                break;
            }
        }

        let latency = start.elapsed();
        if latency > Duration::from_millis(self.config.max_loop_millis) {
            return Err(SafetyError::Replay(format!(
                "safety replay exceeded {}ms latency budget",
                self.config.max_loop_millis
            )));
        }

        Ok((
            SafetyDecision {
                machine_id: proposal.machine_id,
                timestamp: proposal.timestamp,
                original: proposal.clone(),
                final_command,
                outcome,
                reasons,
                latency_micros: latency.as_micros() as u64,
            },
            events,
        ))
    }

    pub fn run_replay(&self, steps: &[ReplayStep]) -> Result<ReplayReport, SafetyError> {
        let mut decisions = Vec::with_capacity(steps.len());
        let mut events = Vec::new();
        let mut max_latency = Duration::ZERO;

        for step in steps {
            let (decision, mut step_events) = self.evaluate(&step.state, &step.proposal)?;
            max_latency = max_latency.max(Duration::from_micros(decision.latency_micros));
            decisions.push(decision);
            events.append(&mut step_events);
        }

        Ok(ReplayReport {
            decisions,
            events,
            max_latency,
        })
    }
}

impl Default for SafetyRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use contracts::{ActionCommand, MachineId, MonotonicMicros};

    fn proposal() -> ActionProposal {
        ActionProposal {
            proposal_id: EventId::default(),
            machine_id: MachineId::test_id(1),
            timestamp: MonotonicMicros::new(1),
            planner: "test".to_string(),
            confidence: 0.8,
            command: ActionCommand {
                linear_velocity: 8.0,
                angular_velocity: 0.0,
                emergency_stop: false,
            },
            rationale: "test".to_string(),
        }
    }

    #[test]
    fn replay_runtime_stays_under_budget() {
        let runtime = SafetyRuntime::default();
        let report = runtime
            .run_replay(&[ReplayStep {
                state: SafetyState::default(),
                proposal: proposal(),
            }])
            .unwrap();
        assert!(report.max_latency.as_millis() < 50);
        assert_eq!(report.decisions.len(), 1);
    }

    #[test]
    fn collision_override_stops_machine() {
        let mut state = SafetyState::default();
        state.obstacles.push(shared::Obstacle {
            id: uuid::Uuid::new_v4(),
            position: shared::Position {
                x: 0.5,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.1,
        });
        let runtime = SafetyRuntime::default();
        let report = runtime
            .run_replay(&[ReplayStep {
                state,
                proposal: proposal(),
            }])
            .unwrap();
        assert!(matches!(
            report.decisions[0].outcome,
            SafetyOutcome::Modified
        ));
        assert_eq!(report.decisions[0].final_command.linear_velocity, 0.0);
    }
}
