//! Canonical observations: the output of the ingest pipeline.
//!
//! These are the standardized, vendor-neutral observations that
//! downstream systems (risk detectors, exposure counters) consume.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::*;
use ctw_geo::Point3;

/// A canonical observation produced by the ingest pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Observation {
    /// Machine motion state.
    Motion(MotionObservation),
    /// Machine pose/position.
    Pose(PoseObservation),
    /// Distance to nearest entity (worker, equipment, boundary).
    Proximity(ProximityObservation),
    /// Geofence/zone membership change.
    ZoneMembership(ZoneObservation),
    /// Machine health snapshot.
    Health(HealthObservation),
    /// Control mode change.
    ControlChange(ControlObservation),
    /// Visibility conditions.
    Visibility(VisibilityObservation),
    /// Load/capacity state (cranes, loaders).
    Load(LoadObservation),
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MotionObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub speed: MetersPerSecond,
    pub acceleration: MetersPerSecondSq,
    pub jerk: MetersPerSecondCubed,
    pub yaw_rate: RadiansPerSecond,
    pub direction: TravelDirection,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PoseObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub position: Point3,
    pub heading: Radians,
    pub pitch: Radians,
    pub roll: Radians,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProximityObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub target_type: ProximityTarget,
    pub distance: Meters,
    pub bearing: Option<Radians>,
    pub confidence: Confidence,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProximityTarget {
    Worker,
    Equipment,
    Vehicle,
    Structure,
    TrenchEdge,
    ZoneBoundary,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ZoneObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub zone_id: ZoneId,
    pub margin: Meters,
    pub inside: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HealthObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub status: HealthStatus,
    pub engine_hours: Hours,
    pub maintenance_due_in: Hours,
    pub active_faults: alloc::vec::Vec<u32>,
}

extern crate alloc;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ControlObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub previous_mode: ControlMode,
    pub new_mode: ControlMode,
    pub reason: TakeoverReason,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VisibilityObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub visibility_class: VisibilityClass,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VisibilityClass {
    Good,
    Reduced,
    Low,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LoadObservation {
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub current_load_kg: Kilograms,
    pub rated_capacity_kg: Kilograms,
    pub load_fraction: UnitFloat,
}
