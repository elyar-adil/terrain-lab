//! How straight chord-space lines become roads that bend.
//!
//! All the structure (where streets leave a chord, which cross which) is decided
//! on straight lines. Bending is applied last, as a smooth function of position
//! (`warp`) plus a small wiggle that is zero at both ends of a road. Because the
//! warp is a function of position only, two roads meeting at a node meet at the
//! same warped point whichever of them was built first.

use worldgen_contracts::{Polyline, V2, v2};
use worldgen_core::noise::fbm;
use worldgen_core::{Rect, Seed};

use crate::config::RoadsConfig;

/// A smooth displacement of the whole plane.
pub fn warp(fabric: Seed, cfg: &RoadsConfig, p: V2) -> V2 {
    let (sx, sy) = (fabric.derive("warp.x"), fabric.derive("warp.y"));
    let big = cfg.warp_scale_m;
    let fine = cfg.wiggle_scale_m;
    let dx = (fbm(sx, p.x / big, p.y / big, 3, 0.5) - 0.5) * 2.0 * cfg.warp_amp_m
        + (fbm(sx.derive("fine"), p.x / fine, p.y / fine, 2, 0.5) - 0.5) * 2.0 * cfg.wiggle_amp_m;
    let dy = (fbm(sy, p.x / big, p.y / big, 3, 0.5) - 0.5) * 2.0 * cfg.warp_amp_m
        + (fbm(sy.derive("fine"), p.x / fine, p.y / fine, 2, 0.5) - 0.5) * 2.0 * cfg.wiggle_amp_m;
    p + v2(dx, dy)
}

/// The road from `a` to `b` (chord space), bent. The first and last points are
/// exactly `warp(a)` and `warp(b)`; in between a lateral wiggle that vanishes at
/// both ends adds the irregularity of a street that was not ruled with a straight edge.
pub fn bend(fabric: Seed, cfg: &RoadsConfig, a: V2, b: V2, edge_key: u64, wiggle_scale: f64) -> Polyline {
    let len = a.dist(b);
    let n = ((len / cfg.sample_step_m).ceil() as usize).max(2);
    let side = (b - a).norm().perp();
    let noise_seed = fabric.derive("wiggle").derive_u64(edge_key);
    // A short road cannot wander far without kinking.
    let wiggle_scale = wiggle_scale.min(0.05 * len);
    let mut points = Vec::with_capacity(n + 1);
    points.push(warp(fabric, cfg, a));
    for k in 1..n {
        let s = k as f64 / n as f64;
        let envelope = (std::f64::consts::PI * s).sin();
        let lateral = (fbm(noise_seed, s * len / (cfg.wiggle_scale_m * 1.7), 0.5, 2, 0.5) - 0.5) * 2.0 * wiggle_scale * envelope;
        points.push(warp(fabric, cfg, a.lerp(b, s)) + side * lateral);
    }
    points.push(warp(fabric, cfg, b));
    Polyline(points)
}

/// The parts of a polyline inside a rectangle. A point on the rectangle's border
/// is computed from the segment and the border coordinate alone, so two
/// rectangles that share a border cut a segment at the same point.
pub fn clip(line: &Polyline, rect: Rect) -> Vec<Polyline> {
    let mut pieces: Vec<Vec<V2>> = Vec::new();
    let mut current: Vec<V2> = Vec::new();
    for w in line.0.windows(2) {
        match clip_segment(w[0], w[1], rect) {
            Some((p, q)) => {
                if current.last() != Some(&p) {
                    if !current.is_empty() {
                        pieces.push(std::mem::take(&mut current));
                    }
                    current.push(p);
                }
                current.push(q);
                if q != w[1] {
                    pieces.push(std::mem::take(&mut current));
                }
            }
            None => {
                if !current.is_empty() {
                    pieces.push(std::mem::take(&mut current));
                }
            }
        }
    }
    if !current.is_empty() {
        pieces.push(current);
    }
    pieces.into_iter().filter(|p| p.len() >= 2).map(Polyline).collect()
}

