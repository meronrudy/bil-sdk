use serde::{Deserialize, Serialize};

/// A loss development triangle.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LossTriangle {
    /// Origin period labels (e.g., accident months).
    pub origins: Vec<String>,
    /// Cumulative values: origins[i][j] = cumulative at development j.
    pub cumulative: Vec<Vec<f64>>,
}

impl LossTriangle {
    pub fn n_origins(&self) -> usize { self.origins.len() }

    pub fn n_development(&self) -> usize {
        self.cumulative.first().map_or(0, |r| r.len())
    }

    /// Latest diagonal values.
    pub fn latest_diagonal(&self) -> Vec<f64> {
        self.cumulative.iter().map(|row| {
            row.last().copied().unwrap_or(0.0)
        }).collect()
    }
}
