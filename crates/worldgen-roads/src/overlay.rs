//! Laying given roads over the generated network.
//!
//! A planner may hand the layer roads it routed itself (the long ones between
//! towns, following the terrain). They are kept exactly as given, and the
//! generated network is made to meet them: a generated street that crosses one
//! is cut there and the two share a junction, and a generated street that would
//! only run beside one is left out.
//!
//! Everything here is a function of one road and the given roads near it, never
//! of which window was asked for, so the cut points and the ids agree whichever
//! side of a tile border they are seen from.
//!
//! A given road is cut at the borders of the lattice cells (there is a plain
//! degree-two node there, a few kilometres apart) so that every piece belongs to
//! exactly one cell, and at every crossing with a generated street.

use std::sync::Arc;

use worldgen_contracts::{
    EdgeId, EdgeSource, NodeId, NodeKind, PinnedRoad, Polyline, RoadEdge, RoadNode, Setting, Span, SpanKind, V2, closest_on_segment,
    segment_intersection, v2,
};
use worldgen_core::hash::hash_words;
use worldgen_core::{Cell, Context, Dependency, Error, Layer, LayerId, Rect};

use crate::config::{Fields, RoadsConfig};
use crate::lattice::cell_size;
use crate::network::{CELLS, CellNetwork};
use crate::shape::{Border, clip_runs};

pub const OVERLAY: LayerId = LayerId("roads.overlay");

/// A generated street this close to a given road, for most of its length, only
/// duplicates it.
const SHADOW_M: f64 = 18.0;

pub struct OverlayLayer {
    pub config: RoadsConfig,
    pub fields: Fields,
}

struct Crossing {
    point: V2,
    arc: f64,
}

/// The given roads near a window, with only the segments that matter.
struct Near {
    road: Arc<PinnedRoad>,
    segments: Vec<(V2, V2)>,
}

impl Near {
    fn new(road: Arc<PinnedRoad>, window: Rect) -> Option<Near> {
        let segments: Vec<(V2, V2)> = road
            .path
            .0
            .windows(2)
            .filter(|w| {
                w[0].x.max(w[1].x) >= window.min[0]
                    && w[0].x.min(w[1].x) <= window.max[0]
                    && w[0].y.max(w[1].y) >= window.min[1]
                    && w[0].y.min(w[1].y) <= window.max[1]
            })
            .map(|w| (w[0], w[1]))
            .collect();
        (!segments.is_empty()).then_some(Near { road, segments })
    }
}

fn bounds_of(line: &Polyline) -> Rect {
    let (lo, hi) = line.bounds().unwrap_or((V2::ZERO, V2::ZERO));
    Rect { min: [lo.x, lo.y], max: [hi.x, hi.y] }
}

/// Where a generated street's line meets a given road, in order along the street.
fn crossings(line: &Polyline, near: &Near) -> Vec<Crossing> {
    let mut out: Vec<Crossing> = Vec::new();
    let mut run = 0.0;
    for w in line.0.windows(2) {
        let len = w[0].dist(w[1]);
        for &(a, b) in &near.segments {
            if let Some((point, t, _)) = segment_intersection(w[0], w[1], a, b) {
                out.push(Crossing { point, arc: run + t * len });
            }
        }
        run += len;
    }
    out.sort_by(|x, y| x.arc.partial_cmp(&y.arc).unwrap());
    // A crossing exactly at a vertex is found by both segments that share it.
    out.dedup_by(|b, a| a.point == b.point);
    out
}

fn distance_to(near: &Near, p: V2) -> f64 {
    near.segments.iter().map(|&(a, b)| closest_on_segment(a, b, p).2).fold(f64::INFINITY, f64::min)
}

/// The slice of a polyline between two arc lengths, with the end points given
/// exactly (they are nodes, whose positions are fixed elsewhere).
fn slice(line: &[V2], from: (f64, V2), to: (f64, V2)) -> Polyline {
    let mut out = vec![from.1];
    let mut run = 0.0;
    for w in line.windows(2) {
        run += w[0].dist(w[1]);
        if run > from.0 + 1e-9 && run < to.0 - 1e-9 {
            out.push(w[1]);
        }
    }
    out.push(to.1);
    Polyline(out)
}

impl OverlayLayer {
    fn reach(&self, cell_m: f64) -> f64 {
        (self.config.corner_jitter + 0.1) * cell_m + 2.0 * (self.config.warp_amp_m + self.config.wiggle_amp_m) + 40.0
    }

