use ctw_core::*;
use ctw_ingest::observation::MotionObservation;
use super::Detector;
use crate::events::*;

/// Detects excessive swing rate on excavators and cranes.
pub struct OverswingDetector {
    threshold_rad_s: f64,
    event_counter: u64,
    site_id: SiteId,
}

impl OverswingDetector {
    pub fn new(threshold_rad_s: f64, site_id: SiteId) -> Self {
        Self { threshold_rad_s, event_counter: 0, site_id }
    }
}

impl Detector for OverswingDetector {
    type Input = MotionObservation;

    fn step(&mut self, obs: &MotionObservation) -> Option<RiskEvent> {
        let rate = obs.yaw_rate.raw().abs();
        if rate <= self.threshold_rad_s {
            return None;
        }
        self.event_counter += 1;
        let severity = UnitFloat::clamped(rate / 1.2);
        Some(RiskEvent {
            id: EventId::new(self.event_counter as u128),
            timestamp: obs.timestamp,
            machine_id: obs.machine_id,
            site_id: self.site_id,
            event_type: RiskEventType::Overswing,
            severity,
            confidence: Confidence::new_unchecked(0.9),
            details: EventDetails::Overswing {
                swing_rate: obs.yaw_rate,
            },
            schema_version: FeatureVersion::new(1, 0, 0),
        })
    }

    fn reset(&mut self) { self.event_counter = 0; }
    fn name(&self) -> &'static str { "overswing" }
}
