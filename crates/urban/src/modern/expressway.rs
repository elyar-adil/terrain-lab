//! Expressways (城市快速路) and the interchanges that are their only way in.
//!
//! An urban expressway is fully access-controlled (CJJ 129): no street meets it
//! at grade, so it crosses every street on a structure, and traffic joins and
//! leaves it only by ramps on the right. Where it crosses an arterial it gets an
//! interchange; everything else passes underneath.
//!
//! The plan is laid out in two passes around the block layout:
//!
//! 1. **Before the blocks** ([`lay_out`]): the corridor and the ramps go into
//!    the street graph like any other street, split where they cross, so the
//!    blocks are cut around the land they take and no lot is laid under a deck.
//! 2. **After the blocks** ([`grade_separate`]): every node where the expressway
//!    or a ramp only passes a street is taken apart. The street keeps the node;
//!    the expressway gets one of its own at the same place, on layer 1.
//!
//! The interchange is a diamond (菱形立交), the form CJJ 152 gives for an
//! expressway over an arterial and the one in the reference photographs of
//! Guangzhou and Shenzhen: on each side the off-ramp leaves the deck before the
//! arterial, comes down beside the expressway and meets the arterial at a
//! signalised terminal; the on-ramp leaves the same terminal and climbs back to
//! the deck after it.

use std::collections::HashMap;

use super::CityFrame;
use super::geom::V;
use super::roads;
use crate::model::jurisdiction::TrafficRules;
use crate::{HdLane, HdRoad, LaneUse, ModernRoadClass, Movement, SdNode, SdRoad};

/// Distance along the expressway from the crossing to each ramp's end on the
/// deck: a 70 m taper beside the deck, a short level run past the nose, the
/// 9 m fall at 4.5 % with its vertical curves (240 m), and a landing and the
/// terminal's own box before the arterial.
const RAMP_REACH_M: f32 = 420.0;
/// Distance along the arterial from the crossing to each ramp terminal: the
/// half width of the deck, the terminal's own box, and room to queue between.
const TERMINAL_M: f32 = 58.0;
/// The ramp runs beside the deck for this long before it splays away from it.
const TAPER_M: f32 = 70.0;
/// Interchanges are at least this far apart along the expressway.
const SPACING_M: f32 = 1_050.0;
/// No other street may cross the expressway this close to an interchange, or
/// the ramps would come down on it.
const LANDING_CLEAR_M: f32 = 150.0;
/// A ramp may cross a street only where it is still this high above it.
const RAMP_CLEARANCE_M: f32 = 6.0;
/// The deck height the ramp profile falls from, and its grade.
const DECK_M: f32 = 9.0;
const RAMP_GRADE: f32 = 0.045;
/// Half width of a one-lane ramp deck (`ramp_cross_section`).
const RAMP_HALF_M: f32 = 4.25;
/// A ramp is cut into pieces no longer than this, so the blocks follow its curve.
const PIECE_M: f32 = 60.0;

/// One street of the working graph, in local metres.
#[derive(Clone, Debug)]
struct Road {
    from: usize,
    to: usize,
    class: ModernRoadClass,
    bridge: bool,
    oneway: bool,
    line: Vec<V>,
}

/// The plan's street graph in local metres, while the expressway is laid into it.
struct Graph {
    pts: Vec<V>,
    roles: Vec<String>,
    roads: Vec<Road>,
}

/// What the second pass needs from the first.
#[derive(Debug, Default)]
pub(super) struct Layout {
    /// Nodes where a ramp only passes a street (it is elevated there).
    detach: Vec<usize>,
    /// Interchanges built. An expressway with none would be cut off from the
    /// town if it were lifted off its streets, so it stays at grade.
    interchanges: usize,
    /// Nodes where a ramp's own deck begins beside the main deck. For the
    /// blocks the ramp reaches its deck node by a short taper edge from here;
    /// once they are cut, the taper is dropped and the ramp meets the deck node
    /// itself, its line still starting beside the deck.
    tapers: Vec<usize>,
}

fn sub(a: V, b: V) -> V {
    (a.0 - b.0, a.1 - b.1)
}
fn add(a: V, b: V) -> V {
    (a.0 + b.0, a.1 + b.1)
}
fn mul(a: V, k: f32) -> V {
    (a.0 * k, a.1 * k)
}
fn dot(a: V, b: V) -> f32 {
    a.0 * b.0 + a.1 * b.1
}
fn cross(a: V, b: V) -> f32 {
    a.0 * b.1 - a.1 * b.0
}
fn len(a: V) -> f32 {
    a.0.hypot(a.1)
}
fn unit(a: V) -> V {
    let l = len(a).max(1.0e-6);
    (a.0 / l, a.1 / l)
}
/// The kerb side of travel along `t`: right-hand traffic drives on the
/// direction's left normal in this frame (see `RoadCrossSection::lane_offset`).
fn kerb_side(t: V) -> V {
    (-t.1, t.0)
}

fn polyline_len(line: &[V]) -> f32 {
    line.windows(2).map(|w| len(sub(w[1], w[0]))).sum()
}

/// Point and unit tangent at `station` along `line`, clamped to its ends.
fn sample(line: &[V], station: f32) -> (V, V) {
    let mut walked = 0.0;
    let last = line.len().saturating_sub(2);
    for (i, w) in line.windows(2).enumerate() {
        let l = len(sub(w[1], w[0]));
        if walked + l >= station || i == last {
            let t = ((station - walked) / l.max(1.0e-6)).clamp(0.0, 1.0);
            return (add(w[0], mul(sub(w[1], w[0]), t)), unit(sub(w[1], w[0])));
        }
        walked += l;
    }
    (line[0], (1.0, 0.0))
}

