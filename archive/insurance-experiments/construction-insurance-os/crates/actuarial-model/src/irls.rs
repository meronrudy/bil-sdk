//! IRLS (Iteratively Reweighted Least Squares) solver for GLMs.

use nalgebra::{DMatrix, DVector};
use crate::distribution::{Family, LinkFunction};
use actuarial_core::ActuarialError;

/// Configuration for the IRLS solver.
#[derive(Clone, Debug)]
pub struct IrlsConfig {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub ridge_penalty: f64,
}

impl Default for IrlsConfig {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            tolerance: 1e-8,
            ridge_penalty: 1e-6,
        }
    }
}

/// Result of an IRLS fit.
#[derive(Clone, Debug)]
pub struct IrlsResult {
    pub coefficients: DVector<f64>,
    pub standard_errors: DVector<f64>,
    pub p_values: DVector<f64>,
    pub deviance: f64,
    pub aic: f64,
    pub iterations: usize,
    pub converged: bool,
}

/// Fit a GLM using IRLS.
///
/// # Arguments
/// * `x` - Design matrix (n × p), should include intercept column.
/// * `y` - Response vector (n × 1).
/// * `family` - Error distribution family.
/// * `link` - Link function.
/// * `offset` - Optional offset vector (e.g., log-exposure for Poisson).
/// * `config` - Solver configuration.
pub fn fit_glm(
    x: &DMatrix<f64>,
    y: &DVector<f64>,
    family: Family,
    link: LinkFunction,
    offset: Option<&DVector<f64>>,
    config: &IrlsConfig,
) -> Result<IrlsResult, ActuarialError> {
    let (n, p) = (x.nrows(), x.ncols());
    let zero_offset = DVector::zeros(n);
    let off = offset.unwrap_or(&zero_offset);

    // Initialize from OLS on transformed response
    let z_init: DVector<f64> = DVector::from_iterator(n,
        y.iter().map(|&yi| link.link((yi + 0.1).max(0.01)) - 0.0));
    let xtx = x.transpose() * x.clone();
    let xtz = x.transpose() * &z_init;
    let mut beta = xtx.clone()
        .try_inverse()
        .map(|inv| inv * &xtz)
        .unwrap_or_else(|| DVector::zeros(p));

    let mut iterations = 0;
    let mut converged = false;

    for iter in 0..config.max_iterations {
        iterations = iter + 1;

        // Compute linear predictor and mean
        let eta = x * &beta + off;
        let mu: DVector<f64> = DVector::from_iterator(n,
            eta.iter().map(|&e| link.inverse(e).clamp(1e-10, 1e8)));

        // Working response and weights
        let z: DVector<f64> = DVector::from_iterator(n,
            (0..n).map(|i| {
                let d = link.derivative(mu[i]);
                eta[i] - off[i] + (y[i] - mu[i]) * d
            }));

        let w: DVector<f64> = DVector::from_iterator(n,
            (0..n).map(|i| {
                let d = link.derivative(mu[i]);
                let v = family.variance(mu[i]).max(1e-10);
                1.0 / (d * d * v)
            }));

        // Weighted least squares: (X'WX + λI)^{-1} X'Wz
        let w_diag = DMatrix::from_diagonal(&w);
        let xtwx = x.transpose() * &w_diag * x
            + DMatrix::identity(p, p) * config.ridge_penalty;
        let xtwz = x.transpose() * &w_diag * &z;

        let beta_new = xtwx.try_inverse()
            .ok_or_else(|| ActuarialError::ConvergenceFailure { iterations })?
            * &xtwz;

        // Check convergence
        let max_change = (&beta_new - &beta).abs().max();
        // Damped update
        beta = &beta * 0.5 + &beta_new * 0.5;

        if max_change < config.tolerance {
            beta = beta_new;
            converged = true;
            break;
        }
    }

    // Final predictions and diagnostics
    let eta = x * &beta + off;
    let mu: DVector<f64> = DVector::from_iterator(n,
        eta.iter().map(|&e| link.inverse(e).clamp(1e-10, 1e8)));

    let deviance = compute_deviance(y, &mu, &family);
    let aic = deviance + 2.0 * p as f64;

    // Standard errors from Fisher information
    let w_final: DVector<f64> = DVector::from_iterator(n,
        (0..n).map(|i| {
            let d = link.derivative(mu[i]);
            let v = family.variance(mu[i]).max(1e-10);
            1.0 / (d * d * v)
        }));
    let w_diag = DMatrix::from_diagonal(&w_final);
    let fisher = x.transpose() * &w_diag * x;
    let se = fisher.try_inverse()
        .map(|inv| DVector::from_iterator(p, inv.diagonal().iter().map(|&v| v.abs().sqrt())))
        .unwrap_or_else(|| DVector::from_element(p, f64::NAN));

    // Wald p-values
    let z_scores: DVector<f64> = DVector::from_iterator(p,
        (0..p).map(|i| {
            if se[i] > 0.0 { (beta[i] / se[i]).abs() } else { 0.0 }
        }));
    let p_values: DVector<f64> = DVector::from_iterator(p,
        z_scores.iter().map(|&z| 2.0 * normal_cdf_complement(z)));

    Ok(IrlsResult {
        coefficients: beta,
        standard_errors: se,
        p_values,
        deviance,
        aic,
        iterations,
        converged,
    })
}

