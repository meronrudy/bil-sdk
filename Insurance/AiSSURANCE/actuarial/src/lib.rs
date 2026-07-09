//! # AiSSURANCE Actuarial Engine
//!
//! Insurance intelligence for autonomous construction fleets. This crate turns
//! raw machine telemetry into transparent pricing, reserving, explainability,
//! and filing outputs that carriers and fleet operators can act on.
//!
//! The engine pairs classical actuarial methods with real-time telemetry
//! signals, giving AiSSURANCE a risk layer that is fast enough for operations
//! and rigorous enough for underwriting review.
//!
//! ## Commercial Capabilities
//! - `telemetry_normalization`: Convert raw fleet data into canonical signals
//! - `risk_detectors`: Surface jobsite behaviors that move loss outcomes
//! - `feature_aggregation`: Build exposure, experience, and context features
//! - `frequency_glm`: Estimate how often claims are likely to occur
//! - `severity_glm`: Estimate the cost of claims when they happen
//! - `credibility_blending`: Stabilize sparse experience with portfolio priors
//! - `premium_calculation`: Price risk with transparent modifiers and loading
//! - `reserving`: Project outstanding liabilities with chain-ladder methods
//! - `explainability`: Make rating decisions inspectable and defensible
//! - `rate_filing`: Package audit-ready artifacts for regulatory review

pub mod credibility_blending;
pub mod explainability;
pub mod feature_aggregation;
pub mod frequency_glm;
pub mod premium_calculation;
pub mod rate_filing;
pub mod reserving;
pub mod risk_detectors;
pub mod severity_glm;
pub mod telemetry_normalization;

// Re-export key types
pub use credibility_blending::{BlendedLoss, FullCredibilityBlender};
pub use explainability::ExplainabilityEngine;
pub use feature_aggregation::{FeatureAccumulator, FeatureAggregator};
pub use frequency_glm::{fit_poisson_glm, FrequencyModel};
pub use premium_calculation::PremiumEngine;
pub use rate_filing::RateFilingArtifact;
pub use reserving::ChainLadderReserving;
pub use risk_detectors::{Detector, RiskDetector};
pub use severity_glm::{fit_gamma_glm, SeverityModel};
pub use telemetry_normalization::TelemetryNormalizer;
