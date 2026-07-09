//! Timestamps for the construction operating system.
//!
//! Two time representations:
//! - `MonotonicMicros`: monotonic clock, for measuring durations and ordering events.
//!   Never goes backward, immune to NTP adjustments. Used in safety-critical paths.
//! - `WallClockUtc`: wall-clock time for human reporting, claims, and filings.
//!   Subject to clock drift and adjustments. Used in actuarial records.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Monotonic timestamp in microseconds since an arbitrary epoch.
/// Used for all real-time and safety-critical ordering.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MonotonicMicros(pub u64);

impl MonotonicMicros {
    #[must_use]
    pub const fn new(us: u64) -> Self {
        Self(us)
    }

    #[must_use]
    pub const fn as_micros(self) -> u64 {
        self.0
    }

    #[must_use]
    pub fn as_seconds_f64(self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    /// Duration between two monotonic timestamps.
    #[must_use]
    pub fn duration_since(self, earlier: Self) -> Option<MonotonicMicros> {
        self.0.checked_sub(earlier.0).map(MonotonicMicros)
    }

    /// Add microseconds.
    #[must_use]
    pub fn add_micros(self, us: u64) -> Self {
        Self(self.0.saturating_add(us))
    }
}

/// Wall-clock timestamp as Unix epoch seconds + microsecond fraction.
/// Used for actuarial records, claims, filings — anything that needs a date.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WallClockUtc {
    /// Seconds since Unix epoch (1970-01-01T00:00:00Z).
    pub epoch_secs: i64,
    /// Microsecond fraction within the second (0..999_999).
    pub micro_frac: u32,
}

impl WallClockUtc {
    #[must_use]
    pub const fn from_epoch_secs(secs: i64) -> Self {
        Self {
            epoch_secs: secs,
            micro_frac: 0,
        }
    }

    #[must_use]
    pub const fn from_epoch_micros(us: i64) -> Self {
        Self {
            epoch_secs: us / 1_000_000,
            micro_frac: (us % 1_000_000) as u32,
        }
    }

    #[must_use]
    pub const fn as_epoch_micros(self) -> i64 {
        self.epoch_secs * 1_000_000 + self.micro_frac as i64
    }
}

/// A time window for aggregation (e.g., "last 30 days").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimeWindow {
    pub start: MonotonicMicros,
    pub end: MonotonicMicros,
}

impl TimeWindow {
    #[must_use]
    pub fn duration_micros(&self) -> u64 {
        self.end.0.saturating_sub(self.start.0)
    }

    #[must_use]
    pub fn contains(&self, ts: MonotonicMicros) -> bool {
        ts >= self.start && ts <= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotonic_duration() {
        let a = MonotonicMicros::new(1_000_000);
        let b = MonotonicMicros::new(3_500_000);
        let d = b.duration_since(a).unwrap();
        assert_eq!(d.as_micros(), 2_500_000);
        assert!((d.as_seconds_f64() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn wall_clock_roundtrip() {
        let w = WallClockUtc::from_epoch_micros(1_700_000_000_123_456);
        assert_eq!(w.epoch_secs, 1_700_000_000);
        assert_eq!(w.micro_frac, 123_456);
        assert_eq!(w.as_epoch_micros(), 1_700_000_000_123_456);
    }

    #[test]
    fn time_window_contains() {
        let w = TimeWindow {
            start: MonotonicMicros::new(100),
            end: MonotonicMicros::new(200),
        };
        assert!(w.contains(MonotonicMicros::new(150)));
        assert!(!w.contains(MonotonicMicros::new(50)));
        assert!(!w.contains(MonotonicMicros::new(250)));
    }
}
