//! Road-network generation.
//!
//! Streets are planned as whole lines first — a district grid, a 16-sided
//! expressway ring, an inner arterial ring and optional diagonal avenues — and
//! only then turned into a graph.  A planarisation pass splits every crossing
//! (including a line that passes *through* another line's endpoint), snaps
//! T-junctions, merges nodes closer than a minimum edge length and repeats
//! until stable, so the result is a proper planar street network rather than
//! roads that merely overlap.  The river, the local-street growth model and
//! dead-end pruning then run on the planar graph.

use super::CityFrame;
use super::geom::V;
use super::roads::{china_rules, lanes_for_modern_road};
use crate::model::morphology::MorphologyPrior;
use crate::model::probability::{ActionKind, GrowthState, ModelWeights, SplitMix64, sample_action};
use crate::model::{CityGraph, ModernRoadClass};
use crate::{HdRoad, Point, SdNode, SdRoad};

/// Nodes closer than this are one junction.
const MIN_EDGE_M: f32 = 16.0;
/// A node this close to another street's side is a T-junction onto it.
const SNAP_M: f32 = 8.0;
const RING_SIDES: usize = 16;

pub(super) struct GraphOutput {
    pub nodes: Vec<SdNode>,
    pub sd_roads: Vec<SdRoad>,
    pub hd_roads: Vec<HdRoad>,
    pub river: Vec<Point>,
    pub morphology_score: f64,
}

/// Borrowed view of the graph before the owning `ModernCity` exists, so the
/// morphology prior can score candidates during generation.
pub(super) struct GraphProbe<'a> {
    pub nodes: &'a [SdNode],
    pub sd_roads: &'a [SdRoad],
    pub hd_roads: &'a [HdRoad],
}

impl CityGraph for GraphProbe<'_> {
    fn nodes(&self) -> &[SdNode] {
        self.nodes
    }
    fn sd_roads(&self) -> &[SdRoad] {
        self.sd_roads
    }
    fn hd_roads(&self) -> &[HdRoad] {
        self.hd_roads
    }
}

/// How far back from a node a street is trimmed to make room for the junction,
/// in metres. Both the street planner (which must not leave a link shorter than
/// its two boxes) and the scene builder (which draws the boxes) use this one
/// number, or one of them is wrong about whether a road fits.
///
/// At a junction it is the widest street's half width plus the kerb-return
/// radius, because that is where the kerb leaves the straight to turn the corner:
/// the return radius is about 12 m where an arterial is involved, 8 m for
/// collectors and 5 m for local streets (the scale of the 缘石转弯半径 in CJJ 37,
/// reduced a little for a compact block grid). The earlier formula, about 9 m for
/// an arterial, left a corner radius of three metres, which reads as a square
/// corner. A node with two streets is a bend and gets a small trim.
pub fn junction_trim_m(widest_width: f32, arms: usize) -> f32 {
    worldgen_contracts::junction_trim_m(f64::from(widest_width), arms) as f32
}

/// How important a street is: larger is wider and busier. The enum is declared
/// widest first (`Expressway` is 0), so comparing its discriminant directly picks
/// the *lowest* class; every "keep the higher class" below goes through this.
fn importance(class: ModernRoadClass) -> i32 {
    3 - class as i32
}

/// Which family a segment came from, kept through planarisation so junction
/// roles can still be assigned.
const ORIGIN_GRID: u8 = 4;
const ORIGIN_QUAY: u8 = 8;
const ORIGIN_OUTER: u8 = 1;
const ORIGIN_INNER: u8 = 2;
/// A regional road continued into town.  Kept through planarisation so junction
/// tidying can favour it over a grid street of the same class.
const ORIGIN_APPROACH: u8 = 16;

#[derive(Clone, Copy)]
struct Seg {
    a: usize,
    b: usize,
    class: ModernRoadClass,
    origin: u8,
}

fn ring_polygon(radius: f32) -> Vec<V> {
    // Vertices sit half a step off the axes so every edge meets the district
    // grid at a multiple of 22.5°.
    (0..RING_SIDES)
        .map(|i| {
            let a = (i as f32 + 0.5) * std::f32::consts::TAU / RING_SIDES as f32;
            (radius * a.cos(), radius * a.sin())
        })
        .collect()
}

/// Clip the infinite line `p + t·d` to a convex counter-clockwise polygon.
fn clip_line(poly: &[V], p: V, d: V) -> Option<(V, V)> {
    let (mut t0, mut t1) = (-1.0e7_f32, 1.0e7_f32);
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        let (ex, ez) = (b.0 - a.0, b.1 - a.1);
        let len = ex.hypot(ez);
        let n = (-ez / len, ex / len);
        let num = n.0 * (p.0 - a.0) + n.1 * (p.1 - a.1);
        let den = n.0 * d.0 + n.1 * d.1;
        if den.abs() < 1.0e-9 {
            if num < 0.0 {
                return None;
            }
        } else {
            let t = -num / den;
            if den > 0.0 {
                t0 = t0.max(t);
            } else {
                t1 = t1.min(t);
            }
        }
    }
    (t1 - t0 > MIN_EDGE_M).then_some({
        (
            (p.0 + d.0 * t0, p.1 + d.1 * t0),
            (p.0 + d.0 * t1, p.1 + d.1 * t1),
        )
    })
}

/// Clip the segment `a..b` to a convex counter-clockwise polygon.
fn clip_segment(poly: &[V], a: V, b: V) -> Option<(V, V)> {
    let d = (b.0 - a.0, b.1 - a.1);
    let (mut t0, mut t1) = (0.0_f32, 1.0_f32);
    for i in 0..poly.len() {
        let p = poly[i];
        let q = poly[(i + 1) % poly.len()];
        let (ex, ez) = (q.0 - p.0, q.1 - p.1);
        let len = ex.hypot(ez);
        let n = (-ez / len, ex / len);
        let num = n.0 * (a.0 - p.0) + n.1 * (a.1 - p.1);
        let den = n.0 * d.0 + n.1 * d.1;
        if den.abs() < 1.0e-9 {
            if num < 0.0 {
                return None;
            }
        } else {
            let t = -num / den;
            if den > 0.0 {
                t0 = t0.max(t);
            } else {
                t1 = t1.min(t);
            }
        }
        if t1 <= t0 {
            return None;
        }
    }
    Some((
        (a.0 + d.0 * t0, a.1 + d.1 * t0),
        (a.0 + d.0 * t1, a.1 + d.1 * t1),
    ))
}

