//! # JSON Output Module
//!
//! Serializes AiSSURANCE actuarial results into clean JSON that can feed demos,
//! APIs, dashboards, and carrier review workflows. The output is designed to be
//! human-readable while preserving every important modeling artifact.
//!
//! ## Output Strategy
//!
//! - **Structured Output**: Separate sections for each component
//! - **Pretty Printing**: Human-readable JSON with indentation
//! - **Complete Data**: Include all intermediate results for analysis
//! - **Versioning**: Include schema version for compatibility

use serde::{Deserialize, Serialize};
use actuarial::feature_aggregation::RiskFeatureBundle;
use actuarial::frequency_glm::FrequencyCoefficients;
use actuarial::severity_glm::SeverityCoefficients;
use actuarial::premium_calculation::Premium;
use actuarial::reserving::ReserveEstimate;
use actuarial::explainability::ExplainabilityResult;

/// Top-level JSON output structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActuarialResults {
    /// Schema version for compatibility
    pub version: String,
    /// Summary statistics
    pub summary: SummaryStats,
    /// Feature bundles
    pub bundles: Vec<RiskFeatureBundle>,
    /// GLM coefficients
    pub models: ModelResults,
    /// Premium calculations
    pub premiums: Vec<Premium>,
    /// Reserve estimates
    pub reserves: Vec<ReserveEstimate>,
    /// Explainability analysis
    pub explainability: ExplainabilityResult,
}

/// Summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryStats {
    pub total_machines: usize,
    pub total_telemetry_frames: usize,
    pub total_claims: usize,
    pub total_bundles: usize,
    pub processing_time_ms: u64,
}

/// GLM model results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResults {
    pub frequency: FrequencyCoefficients,
    pub severity: SeverityCoefficients,
}

/// Serialize actuarial results to JSON string
pub fn serialize_results(
    bundles: &[RiskFeatureBundle],
    freq_coeffs: &FrequencyCoefficients,
    sev_coeffs: &SeverityCoefficients,
    premiums: &[Premium],
    reserves: &[ReserveEstimate],
    explainability: &ExplainabilityResult,
    summary: SummaryStats,
) -> serde_json::Result<String> {
    let results = ActuarialResults {
        version: "1.0.0".to_string(),
        summary,
        bundles: bundles.to_vec(),
        models: ModelResults {
            frequency: freq_coeffs.clone(),
            severity: sev_coeffs.clone(),
        },
        premiums: premiums.to_vec(),
        reserves: reserves.to_vec(),
        explainability: explainability.clone(),
    };

    serde_json::to_string_pretty(&results)
}

/// Deserialize actuarial results from JSON string
pub fn deserialize_results(json: &str) -> serde_json::Result<ActuarialResults> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actuarial::feature_aggregation::{BehaviorFeatures, ExposureFeatures, ContextFeatures};
    use contracts::feature_versions::FeatureVersion;

    #[test]
    fn test_serialization_roundtrip() {
        let bundle = RiskFeatureBundle {
            machine_id: contracts::ids::MachineId::new(&mut rand::thread_rng()),
            version: FeatureVersion { major: 1, minor: 0 },
            behavior: BehaviorFeatures {
                harsh_decel_count: 5,
                geofence_incursions: 2,
                worker_proximity_events: 1,
                reverse_time_fraction: 0.1,
                blind_spot_occupancy: 0.05,
                operator_takeover_events: 0,
                maintenance_deferral_days: 3,
                slope_trench_edge_events: 1,
            },
            exposure: ExposureFeatures {
                total_hours: 160.0,
                operating_hours: 140.0,
                idle_hours: 20.0,
                weekend_fraction: 0.3,
                night_fraction: 0.2,
            },
            context: ContextFeatures {
                machine_age_years: 2.5,
                site_complexity_score: 7.5,
                weather_risk_score: 3.2,
                terrain_difficulty: 4.1,
            },
        };

        let freq_coeffs = FrequencyCoefficients {
            intercept: -2.1,
            coefficients: vec![0.5, -0.3, 0.1],
            deviance: 45.2,
            converged: true,
        };

        let sev_coeffs = SeverityCoefficients {
            intercept: 9.5,
            coefficients: vec![0.2, 0.1, -0.05],
            deviance: 123.4,
            converged: true,
        };

        let premium = Premium {
            machine_id: bundle.machine_id,
            expected_frequency: 0.08,
            expected_severity: 15000.0,
            expected_loss: 1200.0,
            risk_modifier: 1.2,
            expense_loading: 0.25,
            total_premium: 1800.0,
        };

        let reserve = ReserveEstimate {
            development_period: 12,
            outstanding_loss: 50000.0,
            development_factor: 1.15,
        };

        let explainability = ExplainabilityResult {
            counterfactuals: vec![],
            attributions: vec![],
            audit_log: vec![],
        };

        let summary = SummaryStats {
            total_machines: 10,
            total_telemetry_frames: 10000,
            total_claims: 8,
            total_bundles: 10,
            processing_time_ms: 150,
        };

        let json = serialize_results(
            &[bundle],
            &freq_coeffs,
            &sev_coeffs,
            &[premium],
            &[reserve],
            &explainability,
            summary,
        ).unwrap();

        let deserialized = deserialize_results(&json).unwrap();

        assert_eq!(deserialized.version, "1.0.0");
        assert_eq!(deserialized.summary.total_machines, 10);
        assert_eq!(deserialized.bundles.len(), 1);
        assert_eq!(deserialized.models.frequency.intercept, -2.1);
    }
}
