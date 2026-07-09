//! Risk-layer facade over the actuarial engine.

use actuarial::{
    credibility_blending::{CredibilityParameter, FullCredibilityBlender},
    explainability::{ExplainabilityEngine, ExplainabilityResult},
    feature_aggregation::FeatureAggregator,
    frequency_glm::{fit_poisson_glm, FrequencyModel},
    premium_calculation::{
        ExpenseLoading, Premium, PremiumEngine, PremiumModifier, RiskModifierCalculator,
    },
    rate_filing::RateFilingArtifact,
    risk_detectors::RiskDetector,
    severity_glm::{fit_gamma_glm, SeverityModel},
    telemetry_normalization::TelemetryNormalizer,
};
use contracts::{ClaimRecord, FeatureVersion, RiskEvent, RiskFeatureBundle, TelemetryFrame};
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLayerConfig {
    pub feature_window_days: u32,
    pub feature_version: FeatureVersion,
    pub base_frequency: f32,
    pub base_severity: f32,
    pub expense_loading: f32,
}

impl Default for RiskLayerConfig {
    fn default() -> Self {
        Self {
            feature_window_days: 30,
            feature_version: FeatureVersion::default(),
            base_frequency: 0.10,
            base_severity: 12_000.0,
            expense_loading: 0.20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLayerInput {
    pub telemetry_frames: Vec<TelemetryFrame>,
    pub claims: Vec<ClaimRecord>,
}

impl RiskLayerInput {
    pub fn new(telemetry_frames: Vec<TelemetryFrame>, claims: Vec<ClaimRecord>) -> Self {
        Self {
            telemetry_frames,
            claims,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLayerReport {
    pub frames_ingested: usize,
    pub frames_rejected: usize,
    pub observations: usize,
    pub claims: usize,
    pub risk_events: Vec<RiskEvent>,
    pub feature_bundles: Vec<RiskFeatureBundle>,
    pub model: ModelReport,
    pub premiums: Vec<Premium>,
    pub explainability: ExplainabilityResult,
}

impl RiskLayerReport {
    pub fn to_filing_artifact(&self) -> RateFilingArtifact {
        RateFilingArtifact::new(
            self.feature_bundles.clone(),
            self.premiums.clone(),
            self.explainability.clone(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelReport {
    pub used_fallback_model: bool,
    pub expected_frequency: f32,
    pub expected_severity: f32,
    pub frequency_intercept: f32,
    pub severity_intercept: f32,
    pub frequency_converged: bool,
    pub severity_converged: bool,
}

#[derive(Debug, Error)]
pub enum RiskLayerError {
    #[error("feature aggregation failed")]
    Aggregation,
    #[error("feature window {0} is unsupported")]
    UnsupportedWindow(u32),
    #[error("no feature bundle was produced")]
    EmptyFeatures,
}

#[derive(Debug, Clone)]
pub struct RiskLayer {
    config: RiskLayerConfig,
}

impl RiskLayer {
    pub fn new(config: RiskLayerConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(RiskLayerConfig::default())
    }

    pub fn run_batch(&self, input: RiskLayerInput) -> Result<RiskLayerReport, RiskLayerError> {
        let normalizer = TelemetryNormalizer::new();
        let mut detector = RiskDetector::new();
        let mut aggregator = FeatureAggregator::new();
        let mut risk_events = Vec::new();
        let mut frames_rejected = 0;
        let mut observations = 0;

        for frame in &input.telemetry_frames {
            let Some(obs) = normalizer.normalize(frame) else {
                frames_rejected += 1;
                continue;
            };

            observations += 1;
            if let Some(event) = detector.step(&obs) {
                risk_events.push(event);
            }
            aggregator
                .step(&obs, obs.timestamp())
                .map_err(|_| RiskLayerError::Aggregation)?;
        }

        let bundle = aggregator
            .finalize_bundle(self.config.feature_window_days, self.config.feature_version)
            .map_err(|_| RiskLayerError::UnsupportedWindow(self.config.feature_window_days))?;
        let feature_bundles = vec![bundle];
        let bundle = feature_bundles
            .first()
            .ok_or(RiskLayerError::EmptyFeatures)?;

        let (frequency_model, severity_model, used_fallback_model) =
            self.fit_or_fallback(&feature_bundles, &input.claims);
        let exposure_years = exposure_years(bundle);
        let observed_frequency =
            observed_frequency(&input.claims, exposure_years, self.config.base_frequency);
        let observed_severity = observed_severity(&input.claims, self.config.base_severity);

        let blender = FullCredibilityBlender::new(
            self.config.base_frequency,
            self.config.base_severity,
            CredibilityParameter::default(),
            CredibilityParameter::default(),
        );
        let blended_loss = blender.blend_expected_loss(
            observed_frequency,
            observed_severity,
            exposure_years,
            input.claims.len() as f32,
        );

        let risk_modifier = risk_modifier(bundle);
        let premium_engine = PremiumEngine::with_settings(
            PremiumModifier::new(1.0),
            ExpenseLoading::new(self.config.expense_loading),
        );
        let premiums = vec![premium_engine.calculate_premium(&blended_loss, Some(risk_modifier))];
        let explainability = ExplainabilityEngine::new().summarize(bundle);

        Ok(RiskLayerReport {
            frames_ingested: input.telemetry_frames.len(),
            frames_rejected,
            observations,
            claims: input.claims.len(),
            risk_events,
            feature_bundles,
            model: ModelReport {
                used_fallback_model,
                expected_frequency: blended_loss.frequency.rate(),
                expected_severity: blended_loss.severity.rate(),
                frequency_intercept: frequency_model.intercept,
                severity_intercept: severity_model.intercept,
                frequency_converged: frequency_model.converged,
                severity_converged: severity_model.converged,
            },
            premiums,
            explainability,
        })
    }

    fn fit_or_fallback(
        &self,
        bundles: &[RiskFeatureBundle],
        claims: &[ClaimRecord],
    ) -> (FrequencyModel, SeverityModel, bool) {
        if bundles.len() >= 3 && !claims.is_empty() {
            let features = feature_matrix(bundles);
            let frequency_targets =
                Array1::from_elem(bundles.len(), claims.len() as f32 / bundles.len() as f32);
            let severity_targets = Array1::from_elem(
                bundles.len(),
                observed_severity(claims, self.config.base_severity),
            );

            if let (Ok(freq), Ok(sev)) = (
                fit_poisson_glm(features.view(), frequency_targets.view(), None),
                fit_gamma_glm(features.view(), severity_targets.view(), None),
            ) {
                return (freq, sev, false);
            }
        }

        (
            fallback_frequency_model(self.config.base_frequency),
            fallback_severity_model(self.config.base_severity),
            true,
        )
    }
}

impl Default for RiskLayer {
    fn default() -> Self {
        Self::with_defaults()
    }
}

fn feature_matrix(bundles: &[RiskFeatureBundle]) -> Array2<f32> {
    let mut matrix = Array2::zeros((bundles.len(), 8));
    for (row, bundle) in bundles.iter().enumerate() {
        for (col, value) in bundle.feature_vector().iter().enumerate() {
            matrix[[row, col]] = *value;
        }
    }
    matrix
}

fn fallback_frequency_model(base_frequency: f32) -> FrequencyModel {
    FrequencyModel {
        coefficients: Array1::zeros(8),
        intercept: base_frequency.max(1e-6).ln(),
        deviance: 0.0,
        iterations: 0,
        converged: true,
    }
}

fn fallback_severity_model(base_severity: f32) -> SeverityModel {
    SeverityModel {
        coefficients: Array1::zeros(8),
        intercept: base_severity.max(1e-6).ln(),
        deviance: 0.0,
        iterations: 0,
        converged: true,
        dispersion: 1.0,
    }
}

fn exposure_years(bundle: &RiskFeatureBundle) -> f32 {
    (bundle.exposure.total_hours / (24.0 * 365.25)).max(0.0)
}

fn observed_frequency(claims: &[ClaimRecord], exposure_years: f32, base_frequency: f32) -> f32 {
    if claims.is_empty() || exposure_years <= 0.0 {
        base_frequency
    } else {
        (claims.len() as f32 / exposure_years).clamp(0.0, 10.0)
    }
}

fn observed_severity(claims: &[ClaimRecord], base_severity: f32) -> f32 {
    if claims.is_empty() {
        base_severity
    } else {
        let total: f64 = claims.iter().map(|claim| claim.amount).sum();
        (total / claims.len() as f64) as f32
    }
}

fn risk_modifier(bundle: &RiskFeatureBundle) -> PremiumModifier {
    let behavior_score = (bundle.behavior.harsh_deceleration_count
        + bundle.behavior.geofence_violation_count
        + bundle.behavior.worker_proximity_count
        + bundle.behavior.maintenance_deferral_count) as f32
        / 50.0;
    let exposure_score =
        (bundle.exposure.average_load_factor + bundle.exposure.total_hours / 200.0) / 2.0;
    let context_score = (bundle.context.low_visibility_hours / 10.0).min(1.0);
    let calculator = RiskModifierCalculator::new();
    let score = calculator.risk_score_from_features(
        behavior_score.clamp(0.0, 1.0),
        exposure_score.clamp(0.0, 1.0),
        context_score.clamp(0.0, 1.0),
    );
    calculator.modifier_from_risk_score(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use contracts::{MachineId, MonotonicMicros};

    #[test]
    fn rejects_malformed_frames_but_reports_success() {
        let frame = TelemetryFrame::new(
            MachineId::test_id(1),
            MonotonicMicros::new(1),
            b"not json".to_vec(),
        );
        let report = RiskLayer::default()
            .run_batch(RiskLayerInput::new(vec![frame], vec![]))
            .unwrap();
        assert_eq!(report.frames_rejected, 1);
        assert_eq!(report.observations, 0);
        assert!(report.model.used_fallback_model);
        assert!(report.premiums[0].final_premium > 0.0);
    }
}
