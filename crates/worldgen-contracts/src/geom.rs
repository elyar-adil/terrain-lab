//! Plane geometry in metres. Small on purpose: only what layers exchange.

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

pub const fn v2(x: f64, y: f64) -> V2 {
    V2 { x, y }
}

impl V2 {
    pub const ZERO: V2 = v2(0.0, 0.0);

    pub fn dot(self, o: V2) -> f64 {
        self.x * o.x + self.y * o.y
    }
    /// z component of the 3-D cross product; positive when `o` is anticlockwise of `self`.
    pub fn cross(self, o: V2) -> f64 {
        self.x * o.y - self.y * o.x
    }
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn dist(self, o: V2) -> f64 {
        (self - o).len()
    }
    /// Unit vector; the zero vector stays zero rather than becoming NaN.
    pub fn norm(self) -> V2 {
        let l = self.len();
        if l < 1e-12 {
            V2::ZERO
        } else {
            self * (1.0 / l)
        }
    }
    /// Anticlockwise quarter turn.
    pub fn perp(self) -> V2 {
        v2(-self.y, self.x)
    }
    pub fn lerp(self, o: V2, t: f64) -> V2 {
        self + (o - self) * t
    }
    pub fn rotate(self, radians: f64) -> V2 {
        let (s, c) = radians.sin_cos();
        v2(self.x * c - self.y * s, self.x * s + self.y * c)
    }
    pub fn from_angle(radians: f64) -> V2 {
        let (s, c) = radians.sin_cos();
        v2(c, s)
    }
    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }
    pub fn to_array(self) -> [f64; 2] {
        [self.x, self.y]
    }
    pub fn from_array(a: [f64; 2]) -> V2 {
        v2(a[0], a[1])
    }
}

impl Add for V2 {
    type Output = V2;
    fn add(self, o: V2) -> V2 {
        v2(self.x + o.x, self.y + o.y)
    }
}
impl AddAssign for V2 {
    fn add_assign(&mut self, o: V2) {
        *self = *self + o;
    }
}
impl Sub for V2 {
    type Output = V2;
    fn sub(self, o: V2) -> V2 {
        v2(self.x - o.x, self.y - o.y)
    }
}
impl Mul<f64> for V2 {
    type Output = V2;
    fn mul(self, k: f64) -> V2 {
        v2(self.x * k, self.y * k)
    }
}
impl Neg for V2 {
    type Output = V2;
    fn neg(self) -> V2 {
        v2(-self.x, -self.y)
    }
}

/// Closest point on segment `ab` to `p`: (point, parameter in 0..=1, distance).
pub fn closest_on_segment(a: V2, b: V2, p: V2) -> (V2, f64, f64) {
    let d = b - a;
    let l2 = d.dot(d);
    let t = if l2 < 1e-18 {
        0.0
    } else {
        ((p - a).dot(d) / l2).clamp(0.0, 1.0)
    };
    let q = a + d * t;
    (q, t, q.dist(p))
}

/// Proper intersection of two segments, if any: the point and both parameters.
/// Touching at an endpoint counts; parallel or collinear segments do not.
pub fn segment_intersection(a: V2, b: V2, c: V2, d: V2) -> Option<(V2, f64, f64)> {
    let r = b - a;
    let s = d - c;
    let denom = r.cross(s);
    if denom.abs() < 1e-12 {
        return None;
    }
    let t = (c - a).cross(s) / denom;
    let u = (c - a).cross(r) / denom;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some((a + r * t, t, u))
    } else {
        None
    }
}

/// An open chain of points.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Polyline(pub Vec<V2>);

