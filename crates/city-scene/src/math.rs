//! Metric geometry kernel for the city street layer.
//!
//! Every derived surface in this crate -- carriageway ribbons, junction boxes,
//! corner sidewalks, lane markings, building shells -- is built from the
//! primitives here, in a single city-local frame: **metres, `+Y` up, `+X`/`+Z`
//! in the ground plane**.  `urban` still speaks kilometres; conversion happens
//! once at the crate boundary through `CityFrameInfo`.
//!
//! The station-space `Path` is the load-bearing type.  Lane geometry, markings
//! and traffic all address a road by *arc length* rather than by vertex index,
//! so one representation serves drawing and simulation and the two can never
//! disagree about where "13 m before the stop line" is.

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use worldgen_core::lerp;

pub const EPSILON: f32 = 1.0e-6;
/// Subdivision step for a surface that has to follow terrain.
///
/// This is deliberately *coarse*.  The terrain mesh the city is draped on has
/// cells tens of metres across, so resampling a road every 12 m produced three to
/// ten times more vertices than the surface could ever resolve — the markings
/// layer alone was generating more geometry than the entire skyline.  Curved
/// elements (connector curves, junction fillets) pass a smaller step explicitly.
pub const MAX_RESAMPLE_METRES: f32 = 34.0;
/// Step for genuinely curved geometry, where the shape itself is the detail.
pub const FINE_RESAMPLE_METRES: f32 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// 2-D cross product; its sign resolves which side of a tangent a point is
    /// on, which is how right-hand-traffic laterality is decided.
    pub fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(self) -> Self {
        let length = self.length();
        if length > EPSILON {
            self / length
        } else {
            Self::ZERO
        }
    }

    /// Perpendicular rotated a quarter turn counter-clockwise.
    pub fn left_normal(self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }

    pub fn clamped(self, limit: f32) -> Self {
        let length = self.length();
        if length > limit && length > EPSILON {
            self / length * limit
        } else {
            self
        }
    }

    /// Angle from `+X`; used for stable port ordering around a junction.
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Ground-plane point plus an explicit elevation.
    pub fn from_plan(point: Vec2, y: f32) -> Self {
        Self::new(point.x, y, point.y)
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Unit vector, falling back to `+Y` for a degenerate vector so a
    /// zero-length segment can never produce a `NaN` normal in a buffer.
    pub fn normalized_or_up(self) -> Self {
        let length = self.length();
        if length > 1.0e-9 {
            self / length
        } else {
            Self::new(0.0, 1.0, 0.0)
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

pub fn clamp(value: f32, low: f32, high: f32) -> f32 {
    value.clamp(low, high)
}

/// A 3-D polyline addressed by arc length, measured in plan.
///
/// Vertices keep the caller's order so `trim` can slice the path at a station
/// and the remainder keeps its own tangents and elevation.
#[derive(Debug, Clone, Default)]
pub struct Path {
    points: Vec<Vec3>,
    /// Cumulative plan arc length at each vertex; `stations[0] == 0`.
    stations: Vec<f32>,
    length: f32,
}

impl Path {
    pub fn new(points: Vec<Vec3>) -> Self {
        let mut stations = Vec::with_capacity(points.len());
        let mut length = 0.0;
        for index in 0..points.len() {
            if index > 0 {
                let a = points[index - 1];
                let b = points[index];
                length += (b.x - a.x).hypot(b.z - a.z);
            }
            stations.push(length);
        }
        Self {
            points,
            stations,
            length,
        }
    }

    pub fn flat(points: Vec<Vec2>) -> Self {
        Self::new(
            points
                .into_iter()
                .map(|p| Vec3::from_plan(p, 0.0))
                .collect(),
        )
    }

    pub fn from_plan(points: impl IntoIterator<Item = [f32; 2]>) -> Self {
        Self::flat(points.into_iter().map(|p| Vec2::new(p[0], p[1])).collect())
    }

    pub fn points(&self) -> &[Vec3] {
        &self.points
    }

    pub fn stations(&self) -> &[f32] {
        &self.stations
    }

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.points.len() < 2 || self.length <= EPSILON
    }

    pub fn start(&self) -> Vec3 {
        self.points.first().copied().unwrap_or_default()
    }

    pub fn end(&self) -> Vec3 {
        self.points.last().copied().unwrap_or_default()
    }

    /// Position and plan unit tangent at an arc length, clamped to both ends.
    /// The tangent points forward along increasing station.
    pub fn sample(&self, station: f32) -> (Vec3, Vec2) {
        if self.points.is_empty() {
            return (Vec3::default(), Vec2::new(1.0, 0.0));
        }
        if self.points.len() == 1 || self.length <= EPSILON {
            return (self.points[0], Vec2::new(1.0, 0.0));
        }
        let station = clamp(station, 0.0, self.length);
        let mut index = 0;
        while index + 2 < self.stations.len() && self.stations[index + 1] < station {
            index += 1;
        }
        let start = self.stations[index];
        let segment = self.stations[index + 1] - start;
        let t = if segment > EPSILON {
            (station - start) / segment
        } else {
            0.0
        };
        let a = self.points[index];
        let b = self.points[index + 1];
        (
            Vec3::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t)),
            Vec2::new(b.x - a.x, b.z - a.z).normalize(),
        )
    }

    /// Plan position at a station, no lateral offset.
    pub fn plan_at(&self, station: f32) -> Vec2 {
        let (position, _) = self.sample(station);
        Vec2::new(position.x, position.z)
    }

    /// Plan unit tangent at a station.
    pub fn tangent_at(&self, station: f32) -> Vec2 {
        self.sample(station).1
    }

    /// 3-D point offset laterally from the centreline.
    ///
    /// `offset` is measured along the left normal -- the same sign convention
    /// the source kernel uses, so ported offsets keep their meaning.  `lift`
    /// raises the point above the roadbed, which is how markings, kerbs and
    /// sidewalks get their layering without z-fighting polygons.
    pub fn offset_at(&self, station: f32, offset: f32, lift: f32) -> Vec3 {
        let (position, tangent) = self.sample(station);
        let lateral = tangent.left_normal() * offset;
        Vec3::new(
            position.x + lateral.x,
            position.y + lift,
            position.z + lateral.y,
        )
    }

    /// Sub-path between two stations, resampled so the result has its own
    /// zero-based station space.  Trim radii at both ends of a road therefore
    /// produce a path whose station 0 is exactly the junction edge.
    pub fn trim(&self, from: f32, to: f32) -> Self {
        let (from, to) = if from <= to { (from, to) } else { (to, from) };
        let from = clamp(from, 0.0, self.length);
        let to = clamp(to, 0.0, self.length);
        let mut points = vec![self.sample(from).0];
        for index in 0..self.points.len() {
            let station = self.stations[index];
            if station > from && station < to {
                points.push(self.points[index]);
            }
        }
        points.push(self.sample(to).0);
        Self::new(points)
    }

    /// Densify a path so no segment exceeds `MAX_RESAMPLE_METRES`.  A surface
    /// built from the densified path can follow terrain without the ribbon
    /// cutting through a hill.
    pub fn densified(&self, max_segment: f32) -> Self {
        if self.points.len() < 2 || max_segment <= EPSILON {
            return self.clone();
        }
        let mut points = Vec::new();
        for index in 0..self.points.len() - 1 {
            let a = self.points[index];
            let b = self.points[index + 1];
            let steps = (((b.x - a.x).hypot(b.z - a.z)) / max_segment)
                .ceil()
                .max(1.0) as usize;
            for step in 0..steps {
                let t = step as f32 / steps as f32;
                points.push(Vec3::new(
                    lerp(a.x, b.x, t),
                    lerp(a.y, b.y, t),
                    lerp(a.z, b.z, t),
                ));
            }
        }
        points.push(*self.points.last().unwrap());
        Self::new(points)
    }

    /// Largest absolute turn between consecutive vertices, in degrees.  The
    /// source pipeline uses this to tell a real junction apart from a shallow
    /// bend that should be smoothed into a single straight street.
    pub fn max_turn_degrees(&self) -> f32 {
        let mut worst: f32 = 0.0;
        if self.points.len() < 3 {
            return 0.0;
        }
        for index in 1..self.points.len() - 1 {
            let a = Vec2::new(
                self.points[index].x - self.points[index - 1].x,
                self.points[index].z - self.points[index - 1].z,
            )
            .normalize();
            let b = Vec2::new(
                self.points[index + 1].x - self.points[index].x,
                self.points[index + 1].z - self.points[index].z,
            )
            .normalize();
            let delta = (b.angle() - a.angle()).abs();
            let wrapped = delta.min(std::f32::consts::TAU - delta);
            worst = worst.max(wrapped.to_degrees());
        }
        worst
    }

    /// Longitudinal grade as a fraction, used for the vertical-clearance
    /// warnings the source pipeline reports instead of silently accepting an
    /// impossible ramp.
    pub fn max_grade(&self) -> f32 {
        let mut worst: f32 = 0.0;
        for index in 1..self.points.len() {
            let a = self.points[index - 1];
            let b = self.points[index];
            let plan = (b.x - a.x).hypot(b.z - a.z);
            if plan > EPSILON {
                worst = worst.max((b.y - a.y).abs() / plan);
            }
        }
        worst
    }
}

