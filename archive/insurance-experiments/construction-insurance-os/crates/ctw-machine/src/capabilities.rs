//! Capability traits: what can this machine report?

use ctw_core::*;

/// A machine that can report its motion state.
pub trait MotionSource {
    fn linear_speed(&self) -> Option<MetersPerSecond>;
    fn longitudinal_accel(&self) -> Option<MetersPerSecondSq>;
    fn lateral_accel(&self) -> Option<MetersPerSecondSq>;
    fn yaw_rate(&self) -> Option<RadiansPerSecond>;
    fn travel_direction(&self) -> Option<TravelDirection>;
}

/// A machine that can report its pose.
pub trait PoseSource {
    fn position_local(&self) -> Option<[f64; 3]>;
    fn heading(&self) -> Option<Radians>;
    fn pitch(&self) -> Option<Radians>;
    fn roll(&self) -> Option<Radians>;
}

/// An excavator that can report arm kinematics.
pub trait ExcavatorKinematics {
    fn boom_angle(&self) -> Option<Radians>;
    fn stick_angle(&self) -> Option<Radians>;
    fn bucket_angle(&self) -> Option<Radians>;
    fn swing_angle(&self) -> Option<Radians>;
    fn swing_rate(&self) -> Option<RadiansPerSecond>;
}

/// A crane that can report lift state.
pub trait CraneKinematics {
    fn boom_length(&self) -> Option<Meters>;
    fn boom_angle(&self) -> Option<Radians>;
    fn load_weight(&self) -> Option<Kilograms>;
    fn rated_capacity_at_radius(&self, radius: Meters) -> Option<Kilograms>;
    fn swing_rate(&self) -> Option<RadiansPerSecond>;
}

/// A machine that can report its health.
pub trait HealthSource {
    fn engine_hours(&self) -> Option<Hours>;
    fn engine_temp(&self) -> Option<Celsius>;
    fn hydraulic_pressure(&self) -> Option<Bar>;
    fn maintenance_due_in(&self) -> Option<Hours>;
    fn active_fault_codes(&self) -> &[u32];
}
