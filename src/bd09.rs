use geo::Coord;
use std::f64::consts::PI;

use crate::gcj02::{gcj02_to_wgs84, wgs84_to_gcj02};

pub fn wgs84_to_bd09(coordinate: Coord) -> Coord {
    gcj02_to_bd09(wgs84_to_gcj02(coordinate))
}

pub fn bd09_to_wgs84(coordinate: Coord) -> Coord {
    gcj02_to_wgs84(bd09_to_gcj02(coordinate))
}

/// Baidu's artificial deviations
const BD_DLAT: f64 = 0.0060;
const BD_DLON: f64 = 0.0065;

pub fn gcj02_to_bd09(coordinate: Coord) -> Coord {
    let x = coordinate.x;
    let y = coordinate.y;

    let r = (x * x + y * y).sqrt() + 0.00002 * (y * PI * 3000.0 / 180.0).sin();
    let theta = (y / x).atan() + 0.000003 * (x * PI * 3000.0 / 180.0).cos();

    (r * theta.cos() + BD_DLON, r * theta.sin() + BD_DLAT).into()
}

pub fn bd09_to_gcj02(coordinate: Coord) -> Coord {
    let x = coordinate.x - BD_DLON;
    let y = coordinate.y - BD_DLAT;

    let r = (x * x + y * y).sqrt() - 0.00002 * (y * PI * 3000.0 / 180.0).sin();
    let theta = y.atan2(x) - 0.000003 * (x * PI * 3000.0 / 180.0).cos();

    (r * theta.cos(), r * theta.sin()).into()
}