    /// The given roads that come near a generated street, with only the nearby segments.
    fn near_edge(&self, line: &Polyline) -> Vec<Near> {
        let b = bounds_of(line);
        let window = Rect { min: [b.min[0] - SHADOW_M, b.min[1] - SHADOW_M], max: [b.max[0] + SHADOW_M, b.max[1] + SHADOW_M] };
        self.fields
            .pinned
            .within(v2(window.min[0], window.min[1]), v2(window.max[0], window.max[1]))
            .into_iter()
            .filter_map(|r| Near::new(r, window))
            .collect()
    }

    /// Does this street just run beside a given road that is at least as big?
    fn is_shadow(&self, e: &RoadEdge, near: &[Near]) -> bool {
        let line = &e.pieces[0];
        let total = line.length();
        if total < 1.0 {
            return false;
        }
        let ends = [line.first().unwrap(), line.last().unwrap()];
        let n = (total / 10.0).ceil() as usize;
        let (mut counted, mut beside) = (0, 0);
        for k in 0..=n {
            let p = line.at(total * k as f64 / n as f64).unwrap().0;
            // The last stretch into a junction is supposed to converge.
            if ends.iter().any(|q| q.dist(p) < 35.0) {
                continue;
            }
            counted += 1;
            if near.iter().any(|r| r.road.class >= e.class && distance_to(r, p) < SHADOW_M) {
                beside += 1;
            }
        }
        counted > 0 && beside * 2 > counted
    }

    /// A generated street, cut where it crosses given roads; `None` if it is not built.
    fn street(&self, e: &RoadEdge) -> Option<Vec<(RoadEdge, [RoadNode; 2])>> {
        let line = &e.pieces[0];
        let near = self.near_edge(line);
        if near.is_empty() {
            return Some(Vec::new()); // caller keeps it as it is
        }
        if self.is_shadow(e, &near) {
            return None;
        }
        let mut stops: Vec<(f64, V2, NodeId)> = Vec::new();
        for r in &near {
            for (k, c) in crossings(line, r).into_iter().enumerate() {
                let id = NodeId(hash_words(&[r.road.id, e.id.0, k as u64, 0xC805]));
                stops.push((c.arc, c.point, id));
            }
        }
        stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        if stops.is_empty() {
            return Some(Vec::new());
        }
        let total = line.length();
        let mut chain = vec![(0.0, line.first().unwrap(), e.a)];
        chain.extend(stops);
        chain.push((total, line.last().unwrap(), e.b));
        let mut out = Vec::new();
        for w in chain.windows(2) {
            if w[0].2 == w[1].2 || w[0].1.dist(w[1].1) < 0.5 {
                continue;
            }
            let piece = slice(&line.0, (w[0].0, w[0].1), (w[1].0, w[1].1));
            let spans: Vec<Span> = e
                .spans
                .iter()
                .filter(|s| piece.closest(s.from).is_some_and(|(d, _)| d < 1e-6))
                .copied()
                .collect();
            let id = EdgeId::between(w[0].2, w[1].2, e.id.0);
            let nodes = [
                RoadNode { id: w[0].2, position: w[0].1, kind: NodeKind::Junction },
                RoadNode { id: w[1].2, position: w[1].1, kind: NodeKind::Junction },
            ];
            out.push((RoadEdge { id, a: w[0].2, b: w[1].2, spans, pieces: vec![piece], ..e.clone() }, nodes));
        }
        Some(out)
    }
}

/// Identity of the node where a given road crosses a lattice border.
fn border_node(road: u64, segment: usize, border: Border, cell: Cell) -> NodeId {
    let line = if border.axis == 0 { cell.x } else { cell.y } + i64::from(border.max);
    NodeId(hash_words(&[road, segment as u64, u64::from(border.axis), line as u64, 0xB0D]))
}

impl Layer for OverlayLayer {
    type Output = CellNetwork;

    fn id(&self) -> LayerId {
        OVERLAY
    }

