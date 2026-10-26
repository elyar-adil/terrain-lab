//! Tidy a street network supplied from outside before a town is built on it.
//!
//! Measured on the roads layer's town plans, three defects made a street map
//! read as a tangle, and none of them is a street a planner would draw:
//!
//! - **junction clusters**: 164 pairs of junctions closer than 40 m in one town,
//!   64 closer than 25 m, so their boxes, stop lines and crosswalks ran into one
//!   another;
//! - **parallel duplicates**: 30 % of the collector length ran within 45 m of a
//!   parallel street of the same or a higher class, two carriageways side by side
//!   where there is room for one and a row of buildings;
//! - the stubs either of these leaves behind.
//!
//! This merges each cluster into one junction, drops each duplicate, and prunes
//! the short dead ends that result. It never drops an arterial or an expressway,
//! and never a bridge.

use std::collections::{HashMap, HashSet};

use super::{ExternalRoad, ExternalStreets};
use crate::{ModernRoadClass, Point};

/// Junctions closer than this are one junction.
const CLUSTER_M: f32 = 30.0;
/// A street this close to a parallel one, for most of its length, duplicates it.
const DUPLICATE_M: f32 = 40.0;
/// ... and "parallel" is within this many degrees.
const DUPLICATE_DEG: f32 = 15.0;
/// ... and "most" is this share of its sampled length.
const DUPLICATE_SHARE: f32 = 0.6;
/// A dead end left behind shorter than this goes too.
const STUB_M: f32 = 70.0;

fn rank(class: ModernRoadClass) -> u8 {
    match class {
        ModernRoadClass::Expressway => 0,
        ModernRoadClass::Arterial => 1,
        ModernRoadClass::Collector => 2,
        ModernRoadClass::Local => 3,
    }
}

fn metres(p: Point) -> (f32, f32) {
    (p.x_km * 1000.0, p.y_km * 1000.0)
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

fn length(road: &ExternalRoad) -> f32 {
    road.centreline
        .windows(2)
        .map(|w| dist(metres(w[0]), metres(w[1])))
        .sum()
}

/// Merge clusters, drop duplicates, prune stubs.
pub(super) fn tidy(streets: &mut ExternalStreets) {
    merge_clusters(streets);
    drop_overlaps(streets);
    drop_duplicates(streets);
    prune_stubs(streets);
}

/// Heading a road leaves `node` on, over its first 15 m.
fn leaving(road: &ExternalRoad, node: u64) -> f32 {
    let mut pts: Vec<(f32, f32)> = road.centreline.iter().map(|p| metres(*p)).collect();
    if road.to == node && road.from != node {
        pts.reverse();
    }
    let start = pts[0];
    let mut end = start;
    for p in &pts[1..] {
        end = *p;
        if dist(start, *p) > 15.0 {
            break;
        }
    }
    (end.1 - start.1).atan2(end.0 - start.0)
}

/// Where two streets leave one junction within 20 degrees of each other, they
/// run over one another for their first metres: two duplicates whose ends a
/// merged cluster brought together. The lesser goes.
fn drop_overlaps(streets: &mut ExternalStreets) {
    let tolerance = 20.0_f32.to_radians();
    loop {
        let mut at: HashMap<u64, Vec<usize>> = HashMap::new();
        for (index, road) in streets.roads.iter().enumerate() {
            at.entry(road.from).or_default().push(index);
            at.entry(road.to).or_default().push(index);
        }
        let mut worst: Option<usize> = None;
        'nodes: for (node, list) in &at {
            for (i, &a) in list.iter().enumerate() {
                for &b in &list[i + 1..] {
                    let turn = (leaving(&streets.roads[a], *node)
                        - leaving(&streets.roads[b], *node))
                    .rem_euclid(std::f32::consts::TAU);
                    if turn.min(std::f32::consts::TAU - turn) < tolerance {
                        let (ra, rb) = (&streets.roads[a], &streets.roads[b]);
                        if ra.bridge || rb.bridge {
                            continue;
                        }
                        worst = Some(
                            if (rank(ra.class), -length(ra)) >= (rank(rb.class), -length(rb)) {
                                a
                            } else {
                                b
                            },
                        );
                        break 'nodes;
                    }
                }
            }
        }
        match worst {
            Some(index) if rank(streets.roads[index].class) >= 2 => {
                streets.roads.remove(index);
            }
            _ => break,
        }
    }
}

