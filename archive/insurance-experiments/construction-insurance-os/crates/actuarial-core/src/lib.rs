//! Actuarial domain types.
//!
//! These types represent the insurance business objects:
//! policies, claims, coverage forms, loss amounts.

pub mod coverage;
pub mod policy;
pub mod claim;
pub mod loss;
pub mod peril;
pub mod error;

pub use coverage::*;
pub use policy::*;
pub use claim::*;
pub use loss::*;
pub use peril::*;
pub use error::ActuarialError;
