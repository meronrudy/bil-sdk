use axiom::{app::App, models::ViewId};

#[test]
fn app_constructs_with_default_scenario() {
    let app = App::default();
    assert_eq!(app.scenario_id(), "default");
    assert!(app.visible_row_count_for(ViewId::RiskRegistry) > 0);
}
