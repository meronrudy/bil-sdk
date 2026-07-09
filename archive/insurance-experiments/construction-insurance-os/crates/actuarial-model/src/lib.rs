//! Actuarial loss modeling.
//!
//! Implements Generalized Linear Models (GLMs) for frequency and severity:
//! - **Frequency**: Poisson GLM with log link — E[N] = exp(Xβ + offset)
//! - **Severity**: Gamma GLM with log link — E[S|N>0] = exp(Xβ)
//! - **Loss cost**: E[Loss] = E[N] × E[S]
//!
//! All fitting uses IRLS (Iteratively Reweighted Least Squares),
//! the standard algorithm for GLMs. This is the same math that
//! R's `glm()` and Python's `statsmodels` use internally.

pub mod glm;
pub mod irls;
pub mod distribution;
pub mod diagnostics;

pub use glm::*;
pub use irls::*;
pub use distribution::*;
pub use diagnostics::*;
