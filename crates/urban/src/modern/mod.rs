mod blocks;
mod geom;
mod graph;
mod landscape;
mod parcels;
mod roads;

pub use graph::junction_trim_m;
use graph::{GraphOutput, build_graph, legacy_hash, modern_hash, modern_phase};
use landscape::derive_compounds_and_trees;
use parcels::{ParcelOutput, build_parcels};

use crate::{CityStyle, ModernChinaSpec, ModernCity, Point};

/// Deterministic `0..1` hash for any pair of integer coordinates.
///
/// Exposed because downstream scene derivation needs *stable per-object*
/// variation — a facade tint, a rooftop plant placement, a U-flip — that does
/// not consume from a shared random stream. Consuming a stream would make one
/// building's appearance depend on how many draws happened before it, which
/// breaks incremental regeneration.
pub fn hash_u32(seed: u32, a: i32, b: i32) -> f32 {
    legacy_hash(seed, a, b, 0x5f37_1d1b)
}

/// What a city plan reads from the world when its streets come from outside: how
/// built up and how intense the place is at a local point, and the river, if any.
pub(super) struct ExternalFrame {
    urbanness: Box<dyn Fn(f32, f32) -> f32 + Send + Sync>,
    intensity: Box<dyn Fn(f32, f32) -> f32 + Send + Sync>,
    /// The river in local metres, source to mouth.
    river: Vec<geom::V>,
}

struct CityFrame {
    external: Option<ExternalFrame>,
    spec: ModernChinaSpec,
    radius_m: f32,
    block_m: f32,
    density: f32,
    organic: f32,
    river_half: f32,
    /// Peak of the height/density field and a weaker secondary sub-centre, in
    /// local metres.  Seeded, so the tallest district is not always dead centre.
    core: geom::V,
    sub_core: geom::V,
    /// Grow an irregular built-up area (see `urbanness`) instead of a disc.
    organic_footprint: bool,
    /// Directions (local angle, strength 0..1) along which the town runs out
    /// beside its regional roads.
    lobes: Vec<(f32, f32)>,
}

impl CityFrame {
    fn new(spec: ModernChinaSpec) -> Self {
        let radius_m = spec.radius_km.max(0.2) * 1_000.0;
        let place = |salt: i32, lo: f32, hi: f32| -> geom::V {
            let a = modern_hash(spec.seed, salt, 1, 811) * std::f32::consts::TAU;
            let r = radius_m * (lo + (hi - lo) * modern_hash(spec.seed, salt, 2, 813));
            (r * a.cos(), r * a.sin())
        };
        let core = place(1, 0.04, 0.32);
        let sub_core = place(2, 0.35, 0.65);
        Self {
            external: None,
            organic_footprint: false,
            lobes: Vec::new(),
            core,
            sub_core,
            radius_m: spec.radius_km.max(0.2) * 1_000.0,
            block_m: spec.block_size_metres.clamp(70.0, 180.0),
            density: spec.density.clamp(0.0, 1.0),
            organic: spec.organic.clamp(0.0, 1.0),
            river_half: spec.river_width_metres.clamp(24.0, 140.0) * 0.5,
            spec,
        }
    }

    fn to_world(&self, x_m: f32, z_m: f32) -> Point {
        let c = self.spec.rotation_radians.cos();
        let s = self.spec.rotation_radians.sin();
        Point {
            x_km: self.spec.centre.x_km + (x_m * c - z_m * s) / 1_000.0,
            y_km: self.spec.centre.y_km + (x_m * s + z_m * c) / 1_000.0,
        }
    }

    /// Inverse of `to_world`: world km back to local city metres.
    fn to_local(&self, p: Point) -> (f32, f32) {
        let dx = (p.x_km - self.spec.centre.x_km) * 1_000.0;
        let dy = (p.y_km - self.spec.centre.y_km) * 1_000.0;
        let c = self.spec.rotation_radians.cos();
        let s = self.spec.rotation_radians.sin();
        (dx * c + dy * s, -dx * s + dy * c)
    }

    /// Urban intensity 0..1 at a local point: a main peak, a weaker sub-centre
    /// and low-frequency local variation.  Drives block size, height and density.
    fn core_weight(&self, x: f32, z: f32) -> f32 {
        if let Some(external) = &self.external {
            return (external.intensity)(x, z).clamp(0.0, 1.0);
        }
        let r = self.radius_m;
        let g = |c: geom::V, s: f32| {
            let d = ((x - c.0).powi(2) + (z - c.1).powi(2)).sqrt() / (r * s);
            (-d * d).exp()
        };
        let base = g(self.core, 0.5).max(0.62 * g(self.sub_core, 0.28));
        let ph = modern_phase(self.spec.seed);
        let wob = 0.10 * ((x / 95.0 + ph).sin() * (z / 120.0 + ph * 1.7).cos())
            + 0.06 * ((x + z) / 55.0 + ph * 0.6).sin();
        (base + wob * (0.4 + base)).clamp(0.0, 1.0)
    }

