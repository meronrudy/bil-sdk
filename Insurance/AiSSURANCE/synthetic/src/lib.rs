//! # Synthetic Data Generator
//!
//! Deterministic generation of test data for telemetry and claims.

pub mod claims_generator;
pub mod fleet_config;
pub mod telemetry_generator;

// Re-export main types
pub use claims_generator::*;
pub use fleet_config::*;
pub use telemetry_generator::*;
