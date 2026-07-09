//! # Chain-Ladder Reserving
//!
//! Projects outstanding claim liabilities with a familiar, auditable reserving
//! workflow. AiSSURANCE uses chain-ladder methods to connect field telemetry,
//! claims emergence, and finance-ready reserve estimates.
//!
//! ## Reserving Outputs
//! - Claims Triangle: Cumulative paid/recovered claims by accident year and development period
//! - Development Factors: Ratios of consecutive development periods
//! - Ultimate Loss: Projected final claims amount
//! - Reserve: Ultimate - Paid (outstanding liability)
//!
//! Real-time constraints: <10ms for triangle updates and reserve calculations.
//! No_std compatible for core logic.

use core::ops::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};

/// Claims triangle dimensions (accident years × development periods)
pub const MAX_ACCIDENT_YEARS: usize = 10;
pub const MAX_DEVELOPMENT_PERIODS: usize = 12;

/// Cumulative claims amount (paid or incurred)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CumulativeClaims(f32);

impl CumulativeClaims {
    pub fn new(amount: f32) -> Self {
        Self(amount.max(0.0)) // Non-negative
    }

    pub fn amount(&self) -> f32 {
        self.0
    }
}

impl Add for CumulativeClaims {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for CumulativeClaims {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self((self.0 - other.0).max(0.0))
    }
}

impl Mul<f32> for CumulativeClaims {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self(self.0 * scalar.max(0.0))
    }
}

impl Div<f32> for CumulativeClaims {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        if scalar > 0.0 {
            Self(self.0 / scalar)
        } else {
            Self(0.0)
        }
    }
}

/// Claims triangle: cumulative claims by accident year and development period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimsTriangle {
    pub data: [[Option<CumulativeClaims>; MAX_DEVELOPMENT_PERIODS]; MAX_ACCIDENT_YEARS],
    pub accident_years: usize,
    pub max_periods: usize,
}

impl ClaimsTriangle {
    /// Create empty triangle
    pub fn new() -> Self {
        Self {
            data: [[None; MAX_DEVELOPMENT_PERIODS]; MAX_ACCIDENT_YEARS],
            accident_years: 0,
            max_periods: 0,
        }
    }

    /// Add claims for specific accident year and development period
    pub fn add_claims(
        &mut self,
        accident_year: usize,
        dev_period: usize,
        claims: CumulativeClaims,
    ) {
        if accident_year < MAX_ACCIDENT_YEARS && dev_period < MAX_DEVELOPMENT_PERIODS {
            self.data[accident_year][dev_period] = Some(claims);
            self.accident_years = self.accident_years.max(accident_year + 1);
            self.max_periods = self.max_periods.max(dev_period + 1);
        }
    }

    /// Get claims at position (or None if not available)
    pub fn get(&self, accident_year: usize, dev_period: usize) -> Option<CumulativeClaims> {
        if accident_year < MAX_ACCIDENT_YEARS && dev_period < MAX_DEVELOPMENT_PERIODS {
            self.data[accident_year][dev_period]
        } else {
            None
        }
    }
}

/// Development factor: ratio between consecutive development periods
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DevelopmentFactor(f32);

impl DevelopmentFactor {
    /// Calculate factor from two cumulative amounts: C_{i,j+1} / C_{i,j}
    pub fn from_cumulative(later: CumulativeClaims, earlier: CumulativeClaims) -> Option<Self> {
        if earlier.amount() > 0.0 {
            Some(Self(later.amount() / earlier.amount()))
        } else {
            None
        }
    }

    /// Get the factor value
    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Chain-ladder development factors for each development period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentFactors {
    pub factors: [Option<DevelopmentFactor>; MAX_DEVELOPMENT_PERIODS - 1],
}

impl DevelopmentFactors {
    /// Calculate development factors from triangle
    pub fn from_triangle(triangle: &ClaimsTriangle) -> Self {
        let mut factors = [None; MAX_DEVELOPMENT_PERIODS - 1];

        // For each development period j, calculate average factor across accident years
        for j in 0..(triangle.max_periods - 1) {
            let mut sum_factors = 0.0;
            let mut count = 0;

            for i in 0..triangle.accident_years {
                if let (Some(later), Some(earlier)) = (triangle.get(i, j + 1), triangle.get(i, j)) {
                    if let Some(factor) = DevelopmentFactor::from_cumulative(later, earlier) {
                        sum_factors += factor.value();
                        count += 1;
                    }
                }
            }

            if count > 0 {
                factors[j] = Some(DevelopmentFactor(sum_factors / count as f32));
            }
        }

        Self { factors }
    }

