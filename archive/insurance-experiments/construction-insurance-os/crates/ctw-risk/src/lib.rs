//! Risk event detection and accumulation.
//!
//! This crate converts canonical observations into:
//! 1. **Atomic risk events** — "a harsh decel happened at time T"
//! 2. **Accumulated features** — "12 harsh decels per 100 machine-hours over 30 days"
//!
//! The detectors are small state machines. The accumulators are rolling
//! windows. Together they produce the behavioral feature vector X that
//! feeds the actuarial models.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod events;
pub mod detectors;
pub mod accumulators;
pub mod thresholds;
pub mod taxonomy;

pub use events::*;
pub use thresholds::*;
pub use taxonomy::*;
