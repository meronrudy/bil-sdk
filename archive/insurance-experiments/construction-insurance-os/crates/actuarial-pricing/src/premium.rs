//! Final premium calculation.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PremiumResult {
    pub base_gross_premium: f64,
    pub experience_modifier: f64,
    pub indicated_premium: f64,
    pub minimum_premium: f64,
    pub final_premium: f64,
    pub premium_per_1000_hours: f64,
}

pub fn calculate_final_premium(
    gross_premium: f64,
    experience_modifier: f64,
    exposure_hours: f64,
    minimum_premium: f64,
) -> PremiumResult {
    let indicated = gross_premium * experience_modifier;
    let final_p = indicated.max(minimum_premium);
    PremiumResult {
        base_gross_premium: gross_premium,
        experience_modifier,
        indicated_premium: indicated,
        minimum_premium,
        final_premium: final_p,
        premium_per_1000_hours: final_p / (exposure_hours / 1000.0).max(0.001),
    }
}
