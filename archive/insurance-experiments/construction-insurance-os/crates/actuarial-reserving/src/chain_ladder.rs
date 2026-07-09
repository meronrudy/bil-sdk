use serde::{Deserialize, Serialize};
use crate::triangle::LossTriangle;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainLadderResult {
    pub loss_development_factors: Vec<f64>,
    pub cumulative_ldf: f64,
    pub total_incurred: f64,
    pub estimated_ultimate: f64,
    pub ibnr: f64,
    pub ibnr_as_pct: f64,
}

/// Run chain-ladder method on a loss triangle.
pub fn chain_ladder(triangle: &LossTriangle) -> ChainLadderResult {
    let n_dev = triangle.n_development();
    let mut ldfs = Vec::new();

    for d in 0..n_dev.saturating_sub(1) {
        let mut num = 0.0;
        let mut den = 0.0;
        for row in &triangle.cumulative {
            if row.len() > d + 1 {
                num += row[d + 1];
                den += row[d];
            }
        }
        ldfs.push(if den > 0.0 { num / den } else { 1.05 });
    }

    let total_incurred: f64 = triangle.latest_diagonal().iter().sum();
    let cum_ldf: f64 = ldfs.iter().product();
    let ultimate = total_incurred * cum_ldf;
    let ibnr = ultimate - total_incurred;

    ChainLadderResult {
        loss_development_factors: ldfs,
        cumulative_ldf: cum_ldf,
        total_incurred,
        estimated_ultimate: ultimate,
        ibnr,
        ibnr_as_pct: if total_incurred > 0.0 { ibnr / total_incurred * 100.0 } else { 0.0 },
    }
}
