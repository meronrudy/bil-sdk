use ctw_core::*;
use ctw_ingest::observation::ProximityObservation;
use super::Detector;
use crate::events::*;

/// Detects worker proximity breach events.
pub struct WorkerProximityDetector {
    threshold_m: f64,
    critical_threshold_m: f64,
    event_counter: u64,
    site_id: SiteId,
}

impl WorkerProximityDetector {
    pub fn new(threshold_m: f64, critical_m: f64, site_id: SiteId) -> Self {
        Self { threshold_m, critical_threshold_m: critical_m, event_counter: 0, site_id }
    }
}

impl Detector for WorkerProximityDetector {
    type Input = ProximityObservation;

    fn step(&mut self, obs: &ProximityObservation) -> Option<RiskEvent> {
        if obs.target_type != ctw_ingest::observation::ProximityTarget::Worker {
            return None;
        }
        let dist = obs.distance.raw();
        if dist >= self.threshold_m {
            return None;
        }
        self.event_counter += 1;
        let is_critical = dist < self.critical_threshold_m;
        let severity = UnitFloat::clamped(1.0 - dist / self.threshold_m);
        Some(RiskEvent {
            id: EventId::new(self.event_counter as u128),
            timestamp: obs.timestamp,
            machine_id: obs.machine_id,
            site_id: self.site_id,
            event_type: if is_critical {
                RiskEventType::WorkerProximityCritical
            } else {
                RiskEventType::WorkerProximity
            },
            severity,
            confidence: obs.confidence,
            details: EventDetails::WorkerProximity {
                distance: obs.distance,
                relative_speed: MetersPerSecond::new(0.0),
                worker_count: 1,
            },
            schema_version: FeatureVersion::new(1, 0, 0),
        })
    }

    fn reset(&mut self) { self.event_counter = 0; }
    fn name(&self) -> &'static str { "worker_proximity" }
}