/// The parts of a regional road that lie inside the outer ring, in local
/// metres.  Each starts exactly on the ring where the road enters; a road that
/// ends in town simply stops at the end of its path.
fn approach_runs(frame: &CityFrame, outer: &[V], path: &[Point]) -> Vec<Vec<V>> {
    let local: Vec<V> = path.iter().map(|p| frame.to_local(*p)).collect();
    let mut runs: Vec<Vec<V>> = Vec::new();
    let mut open = false;
    for w in local.windows(2) {
        let Some((c, d)) = clip_segment(outer, w[0], w[1]) else {
            open = false;
            continue;
        };
        let continues = open
            && runs
                .last()
                .is_some_and(|r| r.last().is_some_and(|l| (l.0 - c.0).hypot(l.1 - c.1) < 0.5));
        if continues {
            runs.last_mut().unwrap().push(d);
        } else {
            runs.push(vec![c, d]);
        }
        // The run stays open only while the segment's own end is inside.
        open = (d.0 - w[1].0).hypot(d.1 - w[1].1) < 0.5;
    }
    runs.retain(|r| {
        r.windows(2)
            .map(|s| (s[1].0 - s[0].0).hypot(s[1].1 - s[0].1))
            .sum::<f32>()
            > 3.0 * MIN_EDGE_M
    });
    runs
}

/// Streets a regional road may join: collector or better, and not the outer ring
/// unless the caller says so (a road that enters town in the river reaches the
/// ring from the bank).
fn joinable(s: &Seg, allow_ring: bool) -> bool {
    (allow_ring || s.origin != ORIGIN_OUTER)
        && matches!(
            s.class,
            ModernRoadClass::Expressway | ModernRoadClass::Arterial | ModernRoadClass::Collector
        )
}

/// Cut `run` at the first joinable street it crosses (where `valid` allows a
/// junction) deeper than `deeper_than` metres from the centre, walking from its start (the anchored end).  Anything
/// within a junction's width of the start is ignored: that is the ring itself.
fn cut_at_first_street(
    pts: &[V],
    segs: &[Seg],
    run: &mut Vec<V>,
    deeper_than: f32,
    valid: &dyn Fn(V) -> bool,
) -> bool {
    let mut walked = 0.0_f32;
    for i in 0..run.len() - 1 {
        let (a, b) = (run[i], run[i + 1]);
        let len = (b.0 - a.0).hypot(b.1 - a.1);
        let mut best: Option<(f32, V)> = None;
        for s in segs.iter().filter(|s| joinable(s, false)) {
            if let Some((t, _, p)) = seg_intersect(a, b, pts[s.a], pts[s.b]) {
                // Never cut before the handover radius: the regional renderer draws
                // the road up to there and the city must carry it the rest of the way.
                if walked + t * len > 2.0 * MIN_EDGE_M
                    && p.0.hypot(p.1) < deeper_than
                    && valid(p)
                    && best.is_none_or(|(bt, _)| t < bt)
                {
                    best = Some((t, p));
                }
            }
        }
        if let Some((_, p)) = best {
            run.truncate(i + 1);
            run.push(p);
            return true;
        }
        walked += len;
    }
    false
}

