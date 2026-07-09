//! Exposure measurement — the denominators of actuarial rates.
//!
//! Exposure is "how much was at risk." Without correct exposure,
//! every rate is meaningless. 20 events in 10 hours ≠ 20 events in 10,000 hours.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::*;

/// Complete exposure snapshot for a policy period.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExposureBundle {
    /// Total machine-hours of operation.
    pub machine_hours: Hours,
    /// Number of distinct machine-days of operation.
    pub machine_days: u32,
    /// Number of unique machines.
    pub unique_machines: u16,
    /// Hours in autonomous/assist mode.
    pub autonomous_hours: Hours,
    /// Hours in manual mode.
    pub manual_hours: Hours,
    /// Fraction of time in autonomous mode.
    pub autonomous_fraction: UnitFloat,
    /// Hours during night shifts.
    pub night_hours: Hours,
    /// Fraction of time during night.
    pub night_fraction: UnitFloat,
    /// Total reverse travel hours.
    pub reverse_hours: Hours,
    /// Total active swing hours.
    pub swing_hours: Hours,
    /// Loaded cycles (haul trucks, loaders).
    pub loaded_cycles: u32,
    /// Estimated tonnage moved.
    pub tonnage_moved_kg: Kilograms,
    /// Estimated trench meters excavated.
    pub trench_meters: Meters,
    /// Total haul distance.
    pub haul_distance: Meters,
    /// Worker-density-hours (workers × hours in proximity).
    pub worker_density_hours: f64,
}

/// Incrementally build an exposure bundle.
#[derive(Clone, Debug, Default)]
pub struct ExposureAccumulator {
    total_hours: f64,
    auto_hours: f64,
    manual_hours: f64,
    night_hours: f64,
    reverse_hours: f64,
    swing_hours: f64,
    machine_days: u32,
    machines_seen: u16,
    loaded_cycles: u32,
}

impl ExposureAccumulator {
    pub fn new() -> Self { Self::default() }

    /// Record one hour of machine operation.
    pub fn record_hour(
        &mut self,
        is_autonomous: bool,
        is_night: bool,
        is_reversing: bool,
        is_swinging: bool,
    ) {
        self.total_hours += 1.0;
        if is_autonomous { self.auto_hours += 1.0; }
        else { self.manual_hours += 1.0; }
        if is_night { self.night_hours += 1.0; }
        if is_reversing { self.reverse_hours += 1.0; }
        if is_swinging { self.swing_hours += 1.0; }
    }

    /// Finalize into an ExposureBundle.
    pub fn finalize(&self) -> ExposureBundle {
        let total = self.total_hours.max(1.0);
        ExposureBundle {
            machine_hours: Hours::new(self.total_hours),
            machine_days: self.machine_days,
            unique_machines: self.machines_seen,
            autonomous_hours: Hours::new(self.auto_hours),
            manual_hours: Hours::new(self.manual_hours),
            autonomous_fraction: UnitFloat::clamped(self.auto_hours / total),
            night_hours: Hours::new(self.night_hours),
            night_fraction: UnitFloat::clamped(self.night_hours / total),
            reverse_hours: Hours::new(self.reverse_hours),
            swing_hours: Hours::new(self.swing_hours),
            loaded_cycles: self.loaded_cycles,
            ..Default::default()
        }
    }
}
