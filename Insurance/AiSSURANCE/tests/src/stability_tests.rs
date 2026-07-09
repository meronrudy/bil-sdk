//! Stability tests for sparse and malformed risk-layer inputs.

use actuarial::frequency_glm::fit_poisson_glm;
use actuarial::premium_calculation::{ExpenseLoading, Premium, PremiumModifier};
use contracts::{MachineId, MonotonicMicros, TelemetryFrame};
use ndarray::{array, Array1};
use risk_layer::{RiskLayer, RiskLayerInput};

#[test]
fn malformed_frames_are_counted_and_do_not_abort() {
    let frames = vec![
        TelemetryFrame::new(
            MachineId::test_id(1),
            MonotonicMicros::new(1),
            b"{\"type\":\"motion\",\"speed\":2.0,\"acceleration\":-1.0,\"jerk\":0.0}".to_vec(),
        ),
        TelemetryFrame::new(
            MachineId::test_id(1),
            MonotonicMicros::new(2),
            b"not-json".to_vec(),
        ),
    ];

    let report = RiskLayer::default()
        .run_batch(RiskLayerInput::new(frames, vec![]))
        .expect("risk layer should tolerate bad frames");

    assert_eq!(report.frames_rejected, 1);
    assert_eq!(report.observations, 1);
    assert!(report.premiums[0].final_premium.is_finite());
}

#[test]
fn sparse_no_claim_input_uses_fallback_model() {
    let frame = TelemetryFrame::new(
        MachineId::test_id(1),
        MonotonicMicros::new(1),
        b"{\"type\":\"load\",\"load_percentage\":0.7}".to_vec(),
    );

    let report = RiskLayer::default()
        .run_batch(RiskLayerInput::new(vec![frame], vec![]))
        .expect("risk layer should run");

    assert!(report.model.used_fallback_model);
    assert!(report.model.frequency_converged);
    assert!(report.model.severity_converged);
    assert!(report.premiums[0].final_premium > 0.0);
}

#[test]
fn poisson_fit_handles_extreme_features_without_non_finite_values() {
    let features = array![[1e6, -1e6, 1e-6], [100.0, -100.0, 0.01], [0.0, 0.0, 0.0]];
    let targets = Array1::from_vec(vec![1.0, 0.0, 2.0]);
    let model = fit_poisson_glm(features.view(), targets.view(), None).unwrap();

    assert!(model.converged);
    assert!(model.intercept.is_finite());
    assert!(model.coefficients.iter().all(|coeff| coeff.is_finite()));
}

#[test]
fn premium_modifier_and_loading_bounds_hold() {
    let premium = Premium::calculate(
        1_000.0,
        PremiumModifier::new(99.0),
        ExpenseLoading::new(99.0),
    );

    assert_eq!(premium.modifier.value(), 2.0);
    assert_eq!(premium.expense_loading.percentage(), 1.0);
    assert_eq!(premium.final_premium, 4_000.0);
}
