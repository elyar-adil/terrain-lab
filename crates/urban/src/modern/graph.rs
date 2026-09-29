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

/// Which family a segment came from, kept through planarisation so junction
/// roles can still be assigned.
const ORIGIN_GRID: u8 = 4;
const ORIGIN_OUTER: u8 = 1;
const ORIGIN_INNER: u8 = 2;

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
    (t1 - t0 > MIN_EDGE_M)
        .then(|| ((p.0 + d.0 * t0, p.1 + d.1 * t0), (p.0 + d.0 * t1, p.1 + d.1 * t1)))
}

fn add_edge(pts: &mut Vec<V>, segs: &mut Vec<Seg>, a: V, b: V, class: ModernRoadClass, origin: u8) {
    let base = pts.len();
    pts.push(a);
    pts.push(b);
    segs.push(Seg { a: base, b: base + 1, class, origin });
}

fn add_ring(pts: &mut Vec<V>, segs: &mut Vec<Seg>, poly: &[V], class: ModernRoadClass, origin: u8) {
    let base = pts.len();
    pts.extend_from_slice(poly);
    for i in 0..poly.len() {
        segs.push(Seg { a: base + i, b: base + (i + 1) % poly.len(), class, origin });
    }
}

/// Offsets and classes of one family of parallel grid lines.  Lines that would
/// run alongside a ring edge are dropped; the ring itself carries that traffic.
fn grid_offsets(
    half: f32,
    block: f32,
    organic: f32,
    seed: u32,
    salt: i32,
    skip: &[f32],
) -> Vec<(f32, ModernRoadClass)> {
    let count = (half / block).ceil() as i32;
    let mut lines: Vec<(f32, ModernRoadClass)> = Vec::new();
    for i in -count..=count {
        let base = i as f32 * block;
        if base.abs() >= half {
            continue;
        }
        let jitter = if i.abs() <= 1 {
            0.0
        } else {
            (modern_hash(seed, i, salt, 733) - 0.5) * block * 0.07 * organic
        };
        let value = base + jitter;
        let class = if i.rem_euclid(5) == 0 {
            ModernRoadClass::Arterial
        } else if i.rem_euclid(2) == 0 {
            ModernRoadClass::Collector
        } else {
            ModernRoadClass::Local
        };
        if skip.iter().any(|a| (value.abs() - a).abs() < block * 0.35) {
            continue;
        }
        if lines.last().map(|(last, _)| (value - *last).abs() < block * 0.35).unwrap_or(false) {
            continue;
        }
        lines.push((value, class));
    }
    lines
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
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u))
        .then(|| (t, u, (a.0 + r.0 * t, a.1 + r.1 * t)))
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
        .map(|i| if used[i] { new_id[find(&mut parent, i)] } else { usize::MAX })
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
            if (s.class as i32) > (existing.class as i32) {
                existing.class = s.class;
                existing.origin = s.origin;
            }
        } else {
            out.push(Seg { a, b, class: s.class, origin: s.origin });
        }
    }
    *segs = out;
    merged
}

/// Split edges at T-junctions (a node within `SNAP_M` of another edge's side)
/// and at genuine interior crossings.  True if anything was split.
fn split_edges(pts: &mut Vec<V>, segs: &mut Vec<Seg>) -> bool {
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
            if d < SNAP_M {
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
            if t * li <= guard || (1.0 - t) * li <= guard || u * lj <= guard || (1.0 - u) * lj <= guard
            {
                continue;
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
            out.push(Seg { a: prev, b: n, ..*s });
            prev = n;
        }
        if prev != s.b {
            out.push(Seg { a: prev, b: s.b, ..*s });
        }
    }
    *segs = out;
    changed
}

fn planarize(pts: &mut Vec<V>, segs: &mut Vec<Seg>) {
    for _ in 0..8 {
        let merged = merge_close(pts, segs);
        let split = split_edges(pts, segs);
        if !merged && !split {
            break;
        }
    }
    merge_close(pts, segs);
}

