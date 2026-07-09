//! Individual risk event detectors.
//!
//! Each detector is a small state machine that consumes observations
//! and optionally emits a risk event.

pub mod harsh_decel;
pub mod worker_proximity;
pub mod geofence;
pub mod overswing;

/// The core detector trait.
pub trait Detector {
    /// The observation type this detector consumes.
    type Input;
    /// Process one observation, optionally producing a risk event.
    fn step(&mut self, input: &Self::Input) -> Option<super::RiskEvent>;
    /// Reset detector state.
    fn reset(&mut self);
    /// Detector name for logging/audit.
    fn name(&self) -> &'static str;
}
