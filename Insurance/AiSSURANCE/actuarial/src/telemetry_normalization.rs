//! Telemetry normalization from raw JSON frames to canonical observations.

use contracts::{
    CanonicalObservation, ControlModeType, Degrees, MachineId, Meters, MetersPerSecond,
    MetersPerSecondSquared, MonotonicMicros, TelemetryFrame, VisibilityConditions, WorkerId,
    ZoneType,
};
use serde_json::Value;

#[derive(Debug, Default)]
pub struct TelemetryNormalizer;

impl TelemetryNormalizer {
    pub fn new() -> Self {
        Self
    }

    pub fn normalize(&self, frame: &TelemetryFrame) -> Option<CanonicalObservation> {
        let json: Value = serde_json::from_slice(&frame.raw_data).ok()?;
        let obs_type = json.get("type")?.as_str()?;
        Self::parse_json(obs_type, &json, frame.machine_id, frame.timestamp)
    }

    fn parse_json(
        obs_type: &str,
        json: &Value,
        machine_id: MachineId,
        timestamp: MonotonicMicros,
    ) -> Option<CanonicalObservation> {
        match obs_type {
            "motion" => Some(CanonicalObservation::Motion {
                machine_id,
                timestamp,
                speed: MetersPerSecond::new(number(json, "speed")?)?,
                acceleration: MetersPerSecondSquared::new(number(json, "acceleration")?)?,
                jerk: MetersPerSecondSquared::new(number(json, "jerk").unwrap_or(0.0))?,
                distance_delta: number(json, "distance_delta").unwrap_or(0.0).max(0.0),
            }),
            "pose" => Some(CanonicalObservation::Pose {
                machine_id,
                timestamp,
                x: Meters::new(number(json, "x")?)?,
                y: Meters::new(number(json, "y")?)?,
                heading: Degrees::new(number(json, "heading")?)?,
            }),
            "proximity" => Some(CanonicalObservation::Proximity {
                machine_id,
                timestamp,
                worker_id: parse_worker_id(json).unwrap_or_else(|| WorkerId::test_id(1)),
                distance: Meters::new(number(json, "distance")?)?,
            }),
            "zone" => Some(CanonicalObservation::Zone {
                machine_id,
                timestamp,
                zone_type: match json.get("zone_type")?.as_str()? {
                    "geofence" => ZoneType::Geofence,
                    "blind_spot" => ZoneType::BlindSpot,
                    "trench" => ZoneType::Trench,
                    "slope" => ZoneType::Slope,
                    "worker_area" => ZoneType::WorkerArea,
                    _ => return None,
                },
                entered: json.get("entered")?.as_bool()?,
            }),
            "health" => Some(CanonicalObservation::Health {
                machine_id,
                timestamp,
                engine_temp: number(json, "engine_temp")?,
                fuel_level: number(json, "fuel_level")?,
                maintenance_due: json.get("maintenance_due")?.as_bool()?,
            }),
            "control_mode" => Some(CanonicalObservation::ControlMode {
                machine_id,
                timestamp,
                mode: match json.get("mode")?.as_str()? {
                    "manual" => ControlModeType::Manual,
                    "autonomous" | "auto" => ControlModeType::Autonomous,
                    "remote" => ControlModeType::Remote,
                    _ => return None,
                },
            }),
            "visibility" => Some(CanonicalObservation::Visibility {
                machine_id,
                timestamp,
                visibility: Meters::new(number(json, "visibility")?)?,
                conditions: match json.get("conditions")?.as_str()? {
                    "clear" => VisibilityConditions::Clear,
                    "fog" => VisibilityConditions::Fog,
                    "rain" => VisibilityConditions::Rain,
                    "dust" => VisibilityConditions::Dust,
                    "night" => VisibilityConditions::Night,
                    _ => return None,
                },
            }),
            "load" => Some(CanonicalObservation::Load {
                machine_id,
                timestamp,
                load_percentage: number(json, "load_percentage")?.clamp(0.0, 1.5),
            }),
            _ => None,
        }
    }
}

fn number(json: &Value, key: &str) -> Option<f32> {
    json.get(key)?.as_f64().map(|value| value as f32)
}

fn parse_worker_id(json: &Value) -> Option<WorkerId> {
    if let Some(id) = json.get("worker_id").and_then(Value::as_u64) {
        return Some(WorkerId::test_id(id as u8));
    }

    let values = json.get("worker_id")?.as_array()?;
    let mut bytes = [0u8; 16];
    for (idx, value) in values.iter().take(16).enumerate() {
        bytes[idx] = value.as_u64()? as u8;
    }
    Some(WorkerId::from_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_motion_json() {
        let frame = TelemetryFrame::new(
            MachineId::test_id(1),
            MonotonicMicros::new(10),
            br#"{"type":"motion","speed":1.0,"acceleration":-2.0,"jerk":3.0}"#.to_vec(),
        );

        let obs = TelemetryNormalizer::new().normalize(&frame);
        assert!(matches!(obs, Some(CanonicalObservation::Motion { .. })));
    }
}