    fn inputs(&self) -> Vec<Dependency> {
        vec![Dependency::required(CELLS)]
    }

    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<CellNetwork, Error> {
        let frame = ctx.frame();
        let rect = cell.rect(frame);
        let reach = self.reach(cell_size(frame, self.config.lattice_level));
        let window = rect.grown(reach);
        let pinned = self.fields.pinned.within(v2(window.min[0], window.min[1]), v2(window.max[0], window.max[1]));
        let own = ctx.input::<CellNetwork>(CELLS, cell)?;
        if pinned.is_empty() {
            return Ok((*own).clone());
        }

        let mut nodes: std::collections::BTreeMap<NodeId, RoadNode> = std::collections::BTreeMap::new();
        let mut edges: Vec<RoadEdge> = Vec::new();

        // Generated streets, cut where they cross a given road.
        for n in &own.nodes {
            nodes.insert(n.id, n.clone());
        }
        for e in &own.edges {
            match self.street(e) {
                None => {}
                Some(parts) if parts.is_empty() => edges.push(e.clone()),
                Some(parts) => {
                    for (edge, ends) in parts {
                        for n in ends {
                            nodes.entry(n.id).or_insert(n);
                        }
                        edges.push(edge);
                    }
                }
            }
        }

        // The given roads, cut at this cell's border and at the generated streets
        // that cross them. Streets of the neighbouring cells reach in, so read them too.
        let mut ring: Vec<Arc<CellNetwork>> = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                ring.push(ctx.input::<CellNetwork>(CELLS, cell.neighbour(dx, dy))?);
            }
        }
        for road in &pinned {
            for run in clip_runs(&road.path, rect) {
                let mut stops: Vec<(f64, V2, NodeId)> = Vec::new();
                let run_line = Polyline(run.points.clone());
                for net in &ring {
                    for e in &net.edges {
                        let b = bounds_of(&e.pieces[0]);
                        if b.max[0] < rect.min[0] || b.min[0] > rect.max[0] || b.max[1] < rect.min[1] || b.min[1] > rect.max[1] {
                            continue;
                        }
                        let around = self.near_edge(&e.pieces[0]);
                        let Some(mine) = around.iter().find(|r| r.road.id == road.id) else { continue };
                        if self.is_shadow(e, &around) {
                            continue;
                        }
                        for (k, c) in crossings(&e.pieces[0], mine).into_iter().enumerate() {
                            if !rect.contains(c.point.to_array()) {
                                continue;
                            }
                            if let Some((d, arc)) = run_line.closest(c.point) {
                                if d < 1e-6 {
                                    let id = NodeId(hash_words(&[road.id, e.id.0, k as u64, 0xC805]));
                                    stops.push((arc, c.point, id));
                                }
                            }
                        }
                    }
                }
                stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let total = run_line.length();
                let first = run.points[0];
                let last = *run.points.last().unwrap();
                let start_id = match run.entered {
                    Some(b) => border_node(road.id, run.first_segment, b, cell),
                    None => NodeId(hash_words(&[road.id, 0x57A12])),
                };
                let end_id = match run.left {
                    Some(b) => border_node(road.id, run.last_segment, b, cell),
                    None => NodeId(hash_words(&[road.id, 0xE4D])),
                };
                let mut chain = vec![(0.0, first, start_id)];
                chain.extend(stops);
                chain.push((total, last, end_id));
                for w in chain.windows(2) {
                    if w[0].2 == w[1].2 || w[0].1.dist(w[1].1) < 0.5 {
                        continue;
                    }
                    let piece = slice(&run.points, (w[0].0, w[0].1), (w[1].0, w[1].1));
                    let mid = piece.at(piece.length() * 0.5).map_or(w[0].1, |m| m.0);
                    let setting =
                        if self.fields.urban.urbanness(mid) >= self.config.urban_threshold { Setting::Urban } else { Setting::Rural };
                    let spans = given_road_spans(&*self.fields.water, &piece);
                    for (id, p) in [(w[0].2, w[0].1), (w[1].2, w[1].1)] {
                        nodes.entry(id).or_insert(RoadNode { id, position: p, kind: NodeKind::Junction });
                    }
                    edges.push(RoadEdge {
                        id: EdgeId::between(w[0].2, w[1].2, road.id),
                        a: w[0].2,
                        b: w[1].2,
                        class: road.class,
                        setting,
                        spans,
                        source: EdgeSource::Given,
                        pieces: vec![piece],
                    });
                }
            }
        }
        let referenced: std::collections::BTreeSet<NodeId> = edges.iter().flat_map(|e| [e.a, e.b]).collect();
        Ok(CellNetwork { nodes: nodes.into_values().filter(|n| referenced.contains(&n.id)).collect(), edges })
    }
}

/// A given road that meets water is carried over it: it was routed there on purpose.
fn given_road_spans(water: &dyn worldgen_contracts::WaterField, line: &Polyline) -> Vec<Span> {
    const PROBE_M: f64 = 4.0;
    const APPROACH_M: f64 = 12.0;
    let total = line.length();
    let n = (total / PROBE_M).ceil().max(1.0) as usize;
    let wet: Vec<f64> = (0..=n)
        .map(|k| total * k as f64 / n as f64)
        .filter(|&s| line.at(s).is_some_and(|(p, _)| water.is_water(p)))
        .collect();
    let (Some(&first), Some(&last)) = (wet.first(), wet.last()) else { return Vec::new() };
    if wet.len() <= 1 {
        return Vec::new();
    }
    let (Some((from, _)), Some((to, _))) = (line.at((first - APPROACH_M).max(0.0)), line.at((last + APPROACH_M).min(total))) else {
        return Vec::new();
    };
    vec![Span { kind: SpanKind::Bridge, from, to }]
}