/// Carry the end of `run` on to the nearest joinable street.
fn reach_nearest_street(
    pts: &[V],
    segs: &[Seg],
    run: &mut Vec<V>,
    allow_ring: bool,
    dry: &dyn Fn(V) -> bool,
) {
    let end = *run.last().unwrap();
    let nearest = segs
        .iter()
        .filter(|s| joinable(s, allow_ring))
        .flat_map(|s| {
            // Candidate joins are sampled along the street, and only on dry land:
            // a junction in the channel is never allowed.
            let (a, b) = (pts[s.a], pts[s.b]);
            let steps = (((b.0 - a.0).hypot(b.1 - a.1) / 8.0).ceil() as usize).max(1);
            (0..=steps).map(move |k| {
                let t = k as f32 / steps as f32;
                (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
            })
        })
        .filter(|p| dry(*p))
        .map(|p| ((p.0 - end.0).hypot(p.1 - end.1), p))
        .min_by(|x, y| x.0.total_cmp(&y.0));
    if let Some((d, p)) = nearest
        && d > 1.0
    {
        run.push(p);
    }
}

/// A road that ends in town must end *on* the street plan, not in the middle of
/// a block: a dangling street is pruned back to its last junction, taking the
/// regional road with it.  So:
/// - the end on the outer ring is the entry and stays where it is;
/// - an end that stops in town is cut at the first collector-or-better street it
///   meets (or carried on to the nearest one);
/// - an end that lands in the river is moved to the bank and joined to the
///   nearest street, ring included, because the ring bridges the water there.
fn tie_run_into_plan(
    frame: &CityFrame,
    outer: &[V],
    pts: &[V],
    segs: &[Seg],
    mut run: Vec<V>,
) -> Vec<V> {
    let zone = frame.river_half + QUAY_OFF_M + 12.0;
    let in_zone = |p: V| (p.0 - frame.river_x(p.1)).abs() <= zone;
    let dry = |p: V| {
        !in_zone(p)
            && (!frame.organic_footprint
                || frame.urbanness(p.0, p.1) > 0.3
                || p.0.hypot(p.1) > frame.radius_m * 0.95)
    };
    // Only an end that lies in the channel itself (where no junction may stand)
    // is moved to the bank. One merely in the quay zone keeps its place: the road
    // crosses the water from there as a bridge, and trimming it would drop the
    // whole crossing and leave a gap between the regional road and the town.
    let in_channel = |p: V| (p.0 - frame.river_x(p.1)).abs() <= frame.river_half + SNAP_M + 2.0;
    let deeper = frame.radius_m * (super::REGIONAL_ROAD_HANDOVER - 0.03);
    // With an irregular built-up area the grid outside it is pruned, so a road
    // may only join streets that will survive: those inside the town proper.
    let in_town = |p: V| !frame.organic_footprint || frame.urbanness(p.0, p.1) > 0.3;
    let (mut start_on, mut end_on) = (on_ring(outer, run[0]), on_ring(outer, *run.last().unwrap()));
    // Move river-bound ends to dry land.
    let (mut start_wet, mut end_wet) = (false, false);
    while run.len() > 2 && in_channel(run[0]) {
        run.remove(0);
        start_wet = true;
    }
    while run.len() > 2 && in_channel(*run.last().unwrap()) {
        run.pop();
        end_wet = true;
    }
    if start_wet {
        start_on = false;
    }
    if end_wet {
        end_on = false;
    }
    if start_wet {
        run.reverse();
        reach_nearest_street(pts, segs, &mut run, true, &dry);
        run.reverse();
    }
    if end_wet {
        reach_nearest_street(pts, segs, &mut run, true, &dry);
    }
    // Ends that were never on the ring and did not need the bank fix stop in town.
    let free_start = !start_on && !start_wet;
    let free_end = !end_on && !end_wet;
    match (free_start, free_end) {
        (false, false) => {}
        (false, true) => {
            if !cut_at_first_street(pts, segs, &mut run, deeper, &in_town) {
                reach_nearest_street(pts, segs, &mut run, false, &dry);
            }
        }
        (true, false) => {
            run.reverse();
            if !cut_at_first_street(pts, segs, &mut run, deeper, &in_town) {
                reach_nearest_street(pts, segs, &mut run, false, &dry);
            }
            run.reverse();
        }
        (true, true) => {
            reach_nearest_street(pts, segs, &mut run, false, &dry);
            run.reverse();
            reach_nearest_street(pts, segs, &mut run, false, &dry);
            run.reverse();
        }
    }
    run
}

/// Whether `p` lies on the boundary of the convex counter-clockwise polygon.
fn on_ring(poly: &[V], p: V) -> bool {
    (0..poly.len()).any(|i| {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        let (ex, ez) = (b.0 - a.0, b.1 - a.1);
        let len = ex.hypot(ez);
        ((-ez / len) * (p.0 - a.0) + (ex / len) * (p.1 - a.1)).abs() < 0.75
    })
}

fn add_edge(pts: &mut Vec<V>, segs: &mut Vec<Seg>, a: V, b: V, class: ModernRoadClass, origin: u8) {
    let base = pts.len();
    pts.push(a);
    pts.push(b);
    segs.push(Seg {
        a: base,
        b: base + 1,
        class,
        origin,
    });
}

/// Distance from the river centreline to a quay (riverside road) centreline,
/// beyond the water's half width.  The quay's outer kerb then sits about a dozen
/// metres from the water (a narrow promenade), while a junction box on it, up to
/// twenty-odd metres deep, still lands on dry ground.
pub(super) const QUAY_OFF_M: f32 = 28.0;

/// Offsets and classes of one family of parallel grid lines.  Spacing is tight
/// where the urban intensity is high and opens out towards the edge, every
/// line is jittered, and the road class follows the distance since the last
/// higher-class street rather than a fixed count.  Lines that would run
/// alongside a ring edge are dropped; the ring itself carries that traffic.
fn grid_offsets(
    frame: &CityFrame,
    x_family: bool,
    salt: i32,
    skip: &[f32],
) -> Vec<(f32, ModernRoadClass)> {
    let half = frame.radius_m;
    let block = frame.block_m;
    let seed = frame.spec.seed;
    let weight = |pos: f32| {
        if x_family {
            frame.core_weight(pos, frame.core.1)
        } else {
            frame.core_weight(frame.core.0, pos)
        }
    };
    let start = (modern_hash(seed, 0, salt, 733) - 0.5) * block * 0.5;
    let mut lines: Vec<(f32, ModernRoadClass)> = vec![(start, ModernRoadClass::Arterial)];
    for dir in [1.0_f32, -1.0] {
        let mut pos = start;
        let mut since_art = 0.0_f32;
        let mut since_col = 0.0_f32;
        for i in 1..80 {
            let w = weight(pos);
            let jit = 0.8 + 0.45 * modern_hash(seed, i * 2 + (dir > 0.0) as i32, salt, 739);
            let step = (block * (0.58 + 0.82 * (1.0 - w)) * jit).max(52.0);
            pos += dir * step;
            since_art += step;
            since_col += step;
            if pos.abs() >= half {
                break;
            }
            let class = if since_art >= 420.0 * (0.8 + 0.4 * modern_hash(seed, i, salt, 743)) {
                since_art = 0.0;
                since_col = 0.0;
                ModernRoadClass::Arterial
            } else if since_col >= 170.0 {
                since_col = 0.0;
                ModernRoadClass::Collector
            } else {
                ModernRoadClass::Local
            };
            if skip.iter().any(|a| (pos.abs() - a).abs() < step * 0.4) {
                continue;
            }
            lines.push((pos, class));
        }
    }
    lines.sort_by(|a, b| a.0.total_cmp(&b.0));
    lines
}

/// Smooth displacement that bends the ideal grid into a hand-drawn one.  It
/// fades to nothing at the outer ring so lines still meet it exactly.
fn warp(frame: &CityFrame, p: V) -> V {
    let seed = frame.spec.seed;
    let amp = frame.block_m * (0.08 + 0.2 * frame.organic);
    let ph = |k: i32| modern_hash(seed, k, 7, 907) * std::f32::consts::TAU;
    let r = frame.radius_m;
    let (x, z) = p;
    let dx =
        (z / (r * 1.1) + ph(1)).sin() * 0.7 + (x / (r * 0.8) + z / (r * 1.0) + ph(2)).sin() * 0.5;
    let dz =
        (x / (r * 1.2) + ph(3)).sin() * 0.7 + (z / (r * 0.75) - x / (r * 0.9) + ph(4)).sin() * 0.5;
    let edge = ((r * 0.975 - p.0.hypot(p.1)) / 70.0).clamp(0.0, 1.0);
    (x + dx * amp * edge, z + dz * amp * edge)
}

/// Add a street as a chain of short segments through `samples`, dropping any
/// sample that falls in the river/quay zone so that the street crosses the
/// water in a single span (which becomes a bridge meeting the quay).
fn add_polyline(
    frame: &CityFrame,
    pts: &mut Vec<V>,
    segs: &mut Vec<Seg>,
    samples: &[V],
    class: ModernRoadClass,
    origin: u8,
) {
    let zone = frame.river_half + QUAY_OFF_M + 12.0;
    let last = samples.len() - 1;
    let kept: Vec<V> = samples
        .iter()
        .enumerate()
        .filter(|(i, p)| *i == 0 || *i == last || (p.0 - frame.river_x(p.1)).abs() > zone)
        .map(|(_, p)| *p)
        .collect();
    for w in kept.windows(2) {
        add_edge(pts, segs, w[0], w[1], class, origin);
    }
}

/// A closed ring road, subdivided along its own edges (so its shape is
/// unchanged) with every sample inside the river zone dropped: the ring then
/// leaps the water in one span, which becomes a bridge, instead of ending at a
/// vertex that happens to lie in the channel.
fn add_ring_over_river(
    frame: &CityFrame,
    pts: &mut Vec<V>,
    segs: &mut Vec<Seg>,
    poly: &[V],
    class: ModernRoadClass,
    origin: u8,
) {
    let zone = frame.river_half + QUAY_OFF_M + 12.0;
    let mut samples: Vec<V> = Vec::new();
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        let n = (((b.0 - a.0).hypot(b.1 - a.1) / 40.0).ceil() as usize).max(1);
        for k in 0..n {
            let t = k as f32 / n as f32;
            samples.push((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t));
        }
    }
    let kept: Vec<V> = samples
        .into_iter()
        .filter(|p| (p.0 - frame.river_x(p.1)).abs() > zone)
        .collect();
    for i in 0..kept.len() {
        add_edge(
            pts,
            segs,
            kept[i],
            kept[(i + 1) % kept.len()],
            class,
            origin,
        );
    }
}

/// Sample a straight street at about `step` metres and bend it by `warp`.
fn warped_line(frame: &CityFrame, a: V, b: V, step: f32) -> Vec<V> {
    let len = (b.0 - a.0).hypot(b.1 - a.1);
    let n = ((len / step).ceil() as usize).max(2);
    (0..=n)
        .map(|i| {
            let t = i as f32 / n as f32;
            warp(frame, (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t))
        })
        .collect()
}

