//! Polygon operations for geofences and exclusion zones.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ctw_core::Meters;
use crate::point::Point2;

/// A closed polygon defined by its vertices.
/// Vertices are assumed to be in order (clockwise or counter-clockwise).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Polygon {
    pub vertices: alloc::vec::Vec<Point2>,
}

extern crate alloc;

impl Polygon {
    /// Create a polygon from vertices.
    #[must_use]
    pub fn new(vertices: alloc::vec::Vec<Point2>) -> Self {
        Self { vertices }
    }

    /// Test if a point is inside the polygon using ray casting.
    #[must_use]
    pub fn contains(&self, point: &Point2) -> bool {
        let n = self.vertices.len();
        if n < 3 {
            return false;
        }
        let mut inside = false;
        let mut j = n - 1;
        for i in 0..n {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];
            if ((vi.y.raw() > point.y.raw()) != (vj.y.raw() > point.y.raw()))
                && (point.x.raw()
                    < (vj.x.raw() - vi.x.raw()) * (point.y.raw() - vi.y.raw())
                        / (vj.y.raw() - vi.y.raw())
                        + vi.x.raw())
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    /// Minimum distance from a point to the polygon boundary.
    /// Negative if inside (signed distance).
    #[must_use]
    pub fn signed_distance(&self, point: &Point2) -> Meters {
        let n = self.vertices.len();
        if n < 2 {
            return Meters::new(f64::INFINITY);
        }

        let mut min_dist = f64::INFINITY;
        for i in 0..n {
            let j = (i + 1) % n;
            let d = point_to_segment_distance(
                point,
                &self.vertices[i],
                &self.vertices[j],
            );
            if d < min_dist {
                min_dist = d;
            }
        }

        let sign = if self.contains(point) { -1.0 } else { 1.0 };
        Meters::new(sign * min_dist)
    }
}

/// Distance from a point to a line segment.
fn point_to_segment_distance(p: &Point2, a: &Point2, b: &Point2) -> f64 {
    let dx = b.x.raw() - a.x.raw();
    let dy = b.y.raw() - a.y.raw();
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-12 {
        return p.distance_to(*a).raw();
    }

    let t = ((p.x.raw() - a.x.raw()) * dx + (p.y.raw() - a.y.raw()) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj = Point2::new(
        Meters::new(a.x.raw() + t * dx),
        Meters::new(a.y.raw() + t * dy),
    );
    p.distance_to(proj).raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Polygon {
        Polygon::new(alloc::vec![
            Point2::new(Meters::new(0.0), Meters::new(0.0)),
            Point2::new(Meters::new(10.0), Meters::new(0.0)),
            Point2::new(Meters::new(10.0), Meters::new(10.0)),
            Point2::new(Meters::new(0.0), Meters::new(10.0)),
        ])
    }

    #[test]
    fn point_inside() {
        let p = Point2::new(Meters::new(5.0), Meters::new(5.0));
        assert!(square().contains(&p));
    }

    #[test]
    fn point_outside() {
        let p = Point2::new(Meters::new(15.0), Meters::new(5.0));
        assert!(!square().contains(&p));
    }

    #[test]
    fn signed_distance_inside_is_negative() {
        let p = Point2::new(Meters::new(5.0), Meters::new(5.0));
        assert!(square().signed_distance(&p).raw() < 0.0);
    }

    #[test]
    fn signed_distance_outside_is_positive() {
        let p = Point2::new(Meters::new(12.0), Meters::new(5.0));
        assert!(square().signed_distance(&p).raw() > 0.0);
    }
}
