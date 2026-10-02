//! Smooth noise from the lattice hash: a pure function of position and seed.
//!
//! Nothing here keeps state, so the value at a point is the same whichever tile
//! asked for it, which is what lets a field vary smoothly across a seam.

use crate::hash::{hash_words, to_unit};
use crate::seed::Seed;

fn lattice(seed: Seed, ix: i64, iy: i64) -> f64 {
    to_unit(hash_words(&[seed.0, ix as u64, iy as u64, 0x401E]))
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Value noise in `[0, 1)`; features are about one unit across.
pub fn value(seed: Seed, x: f64, y: f64) -> f64 {
    let (fx, fy) = (x.floor(), y.floor());
    let (ix, iy) = (fx as i64, fy as i64);
    let (tx, ty) = (fade(x - fx), fade(y - fy));
    let a = lattice(seed, ix, iy);
    let b = lattice(seed, ix + 1, iy);
    let c = lattice(seed, ix, iy + 1);
    let d = lattice(seed, ix + 1, iy + 1);
    let top = a + (b - a) * tx;
    let bottom = c + (d - c) * tx;
    top + (bottom - top) * ty
}

/// Fractal sum of `octaves` value noises, normalised to `[0, 1)`. Each octave has
/// twice the frequency and `gain` times the weight of the one before.
pub fn fbm(seed: Seed, x: f64, y: f64, octaves: u32, gain: f64) -> f64 {
    let (mut sum, mut weight, mut norm, mut freq) = (0.0, 1.0, 0.0, 1.0);
    for o in 0..octaves {
        // A rotation per octave keeps the lattice axes from lining up.
        let (s, c) = (0.5_f64, 0.866_025_403_784_438_6_f64);
        let (rx, ry) = (x * freq * c - y * freq * s, x * freq * s + y * freq * c);
        sum += weight * value(seed.derive_index(i64::from(o)), rx + 17.0 * f64::from(o), ry - 31.0 * f64::from(o));
        norm += weight;
        weight *= gain;
        freq *= 2.0;
    }
    sum / norm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_is_deterministic_bounded_and_continuous() {
        let s = Seed::new(3);
        let mut prev = value(s, 0.0, 0.3);
        for i in 1..2000 {
            let x = i as f64 * 0.01;
            let v = value(s, x, 0.3);
            assert!((0.0..1.0).contains(&v));
            assert!((v - prev).abs() < 0.08, "a step of 0.01 moved the value by {}", (v - prev).abs());
            prev = v;
        }
        assert_eq!(value(s, 4.2, -7.7), value(s, 4.2, -7.7));
        assert_ne!(value(s, 4.2, -7.7), value(Seed::new(4), 4.2, -7.7));
    }

    #[test]
    fn noise_has_the_spread_of_a_uniform_field_and_fbm_stays_in_range() {
        let s = Seed::new(8);
        let (mut lo, mut hi, mut sum) = (1.0f64, 0.0f64, 0.0);
        let n = 20_000;
        for i in 0..n {
            let v = fbm(s, (i % 200) as f64 * 0.37, (i / 200) as f64 * 0.37, 4, 0.5);
            assert!((0.0..1.0).contains(&v));
            lo = lo.min(v);
            hi = hi.max(v);
            sum += v;
        }
        assert!(lo < 0.25 && hi > 0.75, "{lo} {hi}");
        assert!((sum / f64::from(n) - 0.5).abs() < 0.05);
    }

    #[test]
    fn it_works_across_the_origin_and_for_huge_coordinates() {
        let s = Seed::new(1);
        assert!((value(s, -0.001, 0.0) - value(s, 0.001, 0.0)).abs() < 0.01);
        assert!((0.0..1.0).contains(&value(s, 1.0e9, -1.0e9)));
    }
}
