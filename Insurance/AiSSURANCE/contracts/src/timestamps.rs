//! Runtime and wall-clock timestamps used by cross-crate contracts.

use core::ops::{Add, Sub};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MonotonicMicros(pub u64);

impl MonotonicMicros {
    pub const fn new(micros: u64) -> Self {
        Self(micros)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub const fn add_micros(self, micros: u64) -> Self {
        Self(self.0 + micros)
    }

    pub const fn sub_micros(self, micros: u64) -> Self {
        Self(self.0.saturating_sub(micros))
    }
}

impl Default for MonotonicMicros {
    fn default() -> Self {
        Self(0)
    }
}

impl Add<u64> for MonotonicMicros {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub for MonotonicMicros {
    type Output = i64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as i64 - rhs.0 as i64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SiteTime(pub chrono::DateTime<chrono::Utc>);

impl SiteTime {
    pub fn new(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self(dt)
    }

    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }

    pub fn from_utc(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self(dt)
    }

    pub fn from_timestamp_micros(micros: i64) -> Option<Self> {
        chrono::DateTime::from_timestamp_micros(micros).map(Self)
    }

    pub fn timestamp_micros(self) -> i64 {
        self.0.timestamp_micros()
    }

    pub fn datetime(self) -> chrono::DateTime<chrono::Utc> {
        self.0
    }
}

impl Default for SiteTime {
    fn default() -> Self {
        Self(chrono::DateTime::UNIX_EPOCH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotonic_arithmetic_is_saturating() {
        let t = MonotonicMicros::new(100);
        assert_eq!(t.sub_micros(200), MonotonicMicros::new(0));
    }
}
