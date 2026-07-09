//! Alpha platform integration tests across VLA, safety, control plane, and risk.

use contracts::{ActionCommand, MachineId, MonotonicMicros, PlannerStatus};
use control_plane::{BatchJobStatus, ControlPlane, ControlPlaneConfig};
use safety_layer::{runtime::ReplayStep, runtime::SafetyRuntime, SafetyState};
use synthetic::{
    ClaimsGenerator, ClaimsParameters, FleetConfig, TelemetryGenerator, TelemetryParameters,
};
use vla_layer::{DeterministicPlanner, PlannerInput};

#[test]
fn alpha_platform_demo_runs_end_to_end() {
    let planner = DeterministicPlanner::default();
    let planner_input = PlannerInput {
        machine_id: MachineId::test_id(1),
        timestamp: MonotonicMicros::new(1),
        state: shared::MachineState::default(),
        requested_command: ActionCommand {
            linear_velocity: 5.0,
            angular_velocity: 0.2,
            emergency_stop: false,
        },
        obstacle_count: 0,
        workers_nearby: 1,
        route_label: "design-partner-alpha".to_string(),
    };
    let plan = planner.plan(&planner_input).unwrap();
    assert!(matches!(
        plan.status,
        PlannerStatus::Fallback | PlannerStatus::Ready
    ));

    let proposal = plan.proposal.clone().expect("planner should emit proposal");
    let safety_report = SafetyRuntime::default()
        .run_replay(&[ReplayStep {
            state: SafetyState {
                machine_id: proposal.machine_id,
                ..SafetyState::default()
            },
            proposal,
        }])
        .unwrap();
    assert_eq!(safety_report.decisions.len(), 1);
    assert!(safety_report.max_latency.as_millis() < 50);

    let fleet_config = FleetConfig::default();
    let telemetry_frames: Vec<_> = TelemetryGenerator::new(
        fleet_config.clone(),
        TelemetryParameters {
            period_days: 1,
            seed: 42,
        },
    )
    .collect();
    let mut claims_generator = ClaimsGenerator::new(fleet_config, ClaimsParameters::default());
    let input =
        risk_layer::RiskLayerInput::new(telemetry_frames, claims_generator.generate_claims());

    let root = std::env::temp_dir().join(format!(
        "aissurance-alpha-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let control_plane = ControlPlane::new(
        ControlPlaneConfig {
            data_dir: root.clone(),
        },
        risk_layer::RiskLayer::default(),
    );
    let job = control_plane.submit_batch(input).unwrap();
    let report = control_plane.report(&job.job_id).unwrap();

    assert_eq!(job.status, BatchJobStatus::Completed);
    assert!(job.artifacts.report_path.exists());
    assert!(!report.feature_bundles.is_empty());
    assert!(!report.premiums.is_empty());

    let _ = std::fs::remove_dir_all(root);
}
