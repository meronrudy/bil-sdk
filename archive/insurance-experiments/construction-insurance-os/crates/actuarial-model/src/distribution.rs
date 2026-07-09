use serde::{Deserialize, Serialize};

/// GLM family (error distribution).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Family {
    Poisson,
    Gamma,
    Gaussian,
    InverseGaussian,
    Tweedie { power: u32 }, // power parameter × 100 (e.g., 150 = 1.50)
}

/// GLM link function.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum LinkFunction {
    Log,
    Identity,
    Inverse,
    Logit,
}

impl Family {
    /// Variance function V(μ) for this family.
    pub fn variance(&self, mu: f64) -> f64 {
        match self {
            Self::Poisson => mu,
            Self::Gamma => mu * mu,
            Self::Gaussian => 1.0,
            Self::InverseGaussian => mu.powi(3),
            Self::Tweedie { power } => {
                let p = *power as f64 / 100.0;
                mu.powf(p)
            }
        }
    }
}

impl LinkFunction {
    pub fn link(&self, mu: f64) -> f64 {
        match self {
            Self::Log => mu.ln(),
            Self::Identity => mu,
            Self::Inverse => 1.0 / mu,
            Self::Logit => (mu / (1.0 - mu)).ln(),
        }
    }

    pub fn inverse(&self, eta: f64) -> f64 {
        match self {
            Self::Log => eta.exp(),
            Self::Identity => eta,
            Self::Inverse => 1.0 / eta,
            Self::Logit => 1.0 / (1.0 + (-eta).exp()),
        }
    }

    pub fn derivative(&self, mu: f64) -> f64 {
        match self {
            Self::Log => 1.0 / mu,
            Self::Identity => 1.0,
            Self::Inverse => -1.0 / (mu * mu),
            Self::Logit => 1.0 / (mu * (1.0 - mu)),
        }
    }
}
