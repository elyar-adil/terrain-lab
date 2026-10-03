//! The world seen by the roads layer, and the roads layer's output seen by a city.
//!
//! `worldgen-roads` knows nothing about this project's terrain or settlements: it
//! asks for how built up a place is, how high the ground is, where water is, and
//! which roads were already routed. This module answers those from the world, runs
//! the layer, and hands a town's streets to the city planner.

use std::sync::Arc;

use terrain_core::TerrainData;
use world_core::{GridPoint, WorldGrid};
use worldgen_contracts::{
    EdgeSource, HeightField, NodeId, PinnedRoad, PinnedSet, Polyline, RoadClass, RoadNetwork,
    RoadTile, UrbanField, V2, v2,
};
use worldgen_core::hash::hash_words;
use worldgen_core::noise::fbm;
use worldgen_core::{Cell, Engine, Frame, Seed};
use worldgen_roads::shape::round_corners;
use worldgen_roads::{Fields, ROADS, RoadsConfig};

use crate::rivers::{Rivers, WorldWater};
use crate::{Road, SettlementClass, SettlementSite, settlement_radius_km};

/// Height of the ground at any point: the terrain's grid, interpolated.
pub struct GridHeight {
    size: usize,
    cell_m: f64,
    height: Vec<f32>,
}

impl GridHeight {
    pub fn new(terrain: &TerrainData, grid: WorldGrid) -> Self {
        Self {
            size: grid.size,
            cell_m: f64::from(grid.cell_metres()),
            height: terrain.height.clone(),
        }
    }
}

impl HeightField for GridHeight {
    fn height_m(&self, p: V2) -> f64 {
        let n = self.size;
        let (fx, fy) = (
            (p.x / self.cell_m).clamp(0.0, (n - 1) as f64),
            (p.y / self.cell_m).clamp(0.0, (n - 1) as f64),
        );
        let (x0, y0) = (fx.floor() as usize, fy.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(n - 1), (y0 + 1).min(n - 1));
        let (tx, ty) = (fx - x0 as f64, fy - y0 as f64);
        let v = |x: usize, y: usize| f64::from(self.height[y * n + x]);
        let top = v(x0, y0) + (v(x1, y0) - v(x0, y0)) * tx;
        let bottom = v(x0, y1) + (v(x1, y1) - v(x0, y1)) * tx;
        top + (bottom - top) * ty
    }
}

/// One settlement's pull on the urban field.
struct Site {
    centre: V2,
    radius_m: f64,
    /// How built up its heart gets: a regional centre is wall to wall, a village is not.
    strength: f64,
    /// How tall and commercial its heart gets, from a low village to a downtown.
    weight: f64,
    phase: f64,
    seed: Seed,
    /// Directions (angle, strength) in which the town runs out along its roads.
    lobes: Vec<(f64, f64)>,
    core: V2,
    sub_core: V2,
}

/// How built up the world is: the settlements, each with a ragged outline that runs
/// out along the roads that feed it and thins into scattered houses and then fields.
pub struct SettlementUrban {
    sites: Vec<Site>,
}

