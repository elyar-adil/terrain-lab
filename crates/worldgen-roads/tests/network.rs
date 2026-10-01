//! The promises of the road layer, checked on real output.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use worldgen_contracts::{
    DryLand, NodeId, Polyline, PolylineRiver, RoadClass, RoadNetwork, RoadTile, UrbanField, V2, segment_intersection, v2,
};
use worldgen_core::{Cell, Engine, Frame, Seed};
use worldgen_roads::quad::{QUADS, Quad};
use worldgen_roads::{Fields, HashedTowns, ROADS, RoadsConfig, engine};

const FRAME: Frame = Frame::new([0.0, 0.0], 1_048_576.0);
/// Centre of a dense city in the seed-7 world (found by scanning the town field).
const CITY: V2 = v2(20000.0, 2400.0);

fn world(seed: u64, min_class: RoadClass) -> Engine {
    let config = RoadsConfig { min_class, ..RoadsConfig::default() };
    engine(Seed::new(seed), FRAME, config, Fields::new(HashedTowns::shared(Seed::new(seed)))).unwrap()
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
    RoadNetwork::assemble(tiles.iter().map(|t| &**t)).expect("tiles must merge without a seam conflict")
}

#[test]
fn the_same_place_is_the_same_whatever_was_asked_first() {
    let forward = world(7, RoadClass::Track);
    let backward = world(7, RoadClass::Track);
    let origin = Cell::containing(&FRAME, CITY.to_array(), 11);
    let cells: Vec<Cell> = (0..4).flat_map(|j| (0..4).map(move |i| origin.neighbour(i - 1, j - 1))).collect();
    let a: Vec<_> = cells.iter().map(|c| forward.get::<RoadTile>(ROADS, *c).unwrap()).collect();
    let b: Vec<_> = cells.iter().rev().map(|c| backward.get::<RoadTile>(ROADS, *c).unwrap()).collect();
    for (x, y) in a.iter().zip(b.iter().rev()) {
        assert_eq!(**x, **y);
    }
    assert!(a.iter().map(|t| t.edges.len()).sum::<usize>() > 100, "the fixture has roads");
}

#[test]
fn a_big_tile_is_exactly_the_sum_of_the_tiles_inside_it() {
    let e = world(7, RoadClass::Track);
    let parent = Cell::containing(&FRAME, CITY.to_array(), 11);
    let big = e.get::<RoadTile>(ROADS, parent).unwrap();
    let parts: Vec<_> = parent.children().iter().map(|c| e.get::<RoadTile>(ROADS, *c).unwrap()).collect();
    let whole = network(&[big.clone()]);
    let merged = network(&parts);
    assert!(!whole.edges.is_empty());
    assert_eq!(whole.edges.keys().collect::<Vec<_>>(), merged.edges.keys().collect::<Vec<_>>());
    for (id, edge) in &whole.edges {
        let other = &merged.edges[id];
        assert_eq!((edge.a, edge.b, edge.class, edge.setting), (other.a, other.b, other.class, other.setting));
        // The same line, however it was cut: same pieces, to the bit.
        let mut x: Vec<&Polyline> = edge.pieces.iter().collect();
        let mut y: Vec<&Polyline> = other.pieces.iter().collect();
        let key = |p: &&Polyline| (p.0[0].x.to_bits(), p.0[0].y.to_bits());
        x.sort_by_key(key);
        y.sort_by_key(key);
        assert_eq!(x.len(), y.len(), "edge {id:?} is in a different number of pieces");
        for (whole_piece, cut_piece) in x.iter().zip(&y) {
            // Cutting adds a vertex where the line crosses a tile border, on the
            // line itself, and changes nothing else: every original vertex survives,
            // every extra one lies on the original line, and the length is the same.
            for v in &whole_piece.0 {
                assert!(cut_piece.0.contains(v), "edge {id:?} lost the vertex {v:?}");
            }
            for v in &cut_piece.0 {
                let off = whole_piece.closest(*v).unwrap().0;
                assert!(off < 1e-9, "edge {id:?} gained a vertex {off} m off the line");
            }
            assert!((whole_piece.length() - cut_piece.length()).abs() < 1e-6);
        }
    }
    assert_eq!(whole.nodes, merged.nodes);
}

#[test]
fn zooming_out_only_removes_streets_it_never_moves_or_changes_them() {
    let fine = world(7, RoadClass::Track);
    let coarse = world(7, RoadClass::Collector);
    let f = network(&tiles(&fine, 11, CITY, 1500.0));
    let c = network(&tiles(&coarse, 11, CITY, 1500.0));
    assert!(!c.edges.is_empty() && c.edges.len() < f.edges.len(), "{} vs {}", c.edges.len(), f.edges.len());
    for (id, edge) in &c.edges {
        assert!(edge.class >= RoadClass::Collector);
        assert_eq!(edge, &f.edges[id], "a collector changed when finer streets were added");
    }
    assert!(f.edges.values().any(|e| e.class < RoadClass::Collector));
}

