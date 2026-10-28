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
//! and never a bridge. A longer street a dropped duplicate leaves ending short of
//! the street it used to meet is carried on to it, so it ends in a junction and
//! not in a kerb a few metres from the road it obviously meets.

use std::collections::{HashMap, HashSet};

use super::{ExternalRoad, ExternalStreets};
use crate::{ModernRoadClass, Point};

/// Junctions closer than this are one junction.
const CLUSTER_M: f32 = 30.0;
/// ... and closer than this they are one even if that makes a five-way
/// junction: two boxes this near overlap, and no staggered pair is built so.
const STAR_M: f32 = 26.0;
/// A street this close to a parallel one, for most of its length, duplicates it.
const DUPLICATE_M: f32 = 40.0;
/// ... and "parallel" is within this many degrees.
const DUPLICATE_DEG: f32 = 15.0;
/// ... and "most" is this share of its sampled length.
const DUPLICATE_SHARE: f32 = 0.6;
/// A dead end left behind shorter than this goes too.
const STUB_M: f32 = 70.0;
/// A piece of street free at both ends and shorter than this, left after the
/// dead ends are joined, is a fragment and not a street.
const ORPHAN_M: f32 = 120.0;
/// ... and, more loosely, one this far ahead, within this many degrees.
const FAR_JOIN_M: f32 = 150.0;
const FAR_JOIN_DEG: f32 = 30.0;
/// A dead-end branch (to its first junction) shorter than this goes.
const BRANCH_M: f32 = 800.0;
/// A dead end that points at a street this close ahead is carried on to it.
const JOIN_M: f32 = 60.0;
/// ... "points at" meaning within this many degrees of its own heading.
const JOIN_DEG: f32 = 35.0;

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
    cut_squeezed(streets);
    prune_stubs(streets);
    drop_orphans(streets, false);
    join_dead_ends(streets, JOIN_M, JOIN_DEG);
    // A street the plan stops in the open, at the town's edge or short of
    // the one it was heading for: carried on to that one when it lies ahead,
    // and otherwise taken back to its junction. A street that ends in the
    // grass with its footway wrapped round the end is not one a town has.
    join_dead_ends(streets, FAR_JOIN_M, FAR_JOIN_DEG);
    prune_branches(streets, BRANCH_M);
    drop_orphans(streets, true);
    // Joined ends make junctions too, and some land close to others.
    merge_clusters(streets);
    drop_overlaps(streets);
    break_knots(streets);
    // What the merges and drops above left hanging.
    prune_branches(streets, BRANCH_M);
    drop_orphans(streets, true);
    round_bends(streets);
}

/// Three junctions joined to one another by streets shorter than this are a
/// knot, not three junctions.
const KNOT_M: f32 = 45.0;

/// Where three junctions sit in a triangle of short streets, too many arms
/// for one junction and too close for three, their boxes run into one
/// another. A planner would not build one: the triangle's least street (the
/// lowest class, the longest of those) goes, leaving a staggered pair or a
/// junction and a bend. Arterials, expressways and bridges stay.
fn break_knots(streets: &mut ExternalStreets) {
    loop {
        let degree = degrees(&streets.roads);
        let short: Vec<(usize, u64, u64, f32)> = streets
            .roads
            .iter()
            .enumerate()
            .filter(|(_, r)| r.from != r.to)
            .map(|(i, r)| (i, r.from.min(r.to), r.from.max(r.to), length(r)))
            .filter(|&(_, a, b, l)| l < KNOT_M && degree[&a] >= 3 && degree[&b] >= 3)
            .collect();
        let mut side: HashMap<(u64, u64), usize> = HashMap::new();
        let mut near: HashMap<u64, Vec<u64>> = HashMap::new();
        for &(i, a, b, _) in &short {
            side.insert((a, b), i);
            near.entry(a).or_default().push(b);
            near.entry(b).or_default().push(a);
        }
        let mut drop = None;
        'search: for &(i, a, b, _) in &short {
            for c in near.get(&a).into_iter().flatten() {
                let (Some(&j), Some(&k)) = (
                    side.get(&(a.min(*c), a.max(*c))),
                    side.get(&(b.min(*c), b.max(*c))),
                ) else {
                    continue;
                };
                if *c == b {
                    continue;
                }
                let least = [i, j, k]
                    .into_iter()
                    .max_by(|&x, &y| {
                        let (rx, ry) = (&streets.roads[x], &streets.roads[y]);
                        rank(rx.class)
                            .cmp(&rank(ry.class))
                            .then(length(rx).total_cmp(&length(ry)))
                    })
                    .unwrap();
                let road = &streets.roads[least];
                if road.bridge || rank(road.class) <= rank(ModernRoadClass::Arterial) {
                    continue;
                }
                drop = Some(least);
                break 'search;
            }
        }
        match drop {
            Some(index) => {
                streets.roads.remove(index);
            }
            None => break,
        }
    }
}

