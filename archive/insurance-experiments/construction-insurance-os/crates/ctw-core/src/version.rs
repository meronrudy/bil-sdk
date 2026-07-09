//! Feature and schema versioning.
//!
//! This matters enormously in insurance: if you change the definition
//! of "near_miss" from distance < 1.5m to distance < 2.0m, that is
//! not a minor implementation change. It is a statistical regime shift
//! that can silently corrupt downstream pricing models.
//!
//! Every feature definition, threshold, and schema carries a version.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Semantic version for a feature definition or event schema.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FeatureVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl FeatureVersion {
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }

    /// Check if this version is compatible with another (same major).
    #[must_use]
    pub const fn is_compatible_with(self, other: Self) -> bool {
        self.major == other.major
    }
}

impl core::fmt::Display for FeatureVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