#[test]
fn no_two_roads_cross_except_at_a_junction() {
    let e = world(7, RoadClass::Track);
    let net = network(&tiles(&e, 11, CITY, 2200.0));
    let segs: Vec<(u64, V2, V2)> = net
        .edges
        .values()
        .flat_map(|ed| ed.pieces.iter().flat_map(move |p| p.0.windows(2).map(move |w| (ed.id.0, w[0], w[1]))))
        .collect();
    let mut crossings = Vec::new();
    for (i, a) in segs.iter().enumerate() {
        for b in &segs[i + 1..] {
            if a.0 == b.0 {
                continue;
            }
            if let Some((p, t, u)) = segment_intersection(a.1, a.2, b.1, b.2) {
                let interior = |t: f64| t > 1e-6 && t < 1.0 - 1e-6;
                // Meeting at an end of both is a junction; one of them continuing through is a crossing.
                if interior(t) || interior(u) {
                    crossings.push((a.0, b.0, p));
                }
            }
        }
    }
    assert!(segs.len() > 500, "{} segments", segs.len());
    assert!(crossings.is_empty(), "{} unmarked crossings, first at {:?}", crossings.len(), crossings.first());
}

#[test]
fn a_town_is_one_connected_network() {
    let e = world(7, RoadClass::Track);
    let net = network(&tiles(&e, 11, CITY, 1200.0));
    let mut parent: BTreeMap<NodeId, NodeId> = net.nodes.keys().map(|k| (*k, *k)).collect();
    fn find(p: &mut BTreeMap<NodeId, NodeId>, x: NodeId) -> NodeId {
        let q = p[&x];
        if q == x {
            return x;
        }
        let r = find(p, q);
        p.insert(x, r);
        r
    }
    for ed in net.edges.values() {
        let (a, b) = (find(&mut parent, ed.a), find(&mut parent, ed.b));
        parent.insert(a, b);
    }
    let mut size: BTreeMap<NodeId, usize> = BTreeMap::new();
    for ed in net.edges.values() {
        *size.entry(find(&mut parent, ed.a)).or_default() += 1;
    }
    let biggest = size.values().copied().max().unwrap();
    assert!(biggest as f64 >= 0.97 * net.edges.len() as f64, "{biggest} of {} edges are in the main network", net.edges.len());
}

#[test]
fn no_road_is_a_stub_a_few_metres_long_and_junctions_are_spaced_like_streets() {
    let e = world(7, RoadClass::Track);
    let net = network(&tiles(&e, 11, CITY, 1500.0));
    let lengths: Vec<f64> = net.edges.values().map(|ed| ed.pieces.iter().map(Polyline::length).sum::<f64>()).collect();
    // Edges clipped by a tile border can be short; judge whole edges only, the ones fully inside.
    let whole: Vec<f64> = net
        .edges
        .values()
        .filter(|ed| ed.pieces.len() == 1 && net.nodes[&ed.a].position.dist(ed.pieces[0].first().unwrap()) < 1e-6 && net.nodes[&ed.b].position.dist(ed.pieces[0].last().unwrap()) < 1e-6)
        .map(|ed| ed.pieces[0].length())
        .collect();
    assert!(whole.len() > 200, "{} whole edges of {}", whole.len(), lengths.len());
    let shortest = whole.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(shortest >= 12.0, "an edge only {shortest:.1} m long");
    let mut sorted = whole.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = sorted[sorted.len() / 2];
    assert!((40.0..260.0).contains(&median), "median edge {median:.0} m");
}

#[test]
fn roads_bend_gently_and_never_double_back() {
    let e = world(7, RoadClass::Track);
    let net = network(&tiles(&e, 11, CITY, 1500.0));
    let worst = net.edges.values().flat_map(|ed| ed.pieces.iter()).map(Polyline::max_turn).fold(0.0, f64::max);
    assert!(worst < 0.5, "a road turns {worst:.2} rad inside one edge");
}

