//! Machine and system status types.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Control mode of a machine.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ControlMode {
    Manual,
    Teleoperated,
    AutonomousAssist,
    FullyAutonomous,
}

/// Direction of travel.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TravelDirection {
    Forward,
    Reverse,
    Stationary,
}

/// Reason for an operator takeover event.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TakeoverReason {
    OperatorInitiated,
    SafetyIntervention,
    SystemFault,
    ConnectivityLoss,
    GeofenceBreach,
    ProximityOverride,
    VisibilityDegraded,
    Unknown,
}

/// Health status of a machine subsystem.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HealthStatus {
    Normal,
    Warning,
    Critical,
    Fault,
    Unknown,
}

/// Shift classification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShiftType {
    Day,
    Night,
    Swing,
}
