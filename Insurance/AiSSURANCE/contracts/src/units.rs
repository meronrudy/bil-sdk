//! Unit-safe physical measurements shared by the risk pipeline.

use core::ops::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};

macro_rules! finite_unit {
    ($name:ident, $non_negative:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
        pub struct $name(pub f32);

        impl $name {
            pub fn new(value: f32) -> Option<Self> {
                if value.is_finite() && (!$non_negative || value >= 0.0) {
                    Some(Self(value))
                } else {
                    None
                }
            }

            pub const fn unchecked(value: f32) -> Self {
                Self(value)
            }

            pub fn value(self) -> f32 {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(0.0)
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl Mul<f32> for $name {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl Div<f32> for $name {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                if rhs == 0.0 {
                    Self(0.0)
                } else {
                    Self(self.0 / rhs)
                }
            }
        }
    };
}

finite_unit!(Meters, true);
finite_unit!(MetersPerSecond, false);
finite_unit!(Seconds, true);
finite_unit!(MetersPerSecondSquared, false);
finite_unit!(Kilograms, true);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Degrees(pub f32);

impl Degrees {
    pub fn new(value: f32) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let normalized = ((value + 180.0).rem_euclid(360.0)) - 180.0;
        Some(Self(normalized))
    }

    pub fn value(self) -> f32 {
        self.0
    }
}

impl Default for Degrees {
    fn default() -> Self {
        Self(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_non_negative_distance() {
        assert!(Meters::new(1.0).is_some());
        assert!(Meters::new(-1.0).is_none());
    }
}
