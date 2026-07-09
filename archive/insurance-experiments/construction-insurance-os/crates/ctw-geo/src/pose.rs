//! Machine pose: position + orientation + slope.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::{Meters, Radians, MonotonicMicros};
use crate::point::Point3;

/// Full 6-DOF pose of a machine at a moment in time.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MachinePose {
    pub timestamp: MonotonicMicros,
    pub position: Point3,
    /// Heading in radians from north, clockwise.
    pub heading: Radians,
    /// Pitch (slope along longitudinal axis), positive = nose up.
    pub pitch: Radians,
    /// Roll (lateral tilt), positive = right side down.
    pub roll: Radians,
}
