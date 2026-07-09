//! # Premium Calculation
//!
//! Turns expected loss into transparent, configurable premiums for autonomous
//! construction risk. The module keeps every adjustment visible so carriers can
//! explain price movements and operators can see which behaviors matter.
//!
//! Formula: Premium = E[Loss] × Modifier × (1 + Expense Loading)
//!
//! ## Pricing Components
//! - E[Loss] = Frequency × Severity (from credibility blending)
//! - Modifier: Bounded adjustment factor (0.5 to 2.0) based on risk features
//! - Expense Loading: Fixed percentage for administrative costs (e.g., 20%)
//!
//! Real-time constraints: <1ms per premium calculation.
//! No_std compatible for core logic.

use crate::credibility_blending::BlendedLoss;
use serde::{Deserialize, Serialize};

/// Bounded modifier for premium adjustment (0.5 to 2.0)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PremiumModifier(f32);

impl PremiumModifier {
    /// Create modifier from raw value, clamped to bounds
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.5, 2.0))
    }

    /// Get the modifier value
    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Expense loading percentage (e.g., 0.20 for 20%)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExpenseLoading(f32);

impl ExpenseLoading {
    /// Create loading from percentage (0.0 to 1.0)
    pub fn new(percentage: f32) -> Self {
        Self(percentage.clamp(0.0, 1.0))
    }

    /// Get the loading factor (1 + percentage)
    pub fn factor(&self) -> f32 {
        1.0 + self.0
    }

    /// Get the percentage
    pub fn percentage(&self) -> f32 {
        self.0
    }
}

/// Calculated premium with breakdown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Premium {
    pub expected_loss: f32,
    pub modifier: PremiumModifier,
    pub expense_loading: ExpenseLoading,
    pub final_premium: f32,
}

impl Premium {
    /// Calculate premium from expected loss
    pub fn calculate(
        expected_loss: f32,
        modifier: PremiumModifier,
        expense_loading: ExpenseLoading,
    ) -> Self {
        let adjusted_loss = expected_loss * modifier.value();
        let final_premium = adjusted_loss * expense_loading.factor();

        Self {
            expected_loss,
            modifier,
            expense_loading,
            final_premium,
        }
    }

    /// Get the final premium amount
    pub fn amount(&self) -> f32 {
        self.final_premium
    }
}

/// Premium engine that combines credibility blending with modifiers
pub struct PremiumEngine {
    base_modifier: PremiumModifier,
    expense_loading: ExpenseLoading,
}

impl PremiumEngine {
    /// Create engine with default settings
    pub fn new() -> Self {
        Self {
            base_modifier: PremiumModifier::new(1.0),
            expense_loading: ExpenseLoading::new(0.20), // 20% loading
        }
    }

    /// Create with custom settings
    pub fn with_settings(base_modifier: PremiumModifier, expense_loading: ExpenseLoading) -> Self {
        Self {
            base_modifier,
            expense_loading,
        }
    }

    /// Calculate premium from blended loss and optional risk modifier
    /// Risk modifier adjusts based on machine behavior, site conditions, etc.
    pub fn calculate_premium(
        &self,
        blended_loss: &BlendedLoss,
        risk_modifier: Option<PremiumModifier>,
    ) -> Premium {
        let expected_loss = blended_loss.expected_loss;
        let modifier = risk_modifier.unwrap_or(self.base_modifier);

        Premium::calculate(expected_loss, modifier, self.expense_loading)
    }
}

impl Default for PremiumEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RiskModifierCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Risk-based modifier calculator
/// Adjusts premium based on risk features (behavior, exposure, context)
pub struct RiskModifierCalculator {
    // Coefficients for risk adjustment (simplified linear model)
    behavior_weight: f32,
    exposure_weight: f32,
    context_weight: f32,
}

impl RiskModifierCalculator {
    /// Create with default weights
    pub fn new() -> Self {
        Self {
            behavior_weight: 0.4,
            exposure_weight: 0.3,
            context_weight: 0.3,
        }
    }

    /// Calculate modifier from risk score (0.0 to 1.0, where 1.0 is highest risk)
    /// Maps risk score to modifier: low risk -> discount (0.5), high risk -> surcharge (2.0)
    pub fn modifier_from_risk_score(&self, risk_score: f32) -> PremiumModifier {
        // Linear mapping: risk_score 0.0 -> modifier 0.5, risk_score 1.0 -> modifier 2.0
        let modifier_value = 0.5 + risk_score * 1.5;
        PremiumModifier::new(modifier_value)
    }

    /// Calculate risk score from feature bundles (simplified)
    /// In practice, this would use GLM predictions or rule-based scoring
    pub fn risk_score_from_features(
        &self,
        behavior_score: f32, // 0.0 to 1.0
        exposure_score: f32, // 0.0 to 1.0
        context_score: f32,  // 0.0 to 1.0
    ) -> f32 {
        let weighted_score = self.behavior_weight * behavior_score
            + self.exposure_weight * exposure_score
            + self.context_weight * context_score;
        weighted_score.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credibility_blending::{BlendedRate, CredibilityWeight};

    #[test]
    fn test_premium_modifier_bounds() {
        let low = PremiumModifier::new(0.0);
        assert_eq!(low.value(), 0.5);

        let high = PremiumModifier::new(3.0);
        assert_eq!(high.value(), 2.0);

        let normal = PremiumModifier::new(1.5);
        assert_eq!(normal.value(), 1.5);
    }

    #[test]
    fn test_expense_loading() {
        let loading = ExpenseLoading::new(0.20);
        assert!((loading.factor() - 1.2).abs() < 0.001);
    }

    #[test]
    fn test_premium_calculation() {
        let expected_loss = 12000.0; // $12K expected loss
        let modifier = PremiumModifier::new(1.2);
        let loading = ExpenseLoading::new(0.20);

        let premium = Premium::calculate(expected_loss, modifier, loading);
        let expected = 12000.0 * 1.2 * 1.2; // 17280.0
        assert!((premium.amount() - expected).abs() < 0.001);
    }

    #[test]
    fn test_premium_engine() {
        let engine = PremiumEngine::new();

        // Mock blended loss
        let freq = BlendedRate::blend(0.1, 0.08, CredibilityWeight(0.6));
        let sev = BlendedRate::blend(15000.0, 12000.0, CredibilityWeight(0.5));
        let blended_loss = BlendedLoss {
            frequency: freq,
            severity: sev,
            expected_loss: freq.rate() * sev.rate(),
        };

        let premium = engine.calculate_premium(&blended_loss, None);
        assert!(premium.amount() > 0.0);
        assert!(premium.amount() > blended_loss.expected_loss); // Should include loading
    }

    #[test]
    fn test_risk_modifier_calculator() {
        let calc = RiskModifierCalculator::new();

        let risk_score = calc.risk_score_from_features(0.8, 0.6, 0.4);
        let modifier = calc.modifier_from_risk_score(risk_score);

        // Risk score should be around 0.62, modifier around 1.43
        assert!(modifier.value() > 1.0 && modifier.value() < 2.0);
    }
}
