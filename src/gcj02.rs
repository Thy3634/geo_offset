use geo::Coord;
use std::f64::consts::PI;

/**
 * convert a WGS84 coordinate to GCJ02 coordinate
 */
pub fn wgs84_to_gcj02(coordinate: Coord) -> Coord {
    // Check if the coordinate is in China
    if !is_in_china_bbox(coordinate) {
        return coordinate;
    }
    // Convert the coordinate from WGS84 to GCJ02
    let d = delta(coordinate);
    return coordinate + d;
}

/**
 * convert a GCJ02 coordinate to WGS84 coordinate
 * precision: 1e-6, unit: degree
 */
pub fn gcj02_to_wgs84(coordinate: Coord) -> Coord {
    if !is_in_china_bbox(coordinate) {
        return coordinate;
    }

    let mut temp_point = wgs84_to_gcj02(coordinate);
    let mut wgs = coordinate.clone();
    let mut d = temp_point - coordinate;

    while (d.x.abs() > 1e-6) || (d.y.abs() > 1e-6) {
        wgs = wgs - d;

        temp_point = wgs84_to_gcj02(wgs);
        d = temp_point - coordinate;
    }

    wgs
}
/**
 * rough check if the coordinate is in China
 */
fn is_in_china_bbox(coord: Coord) -> bool {
    return coord.x >= 72.004 && coord.x <= 137.8347 && coord.y >= 0.8293 && coord.y <= 55.8271;
}

fn delta(coordinate: Coord) -> Coord {
    let mut d: Coord = (
        transform_lon(coordinate.x - 105.0, coordinate.y - 35.0),
        transform_lat(coordinate.x - 105.0, coordinate.y - 35.0),
    )
        .into();

    let rad_lat = (coordinate.y / 180.0) * PI;
    let mut magic = rad_lat.sin();

    // Krasovsky 1940 ellipsoid
    const A: f64 = 6378245.0;
    const EE: f64 = 0.00669342162296594323; // f = 1/298.3; e^2 = 2*f - f**2

    magic = 1.0 - EE * magic * magic;

    let sqrt_magic = magic.sqrt();
    d.x = (d.x * 180.0) / ((A / sqrt_magic) * rad_lat.cos() * PI);
    d.y = (d.y * 180.0) / (((A * (1.0 - EE)) / (magic * sqrt_magic)) * PI);

    d
}

fn transform_lat(x: f64, y: f64) -> f64 {
    -100.0
        + 2.0 * x
        + 3.0 * y
        + 0.2 * y * y
        + 0.1 * x * y
        + 0.2 * x.abs().sqrt()
        + (2.0 * (x * 6.0 * PI).sin()
            + 2.0 * (x * 2.0 * PI).sin()
            + 2.0 * (y * PI).sin()
            + 4.0 * (y / 3.0 * PI).sin()
            + 16.0 * (y / 12.0 * PI).sin()
            + 32.0 * (y * PI / 30.0).sin())
            * 20.0
            / 3.0
}

fn transform_lon(x: f64, y: f64) -> f64 {
    300.0
        + x
        + 2.0 * y
        + 0.1 * x * x
        + 0.1 * x * y
        + 0.1 * (x).abs().sqrt()
        + (2.0 * (x * 6.0 * PI).sin()
            + 2.0 * (x * 2.0 * PI).sin()
            + 2.0 * (x * PI).sin()
            + 4.0 * (x * PI / 3.0).sin()
            + 15.0 * (x * PI / 12.0).sin()
            + 30.0 * (x * PI / 30.0).sin())
            * 20.0
            / 3.0
}
