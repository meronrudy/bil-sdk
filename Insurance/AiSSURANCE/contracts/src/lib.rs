//! Canonical data contracts for cross-crate communication.

pub mod event_schemas;
pub mod feature_bundles;
pub mod feature_versions;
pub mod ids;
pub mod platform;
pub mod risk_events;
pub mod telemetry_observations;
pub mod timestamps;
pub mod units;

// Re-export commonly used types
pub use event_schemas::*;
pub use feature_bundles::*;
pub use feature_versions::*;
pub use ids::*;
pub use platform::*;
pub use risk_events::*;
pub use telemetry_observations::*;
pub use timestamps::*;
pub use units::*;

/// Claim record for insurance claims data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClaimRecord {
    pub claim_id: EventId,
    pub machine_id: MachineId,
    pub site_id: SiteId,
    pub timestamp: SiteTime,
    pub amount: f64,
    pub claim_type: String,
    pub description: String,
}
