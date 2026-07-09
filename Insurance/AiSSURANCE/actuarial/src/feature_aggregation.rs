//! Rolling feature aggregation for underwriting-ready risk features.

use contracts::{
    BehaviorFeatures, CanonicalObservation, ContextFeatures, ExposureFeatures, FeatureVersion,
    MachineId, MonotonicMicros, RiskFeatureBundle, VisibilityConditions, ZoneType,
};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregationError {
    WindowTooLarge,
}

pub type Result<T> = std::result::Result<T, AggregationError>;

pub trait FeatureAccumulator {
    type Feature;

    fn step(&mut self, obs: &CanonicalObservation, timestamp: MonotonicMicros) -> Result<()>;
    fn finalize(&self, window_days: u32) -> Result<Self::Feature>;
}

#[derive(Debug, Clone)]
struct RollingBuffer {
    queue: VecDeque<(MonotonicMicros, f32)>,
    sum: f32,
    max_window_micros: u64,
}

impl RollingBuffer {
    fn new(max_window_days: u32) -> Self {
        Self {
            queue: VecDeque::new(),
            sum: 0.0,
            max_window_micros: max_window_days as u64 * 24 * 60 * 60 * 1_000_000,
        }
    }

    fn push(&mut self, timestamp: MonotonicMicros, value: f32) {
        self.queue.push_back((timestamp, value));
        self.sum += value;
        let cutoff = timestamp.value().saturating_sub(self.max_window_micros);
        while let Some((old_timestamp, old_value)) = self.queue.front().copied() {
            if old_timestamp.value() >= cutoff {
                break;
            }
            self.sum -= old_value;
            self.queue.pop_front();
        }
    }

    fn sum(&self) -> f32 {
        self.sum
    }

    fn average(&self) -> f32 {
        if self.queue.is_empty() {
            0.0
        } else {
            self.sum / self.queue.len() as f32
        }
    }
}

#[derive(Debug)]
pub struct ExposureAccumulator {
    distance_7d: RollingBuffer,
    distance_30d: RollingBuffer,
    distance_90d: RollingBuffer,
    hours_7d: RollingBuffer,
    hours_30d: RollingBuffer,
    hours_90d: RollingBuffer,
    load_7d: RollingBuffer,
    load_30d: RollingBuffer,
    load_90d: RollingBuffer,
}

impl Default for ExposureAccumulator {
    fn default() -> Self {
        Self {
            distance_7d: RollingBuffer::new(7),
            distance_30d: RollingBuffer::new(30),
            distance_90d: RollingBuffer::new(90),
            hours_7d: RollingBuffer::new(7),
            hours_30d: RollingBuffer::new(30),
            hours_90d: RollingBuffer::new(90),
            load_7d: RollingBuffer::new(7),
            load_30d: RollingBuffer::new(30),
            load_90d: RollingBuffer::new(90),
        }
    }
}

impl FeatureAccumulator for ExposureAccumulator {
    type Feature = ExposureFeatures;

