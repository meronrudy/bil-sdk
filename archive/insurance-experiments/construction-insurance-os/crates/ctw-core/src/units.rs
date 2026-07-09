//! Physical units as newtypes.
//!
//! A `Meters` is not an `f64`. A `MetersPerSecond` is not a `Meters`.
//! These wrappers prevent the class of bugs where one vendor adapter
//! emits feet and another emits meters, and nobody notices until
//! a geofence incursion is mispriced by 3×.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! define_unit {
    ($(#[$meta:meta])* $name:ident, $suffix:expr) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name(pub f64);

        impl $name {
            #[must_use]
            pub const fn new(value: f64) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn raw(self) -> f64 {
                self.0
            }

            #[must_use]
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            #[must_use]
            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            #[must_use]
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:.3}{}", self.0, $suffix)
            }
        }

        impl core::ops::Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
        }

        impl core::ops::Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) }
        }

        impl core::ops::Mul<f64> for $name {
            type Output = Self;
            fn mul(self, rhs: f64) -> Self { Self(self.0 * rhs) }
        }

        impl core::ops::Div<f64> for $name {
            type Output = Self;
            fn div(self, rhs: f64) -> Self { Self(self.0 / rhs) }
        }
    };
}

define_unit!(/// Distance in meters.
    Meters, "m");
define_unit!(/// Speed in meters per second.
    MetersPerSecond, "m/s");
define_unit!(/// Acceleration in meters per second squared.
    MetersPerSecondSq, "m/s²");
define_unit!(/// Jerk in meters per second cubed.
    MetersPerSecondCubed, "m/s³");
define_unit!(/// Angular rate in radians per second.
    RadiansPerSecond, "rad/s");
define_unit!(/// Angle in radians.
    Radians, "rad");
define_unit!(/// Angle in degrees.
    Degrees, "°");
define_unit!(/// Duration in seconds.
    Seconds, "s");
define_unit!(/// Duration in hours.
    Hours, "h");
define_unit!(/// Mass in kilograms.
    Kilograms, "kg");
define_unit!(/// Force in newtons.
    Newtons, "N");
define_unit!(/// Temperature in celsius.
    Celsius, "°C");
define_unit!(/// Pressure in bar.
    Bar, "bar");
define_unit!(/// Currency amount (USD by default).
    Currency, "$");
define_unit!(/// Dimensionless ratio (e.g., loss ratio).
    Ratio, "");

/// Convert degrees to radians.
impl From<Degrees> for Radians {
    fn from(d: Degrees) -> Self {
        Radians(d.0 * core::f64::consts::PI / 180.0)
    }
}

/// Convert radians to degrees.
impl From<Radians> for Degrees {
    fn from(r: Radians) -> Self {
        Degrees(r.0 * 180.0 / core::f64::consts::PI)
    }
}

/// Convert hours to seconds.
impl From<Hours> for Seconds {
    fn from(h: Hours) -> Self {
        Seconds(h.0 * 3600.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_arithmetic() {
        let a = Meters::new(3.0);
        let b = Meters::new(4.0);
        assert_eq!((a + b).raw(), 7.0);
        assert_eq!((b - a).raw(), 1.0);
        assert_eq!((a * 2.0).raw(), 6.0);
    }

    #[test]
    fn degree_radian_conversion() {
        let d = Degrees::new(180.0);
        let r: Radians = d.into();
        assert!((r.raw() - core::f64::consts::PI).abs() < 1e-10);
    }
}