fn seg_intersect(a: V, b: V, c: V, d: V) -> Option<(f32, f32, V)> {
    let r = (b.0 - a.0, b.1 - a.1);
    let s = (d.0 - c.0, d.1 - c.1);
    let den = r.0 * s.1 - r.1 * s.0;
    if den.abs() < 1.0e-4 * r.0.hypot(r.1) * s.0.hypot(s.1) {
        return None;
    }
    let q = (c.0 - a.0, c.1 - a.1);
    let t = (q.0 * s.1 - q.1 * s.0) / den;
    let u = (q.0 * r.1 - q.1 * r.0) / den;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)).then_some((
        t,
        u,
        (a.0 + r.0 * t, a.1 + r.1 * t),
    ))
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Merge nodes closer than `MIN_EDGE_M`, drop self loops and duplicate edges
/// (keeping the higher class) and compact the node list.  True if any merge.
fn merge_close(pts: &mut Vec<V>, segs: &mut Vec<Seg>) -> bool {
    let n = pts.len();
    let mut used = vec![false; n];
    for s in segs.iter() {
        used[s.a] = true;
        used[s.b] = true;
    }
    let mut parent: Vec<usize> = (0..n).collect();
    let mut merged = false;
    for i in 0..n {
        if !used[i] {
            continue;
        }
        for j in (i + 1)..n {
            if !used[j] {
                continue;
            }
            if (pts[i].0 - pts[j].0).hypot(pts[i].1 - pts[j].1) < MIN_EDGE_M {
                let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
                if ri != rj {
                    parent[rj] = ri;
                    merged = true;
                }
            }
        }
    }
    let mut new_id = vec![usize::MAX; n];
    let mut acc: Vec<(f32, f32, f32)> = Vec::new();
    for i in 0..n {
        if !used[i] {
            continue;
        }
        let root = find(&mut parent, i);
        if new_id[root] == usize::MAX {
            new_id[root] = acc.len();
            acc.push((0.0, 0.0, 0.0));
        }
        let id = new_id[root];
        acc[id].0 += pts[i].0;
        acc[id].1 += pts[i].1;
        acc[id].2 += 1.0;
    }
    let remap: Vec<usize> = (0..n)
        .map(|i| {
            if used[i] {
                new_id[find(&mut parent, i)]
            } else {
                usize::MAX
            }
        })
        .collect();
    *pts = acc.iter().map(|(x, z, c)| (x / c, z / c)).collect();
    let mut out: Vec<Seg> = Vec::with_capacity(segs.len());
    for s in segs.iter() {
        let (a, b) = (remap[s.a], remap[s.b]);
        if a == b {
            continue;
        }
        let key = (a.min(b), a.max(b));
        if let Some(existing) = out.iter_mut().find(|e| (e.a.min(e.b), e.a.max(e.b)) == key) {
            if importance(s.class) > importance(existing.class) {
                existing.class = s.class;
                existing.origin = s.origin;
            }
        } else {
            out.push(Seg {
                a,
                b,
                class: s.class,
                origin: s.origin,
            });
        }
    }
    *segs = out;
    merged
}

/// Split edges at T-junctions (a node within `SNAP_M` of another edge's side)
/// and at genuine interior crossings.  True if anything was split.
fn split_edges(pts: &mut Vec<V>, segs: &mut Vec<Seg>, forbid: &dyn Fn(V) -> bool) -> bool {
    let m = segs.len();
    let mut cuts: Vec<Vec<(f32, usize)>> = vec![Vec::new(); m];
    let np = pts.len();
    let guard = MIN_EDGE_M * 0.5;
    for (ei, s) in segs.iter().enumerate() {
        let (a, b) = (pts[s.a], pts[s.b]);
        let (dx, dz) = (b.0 - a.0, b.1 - a.1);
        let len = dx.hypot(dz);
        if len < 1.0e-3 {
            continue;
        }
        let (x0, x1) = (a.0.min(b.0) - SNAP_M, a.0.max(b.0) + SNAP_M);
        let (z0, z1) = (a.1.min(b.1) - SNAP_M, a.1.max(b.1) + SNAP_M);
        for n in 0..np {
            if n == s.a || n == s.b {
                continue;
            }
            let p = pts[n];
            if p.0 < x0 || p.0 > x1 || p.1 < z0 || p.1 > z1 {
                continue;
            }
            let t = ((p.0 - a.0) * dx + (p.1 - a.1) * dz) / (len * len);
            if t * len <= guard || (1.0 - t) * len <= guard {
                continue;
            }
            let d = (p.0 - a.0 - dx * t).hypot(p.1 - a.1 - dz * t);
            if d < SNAP_M && !forbid(p) {
                cuts[ei].push((t, n));
            }
        }
    }
    let mut fresh: Vec<V> = Vec::new();
    for i in 0..m {
        let si = segs[i];
        let (a, b) = (pts[si.a], pts[si.b]);
        let li = (b.0 - a.0).hypot(b.1 - a.1);
        for j in (i + 1)..m {
            let sj = segs[j];
            if si.a == sj.a || si.a == sj.b || si.b == sj.a || si.b == sj.b {
                continue;
            }
            let (c, d) = (pts[sj.a], pts[sj.b]);
            if a.0.max(b.0) < c.0.min(d.0)
                || c.0.max(d.0) < a.0.min(b.0)
                || a.1.max(b.1) < c.1.min(d.1)
                || c.1.max(d.1) < a.1.min(b.1)
            {
                continue;
            }
            let Some((t, u, p)) = seg_intersect(a, b, c, d) else {
                continue;
            };
            let lj = (d.0 - c.0).hypot(d.1 - c.1);
            if t * li <= guard
                || (1.0 - t) * li <= guard
                || u * lj <= guard
                || (1.0 - u) * lj <= guard
            {
                continue;
            }
            if forbid(p) {
                continue; // never put a junction in the river
            }
            let id = np + fresh.len();
            fresh.push(p);
            cuts[i].push((t, id));
            cuts[j].push((u, id));
        }
    }
    pts.extend(fresh);
    let mut out: Vec<Seg> = Vec::with_capacity(m + m / 2);
    let mut changed = false;
    for (ei, s) in segs.iter().enumerate() {
        if cuts[ei].is_empty() {
            out.push(*s);
            continue;
        }
        changed = true;
        let mut list = std::mem::take(&mut cuts[ei]);
        list.sort_by(|x, y| x.0.total_cmp(&y.0));
        let mut prev = s.a;
        for (_, n) in list {
            if n == prev {
                continue;
            }
            out.push(Seg {
                a: prev,
                b: n,
                ..*s
            });
            prev = n;
        }
        if prev != s.b {
            out.push(Seg {
                a: prev,
                b: s.b,
                ..*s
            });
        }
    }
    *segs = out;
    changed
}

fn planarize(pts: &mut Vec<V>, segs: &mut Vec<Seg>, forbid: &dyn Fn(V) -> bool) {
    for _ in 0..8 {
        let merged = merge_close(pts, segs);
        let split = split_edges(pts, segs, forbid);
        if !merged && !split {
            break;
        }
    }
    merge_close(pts, segs);
}