    fn step(&mut self, obs: &CanonicalObservation, timestamp: MonotonicMicros) -> Result<()> {
        match obs {
            CanonicalObservation::Motion { distance_delta, .. } => {
                let hours_delta = 60.0 / 3600.0;
                for buffer in [
                    &mut self.distance_7d,
                    &mut self.distance_30d,
                    &mut self.distance_90d,
                ] {
                    buffer.push(timestamp, *distance_delta);
                }
                for buffer in [&mut self.hours_7d, &mut self.hours_30d, &mut self.hours_90d] {
                    buffer.push(timestamp, hours_delta);
                }
            }
            CanonicalObservation::Load {
                load_percentage, ..
            } => {
                for buffer in [&mut self.load_7d, &mut self.load_30d, &mut self.load_90d] {
                    buffer.push(timestamp, *load_percentage);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn finalize(&self, window_days: u32) -> Result<ExposureFeatures> {
        let (distance, hours, load) = match window_days {
            7 => (&self.distance_7d, &self.hours_7d, &self.load_7d),
            30 => (&self.distance_30d, &self.hours_30d, &self.load_30d),
            90 => (&self.distance_90d, &self.hours_90d, &self.load_90d),
            _ => return Err(AggregationError::WindowTooLarge),
        };

        Ok(ExposureFeatures {
            total_distance_meters: distance.sum(),
            total_hours: hours.sum(),
            average_load_factor: load.average(),
        })
    }
}

#[derive(Debug, Default)]
pub struct ExperienceAccumulator {
    harsh_deceleration_count: u32,
    geofence_violation_count: u32,
    worker_proximity_count: u32,
    blind_spot_occupancy_count: u32,
    operator_takeover_count: u32,
    maintenance_deferral_count: u32,
    slope_trench_edge_count: u32,
}

impl FeatureAccumulator for ExperienceAccumulator {
    type Feature = BehaviorFeatures;

    fn step(&mut self, obs: &CanonicalObservation, _timestamp: MonotonicMicros) -> Result<()> {
        match obs {
            CanonicalObservation::Motion { acceleration, .. } if acceleration.value() < -29.4 => {
                self.harsh_deceleration_count += 1;
            }
            CanonicalObservation::Zone {
                zone_type: ZoneType::Geofence,
                entered: true,
                ..
            } => self.geofence_violation_count += 1,
            CanonicalObservation::Proximity { distance, .. } if distance.value() < 5.0 => {
                self.worker_proximity_count += 1;
            }
            CanonicalObservation::Zone {
                zone_type: ZoneType::BlindSpot,
                entered: true,
                ..
            } => self.blind_spot_occupancy_count += 1,
            CanonicalObservation::ControlMode { .. } => self.operator_takeover_count += 1,
            CanonicalObservation::Health {
                maintenance_due: true,
                ..
            } => self.maintenance_deferral_count += 1,
            CanonicalObservation::Zone {
                zone_type: ZoneType::Slope | ZoneType::Trench,
                entered: true,
                ..
            } => self.slope_trench_edge_count += 1,
            _ => {}
        }
        Ok(())
    }

    fn finalize(&self, _window_days: u32) -> Result<BehaviorFeatures> {
        Ok(BehaviorFeatures {
            harsh_deceleration_count: self.harsh_deceleration_count,
            geofence_violation_count: self.geofence_violation_count,
            worker_proximity_count: self.worker_proximity_count,
            reverse_time_fraction: 0.0,
            blind_spot_occupancy_count: self.blind_spot_occupancy_count,
            operator_takeover_count: self.operator_takeover_count,
            maintenance_deferral_count: self.maintenance_deferral_count,
            slope_trench_edge_count: self.slope_trench_edge_count,
        })
    }
}

#[derive(Debug)]
pub struct ContextAccumulator {
    speed_7d: RollingBuffer,
    speed_30d: RollingBuffer,
    speed_90d: RollingBuffer,
    low_visibility_7d: RollingBuffer,
    low_visibility_30d: RollingBuffer,
    low_visibility_90d: RollingBuffer,
    slope_7d: RollingBuffer,
    slope_30d: RollingBuffer,
    slope_90d: RollingBuffer,
}

impl Default for ContextAccumulator {
    fn default() -> Self {
        Self {
            speed_7d: RollingBuffer::new(7),
            speed_30d: RollingBuffer::new(30),
            speed_90d: RollingBuffer::new(90),
            low_visibility_7d: RollingBuffer::new(7),
            low_visibility_30d: RollingBuffer::new(30),
            low_visibility_90d: RollingBuffer::new(90),
            slope_7d: RollingBuffer::new(7),
            slope_30d: RollingBuffer::new(30),
            slope_90d: RollingBuffer::new(90),
        }
    }
}

impl FeatureAccumulator for ContextAccumulator {
    type Feature = ContextFeatures;

    fn step(&mut self, obs: &CanonicalObservation, timestamp: MonotonicMicros) -> Result<()> {
        match obs {
            CanonicalObservation::Motion { speed, .. } => {
                for buffer in [&mut self.speed_7d, &mut self.speed_30d, &mut self.speed_90d] {
                    buffer.push(timestamp, speed.value());
                }
            }
            CanonicalObservation::Visibility {
                visibility,
                conditions,
                ..
            } if visibility.value() < 50.0 || *conditions != VisibilityConditions::Clear => {
                let hours_delta = 60.0 / 3600.0;
                for buffer in [
                    &mut self.low_visibility_7d,
                    &mut self.low_visibility_30d,
                    &mut self.low_visibility_90d,
                ] {
                    buffer.push(timestamp, hours_delta);
                }
            }
            CanonicalObservation::Zone {
                zone_type: ZoneType::Slope,
                entered: true,
                ..
            } => {
                let hours_delta = 60.0 / 3600.0;
                for buffer in [&mut self.slope_7d, &mut self.slope_30d, &mut self.slope_90d] {
                    buffer.push(timestamp, hours_delta);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn finalize(&self, window_days: u32) -> Result<ContextFeatures> {
        let (speed, visibility, slope) = match window_days {
            7 => (&self.speed_7d, &self.low_visibility_7d, &self.slope_7d),
            30 => (&self.speed_30d, &self.low_visibility_30d, &self.slope_30d),
            90 => (&self.speed_90d, &self.low_visibility_90d, &self.slope_90d),
            _ => return Err(AggregationError::WindowTooLarge),
        };

        Ok(ContextFeatures {
            average_speed_mps: speed.average(),
            low_visibility_hours: visibility.sum(),
            slope_exposure_hours: slope.sum(),
        })
    }
}

#[derive(Debug, Default)]
pub struct FeatureAggregator {
    machine_id: Option<MachineId>,
    pub exposure: ExposureAccumulator,
    pub experience: ExperienceAccumulator,
    pub context: ContextAccumulator,
}

impl FeatureAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, obs: &CanonicalObservation, timestamp: MonotonicMicros) -> Result<()> {
        self.machine_id.get_or_insert(obs.machine_id());
        self.exposure.step(obs, timestamp)?;
        self.experience.step(obs, timestamp)?;
        self.context.step(obs, timestamp)?;
        Ok(())
    }

    pub fn finalize_bundle(
        &self,
        window_days: u32,
        version: FeatureVersion,
    ) -> Result<RiskFeatureBundle> {
        Ok(RiskFeatureBundle {
            machine_id: self.machine_id,
            version,
            exposure: self.exposure.finalize(window_days)?,
            behavior: self.experience.finalize(window_days)?,
            context: self.context.finalize(window_days)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use contracts::{MachineId, MetersPerSecond, MetersPerSecondSquared};

    #[test]
    fn aggregates_motion_features() {
        let obs = CanonicalObservation::Motion {
            machine_id: MachineId::test_id(1),
            timestamp: MonotonicMicros::new(1),
            speed: MetersPerSecond(4.0),
            acceleration: MetersPerSecondSquared(-30.0),
            jerk: MetersPerSecondSquared(0.0),
            distance_delta: 10.0,
        };
        let mut aggregator = FeatureAggregator::new();
        aggregator.step(&obs, obs.timestamp()).unwrap();
        let bundle = aggregator
            .finalize_bundle(30, FeatureVersion::default())
            .unwrap();
        assert_eq!(bundle.behavior.harsh_deceleration_count, 1);
        assert_eq!(bundle.exposure.total_distance_meters, 10.0);
    }
}
