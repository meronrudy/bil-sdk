use anyhow::anyhow;
use contracts::{ActionCommand, MachineId, MonotonicMicros};
use control_plane::{BatchJobRecord, ControlPlane, ControlPlaneConfig, StoredArtifacts};
use risk_layer::{RiskLayer, RiskLayerInput, RiskLayerReport};
use safety_layer::{
    runtime::ReplayReport, runtime::ReplayStep, runtime::SafetyRuntime, SafetyState,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use synthetic::{
    ClaimsGenerator, ClaimsParameters, FleetConfig, TelemetryGenerator, TelemetryParameters,
};
use vla_layer::{DeterministicPlanner, PlannerInput, VlaResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoRunResult {
    pub fleet_config: FleetConfig,
    pub input: RiskLayerInput,
    pub report: RiskLayerReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformDemoResult {
    pub fleet_config: FleetConfig,
    pub risk_input: RiskLayerInput,
    pub planner_input: PlannerInput,
    pub planner: VlaResult,
    pub replay_steps: Vec<ReplayStep>,
    pub safety: ReplayReport,
    pub job: BatchJobRecord,
    pub risk_report: RiskLayerReport,
    pub artifacts: StoredArtifacts,
}

pub fn load_config(config_path: Option<PathBuf>) -> anyhow::Result<FleetConfig> {
    match config_path {
        Some(path) => FleetConfig::load_from_file(path).map_err(|err| anyhow!(err.to_string())),
        None => Ok(FleetConfig::default()),
    }
}

pub fn build_risk_input(fleet_config: &FleetConfig) -> RiskLayerInput {
    let telemetry_frames: Vec<_> = TelemetryGenerator::new(
        fleet_config.clone(),
        TelemetryParameters {
            period_days: fleet_config.global.simulation_days.min(90),
            seed: fleet_config.global.random_seed,
        },
    )
    .collect();

    let mut claims_generator = ClaimsGenerator::new(
        fleet_config.clone(),
        ClaimsParameters {
            period_days: 90,
            seed: 42,
            ..ClaimsParameters::default()
        },
    );
    let claims = claims_generator.generate_claims();
    RiskLayerInput::new(telemetry_frames, claims)
}

pub fn default_planner_input() -> PlannerInput {
    PlannerInput {
        machine_id: MachineId::test_id(1),
        timestamp: MonotonicMicros::new(1),
        state: shared::MachineState::default(),
        requested_command: ActionCommand {
            linear_velocity: 4.0,
            angular_velocity: 0.2,
            emergency_stop: false,
        },
        obstacle_count: 0,
        workers_nearby: 1,
        route_label: "alpha-demo-route".to_string(),
    }
}

pub fn default_replay_steps(planner: &VlaResult) -> anyhow::Result<Vec<ReplayStep>> {
    let proposal = planner
        .proposal
        .clone()
        .ok_or_else(|| anyhow!("planner returned no proposal"))?;

    Ok(vec![ReplayStep {
        state: SafetyState {
            machine_id: proposal.machine_id,
            ..SafetyState::default()
        },
        proposal,
    }])
}

pub fn run_demo(config_path: Option<PathBuf>) -> anyhow::Result<DemoRunResult> {
    let fleet_config = load_config(config_path)?;
    let input = build_risk_input(&fleet_config);
    let report = RiskLayer::default().run_batch(input.clone())?;

    Ok(DemoRunResult {
        fleet_config,
        input,
        report,
    })
}

pub fn run_platform_demo(
    config_path: Option<PathBuf>,
    data_dir: PathBuf,
) -> anyhow::Result<PlatformDemoResult> {
    let fleet_config = load_config(config_path)?;
    let risk_input = build_risk_input(&fleet_config);

    let planner_input = default_planner_input();
    let planner = DeterministicPlanner::default().plan(&planner_input)?;
    let replay_steps = default_replay_steps(&planner)?;
    let safety = SafetyRuntime::default().run_replay(&replay_steps)?;

    let control_plane = ControlPlane::new(ControlPlaneConfig { data_dir }, RiskLayer::default());
    let job = control_plane.submit_batch(risk_input.clone())?;
    let risk_report = control_plane.report(&job.job_id)?;

    Ok(PlatformDemoResult {
        fleet_config,
        risk_input,
        planner_input,
        planner,
        replay_steps,
        safety,
        artifacts: job.artifacts.clone(),
        job,
        risk_report,
    })
}
