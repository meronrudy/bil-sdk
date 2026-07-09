//! Sensor data ingestion: raw bytes → canonical observations.
//!
//! This crate defines the typed membrane between hardware and the rest
//! of the system. Raw CAN frames, IMU packets, GNSS fixes, and
//! vision detections all enter here and emerge as canonical observations
//! that the risk layer can consume.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod frame;
pub mod observation;
pub mod adapter;
pub mod normalize;

pub use frame::*;
pub use observation::*;
pub use adapter::*;
