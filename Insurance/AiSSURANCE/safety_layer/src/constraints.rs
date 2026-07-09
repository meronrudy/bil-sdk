//! # Safety Constraints
//!
//! Deployable safety policy for construction autonomy. Static and dynamic
//! constraints define the operating envelope that keeps machines, workers, and
//! sites inside known limits with <1ms validation lookups.
//!
//! ## Constraint Families
//! - envelope: Position/velocity bounds
//! - human_proximity: Worker distance thresholds
//! - trench_edge_rules: Depth/angle limits for trenches
//! - slope_limits: Grade constraints for stability
//!
//! ## Deployment Role
//! Used by controllers and runtime for validation.
//! Configurable via SafetyConfig.

use crate::SafetyError;
use shared::{Position, Velocity};

#[derive(Debug, Clone)]
pub struct SafetyConstraints {
    pub max_velocity: f32,
    pub min_distance_to_workers: f32,
    pub max_trench_depth: f32,
    pub max_slope_angle: f32,
    pub geofence_bounds: (Position, Position), // min, max
}

impl SafetyConstraints {
    pub fn new() -> Self {
        Self {
            max_velocity: 10.0,           // m/s
            min_distance_to_workers: 2.0, // m
            max_trench_depth: 5.0,        // m
            max_slope_angle: 30.0,        // degrees
            geofence_bounds: (
                Position {
                    x: -100.0,
                    y: -100.0,
                    z: -10.0,
                },
                Position {
                    x: 100.0,
                    y: 100.0,
                    z: 10.0,
                },
            ),
        }
    }

    pub fn check_envelope(
        &self,
        position: &Position,
        velocity: &Velocity,
    ) -> Result<(), SafetyError> {
        if position.x < self.geofence_bounds.0.x
            || position.x > self.geofence_bounds.1.x
            || position.y < self.geofence_bounds.0.y
            || position.y > self.geofence_bounds.1.y
            || position.z < self.geofence_bounds.0.z
            || position.z > self.geofence_bounds.1.z
        {
            return Err(SafetyError::ConstraintViolation(
                "Position outside envelope".to_string(),
            ));
        }
        let speed = (velocity.vx.powi(2) + velocity.vy.powi(2) + velocity.vz.powi(2)).sqrt();
        if speed > self.max_velocity {
            return Err(SafetyError::ConstraintViolation(
                "Velocity exceeds limit".to_string(),
            ));
        }
        Ok(())
    }

    pub fn check_human_proximity(
        &self,
        machine_pos: &Position,
        worker_positions: &[Position],
    ) -> Result<(), SafetyError> {
        for worker_pos in worker_positions {
            let dist = ((machine_pos.x - worker_pos.x).powi(2)
                + (machine_pos.y - worker_pos.y).powi(2)
                + (machine_pos.z - worker_pos.z).powi(2))
            .sqrt();
            if dist < self.min_distance_to_workers {
                return Err(SafetyError::ConstraintViolation(
                    "Too close to worker".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn check_trench_edge_rules(
        &self,
        position: &Position,
        trench_depth: f32,
        trench_angle: f32,
    ) -> Result<(), SafetyError> {
        if trench_depth > self.max_trench_depth {
            return Err(SafetyError::ConstraintViolation(
                "Trench too deep".to_string(),
            ));
        }
        if trench_angle > 45.0 {
            // Example angle limit
            return Err(SafetyError::ConstraintViolation(
                "Trench angle too steep".to_string(),
            ));
        }
        // Check proximity to trench edge (simplified)
        if position.z < -trench_depth + 1.0 {
            // 1m margin
            return Err(SafetyError::ConstraintViolation(
                "Too close to trench edge".to_string(),
            ));
        }
        Ok(())
    }

    pub fn check_slope_limits(&self, slope_angle: f32) -> Result<(), SafetyError> {
        if slope_angle > self.max_slope_angle {
            return Err(SafetyError::ConstraintViolation(
                "Slope too steep".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for SafetyConstraints {
    fn default() -> Self {
        Self::new()
    }
}
