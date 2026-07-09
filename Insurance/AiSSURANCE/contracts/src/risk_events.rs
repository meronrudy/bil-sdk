//! Risk events emitted from canonical telemetry observations.

use crate::{
    ControlModeType, MachineId, Meters, MetersPerSecondSquared, MonotonicMicros, WorkerId, ZoneType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskEvent {
    HarshDecel {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        deceleration: MetersPerSecondSquared,
    },
    HarshDecelJerk {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        jerk: MetersPerSecondSquared,
    },
    GeofenceIncursion {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
    },
    WorkerProximity {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        worker_id: WorkerId,
        distance: Meters,
    },
    ReverseTimeFraction {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        fraction: f32,
    },
    BlindSpotOccupancy {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
    },
    OperatorTakeover {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        mode: ControlModeType,
    },
    MaintenanceDeferral {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
    },
    SlopeTrenchEdge {
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        zone_type: ZoneType,
    },
}
