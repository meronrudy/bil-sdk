//! Safety-layer runtime configuration.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub max_velocity: f32,
    pub min_distance_to_workers: f32,
    pub max_loop_millis: u64,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            max_velocity: 10.0,
            min_distance_to_workers: 2.0,
            max_loop_millis: 50,
        }
    }
}
