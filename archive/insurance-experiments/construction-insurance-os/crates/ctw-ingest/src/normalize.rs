//! Normalization utilities for sensor data.

use ctw_core::Meters;
use ctw_geo::Point2;

/// Convert WGS84 lat/lon to local East-North-Up meters from a reference point.
///
/// Uses a simple equirectangular approximation suitable for construction
/// sites (< 5km extent). For larger sites, use a proper UTM projection.
pub fn wgs84_to_local(
    lat: f64,
    lon: f64,
    ref_lat: f64,
    ref_lon: f64,
) -> Point2 {
    let earth_radius = 6_371_000.0; // meters
    let lat_rad = lat.to_radians();
    let ref_lat_rad = ref_lat.to_radians();

    let x = (lon - ref_lon).to_radians() * earth_radius * ref_lat_rad.cos();
    let y = (lat - ref_lat).to_radians() * earth_radius;

    Point2::new(Meters::new(x), Meters::new(y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_point_gives_origin() {
        let p = wgs84_to_local(40.0, -74.0, 40.0, -74.0);
        assert!(p.x.raw().abs() < 0.01);
        assert!(p.y.raw().abs() < 0.01);
    }
}
