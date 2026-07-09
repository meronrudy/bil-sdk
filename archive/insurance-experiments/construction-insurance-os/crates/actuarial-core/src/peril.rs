use serde::{Deserialize, Serialize};

/// Insured peril — what type of loss event.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Peril {
    BodilyInjury,
    PropertyDamage,
    EquipmentDamage,
    EnvironmentalCleanup,
    ThirdPartyLiability,
    BusinessInterruption,
    ProfessionalLiability,
}