/// A segment cut to a rectangle that is closed on the minimum sides and open on
/// the maximum sides, so two rectangles that share a border do not both own a
/// point exactly on it.
fn clip_segment(p: V2, q: V2, r: Rect) -> Option<(V2, V2)> {
    let (mut t0, mut t1) = (0.0_f64, 1.0_f64);
    let d = q - p;
    for (lo, hi, p0, dd) in [(r.min[0], r.max[0], p.x, d.x), (r.min[1], r.max[1], p.y, d.y)] {
        if dd == 0.0 {
            if p0 < lo || p0 >= hi {
                return None;
            }
        } else {
            let (mut ta, mut tb) = ((lo - p0) / dd, (hi - p0) / dd);
            if ta > tb {
                std::mem::swap(&mut ta, &mut tb);
            }
            t0 = t0.max(ta);
            t1 = t1.min(tb);
            if t0 > t1 {
                return None;
            }
        }
    }
    // A point where the segment meets a border is placed on that border exactly,
    // and its other coordinate comes from the segment and the border alone.
    let snap = |value: f64, lo: f64, hi: f64| -> Option<f64> {
        let near = |edge: f64| (value - edge).abs() < 1e-9 * (1.0 + edge.abs());
        if near(lo) {
            Some(lo)
        } else if near(hi) {
            Some(hi)
        } else {
            None
        }
    };
    let at = |t: f64| {
        if t <= 0.0 {
            return p;
        }
        if t >= 1.0 {
            return q;
        }
        let (mut x, mut y) = (p.x + d.x * t, p.y + d.y * t);
        if let Some(border) = snap(x, r.min[0], r.max[0]) {
            x = border;
            y = p.y + d.y * ((x - p.x) / d.x);
        }
        if let Some(border) = snap(y, r.min[1], r.max[1]) {
            y = border;
            x = p.x + d.x * ((y - p.y) / d.y);
        }
        v2(x, y)
    };
    let (a, b) = (at(t0), at(t1));
    (a != b).then_some((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Rect {
        Rect { min: [x0, y0], max: [x1, y1] }
    }

    #[test]
    fn a_road_ends_exactly_at_the_warped_ends_and_the_warp_is_smooth() {
        let cfg = RoadsConfig::default();
        let fabric = Seed::new(5);
        let (a, b) = (v2(100.0, 200.0), v2(900.0, 650.0));
        let line = bend(fabric, &cfg, a, b, 77, 5.0);
        assert_eq!(line.first().unwrap(), warp(fabric, &cfg, a));
        assert_eq!(line.last().unwrap(), warp(fabric, &cfg, b));
        // Gentle: no sharp corners from the bending of a long straight road.
        assert!(line.max_turn() < 0.12, "turn {}", line.max_turn());
        // Same inputs, same road.
        assert_eq!(line, bend(fabric, &cfg, a, b, 77, 5.0));
        // The warp moves points by about its amplitude, not by kilometres.
        let moved = warp(fabric, &cfg, a).dist(a);
        assert!(moved < 2.0 * (cfg.warp_amp_m + cfg.wiggle_amp_m));
        // A neighbouring point moves almost the same way.
        let near = warp(fabric, &cfg, a + v2(1.0, 0.0)) - (a + v2(1.0, 0.0));
        let here = warp(fabric, &cfg, a) - a;
        assert!((near - here).len() < 0.2);
    }

    #[test]
    fn clipping_keeps_what_is_inside_and_cuts_at_the_border() {
        let line = Polyline(vec![v2(-10.0, 5.0), v2(10.0, 5.0), v2(10.0, 30.0)]);
        let pieces = clip(&line, rect(0.0, 0.0, 20.0, 20.0));
        assert_eq!(pieces.len(), 1);
        assert_eq!(pieces[0].first(), Some(v2(0.0, 5.0)));
        assert_eq!(pieces[0].last(), Some(v2(10.0, 20.0)));
        assert!(clip(&line, rect(100.0, 100.0, 120.0, 120.0)).is_empty());
        // A line that leaves and comes back is two pieces.
        let loop_line = Polyline(vec![v2(5.0, 5.0), v2(25.0, 5.0), v2(25.0, 15.0), v2(5.0, 15.0)]);
        assert_eq!(clip(&loop_line, rect(0.0, 0.0, 20.0, 20.0)).len(), 2);
    }

    #[test]
    fn two_rectangles_that_share_a_border_cut_a_road_at_the_same_point() {
        let line = Polyline(vec![v2(3.0, 1.3), v2(97.7, 61.9), v2(190.1, 40.7)]);
        let left = clip(&line, rect(0.0, 0.0, 100.0, 100.0));
        let right = clip(&line, rect(100.0, 0.0, 200.0, 100.0));
        assert_eq!(left.len(), 1);
        assert_eq!(right.len(), 1);
        assert_eq!(left[0].last(), right[0].first(), "the cut point is shared exactly");
        assert_eq!(left[0].last().unwrap().x, 100.0);
        // Rejoined, the pieces are the whole road.
        let joined = worldgen_contracts::network::stitch(vec![left[0].clone(), right[0].clone()]);
        assert_eq!(joined.len(), 1);
        assert!((joined[0].length() - line.length()).abs() < 1e-9);
    }

    #[test]
    fn a_road_along_a_border_belongs_to_one_side() {
        let line = Polyline(vec![v2(100.0, 10.0), v2(100.0, 90.0)]);
        let n = [clip(&line, rect(0.0, 0.0, 100.0, 100.0)).len(), clip(&line, rect(100.0, 0.0, 200.0, 100.0)).len()];
        assert_eq!(n[0] + n[1], 1);
    }
}