/// Station of the nearest point of `line` to `p`, and the signed lateral offset
/// toward the kerb side.
fn project(line: &[V], p: V) -> (f32, f32) {
    let mut best = (f32::MAX, 0.0, 0.0);
    let mut walked = 0.0;
    for w in line.windows(2) {
        let d = sub(w[1], w[0]);
        let l = len(d).max(1.0e-6);
        let t = (dot(sub(p, w[0]), d) / (l * l)).clamp(0.0, 1.0);
        let q = add(w[0], mul(d, t));
        let gap = len(sub(p, q));
        if gap < best.0 {
            best = (gap, walked + t * l, dot(sub(p, q), kerb_side(unit(d))));
        }
        walked += l;
    }
    (best.1, best.2)
}

/// Where segment `a-b` crosses segment `c-d`, as fractions along each.
fn intersect(a: V, b: V, c: V, d: V) -> Option<(f32, f32)> {
    let (r, s) = (sub(b, a), sub(d, c));
    let denom = cross(r, s);
    if denom.abs() < 1.0e-9 {
        return None;
    }
    let qp = sub(c, a);
    let t = cross(qp, s) / denom;
    let u = cross(qp, r) / denom;
    ((0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)).then_some((t, u))
}

/// The piece of `line` between two stations.
fn slice(line: &[V], from: f32, to: f32) -> Vec<V> {
    let mut out = vec![sample(line, from).0];
    let mut walked = 0.0;
    for w in line.windows(2) {
        walked += len(sub(w[1], w[0]));
        if walked > from + 0.01 && walked < to - 0.01 {
            out.push(w[1]);
        }
    }
    out.push(sample(line, to).0);
    out
}

impl Graph {
    fn new(frame: &CityFrame, nodes: &[SdNode], sd: &[SdRoad], hd: &[HdRoad]) -> Self {
        let pts = nodes.iter().map(|n| frame.to_local(n.point)).collect();
        let roles = nodes.iter().map(|n| n.role.clone()).collect();
        let hd_by_id: HashMap<u32, &HdRoad> = hd.iter().map(|r| (r.id, r)).collect();
        let roads = sd
            .iter()
            .map(|r| {
                let line = hd_by_id
                    .get(&r.id)
                    .map(|h| h.centreline.iter().map(|p| frame.to_local(*p)).collect())
                    .unwrap_or_default();
                Road {
                    from: r.from as usize,
                    to: r.to as usize,
                    class: r.class,
                    bridge: r.bridge,
                    oneway: hd_by_id.get(&r.id).is_some_and(|h| h.oneway),
                    line,
                }
            })
            .collect();
        Self { pts, roles, roads }
    }

    fn node(&mut self, p: V, role: &str) -> usize {
        self.pts.push(p);
        self.roles.push(role.into());
        self.pts.len() - 1
    }

    fn incident(&self, node: usize) -> Vec<usize> {
        (0..self.roads.len())
            .filter(|&i| self.roads[i].from == node || self.roads[i].to == node)
            .collect()
    }

    /// Split road `index` at the given stations, each a node to meet there.
    fn split(&mut self, index: usize, mut cuts: Vec<(f32, usize)>) {
        let road = self.roads[index].clone();
        let total = polyline_len(&road.line);
        cuts.retain(|(s, _)| *s > 0.5 && *s < total - 0.5);
        cuts.sort_by(|a, b| a.0.total_cmp(&b.0));
        cuts.dedup_by(|a, b| a.1 == b.1);
        if cuts.is_empty() {
            return;
        }
        let mut pieces = Vec::new();
        let (mut from, mut at) = (road.from, 0.0);
        for (station, node) in &cuts {
            let mut line = slice(&road.line, at, *station);
            *line.last_mut().expect("a slice has two ends") = self.pts[*node];
            pieces.push(Road {
                from,
                to: *node,
                line,
                ..road.clone()
            });
            from = *node;
            at = *station;
        }
        let mut line = slice(&road.line, at, total);
        line[0] = self.pts[from];
        pieces.push(Road {
            from,
            to: road.to,
            line,
            ..road
        });
        self.roads[index] = pieces.remove(0);
        self.roads.extend(pieces);
    }

    /// Every place `line` crosses a street that `crosses` accepts: the street is
    /// split there (or met at its own end, if the crossing is that close to it),
    /// and the crossing is returned as (station along `line`, node).
    fn cross(&mut self, line: &[V], crosses: impl Fn(&Road) -> bool) -> Vec<(f32, usize)> {
        // (road, station on the road, station on the line, point)
        let mut hits: Vec<(usize, f32, f32, V)> = Vec::new();
        for (index, road) in self.roads.iter().enumerate() {
            if road.line.len() < 2 || !crosses(road) {
                continue;
            }
            let mut along_road = 0.0;
            for w in road.line.windows(2) {
                let seg = len(sub(w[1], w[0]));
                let mut along_line = 0.0;
                for v in line.windows(2) {
                    let lseg = len(sub(v[1], v[0]));
                    if let Some((t, u)) = intersect(w[0], w[1], v[0], v[1]) {
                        let point = add(v[0], mul(sub(v[1], v[0]), u));
                        hits.push((index, along_road + t * seg, along_line + u * lseg, point));
                    }
                    along_line += lseg;
                }
                along_road += seg;
            }
        }
        let mut found: Vec<(f32, usize)> = Vec::new();
        let mut cuts: HashMap<usize, Vec<(f32, usize)>> = HashMap::new();
        for (index, at_road, at_line, point) in hits {
            let (from, to, total) = {
                let road = &self.roads[index];
                (road.from, road.to, polyline_len(&road.line))
            };
            // Meet the street at its own end when the crossing is within a few
            // metres of it, and at one node where two of its segments meet.
            let node = if at_road < 6.0 {
                from
            } else if at_road > total - 6.0 {
                to
            } else if let Some(&(_, n)) = cuts
                .get(&index)
                .and_then(|list| list.iter().find(|(s, _)| (s - at_road).abs() < 1.0))
            {
                n
            } else {
                let n = self.node(point, "district-junction");
                cuts.entry(index).or_default().push((at_road, n));
                n
            };
            found.push((at_line, node));
        }
        for (index, list) in cuts {
            self.split(index, list);
        }
        found.sort_by(|a, b| a.0.total_cmp(&b.0));
        found.dedup_by(|a, b| a.1 == b.1);
        found
    }

