//! Distance computations between entities.

use ctw_core::Meters;
use crate::point::Point2;

/// Compute the minimum distance between two sets of points
/// (e.g., machine bounding box vs worker positions).
pub fn min_distance_between_sets(a: &[Point2], b: &[Point2]) -> Meters {
    let mut min_d = f64::INFINITY;
    for pa in a {
        for pb in b {
            let d = pa.distance_to(*pb).raw();
            if d < min_d {
                min_d = d;
            }
        }
    }
    Meters::new(min_d)
}