fn compute_deviance(y: &DVector<f64>, mu: &DVector<f64>, family: &Family) -> f64 {
    let n = y.len();
    match family {
        Family::Poisson => {
            2.0 * (0..n).map(|i| {
                if y[i] > 0.0 { y[i] * (y[i] / mu[i]).ln() } else { 0.0 } - (y[i] - mu[i])
            }).sum::<f64>()
        }
        Family::Gamma => {
            2.0 * (0..n).map(|i| {
                -(y[i] / mu[i]).ln() + (y[i] - mu[i]) / mu[i]
            }).sum::<f64>()
        }
        Family::Gaussian => {
            (0..n).map(|i| (y[i] - mu[i]).powi(2)).sum::<f64>()
        }
        _ => 0.0,
    }
}

/// Complement of the standard normal CDF (upper tail).
fn normal_cdf_complement(z: f64) -> f64 {
    // Abramowitz and Stegun approximation 7.1.26
    let t = 1.0 / (1.0 + 0.2316419 * z.abs());
    let d = 0.3989422804014327; // 1/sqrt(2π)
    let p = d * (-z * z / 2.0).exp()
        * (t * (0.319381530
            + t * (-0.356563782
                + t * (1.781477937
                    + t * (-1.821255978
                        + t * 1.330274429)))));
    if z >= 0.0 { p } else { 1.0 - p }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_poisson_glm() {
        // y ~ Poisson, log(mu) = beta0 + beta1 * x
        let n = 100;
        let mut x_data = Vec::with_capacity(n * 2);
        let mut y_data = Vec::with_capacity(n);
        let mut rng_state: u64 = 42;
        for i in 0..n {
            let xi = i as f64 / n as f64;
            x_data.push(1.0); // intercept
            x_data.push(xi);
            let mu = (0.5 + 1.5 * xi).exp();
            // Simple deterministic "Poisson-like" response
            y_data.push(mu.round());
        }
        let x = DMatrix::from_row_slice(n, 2, &x_data);
        let y = DVector::from_vec(y_data);
        let config = IrlsConfig::default();
        let result = fit_glm(&x, &y, Family::Poisson, LinkFunction::Log, None, &config);
        assert!(result.is_ok());
        let r = result.unwrap();
        assert!(r.converged);
        // Intercept should be near 0.5, slope near 1.5
        assert!((r.coefficients[0] - 0.5).abs() < 0.5);
        assert!((r.coefficients[1] - 1.5).abs() < 0.5);
    }
}
