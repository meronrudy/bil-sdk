//! Underwriting feature bundles created from telemetry windows.

use crate::{FeatureVersion, MachineId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BehaviorFeatures {
    pub harsh_deceleration_count: u32,
    pub geofence_violation_count: u32,
    pub worker_proximity_count: u32,
    pub reverse_time_fraction: f32,
    pub blind_spot_occupancy_count: u32,
    pub operator_takeover_count: u32,
    pub maintenance_deferral_count: u32,
    pub slope_trench_edge_count: u32,
}

impl Default for BehaviorFeatures {
    fn default() -> Self {
        Self {
            harsh_deceleration_count: 0,
            geofence_violation_count: 0,
            worker_proximity_count: 0,
            reverse_time_fraction: 0.0,
            blind_spot_occupancy_count: 0,
            operator_takeover_count: 0,
            maintenance_deferral_count: 0,
            slope_trench_edge_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExposureFeatures {
    pub total_distance_meters: f32,
    pub total_hours: f32,
    pub average_load_factor: f32,
}

impl Default for ExposureFeatures {
    fn default() -> Self {
        Self {
            total_distance_meters: 0.0,
            total_hours: 0.0,
            average_load_factor: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ContextFeatures {
    pub average_speed_mps: f32,
    pub low_visibility_hours: f32,
    pub slope_exposure_hours: f32,
}

impl Default for ContextFeatures {
    fn default() -> Self {
        Self {
            average_speed_mps: 0.0,
            low_visibility_hours: 0.0,
            slope_exposure_hours: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskFeatureBundle {
    pub machine_id: Option<MachineId>,
    pub version: FeatureVersion,
    pub exposure: ExposureFeatures,
    pub behavior: BehaviorFeatures,
    pub context: ContextFeatures,
}

impl RiskFeatureBundle {
    pub fn feature_vector(&self) -> [f32; 8] {
        [
            self.behavior.harsh_deceleration_count as f32,
            self.behavior.geofence_violation_count as f32,
            self.behavior.worker_proximity_count as f32,
            self.behavior.maintenance_deferral_count as f32,
            self.exposure.total_hours,
            self.exposure.average_load_factor,
            self.context.average_speed_mps,
            self.context.low_visibility_hours,
        ]
    }
}