/// Cubic Bezier sampling.  Connectors, corner fillets and kerb ramps are all
/// defined this way in the source kernel, so one sampler keeps the ported
/// shapes identical.
pub fn cubic_points(a: Vec2, b: Vec2, c: Vec2, d: Vec2, steps: usize) -> Vec<Vec2> {
    let steps = steps.max(1);
    (0..=steps)
        .map(|index| {
            let t = index as f32 / steps as f32;
            let u = 1.0 - t;
            a * (u * u * u) + b * (3.0 * u * u * t) + c * (3.0 * u * t * t) + d * (t * t * t)
        })
        .collect()
}

/// Fillet between two arms leaving a node.  `handle` is a fraction of the
/// chord, which is how the source kernel keeps a straight-through movement
/// straight while still curving a real turn.
pub fn fillet(from: Vec2, dir_in: Vec2, to: Vec2, dir_out: Vec2, handle: f32) -> Vec<Vec2> {
    let chord = from.distance(to);
    let h = chord * clamp(handle, 0.0, 0.5);
    cubic_points(from, from + dir_in * h, to - dir_out * h, to, 10)
}

/// Signed area of a closed ring; positive is counter-clockwise.
pub fn signed_area(ring: &[Vec2]) -> f32 {
    if ring.len() < 3 {
        return 0.0;
    }
    let mut total = 0.0;
    for index in 0..ring.len() {
        total += ring[index].cross(ring[(index + 1) % ring.len()]);
    }
    total * 0.5
}