    /// Get factor for development period j
    pub fn get(&self, dev_period: usize) -> Option<DevelopmentFactor> {
        if dev_period < MAX_DEVELOPMENT_PERIODS - 1 {
            self.factors[dev_period]
        } else {
            None
        }
    }
}

/// Ultimate loss projection for an accident year
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UltimateLoss {
    pub projected: CumulativeClaims,
    pub paid_to_date: CumulativeClaims,
    pub reserve: CumulativeClaims,
}

impl UltimateLoss {
    /// Project ultimate loss using chain-ladder method
    pub fn project(
        paid_to_date: CumulativeClaims,
        factors: &DevelopmentFactors,
        current_period: usize,
    ) -> Self {
        let mut projected = paid_to_date;

        // Apply development factors sequentially
        for j in current_period..(MAX_DEVELOPMENT_PERIODS - 1) {
            if let Some(factor) = factors.get(j) {
                projected = projected * factor.value();
            } else {
                break; // No more factors available
            }
        }

        let reserve = projected - paid_to_date;

        Self {
            projected,
            paid_to_date,
            reserve,
        }
    }
}

/// Chain-ladder reserving engine
pub struct ChainLadderReserving {
    triangle: ClaimsTriangle,
    factors: DevelopmentFactors,
}

impl ChainLadderReserving {
    /// Create from claims triangle
    pub fn new(triangle: ClaimsTriangle) -> Self {
        let factors = DevelopmentFactors::from_triangle(&triangle);
        Self { triangle, factors }
    }

    /// Update triangle with new claims data
    pub fn update_triangle(
        &mut self,
        accident_year: usize,
        dev_period: usize,
        claims: CumulativeClaims,
    ) {
        self.triangle.add_claims(accident_year, dev_period, claims);
        // Recalculate factors
        self.factors = DevelopmentFactors::from_triangle(&self.triangle);
    }

    /// Calculate total reserves across all accident years
    pub fn calculate_total_reserve(&self) -> CumulativeClaims {
        let mut total_reserve = CumulativeClaims(0.0);

        for i in 0..self.triangle.accident_years {
            // Find latest development period with data for this accident year
            let mut latest_period = 0;
            let mut paid_to_date = CumulativeClaims(0.0);

            for j in 0..self.triangle.max_periods {
                if let Some(claims) = self.triangle.get(i, j) {
                    paid_to_date = claims;
                    latest_period = j;
                }
            }

            if paid_to_date.amount() > 0.0 {
                let ultimate = UltimateLoss::project(paid_to_date, &self.factors, latest_period);
                total_reserve = total_reserve + ultimate.reserve;
            }
        }

        total_reserve
    }

    /// Get projected ultimate losses for all accident years
    pub fn project_ultimate_losses(&self) -> Vec<Option<UltimateLoss>> {
        let mut projections = Vec::with_capacity(self.triangle.accident_years);

        for i in 0..self.triangle.accident_years {
            // Find latest data
            let mut latest_period = 0;
            let mut paid_to_date = None;

            for j in 0..self.triangle.max_periods {
                if let Some(claims) = self.triangle.get(i, j) {
                    paid_to_date = Some(claims);
                    latest_period = j;
                }
            }

            if let Some(paid) = paid_to_date {
                let ultimate = UltimateLoss::project(paid, &self.factors, latest_period);
                projections.push(Some(ultimate));
            } else {
                projections.push(None);
            }
        }

        projections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cumulative_claims() {
        let c1 = CumulativeClaims::new(1000.0);
        let c2 = CumulativeClaims::new(500.0);
        let sum = c1 + c2;
        assert_eq!(sum.amount(), 1500.0);
    }

    #[test]
    fn test_development_factor() {
        let earlier = CumulativeClaims::new(1000.0);
        let later = CumulativeClaims::new(1200.0);
        let factor = DevelopmentFactor::from_cumulative(later, earlier).unwrap();
        assert!((factor.value() - 1.2).abs() < 0.001);
    }

    #[test]
    fn test_triangle_operations() {
        let mut triangle = ClaimsTriangle::new();
        triangle.add_claims(0, 0, CumulativeClaims::new(1000.0));
        triangle.add_claims(0, 1, CumulativeClaims::new(1200.0));

        assert_eq!(triangle.get(0, 0).unwrap().amount(), 1000.0);
        assert_eq!(triangle.get(0, 1).unwrap().amount(), 1200.0);
        assert!(triangle.get(0, 2).is_none());
    }

    #[test]
    fn test_ultimate_loss_projection() {
        let paid = CumulativeClaims::new(1000.0);
        let factors = DevelopmentFactors {
            factors: [
                Some(DevelopmentFactor(1.2)),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        };

        let ultimate = UltimateLoss::project(paid, &factors, 0);
        assert!(ultimate.projected.amount() > paid.amount());
        assert!(ultimate.reserve.amount() > 0.0);
    }

    #[test]
    fn test_chain_ladder_basic() {
        let mut triangle = ClaimsTriangle::new();
        // Accident year 0: 1000 -> 1200 -> 1300
        triangle.add_claims(0, 0, CumulativeClaims::new(1000.0));
        triangle.add_claims(0, 1, CumulativeClaims::new(1200.0));
        triangle.add_claims(0, 2, CumulativeClaims::new(1300.0));

        // Accident year 1: 800 -> 1000
        triangle.add_claims(1, 0, CumulativeClaims::new(800.0));
        triangle.add_claims(1, 1, CumulativeClaims::new(1000.0));

        let reserving = ChainLadderReserving::new(triangle);
        let total_reserve = reserving.calculate_total_reserve();
        assert!(total_reserve.amount() > 0.0);
    }
}
