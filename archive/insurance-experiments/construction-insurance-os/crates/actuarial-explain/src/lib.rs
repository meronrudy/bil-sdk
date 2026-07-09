//! Explainability: attribute premium to risk drivers and compute counterfactuals.

use serde::{Deserialize, Serialize};
use actuarial_model::FittedGlm;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeatureContribution {
    pub feature: String,
    pub coefficient: f64,
    pub value: f64,
    pub log_contribution: f64,
    pub multiplicative_effect: f64,
    pub direction: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Counterfactual {
    pub intervention: String,
    pub estimated_savings: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PremiumExplanation {
    pub contributions: Vec<FeatureContribution>,
    pub counterfactuals: Vec<Counterfactual>,
}

/// Decompose a GLM prediction into per-feature contributions.
pub fn explain_prediction(model: &FittedGlm, feature_values: &[f64]) -> Vec<FeatureContribution> {
    assert_eq!(feature_values.len(), model.coefficients.len());
    let mut contributions: Vec<FeatureContribution> = model.feature_names.iter()
        .zip(model.coefficients.iter())
        .zip(feature_values.iter())
        .map(|((name, &coef), &val)| {
            let log_c = coef * val;
            FeatureContribution {
                feature: name.clone(),
                coefficient: coef,
                value: val,
                log_contribution: log_c,
                multiplicative_effect: log_c.exp(),
                direction: if coef > 0.0 { "increases risk".to_string() }
                           else { "decreases risk".to_string() },
            }
        })
        .collect();
    contributions.sort_by(|a, b| b.log_contribution.abs().partial_cmp(&a.log_contribution.abs()).unwrap());
    contributions
}
