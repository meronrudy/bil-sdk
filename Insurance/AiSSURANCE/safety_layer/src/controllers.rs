//! Safety controllers that may modify or reject VLA proposals.

use crate::{SafetyError, SafetyState};
use contracts::{ActionCommand, ActionProposal, SafetyEventType};
use shared::Position;

#[derive(Debug, Clone)]
pub struct ControllerOverride {
    pub command: ActionCommand,
    pub event_type: SafetyEventType,
    pub reason: String,
}

pub trait SafetyController {
    fn check_and_override(
        &self,
        state: &SafetyState,
        proposal: &ActionProposal,
    ) -> Result<Option<ControllerOverride>, SafetyError>;
}

pub struct EmergencyStopController;

impl SafetyController for EmergencyStopController {
    fn check_and_override(
        &self,
        state: &SafetyState,
        proposal: &ActionProposal,
    ) -> Result<Option<ControllerOverride>, SafetyError> {
        if proposal.command.emergency_stop
            || state.machine_state.health < 0.5
            || state.machine_state.load > 1.5
        {
            return Ok(Some(ControllerOverride {
                command: ActionCommand {
                    emergency_stop: true,
                    ..ActionCommand::default()
                },
                event_type: SafetyEventType::EmergencyStop,
                reason: "emergency stop triggered by planner request or degraded machine health"
                    .to_string(),
            }));
        }
        Ok(None)
    }
}

pub struct CollisionAvoidanceController;

impl SafetyController for CollisionAvoidanceController {
    fn check_and_override(
        &self,
        state: &SafetyState,
        _proposal: &ActionProposal,
    ) -> Result<Option<ControllerOverride>, SafetyError> {
        let min_obstacle_distance = state
            .obstacles
            .iter()
            .map(|obstacle| distance(&state.position, &obstacle.position) - obstacle.radius)
            .chain(
                state
                    .workers
                    .iter()
                    .map(|worker| distance(&state.position, &worker.position)),
            )
            .fold(f32::INFINITY, f32::min);

        if min_obstacle_distance < 2.0 {
            return Ok(Some(ControllerOverride {
                command: ActionCommand::default(),
                event_type: SafetyEventType::CollisionAvoidanceOverride,
                reason: format!(
                    "nearest obstacle/worker distance {:.2}m under 2.0m threshold",
                    min_obstacle_distance
                ),
            }));
        }

        Ok(None)
    }
}

pub struct GeofenceController;

impl SafetyController for GeofenceController {
    fn check_and_override(
        &self,
        state: &SafetyState,
        _proposal: &ActionProposal,
    ) -> Result<Option<ControllerOverride>, SafetyError> {
        if state.position.x < -100.0
            || state.position.x > 100.0
            || state.position.y < -100.0
            || state.position.y > 100.0
        {
            return Ok(Some(ControllerOverride {
                command: ActionCommand::default(),
                event_type: SafetyEventType::GeofenceOverride,
                reason: "position outside configured alpha geofence".to_string(),
            }));
        }
        Ok(None)
    }
}

pub struct StabilityController;

impl SafetyController for StabilityController {
    fn check_and_override(
        &self,
        state: &SafetyState,
        proposal: &ActionProposal,
    ) -> Result<Option<ControllerOverride>, SafetyError> {
        let slope = (state.position.z / 10.0).abs();
        let max_speed = 5.0 / (1.0 + slope);

        if proposal.command.linear_velocity.abs() > max_speed || state.machine_state.load > 1.0 {
            return Ok(Some(ControllerOverride {
                command: ActionCommand {
                    linear_velocity: proposal.command.linear_velocity.signum()
                        * max_speed.min(proposal.command.linear_velocity.abs()),
                    angular_velocity: proposal.command.angular_velocity,
                    emergency_stop: false,
                },
                event_type: SafetyEventType::StabilityReduction,
                reason: format!(
                    "reduced speed for slope {:.2} and load {:.2}",
                    slope, state.machine_state.load
                ),
            }));
        }
        Ok(None)
    }
}

pub struct LoadChartController;

impl SafetyController for LoadChartController {
    fn check_and_override(
        &self,
        state: &SafetyState,
        _proposal: &ActionProposal,
    ) -> Result<Option<ControllerOverride>, SafetyError> {
        if state.machine_state.load > 1.0 {
            return Ok(Some(ControllerOverride {
                command: ActionCommand::default(),
                event_type: SafetyEventType::LoadLimitStop,
                reason: "load exceeds alpha load chart".to_string(),
            }));
        }
        Ok(None)
    }
}

fn distance(a: &Position, b: &Position) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}
