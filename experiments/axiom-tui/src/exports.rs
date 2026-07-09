use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{
    app::App,
    models::{SimulateTarget, ViewId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPayload {
    pub scenario_id: String,
    pub view: ViewId,
    pub format: String,
    pub audience: String,
    pub generated_at: String,
    pub applied_filter: String,
    pub visible_row_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub scenario_id: String,
    pub target: SimulateTarget,
    pub scenario: String,
    pub iterations: usize,
    pub generated_at: String,
}

pub fn build_export_payload(
    app: &App,
    view: ViewId,
    format: &str,
    audience: &str,
) -> ExportPayload {
    ExportPayload {
        scenario_id: app.scenario_id().to_string(),
        view,
        format: format.to_string(),
        audience: audience.to_string(),
        generated_at: Local::now().format("%Y-%m-%d %H:%M").to_string(),
        applied_filter: app.current_filter(view).to_string(),
        visible_row_count: app.visible_row_count_for(view),
    }
}

pub fn build_simulation_result(
    app: &App,
    target: SimulateTarget,
    scenario: &str,
    iterations: usize,
) -> SimulationResult {
    SimulationResult {
        scenario_id: app.scenario_id().to_string(),
        target,
        scenario: scenario.to_string(),
        iterations,
        generated_at: Local::now().format("%Y-%m-%d %H:%M").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{app::App, models::ViewId};

    #[test]
    fn export_payload_contains_view_filters_and_timestamp() {
        let mut app = App::default();
        app.committed_filters
            .insert(ViewId::Reporting, "board".to_string());

        let payload = build_export_payload(&app, ViewId::Reporting, "Board pack", "Board");

        assert_eq!(payload.applied_filter, "board");
        assert_eq!(payload.view, ViewId::Reporting);
        assert!(!payload.generated_at.is_empty());
    }

    #[test]
    fn simulation_result_reflects_selected_target_scenario_and_iterations() {
        let app = App::default();

        let result = build_simulation_result(
            &app,
            SimulateTarget::Reserves,
            "Adverse large-loss shock",
            5_000,
        );

        assert_eq!(result.target, SimulateTarget::Reserves);
        assert_eq!(result.scenario, "Adverse large-loss shock");
        assert_eq!(result.iterations, 5_000);
    }
}