pub(super) fn build_graph(
    frame: &CityFrame,
    approaches: &[super::RegionalApproach],
) -> GraphOutput {
    let radius_m = frame.radius_m;
    let block_m = frame.block_m;
    let river_half = frame.river_half;
    let seed = frame.spec.seed;
    let rules = china_rules();
    let mut river = Vec::new();
    for i in 0..=24 {
        let z = -radius_m + 2.0 * radius_m * i as f32 / 24.0;
        river.push(frame.to_world(frame.river_x(z), z));
    }

    // ---- 1. plan whole lines ----
    let outer = ring_polygon(radius_m);
    let inner_r = radius_m * 0.72;
    let inner: Vec<V> = ring_polygon(inner_r)
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            let k = 1.0 + (modern_hash(seed, i as i32, 3, 919) - 0.5) * 0.14;
            warp(frame, (v.0 * k, v.1 * k))
        })
        .collect();
    let cos_half = (std::f32::consts::PI / RING_SIDES as f32).cos();
    let skip = [radius_m * cos_half, inner_r * cos_half];
    let mut pts: Vec<V> = Vec::new();
    let mut segs: Vec<Seg> = Vec::new();
    for (x, class) in grid_offsets(frame, true, 11, &skip) {
        // A north–south street that would lie in the river channel for much of
        // its length is not a street, it is a river; drop it in the plan so
        // the cross streets span the water in one piece instead of ending in it.
        let in_channel = (0..=32)
            .filter(|i| {
                let z = -radius_m + 2.0 * radius_m * *i as f32 / 32.0;
                (x - frame.river_x(z)).abs() < river_half + QUAY_OFF_M + 46.0
            })
            .count();
        if in_channel > 8 {
            continue;
        }
        if let Some((a, b)) = clip_line(&outer, (x, 0.0), (0.0, 1.0)) {
            let s = warped_line(frame, a, b, 38.0);
            add_polyline(frame, &mut pts, &mut segs, &s, class, ORIGIN_GRID);
        }
    }
    for (z, class) in grid_offsets(frame, false, 29, &skip) {
        if let Some((a, b)) = clip_line(&outer, (0.0, z), (1.0, 0.0)) {
            let s = warped_line(frame, a, b, 38.0);
            add_polyline(frame, &mut pts, &mut segs, &s, class, ORIGIN_GRID);
        }
    }
    add_ring_over_river(
        frame,
        &mut pts,
        &mut segs,
        &outer,
        ModernRoadClass::Expressway,
        ORIGIN_OUTER,
    );
    add_ring_over_river(
        frame,
        &mut pts,
        &mut segs,
        &inner,
        ModernRoadClass::Collector,
        ORIGIN_INNER,
    );

    // A riverside road (滨河路) on each bank, so the land between the river and
    // the first cross street is a closed block that can be built on rather
    // than an unbounded strip.
    // Far enough back that a junction box on the riverside road (twenty to
    // thirty-five metres deep) stays on dry land instead of hanging over the
    // water and leaving the bridge deck short of the far bank.
    let bank_off = river_half + QUAY_OFF_M;
    let bank_steps = ((2.0 * radius_m / 40.0) as usize).max(8);
    for side in [-1.0_f32, 1.0] {
        let mut prev: Option<V> = None;
        for i in 0..=bank_steps {
            let z = -radius_m + 2.0 * radius_m * i as f32 / bank_steps as f32;
            let v = (frame.river_x(z) + side * bank_off, z);
            let inside = v.0.hypot(v.1) < radius_m * cos_half * 0.985;
            if let (Some(p), true) = (prev, inside) {
                add_edge(
                    &mut pts,
                    &mut segs,
                    p,
                    v,
                    ModernRoadClass::Collector,
                    ORIGIN_QUAY,
                );
            }
            prev = inside.then_some(v);
        }
    }

    // One or two curved arterials cut across the grid at oblique angles, as
    // old radial roads do.  Each is a quadratic Bezier between two points of
    // the outer ring, bowed sideways by a seeded amount.
    let arcs = 1 + (modern_hash(seed, 5, 5, 923) > 0.45) as usize;
    for k in 0..arcs {
        let kk = k as i32;
        let a0 = modern_hash(seed, kk, 1, 929) * std::f32::consts::TAU;
        let a1 = a0 + std::f32::consts::PI + (modern_hash(seed, kk, 2, 937) - 0.5) * 1.1;
        let end = |ang: f32| clip_line(&outer, (0.0, 0.0), (ang.cos(), ang.sin())).map(|(_, e)| e);
        let (Some(p0), Some(p2)) = (end(a0), end(a1)) else {
            continue;
        };
        // Pass beside the core rather than through the middle of the grid.
        let mid = ((p0.0 + p2.0) * 0.5, (p0.1 + p2.1) * 0.5);
        let chord = (p2.0 - p0.0, p2.1 - p0.1);
        let cl = chord.0.hypot(chord.1).max(1.0);
        let perp = (-chord.1 / cl, chord.0 / cl);
        let bow = radius_m
            * (0.18 + 0.3 * modern_hash(seed, kk, 3, 941))
            * if modern_hash(seed, kk, 4, 947) < 0.5 {
                -1.0
            } else {
                1.0
            };
        let ctrl = (mid.0 + perp.0 * bow, mid.1 + perp.1 * bow);
        let n = ((cl * 1.2 / 34.0).ceil() as usize).max(8);
        let samples: Vec<V> = (0..=n)
            .map(|i| {
                let t = i as f32 / n as f32;
                let u = 1.0 - t;
                (
                    u * u * p0.0 + 2.0 * u * t * ctrl.0 + t * t * p2.0,
                    u * u * p0.1 + 2.0 * u * t * ctrl.1 + t * t * p2.1,
                )
            })
            .collect();
        let class = if k == 0 {
            ModernRoadClass::Arterial
        } else {
            ModernRoadClass::Collector
        };
        add_polyline(frame, &mut pts, &mut segs, &samples, class, ORIGIN_INNER);
    }

    // Regional roads enter through the outer ring and carry on as streets of
    // their own class, so the road seen from the air is the road in town.  The
    // planarisation pass below joins them to the grid wherever they cross it.
    for approach in approaches {
        for run in approach_runs(frame, &outer, &approach.path_km) {
            let run = tie_run_into_plan(frame, &outer, &pts, &segs, run);
            add_polyline(
                frame,
                &mut pts,
                &mut segs,
                &run,
                approach.class,
                ORIGIN_APPROACH,
            );
        }
    }

    // ---- 2. planarise: every crossing and T-junction becomes a node ----
    let in_river = |v: V| (v.0 - frame.river_x(v.1)).abs() < river_half + SNAP_M;
    planarize(&mut pts, &mut segs, &in_river);

    // ---- 3. river, local-street growth model ----
    // Local streets are the discretionary layer of a Chinese plan: outer
    // residential superblocks keep their interiors closed while the
    // commercial core holds a dense 支路 grid.  Each candidate segment runs
    // the ported growth model — accessibility, served demand, continuity,
    // block gain, construction cost and morphology decide probabilistically.
    let mut growth_rng = SplitMix64::new(seed as u64 ^ 0x6c6f_6361_6c73);
    let weights = ModelWeights::default();
    let mut local_segment_built = |centrality: f32| -> bool {
        let demand = (0.35 + 0.65 * centrality).clamp(0.15, 1.0) as f64;
        let mut growth = GrowthState::new(demand);
        let superblock_gain = 0.7 + (1.0 - centrality) * 0.9;
        let mut candidates = [
            growth.candidate(ActionKind::Extend, 0.25, 0.18, 0.12),
            growth.candidate(ActionKind::CloseBlock, 0.72, 0.34, 0.16),
            growth.candidate(ActionKind::Stop, 0.12, 0.0, 0.2),
        ];
        candidates[2].served_demand = 0.35 * demand;
        candidates[2].block_gain = superblock_gain as f64;
        let selected = sample_action(&candidates, weights, &mut growth_rng).unwrap_or(0);
        growth.apply(&candidates[selected]);
        candidates[selected].kind != ActionKind::Stop
    };
    let mut kept: Vec<(Seg, bool)> = Vec::with_capacity(segs.len());
    for s in &segs {
        let (p, q) = (pts[s.a], pts[s.b]);
        let river_hit = [0.0_f32, 0.25, 0.5, 0.75, 1.0].iter().any(|t| {
            let x = p.0 + (q.0 - p.0) * t;
            let z = p.1 + (q.1 - p.1) * t;
            (x - frame.river_x(z)).abs() < river_half
        });
        let major = matches!(
            s.class,
            ModernRoadClass::Expressway | ModernRoadClass::Arterial
        );
        if river_hit {
            // Only a street that actually crosses the channel becomes a bridge;
            // one that runs along or inside it has no business existing.
            let bank = |v: V| v.0 - frame.river_x(v.1);
            // Both ends must stand well clear of the channel: a junction box is
            // twenty to thirty metres deep, and one whose node is closer than that
            // to the bank hangs its asphalt over the water, leaving the bridge deck
            // stopping short of the far side.
            let clear = river_half;
            let crosses =
                (bank(p) < -clear && bank(q) > clear) || (bank(p) > clear && bank(q) < -clear);
            // A regional road that crosses the water outside town bridges it in
            // town too, whatever its class.
            let approach = s.origin & ORIGIN_APPROACH != 0;
            // A street that runs along the water has no business existing, but a
            // regional road must not vanish because the town's own river happens
            // to lie along it: it stays, as a causeway on the water.
            if !(major || approach) || (!crosses && !approach) {
                continue;
            }
        }
        // Beyond the built-up area there are no town streets: the grid lines and
        // the ring road that bounded the disc stop where the town does. A regional
        // road is not a town street and keeps running.
        let mid = ((p.0 + q.0) * 0.5, (p.1 + q.1) * 0.5);
        if frame.organic_footprint && s.origin & ORIGIN_APPROACH == 0 {
            let built = frame.urbanness(mid.0, mid.1);
            if s.origin & ORIGIN_OUTER != 0 || built < 0.10 {
                continue;
            }
        }
        // Local streets are discretionary; a regional road is not.
        if s.class == ModernRoadClass::Local && !river_hit && s.origin & ORIGIN_APPROACH == 0 {
            let centrality = frame.core_weight(mid.0, mid.1)
                * if frame.organic_footprint {
                    0.35 + 0.65 * frame.urbanness(mid.0, mid.1)
                } else {
                    1.0
                };
            if !local_segment_built(centrality) {
                continue;
            }
        }
        kept.push((*s, river_hit));
    }

    // Two bridges that cross over the water without a junction: keep the
    // higher class (a diagonal yields to the street it would cut).
    let mut drop = vec![false; kept.len()];
    for i in 0..kept.len() {
        if !kept[i].1 {
            continue;
        }
        for j in (i + 1)..kept.len() {
            if !kept[j].1 || drop[i] || drop[j] {
                continue;
            }
            let (si, sj) = (kept[i].0, kept[j].0);
            if si.a == sj.a || si.a == sj.b || si.b == sj.a || si.b == sj.b {
                continue;
            }
            if seg_intersect(pts[si.a], pts[si.b], pts[sj.a], pts[sj.b]).is_some() {
                let loser = if importance(si.class) < importance(sj.class) {
                    j
                } else if importance(sj.class) < importance(si.class) || si.origin == ORIGIN_INNER {
                    i
                } else {
                    j
                };
                drop[loser] = true;
            }
        }
    }
    let mut idx = 0;
    kept.retain(|_| {
        idx += 1;
        !drop[idx - 1]
    });

    // ---- 3a. no link shorter than its own junction boxes ----
    // The scene builder trims each end of a road by roughly 0.8 x the widest
    // street at that node, capped at 40 % of the road's length.  A link shorter
    // than the two boxes it joins leaves neither box its full size: the
    // carriageways of adjacent roads then no longer meet in a common corner, and
    // the kerbs cross instead of turning.  Contract such links into one node.
    loop {
        let mut degree = vec![0_u32; pts.len()];
        let mut widest = vec![0.0_f32; pts.len()];
        for (s, _) in &kept {
            for n in [s.a, s.b] {
                degree[n] += 1;
                widest[n] = widest[n].max(s.class.width_metres());
            }
        }
        // The junction radius the scene builder uses (the same function).
        let need = |n: usize| junction_trim_m(widest[n], degree[n] as usize);
        let victim = kept
            .iter()
            .enumerate()
            .filter(|(_, (_, bridge))| !*bridge)
            .map(|(i, (s, _))| {
                let len = (pts[s.a].0 - pts[s.b].0).hypot(pts[s.a].1 - pts[s.b].1);
                (i, len, ((need(s.a) + need(s.b)) * 1.1).max(28.0))
            })
            .filter(|(_, len, want)| len < want)
            .min_by(|a, b| (a.1 / a.2).total_cmp(&(b.1 / b.2)));
        let Some((i, _, _)) = victim else { break };
        let (s, _) = kept[i];
        // Keep the busier node where it is, so grid lines stay straight.
        let (keep, gone) = if degree[s.a] >= degree[s.b] {
            (s.a, s.b)
        } else {
            (s.b, s.a)
        };
        kept.remove(i);
        for (e, _) in kept.iter_mut() {
            if e.a == gone {
                e.a = keep;
            }
            if e.b == gone {
                e.b = keep;
            }
        }
        // Drop loops and merge parallel duplicates, keeping the wider class.
        kept.retain(|(e, _)| e.a != e.b);
        // Only edges now touching `keep` can have become duplicates.
        let mut first: std::collections::HashMap<(usize, usize), usize> =
            std::collections::HashMap::new();
        let mut dead = vec![false; kept.len()];
        for i in 0..kept.len() {
            let e = kept[i].0;
            if e.a != keep && e.b != keep {
                continue;
            }
            let key = (e.a.min(e.b), e.a.max(e.b));
            match first.get(&key) {
                Some(&k) => {
                    if importance(e.class) > importance(kept[k].0.class) {
                        kept[k] = kept[i];
                    }
                    dead[i] = true;
                }
                None => {
                    first.insert(key, i);
                }
            }
        }
        let mut idx = 0;
        kept.retain(|_| {
            idx += 1;
            !dead[idx - 1]
        });
    }

    // ---- 3b. tidy junctions ----
    // A diagonal that runs through (or near) a grid intersection leaves a node
    // with six or eight arms and slivers of 22-45 degrees between them.  No
    // junction box can be drawn cleanly from that: the crosswalks overlap and
    // the corner paving degenerates into spikes.  Keep at most four arms and no
    // two closer than 50 degrees, sacrificing the narrowest street each time.
    loop {
        let mut victim: Option<usize> = None;
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); pts.len()];
        for (i, (s, _)) in kept.iter().enumerate() {
            adj[s.a].push(i);
            adj[s.b].push(i);
        }
        'nodes: for n in 0..pts.len() {
            if adj[n].len() < 3 {
                continue;
            }
            let arms: Vec<(usize, f32)> = adj[n]
                .iter()
                .map(|&i| {
                    let s = &kept[i].0;
                    let o = if s.a == n { s.b } else { s.a };
                    (i, (pts[o].1 - pts[n].1).atan2(pts[o].0 - pts[n].0))
                })
                .collect();
            if arms.len() < 3 {
                continue;
            }
            let rank = |i: usize| {
                let (s, bridge) = &kept[i];
                (
                    (s.origin & ORIGIN_APPROACH != 0) as i32,
                    importance(s.class),
                    *bridge as i32,
                    -((pts[s.a].0 - pts[s.b].0).hypot(pts[s.a].1 - pts[s.b].1) * 10.0) as i32,
                )
            };
            let mut offenders: Vec<usize> = Vec::new();
            for x in 0..arms.len() {
                for y in (x + 1)..arms.len() {
                    let mut d = (arms[x].1 - arms[y].1).abs() % std::f32::consts::TAU;
                    if d > std::f32::consts::PI {
                        d = std::f32::consts::TAU - d;
                    }
                    if d < 40.0_f32.to_radians() {
                        offenders.push(arms[x].0);
                        offenders.push(arms[y].0);
                    }
                }
            }
            if offenders.is_empty() && arms.len() > 4 {
                offenders = arms.iter().map(|a| a.0).collect();
            }
            if let Some(&worst) = offenders.iter().min_by_key(|&&i| rank(i)) {
                victim = Some(worst);
                break 'nodes;
            }
        }
        match victim {
            Some(i) => {
                kept.remove(i);
            }
            None => break,
        }
    }

    // ---- 3c. straighten chains ----
    // Curved and warped streets were planned as chains of short samples.  A
    // bend node that barely bends carries no information but splits the street
    // into stubs too short to furnish, so drop it while the chord stays within
    // a few metres of the original line and the street stays short enough to
    // read as one block face.
    loop {
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); pts.len()];
        for (i, (e, _)) in kept.iter().enumerate() {
            adj[e.a].push(i);
            adj[e.b].push(i);
        }
        let mut joined = false;
        for n in 0..pts.len() {
            if adj[n].len() != 2 {
                continue;
            }
            let (i, j) = (adj[n][0], adj[n][1]);
            let ((e1, b1), (e2, b2)) = (kept[i], kept[j]);
            if b1 || b2 || e1.class != e2.class {
                continue;
            }
            let a = if e1.a == n { e1.b } else { e1.a };
            let b = if e2.a == n { e2.b } else { e2.a };
            if a == b {
                continue;
            }
            let (pa, pb) = (pts[a], pts[b]);
            let chord = (pb.0 - pa.0).hypot(pb.1 - pa.1);
            if chord > 140.0 || super::geom::point_seg_dist(pts[n], pa, pb) > 2.5 {
                continue;
            }
            if kept
                .iter()
                .any(|(e, _)| (e.a == a && e.b == b) || (e.a == b && e.b == a))
            {
                continue;
            }
            kept[i].0 = Seg { a, b, ..e1 };
            kept.remove(j);
            joined = true;
            break;
        }
        if !joined {
            break;
        }
    }

    // ---- 4. few cul-de-sacs, one connected network ----
    // A short local stub that hangs off a proper junction may survive as a dead
    // end (a seeded minority); every other dangling street is pruned.
    let mut protected = vec![false; pts.len()];
    {
        let mut degree = vec![0_u32; pts.len()];
        for (s, _) in &kept {
            degree[s.a] += 1;
            degree[s.b] += 1;
        }
        // A regional road runs on past the town's edge to where the regional
        // renderer takes over; its far end is not a dead end to be trimmed.
        for (s, _) in &kept {
            if s.origin & ORIGIN_APPROACH != 0 {
                for n in [s.a, s.b] {
                    if degree[n] == 1 {
                        protected[n] = true;
                    }
                }
            }
        }
        for (i, (s, bridge)) in kept.iter().enumerate() {
            let (leaf, other) = if degree[s.a] == 1 {
                (s.a, s.b)
            } else {
                (s.b, s.a)
            };
            let len = (pts[s.a].0 - pts[s.b].0).hypot(pts[s.a].1 - pts[s.b].1);
            if !*bridge
                && degree[leaf] == 1
                && degree[other] >= 3
                && s.class == ModernRoadClass::Local
                && len < 110.0
                && modern_hash(seed, i as i32, 9, 953) < 0.4
            {
                protected[leaf] = true;
            }
        }
    }
    loop {
        let mut degree = vec![0_u32; pts.len()];
        for (s, _) in &kept {
            degree[s.a] += 1;
            degree[s.b] += 1;
        }
        let before = kept.len();
        kept.retain(|(s, _)| {
            (degree[s.a] > 1 || protected[s.a]) && (degree[s.b] > 1 || protected[s.b])
        });
        if kept.len() == before {
            break;
        }
    }
    let mut parent: Vec<usize> = (0..pts.len()).collect();
    for (s, _) in &kept {
        let (ra, rb) = (find(&mut parent, s.a), find(&mut parent, s.b));
        if ra != rb {
            parent[rb] = ra;
        }
    }
    let mut size = vec![0_u32; pts.len()];
    for (s, _) in &kept {
        let root = find(&mut parent, s.a);
        size[root] += 1;
    }
    if let Some(main) = (0..pts.len()).max_by_key(|r| size[*r]) {
        // A regional road is kept even when it ends up apart from the town's
        // network (the town's own river may cut it off): it is a real road, and
        // dropping it would leave a gap between the regional road and the town.
        kept.retain(|(s, _)| find(&mut parent, s.a) == main || s.origin & ORIGIN_APPROACH != 0);
    }

    // ---- 5. emit SD / HD graph ----
    let mut new_id = vec![u32::MAX; pts.len()];
    let mut flags = vec![0_u8; pts.len()];
    let mut order: Vec<usize> = Vec::new();
    for (s, _) in &kept {
        for n in [s.a, s.b] {
            flags[n] |= s.origin;
            if new_id[n] == u32::MAX {
                new_id[n] = order.len() as u32;
                order.push(n);
            }
        }
    }
    let nodes: Vec<SdNode> = order
        .iter()
        .enumerate()
        .map(|(id, &n)| {
            let (x, z) = pts[n];
            let role = if flags[n] & ORIGIN_OUTER != 0 {
                "regional-gateway"
            } else if flags[n] & ORIGIN_INNER != 0 && flags[n] & ORIGIN_GRID != 0 {
                "grade-crossing"
            } else if x.abs() < block_m * 1.2 && z.abs() < block_m * 1.2 {
                "core-junction"
            } else {
                "district-junction"
            };
            SdNode {
                id: id as u32,
                point: frame.to_world(x, z),
                role: role.into(),
            }
        })
        .collect();
    let mut adjacency: Vec<Vec<(usize, usize)>> = vec![Vec::new(); pts.len()];
    for (i, (s, _)) in kept.iter().enumerate() {
        adjacency[s.a].push((s.b, i));
        adjacency[s.b].push((s.a, i));
    }
    let mut sd_roads = Vec::with_capacity(kept.len());
    let mut hd_roads = Vec::with_capacity(kept.len());
    for (id, (s, bridge)) in kept.iter().enumerate() {
        let id = id as u32;
        let (from, to) = (new_id[s.a], new_id[s.b]);
        sd_roads.push(SdRoad {
            id,
            from,
            to,
            class: s.class,
            bridge: *bridge,
        });
        let p0 = nodes[from as usize].point;
        let p1 = nodes[to as usize].point;
        hd_roads.push(HdRoad {
            id,
            sd_road: id,
            class: s.class,
            width_metres: s.class.width_metres(),
            median_metres: s.class.median_metres(),
            centreline: smooth_centreline(frame, &pts, &kept, &adjacency, id as usize, p0, p1),
            bridge: *bridge,
            layer: if *bridge { 1 } else { 0 },
            lanes: lanes_for_modern_road(id, s.class, &rules),
            connectors: Vec::new(),
        });
    }

    let morphology_score = MorphologyPrior::default().score(&GraphProbe {
        nodes: &nodes,
        sd_roads: &sd_roads,
        hd_roads: &hd_roads,
    });

    GraphOutput {
        nodes,
        sd_roads,
        hd_roads,
        river,
        morphology_score,
    }
}

