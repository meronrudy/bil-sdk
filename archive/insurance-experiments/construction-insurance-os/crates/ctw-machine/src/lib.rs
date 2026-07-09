//! Machine abstractions: capability traits, not OEM implementations.
//!
//! We abstract by observable capability, not by manufacturer.
//! An excavator is defined by what it can report (boom angle, swing rate,
//! bucket pose), not by whether it's a Cat, Komatsu, or Volvo.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod class;
pub mod capabilities;
pub mod health;
pub mod attachment;

pub use class::*;
pub use capabilities::*;
pub use health::*;
pub use attachment::*;