    /// How built-up the ground is at a local point, 1 in the town proper fading to
    /// 0 past its edge. A real town is not a disc: its outline is lumpy, it runs
    /// out along the roads that feed it, and it thins into scattered houses and
    /// then fields rather than stopping at a ring. Without the organic footprint
    /// this is the old disc.
    fn urbanness(&self, x: f32, z: f32) -> f32 {
        if let Some(external) = &self.external {
            return (external.urbanness)(x, z).clamp(0.0, 1.0);
        }
        let r = x.hypot(z);
        if !self.organic_footprint {
            return if r <= self.radius_m * 1.04 { 1.0 } else { 0.0 };
        }
        use std::f32::consts::{PI, TAU};
        let th = z.atan2(x);
        let ph = modern_phase(self.spec.seed);
        let lump = 0.66
            + 0.12 * (2.0 * th + ph).sin()
            + 0.07 * (3.0 * th + ph * 1.9).sin()
            + 0.04 * (5.0 * th - ph * 0.7).sin();
        let mut reach = self.radius_m * lump;
        for &(angle, strength) in &self.lobes {
            let mut d = (th - angle).abs() % TAU;
            if d > PI {
                d = TAU - d;
            }
            reach += self.radius_m * 0.42 * strength * (-(d / 0.38).powi(2)).exp();
        }
        // Ragged edge: the outline wanders by a block or two.
        let jag = 1.0
            + 0.16 * ((x / 210.0 + ph).sin() * (z / 170.0 - ph).cos())
            + 0.08 * ((x + z) / 90.0).sin();
        let reach = reach * jag;
        ((reach * 1.12 - r) / (reach * 0.42)).clamp(0.0, 1.0)
    }

    /// The river in local metres, from the world's own water when the streets come
    /// from outside, else the plan's own sine.
    fn river_line(&self) -> Vec<geom::V> {
        if let Some(external) = &self.external {
            return external.river.clone();
        }
        (0..=48)
            .map(|i| {
                let z = -self.radius_m * 1.2 + 2.4 * self.radius_m * i as f32 / 48.0;
                (self.river_x(z), z)
            })
            .collect()
    }

    /// Unit normal and offset of the river's bank line through the point of the river
    /// nearest `c`: the line that splits a block into its two banks.
    fn river_split(&self, c: geom::V) -> (geom::V, f32) {
        if let Some(external) = &self.external {
            let mut best = (f32::MAX, (1.0, 0.0), (0.0, 0.0));
            for w in external.river.windows(2) {
                let (a, b) = (w[0], w[1]);
                let d = (b.0 - a.0, b.1 - a.1);
                let l2 = (d.0 * d.0 + d.1 * d.1).max(1.0e-6);
                let t = (((c.0 - a.0) * d.0 + (c.1 - a.1) * d.1) / l2).clamp(0.0, 1.0);
                let q = (a.0 + d.0 * t, a.1 + d.1 * t);
                let dist = (c.0 - q.0).hypot(c.1 - q.1);
                if dist < best.0 {
                    let l = l2.sqrt();
                    best = (dist, (d.1 / l, -d.0 / l), q);
                }
            }
            let (_, n, q) = best;
            return (n, n.0 * q.0 + n.1 * q.1);
        }
        let rx = self.river_x(c.1);
        let slope = (self.river_x(c.1 + 5.0) - self.river_x(c.1 - 5.0)) / 10.0;
        let norm = (1.0 + slope * slope).sqrt();
        let n = (1.0 / norm, -slope / norm);
        (n, n.0 * rx + n.1 * c.1)
    }

    fn river_x(&self, z_m: f32) -> f32 {
        let radius_m = self.radius_m;
        let phase = modern_phase(self.spec.seed);
        radius_m
            * (0.075 * (z_m / radius_m * 2.7 + phase).sin()
                + 0.032 * (z_m / radius_m * 6.2 + phase * 0.4).sin())
    }
}

/// Fraction of a city's radius inside which its own street plan owns every
/// regional road, and outside which the regional renderer does.  The city plans
/// each road from the ring in to a little *deeper* than this, so the two overlap
/// on the same line and a road that grazes the edge of town still has a drawer.
pub const REGIONAL_ROAD_HANDOVER: f32 = 0.9;

