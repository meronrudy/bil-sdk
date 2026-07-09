use serde::{Deserialize, Serialize};
use crate::peril::Peril;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeductibleStructure {
    pub per_occurrence: f64,
    pub aggregate_annual: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LimitStructure {
    pub per_occurrence: f64,
    pub aggregate_annual: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CoverageTrigger {
    Occurrence,
    ClaimsMade,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoverageCondition {
    pub name: String,
    pub description: String,
    pub required: bool,
}

/// Complete coverage form — the insurance product definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoverageForm {
    pub name: String,
    pub trigger: CoverageTrigger,
    pub perils_covered: Vec<Peril>,
    pub exclusions: Vec<String>,
    pub deductible: DeductibleStructure,
    pub limits: LimitStructure,
    pub conditions: Vec<CoverageCondition>,
    pub territory: String,
    pub policy_term_months: u16,
}

impl Default for CoverageForm {
    fn default() -> Self {
        Self {
            name: "Construction Equipment Liability".to_string(),
            trigger: CoverageTrigger::Occurrence,
            perils_covered: vec![
                Peril::BodilyInjury,
                Peril::PropertyDamage,
                Peril::EquipmentDamage,
            ],
            exclusions: vec![
                "intentional_acts".to_string(),
                "war_and_terrorism".to_string(),
                "nuclear_hazard".to_string(),
            ],
            deductible: DeductibleStructure {
                per_occurrence: 25_000.0,
                aggregate_annual: 100_000.0,
            },
            limits: LimitStructure {
                per_occurrence: 1_000_000.0,
                aggregate_annual: 5_000_000.0,
            },
            conditions: vec![
                CoverageCondition {
                    name: "safety_system_operational".to_string(),
                    description: "Safety layer must be active during coverage period".to_string(),
                    required: true,
                },
            ],
            territory: "US_multi_state".to_string(),
            policy_term_months: 12,
        }
    }
}