    /// Cut road `index` into pieces no longer than `max`, at nodes of its own.
    fn densify(&mut self, index: usize, max: f32) {
        let total = polyline_len(&self.roads[index].line);
        let pieces = (total / max).ceil() as usize;
        if pieces < 2 {
            return;
        }
        let cuts: Vec<(f32, usize)> = (1..pieces)
            .map(|k| {
                let s = total * k as f32 / pieces as f32;
                let p = sample(&self.roads[index].line, s).0;
                (s, self.node(p, "viaduct"))
            })
            .collect();
        self.split(index, cuts);
    }

    /// Lay a one-way chain from node `from` to node `to` along `line`, crossing
    /// whatever it crosses, in pieces short enough for the blocks to follow.
    /// Returns the crossing nodes.
    fn lay_ramp(
        &mut self,
        from: usize,
        to: usize,
        line: &[V],
        tapers: &mut Vec<usize>,
    ) -> Vec<usize> {
        // Where the line does not start (or end) at its node, which is a deck
        // node on the expressway's axis, the ramp's deck begins beside the main
        // deck: a node there, and a taper edge to the deck node.
        let (mut from, mut to) = (from, to);
        let n = line.len();
        if len(sub(line[0], self.pts[from])) > 1.0 {
            let start = self.node(line[0], "ramp-deck");
            self.roads.push(Road {
                from,
                to: start,
                class: ModernRoadClass::Expressway,
                bridge: false,
                oneway: true,
                line: vec![self.pts[from], line[0]],
            });
            tapers.push(start);
            from = start;
        }
        if len(sub(line[n - 1], self.pts[to])) > 1.0 {
            let end = self.node(line[n - 1], "ramp-deck");
            self.roads.push(Road {
                from: end,
                to,
                class: ModernRoadClass::Expressway,
                bridge: false,
                oneway: true,
                line: vec![line[n - 1], self.pts[to]],
            });
            tapers.push(end);
            to = end;
        }
        let total = polyline_len(line);
        // Its own ends are where it meets the streets it serves, not crossings.
        let crossings: Vec<(f32, usize)> = self
            .cross(line, |r| r.class != ModernRoadClass::Expressway)
            .into_iter()
            .filter(|&(s, n)| s > 1.0 && s < total - 1.0 && n != from && n != to)
            .collect();
        let mut stops: Vec<(f32, usize)> = crossings.clone();
        let pieces = (total / PIECE_M).ceil().max(1.0) as usize;
        for k in 1..pieces {
            let s = total * k as f32 / pieces as f32;
            if stops.iter().all(|(c, _)| (c - s).abs() > 15.0) {
                let p = sample(line, s).0;
                let n = self.node(p, "ramp");
                stops.push((s, n));
            }
        }
        stops.sort_by(|a, b| a.0.total_cmp(&b.0));
        let (mut at_node, mut at) = (from, 0.0);
        for (s, n) in stops.iter().copied().chain(std::iter::once((total, to))) {
            // The ends stay where the line has them: a ramp starts beside the
            // deck, not on its axis where the node is.
            let piece = slice(line, at, s);
            self.roads.push(Road {
                from: at_node,
                to: n,
                class: ModernRoadClass::Expressway,
                bridge: false,
                oneway: true,
                line: piece,
            });
            at_node = n;
            at = s;
        }
        crossings.into_iter().map(|(_, n)| n).collect()
    }

    fn into_plan(
        self,
        frame: &CityFrame,
        rules: &TrafficRules,
    ) -> (Vec<SdNode>, Vec<SdRoad>, Vec<HdRoad>) {
        let nodes = self
            .pts
            .iter()
            .zip(&self.roles)
            .enumerate()
            .map(|(id, (p, role))| SdNode {
                id: id as u32,
                point: frame.to_world(p.0, p.1),
                role: role.clone(),
            })
            .collect();
        let mut sd = Vec::with_capacity(self.roads.len());
        let mut hd = Vec::with_capacity(self.roads.len());
        for road in self.roads {
            if road.from == road.to || road.line.len() < 2 {
                continue;
            }
            let id = sd.len() as u32;
            sd.push(SdRoad {
                id,
                from: road.from as u32,
                to: road.to as u32,
                class: road.class,
                bridge: road.bridge,
            });
            hd.push(hd_road(id, &road, frame, rules));
        }
        (nodes, sd, hd)
    }
}

fn hd_road(id: u32, road: &Road, frame: &CityFrame, rules: &TrafficRules) -> HdRoad {
    let section = if road.oneway {
        crate::ramp_cross_section()
    } else {
        crate::cross_section(road.class)
    };
    HdRoad {
        id,
        sd_road: id,
        class: road.class,
        width_metres: section.width_metres,
        median_metres: section.median_metres,
        centreline: road.line.iter().map(|p| frame.to_world(p.0, p.1)).collect(),
        bridge: road.bridge,
        layer: i8::from(road.bridge),
        oneway: road.oneway,
        lanes: if road.oneway {
            ramp_lanes(id)
        } else {
            roads::lanes_for_modern_road(id, road.class, rules)
        },
        connectors: Vec::new(),
    }
}

/// The single lane of a ramp, centred on its deck.
fn ramp_lanes(road_id: u32) -> Vec<HdLane> {
    let section = crate::ramp_cross_section();
    vec![HdLane {
        id: format!("road/{road_id}/lane/1/0"),
        direction: 1,
        index_from_curb: 0,
        use_type: LaneUse::Through,
        width_metres: section.motor_lane_width,
        offset_metres: 0.0,
        allowed_movements: vec![Movement::Through],
        markings: Vec::new(),
    }]
}

/// The main line of an expressway around a node: its centreline as one
/// polyline, the station of the node on it, and the main-line pieces in order.
struct Chain {
    line: Vec<V>,
    at: f32,
    /// (road index, station of its start on `line`, whether it runs forward).
    pieces: Vec<(usize, f32, bool)>,
    /// Stations of the nodes the chain passes.
    nodes: Vec<(f32, usize)>,
}