/// A regional road as the world planner routed it: a polyline in world
/// kilometres (the same frame as `ModernChinaSpec::centre`).  The city clips it
/// to its outer ring and plans the part inside as one of its own streets, so a
/// road that arrives at the edge of town is the same road inside it.
#[derive(Debug, Clone)]
pub struct RegionalApproach {
    pub class: crate::ModernRoadClass,
    pub path_km: Vec<Point>,
}

pub fn generate_modern_chinese_city(spec: ModernChinaSpec) -> ModernCity {
    generate_modern_chinese_city_with_approaches(spec, &[])
}

/// Optional behaviour of the city generator. The default reproduces the plain
/// disc the generator always made.
#[derive(Debug, Clone, Copy, Default)]
pub struct CityOptions {
    /// Grow an irregular built-up area that runs out along the regional roads.
    pub organic_footprint: bool,
}

pub fn generate_modern_chinese_city_with_approaches(
    spec: ModernChinaSpec,
    approaches: &[RegionalApproach],
) -> ModernCity {
    generate_modern_chinese_city_with_options(spec, approaches, CityOptions::default())
}

/// Directions in which the town should run out: one lobe per regional road that
/// reaches it, stronger for bigger roads.
fn approach_lobes(frame: &CityFrame, approaches: &[RegionalApproach]) -> Vec<(f32, f32)> {
    let mut lobes: Vec<(f32, f32)> = Vec::new();
    for approach in approaches {
        let local: Vec<(f32, f32)> = approach.path_km.iter().map(|p| frame.to_local(*p)).collect();
        // The point of the road nearest 0.9 of the radius gives its bearing.
        let target = frame.radius_m * 0.9;
        let Some(best) = local.iter().min_by(|a, b| {
            (a.0.hypot(a.1) - target).abs().total_cmp(&(b.0.hypot(b.1) - target).abs())
        }) else {
            continue;
        };
        if (best.0.hypot(best.1) - target).abs() > frame.radius_m * 0.5 {
            continue;
        }
        let strength = match approach.class {
            crate::ModernRoadClass::Expressway => 1.0,
            crate::ModernRoadClass::Arterial => 0.8,
            crate::ModernRoadClass::Collector => 0.5,
            crate::ModernRoadClass::Local => 0.25,
        };
        lobes.push((best.1.atan2(best.0), strength));
    }
    lobes
}

pub fn generate_modern_chinese_city_with_options(
    spec: ModernChinaSpec,
    approaches: &[RegionalApproach],
    options: CityOptions,
) -> ModernCity {
    let mut frame = CityFrame::new(spec);
    if options.organic_footprint {
        frame.organic_footprint = true;
        frame.lobes = approach_lobes(&frame, approaches);
    }
    let frame = frame;
    let GraphOutput {
        nodes,
        sd_roads,
        hd_roads,
        river,
        morphology_score,
    } = build_graph(&frame, approaches);
    let ParcelOutput {
        blocks,
        parcels,
        buildings,
    } = build_parcels(&frame, &nodes, &sd_roads);
    let (compounds, trees) = derive_compounds_and_trees(&parcels, spec.seed);
    ModernCity {
        version: 3,
        jurisdiction: "ChinaMainland".into(),
        style: CityStyle::ChineseModern,
        seed: spec.seed,
        frame: crate::model::scene::CityFrameInfo {
            origin: spec.centre,
            rotation_radians: spec.rotation_radians,
        },
        nodes,
        sd_roads,
        hd_roads,
        blocks,
        parcels,
        buildings,
        compounds,
        trees,
        river: Some(river),
        river_width_metres: frame.river_half * 2.0,
        morphology_score,
    }
}

/// A street network supplied from outside: nodes and roads in world kilometres.
/// The roads carry their own centrelines (already bent) and whether they are a
/// bridge; the plan makes nothing of its own and builds blocks, lots and
/// buildings on what it is given.
pub struct ExternalStreets {
    pub nodes: Vec<ExternalNode>,
    pub roads: Vec<ExternalRoad>,
    /// The river through the town in world kilometres, source to mouth; may be empty.
    pub river: Vec<Point>,
}

pub struct ExternalNode {
    /// Any stable id; roads refer to nodes by it.
    pub id: u64,
    pub point: Point,
}

pub struct ExternalRoad {
    pub from: u64,
    pub to: u64,
    pub class: crate::ModernRoadClass,
    pub bridge: bool,
    /// From the `from` node to the `to` node, ends included.
    pub centreline: Vec<Point>,
}