/// The smallest radius a street of each class bends on: CJJ 37-2012's limit
/// with superelevation for its design speed (50, 40 and 30 km/h).
fn bend_radius(class: ModernRoadClass) -> Option<f32> {
    match class {
        ModernRoadClass::Expressway => None,
        ModernRoadClass::Arterial => Some(100.0),
        ModernRoadClass::Collector => Some(70.0),
        ModernRoadClass::Local => Some(40.0),
    }
}

/// Every bend of a street an arc, not a vertex: merging junctions and carrying
/// dead ends on leave a street a corner of 20 to 30 degrees a few tens of
/// metres from its end, and a street is built to a curvature. The arc takes
/// the class's radius, or the largest that keeps to 45 % of either leg, so a
/// street's first metres out of a junction stay straight. Bridges keep their
/// straight decks; expressways are eased where they are laid out.
fn round_bends(streets: &mut ExternalStreets) {
    for road in &mut streets.roads {
        let Some(radius) = bend_radius(road.class) else {
            continue;
        };
        if road.bridge || road.centreline.len() < 3 {
            continue;
        }
        let p: Vec<(f32, f32)> = road.centreline.iter().map(|q| metres(*q)).collect();
        let mut out = vec![p[0]];
        for i in 1..p.len() - 1 {
            let (a, b, c) = (p[i - 1], p[i], p[i + 1]);
            let (l0, l1) = (dist(a, b), dist(b, c));
            if l0 < 1.0e-3 || l1 < 1.0e-3 {
                continue;
            }
            let d0 = ((b.0 - a.0) / l0, (b.1 - a.1) / l0);
            let d1 = ((c.0 - b.0) / l1, (c.1 - b.1) / l1);
            let turn = (d0.0 * d1.1 - d0.1 * d1.0).atan2(d0.0 * d1.0 + d0.1 * d1.1);
            if turn.abs() < 2.0_f32.to_radians() {
                out.push(b);
                continue;
            }
            let half = turn.abs() * 0.5;
            let reach = (radius * half.tan()).min(0.45 * l0.min(l1));
            let r = reach / half.tan();
            let entry = (b.0 - d0.0 * reach, b.1 - d0.1 * reach);
            // The arc's centre, `r` to the side the street turns to.
            let side = turn.signum();
            let centre = (entry.0 - d0.1 * r * side, entry.1 + d0.0 * r * side);
            let from = (entry.1 - centre.1).atan2(entry.0 - centre.0);
            let steps = ((r * turn.abs() / 3.0).ceil() as usize).max(2);
            out.extend((0..=steps).map(|k| {
                let angle = from + turn * k as f32 / steps as f32;
                (centre.0 + angle.cos() * r, centre.1 + angle.sin() * r)
            }));
        }
        out.push(p[p.len() - 1]);
        road.centreline = out
            .into_iter()
            .map(|(x, y)| Point {
                x_km: x / 1000.0,
                y_km: y / 1000.0,
            })
            .collect();
    }
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
    join_facing_ends(streets);
    merge_nodes(streets, CLUSTER_M, 3);
}

/// Two free ends this close are one street broken in two.
const FACING_M: f32 = 8.0;

