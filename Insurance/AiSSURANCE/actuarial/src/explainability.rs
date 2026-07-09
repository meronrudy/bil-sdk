//! Lightweight explainability helpers for risk-layer model summaries.

use contracts::{RiskFeatureBundle, SiteTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionResult {
    pub feature_name: String,
    pub importance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualScenario {
    pub scenario_name: String,
    pub expected_loss_delta: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: SiteTime,
    pub action: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityResult {
    pub counterfactuals: Vec<CounterfactualScenario>,
    pub attributions: Vec<AttributionResult>,
    pub audit_log: Vec<AuditLogEntry>,
}

#[derive(Debug, Default)]
pub struct ExplainabilityEngine;

impl ExplainabilityEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn summarize(&self, bundle: &RiskFeatureBundle) -> ExplainabilityResult {
        let features = [
            (
                "harsh_deceleration_count",
                bundle.behavior.harsh_deceleration_count as f32,
            ),
            (
                "geofence_violation_count",
                bundle.behavior.geofence_violation_count as f32,
            ),
            (
                "worker_proximity_count",
                bundle.behavior.worker_proximity_count as f32,
            ),
            ("average_load_factor", bundle.exposure.average_load_factor),
            ("low_visibility_hours", bundle.context.low_visibility_hours),
        ];

        let max = features
            .iter()
            .map(|(_, value)| value.abs())
            .fold(1.0_f32, f32::max);
        let attributions = features
            .iter()
            .map(|(name, value)| AttributionResult {
                feature_name: (*name).to_string(),
                importance: (value.abs() / max).clamp(0.0, 1.0),
            })
            .collect();

        ExplainabilityResult {
            counterfactuals: vec![CounterfactualScenario {
                scenario_name: "remove_behavior_events".to_string(),
                expected_loss_delta: -0.10,
            }],
            attributions,
            audit_log: vec![AuditLogEntry {
                timestamp: SiteTime::now(),
                action: "risk_layer_summary".to_string(),
                details: "Generated deterministic feature attribution summary".to_string(),
            }],
        }
    }
}