#[test]
fn the_hierarchy_follows_how_built_up_a_place_is() {
    let e = world(7, RoadClass::Track);
    let city = network(&tiles(&e, 11, CITY, 1000.0));
    let towns = HashedTowns::new(Seed::new(7));
    // The middle of the country: the least built-up place of a coarse scan.
    let mut empty = v2(0.0, 0.0);
    for k in 0..4000 {
        let p = v2(60000.0 + (k % 63) as f64 * 700.0, 60000.0 + (k / 63) as f64 * 700.0);
        if towns.urbanness(p) == 0.0 && towns.urbanness(p + v2(1500.0, 1500.0)) == 0.0 {
            empty = p;
            break;
        }
    }
    let country = network(&tiles(&e, 11, empty + v2(750.0, 750.0), 700.0));
    let by_class = |n: &RoadNetwork| {
        let mut m: BTreeMap<RoadClass, usize> = BTreeMap::new();
        for ed in n.edges.values() {
            *m.entry(ed.class).or_default() += 1;
        }
        m
    };
    let (c, r) = (by_class(&city), by_class(&country));
    for class in [RoadClass::Arterial, RoadClass::Collector, RoadClass::Local, RoadClass::Service] {
        assert!(c.get(&class).copied().unwrap_or(0) > 0, "the city has no {class:?}: {c:?}");
    }
    assert!(city.edges.len() > 10 * country.edges.len().max(1), "city {} country {}: {r:?}", city.edges.len(), country.edges.len());
    assert!(r.get(&RoadClass::Local).copied().unwrap_or(0) + r.get(&RoadClass::Service).copied().unwrap_or(0) == 0 || r.keys().all(|k| *k <= RoadClass::Collector || *k == RoadClass::Local || *k == RoadClass::Service || *k == RoadClass::Track));
}

#[test]
fn different_places_get_different_roads() {
    let e = world(7, RoadClass::Track);
    let other = world(8, RoadClass::Track);
    let a = network(&tiles(&e, 11, CITY, 1500.0));
    let b = network(&tiles(&other, 11, CITY, 1500.0));
    assert_ne!(a.edges.len(), b.edges.len());
    let ids: BTreeSet<_> = a.edges.keys().collect();
    assert!(b.edges.keys().filter(|k| ids.contains(k)).count() * 20 < b.edges.len().max(1));
}

#[test]
fn city_blocks_have_a_realistic_size_and_most_of_a_block_is_not_a_sliver() {
    let e = world(7, RoadClass::Track);
    let cell = Cell::containing(&FRAME, [CITY.x, CITY.y], 9);
    let mut areas = Vec::new();
    let mut slivers = 0;
    for dj in -1..=1 {
        for di in -1..=1 {
            let q = e.get::<Quad>(QUADS, cell.neighbour(di, dj)).unwrap();
            for b in &q.blocks {
                if b.rung > 1 {
                    continue; // only blocks of the town's own streets
                }
                let c = b.corners;
                let area = 0.5 * ((c[2] - c[0]).cross(c[3] - c[1])).abs();
                let sides = [c[0].dist(c[1]), c[1].dist(c[2]), c[2].dist(c[3]), c[3].dist(c[0])];
                let (lo, hi) = (sides.iter().copied().fold(f64::INFINITY, f64::min), sides.iter().copied().fold(0.0, f64::max));
                slivers += usize::from(hi > 6.0 * lo);
                areas.push(area);
            }
        }
    }
    assert!(areas.len() > 40, "{} town blocks", areas.len());
    areas.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = areas[areas.len() / 2];
    // 100 to 300 m on a side: a hectare or three.
    assert!((4_000.0..60_000.0).contains(&median), "median block {median:.0} m²");
    assert!(slivers * 8 < areas.len(), "{slivers} of {} blocks are slivers", areas.len());
}

#[test]
fn water_is_crossed_by_bridges_on_big_roads_and_not_by_small_ones() {
    let river = PolylineRiver { id: 1, line: vec![v2(CITY.x - 3000.0, CITY.y + 300.0), v2(CITY.x + 3000.0, CITY.y + 300.0)], width_m: 40.0 };
    let fields = Fields::new(HashedTowns::shared(Seed::new(7))).with_water(Arc::new(river.clone()));
    let e = engine(Seed::new(7), FRAME, RoadsConfig::default(), fields).unwrap();
    let net = network(&tiles(&e, 11, CITY, 2000.0));
    let mut bridges = 0;
    for ed in net.edges.values() {
        let wet = ed.pieces.iter().flat_map(|p| p.resampled(3.0).0).filter(|p| worldgen_contracts::WaterField::is_water(&river, *p)).count();
        if wet > 2 {
            assert!(!ed.spans.is_empty(), "{:?} runs through the river without a bridge", ed.class);
            assert!(ed.class >= RoadClass::Collector);
            bridges += 1;
        }
    }
    assert!(bridges >= 2, "{bridges} bridges");
    let dry = world(7, RoadClass::Track);
    let dry_net = network(&tiles(&dry, 11, CITY, 2000.0));
    assert!(dry_net.edges.len() > net.edges.len(), "the river cost the small roads that could not cross it");
    let _ = DryLand;
}