fn main_line(road: &Road) -> bool {
    road.class == ModernRoadClass::Expressway && !road.oneway
}

/// Walk the expressway main line out of `node` both ways, up to `reach` metres.
fn chain_at(graph: &Graph, node: usize, reach: f32) -> Option<Chain> {
    let arms: Vec<usize> = graph
        .incident(node)
        .into_iter()
        .filter(|&i| main_line(&graph.roads[i]))
        .collect();
    let [first, second] = arms.as_slice() else {
        return None;
    };
    // One direction: a list of (road, forward) leading away from `node`.
    let walk = |start: usize| {
        let mut out: Vec<(usize, bool)> = Vec::new();
        let (mut road, mut at, mut walked) = (start, node, 0.0);
        loop {
            let r = &graph.roads[road];
            let forward = r.from == at;
            out.push((road, forward));
            walked += polyline_len(&r.line);
            let next = if forward { r.to } else { r.from };
            if walked >= reach || next == node {
                break;
            }
            let onward: Vec<usize> = graph
                .incident(next)
                .into_iter()
                .filter(|&i| i != road && main_line(&graph.roads[i]))
                .collect();
            let [only] = onward.as_slice() else {
                break;
            };
            if out.iter().any(|(r, _)| r == only) {
                break;
            }
            road = *only;
            at = next;
        }
        out
    };
    let back = walk(*first);
    let ahead = walk(*second);
    let mut line: Vec<V> = Vec::new();
    let mut pieces = Vec::new();
    let mut nodes = Vec::new();
    // The back half, reversed so the chain runs toward `node`.
    for &(road, forward) in back.iter().rev() {
        let r = &graph.roads[road];
        let mut pts = r.line.clone();
        // Away from `node` it runs `forward`; toward it, the other way.
        if forward {
            pts.reverse();
        }
        let start = polyline_len(&line);
        let start_node = if forward { r.to } else { r.from };
        nodes.push((start, start_node));
        if !line.is_empty() {
            pts.remove(0);
        }
        line.extend(pts);
        pieces.push((road, start, !forward));
    }
    let at = polyline_len(&line);
    nodes.push((at, node));
    for &(road, forward) in &ahead {
        let r = &graph.roads[road];
        let mut pts = r.line.clone();
        if !forward {
            pts.reverse();
        }
        let start = polyline_len(&line);
        pts.remove(0);
        line.extend(pts);
        pieces.push((road, start, forward));
        let end_node = if forward { r.to } else { r.from };
        nodes.push((polyline_len(&line), end_node));
    }
    Some(Chain {
        line,
        at,
        pieces,
        nodes,
    })
}

/// Cut the chain's main line at `station`, returning the node there.
fn cut_chain(graph: &mut Graph, chain: &Chain, station: f32, role: &str) -> Option<usize> {
    let (index, start, forward) = chain
        .pieces
        .iter()
        .copied()
        .rev()
        .find(|(_, start, _)| *start <= station)?;
    let r = &graph.roads[index];
    let total = polyline_len(&r.line);
    let local = station - start;
    let along = if forward { local } else { total - local };
    if total < 6.0 {
        return None;
    }
    // A station on top of the piece's own end moves just off it, so the node
    // made here is never one already on the line.
    let along = along.clamp(2.0, total - 2.0);
    let p = sample(&r.line, along).0;
    let n = graph.node(p, role);
    graph.split(index, vec![(along, n)]);
    Some(n)
}

/// The node at the far end of `road` from `node`, and the road's line from `node`.
fn arm(graph: &Graph, road: usize, node: usize) -> (usize, Vec<V>, bool) {
    let r = &graph.roads[road];
    let mut line = r.line.clone();
    let forward = r.from == node;
    if !forward {
        line.reverse();
    }
    (if forward { r.to } else { r.from }, line, forward)
}

/// The height of a ramp `v` metres before the ground end it falls to.
fn ramp_height(v: f32) -> f32 {
    // The landing and the terminal's box, then the fall (`RampProfile` in
    // city-scene draws the same long section).
    ((v - 50.0) * RAMP_GRADE).clamp(0.0, DECK_M)
}

/// One diamond, fully planned before anything is changed.
struct Diamond {
    node: usize,
    chain: Chain,
    /// The two arms of the street across: (road index, or `None` for the stub
    /// that carries an arterial across; its line from the node).
    terminals: [(Option<usize>, Vec<V>); 2],
    ramps: Vec<RampPlan>,
    /// Local streets cut where the ramps come down.
    remove: Vec<usize>,
    /// The arterial carried across the expressway, when it ended there.
    stub: Option<Vec<V>>,
}

struct RampPlan {
    /// `true` from the deck down to the terminal.
    off: bool,
    /// Which terminal, 0 or 1.
    terminal: usize,
    /// Station on the main line of the ramp's deck end.
    deck_station: f32,
    line: Vec<V>,
}

