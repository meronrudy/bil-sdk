//! Deterministic guardrail layer for replay and alpha integration.

pub mod config;
pub mod constraints;
pub mod controllers;
pub mod models;
pub mod runtime;

use contracts::MachineId;
use serde::{Deserialize, Serialize};
use shared::{MachineState, Obstacle, Position, Velocity, Worker};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyState {
    pub machine_id: MachineId,
    pub position: Position,
    pub velocity: Velocity,
    pub obstacles: Vec<Obstacle>,
    pub workers: Vec<Worker>,
    pub machine_state: MachineState,
}

impl Default for SafetyState {
    fn default() -> Self {
        Self {
            machine_id: MachineId::default(),
            position: Position::default(),
            velocity: Velocity::default(),
            obstacles: Vec::new(),
            workers: Vec::new(),
            machine_state: MachineState::default(),
        }
    }
}

#[derive(Error, Debug)]
pub enum SafetyError {
    #[error("constraint violation: {0}")]
    ConstraintViolation(String),
    #[error("replay failed: {0}")]
    Replay(String),
}
