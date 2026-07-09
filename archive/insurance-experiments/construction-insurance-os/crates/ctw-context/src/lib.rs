#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::*;

/// Site type classification for peer-group normalization.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SiteType {
    UrbanFoundation,
    HighwayCivil,
    IndustrialHeavy,
    Residential,
    BridgeStructure,
    Demolition,
    Utility,
    Mining,
}

/// Weather severity for context normalization.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WeatherCondition {
    Clear, Rain, Fog, Snow, HighWind, Thunderstorm, Extreme,
}

/// Task classification for exposure grouping.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TaskClass {
    Excavation, Grading, Hauling, Lifting, Demolition,
    Trenching, Loading, Paving, Drilling, Compaction, Piling,
}

/// Complete context bundle (C in the actuarial formula).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContextBundle {
    pub low_visibility_fraction: UnitFloat,
    pub rain_fraction: UnitFloat,
    pub mean_terrain_slope_deg: f64,
    pub mean_soil_stability: f64,
    pub mean_workers_nearby: f64,
    pub maintenance_compliance: UnitFloat,
    pub weather_severity_index: f64,
}
