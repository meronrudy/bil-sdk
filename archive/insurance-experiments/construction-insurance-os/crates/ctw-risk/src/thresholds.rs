//! Detector thresholds — versioned, auditable, immutable per model period.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::*;

/// Complete threshold configuration for all detectors.
/// This is versioned because changing a threshold is a model change.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThresholdConfig {
    pub version: FeatureVersion,
    pub harsh_decel_mps2: f64,
    pub harsh_accel_mps2: f64,
    pub worker_proximity_m: f64,
    pub worker_proximity_critical_m: f64,
    pub equipment_proximity_m: f64,
    pub geofence_incursion_m: f64,
    pub overswing_rad_s: f64,
    pub overspeed_mps: f64,
    pub maintenance_overdue_hrs: f64,
    pub slope_danger_deg: f64,
    pub overload_fraction: f64,
    pub trench_edge_margin_m: f64,
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            version: FeatureVersion::new(1, 0, 0),
            harsh_decel_mps2: 3.5,
            harsh_accel_mps2: 3.0,
            worker_proximity_m: 2.0,
            worker_proximity_critical_m: 1.0,
            equipment_proximity_m: 3.0,
            geofence_incursion_m: 0.0,
            overswing_rad_s: 0.6,
            overspeed_mps: 4.0,
            maintenance_overdue_hrs: 50.0,
            slope_danger_deg: 15.0,
            overload_fraction: 1.0,
            trench_edge_margin_m: 1.5,
        }
    }
}