/// Plan the diamond at `node`, or say why not.
fn plan_diamond(graph: &Graph, node: usize, half_deck: f32) -> Option<Diamond> {
    // Each `return None` says why, for whoever reads this next.
    let debug = |_why: &str| {};
    let incident = graph.incident(node);
    let streets: Vec<usize> = incident
        .iter()
        .copied()
        .filter(|&i| !main_line(&graph.roads[i]))
        .collect();
    if incident.len() - streets.len() != 2 || streets.iter().any(|&i| graph.roads[i].oneway) {
        debug("not a plain crossing of the main line");
        return None;
    }
    let reach = RAMP_REACH_M + LANDING_CLEAR_M;
    let Some(chain) = chain_at(graph, node, reach) else {
        debug("no main line through it");
        return None;
    };
    let total = polyline_len(&chain.line);
    if chain.at < RAMP_REACH_M + 40.0 || total - chain.at < RAMP_REACH_M + 40.0 {
        debug("too near the end of the main line");
        return None;
    }
    let (_, t) = sample(&chain.line, chain.at);
    // The street across: an arterial and whatever continues it on the far
    // side, an arterial or a collector. An arterial that ends at the expressway
    // is carried across it and out of town, as a radial road is.
    let direction = |road: usize| {
        let (_, line, _) = arm(graph, road, node);
        unit(sub(sample(&line, 12.0).0, line[0]))
    };
    let mut across: Vec<Option<usize>> = Vec::new();
    let arterial: Vec<usize> = streets
        .iter()
        .copied()
        .filter(|&i| graph.roads[i].class == ModernRoadClass::Arterial)
        .collect();
    let Some(&first) = arterial.first() else {
        debug("no arterial");
        return None;
    };
    across.push(Some(first));
    let far = streets.iter().copied().find(|&i| {
        i != first
            && matches!(
                graph.roads[i].class,
                ModernRoadClass::Arterial | ModernRoadClass::Collector
            )
            && dot(direction(i), direction(first)) < -0.85
    });
    let mut stub: Option<Vec<V>> = None;
    match far {
        Some(i) if streets.len() == 2 => across.push(Some(i)),
        None if streets.len() == 1 => {
            let a = direction(first);
            let out = mul(a, -1.0);
            let line = vec![
                graph.pts[node],
                add(graph.pts[node], mul(out, TERMINAL_M + 90.0)),
            ];
            // The far side must be open ground.
            let blocked = graph.roads.iter().any(|r| {
                !main_line(r)
                    && r.line.windows(2).any(|w| {
                        intersect(w[0], w[1], line[0], line[1]).is_some_and(|(_, u)| u > 0.05)
                    })
            });
            if blocked {
                debug("the arterial cannot be carried across");
                return None;
            }
            stub = Some(line);
            across.push(None);
        }
        _ => {
            debug("the crossing street is not one street across");
            return None;
        }
    }
    let arm_line = |slot: Option<usize>| match slot {
        Some(road) => arm(graph, road, node).1,
        None => stub.clone().unwrap_or_default(),
    };
    let mut terminals: Vec<(Option<usize>, Vec<V>)> = Vec::new();
    for slot in across {
        let line = arm_line(slot);
        // Room for the terminal and its box before the arm's next junction.
        if polyline_len(&line) < TERMINAL_M + 30.0 {
            debug("no room for a terminal on the arterial");
            return None;
        }
        // The arterial crosses square enough for a diamond.
        let a = unit(sub(sample(&line, 12.0).0, line[0]));
        if dot(a, t).abs() > 0.45 {
            debug("the arterial crosses too obliquely");
            return None;
        }
        terminals.push((slot, line));
    }
    // A local street crossing where the ramps come down is cut short of the
    // expressway (it ends at the frontage, as it would); anything bigger rules
    // the interchange out.
    let mut remove: Vec<usize> = Vec::new();
    for &(s, n) in &chain.nodes {
        if n == node || (s - chain.at).abs() >= LANDING_CLEAR_M {
            continue;
        }
        for i in graph.incident(n) {
            let r = &graph.roads[i];
            if main_line(r) {
                continue;
            }
            if r.class != ModernRoadClass::Local {
                debug("a street crosses where the ramps land");
                return None;
            }
            remove.push(i);
        }
    }
    let terminals: [(Option<usize>, Vec<V>); 2] = terminals.try_into().ok()?;
    let mut ramps = Vec::new();
    for (k, (_, line)) in terminals.iter().enumerate() {
        let tp = sample(line, TERMINAL_M).0;
        let (s_t, lateral) = project(&chain.line, tp);
        // Traffic on the terminal's side of the deck runs with the main line
        // when that side is its kerb side.
        let sign = if lateral > 0.0 { 1.0 } else { -1.0 };
        let d = lateral.abs();
        if d < half_deck + RAMP_HALF_M + 14.0 {
            debug("the terminal is too close to the deck for the ramps to splay");
            return None;
        }
        let abut = half_deck + RAMP_HALF_M;
        // Station along the main line in travel direction: `sign` 1 runs with
        // the chain, -1 against it.
        let point = |s_travel: f32, offset: f32| {
            let s = chain.at + sign * s_travel;
            let (p, tt) = sample(&chain.line, s);
            add(p, mul(kerb_side(mul(tt, sign)), offset))
        };
        let s_term = sign * (s_t - chain.at);
        // Off-ramp: from beside the deck `RAMP_REACH_M` before the crossing to
        // the terminal, splaying out after the taper and running in straight
        // for the last 30 m.
        let lateral_at = |s: f32, s_from: f32, s_to: f32| {
            let x = ((s - s_from) / (s_to - s_from)).clamp(0.0, 1.0);
            abut + (d - abut) * x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
        };
        let off_start = -RAMP_REACH_M + TAPER_M;
        let off_end = s_term;
        let mut off: Vec<V> = Vec::new();
        let mut s = off_start;
        while s < off_end - 30.0 {
            off.push(point(s, lateral_at(s, off_start, off_end - 30.0)));
            s += 10.0;
        }
        off.push(point(off_end - 30.0, d));
        off.push(tp);
        // On-ramp: the mirror, from the terminal to beside the deck after it.
        let on_start = s_term + 30.0;
        let on_end = RAMP_REACH_M - TAPER_M;
        let mut on: Vec<V> = vec![tp, point(on_start, d)];
        let mut s = on_start + 10.0;
        while s < on_end {
            // The same splay, run backwards from the deck.
            on.push(point(
                s,
                lateral_at(on_end - (s - on_start), on_start, on_end),
            ));
            s += 10.0;
        }
        on.push(point(on_end, abut));
        ramps.push(RampPlan {
            off: true,
            terminal: k,
            deck_station: chain.at - sign * RAMP_REACH_M,
            line: off,
        });
        ramps.push(RampPlan {
            off: false,
            terminal: k,
            deck_station: chain.at + sign * RAMP_REACH_M,
            line: on,
        });
    }
    // Both sides of the deck, one terminal each.
    let sides: Vec<f32> = terminals
        .iter()
        .map(|(_, line)| project(&chain.line, sample(line, TERMINAL_M).0).1.signum())
        .collect();
    if sides[0] == sides[1] {
        debug("both terminals are on one side of the deck");
        return None;
    }
    // A ramp may pass over a street only while it is high above it; near the
    // ground it must have the land to itself, so a local street there is cut.
    for ramp in &ramps {
        let total = polyline_len(&ramp.line);
        for (index, road) in graph.roads.iter().enumerate() {
            if road.class == ModernRoadClass::Expressway || remove.contains(&index) {
                continue;
            }
            if terminals.iter().any(|(slot, _)| *slot == Some(index)) {
                continue;
            }
            let mut low = false;
            for w in road.line.windows(2) {
                let mut along = 0.0;
                for v in ramp.line.windows(2) {
                    let l = len(sub(v[1], v[0]));
                    if let Some((_, u)) = intersect(w[0], w[1], v[0], v[1]) {
                        let at = along + u * l;
                        let to_ground = if ramp.off { total - at } else { at };
                        low |= ramp_height(to_ground) < RAMP_CLEARANCE_M;
                    }
                    along += l;
                }
            }
            if low {
                if road.class != ModernRoadClass::Local {
                    debug("a ramp would come down on a street");
                    return None;
                }
                remove.push(index);
            }
        }
    }
    remove.sort_unstable();
    remove.dedup();
    Some(Diamond {
        node,
        chain,
        terminals,
        ramps,
        remove,
        stub,
    })
}

