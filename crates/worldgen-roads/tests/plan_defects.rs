//! What is wrong with a town's street plan, counted on real output: streets that end
//! just short of another street (a junction that failed to join), streets that stop in
//! the open, and the size of the blocks they enclose. Prints the numbers; run with
//! `--nocapture`.

use std::collections::BTreeMap;
use std::sync::Arc;

use worldgen_contracts::{NodeId, RoadNetwork, RoadTile, V2, v2};
use worldgen_core::{Cell, Engine, Frame, Seed};
use worldgen_roads::quad::{QUADS, Quad};
use worldgen_roads::{Fields, HashedTowns, ROADS, RoadsConfig, engine};

const FRAME: Frame = Frame::new([0.0, 0.0], 1_048_576.0);
/// Centre of a dense city in the seed-7 world (as in `network.rs`).
const CITY: V2 = v2(20000.0, 2400.0);
/// Only defects this far from the centre are counted, clear of the clipped tile borders.
const INNER_M: f64 = 900.0;

fn world(seed: u64) -> Engine {
    let config = RoadsConfig::default();
    engine(
        Seed::new(seed),
        FRAME,
        config,
        Fields::new(HashedTowns::shared(Seed::new(seed))),
    )
    .unwrap()
}

fn tiles(e: &Engine, level: u8, centre: V2, half: f64) -> Vec<Arc<RoadTile>> {
    let (lo, hi) = (
        Cell::containing(&FRAME, [centre.x - half, centre.y - half], level),
        Cell::containing(&FRAME, [centre.x + half, centre.y + half], level),
    );
    let mut out = Vec::new();
    for j in lo.y..=hi.y {
        for i in lo.x..=hi.x {
            out.push(e.get::<RoadTile>(ROADS, Cell::new(level, i, j)).unwrap());
        }
    }
    out
}

fn network(tiles: &[Arc<RoadTile>]) -> RoadNetwork {
    RoadNetwork::assemble(tiles.iter().map(|t| &**t)).expect("tiles must merge")
}

fn inside(p: V2) -> bool {
    (p.x - CITY.x).abs() < INNER_M && (p.y - CITY.y).abs() < INNER_M
}

fn percentile(sorted: &[f64], q: f64) -> f64 {
    sorted[((sorted.len() - 1) as f64 * q) as usize]
}

/// A measurement, not a check: run with `--ignored --nocapture` to print the counts.
#[test]
#[ignore = "diagnostic: run with --ignored --nocapture"]
fn print_plan_defects() {
    let e = world(7);
    let net = network(&tiles(&e, 11, CITY, 1500.0));

    let mut degree: BTreeMap<NodeId, usize> = BTreeMap::new();
    for ed in net.edges.values() {
        *degree.entry(ed.a).or_default() += 1;
        *degree.entry(ed.b).or_default() += 1;
    }

    // A street end inside the window, and the nearest other street to it.
    let mut ends = 0;
    let mut near_miss = Vec::new();
    let mut open_stub = 0;
    for (id, node) in &net.nodes {
        if degree.get(id) != Some(&1) || !inside(node.position) {
            continue;
        }
        ends += 1;
        let own = net
            .edges
            .values()
            .find(|ed| ed.a == *id || ed.b == *id)
            .unwrap();
        let gap = net
            .edges
            .values()
            .filter(|ed| ed.id != own.id)
            .flat_map(|ed| ed.pieces.iter())
            .filter_map(|pl| pl.closest(node.position).map(|(d, _)| d))
            .fold(f64::INFINITY, f64::min);
        if gap <= 30.0 {
            near_miss.push(gap);
        } else {
            open_stub += 1;
        }
    }
    near_miss.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("street ends inside the window: {ends}");
    println!(
        "  ending within 30 m of another street (a junction not joined): {}",
        near_miss.len()
    );
    if !near_miss.is_empty() {
        println!(
            "    gap to that street: median {:.1} m, max {:.1} m",
            percentile(&near_miss, 0.5),
            near_miss.last().unwrap()
        );
    }
    println!("  ending in the open, more than 30 m from any street: {open_stub}");

    // Blocks of the town's own streets, inside the window.
    let cell = Cell::containing(&FRAME, [CITY.x, CITY.y], 9);
    let mut areas = Vec::new();
    let mut long_sides = Vec::new();
    let mut slivers = 0;
    for dj in -1..=1 {
        for di in -1..=1 {
            let q = e.get::<Quad>(QUADS, cell.neighbour(di, dj)).unwrap();
            for b in &q.blocks {
                if b.rung > 2 {
                    continue;
                }
                let c = b.corners;
                let (mx, my) = (
                    (c[0].x + c[1].x + c[2].x + c[3].x) / 4.0,
                    (c[0].y + c[1].y + c[2].y + c[3].y) / 4.0,
                );
                if !inside(v2(mx, my)) {
                    continue;
                }
                let area = 0.5 * ((c[2] - c[0]).cross(c[3] - c[1])).abs();
                let sides = [
                    c[0].dist(c[1]),
                    c[1].dist(c[2]),
                    c[2].dist(c[3]),
                    c[3].dist(c[0]),
                ];
                let lo = sides.iter().copied().fold(f64::INFINITY, f64::min);
                let hi = sides.iter().copied().fold(0.0, f64::max);
                slivers += usize::from(hi > 6.0 * lo);
                areas.push(area);
                long_sides.push(hi);
            }
        }
    }
    areas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    long_sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("blocks inside the window: {}", areas.len());
    println!(
        "  area m²: median {:.0}, p90 {:.0}",
        percentile(&areas, 0.5),
        percentile(&areas, 0.9)
    );
    println!(
        "  longest side m: median {:.0}, p90 {:.0}, over 250 m: {}",
        percentile(&long_sides, 0.5),
        percentile(&long_sides, 0.9),
        long_sides.iter().filter(|s| **s > 250.0).count()
    );
    println!("  slivers (longest side over 6x the shortest): {slivers}");
}
