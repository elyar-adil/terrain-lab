//! Turning a quadrilateral's matched streets into nodes and edges.
//!
//! A cell builds the streets inside its own quadrilateral and the two chords it
//! owns (bottom and left). Chords are shared with a neighbour, so a division is
//! kept on a chord if *either* quadrilateral beside it uses it: that is why a cell
//! reads its lower and left neighbours as well as itself.

use std::collections::BTreeMap;

use worldgen_contracts::{
    EdgeId, EdgeSource, HeightField, NodeId, NodeKind, Polyline, RoadClass, RoadEdge, RoadNode, Setting, Span, SpanKind, V2,
    WaterField,
};
use worldgen_core::{Cell, Context, Dependency, Error, Layer, LayerId, Seed};

use crate::config::{Fields, LEVELS, RoadsConfig, TOP_RUNG};
use crate::lattice::{CHORDS, CellChords, Chord, corner, corner_id, fabric_seed};
use crate::quad::{QUADS, Quad, UsedNode};
use crate::shape::{bend, warp};

pub const CELLS: LayerId = LayerId("roads.cells");

/// Everything one lattice cell contributes, as whole edges (not yet clipped).
#[derive(Debug, Clone, PartialEq)]
pub struct CellNetwork {
    pub nodes: Vec<RoadNode>,
    pub edges: Vec<RoadEdge>,
}

pub struct CellLayer {
    pub config: RoadsConfig,
    pub fields: Fields,
}

/// A place on a road before the grid is bent.
#[derive(Clone, Copy)]
struct Vertex {
    id: NodeId,
    pos: V2,
}

fn level_class(level: u8) -> RoadClass {
    match TOP_RUNG.saturating_sub(level) {
        0 => RoadClass::Arterial,
        1 => RoadClass::Collector,
        2 => RoadClass::Local,
        3 => RoadClass::Service,
        _ => RoadClass::Track,
    }
}

fn demote(class: RoadClass, steps: u8) -> RoadClass {
    (0..steps).fold(class, |c, _| c.demoted())
}

/// How crooked a street of each class is allowed to be.
fn wiggle_factor(class: RoadClass) -> f64 {
    match class {
        RoadClass::Motorway | RoadClass::Arterial => 0.3,
        RoadClass::Collector => 0.6,
        RoadClass::Local => 1.0,
        RoadClass::Service => 1.4,
        RoadClass::Track => 1.8,
    }
}

struct Builder<'a> {
    fabric: Seed,
    cfg: &'a RoadsConfig,
    fields: &'a Fields,
    nodes: BTreeMap<NodeId, RoadNode>,
    edges: Vec<RoadEdge>,
}

impl Builder<'_> {
    fn node(&mut self, v: Vertex) {
        let position = warp(self.fabric, self.cfg, v.pos);
        self.nodes.entry(v.id).or_insert(RoadNode { id: v.id, position, kind: NodeKind::Junction });
    }

    /// One road between two vertices, if it can exist (it cannot, if it would
    /// have to cross water it has no bridge for).
    fn edge(&mut self, a: Vertex, b: Vertex, slot: u64, level: u8) {
        let mid = a.pos.lerp(b.pos, 0.5);
        let urbanness = self.fields.urban.urbanness(mid);
        // A street is only built where there is a town for it to serve. The layout
        // (which streets cross which) is decided for the whole face, so a street that
        // runs on into the fields is simply not built there, and ends.
        if level < TOP_RUNG && urbanness < 0.7 * LEVELS[usize::from(level)].min_urban {
            return;
        }
        // What a street is called follows its rung, but a small place does not have
        // arterials: how intense the development is steps the class down.
        let (class, setting) = if urbanness >= self.cfg.urban_threshold {
            let intensity = self.fields.urban.intensity(mid);
            let steps = if intensity >= 0.4 { 0 } else if intensity >= 0.12 { 1 } else { 2 };
            (demote(level_class(level), steps), Setting::Urban)
        } else {
            (level_class(level).demoted(), Setting::Rural)
        };
        let id = EdgeId::between(a.id, b.id, slot);
        let line = bend(self.fabric, self.cfg, a.pos, b.pos, id.0, self.cfg.wiggle_amp_m * wiggle_factor(class));
        if steepest_grade(&*self.fields.height, &line) > grade_limit(class) {
            return;
        }
        let Some(spans) = water_spans(&*self.fields.water, &line, class) else {
            return;
        };
        self.node(a);
        self.node(b);
        self.edges.push(RoadEdge {
            id,
            a: a.id,
            b: b.id,
            class,
            setting,
            spans,
            source: EdgeSource::Generated,
            pieces: vec![line],
        });
    }

    /// A chain of vertices as consecutive roads.
    fn chain(&mut self, vertices: &[Vertex], slot: u64, level: u8) {
        for w in vertices.windows(2) {
            if w[0].pos.dist(w[1].pos) > 1.0 {
                self.edge(w[0], w[1], slot, level);
            }
        }
    }
}