/// Build a planned diamond into the graph.
fn build_diamond(graph: &mut Graph, diamond: Diamond, layout: &mut Layout) {
    // Cut what the landings need cleared.
    for &index in &diamond.remove {
        // Gone: no ends, no line, so nothing meets or crosses it again.
        graph.roads[index].from = usize::MAX;
        graph.roads[index].to = usize::MAX;
        graph.roads[index].line.clear();
    }
    // The terminals first: they cut the arterial arms, which the ramps end on.
    let mut terminal_nodes = Vec::new();
    for (slot, line) in &diamond.terminals {
        let road = match slot {
            Some(road) => *road,
            None => {
                let Some(stub) = diamond.stub.clone() else {
                    return;
                };
                let end = graph.node(stub[stub.len() - 1], "regional-gateway");
                graph.roads.push(Road {
                    from: diamond.node,
                    to: end,
                    class: ModernRoadClass::Arterial,
                    bridge: false,
                    oneway: false,
                    line: stub,
                });
                graph.roads.len() - 1
            }
        };
        let p = sample(line, TERMINAL_M).0;
        let n = graph.node(p, "ramp-terminal");
        let r = &graph.roads[road];
        let total = polyline_len(&r.line);
        let along = if r.from == diamond.node {
            TERMINAL_M
        } else {
            total - TERMINAL_M
        };
        graph.split(road, vec![(along, n)]);
        terminal_nodes.push(n);
    }
    // The deck ends: two nodes on the main line, each shared by an off-ramp of
    // one direction and an on-ramp of the other. Cut the farther station first,
    // so the chain's pieces still hold for the nearer one.
    // The chain is walked again for each cut, since a cut renumbers its pieces.
    let mut offsets: Vec<f32> = diamond
        .ramps
        .iter()
        .map(|r| r.deck_station - diamond.chain.at)
        .collect();
    offsets.sort_by(|a, b| a.total_cmp(b));
    offsets.dedup_by(|a, b| (*a - *b).abs() < 1.0);
    let mut deck_nodes: Vec<(f32, usize)> = Vec::new();
    for offset in offsets {
        let Some(chain) = chain_at(graph, diamond.node, RAMP_REACH_M + LANDING_CLEAR_M) else {
            continue;
        };
        if let Some(n) = cut_chain(graph, &chain, chain.at + offset, "interchange") {
            deck_nodes.push((diamond.chain.at + offset, n));
        }
    }
    for ramp in &diamond.ramps {
        let Some(&(_, deck)) = deck_nodes
            .iter()
            .find(|(s, _)| (s - ramp.deck_station).abs() < 1.0)
        else {
            continue;
        };
        let terminal = terminal_nodes[ramp.terminal];
        let (from, to) = if ramp.off {
            (deck, terminal)
        } else {
            (terminal, deck)
        };
        let crossings = graph.lay_ramp(from, to, &ramp.line, &mut layout.tapers);
        layout.detach.extend(crossings);
    }
}

/// Lay the interchanges into the street graph, before the blocks are cut.
pub(super) fn lay_out(
    frame: &CityFrame,
    nodes: Vec<SdNode>,
    sd: Vec<SdRoad>,
    hd: Vec<HdRoad>,
    rules: &TrafficRules,
) -> (Vec<SdNode>, Vec<SdRoad>, Vec<HdRoad>, Layout) {
    let mut layout = Layout::default();
    if !sd.iter().any(|r| r.class == ModernRoadClass::Expressway) {
        return (nodes, sd, hd, layout);
    }
    let mut graph = Graph::new(frame, &nodes, &sd, &hd);
    let half_deck = crate::cross_section(ModernRoadClass::Expressway).width_metres * 0.5;
    // Candidate crossings, the most central first, so a town's interchange is
    // where its traffic is.
    let mut candidates: Vec<usize> = (0..graph.pts.len())
        .filter(|&n| {
            let list = graph.incident(n);
            list.iter().any(|&i| main_line(&graph.roads[i]))
                && list
                    .iter()
                    .any(|&i| graph.roads[i].class == ModernRoadClass::Arterial)
        })
        .collect();
    candidates.sort_by(|a, b| {
        let (pa, pb) = (graph.pts[*a], graph.pts[*b]);
        len(pa).total_cmp(&len(pb))
    });
    let mut built: Vec<V> = Vec::new();
    for node in candidates {
        let p = graph.pts[node];
        // Spacing along the line, measured straight: near enough for a town.
        if built.iter().any(|q| len(sub(*q, p)) < SPACING_M) {
            continue;
        }
        let Some(diamond) = plan_diamond(&graph, node, half_deck) else {
            continue;
        };
        build_diamond(&mut graph, diamond, &mut layout);
        built.push(p);
        layout.interchanges += 1;
    }
    // Near an interchange the main line curves beside its ramps; cut it as
    // finely as the ramps, so the blocks see both curves and not a chord of
    // one across the other.
    if !built.is_empty() {
        for index in 0..graph.roads.len() {
            let road = &graph.roads[index];
            if !main_line(road) || road.line.len() < 2 {
                continue;
            }
            let near = road.line.iter().any(|p| {
                built
                    .iter()
                    .any(|x| len(sub(*p, *x)) < RAMP_REACH_M + 250.0)
            });
            if near {
                graph.densify(index, PIECE_M);
            }
        }
    }
    let (nodes, sd, hd) = graph.into_plan(frame, rules);
    (nodes, sd, hd, layout)
}