/// What the plan needs to know about the place, as functions of world position.
pub struct ExternalFields {
    /// 0 in open country to 1 in a built-up town.
    pub urbanness: Box<dyn Fn(Point) -> f32 + Send + Sync>,
    /// 0 at the edge to 1 at the heart of the town: height and commercial share follow it.
    pub intensity: Box<dyn Fn(Point) -> f32 + Send + Sync>,
}

/// Build blocks, lots and buildings on a street network that was generated
/// elsewhere. The city's frame (`centre`, `rotation_radians`, `radius_km`) says
/// where local metres are; `radius_km` bounds the area that gets buildings.
pub fn generate_modern_chinese_city_from_streets(
    spec: ModernChinaSpec,
    streets: ExternalStreets,
    fields: ExternalFields,
) -> ModernCity {
    let mut frame = CityFrame::new(spec);
    frame.organic_footprint = true;
    let urban = std::sync::Arc::new(fields.urbanness);
    let intensity = std::sync::Arc::new(fields.intensity);
    let (centre, rotation) = (spec.centre, spec.rotation_radians);
    let world_of = move |x: f32, z: f32| {
        let (c, s) = (rotation.cos(), rotation.sin());
        Point { x_km: centre.x_km + (x * c - z * s) / 1_000.0, y_km: centre.y_km + (x * s + z * c) / 1_000.0 }
    };
    let (u2, i2) = (urban.clone(), intensity.clone());
    let river: Vec<geom::V> = streets.river.iter().map(|p| frame.to_local(*p)).collect();
    frame.external = Some(ExternalFrame {
        urbanness: Box::new(move |x, z| u2(world_of(x, z))),
        intensity: Box::new(move |x, z| i2(world_of(x, z))),
        river,
    });
    let frame = frame;

    // Renumber nodes 0..n and keep only those a road uses.
    let mut index: std::collections::HashMap<u64, u32> = std::collections::HashMap::new();
    let mut nodes: Vec<crate::SdNode> = Vec::new();
    let mut position: std::collections::HashMap<u64, Point> = streets.nodes.iter().map(|n| (n.id, n.point)).collect();
    let mut sd_roads = Vec::new();
    let mut hd_roads = Vec::new();
    let rules = roads::china_rules();
    let block_m = frame.block_m;
    for road in &streets.roads {
        let mut ends = [0_u32; 2];
        for (k, id) in [road.from, road.to].into_iter().enumerate() {
            let Some(point) = position.get(&id).copied() else { continue };
            ends[k] = *index.entry(id).or_insert_with(|| {
                let (x, z) = frame.to_local(point);
                let role = if x.abs() < block_m * 1.2 && z.abs() < block_m * 1.2 { "core-junction" } else { "district-junction" };
                nodes.push(crate::SdNode { id: nodes.len() as u32, point, role: role.into() });
                nodes.len() as u32 - 1
            });
        }
        if !position.contains_key(&road.from) || !position.contains_key(&road.to) || ends[0] == ends[1] {
            continue;
        }
        let id = sd_roads.len() as u32;
        sd_roads.push(crate::SdRoad { id, from: ends[0], to: ends[1], class: road.class, bridge: road.bridge });
        let mut centreline = road.centreline.clone();
        if let (Some(first), Some(last)) = (centreline.first_mut(), position.get(&road.from)) {
            *first = *last;
        }
        if let (Some(last), Some(end)) = (centreline.last_mut(), position.get(&road.to)) {
            *last = *end;
        }
        hd_roads.push(crate::HdRoad {
            id,
            sd_road: id,
            class: road.class,
            width_metres: road.class.width_metres(),
            median_metres: road.class.median_metres(),
            centreline,
            bridge: road.bridge,
            layer: i8::from(road.bridge),
            lanes: roads::lanes_for_modern_road(id, road.class, &rules),
            connectors: Vec::new(),
        });
    }
    position.clear();
    let morphology_score = crate::MorphologyPrior::default().score(&graph::GraphProbe {
        nodes: &nodes,
        sd_roads: &sd_roads,
        hd_roads: &hd_roads,
    });
    let ParcelOutput { blocks, parcels, buildings } = build_parcels(&frame, &nodes, &sd_roads);
    let (compounds, trees) = derive_compounds_and_trees(&parcels, spec.seed);
    ModernCity {
        version: 3,
        jurisdiction: "ChinaMainland".into(),
        style: CityStyle::ChineseModern,
        seed: spec.seed,
        frame: crate::model::scene::CityFrameInfo { origin: spec.centre, rotation_radians: spec.rotation_radians },
        nodes,
        sd_roads,
        hd_roads,
        blocks,
        parcels,
        buildings,
        compounds,
        trees,
        river: (!streets.river.is_empty()).then_some(streets.river),
        river_width_metres: frame.river_half * 2.0,
        morphology_score,
    }
}
