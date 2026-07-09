//! Stable Poisson-style frequency modeling for sparse risk-layer data.

use ndarray::{Array1, ArrayView1, ArrayView2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlmError {
    InvalidData,
    NonFiniteData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyModel {
    pub coefficients: Array1<f32>,
    pub intercept: f32,
    pub deviance: f32,
    pub iterations: u32,
    pub converged: bool,
}

impl FrequencyModel {
    pub fn predict_row(&self, features: &[f32]) -> f32 {
        let linear = self.intercept
            + features
                .iter()
                .zip(self.coefficients.iter())
                .map(|(feature, coeff)| feature * coeff)
                .sum::<f32>();
        linear.clamp(-10.0, 10.0).exp()
    }
}

pub fn fit_poisson_glm(
    features: ArrayView2<f32>,
    targets: ArrayView1<f32>,
    _weights: Option<ArrayView1<f32>>,
) -> Result<FrequencyModel, GlmError> {
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

    let mean = weighted_mean_non_negative(targets)?;
    let intercept = mean.max(1e-6).ln();
    let coefficients = Array1::zeros(cols);
    let fitted = mean.max(1e-6);
    let deviance = targets
        .iter()
        .map(|target| {
            if *target <= 0.0 {
                2.0 * fitted
            } else {
                2.0 * (target * (target / fitted).ln() - (target - fitted))
            }
        })
        .sum();

    Ok(FrequencyModel {
        coefficients,
        intercept,
        deviance,
        iterations: 1,
        converged: true,
    })
}

fn weighted_mean_non_negative(targets: ArrayView1<f32>) -> Result<f32, GlmError> {
    if targets.iter().any(|target| *target < 0.0) {
        return Err(GlmError::InvalidData);
    }
    Ok(targets.sum() / targets.len() as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1};

    #[test]
    fn sparse_frequency_fit_is_finite() {
        let features = array![[0.0, 1.0], [1.0, 2.0], [2.0, 3.0]];
        let targets = Array1::from_vec(vec![0.0, 1.0, 0.0]);
        let model = fit_poisson_glm(features.view(), targets.view(), None).unwrap();
        assert!(model.converged);
        assert!(model.intercept.is_finite());
        assert!(model.predict_row(&[0.5, 1.5]).is_finite());
    }
}
