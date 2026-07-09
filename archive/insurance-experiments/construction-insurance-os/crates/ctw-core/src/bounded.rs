//! Bounded numeric types that enforce invariants at construction time.
//!
//! In insurance math, many values have natural bounds:
//! - Confidence is 0.0..=1.0
//! - Fractions are 0.0..=1.0
//! - Credibility factors are 0.0..=1.0
//! - Loss ratios can be >1.0 but not negative
//!
//! Encoding these bounds in the type system prevents entire classes of bugs.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// A value in [0.0, 1.0], representing confidence, probability, or fraction.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnitFloat(f64);

impl UnitFloat {
    /// Create a new `UnitFloat`, returning an error if out of [0.0, 1.0].
    pub fn new(value: f64) -> Result<Self, CoreError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(CoreError::OutOfBounds {
                name: "UnitFloat",
                value,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self(value))
    }

    /// Create without bounds checking. Use only when the source is trusted.
    ///
    /// # Safety (logical)
    /// Caller must guarantee value is in [0.0, 1.0].
    #[must_use]
    pub const fn new_unchecked(value: f64) -> Self {
        Self(value)
    }

    /// Clamp to [0.0, 1.0].
    #[must_use]
    pub fn clamped(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    #[must_use]
    pub const fn raw(self) -> f64 {
        self.0
    }

    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);
}

impl Default for UnitFloat {
    fn default() -> Self {
        Self::ZERO
    }
}

/// A value that is strictly non-negative.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NonNegative(f64);

impl NonNegative {
    pub fn new(value: f64) -> Result<Self, CoreError> {
        if value < 0.0 {
            return Err(CoreError::OutOfBounds {
                name: "NonNegative",
                value,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn clamped(value: f64) -> Self {
        Self(value.max(0.0))
    }

    #[must_use]
    pub const fn raw(self) -> f64 {
        self.0
    }

    pub const ZERO: Self = Self(0.0);
}

impl Default for NonNegative {
    fn default() -> Self {
        Self::ZERO
    }
}

/// A bounded float with configurable min/max, enforced at runtime.
/// Used for experience modifiers, premium adjustments, and schedule rating.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoundedFloat {
    value: f64,
    min: f64,
    max: f64,
}

impl BoundedFloat {
    pub fn new(value: f64, min: f64, max: f64) -> Result<Self, CoreError> {
        if min > max {
            return Err(CoreError::InvalidBounds { min, max });
        }
        Ok(Self {
            value: value.clamp(min, max),
            min,
            max,
        })
    }

    #[must_use]
    pub fn raw(self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn min_bound(self) -> f64 {
        self.min
    }

    #[must_use]
    pub fn max_bound(self) -> f64 {
        self.max
    }
}

/// Confidence level for a measurement or detection.
pub type Confidence = UnitFloat;

/// Credibility factor Z ∈ [0, 1].
pub type CredibilityZ = UnitFloat;

/// Fraction of time or exposure.
pub type Fraction = UnitFloat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_float_bounds() {
        assert!(UnitFloat::new(0.5).is_ok());
        assert!(UnitFloat::new(-0.1).is_err());
        assert!(UnitFloat::new(1.1).is_err());
    }

    #[test]
    fn clamping() {
        let f = UnitFloat::clamped(1.5);
        assert_eq!(f.raw(), 1.0);
        let f = UnitFloat::clamped(-0.5);
        assert_eq!(f.raw(), 0.0);
    }

    #[test]
    fn bounded_float_clamps() {
        let b = BoundedFloat::new(2.0, 0.7, 1.5).unwrap();
        assert_eq!(b.raw(), 1.5);
    }
}
