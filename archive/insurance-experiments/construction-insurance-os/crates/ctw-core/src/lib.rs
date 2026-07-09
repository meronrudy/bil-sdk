//! # ctw-core
//!
//! Foundational types for the construction insurance operating system.
//! This crate is `no_std` by default so it can run on embedded targets,
//! edge compute, and bare-metal safety controllers.
//!
//! ## Design Principles
//!
//! 1. **Typed units everywhere** — no naked `f32` for physical quantities.
//! 2. **Stable IDs** — 128-bit identifiers that survive system boundaries.
//! 3. **Bounded values** — confidence, fractions, and envelopes that enforce invariants.
//! 4. **Zero allocation** — core types are Copy and stack-allocated.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod ids;
pub mod units;
pub mod time;
pub mod bounded;
pub mod status;
pub mod error;
pub mod version;

// Re-exports for ergonomics
pub use ids::*;
pub use units::*;
pub use time::*;
pub use bounded::*;
pub use status::*;
pub use error::CoreError;
pub use version::FeatureVersion;
