//! Scalar helpers shared by every crate, generic over `f32` and `f64`.
//!
//! The legacy layers work in `f32` and the world layers in `f64`; both used to
//! carry private copies of these. There is one definition of each curve here, and
//! the names say which curve it is (`smoothstep` is the cubic, `smootherstep` the
//! quintic).

use std::ops::{Add, Div, Mul, Sub};

/// A floating-point scalar: `f32` or `f64`.
pub trait Real:
    Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + From<f32>
{
}
impl Real for f32 {}
impl Real for f64 {}

/// `a` at `t = 0`, `b` at `t = 1`; `t` is not clamped.
#[inline]
pub fn lerp<T: Real>(a: T, b: T, t: T) -> T {
    a + (b - a) * t
}

/// `x` limited to `[0, 1]`. NaN passes through.
#[inline]
pub fn clamp01<T: Real>(x: T) -> T {
    if x < T::from(0.0) {
        T::from(0.0)
    } else if x > T::from(1.0) {
        T::from(1.0)
    } else {
        x
    }
}

/// Where `x` lies between `a` and `b`, as a fraction; not clamped.
#[inline]
pub fn inv_lerp<T: Real>(a: T, b: T, x: T) -> T {
    (x - a) / (b - a)
}

/// The cubic Hermite step `3t² − 2t³` of `t` clamped to `[0, 1]`: zero slope at
/// both ends.
#[inline]
pub fn smooth01<T: Real>(t: T) -> T {
    let t = clamp01(t);
    t * t * (T::from(3.0) - T::from(2.0) * t)
}

/// [`smooth01`] of where `x` lies between `edge0` and `edge1`.
#[inline]
pub fn smoothstep<T: Real>(edge0: T, edge1: T, x: T) -> T {
    smooth01(inv_lerp(edge0, edge1, x))
}

/// The quintic step `6t⁵ − 15t⁴ + 10t³` of `t` clamped to `[0, 1]`: zero slope
/// *and* zero curvature at both ends, which is what lattice noise needs to have
/// no visible grid.
#[inline]
pub fn smootherstep<T: Real>(t: T) -> T {
    let t = clamp01(t);
    t * t * t * (T::from(10.0) + t * (T::from(-15.0) + T::from(6.0) * t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_curves_hit_their_endpoints_and_midpoint() {
        for f in [smooth01::<f32>, smootherstep::<f32>] {
            assert_eq!(f(-1.0), 0.0);
            assert_eq!(f(0.0), 0.0);
            assert_eq!(f(1.0), 1.0);
            assert_eq!(f(2.0), 1.0);
            assert!((f(0.5) - 0.5).abs() < 1.0e-6);
        }
        assert_eq!(smoothstep(2.0_f64, 4.0, 3.0), 0.5);
        assert_eq!(smoothstep(2.0_f64, 4.0, 9.0), 1.0);
    }

    #[test]
    fn lerp_and_clamp_work_for_both_widths() {
        assert_eq!(lerp(2.0_f32, 4.0, 0.5), 3.0);
        assert_eq!(lerp(2.0_f64, 4.0, 1.5), 5.0);
        assert_eq!(clamp01(-0.5_f32), 0.0);
        assert_eq!(clamp01(1.5_f64), 1.0);
        assert!(clamp01(f32::NAN).is_nan());
        assert_eq!(inv_lerp(10.0_f32, 20.0, 15.0), 0.5);
    }
}