pub fn ring_centroid(ring: &[Vec2]) -> Vec2 {
    if ring.is_empty() {
        return Vec2::ZERO;
    }
    let area = signed_area(ring);
    if area.abs() < EPSILON {
        let sum = ring.iter().fold(Vec2::ZERO, |acc, point| acc + *point);
        return sum / ring.len() as f32;
    }
    let mut accumulated = Vec2::ZERO;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        accumulated += (a + b) * a.cross(b);
    }
    accumulated / (6.0 * area)
}

/// Ray-casting containment test.  Used to keep rooftop equipment inside a
/// building ring and street furniture off the carriageway.
pub fn point_in_ring(point: Vec2, ring: &[Vec2]) -> bool {
    if ring.len() < 3 {
        return false;
    }
    let mut inside = false;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        if (a.y > point.y) != (b.y > point.y) {
            let t = (point.y - a.y) / (b.y - a.y);
            if point.x < a.x + t * (b.x - a.x) {
                inside = !inside;
            }
        }
    }
    inside
}

pub fn point_segment_distance(point: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let length_squared = ab.length_squared();
    if length_squared < EPSILON {
        return point.distance(a);
    }
    let t = clamp((point - a).dot(ab) / length_squared, 0.0, 1.0);
    point.distance(a + ab * t)
}