/// The centreline of one street between two nodes. A node where only two streets
/// of the same class meet is a bend, not a junction, and drawing each street as a
/// straight chord puts a kink in the kerb at every bend: the road reads as a
/// zigzag. So a street runs on a cubic through its two ends whose tangent at a
/// bend is the direction through the bend, which makes the road continuous in
/// direction across it. Sharp turns, true junctions and bridge decks stay as the
/// straight chord.
fn smooth_centreline(
    frame: &CityFrame,
    pts: &[V],
    kept: &[(Seg, bool)],
    adjacency: &[Vec<(usize, usize)>],
    edge: usize,
    p0_world: Point,
    p1_world: Point,
) -> Vec<Point> {
    let (seg, bridge) = kept[edge];
    let (a, b) = (pts[seg.a], pts[seg.b]);
    let chord = (b.0 - a.0, b.1 - a.1);
    let length = chord.0.hypot(chord.1);
    let straight = || {
        vec![
            p0_world,
            Point {
                x_km: (p0_world.x_km + p1_world.x_km) * 0.5,
                y_km: (p0_world.y_km + p1_world.y_km) * 0.5,
            },
            p1_world,
        ]
    };
    if bridge || length < 12.0 {
        return straight();
    }
    // Unit direction through a bend node, in the sense of travel `from -> to` of
    // the street being drawn, or None where the node is a junction or a sharp turn.
    let through = |node: usize, toward: usize| -> Option<V> {
        let list = &adjacency[node];
        if list.len() != 2 {
            return None;
        }
        let ((u, eu), (v, ev)) = (list[0], list[1]);
        if kept[eu].1 || kept[ev].1 || kept[eu].0.class != kept[ev].0.class {
            return None;
        }
        let into = (pts[node].0 - pts[u].0, pts[node].1 - pts[u].1);
        let out = (pts[v].0 - pts[node].0, pts[v].1 - pts[node].1);
        let (li, lo) = (into.0.hypot(into.1), out.0.hypot(out.1));
        if li < 1.0e-3 || lo < 1.0e-3 {
            return None;
        }
        // Turns sharper than 35 degrees are corners, not bends.
        if (into.0 * out.0 + into.1 * out.1) / (li * lo) < 0.82 {
            return None;
        }
        let d = (pts[v].0 - pts[u].0, pts[v].1 - pts[u].1);
        let ld = d.0.hypot(d.1).max(1.0e-3);
        let d = (d.0 / ld, d.1 / ld);
        // `d` runs u -> v; flip it to point toward `toward`.
        if toward == v {
            Some(d)
        } else {
            Some((-d.0, -d.1))
        }
    };
    let dir = (chord.0 / length, chord.1 / length);
    let t0 = through(seg.a, seg.b).unwrap_or(dir);
    let t1 = through(seg.b, seg.a).map(|t| (-t.0, -t.1)).unwrap_or(dir);
    if t0 == dir && t1 == dir {
        return straight();
    }
    let k = length * 0.9;
    let (m0, m1) = ((t0.0 * k, t0.1 * k), (t1.0 * k, t1.1 * k));
    // Dense enough that the first and last chord run along the end tangent: with
    // a point only every 20 m the polyline still kinks at the bend by half its turn.
    let steps = ((length / 4.0).ceil() as usize).max(10);
    (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            let (t2, t3) = (t * t, t * t * t);
            let (h00, h10, h01, h11) = (
                2.0 * t3 - 3.0 * t2 + 1.0,
                t3 - 2.0 * t2 + t,
                -2.0 * t3 + 3.0 * t2,
                t3 - t2,
            );
            let x = h00 * a.0 + h10 * m0.0 + h01 * b.0 + h11 * m1.0;
            let z = h00 * a.1 + h10 * m0.1 + h01 * b.1 + h11 * m1.1;
            if i == 0 {
                p0_world
            } else if i == steps {
                p1_world
            } else {
                frame.to_world(x, z)
            }
        })
        .collect()
}

pub(super) fn modern_phase(seed: u32) -> f32 {
    modern_hash(seed, 3, 5, 751) * std::f32::consts::TAU
}

/// The original one-round mixer.  Kept verbatim for `hash_u32`, whose callers
/// (facade variants, rooftop plant) were tuned against its exact values.
pub(super) fn legacy_hash(seed: u32, x: i32, y: i32, salt: i32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (salt as u32).wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value as f32 / u32::MAX as f32
}

pub(super) fn modern_hash(seed: u32, x: i32, y: i32, salt: i32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (salt as u32).wrapping_mul(0xc2b2_ae35);
    // Full murmur3 finaliser: a one-bit change in the seed must reach the top
    // bits, or neighbouring seeds produce near-identical cities.
    value ^= value >> 16;
    value = value.wrapping_mul(0x85eb_ca6b);
    value ^= value >> 13;
    value = value.wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value as f32 / u32::MAX as f32
}
