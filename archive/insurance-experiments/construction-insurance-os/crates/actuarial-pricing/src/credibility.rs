//! Credibility and experience rating.
//! Bühlmann credibility blended with telematics behavioral score.

use serde::{Deserialize, Serialize};
use ctw_core::UnitFloat;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CredibilityParams {
    pub full_credibility_exposure: f64,
    pub behavioral_weight: f64,
    pub modifier_floor: f64,
    pub modifier_ceiling: f64,
}

impl Default for CredibilityParams {
    fn default() -> Self {
        Self {
            full_credibility_exposure: 50_000.0,
            behavioral_weight: 0.3,
            modifier_floor: 0.70,
            modifier_ceiling: 1.50,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExperienceModifier {
    pub credibility_z: f64,
    pub actual_losses: f64,
    pub expected_losses: f64,
    pub raw_loss_ratio: f64,
    pub dampened_loss_modifier: f64,
    pub behavioral_score: f64,
    pub blended_raw: f64,
    pub final_modifier: f64,
}

pub fn compute_experience_modifier(
    exposure_hours: f64,
    actual_losses: f64,
    expected_loss_per_1000h: f64,
    behavioral_score: f64,
    params: &CredibilityParams,
) -> ExperienceModifier {
    let z = (exposure_hours / params.full_credibility_exposure).sqrt().min(1.0);
    let expected = expected_loss_per_1000h * (exposure_hours / 1000.0);
    let lr = if expected > 0.0 { actual_losses / expected } else { 1.0 };
    let dampened = 1.0 + z * (lr - 1.0);

    let w = params.behavioral_weight;
    let blended = (1.0 - w) * dampened + w * behavioral_score;
    let capped = blended.clamp(params.modifier_floor, params.modifier_ceiling);

    ExperienceModifier {
        credibility_z: z,
        actual_losses,
        expected_losses: expected,
        raw_loss_ratio: lr,
        dampened_loss_modifier: dampened,
        behavioral_score,
        blended_raw: blended,
        final_modifier: capped,
    }
}