/// Shortest distance from a point to a polyline, used to keep trees, poles and
/// furniture out of the carriageway.
pub fn distance_to_polyline(point: Vec2, points: &[Vec2]) -> f32 {
    match points.len() {
        0 => f32::MAX,
        1 => point.distance(points[0]),
        _ => (0..points.len() - 1)
            .map(|index| point_segment_distance(point, points[index], points[index + 1]))
            .fold(f32::MAX, f32::min),
    }
}

fn cross3(o: Vec2, a: Vec2, b: Vec2) -> f32 {
    (a - o).cross(b - a)
}

fn point_in_triangle(point: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let d1 = cross3(a, b, point);
    let d2 = cross3(b, c, point);
    let d3 = cross3(c, a, point);
    let has_negative = d1 < -EPSILON || d2 < -EPSILON || d3 < -EPSILON;
    let has_positive = d1 > EPSILON || d2 > EPSILON || d3 > EPSILON;
    !(has_negative && has_positive)
}

/// Ear-clipping triangulation for simple polygons (no holes).
///
/// Junction boxes and block ground plates are concave wherever a road cuts a
/// corner out of them, so a triangle fan would produce overlapping garbage.
/// This is the same algorithm behind Three.js `ShapeUtils.triangulateShape`, so
/// the Rust port produces the same faces the source renderer did.
pub fn triangulate(ring: &[Vec2]) -> Vec<[usize; 3]> {
    let count = ring.len();
    let mut triangles = Vec::new();
    if count < 3 {
        return triangles;
    }
    // Work in counter-clockwise order; the output winding is fixed up by the
    // caller, which knows which side of the surface it wants lit.
    let mut indices: Vec<usize> = (0..count).collect();
    if signed_area(ring) < 0.0 {
        indices.reverse();
    }
    let mut guard = count * count + 8;
    while indices.len() > 3 && guard > 0 {
        guard -= 1;
        let mut clipped = false;
        for position in 0..indices.len() {
            let len = indices.len();
            let previous = indices[(position + len - 1) % len];
            let current = indices[position];
            let next = indices[(position + 1) % len];
            let (a, b, c) = (ring[previous], ring[current], ring[next]);
            // Convex corner in a counter-clockwise ring turns left.
            if cross3(a, b, c) <= EPSILON {
                continue;
            }
            if indices.iter().any(|&other| {
                other != previous
                    && other != current
                    && other != next
                    && point_in_triangle(ring[other], a, b, c)
            }) {
                continue;
            }
            triangles.push([previous, current, next]);
            indices.remove(position);
            clipped = true;
            break;
        }
        if !clipped {
            // Degenerate or self-intersecting ring: fall back to a fan so the
            // caller still gets a surface rather than a hole in the road.
            for index in 1..indices.len() - 1 {
                triangles.push([indices[0], indices[index], indices[index + 1]]);
            }
            return triangles;
        }
    }
    if indices.len() == 3 {
        triangles.push([indices[0], indices[1], indices[2]]);
    }
    triangles
}