impl SettlementUrban {
    pub fn new(
        settlements: &[SettlementSite],
        grid: WorldGrid,
        roads: &[Polyline],
        road_class: &[RoadClass],
        seed: u64,
    ) -> Self {
        let cell_m = f64::from(grid.cell_metres());
        let sites = settlements
            .iter()
            .map(|s| {
                let centre = v2(s.location.x as f64 * cell_m, s.location.y as f64 * cell_m);
                let radius_m = f64::from(settlement_radius_km(s.class)) * 1000.0;
                let site_seed = Seed::new(seed).derive("site").derive_u64(u64::from(s.id));
                // Even a village has streets of its own: what differs is its size,
                // and how tall it grows (see `intensity`).
                let strength = 1.0;
                let weight = match s.class {
                    SettlementClass::RegionalCentre => 1.0,
                    SettlementClass::Town => 0.8,
                    SettlementClass::Village => 0.35,
                };
                // Lobes: where a road crosses 0.9 of the radius.
                let mut lobes = Vec::new();
                for (road, class) in roads.iter().zip(road_class) {
                    let target = radius_m * 0.9;
                    let best = road.0.iter().min_by(|a, b| {
                        (a.dist(centre) - target)
                            .abs()
                            .total_cmp(&(b.dist(centre) - target).abs())
                    });
                    if let Some(p) =
                        best.filter(|p| (p.dist(centre) - target).abs() < radius_m * 0.5)
                    {
                        let gain = match class {
                            RoadClass::Motorway => 1.0,
                            RoadClass::Arterial => 0.8,
                            RoadClass::Collector => 0.5,
                            _ => 0.25,
                        };
                        lobes.push(((*p - centre).angle(), gain));
                    }
                }
                let place = |salt: &str, lo: f64, hi: f64| {
                    let h = site_seed.derive(salt);
                    let (a, r) = (
                        h.derive("a").unit() * std::f64::consts::TAU,
                        radius_m * (lo + (hi - lo) * h.derive("r").unit()),
                    );
                    centre + V2::from_angle(a) * r
                };
                Site {
                    centre,
                    radius_m,
                    strength,
                    weight,
                    phase: site_seed.derive("phase").unit() * std::f64::consts::TAU,
                    seed: site_seed,
                    lobes,
                    core: place("core", 0.04, 0.32),
                    sub_core: place("sub", 0.35, 0.65),
                }
            })
            .collect();
        Self { sites }
    }

    /// How built-up the ground is at distance `r` from the centre where the town's outline
    /// reaches `reach`: solid inside the town, then a long tail of suburb and scattered
    /// farms. The tail is what a real edge is made of; it ends at 1.4 reach plus a
    /// quarter kilometre, so even a village has a few hundred metres of it.
    fn profile(reach: f64, r: f64) -> f64 {
        let plateau = 0.6 * reach;
        let end = 1.4 * reach + 250.0;
        let x = ((end - r) / (end - plateau)).clamp(0.0, 1.0);
        x.powf(1.5)
    }

    /// The town's outline: how far its built-up area reaches in the direction of `p`.
    fn reach(site: &Site, p: V2) -> f64 {
        use std::f64::consts::{PI, TAU};
        let d = p - site.centre;
        let th = d.angle();
        let ph = site.phase;
        let lump = 0.66
            + 0.12 * (2.0 * th + ph).sin()
            + 0.07 * (3.0 * th + ph * 1.9).sin()
            + 0.04 * (5.0 * th - ph * 0.7).sin();
        let mut reach = site.radius_m * lump;
        for &(angle, strength) in &site.lobes {
            let mut a = (th - angle).abs() % TAU;
            if a > PI {
                a = TAU - a;
            }
            reach += site.radius_m * 0.42 * strength * (-(a / 0.38).powi(2)).exp();
        }
        // The outline wanders by a block or two.
        let jag = 0.84 + 0.32 * fbm(site.seed.derive("jag"), p.x / 260.0, p.y / 260.0, 3, 0.5);
        reach * jag
    }
}

impl UrbanField for SettlementUrban {
    fn urbanness(&self, p: V2) -> f64 {
        let mut best = 0.0_f64;
        for site in &self.sites {
            let r = p.dist(site.centre);
            if r > site.radius_m * 2.0 {
                continue;
            }
            let reach = Self::reach(site, p);
            best = best.max(site.strength * Self::profile(reach, r));
        }
        best
    }

    fn intensity(&self, p: V2) -> f64 {
        let mut best = 0.0_f64;
        for site in &self.sites {
            let r = p.dist(site.centre);
            if r > site.radius_m * 2.0 {
                continue;
            }
            let g = |c: V2, s: f64| (-(p.dist(c) / (site.radius_m * s)).powi(2)).exp();
            let base =
                g(site.centre + (site.core - site.centre), 0.5).max(0.62 * g(site.sub_core, 0.28));
            best = best.max(site.weight * base * self.urbanness(p).max(0.0).sqrt());
        }
        best
    }
}

/// The corner radius a router's staircase is rounded to, by road class: a
/// motorway is built to a much gentler curve than a village lane.
fn corner_radius_m(class: crate::RoadClass) -> f64 {
    match class {
        crate::RoadClass::Motorway => 700.0,
        crate::RoadClass::Arterial => 450.0,
        crate::RoadClass::Collector => 280.0,
        crate::RoadClass::Local => 170.0,
        crate::RoadClass::Rural => 110.0,
    }
}

