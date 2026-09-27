use crate::Point;

pub(crate) fn centroid(points: &[Point]) -> Point {
    let n = points.len().max(1) as f32;
    Point {
        x_km: points.iter().map(|p| p.x_km).sum::<f32>() / n,
        y_km: points.iter().map(|p| p.y_km).sum::<f32>() / n,
    }
}

pub(crate) fn scale_polygon(points: &[Point], centre: Point, scale: f32) -> Vec<Point> {
    points
        .iter()
        .map(|p| Point {
            x_km: centre.x_km + (p.x_km - centre.x_km) * scale,
            y_km: centre.y_km + (p.y_km - centre.y_km) * scale,
        })
        .collect()
}

pub(crate) fn hash01(seed: u32, x: i32, y: i32) -> f32 {
    let mut value =
        seed ^ (x as u32).wrapping_mul(0x9e37_79b9) ^ (y as u32).wrapping_mul(0x85eb_ca6b);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value as f32 / u32::MAX as f32
}
