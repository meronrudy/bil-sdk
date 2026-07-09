//! Deterministic claim generation for repeatable risk-layer tests.

use crate::fleet_config::FleetConfig;
use contracts::{ClaimRecord, EventId, SiteTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimType {
    EquipmentDamage,
    WorkerInjury,
    PropertyDamage,
    Theft,
    Environmental,
    ThirdPartyLiability,
}

impl ClaimType {
    fn as_str(self) -> &'static str {
        match self {
            Self::EquipmentDamage => "equipment_damage",
            Self::WorkerInjury => "worker_injury",
            Self::PropertyDamage => "property_damage",
            Self::Theft => "theft",
            Self::Environmental => "environmental",
            Self::ThirdPartyLiability => "third_party_liability",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ClaimsParameters {
    pub base_frequency: f64,
    pub severity_shape: f64,
    pub severity_scale: f64,
    pub period_days: u32,
    pub seed: u64,
}

impl Default for ClaimsParameters {
    fn default() -> Self {
        Self {
            base_frequency: 0.10,
            severity_shape: 2.0,
            severity_scale: 10_000.0,
            period_days: 90,
            seed: 42,
        }
    }
}

pub struct ClaimsGenerator {
    fleet: FleetConfig,
    params: ClaimsParameters,
}

impl ClaimsGenerator {
    pub fn new(fleet: FleetConfig, params: ClaimsParameters) -> Self {
        Self { fleet, params }
    }

    pub fn generate_claims(&mut self) -> Vec<ClaimRecord> {
        let mut claims = Vec::new();
        let site_id = self
            .fleet
            .sites
            .first()
            .map(|site| site.id)
            .unwrap_or_default();

        for (idx, machine) in self.fleet.machines.iter().enumerate() {
            let exposure_years = self.params.period_days as f64 / 365.25;
            let expected = self.params.base_frequency * exposure_years;
            let count = if expected <= 0.0 {
                0
            } else {
                (expected.ceil() as usize).max(1)
            };

            for claim_idx in 0..count {
                let amount = self.params.severity_shape
                    * self.params.severity_scale
                    * (1.0 + idx as f64 * 0.08 + claim_idx as f64 * 0.05);
                let timestamp = SiteTime::from_timestamp_micros(
                    (self.params.seed as i64 + idx as i64 + claim_idx as i64) * 1_000_000,
                )
                .unwrap_or_default();
                let claim_type = match (idx + claim_idx) % 3 {
                    0 => ClaimType::EquipmentDamage,
                    1 => ClaimType::WorkerInjury,
                    _ => ClaimType::PropertyDamage,
                };

                claims.push(ClaimRecord {
                    claim_id: EventId::test_id((idx * 100 + claim_idx) as u32),
                    machine_id: machine.id,
                    site_id,
                    timestamp,
                    amount,
                    claim_type: claim_type.as_str().to_string(),
                    description: format!("Synthetic {:?} claim", claim_type),
                });
            }
        }

        claims.sort_by_key(|claim| claim.timestamp);
        claims
    }
}

pub struct ClaimsIterator {
    claims: Vec<ClaimRecord>,
    index: usize,
}

impl ClaimsIterator {
    pub fn new(fleet: FleetConfig, params: ClaimsParameters) -> Self {
        let mut generator = ClaimsGenerator::new(fleet, params);
        Self {
            claims: generator.generate_claims(),
            index: 0,
        }
    }
}

impl Iterator for ClaimsIterator {
    type Item = ClaimRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let claim = self.claims.get(self.index).cloned();
        self.index += usize::from(claim.is_some());
        claim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_claims_are_positive() {
        let mut generator =
            ClaimsGenerator::new(FleetConfig::default(), ClaimsParameters::default());
        let claims = generator.generate_claims();
        assert!(!claims.is_empty());
        assert!(claims.iter().all(|claim| claim.amount > 0.0));
    }
}
