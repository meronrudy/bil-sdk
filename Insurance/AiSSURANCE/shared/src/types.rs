//! # Shared Types
//!
//! Shared product primitives for the AiSSURANCE platform. These types give data
//! a consistent shape as it moves between safety, autonomy, and risk workflows.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub id: Uuid,
    pub position: Position,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub id: Uuid,
    pub position: Position,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineState {
    pub machine_id: Uuid,
    pub position: Position,
    pub velocity: Velocity,
    pub load: f32,
    pub health: f32,
}

impl Default for MachineState {
    fn default() -> Self {
        Self {
            machine_id: Uuid::new_v4(),
            position: Position::default(),
            velocity: Velocity::default(),
            load: 0.0,
            health: 1.0,
        }
    }
}
