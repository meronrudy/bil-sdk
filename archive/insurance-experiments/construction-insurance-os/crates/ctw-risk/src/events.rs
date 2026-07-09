//! Risk event types — the atomic unit of insurable behavior measurement.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::*;

/// Every risk event the system can detect.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RiskEvent {
    pub id: EventId,
    pub timestamp: MonotonicMicros,
    pub machine_id: MachineId,
    pub site_id: SiteId,
    pub event_type: RiskEventType,
    pub severity: UnitFloat,
    pub confidence: Confidence,
    pub details: EventDetails,
    pub schema_version: FeatureVersion,
}

/// Classification of risk events.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskEventType {
    HarshDeceleration,
    HarshAcceleration,
    WorkerProximity,
    WorkerProximityCritical,
    EquipmentProximity,
    GeofenceIncursion,
    BlindSpotSwing,
    Overswing,
    Overspeed,
    MaintenanceDeferral,
    ReverseLowVisibility,
    SlopeDanger,
    OverloadAttempt,
    TrenchEdgeApproach,
    OperatorTakeover,
    SafetyInterventionTriggered,
    HealthException,
    UnstableLoadPath,
}

/// Detailed context for each event type.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EventDetails {
    HarshDecel {
        decel_mps2: f64,
        jerk_mps3: f64,
        speed_at_event: MetersPerSecond,
    },
    WorkerProximity {
        distance: Meters,
        relative_speed: MetersPerSecond,
        worker_count: u32,
    },
    GeofenceIncursion {
        zone_id: ZoneId,
        depth: Meters,
        duration_so_far: Seconds,
    },
    BlindSpotSwing {
        swing_rate: RadiansPerSecond,
        nearest_worker_distance: Meters,
    },
    Overswing {
        swing_rate: RadiansPerSecond,
    },
    MaintenanceDeferral {
        overdue_hours: Hours,
    },
    ReverseLowVis {
        speed: MetersPerSecond,
        visibility_class: ctw_ingest::observation::VisibilityClass,
    },
    SlopeDanger {
        slope_degrees: Degrees,
    },
    OperatorTakeover {
        from_mode: ControlMode,
        to_mode: ControlMode,
        reason: TakeoverReason,
    },
    Generic {
        description: alloc::string::String,
        value: f64,
        unit: alloc::string::String,
    },
}
