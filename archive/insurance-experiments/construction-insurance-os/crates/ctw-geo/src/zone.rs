//! Site zones: geofences, exclusion areas, hazard zones.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::ZoneId;
use crate::polygon::Polygon;

/// Type of zone on a construction site.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ZoneType {
    /// Hard boundary — must not cross.
    Geofence,
    /// Worker-only zone — machines excluded.
    WorkerExclusion,
    /// Machine operating envelope.
    OperatingEnvelope,
    /// Known hazard area (trench edge, drop-off, overhead lines).
    HazardArea,
    /// Material storage / staging.
    StagingArea,
    /// Active work zone — heightened risk.
    ActiveWorkZone,
    /// Ingress/egress corridor.
    AccessCorridor,
}

/// A zone on the construction site.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Zone {
    pub id: ZoneId,
    pub zone_type: ZoneType,
    pub boundary: Polygon,
    pub name: alloc::string::String,
    /// Whether this zone is currently active.
    pub active: bool,
}

extern crate alloc;