pub(super) fn build_graph(frame: &CityFrame) -> GraphOutput {
    let radius_m = frame.radius_m;
    let block_m = frame.block_m;
    let organic = frame.organic;
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
    let inner = ring_polygon(inner_r);
    let cos_half = (std::f32::consts::PI / RING_SIDES as f32).cos();
    let skip = [radius_m * cos_half, inner_r * cos_half];
    let mut pts: Vec<V> = Vec::new();
    let mut segs: Vec<Seg> = Vec::new();
    for (x, class) in grid_offsets(radius_m, block_m, organic, seed, 11, &skip) {
        if let Some((a, b)) = clip_line(&outer, (x, 0.0), (0.0, 1.0)) {
            add_edge(&mut pts, &mut segs, a, b, class, ORIGIN_GRID);
        }
    }
    for (z, class) in grid_offsets(radius_m, block_m, organic, seed, 29, &skip) {
        if let Some((a, b)) = clip_line(&outer, (0.0, z), (1.0, 0.0)) {
            add_edge(&mut pts, &mut segs, a, b, class, ORIGIN_GRID);
        }
    }
    add_ring(&mut pts, &mut segs, &outer, ModernRoadClass::Expressway, ORIGIN_OUTER);
    add_ring(&mut pts, &mut segs, &inner, ModernRoadClass::Arterial, ORIGIN_INNER);

    // Which diagonal avenues through the historic core are realised is chosen by
    // the ported stochastic model over candidate corridors, like the source
    // kernel's regional-mobility choice set.
    let mut corridor_rng = SplitMix64::new(seed as u64 ^ 0x6469_6167);
    let corridor_growth = GrowthState::new(0.78);
    let candidate_specs: [Option<(bool, bool)>; 4] =
        [Some((true, false)), Some((false, true)), Some((true, true)), None];
    let candidates: Vec<crate::model::ActionCandidate> = candidate_specs
        .iter()
        .map(|spec| match spec {
            Some((true, true)) => corridor_growth.candidate(ActionKind::Connect, 0.95, 0.95, 0.18),
            Some(_) => corridor_growth.candidate(ActionKind::Connect, 0.95, 0.52, 0.18),
            None => corridor_growth.candidate(ActionKind::Stop, 0.0, 0.0, 0.0),
        })
        .collect();
    let chosen = sample_action(&candidates, ModelWeights::default(), &mut corridor_rng)
        .unwrap_or(candidate_specs.len() - 1);
    let root_half = std::f32::consts::FRAC_1_SQRT_2;
    if let Some(Some((rising, falling))) = candidate_specs.get(chosen) {
        for (enabled, dir) in
            [(*rising, (root_half, root_half)), (*falling, (root_half, -root_half))]
        {
            if !enabled {
                continue;
            }
            if let Some((a, b)) = clip_line(&inner, (0.0, 0.0), dir) {
                add_edge(&mut pts, &mut segs, a, b, ModernRoadClass::Arterial, ORIGIN_INNER);
            }
        }
    }

    // ---- 2. planarise: every crossing and T-junction becomes a node ----
    planarize(&mut pts, &mut segs);

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
        let major = matches!(s.class, ModernRoadClass::Expressway | ModernRoadClass::Arterial);
        if river_hit && !major {
            continue;
        }
        if s.class == ModernRoadClass::Local && !river_hit {
            let mid = ((p.0 + q.0) * 0.5, (p.1 + q.1) * 0.5);
            let centrality = (1.0 - mid.0.hypot(mid.1) / radius_m).clamp(0.0, 1.0);
            if !local_segment_built(centrality) {
                continue;
            }
        }
        kept.push((*s, river_hit));
    }

    // ---- 4. no cul-de-sacs, one connected network ----
    loop {
        let mut degree = vec![0_u32; pts.len()];
        for (s, _) in &kept {
            degree[s.a] += 1;
            degree[s.b] += 1;
        }
        let before = kept.len();
        kept.retain(|(s, _)| degree[s.a] > 1 && degree[s.b] > 1);
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
        kept.retain(|(s, _)| find(&mut parent, s.a) == main);
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
            SdNode { id: id as u32, point: frame.to_world(x, z), role: role.into() }
        })
        .collect();
    let mut sd_roads = Vec::with_capacity(kept.len());
    let mut hd_roads = Vec::with_capacity(kept.len());
    for (id, (s, bridge)) in kept.iter().enumerate() {
        let id = id as u32;
        let (from, to) = (new_id[s.a], new_id[s.b]);
        sd_roads.push(SdRoad { id, from, to, class: s.class, bridge: *bridge });
        let p0 = nodes[from as usize].point;
        let p1 = nodes[to as usize].point;
        hd_roads.push(HdRoad {
            id,
            sd_road: id,
            class: s.class,
            width_metres: s.class.width_metres(),
            median_metres: s.class.median_metres(),
            centreline: vec![
                p0,
                Point { x_km: (p0.x_km + p1.x_km) * 0.5, y_km: (p0.y_km + p1.y_km) * 0.5 },
                p1,
            ],
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

    GraphOutput { nodes, sd_roads, hd_roads, river, morphology_score }
}

pub(super) fn modern_phase(seed: u32) -> f32 {
    modern_hash(seed, 3, 5, 751) * std::f32::consts::TAU
}

pub(super) fn modern_hash(seed: u32, x: i32, y: i32, salt: i32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ (salt as u32).wrapping_mul(0xc2b2_ae35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value as f32 / u32::MAX as f32
}