/// A regional road as a smooth centreline in world metres. The router's grid
/// path turns in steps; this rounds them to the curvature the class is built to.
pub fn road_centreline_m(road: &Road, grid: WorldGrid) -> Polyline {
    let cell_m = f64::from(grid.cell_metres());
    let raw = Polyline(
        road.path
            .iter()
            .map(|p: &GridPoint| v2(p.x as f64 * cell_m, p.y as f64 * cell_m))
            .collect(),
    );
    round_corners(&raw, corner_radius_m(road.class), 20.0)
}

/// The world's road layer, ready to answer for any window.
pub struct WorldFabric {
    engine: Engine,
    pub rivers: Arc<Rivers>,
    urban: Arc<SettlementUrban>,
    centres: Vec<V2>,
    radii_m: Vec<f64>,
    frame: Frame,
    lattice_cell_m: f64,
}

impl WorldFabric {
    pub fn new(
        terrain: &TerrainData,
        grid: WorldGrid,
        settlements: &[SettlementSite],
        roads: &[Road],
        seed: u64,
    ) -> Result<Self, worldgen_core::Error> {
        let centrelines: Vec<Polyline> = roads.iter().map(|r| road_centreline_m(r, grid)).collect();
        let classes: Vec<RoadClass> = roads.iter().map(|r| r.class.contract()).collect();
        let urban = Arc::new(SettlementUrban::new(
            settlements,
            grid,
            &centrelines,
            &classes,
            seed,
        ));
        let pinned = PinnedSet::new(
            roads
                .iter()
                .zip(&centrelines)
                .map(|(r, line)| PinnedRoad {
                    id: hash_words(&[u64::from(r.id), 0x9A7]),
                    class: r.class.contract(),
                    path: line.clone(),
                })
                .collect(),
        );
        let rivers = Arc::new(Rivers::from_terrain(terrain, grid));
        let water = WorldWater::new(terrain, grid, rivers.clone());
        let fields = Fields::new(urban.clone())
            .with_water(Arc::new(water))
            .with_height(Arc::new(GridHeight::new(terrain, grid)))
            .with_pinned(Arc::new(pinned));
        // The lattice cell is 2048 m; the frame is the smallest power-of-two square that holds the world.
        let world_m = f64::from(grid.world_size_km) * 1000.0 + 4096.0;
        let root = (2048.0_f64 * 2.0).max(2.0_f64.powf(world_m.log2().ceil()));
        let lattice_level = (root / 2048.0).log2().round() as u8;
        let frame = Frame::new([-2048.0, -2048.0], root);
        let config = RoadsConfig {
            lattice_level,
            ..RoadsConfig::default()
        };
        let engine = worldgen_roads::engine(Seed::new(seed), frame, config, fields)?;
        Ok(Self {
            engine,
            rivers,
            urban,
            centres: settlements
                .iter()
                .map(|s| {
                    v2(
                        s.location.x as f64 * f64::from(grid.cell_metres()),
                        s.location.y as f64 * f64::from(grid.cell_metres()),
                    )
                })
                .collect(),
            radii_m: settlements
                .iter()
                .map(|s| f64::from(settlement_radius_km(s.class)) * 1000.0)
                .collect(),
            frame,
            lattice_cell_m: 2048.0,
        })
    }

    pub fn urban(&self) -> Arc<SettlementUrban> {
        self.urban.clone()
    }

    /// The network inside a square window, assembled from tiles.
    pub fn network(&self, centre: V2, half_m: f64) -> Result<RoadNetwork, String> {
        // A tile level whose cells are about the size of the window.
        let level = ((self.frame.root_size_m / (2.0 * half_m)).log2().floor() as u8).max(1);
        let (lo, hi) = (
            Cell::containing(&self.frame, [centre.x - half_m, centre.y - half_m], level),
            Cell::containing(&self.frame, [centre.x + half_m, centre.y + half_m], level),
        );
        let mut tiles: Vec<Arc<RoadTile>> = Vec::new();
        for y in lo.y..=hi.y {
            for x in lo.x..=hi.x {
                tiles.push(
                    self.engine
                        .get::<RoadTile>(ROADS, Cell::new(level, x, y))
                        .map_err(|e| e.to_string())?,
                );
            }
        }
        RoadNetwork::assemble(tiles.iter().map(|t| &**t))
            .map_err(|c| format!("tiles do not merge: {c:?}"))
    }

