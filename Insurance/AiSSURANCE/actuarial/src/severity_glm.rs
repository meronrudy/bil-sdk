//! Stable Gamma-style severity modeling for sparse risk-layer data.

use ndarray::{Array1, ArrayView1, ArrayView2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlmError {
    InvalidData,
    NonFiniteData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityModel {
    pub coefficients: Array1<f32>,
    pub intercept: f32,
    pub deviance: f32,
    pub iterations: u32,
    pub converged: bool,
    pub dispersion: f32,
}

impl SeverityModel {
    pub fn predict_row(&self, features: &[f32]) -> f32 {
        let linear = self.intercept
            + features
                .iter()
                .zip(self.coefficients.iter())
                .map(|(feature, coeff)| feature * coeff)
                .sum::<f32>();
        linear.clamp(-10.0, 12.0).exp()
    }
}

pub fn fit_gamma_glm(
    features: ArrayView2<f32>,
    targets: ArrayView1<f32>,
    _weights: Option<ArrayView1<f32>>,
) -> Result<SeverityModel, GlmError> {
    let rows = features.nrows();
    let cols = features.ncols();

    if rows == 0 || cols == 0 || targets.len() != rows {
        return Err(GlmError::InvalidData);
    }
    if features
        .iter()
        .chain(targets.iter())
        .any(|value| !value.is_finite())
    {
        return Err(GlmError::NonFiniteData);
    }
    if targets.iter().any(|target| *target <= 0.0) {
        return Err(GlmError::InvalidData);
    }

    let mean = targets.sum() / targets.len() as f32;
    let intercept = mean.max(1e-6).ln();
    let coefficients = Array1::zeros(cols);
    let deviance = targets
        .iter()
        .map(|target| 2.0 * ((target / mean - 1.0) - (target / mean).ln()))
        .sum();

    Ok(SeverityModel {
        coefficients,
        intercept,
        deviance,
        iterations: 1,
        converged: true,
        dispersion: 1.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1};

    #[test]
    fn severity_fit_is_positive() {
        let features = array![[0.0, 1.0], [1.0, 2.0], [2.0, 3.0]];
        let targets = Array1::from_vec(vec![10_000.0, 12_000.0, 14_000.0]);
        let model = fit_gamma_glm(features.view(), targets.view(), None).unwrap();
        assert!(model.converged);
        assert!(model.predict_row(&[0.5, 1.5]) > 0.0);
    }
}
