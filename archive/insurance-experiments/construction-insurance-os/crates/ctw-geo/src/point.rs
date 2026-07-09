//! 2D and 3D points in local site coordinates.
//!
//! All site geometry uses a local metric frame. GNSS coordinates are
//! converted at the ingest boundary. This prevents every detector
//! from needing to handle geodetic math.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::Meters;

/// A point in the local site coordinate frame (meters, East-North-Up).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point2 {
    pub x: Meters,
    pub y: Meters,
}

impl Point2 {
    #[must_use]
    pub const fn new(x: Meters, y: Meters) -> Self {
        Self { x, y }
    }

    /// Euclidean distance to another point.
    #[must_use]
    pub fn distance_to(self, other: Self) -> Meters {
        let dx = self.x.raw() - other.x.raw();
        let dy = self.y.raw() - other.y.raw();
        Meters::new((dx * dx + dy * dy).sqrt())
    }
}

/// A 3D point in local site coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point3 {
    pub x: Meters,
    pub y: Meters,
    pub z: Meters,
}

impl Point3 {
    #[must_use]
    pub const fn new(x: Meters, y: Meters, z: Meters) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn distance_to(self, other: Self) -> Meters {
        let dx = self.x.raw() - other.x.raw();
        let dy = self.y.raw() - other.y.raw();
        let dz = self.z.raw() - other.z.raw();
        Meters::new((dx * dx + dy * dy + dz * dz).sqrt())
    }

    #[must_use]
    pub fn to_2d(self) -> Point2 {
        Point2 { x: self.x, y: self.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_2d() {
        let a = Point2::new(Meters::new(0.0), Meters::new(0.0));
        let b = Point2::new(Meters::new(3.0), Meters::new(4.0));
        let d = a.distance_to(b);
        assert!((d.raw() - 5.0).abs() < 1e-10);
    }
}
