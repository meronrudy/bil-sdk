//! Risk signal detection over canonical observations.

use contracts::{CanonicalObservation, ControlModeType, RiskEvent, ZoneType};

pub trait Detector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent>;
}

#[derive(Debug, Default)]
pub struct RiskDetector {
    pub harsh_decel_jerk: HarshDecelJerkDetector,
    pub geofence_incursion: GeofenceIncursionDetector,
    pub worker_proximity: WorkerProximityDetector,
    pub reverse_time_fraction: ReverseTimeFractionDetector,
    pub blind_spot_occupancy: BlindSpotOccupancyDetector,
    pub operator_takeover: OperatorTakeoverDetector,
    pub maintenance_deferral: MaintenanceDeferralDetector,
    pub slope_trench_edge: SlopeTrenchEdgeDetector,
}

impl RiskDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        self.harsh_decel_jerk
            .step(obs)
            .or_else(|| self.geofence_incursion.step(obs))
            .or_else(|| self.worker_proximity.step(obs))
            .or_else(|| self.reverse_time_fraction.step(obs))
            .or_else(|| self.blind_spot_occupancy.step(obs))
            .or_else(|| self.operator_takeover.step(obs))
            .or_else(|| self.maintenance_deferral.step(obs))
            .or_else(|| self.slope_trench_edge.step(obs))
    }
}

#[derive(Debug, Default)]
pub struct HarshDecelJerkDetector;

impl Detector for HarshDecelJerkDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        match obs {
            CanonicalObservation::Motion {
                machine_id,
                timestamp,
                acceleration,
                jerk,
                ..
            } if acceleration.value() < -29.4 => Some(RiskEvent::HarshDecel {
                machine_id: *machine_id,
                timestamp: *timestamp,
                deceleration: *acceleration,
            }),
            CanonicalObservation::Motion {
                machine_id,
                timestamp,
                jerk,
                ..
            } if jerk.value().abs() > 98.0 => Some(RiskEvent::HarshDecelJerk {
                machine_id: *machine_id,
                timestamp: *timestamp,
                jerk: *jerk,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct GeofenceIncursionDetector;

impl Detector for GeofenceIncursionDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        match obs {
            CanonicalObservation::Zone {
                machine_id,
                timestamp,
                zone_type: ZoneType::Geofence,
                entered: true,
            } => Some(RiskEvent::GeofenceIncursion {
                machine_id: *machine_id,
                timestamp: *timestamp,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct WorkerProximityDetector;

impl Detector for WorkerProximityDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        match obs {
            CanonicalObservation::Proximity {
                machine_id,
                timestamp,
                worker_id,
                distance,
            } if distance.value() < 5.0 => Some(RiskEvent::WorkerProximity {
                machine_id: *machine_id,
                timestamp: *timestamp,
                worker_id: *worker_id,
                distance: *distance,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct ReverseTimeFractionDetector {
    total_steps: u32,
    reverse_steps: u32,
}

impl Detector for ReverseTimeFractionDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        let CanonicalObservation::Motion {
            machine_id,
            timestamp,
            speed,
            ..
        } = obs
        else {
            return None;
        };

        self.total_steps += 1;
        if speed.value() < 0.0 {
            self.reverse_steps += 1;
        }

        if self.total_steps >= 10 {
            let fraction = self.reverse_steps as f32 / self.total_steps as f32;
            if fraction > 0.2 {
                return Some(RiskEvent::ReverseTimeFraction {
                    machine_id: *machine_id,
                    timestamp: *timestamp,
                    fraction,
                });
            }
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct BlindSpotOccupancyDetector;

impl Detector for BlindSpotOccupancyDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        match obs {
            CanonicalObservation::Zone {
                machine_id,
                timestamp,
                zone_type: ZoneType::BlindSpot,
                entered: true,
            } => Some(RiskEvent::BlindSpotOccupancy {
                machine_id: *machine_id,
                timestamp: *timestamp,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct OperatorTakeoverDetector {
    saw_autonomous: bool,
}

impl Detector for OperatorTakeoverDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        let CanonicalObservation::ControlMode {
            machine_id,
            timestamp,
            mode,
        } = obs
        else {
            return None;
        };

        match mode {
            ControlModeType::Autonomous => {
                self.saw_autonomous = true;
                None
            }
            ControlModeType::Manual if self.saw_autonomous => Some(RiskEvent::OperatorTakeover {
                machine_id: *machine_id,
                timestamp: *timestamp,
                mode: *mode,
            }),
            ControlModeType::Manual => Some(RiskEvent::OperatorTakeover {
                machine_id: *machine_id,
                timestamp: *timestamp,
                mode: *mode,
            }),
            ControlModeType::Remote => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct MaintenanceDeferralDetector;

impl Detector for MaintenanceDeferralDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        match obs {
            CanonicalObservation::Health {
                machine_id,
                timestamp,
                maintenance_due: true,
                ..
            } => Some(RiskEvent::MaintenanceDeferral {
                machine_id: *machine_id,
                timestamp: *timestamp,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct SlopeTrenchEdgeDetector;

impl Detector for SlopeTrenchEdgeDetector {
    fn step(&mut self, obs: &CanonicalObservation) -> Option<RiskEvent> {
        match obs {
            CanonicalObservation::Zone {
                machine_id,
                timestamp,
                zone_type: zone_type @ (ZoneType::Slope | ZoneType::Trench),
                entered: true,
            } => Some(RiskEvent::SlopeTrenchEdge {
                machine_id: *machine_id,
                timestamp: *timestamp,
                zone_type: *zone_type,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use contracts::{
        MachineId, Meters, MetersPerSecond, MetersPerSecondSquared, MonotonicMicros, WorkerId,
    };

    #[test]
    fn detects_harsh_deceleration() {
        let obs = CanonicalObservation::Motion {
            machine_id: MachineId::test_id(1),
            timestamp: MonotonicMicros::new(1),
            speed: MetersPerSecond(3.0),
            acceleration: MetersPerSecondSquared(-30.0),
            jerk: MetersPerSecondSquared(0.0),
            distance_delta: 1.0,
        };
        assert!(matches!(
            RiskDetector::new().step(&obs),
            Some(RiskEvent::HarshDecel { .. })
        ));
    }

    #[test]
    fn detects_worker_proximity() {
        let obs = CanonicalObservation::Proximity {
            machine_id: MachineId::test_id(1),
            timestamp: MonotonicMicros::new(1),
            worker_id: WorkerId::test_id(1),
            distance: Meters(2.5),
        };
        assert!(matches!(
            RiskDetector::new().step(&obs),
            Some(RiskEvent::WorkerProximity { .. })
        ));
    }
}
