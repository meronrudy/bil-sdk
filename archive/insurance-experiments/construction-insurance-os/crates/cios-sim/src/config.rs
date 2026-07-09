//! Configuration structures for simulation parameters.

use serde::{Deserialize, Serialize};

/// Top-level configuration for simulation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    /// Fleet configuration
    pub fleet: FleetConfig,
    /// Sites configuration
    pub sites: SitesConfig,
    /// Telemetry configuration
    pub telemetry: TelemetryConfig,
    /// Risk events configuration
    pub risk_events: RiskEventsConfig,
    /// Exposure configuration
    pub exposure: ExposureConfig,
    /// Policies configuration
    pub policies: PoliciesConfig,
    /// Claims configuration
    pub claims: ClaimsConfig,
    /// Triangles configuration
    pub triangles: TrianglesConfig,
    /// Features configuration
    pub features: FeaturesConfig,
}

/// Fleet generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetConfig {
    pub n_machines: usize,
    pub machine_classes: Vec<String>, // e.g., ["excavator", "loader", "crane", "haul_truck"]
    pub class_weights: Vec<f64>,
}

/// Sites generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitesConfig {
    pub n_sites: usize,
    pub zones_per_site: std::ops::Range<usize>,
    pub workers_per_site: std::ops::Range<usize>,
}

/// Telemetry generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub duration_hours: f64,
    pub sample_rate_hz: f64,
    pub anomaly_rate: f64,
}

/// Risk events generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEventsConfig {
    pub n_events: usize,
    pub event_type_weights: std::collections::HashMap<String, f64>,
    pub severity_distribution: SeverityDist,
}

/// Severity distribution parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeverityDist {
    Beta { alpha: f64, beta: f64 },
    Normal { mean: f64, std: f64 },
}

/// Exposure generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureConfig {
    pub total_hours: f64,
    pub autonomous_fraction: f64,
    pub night_fraction: f64,
}

/// Policies generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliciesConfig {
    pub n_policies: usize,
    pub term_months: usize,
    pub premium_range: std::ops::Range<f64>,
}

/// Claims generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimsConfig {
    pub frequency: f64,
    pub severity_mean: f64,
    pub severity_cv: f64,
}

/// Triangles generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrianglesConfig {
    pub n_origins: usize,
    pub n_development: usize,
    pub base_premium: f64,
    pub loss_ratio: f64,
    pub development_pattern: Vec<f64>,
}

/// Features generation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub n_observations: usize,
    pub n_features: usize,
    pub true_coefficients: Vec<f64>,
    pub family: Family,
}

/// GLM family for feature generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Family {
    Poisson,
    Gamma,
    Gaussian,
}

/// Pre-built scenario profiles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioProfile {
    /// Low-risk fleet with telematics, few claims
    WellManagedFleet,
    /// High-risk fleet: night ops, proximity events, many claims
    HighRiskUrbanSite,
    /// Mixed fleet for credibility testing
    MixedPortfolio,
    /// Minimal data for edge-case testing
    SparseData,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            fleet: FleetConfig::default(),
            sites: SitesConfig::default(),
            telemetry: TelemetryConfig::default(),
            risk_events: RiskEventsConfig::default(),
            exposure: ExposureConfig::default(),
            policies: PoliciesConfig::default(),
            claims: ClaimsConfig::default(),
            triangles: TrianglesConfig::default(),
            features: FeaturesConfig::default(),
        }
    }
}

impl Default for FleetConfig {
    fn default() -> Self {
        Self {
            n_machines: 50,
            machine_classes: vec![
                "excavator".to_string(),
                "loader".to_string(),
                "crane".to_string(),
                "haul_truck".to_string(),
            ],
            class_weights: vec![0.3, 0.25, 0.2, 0.25],
        }
    }
}

impl Default for SitesConfig {
    fn default() -> Self {
        Self {
            n_sites: 5,
            zones_per_site: 3..8,
            workers_per_site: 10..50,
        }
    }
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            duration_hours: 2000.0,
            sample_rate_hz: 10.0,
            anomaly_rate: 0.02,
        }
    }
}

impl Default for RiskEventsConfig {
    fn default() -> Self {
        let mut weights = std::collections::HashMap::new();
        weights.insert("harsh_decel".to_string(), 0.4);
        weights.insert("overswing".to_string(), 0.3);
        weights.insert("proximity".to_string(), 0.2);
        weights.insert("geofence".to_string(), 0.1);

        Self {
            n_events: 500,
            event_type_weights: weights,
            severity_distribution: SeverityDist::Beta { alpha: 2.0, beta: 5.0 },
        }
    }
}

impl Default for ExposureConfig {
    fn default() -> Self {
        Self {
            total_hours: 25000.0,
            autonomous_fraction: 0.15,
            night_fraction: 0.20,
        }
    }
}

impl Default for PoliciesConfig {
    fn default() -> Self {
        Self {
            n_policies: 200,
            term_months: 12,
            premium_range: 5000.0..50000.0,
        }
    }
}

impl Default for ClaimsConfig {
    fn default() -> Self {
        Self {
            frequency: 0.05,
            severity_mean: 15000.0,
            severity_cv: 1.5,
        }
    }
}

impl Default for TrianglesConfig {
    fn default() -> Self {
        Self {
            n_origins: 10,
            n_development: 10,
            base_premium: 1_000_000.0,
            loss_ratio: 0.65,
            development_pattern: vec![
                0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0,
            ],
        }
    }
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            n_observations: 500,
            n_features: 6,
            true_coefficients: vec![0.5, -0.3, 0.2, 0.1, -0.4, 0.6],
            family: Family::Poisson,
        }
    }
}