//! Canonical telemetry observations consumed by actuarial models.

use crate::{
    Degrees, MachineId, Meters, MetersPerSecond, MetersPerSecondSquared, MonotonicMicros, WorkerId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneType {
    Geofence,
    BlindSpot,
    Trench,
    Slope,
    WorkerArea,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlModeType {
    Manual,
    Autonomous,
    Remote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisibilityConditions {
    Clear,
    Fog,
    Rain,
    Dust,
    Night,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CanonicalObservation {
    Motion {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        speed: MetersPerSecond,
        acceleration: MetersPerSecondSquared,
        jerk: MetersPerSecondSquared,
        distance_delta: f32,
    },
    Pose {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        x: Meters,
        y: Meters,
        heading: Degrees,
    },
    Proximity {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        worker_id: WorkerId,
        distance: Meters,
    },
    Zone {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        zone_type: ZoneType,
        entered: bool,
    },
    Health {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        engine_temp: f32,
        fuel_level: f32,
        maintenance_due: bool,
    },
    ControlMode {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        mode: ControlModeType,
    },
    Visibility {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        visibility: Meters,
        conditions: VisibilityConditions,
    },
    Load {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        load_percentage: f32,
    },
}

impl CanonicalObservation {
    pub fn machine_id(&self) -> MachineId {
        match self {
            Self::Motion { machine_id, .. }
            | Self::Pose { machine_id, .. }
            | Self::Proximity { machine_id, .. }
            | Self::Zone { machine_id, .. }
            | Self::Health { machine_id, .. }
            | Self::ControlMode { machine_id, .. }
            | Self::Visibility { machine_id, .. }
            | Self::Load { machine_id, .. } => *machine_id,
        }
    }

    pub fn timestamp(&self) -> MonotonicMicros {
        match self {
            Self::Motion { timestamp, .. }
            | Self::Pose { timestamp, .. }
            | Self::Proximity { timestamp, .. }
            | Self::Zone { timestamp, .. }
            | Self::Health { timestamp, .. }
            | Self::ControlMode { timestamp, .. }
            | Self::Visibility { timestamp, .. }
            | Self::Load { timestamp, .. } => *timestamp,
        }
    }
}
