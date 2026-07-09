//! Deterministic telemetry generation for risk-layer tests.

use crate::fleet_config::FleetConfig;
use contracts::{MachineId, MonotonicMicros, TelemetryFrame, WorkerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TelemetryParameters {
    pub period_days: u32,
    pub seed: u64,
}

impl Default for TelemetryParameters {
    fn default() -> Self {
        Self {
            period_days: 90,
            seed: 42,
        }
    }
}

pub struct TelemetryGenerator {
    config: FleetConfig,
    params: TelemetryParameters,
    frame_index: usize,
    max_frames: usize,
}

impl TelemetryGenerator {
    pub fn new(config: FleetConfig, params: TelemetryParameters) -> Self {
        let machine_count = config.machines.len().max(1);
        let days = params.period_days.max(1) as usize;
        Self {
            config,
            params,
            frame_index: 0,
            max_frames: days * machine_count * 12,
        }
    }

    pub fn frames(&mut self) -> TelemetryFrameIterator<'_> {
        TelemetryFrameIterator { generator: self }
    }

    fn make_frame(
        &self,
        machine_id: MachineId,
        timestamp: MonotonicMicros,
        index: usize,
    ) -> TelemetryFrame {
        let seed_adjust = (self.params.seed % 7) as f32 * 0.05;
        let cycle = (index % 12) as u32;
        let raw_data = match cycle {
            0 => serde_json::json!({
                "type": "motion",
                "speed": 4.5 + seed_adjust,
                "acceleration": -31.0,
                "jerk": 12.0,
                "distance_delta": 270.0
            }),
            1 => serde_json::json!({
                "type": "proximity",
                "worker_id": WorkerId::test_id(1).as_bytes(),
                "distance": 3.0
            }),
            2 => serde_json::json!({
                "type": "zone",
                "zone_type": "geofence",
                "entered": true
            }),
            3 => serde_json::json!({
                "type": "load",
                "load_percentage": 0.82
            }),
            4 => serde_json::json!({
                "type": "visibility",
                "visibility": 42.0,
                "conditions": "dust"
            }),
            5 => serde_json::json!({
                "type": "health",
                "engine_temp": 102.0,
                "fuel_level": 0.42,
                "maintenance_due": true
            }),
            6 => serde_json::json!({
                "type": "control_mode",
                "mode": "manual"
            }),
            _ => serde_json::json!({
                "type": "motion",
                "speed": 3.0 + (cycle as f32 * 0.1) + seed_adjust,
                "acceleration": -1.0,
                "jerk": 2.0,
                "distance_delta": 180.0
            }),
        };

        TelemetryFrame::new(
            machine_id,
            timestamp,
            serde_json::to_vec(&raw_data).unwrap(),
        )
    }
}

impl Iterator for TelemetryGenerator {
    type Item = TelemetryFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frame_index >= self.max_frames || self.config.machines.is_empty() {
            return None;
        }

        let machine_index = self.frame_index % self.config.machines.len();
        let machine_id = self.config.machines[machine_index].id;
        let timestamp = MonotonicMicros::new(self.frame_index as u64 * 60_000_000);
        let frame = self.make_frame(machine_id, timestamp, self.frame_index);
        self.frame_index += 1;
        Some(frame)
    }
}

pub struct TelemetryFrameIterator<'a> {
    generator: &'a mut TelemetryGenerator,
}

impl<'a> Iterator for TelemetryFrameIterator<'a> {
    type Item = TelemetryFrame;

    fn next(&mut self) -> Option<Self::Item> {
        self.generator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_contract_frames() {
        let mut generator =
            TelemetryGenerator::new(FleetConfig::default(), TelemetryParameters::default());
        let frame = generator.next().expect("expected a frame");
        assert!(!frame.raw_data.is_empty());
    }
}
