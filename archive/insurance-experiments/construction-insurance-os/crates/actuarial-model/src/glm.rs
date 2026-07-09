//! High-level GLM model types.

use serde::{Deserialize, Serialize};
use crate::distribution::{Family, LinkFunction};
use crate::irls::IrlsResult;

/// A fitted GLM model with metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FittedGlm {
    pub name: String,
    pub family: Family,
    pub link: LinkFunction,
    pub feature_names: Vec<String>,
    pub coefficients: Vec<f64>,
    pub standard_errors: Vec<f64>,
    pub p_values: Vec<f64>,
    pub aic: f64,
    pub deviance: f64,
    pub n_observations: usize,
    pub converged: bool,
}

impl FittedGlm {
    /// Create from IRLS result and metadata.
    pub fn from_irls(
        name: &str,
        family: Family,
        link: LinkFunction,
        feature_names: Vec<String>,
        result: &IrlsResult,
        n_obs: usize,
    ) -> Self {
        Self {
            name: name.to_string(),
            family,
            link,
            feature_names,
            coefficients: result.coefficients.iter().copied().collect(),
            standard_errors: result.standard_errors.iter().copied().collect(),
            p_values: result.p_values.iter().copied().collect(),
            aic: result.aic,
            deviance: result.deviance,
            n_observations: n_obs,
            converged: result.converged,
        }
    }

    /// Predict the linear predictor η = Xβ for one observation.
    pub fn predict_eta(&self, features: &[f64]) -> f64 {
        assert_eq!(features.len(), self.coefficients.len(),
                   "Feature count mismatch");
        features.iter().zip(&self.coefficients).map(|(x, b)| x * b).sum()
    }

    /// Predict the mean μ = g⁻¹(η) for one observation.
    pub fn predict_mu(&self, features: &[f64]) -> f64 {
        self.link.inverse(self.predict_eta(features))
    }

    /// Number of significant coefficients at given alpha level.
    pub fn n_significant(&self, alpha: f64) -> usize {
        self.p_values.iter().filter(|&&p| p < alpha).count()
    }
}