fn degrees(roads: &[ExternalRoad]) -> HashMap<u64, usize> {
    let mut degree: HashMap<u64, usize> = HashMap::new();
    for road in roads {
        *degree.entry(road.from).or_default() += 1;
        *degree.entry(road.to).or_default() += 1;
    }
    degree
}

fn merge_clusters(streets: &mut ExternalStreets) {
    merge_nodes(streets, 1.0, 0);
    merge_nodes(streets, CLUSTER_M, 3);
}

/// Merge nodes closer than `radius` whose degree is at least `min_degree`. At
/// a 1 m radius and any degree it joins the copies of one node that tile
/// borders leave under different ids; at the cluster radius, junctions only.
fn merge_nodes(streets: &mut ExternalStreets, radius: f32, min_degree: usize) {
    let degree = degrees(&streets.roads);
    // The most important street at each node, to choose where a cluster sits.
    let mut best: HashMap<u64, u8> = HashMap::new();
    for road in &streets.roads {
        for id in [road.from, road.to] {
            let r = best.entry(id).or_insert(u8::MAX);
            *r = (*r).min(rank(road.class));
        }
    }
    let junctions: Vec<(u64, (f32, f32))> = streets
        .nodes
        .iter()
        .filter(|n| degree.get(&n.id).copied().unwrap_or(0) >= min_degree.max(1))
        .map(|n| (n.id, metres(n.point)))
        .collect();
    // Union-find over nodes closer than `radius`, by a coarse grid.
    let mut parent: HashMap<u64, u64> = junctions.iter().map(|(id, _)| (*id, *id)).collect();
    fn find(parent: &mut HashMap<u64, u64>, x: u64) -> u64 {
        let p = parent[&x];
        if p == x {
            return x;
        }
        let root = find(parent, p);
        parent.insert(x, root);
        root
    }
    let cell = |p: (f32, f32)| ((p.0 / radius).floor() as i64, (p.1 / radius).floor() as i64);
    let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (index, (_, p)) in junctions.iter().enumerate() {
        grid.entry(cell(*p)).or_default().push(index);
    }
    // Candidate pairs, nearest first, so the closest junctions join first.
    let mut pairs: Vec<(f32, usize, usize)> = Vec::new();
    for (index, (_, p)) in junctions.iter().enumerate() {
        let (cx, cy) = cell(*p);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for &other in grid
                    .get(&(cx + dx, cy + dy))
                    .map(Vec::as_slice)
                    .unwrap_or(&[])
                {
                    let d = dist(*p, junctions[other].1);
                    if other > index && d < radius {
                        pairs.push((d, index, other));
                    }
                }
            }
        }
    }
    pairs.sort_by(|x, y| x.0.total_cmp(&y.0));
    // Streets between two merge candidates, by their end ids.
    let edges: Vec<(u64, u64)> = streets
        .roads
        .iter()
        .filter(|r| parent.contains_key(&r.from) && parent.contains_key(&r.to))
        .map(|r| (r.from, r.to))
        .collect();
    // Arms a cluster would have: the streets leaving it, those inside it excluded.
    let mut arms: HashMap<u64, usize> = junctions
        .iter()
        .map(|(id, _)| (*id, degree.get(id).copied().unwrap_or(0)))
        .collect();
    for (_, i, j) in pairs {
        let (a, b) = (
            find(&mut parent, junctions[i].0),
            find(&mut parent, junctions[j].0),
        );
        if a == b {
            continue;
        }
        let mut links = 0;
        for &(x, y) in &edges {
            let (rx, ry) = (find(&mut parent, x), find(&mut parent, y));
            if (rx == a && ry == b) || (rx == b && ry == a) {
                links += 1;
            }
        }
        let joined = arms[&a] + arms[&b] - 2 * links;
        // Junctions join only while the result has at most four arms; past that
        // two T-junctions stay a staggered pair (错位交叉口) rather than become
        // a five-way star. Copies of one node (`min_degree` 0) always join.
        if min_degree >= 3 && joined > 4 {
            continue;
        }
        parent.insert(a, b);
        arms.insert(b, joined);
    }
    // Each cluster collapses onto its member carrying the most important street.
    let mut members: HashMap<u64, Vec<u64>> = HashMap::new();
    for (id, _) in &junctions {
        let root = find(&mut parent, *id);
        members.entry(root).or_default().push(*id);
    }
    let mut onto: HashMap<u64, u64> = HashMap::new();
    for list in members.values().filter(|list| list.len() > 1) {
        let keep = *list
            .iter()
            .min_by_key(|id| (best.get(id).copied().unwrap_or(u8::MAX), degree[id], **id))
            .unwrap();
        for id in list {
            if *id != keep {
                onto.insert(*id, keep);
            }
        }
    }
    if onto.is_empty() {
        return;
    }
    let position: HashMap<u64, Point> = streets.nodes.iter().map(|n| (n.id, n.point)).collect();
    for road in &mut streets.roads {
        if let Some(&to) = onto.get(&road.from) {
            road.from = to;
            ease_end(&mut road.centreline, position[&to]);
        }
        if let Some(&to) = onto.get(&road.to) {
            road.to = to;
            road.centreline.reverse();
            ease_end(&mut road.centreline, position[&to]);
            road.centreline.reverse();
        }
    }
    // A street that ran between two members of a cluster is gone; of two streets
    // now joining the same pair of junctions, the more important one stays.
    streets.roads.retain(|road| road.from != road.to);
    let mut pair: HashMap<(u64, u64), usize> = HashMap::new();
    let mut drop: HashSet<usize> = HashSet::new();
    for (index, road) in streets.roads.iter().enumerate() {
        let key = (road.from.min(road.to), road.from.max(road.to));
        match pair.get(&key) {
            Some(&other) => {
                let o = &streets.roads[other];
                let worse = if (rank(road.class), length(road)) > (rank(o.class), length(o)) {
                    index
                } else {
                    pair.insert(key, index);
                    other
                };
                drop.insert(worse);
            }
            None => {
                pair.insert(key, index);
            }
        }
    }
    let mut index = 0;
    streets.roads.retain(|_| {
        let keep = !drop.contains(&index);
        index += 1;
        keep
    });
    let used: HashSet<u64> = streets.roads.iter().flat_map(|r| [r.from, r.to]).collect();
    streets.nodes.retain(|n| used.contains(&n.id));
}

