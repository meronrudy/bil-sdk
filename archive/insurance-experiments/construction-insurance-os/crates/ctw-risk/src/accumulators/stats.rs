//! Running statistics accumulator (mean, variance, min, max, percentiles).

/// Online statistics accumulator using Welford's algorithm.
#[derive(Clone, Debug)]
pub struct RunningStats {
    count: u64,
    mean: f64,
    m2: f64,
    min: f64,
    max: f64,
}

impl RunningStats {
    pub fn new() -> Self {
        Self { count: 0, mean: 0.0, m2: 0.0, min: f64::INFINITY, max: f64::NEG_INFINITY }
    }

    pub fn push(&mut self, value: f64) {
        self.count += 1;
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = value - self.mean;
        self.m2 += delta * delta2;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }

    pub fn count(&self) -> u64 { self.count }
    pub fn mean(&self) -> f64 { self.mean }
    pub fn min(&self) -> f64 { self.min }
    pub fn max(&self) -> f64 { self.max }

    pub fn variance(&self) -> f64 {
        if self.count < 2 { return 0.0; }
        self.m2 / (self.count - 1) as f64
    }

    pub fn std_dev(&self) -> f64 { self.variance().sqrt() }
}

impl Default for RunningStats {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_stats() {
        let mut s = RunningStats::new();
        for v in [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0] {
            s.push(v);
        }
        assert_eq!(s.count(), 8);
        assert!((s.mean() - 5.0).abs() < 1e-10);
        assert_eq!(s.min(), 2.0);
        assert_eq!(s.max(), 9.0);
    }
}