    pub fn centre_m(&self, settlement: usize) -> V2 {
        self.centres[settlement]
    }

    pub fn radius_m(&self, settlement: usize) -> f64 {
        self.radii_m[settlement]
    }

    pub fn lattice_cell_m(&self) -> f64 {
        self.lattice_cell_m
    }
}

/// How a road's edges map onto the city planner's classes.
pub fn modern_road_class(class: RoadClass) -> urban::ModernRoadClass {
    match class {
        RoadClass::Motorway => urban::ModernRoadClass::Expressway,
        RoadClass::Arterial => urban::ModernRoadClass::Arterial,
        RoadClass::Collector => urban::ModernRoadClass::Collector,
        RoadClass::Local | RoadClass::Service | RoadClass::Track => urban::ModernRoadClass::Local,
    }
}

/// Where a polyline is inside a circle, cut exactly at the circle. Each run says
/// whether it was cut at its start and at its end.
fn clip_circle(line: &Polyline, centre: V2, radius: f64) -> Vec<(Polyline, bool, bool)> {
    let mut runs: Vec<(Vec<V2>, bool, bool)> = Vec::new();
    let mut open: Option<(Vec<V2>, bool)> = None;
    for w in line.0.windows(2) {
        let (a, b) = (w[0], w[1]);
        let (da, db) = (a.dist(centre) <= radius, b.dist(centre) <= radius);
        // Where the segment crosses the circle, as fractions of its length.
        let d = b - a;
        let f = a - centre;
        let (qa, qb, qc) = (d.dot(d), 2.0 * f.dot(d), f.dot(f) - radius * radius);
        let disc = qb * qb - 4.0 * qa * qc;
        let (t0, t1) = if disc > 0.0 && qa > 1e-12 {
            let s = disc.sqrt();
            ((-qb - s) / (2.0 * qa), (-qb + s) / (2.0 * qa))
        } else {
            (f64::NAN, f64::NAN)
        };
        match (da, db) {
            (true, true) => {
                let run = open.get_or_insert_with(|| (vec![a], false));
                run.0.push(b);
            }
            (true, false) => {
                let run = open.get_or_insert_with(|| (vec![a], false));
                run.0.push(a + d * t1.clamp(0.0, 1.0));
                let (points, cut_start) = open.take().unwrap();
                runs.push((points, cut_start, true));
            }
            (false, true) => {
                if let Some((points, cut_start)) = open.take() {
                    runs.push((points, cut_start, false));
                }
                open = Some((vec![a + d * t0.clamp(0.0, 1.0), b], true));
            }
            (false, false) => {
                if t0.is_finite() && (0.0..=1.0).contains(&t0) && (0.0..=1.0).contains(&t1) {
                    runs.push((vec![a + d * t0, a + d * t1], true, true));
                }
            }
        }
    }
    if let Some((points, cut_start)) = open {
        runs.push((points, cut_start, false));
    }
    runs.into_iter()
        .filter(|(p, _, _)| p.len() >= 2)
        .map(|(p, s, e)| (Polyline(p), s, e))
        .collect()
}

/// Everything the city planner needs from the road layer for one town.
pub struct TownStreets {
    pub streets: urban::ExternalStreets,
    pub fields: urban::ExternalFields,
    /// Mean width of the river through the town, metres; zero if none.
    pub river_width_m: f32,
}