/// Move the first point of `line` onto `target`, and the line's first 40 m with
/// it, less and less: snapping the end vertex alone put a kink in the street.
fn ease_end(line: &mut [Point], target: Point) {
    let Some(&first) = line.first() else {
        return;
    };
    let (dx, dy) = (target.x_km - first.x_km, target.y_km - first.y_km);
    let reach = 40.0;
    let mut travelled = 0.0;
    let mut previous = metres(first);
    for point in line.iter_mut() {
        travelled += dist(previous, metres(*point));
        previous = metres(*point);
        let k = (1.0 - travelled / reach).max(0.0);
        if k <= 0.0 {
            break;
        }
        point.x_km += dx * k;
        point.y_km += dy * k;
    }
}

/// Samples along a road every 15 m, clear of its ends, with their headings.
fn samples(road: &ExternalRoad) -> Vec<((f32, f32), f32)> {
    let pts: Vec<(f32, f32)> = road.centreline.iter().map(|p| metres(*p)).collect();
    let total = length(road);
    let mut out = Vec::new();
    let mut travelled = 0.0;
    for w in pts.windows(2) {
        let segment = dist(w[0], w[1]);
        if segment < 1e-3 {
            continue;
        }
        let heading = (w[1].1 - w[0].1).atan2(w[1].0 - w[0].0);
        let mut t = (15.0 - travelled % 15.0) % 15.0;
        while t < segment {
            let s = travelled + t;
            if s > 20.0 && s < total - 20.0 {
                let f = t / segment;
                out.push((
                    (
                        w[0].0 + (w[1].0 - w[0].0) * f,
                        w[0].1 + (w[1].1 - w[0].1) * f,
                    ),
                    heading,
                ));
            }
            t += 15.0;
        }
        travelled += segment;
    }
    out
}

