//! Equipment classification for rating and exposure grouping.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Primary equipment class. Determines base rate group in pricing.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MachineClass {
    Excavator,
    WheelLoader,
    HaulTruck,
    ArticulatedDumpTruck,
    CrawlerCrane,
    TowerCrane,
    MobileCrane,
    Dozer,
    Grader,
    CompactTrackLoader,
    SkidSteerLoader,
    Backhoe,
    Telehandler,
    RollerCompactor,
    Paver,
    DrillingRig,
    PileDrivingRig,
    ConcretePump,
    ConcreteMixer,
    AerialWorkPlatform,
}

/// Weight class bracket (affects base rate and exposure).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WeightClass {
    /// < 6 tonnes operating weight
    Mini,
    /// 6–15 tonnes
    Light,
    /// 15–30 tonnes
    Medium,
    /// 30–50 tonnes
    Heavy,
    /// 50–80 tonnes
    ExtraHeavy,
    /// > 80 tonnes
    SuperHeavy,
}

/// Complete machine specification for insurance purposes.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MachineSpec {
    pub class: MachineClass,
    pub weight_class: WeightClass,
    /// Operating weight in kg.
    pub operating_weight_kg: f64,
    /// Maximum rated capacity (context-dependent: lift, bucket, payload).
    pub max_rated_capacity_kg: f64,
    /// Year of manufacture.
    pub year: u16,
    /// OEM make (for reference, not for pricing discrimination).
    pub make: alloc::string::String,
    /// OEM model.
    pub model: alloc::string::String,
}

extern crate alloc;