impl Polyline {
    pub fn new(points: Vec<V2>) -> Polyline {
        Polyline(points)
    }
    pub fn points(&self) -> &[V2] {
        &self.0
    }
    pub fn first(&self) -> Option<V2> {
        self.0.first().copied()
    }
    pub fn last(&self) -> Option<V2> {
        self.0.last().copied()
    }
    pub fn length(&self) -> f64 {
        self.0.windows(2).map(|s| s[0].dist(s[1])).sum()
    }
    pub fn reversed(&self) -> Polyline {
        Polyline(self.0.iter().rev().copied().collect())
    }
    /// Point and unit tangent at arc length `d` from the start (clamped to the ends).
    pub fn at(&self, d: f64) -> Option<(V2, V2)> {
        let n = self.0.len();
        if n < 2 {
            return self.0.first().map(|p| (*p, v2(1.0, 0.0)));
        }
        let mut left = d.max(0.0);
        for s in self.0.windows(2) {
            let l = s[0].dist(s[1]);
            if left <= l || std::ptr::eq(&s[1], &self.0[n - 1]) {
                let t = if l < 1e-12 { 0.0 } else { (left / l).min(1.0) };
                return Some((s[0].lerp(s[1], t), (s[1] - s[0]).norm()));
            }
            left -= l;
        }
        None
    }
    /// Distance from `p` to the line and the arc length of the closest point.
    pub fn closest(&self, p: V2) -> Option<(f64, f64)> {
        let mut best: Option<(f64, f64)> = None;
        let mut run = 0.0;
        for s in self.0.windows(2) {
            let (_, t, d) = closest_on_segment(s[0], s[1], p);
            let l = s[0].dist(s[1]);
            if best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, run + t * l));
            }
            run += l;
        }
        if best.is_none() && self.0.len() == 1 {
            best = Some((self.0[0].dist(p), 0.0));
        }
        best
    }
    /// Points at roughly even spacing, keeping the end points.
    pub fn resampled(&self, step: f64) -> Polyline {
        let total = self.length();
        if self.0.len() < 2 || total < 1e-9 {
            return self.clone();
        }
        let n = (total / step).ceil().max(1.0) as usize;
        let mut out = Vec::with_capacity(n + 1);
        for i in 0..=n {
            out.push(self.at(total * i as f64 / n as f64).unwrap().0);
        }
        Polyline(out)
    }
    pub fn bounds(&self) -> Option<(V2, V2)> {
        let first = *self.0.first()?;
        Some(self.0.iter().fold((first, first), |(lo, hi), p| {
            (
                v2(lo.x.min(p.x), lo.y.min(p.y)),
                v2(hi.x.max(p.x), hi.y.max(p.y)),
            )
        }))
    }
    /// The largest turn between consecutive segments, in radians (0 for straight).
    pub fn max_turn(&self) -> f64 {
        self.0
            .windows(3)
            .map(|w| {
                let (a, b) = (w[1] - w[0], w[2] - w[1]);
                a.cross(b).atan2(a.dot(b)).abs()
            })
            .fold(0.0, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectors_behave() {
        let a = v2(3.0, 4.0);
        assert_eq!(a.len(), 5.0);
        assert!((a.norm().len() - 1.0).abs() < 1e-12);
        assert_eq!(V2::ZERO.norm(), V2::ZERO);
        assert_eq!(v2(1.0, 0.0).perp(), v2(0.0, 1.0));
        assert!(v2(1.0, 0.0).cross(v2(0.0, 1.0)) > 0.0);
        assert!((v2(1.0, 0.0).rotate(std::f64::consts::FRAC_PI_2) - v2(0.0, 1.0)).len() < 1e-12);
    }

    #[test]
    fn segments_intersect_where_they_cross_and_not_when_parallel() {
        let hit = segment_intersection(v2(0.0, 0.0), v2(10.0, 10.0), v2(0.0, 10.0), v2(10.0, 0.0))
            .unwrap();
        assert!((hit.0 - v2(5.0, 5.0)).len() < 1e-12);
        assert!(
            segment_intersection(v2(0.0, 0.0), v2(1.0, 0.0), v2(0.0, 1.0), v2(1.0, 1.0)).is_none()
        );
        assert!(
            segment_intersection(v2(0.0, 0.0), v2(1.0, 0.0), v2(2.0, -1.0), v2(2.0, 1.0)).is_none()
        );
    }

    #[test]
    fn a_polyline_reports_length_position_and_closest_point() {
        let l = Polyline::new(vec![v2(0.0, 0.0), v2(10.0, 0.0), v2(10.0, 10.0)]);
        assert_eq!(l.length(), 20.0);
        let (p, t) = l.at(15.0).unwrap();
        assert!((p - v2(10.0, 5.0)).len() < 1e-12 && (t - v2(0.0, 1.0)).len() < 1e-12);
        assert_eq!(l.at(99.0).unwrap().0, v2(10.0, 10.0));
        let (d, along) = l.closest(v2(4.0, 3.0)).unwrap();
        assert!((d - 3.0).abs() < 1e-12 && (along - 4.0).abs() < 1e-12);
        let r = l.resampled(5.0);
        assert_eq!(r.0.len(), 5);
        assert!((r.length() - 20.0).abs() < 1e-9);
        assert!((l.max_turn() - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
        assert_eq!(l.reversed().first(), l.last());
    }
}
