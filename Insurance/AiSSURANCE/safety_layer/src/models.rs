//! Replay-oriented safety models.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CollisionRiskSnapshot {
    pub nearest_distance_meters: f32,
}

impl CollisionRiskSnapshot {
    pub fn is_critical(&self, threshold_meters: f32) -> bool {
        self.nearest_distance_meters < threshold_meters
    }
}
