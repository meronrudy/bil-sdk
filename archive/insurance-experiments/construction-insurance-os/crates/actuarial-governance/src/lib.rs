//! Governance: rate filing artifacts, audit trails, tamper detection.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use ctw_core::FilingId;
use actuarial_model::FittedGlm;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RatingFactor {
    pub name: String,
    pub coefficient: f64,
    pub p_value: f64,
    pub significant: bool,
    pub relativity: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateFilingArtifact {
    pub filing_id: String,
    pub effective_date: String,
    pub line_of_business: String,
    pub territory: String,
    pub base_rate_per_1000h: f64,
    pub rating_factors: Vec<RatingFactor>,
    pub model_diagnostics: serde_json::Value,
    pub content_hash: String,
}

/// Generate a rate filing artifact with cryptographic hash.
pub fn generate_filing(
    effective_date: &str,
    territory: &str,
    base_rate: f64,
    frequency_model: &FittedGlm,
) -> RateFilingArtifact {
    let factors: Vec<RatingFactor> = frequency_model.feature_names.iter()
        .zip(frequency_model.coefficients.iter())
        .zip(frequency_model.p_values.iter())
        .skip(1) // skip intercept
        .map(|((name, &coef), &pv)| RatingFactor {
            name: name.clone(),
            coefficient: coef,
            p_value: pv,
            significant: pv < 0.05,
            relativity: coef.exp(),
        })
        .collect();

    let content = serde_json::json!({
        "base_rate": base_rate,
        "n_factors": factors.len(),
        "effective": effective_date,
    });
    let hash = {
        let mut hasher = Sha256::new();
        hasher.update(content.to_string().as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    };

    RateFilingArtifact {
        filing_id: format!("FIL-{hash}"),
        effective_date: effective_date.to_string(),
        line_of_business: "Commercial Construction Equipment".to_string(),
        territory: territory.to_string(),
        base_rate_per_1000h: base_rate,
        rating_factors: factors,
        model_diagnostics: serde_json::json!({
            "freq_aic": frequency_model.aic,
            "freq_n": frequency_model.n_observations,
            "converged": frequency_model.converged,
        }),
        content_hash: hash,
    }
}
