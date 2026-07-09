//! Golden tests for the public risk-layer facade.

use risk_layer::{RiskLayer, RiskLayerInput};
use synthetic::{
    ClaimsGenerator, ClaimsParameters, FleetConfig, TelemetryGenerator, TelemetryParameters,
};

#[test]
fn golden_risk_layer_pipeline_is_deterministic() {
    let first = run_pipeline();
    let second = run_pipeline();

    assert_eq!(first.frames_ingested, second.frames_ingested);
    assert_eq!(first.frames_rejected, second.frames_rejected);
    assert_eq!(first.risk_events.len(), second.risk_events.len());
    assert_eq!(first.feature_bundles, second.feature_bundles);
    assert_eq!(
        first.premiums[0].final_premium,
        second.premiums[0].final_premium
    );
}

#[test]
fn golden_risk_layer_pipeline_outputs_underwriting_artifacts() {
    let report = run_pipeline();

    assert!(report.frames_ingested > 0);
    assert_eq!(report.frames_rejected, 0);
    assert!(report.observations > 0);
    assert!(!report.risk_events.is_empty());
    assert_eq!(report.feature_bundles.len(), 1);
    assert_eq!(report.premiums.len(), 1);
    assert!(report.premiums[0].expected_loss > 0.0);
    assert!(report.premiums[0].final_premium > report.premiums[0].expected_loss * 0.5);
    assert!(report.model.expected_frequency.is_finite());
    assert!(report.model.expected_severity.is_finite());
    assert!(!report.explainability.attributions.is_empty());
}

fn run_pipeline() -> risk_layer::RiskLayerReport {
    let fleet_config = FleetConfig::default();
    let telemetry_frames: Vec<_> = TelemetryGenerator::new(
        fleet_config.clone(),
        TelemetryParameters {
            period_days: 7,
            seed: 42,
        },
    )
    .collect();
    let mut claims_generator = ClaimsGenerator::new(
        fleet_config,
        ClaimsParameters {
            period_days: 90,
            seed: 42,
            ..ClaimsParameters::default()
        },
    );
    let claims = claims_generator.generate_claims();

    RiskLayer::default()
        .run_batch(RiskLayerInput::new(telemetry_frames, claims))
        .expect("risk layer should run")
}
