//! Model diagnostics: AIC, deviance, residuals.

use crate::glm::FittedGlm;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDiagnostics {
    pub aic: f64,
    pub deviance: f64,
    pub n_observations: usize,
    pub n_parameters: usize,
    pub converged: bool,
    pub significant_at_05: usize,
    pub significant_at_01: usize,
}

impl From<&FittedGlm> for ModelDiagnostics {
    fn from(m: &FittedGlm) -> Self {
        Self {
            aic: m.aic,
            deviance: m.deviance,
            n_observations: m.n_observations,
            n_parameters: m.coefficients.len(),
            converged: m.converged,
            significant_at_05: m.n_significant(0.05),
            significant_at_01: m.n_significant(0.01),
        }
    }
}
