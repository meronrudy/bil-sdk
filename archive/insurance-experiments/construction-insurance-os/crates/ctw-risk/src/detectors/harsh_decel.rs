use ctw_core::*;
use ctw_ingest::observation::MotionObservation;
use super::Detector;
use crate::events::*;

/// Detects harsh deceleration events.
pub struct HarshDecelDetector {
    threshold_mps2: f64,
    event_counter: u64,
    site_id: SiteId,
}

impl HarshDecelDetector {
    pub fn new(threshold_mps2: f64, site_id: SiteId) -> Self {
        Self { threshold_mps2, event_counter: 0, site_id }
    }
}

impl Detector for HarshDecelDetector {
    type Input = MotionObservation;

    fn step(&mut self, obs: &MotionObservation) -> Option<RiskEvent> {
        let decel = obs.acceleration.raw();
        if decel.abs() <= self.threshold_mps2 {
            return None;
        }
        self.event_counter += 1;
        let severity = UnitFloat::clamped(decel.abs() / 10.0);
        Some(RiskEvent {
            id: EventId::new(self.event_counter as u128),
            timestamp: obs.timestamp,
            machine_id: obs.machine_id,
            site_id: self.site_id,
            event_type: RiskEventType::HarshDeceleration,
            severity,
            confidence: Confidence::new_unchecked(0.95),
            details: EventDetails::HarshDecel {
                decel_mps2: decel,
                jerk_mps3: obs.jerk.raw(),
                speed_at_event: obs.speed,
            },
            schema_version: FeatureVersion::new(1, 0, 0),
        })
    }

    fn reset(&mut self) { self.event_counter = 0; }
    fn name(&self) -> &'static str { "harsh_decel" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctw_core::*;

    fn make_obs(accel: f64) -> MotionObservation {
        MotionObservation {
            timestamp: MonotonicMicros::new(0),
            machine_id: MachineId::new(1),
            speed: MetersPerSecond::new(2.0),
            acceleration: MetersPerSecondSq::new(accel),
            jerk: MetersPerSecondCubed::new(0.0),
            yaw_rate: RadiansPerSecond::new(0.0),
            direction: TravelDirection::Forward,
        }
    }

    #[test]
    fn below_threshold_no_event() {
        let mut d = HarshDecelDetector::new(3.5, SiteId::new(1));
        assert!(d.step(&make_obs(-2.0)).is_none());
    }

    #[test]
    fn above_threshold_emits_event() {
        let mut d = HarshDecelDetector::new(3.5, SiteId::new(1));
        let e = d.step(&make_obs(-5.0));
        assert!(e.is_some());
        assert_eq!(e.unwrap().event_type, RiskEventType::HarshDeceleration);
    }
}
