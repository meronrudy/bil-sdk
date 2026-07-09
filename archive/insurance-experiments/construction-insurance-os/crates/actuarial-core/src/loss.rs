use serde::{Deserialize, Serialize};

/// Loss cost prediction output.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LossCostEstimate {
    pub expected_frequency: f64,
    pub expected_severity: f64,
    pub pure_premium: f64,
    pub loss_cost_per_1000_hours: f64,
}