/// Join every pair of free street ends within `FACING_M` of each other: one
/// street the plan's pieces broke in two (a bridge whose halves came from
/// two edges stopped 3.6 m apart, each a dead end in mid-air).
fn join_facing_ends(streets: &mut ExternalStreets) {
    let degree = degrees(&streets.roads);
    let ends: Vec<(u64, (f32, f32))> = streets
        .nodes
        .iter()
        .filter(|n| degree.get(&n.id) == Some(&1))
        .map(|n| (n.id, metres(n.point)))
        .collect();
    let mut onto: HashMap<u64, u64> = HashMap::new();
    for (i, (a, pa)) in ends.iter().enumerate() {
        if onto.contains_key(a) {
            continue;
        }
        let nearest = ends[i + 1..]
            .iter()
            .filter(|(b, _)| !onto.contains_key(b) && !onto.values().any(|v| v == b))
            .map(|(b, pb)| (dist(*pa, *pb), *b))
            .filter(|(d, _)| *d < FACING_M)
            .min_by(|x, y| x.0.total_cmp(&y.0));
        if let Some((_, b)) = nearest {
            onto.insert(*a, b);
        }
    }
    if onto.is_empty() {
        return;
    }
    let position: HashMap<u64, Point> = streets.nodes.iter().map(|n| (n.id, n.point)).collect();
    for road in &mut streets.roads {
        if let Some(&to) = onto.get(&road.from) {
            road.from = to;
            if let Some(first) = road.centreline.first_mut() {
                *first = position[&to];
            }
        }
        if let Some(&to) = onto.get(&road.to) {
            road.to = to;
            if let Some(last) = road.centreline.last_mut() {
                *last = position[&to];
            }
        }
    }
    streets.roads.retain(|road| road.from != road.to);
    streets.nodes.retain(|n| !onto.contains_key(&n.id));
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
    for (d, i, j) in pairs {
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
        // a five-way star, unless they are so close their boxes would overlap.
        // Copies of one node (`min_degree` 0) always join.
        if min_degree >= 3 && joined > 4 && d >= STAR_M {
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
                && !road.given
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

/// Remove every dead-end branch of collectors and locals shorter than
/// `max_m`: from a free end along its street, through nodes where it only
/// continues, to the first junction. Arterials stay; a bridge on a branch
/// that leads nowhere goes with it.
fn prune_branches(streets: &mut ExternalStreets, max_m: f32) {
    loop {
        let degree = degrees(&streets.roads);
        let mut by_node: HashMap<u64, Vec<usize>> = HashMap::new();
        for (index, road) in streets.roads.iter().enumerate() {
            by_node.entry(road.from).or_default().push(index);
            by_node.entry(road.to).or_default().push(index);
        }
        let mut drop: HashSet<usize> = HashSet::new();
        for (&node, list) in &by_node {
            if degree.get(&node) != Some(&1) || drop.contains(&list[0]) {
                continue;
            }
            let (mut at, mut current) = (node, list[0]);
            let (mut branch, mut total, mut keep) = (Vec::new(), 0.0, false);
            loop {
                let road = &streets.roads[current];
                if rank(road.class) < 2 || road.from == road.to || road.given {
                    keep = true;
                    break;
                }
                branch.push(current);
                total += length(road);
                if total >= max_m {
                    keep = true;
                    break;
                }
                at = if road.from == at { road.to } else { road.from };
                if degree.get(&at) != Some(&2) {
                    // A junction (the branch ends here) or another free end
                    // (a lone piece; `drop_orphans` decides those).
                    keep |= degree.get(&at) == Some(&1);
                    break;
                }
                match by_node[&at].iter().find(|&&i| i != current) {
                    Some(&i) => current = i,
                    None => break,
                }
            }
            if !keep {
                drop.extend(branch);
            }
        }
        if drop.is_empty() {
            break;
        }
        let mut index = 0;
        streets.roads.retain(|_| {
            index += 1;
            !drop.contains(&(index - 1))
        });
    }
    let used: HashSet<u64> = streets.roads.iter().flat_map(|r| [r.from, r.to]).collect();
    streets.nodes.retain(|n| used.contains(&n.id));
}

/// The point of `line` nearest `p`: (distance, segment index, parameter, point).
fn nearest_on(line: &[(f32, f32)], p: (f32, f32)) -> Option<(f32, usize, f32, (f32, f32))> {
    let mut best: Option<(f32, usize, f32, (f32, f32))> = None;
    for (index, w) in line.windows(2).enumerate() {
        let (a, b) = (w[0], w[1]);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let l2 = dx * dx + dy * dy;
        let t = if l2 < 1e-9 {
            0.0
        } else {
            (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0)
        };
        let q = (a.0 + dx * t, a.1 + dy * t);
        let d = dist(p, q);
        if best.is_none_or(|b| d < b.0) {
            best = Some((d, index, t, q));
        }
    }
    best
}

/// Do the open segments `a b` and `c d` cross?
fn crosses(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32, f32)) -> bool {
    let side = |p: (f32, f32), q: (f32, f32), r: (f32, f32)| {
        (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0)
    };
    let (d1, d2) = (side(c, d, a), side(c, d, b));
    let (d3, d4) = (side(a, b, c), side(a, b, d));
    d1 * d2 < 0.0 && d3 * d4 < 0.0
}

fn point_of(p: (f32, f32)) -> Point {
    Point {
        x_km: p.0 / 1000.0,
        y_km: p.1 / 1000.0,
    }
}

/// Carry each dead end on to the street it points at, if one is close ahead:
/// to that street's nearer junction if one is within a cluster's reach of the
/// meeting point, or else into a new T junction that splits the street there.
/// An expressway is never met at grade and a bridge is never split.
/// A dead end to carry on: (end node, its road, the road it meets, that
/// road's segment, the meeting point).
type Join = (u64, usize, usize, usize, (f32, f32));

fn join_dead_ends(streets: &mut ExternalStreets, reach: f32, within_deg: f32) {
    let cos_limit = within_deg.to_radians().cos();
    let mut next_id = streets
        .nodes
        .iter()
        .map(|n| n.id)
        .max()
        .unwrap_or(0)
        .wrapping_add(1);
    // Each pass joins one dead end, then looks again: a join changes the streets.
    for _ in 0..streets.roads.len() {
        let degree = degrees(&streets.roads);
        let points: HashMap<u64, (f32, f32)> = streets
            .nodes
            .iter()
            .map(|n| (n.id, metres(n.point)))
            .collect();
        let lines: Vec<Vec<(f32, f32)>> = streets
            .roads
            .iter()
            .map(|r| r.centreline.iter().map(|p| metres(*p)).collect())
            .collect();
        let mut ends: Vec<(u64, usize)> = streets
            .roads
            .iter()
            .enumerate()
            .flat_map(|(index, road)| [(road.from, index), (road.to, index)])
            .filter(|(node, index)| {
                degree.get(node) == Some(&1)
                    && streets.roads[*index].class != ModernRoadClass::Expressway
                    && !streets.roads[*index].bridge
            })
            .collect();
        ends.sort_unstable();
        let mut join: Option<Join> = None;
        'ends: for (node, index) in ends {
            let Some(&end) = points.get(&node) else {
                continue;
            };
            let heading = leaving(&streets.roads[index], node);
            // Out of the end, away from the street's own body.
            let ahead = (-(heading.cos()), -(heading.sin()));
            let mut best: Option<(f32, usize, usize, (f32, f32))> = None;
            for (target, line) in lines.iter().enumerate() {
                let other = &streets.roads[target];
                if target == index || other.class == ModernRoadClass::Expressway {
                    continue;
                }
                // The nearest point of it ahead: a street meeting another at a
                // slant meets it ahead of its end, not abreast.
                for (segment, w) in line.windows(2).enumerate() {
                    let length = dist(w[0], w[1]);
                    let steps = ((length / 2.0).ceil() as usize).max(1);
                    for k in 0..=steps {
                        let f = k as f32 / steps as f32;
                        let q = (
                            w[0].0 + (w[1].0 - w[0].0) * f,
                            w[0].1 + (w[1].1 - w[0].1) * f,
                        );
                        let d = dist(end, q);
                        if !(0.5..=reach).contains(&d) {
                            continue;
                        }
                        let toward = ((q.0 - end.0) / d, (q.1 - end.1) / d);
                        if toward.0 * ahead.0 + toward.1 * ahead.1 < cos_limit {
                            continue;
                        }
                        if best.is_none_or(|b| d < b.0) {
                            best = Some((d, target, segment, q));
                        }
                    }
                }
            }
            let Some((_, target, segment, q)) = best else {
                continue;
            };
            // The extension must not cross another street on its way.
            for (other, line) in lines.iter().enumerate() {
                if other == index || other == target {
                    continue;
                }
                if line.windows(2).any(|w| crosses(end, q, w[0], w[1])) {
                    continue 'ends;
                }
            }
            if streets.roads[target].bridge {
                // Not split; met at its end only, below.
                let road = &streets.roads[target];
                let near_end = [road.from, road.to]
                    .into_iter()
                    .any(|n| points.get(&n).is_some_and(|p| dist(*p, q) < CLUSTER_M));
                if !near_end {
                    continue;
                }
            }
            join = Some((node, index, target, segment, q));
            break;
        }
        let Some((node, index, target, segment, q)) = join else {
            break;
        };
        // The junction it meets: the target's nearer end if close enough, else a
        // new node splitting the target at the meeting point.
        let target_road = &streets.roads[target];
        let near = [target_road.from, target_road.to]
            .into_iter()
            .filter_map(|n| points.get(&n).map(|p| (dist(*p, q), n)))
            .filter(|(d, _)| *d < CLUSTER_M)
            .min_by(|a, b| a.0.total_cmp(&b.0));
        let meet = match near {
            Some((_, n)) => n,
            None => {
                let id = next_id;
                next_id = next_id.wrapping_add(1);
                streets.nodes.push(super::ExternalNode {
                    id,
                    point: point_of(q),
                });
                let line = &lines[target];
                let mut first: Vec<Point> = line[..=segment].iter().map(|p| point_of(*p)).collect();
                first.push(point_of(q));
                let mut second = vec![point_of(q)];
                second.extend(line[segment + 1..].iter().map(|p| point_of(*p)));
                let (to, class, bridge, given) = (
                    target_road.to,
                    target_road.class,
                    target_road.bridge,
                    target_road.given,
                );
                streets.roads[target].to = id;
                streets.roads[target].centreline = first;
                streets.roads.push(ExternalRoad {
                    from: id,
                    to,
                    class,
                    bridge,
                    given,
                    centreline: second,
                });
                id
            }
        };
        let meet_point = streets
            .nodes
            .iter()
            .find(|n| n.id == meet)
            .map(|n| n.point)
            .unwrap_or(point_of(q));
        let road = &mut streets.roads[index];
        let at_start = road.from == node;
        if at_start {
            road.centreline.reverse();
        }
        carry_on(&mut road.centreline, meet_point);
        if at_start {
            road.centreline.reverse();
            road.from = meet;
        } else {
            road.to = meet;
        }
    }
    let used: HashSet<u64> = streets.roads.iter().flat_map(|r| [r.from, r.to]).collect();
    streets.nodes.retain(|n| used.contains(&n.id));
}

/// Carry the end of `line` on to `target` in one smooth curve: from 25 m back,
/// leaving the way the street ran, bending to arrive at `target`. Bolted on as
/// a straight piece it made a kink a few metres short of the junction, and a
/// kink there skews the street's end and everything painted across it.
fn carry_on(line: &mut Vec<Point>, target: Point) {
    const BACK_M: f32 = 25.0;
    let pts: Vec<(f32, f32)> = line.iter().map(|p| metres(*p)).collect();
    let Some(&end) = pts.last() else {
        return;
    };
    // Where the curve leaves the street: 25 m back, or its start.
    let mut kept = pts.len() - 1;
    let mut travelled = 0.0;
    while kept > 0 && travelled + dist(pts[kept - 1], pts[kept]) < BACK_M {
        travelled += dist(pts[kept - 1], pts[kept]);
        kept -= 1;
    }
    let from = pts[kept];
    let goal = metres(target);
    // The control point on the street's own heading past its old end, as far
    // again as half the way to the target, so the curve leaves tangent.
    let heading = if kept + 1 < pts.len() || kept > 0 {
        let before = if kept + 1 < pts.len() {
            from
        } else {
            pts[kept - 1]
        };
        let d = dist(before, end).max(1e-3);
        ((end.0 - before.0) / d, (end.1 - before.1) / d)
    } else {
        (0.0, 0.0)
    };
    let reach = dist(end, goal) * 0.5;
    let control = (end.0 + heading.0 * reach, end.1 + heading.1 * reach);
    let length = dist(from, control) + dist(control, goal);
    let steps = ((length / 3.0).ceil() as usize).max(2);
    line.truncate(kept + 1);
    for k in 1..=steps {
        let t = k as f32 / steps as f32;
        let u = 1.0 - t;
        let p = (
            u * u * from.0 + 2.0 * u * t * control.0 + t * t * goal.0,
            u * u * from.1 + 2.0 * u * t * control.1 + t * t * goal.1,
        );
        line.push(point_of(p));
    }
    if let Some(last) = line.last_mut() {
        *last = target;
    }
}

/// A street free at both ends joins nothing: a fragment a cut or a dropped
/// duplicate left. Before the dead ends are joined, the ones lying on top of
/// another street (within `DUPLICATE_M` of one for half their length, whatever
/// its class) go; after, the short ones that found nothing to join (`ORPHAN_M`).
/// A bridge stays.
fn drop_orphans(streets: &mut ExternalStreets, short: bool) {
    let degree = degrees(&streets.roads);
    let lines: Vec<Vec<(f32, f32)>> = streets
        .roads
        .iter()
        .map(|r| r.centreline.iter().map(|p| metres(*p)).collect())
        .collect();
    let orphan: Vec<bool> = streets
        .roads
        .iter()
        .enumerate()
        .map(|(index, road)| {
            if road.bridge || road.given || degree[&road.from] != 1 || degree[&road.to] != 1 {
                return false;
            }
            if short {
                return length(road) < ORPHAN_M;
            }
            let points = samples(road);
            if points.is_empty() {
                return length(road) < ORPHAN_M;
            }
            let on_another = points
                .iter()
                .filter(|(p, _)| {
                    lines.iter().enumerate().any(|(other, line)| {
                        other != index && nearest_on(line, *p).is_some_and(|n| n.0 < DUPLICATE_M)
                    })
                })
                .count();
            on_another * 2 >= points.len()
        })
        .collect();
    let mut index = 0;
    streets.roads.retain(|_| {
        let keep = !orphan[index];
        index += 1;
        keep
    });
    let used: HashSet<u64> = streets.roads.iter().flat_map(|r| [r.from, r.to]).collect();
    streets.nodes.retain(|n| used.contains(&n.id));
}

/// Where a street's corridor runs into another's (two streets converging
/// side by side, footway on carriageway, without ever crossing) the lesser
/// street is cut back to the longer of its clear parts, left as a dead end for
/// `join_dead_ends` to carry on to the other street in a proper T. Near a
/// junction the two share, converging is what streets do, and that is left.
fn cut_squeezed(streets: &mut ExternalStreets) {
    const STEP_M: f32 = 5.0;
    const BACK_M: f32 = 8.0;
    let mut next_id = streets
        .nodes
        .iter()
        .map(|n| n.id)
        .max()
        .unwrap_or(0)
        .wrapping_add(1);
    for _ in 0..streets.roads.len() {
        let points: HashMap<u64, (f32, f32)> = streets
            .nodes
            .iter()
            .map(|n| (n.id, metres(n.point)))
            .collect();
        let lines: Vec<Vec<(f32, f32)>> = streets
            .roads
            .iter()
            .map(|r| r.centreline.iter().map(|p| metres(*p)).collect())
            .collect();
        let half: Vec<f32> = streets
            .roads
            .iter()
            .map(|r| r.class.width_metres() * 0.5)
            .collect();
        let bounds: Vec<(f32, f32, f32, f32)> = lines
            .iter()
            .map(|l| {
                l.iter()
                    .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |b, p| {
                        (b.0.min(p.0), b.1.min(p.1), b.2.max(p.0), b.3.max(p.1))
                    })
            })
            .collect();
        let lesser = |a: usize, b: usize| {
            let (ra, rb) = (&streets.roads[a], &streets.roads[b]);
            (rank(ra.class), -length(ra)) > (rank(rb.class), -length(rb))
        };
        // (road, first and last squeezed arc, its length)
        let mut found: Option<(usize, f32, f32, f32)> = None;
        'roads: for (i, line) in lines.iter().enumerate() {
            let road = &streets.roads[i];
            if road.bridge || road.class == ModernRoadClass::Expressway {
                continue;
            }
            let total = length(road);
            let mut squeezed: Vec<f32> = Vec::new();
            for (j, other) in lines.iter().enumerate() {
                let o = &streets.roads[j];
                if j == i || !lesser(i, j) || o.class == ModernRoadClass::Expressway {
                    continue;
                }
                let reach = half[i] + half[j];
                let (a, b) = (bounds[i], bounds[j]);
                if a.0 > b.2 + reach || b.0 > a.2 + reach || a.1 > b.3 + reach || b.1 > a.3 + reach
                {
                    continue;
                }
                let shared: Vec<(f32, f32)> = [road.from, road.to]
                    .into_iter()
                    .filter(|n| *n == o.from || *n == o.to)
                    .filter_map(|n| points.get(&n).copied())
                    .collect();
                let mut travelled = 0.0;
                for w in line.windows(2) {
                    let segment = dist(w[0], w[1]);
                    let mut t = 0.0;
                    while t < segment {
                        let f = t / segment.max(1e-6);
                        let p = (
                            w[0].0 + (w[1].0 - w[0].0) * f,
                            w[0].1 + (w[1].1 - w[0].1) * f,
                        );
                        let near_shared = shared.iter().any(|q| dist(*q, p) < reach + 15.0);
                        if !near_shared && nearest_on(other, p).is_some_and(|n| n.0 < reach - 1.0) {
                            squeezed.push(travelled + t);
                        }
                        t += STEP_M;
                    }
                    travelled += segment;
                }
            }
            if let (Some(&lo), Some(&hi)) = (
                squeezed.iter().min_by(|a, b| a.total_cmp(b)),
                squeezed.iter().max_by(|a, b| a.total_cmp(b)),
            ) {
                found = Some((i, lo, hi, total));
                break 'roads;
            }
        }
        let Some((i, lo, hi, total)) = found else {
            break;
        };
        // Keep the longer clear part, ending a little short of the squeeze.
        let (keep_from, keep_to) = if lo >= total - hi {
            (0.0, (lo - BACK_M).max(0.0))
        } else {
            ((hi + BACK_M).min(total), total)
        };
        if keep_to - keep_from < STUB_M {
            streets.roads.remove(i);
            continue;
        }
        let line = &lines[i];
        let at = |s: f32| -> (f32, f32) {
            let mut travelled = 0.0;
            for w in line.windows(2) {
                let segment = dist(w[0], w[1]);
                if travelled + segment >= s {
                    let f = ((s - travelled) / segment.max(1e-6)).clamp(0.0, 1.0);
                    return (
                        w[0].0 + (w[1].0 - w[0].0) * f,
                        w[0].1 + (w[1].1 - w[0].1) * f,
                    );
                }
                travelled += segment;
            }
            *line.last().unwrap()
        };
        let mut kept: Vec<Point> = vec![point_of(at(keep_from))];
        let mut travelled = 0.0;
        for w in line.windows(2) {
            travelled += dist(w[0], w[1]);
            if travelled > keep_from + 0.5 && travelled < keep_to - 0.5 {
                kept.push(point_of(w[1]));
            }
        }
        kept.push(point_of(at(keep_to)));
        let id = next_id;
        next_id = next_id.wrapping_add(1);
        let road = &mut streets.roads[i];
        if keep_from <= 0.0 {
            streets.nodes.push(super::ExternalNode {
                id,
                point: *kept.last().unwrap(),
            });
            road.to = id;
        } else {
            streets
                .nodes
                .push(super::ExternalNode { id, point: kept[0] });
            road.from = id;
        }
        road.centreline = kept;
    }
    let used: HashSet<u64> = streets.roads.iter().flat_map(|r| [r.from, r.to]).collect();
    streets.nodes.retain(|n| used.contains(&n.id));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modern::ExternalNode;

    fn at(x: f32, y: f32) -> Point {
        Point {
            x_km: x / 1000.0,
            y_km: y / 1000.0,
        }
    }

    fn streets(
        nodes: &[(u64, f32, f32)],
        roads: &[(u64, u64, ModernRoadClass)],
    ) -> ExternalStreets {
        let place: HashMap<u64, Point> = nodes.iter().map(|&(id, x, y)| (id, at(x, y))).collect();
        ExternalStreets {
            nodes: nodes
                .iter()
                .map(|&(id, x, y)| ExternalNode {
                    id,
                    point: at(x, y),
                })
                .collect(),
            roads: roads
                .iter()
                .map(|&(from, to, class)| ExternalRoad {
                    from,
                    to,
                    class,
                    bridge: false,
                    given: false,
                    centreline: vec![place[&from], place[&to]],
                })
                .collect(),
            river: Vec::new(),
            tributaries: Vec::new(),
        }
    }

    #[test]
    fn a_triangle_of_short_streets_between_three_junctions_loses_its_least_street() {
        use ModernRoadClass::{Collector, Local};
        // all2/plan12: junctions 38, 29 and 33 m apart, each with two more arms.
        let mut s = streets(
            &[
                (1, 0.0, 0.0),
                (2, 27.0, -27.0),
                (3, 33.0, 1.0),
                (11, -200.0, 0.0),
                (12, -50.0, 200.0),
                (21, 30.0, -200.0),
                (22, 200.0, -100.0),
                (31, 200.0, 50.0),
                (32, 50.0, 200.0),
            ],
            &[
                (1, 2, Collector),
                (2, 3, Collector),
                (1, 3, Local),
                (1, 11, Collector),
                (1, 12, Local),
                (2, 21, Collector),
                (2, 22, Collector),
                (3, 31, Local),
                (3, 32, Local),
            ],
        );
        break_knots(&mut s);
        assert_eq!(s.roads.len(), 8);
        assert!(
            !s.roads.iter().any(|r| (r.from, r.to) == (1, 3)),
            "the local side goes"
        );
    }

    #[test]
    fn a_street_bends_on_an_arc_of_its_class_radius_and_leaves_its_ends_straight() {
        // all1/plan2: a collector turning 26 degrees 23 m from its end.
        let mut s = streets(
            &[(1, 0.0, 0.0), (2, 100.0, 0.0)],
            &[(1, 2, ModernRoadClass::Collector)],
        );
        let turn = 26.0_f32.to_radians();
        s.roads[0].centreline = vec![
            at(0.0, 0.0),
            at(23.0, 0.0),
            at(23.0 + 60.0 * turn.cos(), 60.0 * turn.sin()),
        ];
        round_bends(&mut s);
        let line: Vec<(f32, f32)> = s.roads[0].centreline.iter().map(|p| metres(*p)).collect();
        assert!(line.len() > 4, "the corner is an arc");
        assert_eq!(line[0], (0.0, 0.0));
        let heading = |a: (f32, f32), b: (f32, f32)| (b.1 - a.1).atan2(b.0 - a.0);
        assert!(
            heading(line[0], line[1]).abs() < 1.0e-3,
            "straight out of its end"
        );
        for w in line.windows(3) {
            let bend = (heading(w[1], w[2]) - heading(w[0], w[1])).abs();
            assert!(
                bend < 4.0_f32.to_radians(),
                "no vertex turns {:.1} degrees",
                bend.to_degrees()
            );
        }
    }

    #[test]
    fn a_branch_leading_nowhere_goes_back_to_its_junction_and_an_arterial_stays() {
        use ModernRoadClass::{Arterial, Collector, Local};
        let mut s = streets(
            &[
                (1, 0.0, 0.0),
                (2, 200.0, 0.0),
                (3, 400.0, 0.0),
                (4, 200.0, 200.0),
                (5, 200.0, 400.0),
                (6, 200.0, -300.0),
                (7, -300.0, 0.0),
            ],
            &[
                (1, 2, Collector),
                (2, 3, Collector),
                (2, 4, Local),
                (4, 5, Local),
                (2, 6, Arterial),
                (7, 1, Collector),
            ],
        );
        prune_branches(&mut s, 800.0);
        let kept: Vec<(u64, u64)> = s.roads.iter().map(|r| (r.from, r.to)).collect();
        // 2-3 (200 m) and 2-4-5 (400 m) lead nowhere; 7-1-2 (500 m) too.
        assert!(!kept.contains(&(2, 3)) && !kept.contains(&(4, 5)) && !kept.contains(&(7, 1)));
        assert!(
            kept.contains(&(2, 6)),
            "the arterial (a regional road) stays"
        );
    }

    #[test]
    fn two_free_ends_a_few_metres_apart_are_one_street() {
        use ModernRoadClass::Collector;
        // all1/plan12: a bridge's two halves stopped 3.6 m apart.
        let mut s = streets(
            &[
                (1, 0.0, 0.0),
                (2, 100.0, 0.0),
                (3, 103.0, 2.0),
                (4, 200.0, 0.0),
            ],
            &[(1, 2, Collector), (3, 4, Collector)],
        );
        join_facing_ends(&mut s);
        let degree = degrees(&s.roads);
        assert_eq!(
            degree.values().filter(|d| **d == 1).count(),
            2,
            "only the street's two real ends"
        );
    }
}
