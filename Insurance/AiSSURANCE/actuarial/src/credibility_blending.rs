//! # Credibility Blending
//!
//! Stabilizes pricing when a machine, site, or fleet has limited experience.
//! Buhlmann credibility blends observed results with portfolio expectations so
//! AiSSURANCE can produce rates that are responsive without overreacting to
//! sparse data.
//!
//! ## Pricing Value
//! - Credibility weight Z = n / (n + k), where n is exposure, k is credibility parameter
//! - k estimated from variance: k = Var(observed) / Var(process)
//! - Blended rate = Z * observed + (1-Z) * base
//!
//! Real-time constraints: <1ms per blend operation.
//! No_std compatible for core logic.

use serde::{Deserialize, Serialize};

/// Credibility parameter k, estimated from variance ratio
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CredibilityParameter(pub f32);

impl CredibilityParameter {
    /// Create from variance ratio: k = Var(observed) / Var(process)
    /// Clamped to reasonable bounds to prevent over/under-credibility
    pub fn from_variance_ratio(observed_var: f32, process_var: f32) -> Self {
        let ratio = observed_var / process_var.max(f32::EPSILON);
        let k = ratio.clamp(0.01, 100.0); // Prevent extreme credibility
        Self(k)
    }

    /// Get the k value
    pub fn k(&self) -> f32 {
        self.0
    }
}

impl Default for CredibilityParameter {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Credibility weight Z = n / (n + k)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CredibilityWeight(pub f32);

impl CredibilityWeight {
    /// Calculate credibility weight from exposure n and parameter k
    /// Exposure typically in years or equivalent units
    pub fn from_exposure(exposure: f32, k: CredibilityParameter) -> Self {
        let n = exposure.max(0.0);
        let z = n / (n + k.k());
        Self(z.clamp(0.0, 1.0)) // Ensure bounds
    }

    /// Get the weight value
    pub fn weight(&self) -> f32 {
        self.0
    }
}

impl Default for CredibilityWeight {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Blended rate combining observed and base rates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BlendedRate {
    pub observed: f32,
    pub base: f32,
    pub credibility: CredibilityWeight,
    pub blended: f32,
}

impl BlendedRate {
    /// Blend observed rate with base rate using credibility
    pub fn blend(observed: f32, base: f32, credibility: CredibilityWeight) -> Self {
        let blended = credibility.weight() * observed + (1.0 - credibility.weight()) * base;
        Self {
            observed,
            base,
            credibility,
            blended,
        }
    }

    /// Get the final blended rate
    pub fn rate(&self) -> f32 {
        self.blended
    }
}

/// Credibility blender for frequency rates (claims per exposure unit)
pub struct FrequencyCredibilityBlender {
    base_frequency: f32, // Base expected claims per year
    k_parameter: CredibilityParameter,
}

impl FrequencyCredibilityBlender {
    /// Create blender with base frequency and credibility parameter
    pub fn new(base_frequency: f32, k_parameter: CredibilityParameter) -> Self {
        Self {
            base_frequency,
            k_parameter,
        }
    }

    /// Blend observed frequency with base using exposure
    /// Returns blended claims per year
    pub fn blend(&self, observed_frequency: f32, exposure_years: f32) -> BlendedRate {
        let credibility = CredibilityWeight::from_exposure(exposure_years, self.k_parameter);
        BlendedRate::blend(observed_frequency, self.base_frequency, credibility)
    }
}

/// Credibility blender for severity rates (loss per claim)
pub struct SeverityCredibilityBlender {
    base_severity: f32, // Base expected loss per claim
    k_parameter: CredibilityParameter,
}

impl SeverityCredibilityBlender {
    /// Create blender with base severity and credibility parameter
    pub fn new(base_severity: f32, k_parameter: CredibilityParameter) -> Self {
        Self {
            base_severity,
            k_parameter,
        }
    }

    /// Blend observed severity with base using claim count
    /// Returns blended loss per claim
    pub fn blend(&self, observed_severity: f32, claim_count: f32) -> BlendedRate {
        let credibility = CredibilityWeight::from_exposure(claim_count, self.k_parameter);
        BlendedRate::blend(observed_severity, self.base_severity, credibility)
    }
}

/// Combined frequency and severity credibility blending
pub struct FullCredibilityBlender {
    frequency_blender: FrequencyCredibilityBlender,
    severity_blender: SeverityCredibilityBlender,
}

impl FullCredibilityBlender {
    /// Create with base rates and credibility parameters
    pub fn new(
        base_frequency: f32,
        base_severity: f32,
        frequency_k: CredibilityParameter,
        severity_k: CredibilityParameter,
    ) -> Self {
        Self {
            frequency_blender: FrequencyCredibilityBlender::new(base_frequency, frequency_k),
            severity_blender: SeverityCredibilityBlender::new(base_severity, severity_k),
        }
    }

    /// Blend both frequency and severity, return expected loss
    pub fn blend_expected_loss(
        &self,
        observed_frequency: f32,
        observed_severity: f32,
        exposure_years: f32,
        claim_count: f32,
    ) -> BlendedLoss {
        let freq_blend = self
            .frequency_blender
            .blend(observed_frequency, exposure_years);
        let sev_blend = self.severity_blender.blend(observed_severity, claim_count);

        BlendedLoss {
            frequency: freq_blend,
            severity: sev_blend,
            expected_loss: freq_blend.rate() * sev_blend.rate(),
        }
    }
}

impl Default for FullCredibilityBlender {
    fn default() -> Self {
        Self::new(
            0.10,
            12_000.0,
            CredibilityParameter::default(),
            CredibilityParameter::default(),
        )
    }
}

/// Result of blending frequency and severity into expected loss
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlendedLoss {
    pub frequency: BlendedRate,
    pub severity: BlendedRate,
    pub expected_loss: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credibility_weight() {
        let k = CredibilityParameter(1.0);
        let z = CredibilityWeight::from_exposure(2.0, k);
        assert!((z.weight() - 2.0 / 3.0).abs() < 0.001);
    }

    #[test]
    fn test_blended_rate() {
        let credibility = CredibilityWeight(0.5);
        let blend = BlendedRate::blend(0.1, 0.05, credibility);
        assert!((blend.rate() - 0.075).abs() < 0.001);
    }

    #[test]
    fn test_frequency_blender() {
        let k = CredibilityParameter(1.0);
        let blender = FrequencyCredibilityBlender::new(0.08, k);
        let blend = blender.blend(0.12, 3.0);
        assert!(blend.rate() > 0.08 && blend.rate() < 0.12);
    }

    #[test]
    fn test_full_blender() {
        let freq_k = CredibilityParameter(1.0);
        let sev_k = CredibilityParameter(2.0);
        let blender = FullCredibilityBlender::new(0.08, 15000.0, freq_k, sev_k);
        let loss = blender.blend_expected_loss(0.12, 20000.0, 3.0, 5.0);
        assert!(loss.expected_loss > 0.0);
    }
}