impl WorldFabric {
    /// The streets of a town: the network inside a circle around it, cut at the
    /// circle, without the country roads that have nothing to do with the town.
    pub fn town_streets(&self, settlement: usize) -> Result<TownStreets, String> {
        let (centre, radius) = (
            self.centre_m(settlement),
            self.radius_m(settlement) * 1.35 + f64::from(crate::WINDOW_TAIL_KM) * 1000.0,
        );
        let net = self.network(centre, radius + 600.0)?;
        let urban_field = self.urban.clone();
        // The waterways through the town: the longest is the river, the rest are
        // drawn as tributaries; a creek too narrow to see is stepped over, not bridged.
        let mut waterways: Vec<(Polyline, f64)> = self
            .rivers
            .through(centre, radius)
            .into_iter()
            .filter(|(_, width)| *width >= 9.0)
            .collect();
        waterways.sort_by(|a, b| b.0.length().total_cmp(&a.0.length()));
        let over_drawn_water = |edge: &worldgen_contracts::RoadEdge| {
            edge.spans.iter().any(|span| {
                let mid = span.from.lerp(span.to, 0.5);
                waterways.iter().any(|(line, width)| {
                    line.closest(mid)
                        .is_some_and(|(d, _)| d < width * 0.5 + 14.0)
                })
            })
        };
        let mut node_points: std::collections::HashMap<u64, urban::Point> =
            std::collections::HashMap::new();
        let km = |p: V2| urban::Point {
            x_km: (p.x / 1000.0) as f32,
            y_km: (p.y / 1000.0) as f32,
        };
        let mut roads = Vec::new();
        for edge in net.edges.values() {
            // A road whose pieces were cut by a tile border is whole again here.
            for piece in &edge.pieces {
                for (run, cut_start, cut_end) in clip_circle(piece, centre, radius) {
                    let mid = run.at(run.length() * 0.5).map_or(centre, |m| m.0);
                    let u = urban_field.urbanness(mid);
                    let keep = match edge.source {
                        EdgeSource::Given => true,
                        // Out in the farms only the lanes and tracks remain, but they do.
                        EdgeSource::Generated => u >= 0.03,
                    };
                    if !keep {
                        continue;
                    }
                    let (first, last) = (run.first().unwrap(), run.last().unwrap());
                    let end_id = |node: NodeId, cut: bool, at: V2, tag: u64| {
                        if cut
                            || net
                                .nodes
                                .get(&node)
                                .is_none_or(|n| n.position.dist(at) > 1e-6)
                        {
                            hash_words(&[edge.id.0, tag, at.x.to_bits(), at.y.to_bits()])
                        } else {
                            node.0
                        }
                    };
                    let (from, to) = (
                        end_id(edge.a, cut_start, first, 1),
                        end_id(edge.b, cut_end, last, 2),
                    );
                    if from == to {
                        continue;
                    }
                    node_points.entry(from).or_insert_with(|| km(first));
                    node_points.entry(to).or_insert_with(|| km(last));
                    roads.push(urban::ExternalRoad {
                        from,
                        to,
                        class: modern_road_class(edge.class),
                        bridge: over_drawn_water(edge),
                        centreline: run.0.iter().map(|p| km(*p)).collect(),
                    });
                }
            }
        }
        let nodes = node_points
            .into_iter()
            .map(|(id, point)| urban::ExternalNode { id, point })
            .collect();
        let to_km =
            |line: &Polyline| -> Vec<urban::Point> { line.0.iter().map(|p| km(*p)).collect() };
        let mut waterways = waterways.into_iter();
        let (river, river_width_m) = waterways.next().map_or((Vec::new(), 0.0), |(line, width)| {
            (to_km(&line), width as f32)
        });
        let tributaries: Vec<(Vec<urban::Point>, f32)> = waterways
            .map(|(line, width)| (to_km(&line), width as f32))
            .collect();
        let (u2, i2) = (self.urban.clone(), self.urban.clone());
        let fields = urban::ExternalFields {
            urbanness: Box::new(move |p| {
                u2.urbanness(v2(f64::from(p.x_km) * 1000.0, f64::from(p.y_km) * 1000.0)) as f32
            }),
            intensity: Box::new(move |p| {
                i2.intensity(v2(f64::from(p.x_km) * 1000.0, f64::from(p.y_km) * 1000.0)) as f32
            }),
        };
        Ok(TownStreets {
            streets: urban::ExternalStreets {
                nodes,
                roads,
                river,
                tributaries,
            },
            fields,
            river_width_m,
        })
    }
}