/// Take the expressway off every street it only passes, once the blocks are cut.
///
/// A node where the main line meets a street (or where a ramp passes one) is
/// split in two: the street keeps it, and the expressway's roads move to a new
/// node at the same place. Every expressway road and ramp is then on layer 1.
pub(super) fn grade_separate(
    frame: &CityFrame,
    nodes: &mut Vec<SdNode>,
    sd: &mut Vec<SdRoad>,
    hd: &mut Vec<HdRoad>,
    layout: &Layout,
) {
    if layout.interchanges == 0 || !sd.iter().any(|r| r.class == ModernRoadClass::Expressway) {
        return;
    }
    let oneway: HashMap<u32, bool> = hd.iter().map(|r| (r.id, r.oneway)).collect();
    let mut incident: Vec<Vec<usize>> = vec![Vec::new(); nodes.len()];
    for (i, r) in sd.iter().enumerate() {
        incident[r.from as usize].push(i);
        incident[r.to as usize].push(i);
    }
    let mut split: Vec<usize> = (0..nodes.len())
        .filter(|&n| {
            let list = &incident[n];
            list.iter().any(|&i| {
                sd[i].class == ModernRoadClass::Expressway
                    && !oneway.get(&sd[i].id).copied().unwrap_or(false)
            }) && list
                .iter()
                .any(|&i| sd[i].class != ModernRoadClass::Expressway)
        })
        .collect();
    split.extend(layout.detach.iter().copied());
    split.sort_unstable();
    split.dedup();
    let mut lifted = Vec::with_capacity(split.len() * 2);
    for n in split {
        let id = nodes.len() as u32;
        // Both sides: the expressway's new node, and the street's old one, which
        // may now be no more than a bend in one street.
        lifted.push(id as usize);
        lifted.push(n);
        nodes.push(SdNode {
            id,
            point: nodes[n].point,
            role: "flyover".into(),
        });
        for &i in &incident[n] {
            if sd[i].class != ModernRoadClass::Expressway {
                continue;
            }
            if sd[i].from as usize == n {
                sd[i].from = id;
            }
            if sd[i].to as usize == n {
                sd[i].to = id;
            }
        }
    }
    // And wherever two pieces of the main line meet with nothing else, which
    // is every corner of a ring road that the grid drew as a polygon.
    let mut degree = vec![0_u32; nodes.len()];
    let mut main_only = vec![true; nodes.len()];
    for r in sd.iter() {
        for n in [r.from as usize, r.to as usize] {
            degree[n] += 1;
            let ramp = oneway.get(&r.id).copied().unwrap_or(false);
            main_only[n] &= r.class == ModernRoadClass::Expressway && !ramp;
        }
    }
    for n in 0..nodes.len() {
        if degree[n] == 2 && main_only[n] && !lifted.contains(&n) {
            lifted.push(n);
        }
    }
    for node in lifted {
        ease_through(frame, nodes, sd, hd, node);
    }
    drop_tapers(sd, hd, &layout.tapers);
    drop_orphans(nodes.len(), sd, hd);
    for road in hd.iter_mut() {
        if road.class == ModernRoadClass::Expressway {
            road.layer = 1;
            road.bridge = false;
        }
    }
    for road in sd.iter_mut() {
        if road.class == ModernRoadClass::Expressway {
            road.bridge = false;
        }
    }
}

