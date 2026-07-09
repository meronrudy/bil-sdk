//! # CIOS Simulation Library
//!
//! Deterministic data generators for construction insurance pipeline simulation.
//!
//! This crate provides seeded generators for all layers of the CIOS system:
//! fleets, sites, telemetry, risk events, exposure, policies, claims, triangles,
//! and feature matrices. All generators produce domain-valid data and are
//! deterministic with explicit seeds.

pub mod config;
pub mod fleet;
pub mod sites;
pub mod telemetry;
pub mod risk_events;
pub mod exposure;
pub mod policies;
pub mod claims;
pub mod triangles;
pub mod features;
pub mod scenarios;

// Re-exports for convenience
pub use config::{SimConfig, ScenarioProfile};
pub use scenarios::ScenarioBundle;

/// Machine capability enumeration for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Capability {
    MotionSource,
    PoseSource,
    ExcavatorKinematics,
    CraneKinematics,
    HealthSource,
}

/// Simplified machine representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub id: String,
    pub spec: ctw_machine::MachineSpec,
    pub capabilities: Vec<Capability>,
}

pub use fleet::generate_fleet;

/// Machine capability enumeration for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Capability {
    MotionSource,
    PoseSource,
    ExcavatorKinematics,
    CraneKinematics,
    HealthSource,
}

/// Simplified machine representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub id: String,
    pub spec: ctw_machine::MachineSpec,
    pub capabilities: Vec<Capability>,
}

pub use fleet::generate_fleet;

/// Machine capability enumeration for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Capability {
    MotionSource,
    PoseSource,
    ExcavatorKinematics,
    CraneKinematics,
    HealthSource,
}

/// Simplified machine representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub id: String,
    pub spec: ctw_machine::MachineSpec,
    pub capabilities: Vec<Capability>,
}

pub use fleet::generate_fleet;

/// Machine capability enumeration for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Capability {
    MotionSource,
    PoseSource,
    ExcavatorKinematics,
    CraneKinematics,
    HealthSource,
}

/// Simplified machine representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub id: String,
    pub spec: ctw_machine::MachineSpec,
    pub capabilities: Vec<Capability>,
}

pub use fleet::generate_fleet;

use ctw_geo::{Point, Polygon, Zone};

/// Simplified site representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub id: String,
    pub location: Point,
    pub boundary: Polygon,
    pub zones: Vec<Zone>,
}

pub use sites::generate_sites;

use ctw_geo::{Point, Polygon, Zone};

/// Simplified site representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub id: String,
    pub location: Point,
    pub boundary: Polygon,
    pub zones: Vec<Zone>,
}

pub use sites::generate_sites;

/// Machine capability enumeration for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Capability {
    MotionSource,
    PoseSource,
    ExcavatorKinematics,
    CraneKinematics,
    HealthSource,
}

/// Simplified machine representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub id: String,
    pub spec: ctw_machine::MachineSpec,
    pub capabilities: Vec<Capability>,
}

pub use fleet::generate_fleet;

use ctw_geo::{Point, Polygon, Zone};

/// Simplified site representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub id: String,
    pub location: Point,
    pub boundary: Polygon,
    pub zones: Vec<Zone>,
}

pub use sites::generate_sites;

use ctw_geo::{Point, Polygon, Zone};

/// Simplified site representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub id: String,
    pub location: Point,
    pub boundary: Polygon,
    pub zones: Vec<Zone>,
}

pub use sites::generate_sites;

use ctw_geo::{Point, Polygon, Zone};

/// Simplified site representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Site {
    pub id: String,
    pub location: Point,
    pub boundary: Polygon,
    pub zones: Vec<Zone>,
}

pub use sites::generate_sites;

/// Machine capability enumeration for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Capability {
    MotionSource,
    PoseSource,
    ExcavatorKinematics,
    CraneKinematics,
    HealthSource,
}

/// Simplified machine representation for simulation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub id: String,
    pub spec: ctw_machine::MachineSpec,
    pub capabilities: Vec<Capability>,
}

pub use fleet::generate_fleet;