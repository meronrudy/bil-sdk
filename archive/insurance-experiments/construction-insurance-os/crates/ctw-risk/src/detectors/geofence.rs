use ctw_core::*;
use ctw_ingest::observation::ZoneObservation;
use super::Detector;
use crate::events::*;

/// Detects geofence incursion events.
pub struct GeofenceDetector {
    event_counter: u64,
    site_id: SiteId,
}

impl GeofenceDetector {
    pub fn new(site_id: SiteId) -> Self {
        Self { event_counter: 0, site_id }
    }
}

impl Detector for GeofenceDetector {
    type Input = ZoneObservation;

    fn step(&mut self, obs: &ZoneObservation) -> Option<RiskEvent> {
        // Negative margin = inside the boundary = incursion
        if obs.margin.raw() >= 0.0 {
            return None;
        }
        self.event_counter += 1;
        let severity = UnitFloat::clamped(obs.margin.raw().abs() / 5.0);
        Some(RiskEvent {
            id: EventId::new(self.event_counter as u128),
            timestamp: obs.timestamp,
            machine_id: obs.machine_id,
            site_id: self.site_id,
            event_type: RiskEventType::GeofenceIncursion,
            severity,
            confidence: Confidence::new_unchecked(0.99),
            details: EventDetails::GeofenceIncursion {
                zone_id: obs.zone_id,
                depth: Meters::new(obs.margin.raw().abs()),
                duration_so_far: Seconds::new(0.0),
            },
            schema_version: FeatureVersion::new(1, 0, 0),
        })
    }

    fn reset(&mut self) { self.event_counter = 0; }
    fn name(&self) -> &'static str { "geofence" }
}
