//! # Python Integration Surface
//!
//! Stable traits for bringing AiSSURANCE actuarial intelligence into Python
//! workflows. These interfaces are designed for analytics notebooks, API
//! services, model governance tooling, and future PyO3 class wrappers.
//!
//! The goal is a polished Python-facing product surface without giving up the
//! performance and type discipline of the Rust engine underneath.

use crate::credibility_blending::{CredibilityBlender, BlendedLoss};
use crate::explainability::{CounterfactualEngine, AttributionEngine, ExplainabilityError};
use crate::feature_aggregation::FeatureAggregator;
use crate::premium_calculation::{PremiumEngine, Premium};
use crate::reserving::{ChainLadderReserving, Reserves};
use contracts::feature_bundles::RiskFeatureBundle;
use contracts::telemetry_observations::CanonicalObservation;
use std::collections::HashMap;

/// Trait for frequency models (Poisson GLM).
/// Predicts expected number of claims per exposure period.
pub trait FrequencyModel {
    /// Predict frequency for a feature bundle.
    /// Returns expected claims per unit exposure.
    fn predict_frequency(&self, bundle: &RiskFeatureBundle) -> Result<f64, ModelError>;

    /// Get model coefficients for explainability.
    fn get_coefficients(&self) -> HashMap<String, f64>;
}

/// Trait for severity models (Gamma GLM).
/// Predicts expected claim severity (cost per claim).
pub trait SeverityModel {
    /// Predict severity for a feature bundle.
    /// Returns expected cost per claim.
    fn predict_severity(&self, bundle: &RiskFeatureBundle) -> Result<f64, ModelError>;

    /// Get model coefficients for explainability.
    fn get_coefficients(&self) -> HashMap<String, f64>;
}

/// Combined frequency and severity model for loss prediction.
pub trait LossModel {
    /// Predict expected loss (frequency × severity).
    fn predict_loss(&self, bundle: &RiskFeatureBundle) -> Result<f64, ModelError>;

    /// Get both frequency and severity components.
    fn predict_components(&self, bundle: &RiskFeatureBundle) -> Result<LossComponents, ModelError>;
}

/// Components of expected loss prediction.
#[derive(Debug, Clone)]
pub struct LossComponents {
    pub frequency: f64,
    pub severity: f64,
    pub expected_loss: f64,
}

/// Trait for premium calculation engines.
pub trait PremiumCalculator {
    /// Calculate premium from expected loss with modifiers and loading.
    fn calculate_premium(&self, expected_loss: f64, modifiers: &[f64]) -> Result<Premium, ModelError>;
}

/// Trait for credibility blending engines.
pub trait CredibilityEngine {
    /// Blend observed and base rates using credibility theory.
    fn blend_rates(&self, observed: f64, base: f64, exposure: f64) -> Result<BlendedLoss, ModelError>;
}

/// Trait for reserving engines (chain-ladder method).
pub trait ReservingEngine {
    /// Calculate outstanding reserves from claims triangle.
    fn calculate_reserves(&self, claims_data: &[Vec<f64>]) -> Result<Reserves, ModelError>;
}

/// Trait for explainability engines.
pub trait ExplainabilityEngine {
    /// Generate counterfactual scenarios.
    fn counterfactual(&self, baseline: &RiskFeatureBundle, changes: HashMap<String, f64>) -> Result<CounterfactualResult, ExplainabilityError>;

    /// Calculate feature attribution (importance).
    fn attribute(&self, bundle: &RiskFeatureBundle) -> Result<AttributionResult, ExplainabilityError>;
}

/// Result of counterfactual analysis.
#[derive(Debug, Clone)]
pub struct CounterfactualResult {
    pub baseline_loss: f64,
    pub counterfactual_loss: f64,
    pub loss_difference: f64,
    pub percent_change: f64,
}

/// Result of attribution analysis.
#[derive(Debug, Clone)]
pub struct AttributionResult {
    pub feature_importance: HashMap<String, f64>,
    pub total_attribution: f64,
}

/// Trait for feature aggregation engines.
pub trait FeatureEngine {
    /// Process a stream of observations into feature bundles.
    fn process_observations(&self, observations: &[CanonicalObservation]) -> Result<RiskFeatureBundle, ModelError>;
}

/// Trait for telemetry normalization.
pub trait TelemetryNormalizer {
    /// Normalize raw telemetry frame to canonical observations.
    fn normalize_frame(&self, raw_data: &[u8]) -> Result<Vec<CanonicalObservation>, ModelError>;
}

/// Error type for model operations.
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Model not fitted")]
    NotFitted,
    #[error("Numerical error: {0}")]
    Numerical(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Factory trait for creating model instances.
/// Useful for Python bindings to construct models from configuration.
pub trait ModelFactory {
    /// Create a frequency model from fitted coefficients.
    fn create_frequency_model(&self, coefficients: HashMap<String, f64>) -> Result<Box<dyn FrequencyModel>, ModelError>;

    /// Create a severity model from fitted coefficients.
    fn create_severity_model(&self, coefficients: HashMap<String, f64>) -> Result<Box<dyn SeverityModel>, ModelError>;

    /// Create a combined loss model.
    fn create_loss_model(&self, freq_model: Box<dyn FrequencyModel>, sev_model: Box<dyn SeverityModel>) -> Result<Box<dyn LossModel>, ModelError>;
}

/// Configuration for model hyperparameters.
/// Used to parameterize model creation in Python.
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub ridge_penalty: f64,
    pub max_iterations: usize,
    pub convergence_tolerance: f64,
    pub step_halving: bool,
    pub clamp_predictions: bool,
}

/// Default configuration for conservative, stable fitting.
impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            ridge_penalty: 0.01,
            max_iterations: 100,
            convergence_tolerance: 1e-8,
            step_halving: true,
            clamp_predictions: true,
        }
    }
}
