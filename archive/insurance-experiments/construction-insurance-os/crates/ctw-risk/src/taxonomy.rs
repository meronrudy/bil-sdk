//! Event taxonomy: severity tiers and risk hierarchy.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Risk tier classification for prioritization and pricing.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskTier {
    /// Level 1: Immediate risky behaviors (high volume, fast signal).
    ImmediateBehavior,
    /// Level 2: Near-miss and control quality (intermediate signal).
    NearMiss,
    /// Level 3: Loss precursors (slower signal).
    LossPrecursor,
    /// Level 4: Actual claims/incidents (sparse but decisive).
    ActualLoss,
}

/// Map event types to their risk tier.
pub fn event_tier(event_type: super::RiskEventType) -> RiskTier {
    use super::RiskEventType::*;
    match event_type {
        HarshDeceleration | HarshAcceleration | Overspeed | Overswing => {
            RiskTier::ImmediateBehavior
        }
        WorkerProximity | EquipmentProximity | GeofenceIncursion
        | BlindSpotSwing | ReverseLowVisibility | SlopeDanger
        | TrenchEdgeApproach | OverloadAttempt | UnstableLoadPath => {
            RiskTier::NearMiss
        }
        WorkerProximityCritical | OperatorTakeover
        | SafetyInterventionTriggered | HealthException
        | MaintenanceDeferral => {
            RiskTier::LossPrecursor
        }
    }
}
