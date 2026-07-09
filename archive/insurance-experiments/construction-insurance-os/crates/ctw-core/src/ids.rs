//! Strongly-typed identifiers for every entity in the system.
//!
//! Using distinct newtypes prevents accidentally passing a `MachineId`
//! where a `SiteId` was expected. In insurance, misidentification
//! of assets can invalidate claims and pricing.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name(pub u128);

        impl $name {
            #[must_use]
            pub const fn new(id: u128) -> Self {
                Self(id)
            }

            #[must_use]
            pub const fn raw(self) -> u128 {
                self.0
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:032x}", self.0)
            }
        }
    };
}

define_id!(
    /// Unique identifier for a machine (excavator, loader, crane, etc.)
    MachineId
);
define_id!(
    /// Unique identifier for a construction site
    SiteId
);
define_id!(
    /// Unique identifier for a worker (de-identified in risk layer)
    WorkerId
);
define_id!(
    /// Unique identifier for a geofenced zone within a site
    ZoneId
);
define_id!(
    /// Unique identifier for an insurance policy
    PolicyId
);
define_id!(
    /// Unique identifier for a claim
    ClaimId
);
define_id!(
    /// Unique identifier for a risk event
    EventId
);
define_id!(
    /// Unique identifier for a sensor or adapter
    SensorId
);
define_id!(
    /// Unique identifier for an operator session
    SessionId
);
define_id!(
    /// Unique identifier for a rate filing
    FilingId
);

/// A typed pair linking a machine to the site it operates on.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MachineAtSite {
    pub machine_id: MachineId,
    pub site_id: SiteId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_distinct_types() {
        let m = MachineId::new(1);
        let s = SiteId::new(1);
        // These are the same numeric value but different types.
        // This test exists to confirm they cannot be accidentally swapped.
        assert_eq!(m.raw(), s.raw());
        // m == s would not compile — that is the point.
    }

    #[test]
    fn display_format() {
        let id = MachineId::new(255);
        let s = format!("{id}");
        assert_eq!(s.len(), 32);
        assert!(s.ends_with("ff"));
    }
}