/// The steepest grade (rise over run) a class of road is built to. Switchbacks
/// are not modelled, so a road that would have to climb more than this is not built.
fn grade_limit(class: RoadClass) -> f64 {
    match class {
        RoadClass::Motorway => 0.05,
        RoadClass::Arterial => 0.07,
        RoadClass::Collector => 0.09,
        RoadClass::Local => 0.12,
        RoadClass::Service => 0.16,
        RoadClass::Track => 0.25,
    }
}

/// The steepest grade along a road, over stretches about 50 m long: a short
/// steep step does not stop a road, a long climb does.
fn steepest_grade(height: &dyn HeightField, line: &Polyline) -> f64 {
    const STRETCH_M: f64 = 50.0;
    let total = line.length();
    if total < 1.0 {
        return 0.0;
    }
    let n = (total / STRETCH_M).ceil().max(1.0) as usize;
    let heights: Vec<f64> = (0..=n).map(|k| line.at(total * k as f64 / n as f64).map_or(0.0, |(p, _)| height.height_m(p))).collect();
    let run = total / n as f64;
    heights.windows(2).map(|w| (w[1] - w[0]).abs() / run).fold(0.0, f64::max)
}

/// Where a road meets water. `None` means it must not exist (a small road with no
/// way across); an empty list means it stays on the ground.
fn water_spans(water: &dyn WaterField, line: &Polyline, class: RoadClass) -> Option<Vec<Span>> {
    const PROBE_M: f64 = 4.0;
    const APPROACH_M: f64 = 12.0;
    let total = line.length();
    let n = (total / PROBE_M).ceil() as usize;
    let (mut first, mut last, mut wet) = (None, None, 0usize);
    for k in 0..=n {
        let s = total * k as f64 / n.max(1) as f64;
        if line.at(s).is_some_and(|(p, _)| water.is_water(p)) {
            first.get_or_insert(s);
            last = Some(s);
            wet += 1;
        }
    }
    let (Some(first), Some(last)) = (first, last) else {
        return Some(Vec::new());
    };
    if wet as f64 * PROBE_M <= PROBE_M {
        return Some(Vec::new()); // a ditch: a culvert, not a bridge
    }
    if class < RoadClass::Collector {
        return None;
    }
    let from = line.at((first - APPROACH_M).max(0.0))?.0;
    let to = line.at((last + APPROACH_M).min(total))?.0;
    Some(vec![Span { kind: SpanKind::Bridge, from, to }])
}

impl Layer for CellLayer {
    type Output = CellNetwork;

    fn id(&self) -> LayerId {
        CELLS
    }

    fn inputs(&self) -> Vec<Dependency> {
        vec![Dependency::required(QUADS), Dependency::required(CHORDS)]
    }

    fn collapse(&self, ctx: &Context<'_>, cell: Cell) -> Result<CellNetwork, Error> {
        let fabric = fabric_seed(ctx);
        let (i, j) = (cell.x, cell.y);
        let quad = ctx.input::<Quad>(QUADS, cell)?;
        let below = ctx.input::<Quad>(QUADS, cell.neighbour(0, -1))?;
        let left = ctx.input::<Quad>(QUADS, cell.neighbour(-1, 0))?;
        let own = ctx.input::<CellChords>(CHORDS, cell)?;
        let mut b = Builder {
            fabric,
            cfg: &self.config,
            fields: &self.fields,
            nodes: BTreeMap::new(),
            edges: Vec::new(),
        };
        let frame = ctx.frame();

        // Streets across the interior.
        interior(&mut b, &quad);

        // The two chords this cell owns. A division is a junction if either side uses it.
        let c00 = Vertex { id: corner_id(fabric, i, j), pos: corner(fabric, &self.config, frame, i, j) };
        let c10 = Vertex { id: corner_id(fabric, i + 1, j), pos: corner(fabric, &self.config, frame, i + 1, j) };
        let c01 = Vertex { id: corner_id(fabric, i, j + 1), pos: corner(fabric, &self.config, frame, i, j + 1) };
        chord_edges(&mut b, &own.bottom, c00, c10, &quad.used_bottom, &below.used_top);
        chord_edges(&mut b, &own.left, c00, c01, &quad.used_left, &left.used_right);

        Ok(CellNetwork { nodes: b.nodes.into_values().collect(), edges: b.edges })
    }
}

fn chord_edges(b: &mut Builder<'_>, chord: &Chord, from: Vertex, to: Vertex, side_a: &[UsedNode], side_b: &[UsedNode]) {
    // A node is a junction if a street of either neighbouring quadrilateral uses it.
    let mut used: Vec<&UsedNode> = side_a.iter().chain(side_b).collect();
    used.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap().then(x.id.cmp(&y.id)));
    used.dedup_by(|y, x| x.id == y.id);
    let mut chain = vec![from];
    chain.extend(used.into_iter().map(|u| Vertex { id: u.id, pos: u.pos }));
    chain.push(to);
    b.chain(&chain, chord.id, TOP_RUNG);
}

fn interior(b: &mut Builder<'_>, quad: &Quad) {
    for e in &quad.edges {
        b.edge(Vertex { id: e.a, pos: e.a_pos }, Vertex { id: e.b, pos: e.b_pos }, e.slot, e.level);
    }
}