/// Ease a road through a node where only it now runs: the expressway through
/// a node it was lifted off, or a street through the node the expressway left.
/// The street grid put a corner there, which a street turns at a junction but
/// not as a bend in one road, and an expressway never: the last stretch of each
/// piece is replaced by a circular arc through the corner, tangent to both, and
/// the node moves onto the arc.
fn ease_through(
    frame: &CityFrame,
    nodes: &mut [SdNode],
    sd: &[SdRoad],
    hd: &mut [HdRoad],
    node: usize,
) {
    let at: Vec<usize> = (0..sd.len())
        .filter(|&i| sd[i].from as usize == node || sd[i].to as usize == node)
        .collect();
    let [a, b] = at.as_slice() else {
        return;
    };
    let (a, b) = (*a, *b);
    if hd[a].oneway != hd[b].oneway || sd[a].class != sd[b].class {
        return;
    }
    let local = |road: usize| -> Vec<V> {
        hd[road]
            .centreline
            .iter()
            .map(|p| frame.to_local(*p))
            .collect()
    };
    // Each piece as a line toward the node.
    let toward = |road: usize| {
        let mut line = local(road);
        if sd[road].from as usize == node {
            line.reverse();
        }
        line
    };
    let (la, lb) = (toward(a), toward(b));
    let (len_a, len_b) = (polyline_len(&la), polyline_len(&lb));
    let reach = 45.0_f32.min(len_a * 0.45).min(len_b * 0.45);
    if reach < 4.0 {
        return;
    }
    let corner = la[la.len() - 1];
    let (pa, _) = sample(&la, len_a - reach);
    let (pb, _) = sample(&lb, len_b - reach);
    // Unit vectors from the corner back along each piece.
    let (ua, ub) = (unit(sub(pa, corner)), unit(sub(pb, corner)));
    let opening = dot(ua, ub).clamp(-1.0, 1.0).acos();
    // Straight through already, or a hairpin no arc should round.
    if !(0.3..=std::f32::consts::PI - 0.01).contains(&opening) {
        return;
    }
    // Two streets that met the expressway at a real corner are left meeting
    // at it; only a gentle bend (up to 45 degrees) is one street running on.
    let expressway = sd[a].class == ModernRoadClass::Expressway;
    if !expressway && opening < std::f32::consts::PI * 0.75 {
        return;
    }
    // The arc tangent to both at `reach` from the corner.
    let radius = reach * (opening * 0.5).tan();
    let bisector = unit(add(ua, ub));
    let centre = add(corner, mul(bisector, reach / (opening * 0.5).cos()));
    let start = sub(pa, centre).1.atan2(sub(pa, centre).0);
    let mut sweep = sub(pb, centre).1.atan2(sub(pb, centre).0) - start;
    while sweep > std::f32::consts::PI {
        sweep -= std::f32::consts::TAU;
    }
    while sweep < -std::f32::consts::PI {
        sweep += std::f32::consts::TAU;
    }
    let arc = |t: f32| {
        let angle = start + sweep * t;
        add(centre, (radius * angle.cos(), radius * angle.sin()))
    };
    let steps = ((radius * sweep.abs() / 3.0).ceil() as usize).clamp(4, 40);
    let half = steps.div_ceil(2);
    let middle = arc(0.5);
    let first: Vec<V> = (0..=half)
        .map(|k| arc(0.5 * k as f32 / half as f32))
        .collect();
    let second: Vec<V> = (0..=half)
        .map(|k| arc(1.0 - 0.5 * k as f32 / half as f32))
        .collect();
    let rebuild = |line: &[V], length: f32, tail: &[V]| -> Vec<V> {
        let mut kept = slice(line, 0.0, length - reach);
        kept.extend_from_slice(&tail[1..]);
        kept
    };
    let mut new_a = rebuild(&la, len_a, &first);
    let mut new_b = rebuild(&lb, len_b, &second);
    if sd[a].from as usize == node {
        new_a.reverse();
    }
    if sd[b].from as usize == node {
        new_b.reverse();
    }
    hd[a].centreline = new_a.iter().map(|p| frame.to_world(p.0, p.1)).collect();
    hd[b].centreline = new_b.iter().map(|p| frame.to_world(p.0, p.1)).collect();
    nodes[node].point = frame.to_world(middle.0, middle.1);
}

/// Drop the taper edges that joined each ramp to its deck node for the block
/// layout: the ramp piece that met the taper meets the deck node instead, its
/// line still starting beside the deck. Roads are renumbered (nothing else
/// refers to them by id yet), and their lane ids with them.
fn drop_tapers(sd: &mut Vec<SdRoad>, hd: &mut Vec<HdRoad>, tapers: &[usize]) {
    if tapers.is_empty() {
        return;
    }
    let mut drop = vec![false; sd.len()];
    for &t in tapers {
        let at: Vec<usize> = (0..sd.len())
            .filter(|&i| !drop[i] && (sd[i].from as usize == t || sd[i].to as usize == t))
            .collect();
        // The taper is the piece with a two-point line; the ramp is the other.
        let Some(&taper) = at.iter().find(|&&i| hd[i].centreline.len() == 2) else {
            continue;
        };
        let Some(&ramp) = at.iter().find(|&&i| i != taper) else {
            continue;
        };
        let deck = if sd[taper].from as usize == t {
            sd[taper].to
        } else {
            sd[taper].from
        };
        if sd[ramp].from as usize == t {
            sd[ramp].from = deck;
        } else {
            sd[ramp].to = deck;
        }
        drop[taper] = true;
    }
    remove_roads(sd, hd, &drop);
}

/// A few local street pieces that reached the town only through the
/// expressway are left on their own once it is lifted off them; they go.
fn drop_orphans(node_count: usize, sd: &mut Vec<SdRoad>, hd: &mut Vec<HdRoad>) {
    let mut parent: Vec<usize> = (0..node_count).collect();
    fn root(p: &mut [usize], mut x: usize) -> usize {
        while p[x] != x {
            p[x] = p[p[x]];
            x = p[x];
        }
        x
    }
    for r in sd.iter() {
        let (a, b) = (
            root(&mut parent, r.from as usize),
            root(&mut parent, r.to as usize),
        );
        parent[a] = b;
    }
    let mut size: HashMap<usize, usize> = HashMap::new();
    for r in sd.iter() {
        *size.entry(root(&mut parent, r.from as usize)).or_insert(0) += 1;
    }
    let Some((&main, _)) = size.iter().max_by_key(|(_, n)| **n) else {
        return;
    };
    let drop: Vec<bool> = sd
        .iter()
        .map(|r| {
            let c = root(&mut parent, r.from as usize);
            c != main && size[&c] <= 3 && r.class == ModernRoadClass::Local
        })
        .collect();
    if drop.iter().any(|d| *d) {
        remove_roads(sd, hd, &drop);
    }
}

/// Remove the roads marked in `drop`, renumbering the rest and their lane ids.
fn remove_roads(sd: &mut Vec<SdRoad>, hd: &mut Vec<HdRoad>, drop: &[bool]) {
    let mut kept_sd = Vec::with_capacity(sd.len());
    let mut kept_hd = Vec::with_capacity(hd.len());
    for (i, (mut s, mut h)) in sd.drain(..).zip(hd.drain(..)).enumerate() {
        if drop[i] {
            continue;
        }
        let id = kept_sd.len() as u32;
        let (old, new) = (format!("road/{}/", s.id), format!("road/{id}/"));
        for lane in &mut h.lanes {
            lane.id = lane.id.replacen(&old, &new, 1);
        }
        s.id = id;
        h.id = id;
        h.sd_road = id;
        kept_sd.push(s);
        kept_hd.push(h);
    }
    *sd = kept_sd;
    *hd = kept_hd;
}
