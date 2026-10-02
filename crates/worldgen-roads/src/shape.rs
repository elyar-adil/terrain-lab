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

/// A route with its corners rounded: each bend is replaced by a circular arc of
/// the given radius, or the largest that fits if the legs are short. A router that
/// works on a grid produces a staircase of kinks; roads are built to a curvature.
///
/// The result is a function of three consecutive vertices at a time, so rounding a
/// long route in pieces gives the same curve as rounding it whole.
pub fn round_corners(line: &Polyline, radius_m: f64, step_m: f64) -> Polyline {
    let p = &line.0;
    if p.len() < 3 {
        return line.clone();
    }
    let mut out = vec![p[0]];
    for i in 1..p.len() - 1 {
        let (a, b, c) = (p[i - 1], p[i], p[i + 1]);
        let (d0, d1) = ((b - a).norm(), (c - b).norm());
        let turn = d0.cross(d1).atan2(d0.dot(d1));
        let (l0, l1) = (a.dist(b), b.dist(c));
        if turn.abs() < 1e-3 || l0 < 1e-9 || l1 < 1e-9 {
            out.push(b);
            continue;
        }
        // The arc touches each leg `reach` before the corner. It may not use more
        // than 45% of a leg, so neighbouring arcs never overlap.
        let half = turn.abs() * 0.5;
        let reach = (radius_m * half.tan()).min(0.45 * l0.min(l1));
        let r = reach / half.tan();
        let (entry, exit) = (b - d0 * reach, b + d1 * reach);
        let centre = entry + d0.perp() * (r * turn.signum());
        let (from, to) = ((entry - centre).angle(), (exit - centre).angle());
        let mut sweep = to - from;
        while sweep > std::f64::consts::PI {
            sweep -= std::f64::consts::TAU;
        }
        while sweep < -std::f64::consts::PI {
            sweep += std::f64::consts::TAU;
        }
        let n = ((r * sweep.abs() / step_m).ceil() as usize).max(2);
        for k in 0..=n {
            let angle = from + sweep * k as f64 / n as f64;
            out.push(centre + V2::from_angle(angle) * r);
        }
    }
    out.push(*p.last().unwrap());
    Polyline(out)
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

/// A side of a rectangle: the axis (0 for x, 1 for y) and whether it is the
/// maximum side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Border {
    pub axis: u8,
    pub max: bool,
}

/// A segment cut to a rectangle that is closed on the minimum sides and open on
/// the maximum sides, so two rectangles that share a border do not both own a
/// point exactly on it. Also says which side the cut ends lie on, if they were cut.
pub(crate) fn clip_segment(p: V2, q: V2, r: Rect) -> Option<(V2, V2)> {
    clip_segment_at(p, q, r).map(|(a, b, _, _)| (a, b))
}

pub(crate) fn clip_segment_at(p: V2, q: V2, r: Rect) -> Option<(V2, V2, Option<Border>, Option<Border>)> {
    let (mut t0, mut t1) = (0.0_f64, 1.0_f64);
    let (mut entry, mut exit): (Option<Border>, Option<Border>) = (None, None);
    let d = q - p;
    for (axis, (lo, hi, p0, dd)) in [(r.min[0], r.max[0], p.x, d.x), (r.min[1], r.max[1], p.y, d.y)].into_iter().enumerate() {
        if dd == 0.0 {
            if p0 < lo || p0 >= hi {
                return None;
            }
        } else {
            let (mut ta, mut tb) = ((lo - p0) / dd, (hi - p0) / dd);
            // Walking in the positive direction, the minimum side is entered first.
            let (mut enter_max, mut leave_max) = (false, true);
            if ta > tb {
                std::mem::swap(&mut ta, &mut tb);
                (enter_max, leave_max) = (true, false);
            }
            if ta > t0 {
                t0 = ta;
                entry = Some(Border { axis: axis as u8, max: enter_max });
            }
            if tb < t1 {
                t1 = tb;
                exit = Some(Border { axis: axis as u8, max: leave_max });
            }
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
    (a != b).then_some((a, b, entry.filter(|_| t0 > 0.0), exit.filter(|_| t1 < 1.0)))
}

/// A continuous run of a polyline inside a rectangle, with where it came in and went out.
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub points: Vec<V2>,
    /// Index of the segment of the original polyline the run starts in.
    pub first_segment: usize,
    pub last_segment: usize,
    /// The side it entered by, if it was cut there rather than starting inside.
    pub entered: Option<Border>,
    pub left: Option<Border>,
}

/// Like [`clip`], but remembers which segment and which border each cut was at.
pub fn clip_runs(line: &Polyline, rect: Rect) -> Vec<Run> {
    let mut runs: Vec<Run> = Vec::new();
    let mut open: Option<Run> = None;
    for (i, w) in line.0.windows(2).enumerate() {
        match clip_segment_at(w[0], w[1], rect) {
            Some((p, q, entry, exit)) => {
                let continuing = open.as_ref().is_some_and(|r| r.points.last() == Some(&p));
                if !continuing {
                    if let Some(r) = open.take() {
                        runs.push(r);
                    }
                    open = Some(Run { points: vec![p], first_segment: i, last_segment: i, entered: entry, left: None });
                }
                let run = open.as_mut().unwrap();
                run.points.push(q);
                run.last_segment = i;
                if exit.is_some() {
                    run.left = exit;
                    runs.push(open.take().unwrap());
                }
            }
            None => {
                if let Some(r) = open.take() {
                    runs.push(r);
                }
            }
        }
    }
    runs.extend(open);
    runs.into_iter().filter(|r| r.points.len() >= 2).collect()
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
    fn rounding_replaces_a_kink_with_an_arc_that_stays_close_to_the_route() {
        let route = Polyline(vec![v2(0.0, 0.0), v2(500.0, 0.0), v2(500.0, 400.0), v2(1200.0, 450.0)]);
        let round = round_corners(&route, 120.0, 15.0);
        assert!(route.max_turn() > 1.4, "the route has a right angle");
        assert!(round.max_turn() < 0.2, "no kink is left: {}", round.max_turn());
        assert_eq!(round.first(), route.first());
        assert_eq!(round.last(), route.last());
        // It cuts the corner by about the fillet, not by hundreds of metres.
        assert!(round.length() < route.length() && round.length() > route.length() - 150.0);
        for p in &round.0 {
            assert!(route.closest(*p).unwrap().0 < 60.0, "{p:?} is far from the route");
        }
        // Straight routes and short ones pass through.
        let straight = Polyline(vec![v2(0.0, 0.0), v2(10.0, 0.0), v2(20.0, 0.0)]);
        assert_eq!(round_corners(&straight, 100.0, 10.0).0.len(), 3);
        assert_eq!(round_corners(&Polyline(vec![v2(0.0, 0.0), v2(1.0, 1.0)]), 100.0, 10.0).0.len(), 2);
    }

    #[test]
    fn rounding_a_route_in_halves_matches_rounding_it_whole() {
        let route = Polyline((0..9).map(|k| v2(k as f64 * 300.0, if k % 2 == 0 { 0.0 } else { 220.0 })).collect());
        let whole = round_corners(&route, 90.0, 12.0);
        // Rounding keeps every vertex of the interior corners' neighbourhoods local.
        let left = round_corners(&Polyline(route.0[..6].to_vec()), 90.0, 12.0);
        for p in left.0.iter().take(left.0.len() - 12) {
            assert!(whole.0.contains(p), "{p:?} differs when the route is cut");
        }
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