fn drop_duplicates(streets: &mut ExternalStreets) {
    let all: Vec<Vec<((f32, f32), f32)>> = streets.roads.iter().map(samples).collect();
    let cell = |p: (f32, f32)| {
        (
            (p.0 / DUPLICATE_M).floor() as i64,
            (p.1 / DUPLICATE_M).floor() as i64,
        )
    };
    let mut grid: HashMap<(i64, i64), Vec<(usize, usize)>> = HashMap::new();
    for (road, list) in all.iter().enumerate() {
        for (k, (p, _)) in list.iter().enumerate() {
            grid.entry(cell(*p)).or_default().push((road, k));
        }
    }
    // The least important, then the shortest, are judged first: of two duplicates
    // it is the lesser that goes, and the greater then no longer has a twin.
    let mut order: Vec<usize> = (0..streets.roads.len()).collect();
    order.sort_by(|&a, &b| {
        let (ra, rb) = (&streets.roads[a], &streets.roads[b]);
        rank(rb.class)
            .cmp(&rank(ra.class))
            .then(length(ra).total_cmp(&length(rb)))
    });
    let tolerance = DUPLICATE_DEG.to_radians();
    let mut removed = vec![false; streets.roads.len()];
    for &index in &order {
        let road = &streets.roads[index];
        if road.bridge || rank(road.class) <= 1 || all[index].len() < 3 {
            continue;
        }
        let ends = [road.from, road.to];
        let twinned = all[index]
            .iter()
            .filter(|(p, heading)| {
                let (cx, cy) = cell(*p);
                (-1..=1).any(|dx| {
                    (-1..=1).any(|dy| {
                        grid.get(&(cx + dx, cy + dy)).is_some_and(|hits| {
                            hits.iter().any(|&(other, k)| {
                                if other == index || removed[other] {
                                    return false;
                                }
                                let o = &streets.roads[other];
                                if rank(o.class) > rank(road.class)
                                    || ends.contains(&o.from)
                                    || ends.contains(&o.to)
                                {
                                    return false;
                                }
                                let (q, h2) = all[other][k];
                                let turn = ((heading - h2 + std::f32::consts::FRAC_PI_2)
                                    .rem_euclid(std::f32::consts::PI)
                                    - std::f32::consts::FRAC_PI_2)
                                    .abs();
                                let d = dist(*p, q);
                                turn < tolerance && d > 6.0 && d < DUPLICATE_M
                            })
                        })
                    })
                })
            })
            .count();
        if twinned as f32 > all[index].len() as f32 * DUPLICATE_SHARE {
            removed[index] = true;
        }
    }
    let mut index = 0;
    streets.roads.retain(|_| {
        let keep = !removed[index];
        index += 1;
        keep
    });
}

fn prune_stubs(streets: &mut ExternalStreets) {
    // A short lesser street hanging off a junction by one end, its other end
    // free: what a dropped duplicate or a merged cluster leaves behind.
    loop {
        let degree = degrees(&streets.roads);
        let before = streets.roads.len();
        streets.roads.retain(|road| {
            let dead = degree[&road.from] == 1 || degree[&road.to] == 1;
            !(dead
                && rank(road.class) >= 2
                && !road.bridge
                && length(road) < STUB_M
                && degree[&road.from] + degree[&road.to] >= 3)
        });
        if streets.roads.len() == before {
            break;
        }
    }
    let used: HashSet<u64> = streets.roads.iter().flat_map(|r| [r.from, r.to]).collect();
    streets.nodes.retain(|n| used.contains(&n.id));
}
