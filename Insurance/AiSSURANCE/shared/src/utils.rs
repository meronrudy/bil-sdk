//! # Shared Utilities
//!
//! Practical helpers shared across the AiSSURANCE platform. Keep cross-layer
//! behavior consistent without burying product logic in duplicate glue code.

use crate::Position;

pub fn distance(a: &Position, b: &Position) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

pub fn timestamp_now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}