/// Miter-offset a closed ring inward by `distance`.
/// Miter-offset a closed ring inward by `distance`.
///
/// Parcels, compound walls, lawn plates and setback crowns all use this.  A
/// proper miter is required rather than a per-vertex normal average: averaging
/// collapses the offset distance on the acute corners block faces produce, and
/// the source kernel's half-plane intersection leaves wedges unfilled there.
///
/// The bisector is built from the two adjacent edges' *inward normals*, not
/// their travel directions.  A counter-clockwise ring's averaged directions
/// point away from the interior, which is the classic off-by-180 in this
/// routine and quietly turns every inset into an outset.
pub fn inset_ring(ring: &[Vec2], distance: f32) -> Vec<Vec2> {
    let count = ring.len();
    if count < 3 || distance.abs() <= EPSILON {
        return ring.to_vec();
    }
    // A counter-clockwise ring's interior is on the left of every edge, so a
    // positive offset moves inward; a clockwise ring needs the opposite sign.
    let sign = if signed_area(ring) >= 0.0 { 1.0 } else { -1.0 };
    let mut normals = Vec::with_capacity(count);
    for index in 0..count {
        let direction = (ring[(index + 1) % count] - ring[index]).normalize();
        normals.push(direction.left_normal() * sign);
    }
    let mut result = Vec::with_capacity(count);
    for index in 0..count {
        let previous = normals[(index + count - 1) % count];
        let current = normals[index];
        let sum = previous + current;
        if sum.length_squared() < EPSILON {
            // A 180-degree reversal has no miter; nudge along the edge normal.
            result.push(ring[index] + current * distance);
            continue;
        }
        let bisector = sum.normalize();
        // Miter length: the scalar that keeps the offset edges parallel to the
        // originals.  Clamped so a needle-sharp corner cannot fling a vertex
        // across the city.
        let cos_half = bisector.dot(current);
        let miter = if cos_half.abs() > 0.25 {
            (1.0 / cos_half).min(4.0)
        } else {
            4.0
        };
        result.push(ring[index] + bisector * (distance * miter));
    }
    result
}
/// mulberry32.  A city must be byte-identical for a seed, so every stochastic
/// decision draws from one of these rather than from `rand`.
#[derive(Debug, Clone)]
pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        // Offset the seed so `Rng::new(0)` still produces a usable stream
        // instead of a fixed point.
        Self {
            state: seed ^ 0x9e37_79b9,
        }
    }

    /// Mix an integer into the stream so independent call sites (tree 3 versus
    /// tree 4) never share a correlated prefix.
    pub fn fork(&mut self, salt: u32) {
        self.state =
            self.state.wrapping_mul(0x85eb_ca6b).rotate_left(13) ^ salt.wrapping_mul(0x27d4_eb2f);
        self.state ^= self.state >> 15;
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(0x6d2b_79f5);
        let mut t = self.state;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        t ^ (t >> 14)
    }

    pub fn unit(&mut self) -> f32 {
        self.next_u32() as f32 / (u32::MAX as f32 + 1.0)
    }

    pub fn range(&mut self, low: f32, high: f32) -> f32 {
        low + (high - low) * self.unit()
    }

    pub fn int(&mut self, count: u32) -> u32 {
        if count == 0 {
            0
        } else {
            self.next_u32() % count
        }
    }

    pub fn chance(&mut self, probability: f32) -> bool {
        self.unit() < probability
    }

    pub fn sign(&mut self) -> f32 {
        if self.chance(0.5) { 1.0 } else { -1.0 }
    }

    /// Uniform direction on the unit circle.
    pub fn direction(&mut self) -> Vec2 {
        let angle = self.unit() * std::f32::consts::TAU;
        Vec2::new(angle.cos(), angle.sin())
    }

    /// Uniform direction on the unit sphere, returned as `(x, z)` with `y`
    /// implied by `sqrt(1 - x^2 - z^2)`.  Leaf cards sample this so a canopy is
    /// not biased toward the poles.
    pub fn sphere(&mut self) -> Vec3 {
        let azimuth = self.unit() * std::f32::consts::TAU;
        let cos_polar = 2.0 * self.unit() - 1.0;
        let sin_polar = (1.0 - cos_polar * cos_polar).max(0.0).sqrt();
        Vec3::new(
            sin_polar * azimuth.cos(),
            cos_polar,
            sin_polar * azimuth.sin(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ring_square() -> Vec<Vec2> {
        vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 10.0),
        ]
    }

    #[test]
    fn path_sample_interpolates_position_and_tangent() {
        let path = Path::new(vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(10.0, 0.0, 0.0),
            Vec3::new(10.0, 2.0, 10.0),
        ]);
        assert!((path.length() - 20.0).abs() < 1.0e-4);
        let (position, tangent) = path.sample(5.0);
        assert!((position - Vec3::new(5.0, 0.0, 0.0)).length() < 1.0e-4);
        assert!((tangent - Vec2::new(1.0, 0.0)).length() < 1.0e-4);
        let (position, tangent) = path.sample(15.0);
        assert!((position - Vec3::new(10.0, 1.0, 5.0)).length() < 1.0e-4);
        assert!((tangent - Vec2::new(0.0, 1.0)).length() < 1.0e-4);
    }

    #[test]
    fn offset_at_uses_the_left_normal_so_positive_is_left() {
        let path = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(50.0, 0.0)]);
        let point = path.offset_at(25.0, 2.0, 0.15);
        assert!((point - Vec3::new(25.0, 0.15, 2.0)).length() < 1.0e-4);
    }

    #[test]
    fn trim_rebases_station_space_and_keeps_elevation() {
        let path = Path::new(vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 0.0)]);
        let trimmed = path.trim(20.0, 60.0);
        assert!((trimmed.length() - 40.0).abs() < 1.0e-4);
        let (position, _) = trimmed.sample(0.0);
        assert!((position - Vec3::new(20.0, 0.0, 0.0)).length() < 1.0e-4);
        let (position, _) = trimmed.sample(40.0);
        assert!((position - Vec3::new(60.0, 0.0, 0.0)).length() < 1.0e-4);
    }

    #[test]
    fn triangulate_covers_a_concave_polygon_exactly_once() {
        // An L shape: the outline a junction box or block face takes when a road
        // cuts a corner out of it.
        let ring = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 4.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(4.0, 10.0),
            Vec2::new(0.0, 10.0),
        ];
        let triangles = triangulate(&ring);
        assert_eq!(triangles.len(), 4, "n-2 triangles for a simple n-gon");
        let covered: f32 = triangles
            .iter()
            .map(|[a, b, c]| (ring[*b] - ring[*a]).cross(ring[*c] - ring[*a]).abs() * 0.5)
            .sum();
        assert!((covered - signed_area(&ring).abs()).abs() < 1.0e-3);
    }

    #[test]
    fn inset_ring_offsets_every_edge_by_the_requested_distance() {
        // Distance from a point to the *infinite* line through two points.
        // A finite-segment distance clamps at the end points, and a mitered
        // corner legitimately sits past the end of the edge it belongs to.
        fn line_distance(point: Vec2, a: Vec2, b: Vec2) -> f32 {
            let ab = b - a;
            let length = ab.length();
            assert!(length > EPSILON);
            (point - a).cross(ab).abs() / length
        }
        let ring = ring_square();
        let inset = inset_ring(&ring, 2.0);
        // Each offset edge must be parallel to its original and exactly the
        // requested distance away -- that is what a kerb or a setback needs.
        for index in 0..ring.len() {
            let next = (index + 1) % ring.len();
            let original = ring[next] - ring[index];
            let moved = inset[next] - inset[index];
            assert!(
                original.normalize().cross(moved.normalize()).abs() < 1.0e-3,
                "offset edge {index} is not parallel"
            );
            assert!(
                (line_distance(ring[index], inset[index], inset[next]) - 2.0).abs() < 1.0e-3,
                "offset edge {index} is not 2 m out"
            );
        }
        // 10x10 inset by 2 leaves a 6x6 core.
        assert!((signed_area(&inset).abs() - 36.0).abs() < 1.0e-2);
    }

    #[test]
    fn inset_ring_survives_an_acute_corner() {
        // A spike narrower than the offset distance: the naive normal average
        // flips the corner inside out here.
        let ring = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(12.0, 0.0),
            Vec2::new(1.0, 0.6),
            Vec2::new(12.0, 1.2),
        ];
        let inset = inset_ring(&ring, 1.0);
        assert!(inset.iter().all(|point| point.is_finite()));
    }

    #[test]
    fn max_turn_degrees_detects_a_real_bend() {
        let straight = Path::flat(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(20.0, 0.0),
        ]);
        assert!(straight.max_turn_degrees() < 0.01);
        let bent = Path::flat(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
        ]);
        assert!((bent.max_turn_degrees() - 90.0).abs() < 0.01);
    }

    #[test]
    fn rng_is_reproducible_and_forks_decorrelate() {
        let mut a = Rng::new(7);
        let mut b = Rng::new(7);
        for _ in 0..16 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
        let mut first = Rng::new(7);
        let mut second = Rng::new(7);
        first.fork(3);
        second.fork(11);
        assert_ne!(first.next_u32(), second.next_u32());
    }

    #[test]
    fn sphere_samples_stay_on_the_unit_sphere() {
        let mut rng = Rng::new(11);
        for _ in 0..64 {
            let v = rng.sphere();
            assert!((v.x * v.x + v.y * v.y + v.z * v.z - 1.0).abs() < 1.0e-3);
        }
    }
}
