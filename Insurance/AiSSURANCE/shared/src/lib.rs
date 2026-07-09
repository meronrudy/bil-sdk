//! # Shared Foundation
//!
//! Common building blocks that keep AiSSURANCE safety, autonomy, and risk
//! components speaking the same language.
//!
//! ## Modules
//! - types: Shared data structures for cross-layer workflows
//! - utils: Small helpers that keep implementation details consistent
//!
//! ## Integration
//! Used by all layers for consistent interfaces.

pub mod types;
pub mod utils;

pub use types::*;
pub use utils::*;
