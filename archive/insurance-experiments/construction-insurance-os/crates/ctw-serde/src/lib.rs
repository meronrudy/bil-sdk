//! Serialization support. Enables JSON, CBOR, and other formats
//! for risk bundles, events, and actuarial inputs.

pub use serde_json;

use ctw_core::*;
use ctw_risk::events::RiskEvent;
use ctw_exposure::ExposureBundle;
use ctw_context::ContextBundle;

/// The complete risk bundle in serializable form.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SerializableRiskBundle {
    pub policy_id: PolicyId,
    pub period_start_epoch: i64,
    pub period_end_epoch: i64,
    pub exposure: ExposureBundle,
    pub context: ContextBundle,
    pub event_summary: EventSummary,
    pub schema_version: FeatureVersion,
}

/// Summary statistics of risk events for a period.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EventSummary {
    pub total_events: u64,
    pub harsh_decel_count: u64,
    pub worker_prox_count: u64,
    pub worker_prox_critical_count: u64,
    pub geofence_count: u64,
    pub overswing_count: u64,
    pub maint_deferral_count: u64,
    pub mean_severity: f64,
    pub p95_severity: f64,
}
