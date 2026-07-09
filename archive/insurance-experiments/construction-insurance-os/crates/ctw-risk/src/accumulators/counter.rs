//! Event counter accumulator.

use crate::events::RiskEventType;

/// Counts events by type within a window.
#[derive(Clone, Debug, Default)]
pub struct EventCounter {
    counts: alloc::collections::BTreeMap<RiskEventType, u64>,
    total: u64,
}

extern crate alloc;

// RiskEventType needs Ord for BTreeMap
impl Eq for RiskEventType {}
impl PartialOrd for RiskEventType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RiskEventType {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl EventCounter {
    pub fn new() -> Self { Self::default() }

    pub fn record(&mut self, event_type: RiskEventType) {
        *self.counts.entry(event_type).or_insert(0) += 1;
        self.total += 1;
    }

    pub fn count(&self, event_type: RiskEventType) -> u64 {
        self.counts.get(&event_type).copied().unwrap_or(0)
    }

    pub fn total(&self) -> u64 { self.total }

    /// Rate per given denominator (e.g., per 100 machine-hours).
    pub fn rate_per(&self, event_type: RiskEventType, denominator: f64) -> f64 {
        if denominator <= 0.0 { return 0.0; }
        self.count(event_type) as f64 / denominator
    }

    pub fn reset(&mut self) {
        self.counts.clear();
        self.total = 0;
    }
}
