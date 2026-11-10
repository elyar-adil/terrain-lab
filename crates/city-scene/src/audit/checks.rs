//! The checks themselves. See [`super::CHECKS`] for the catalogue.

use std::collections::{HashMap, HashSet};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use urban::{JunctionKind, ModernCity, ModernRoadClass, ParcelUse};

use super::geom::{
    Grid, JunctionIndex, RoadIndex, at_grade, bounds, centroid, distance_to_ring, lane_band,
    overlap_area, ring_gap, ring_samples,
};
use super::{CHECKS, Inventory, KEEP_WORST, Report, Summary, Violation};
use crate::buildings::{level, water_level};
use crate::math::{Vec2, distance_to_polyline, point_in_ring, signed_area};
use crate::network::{Network, segment_intersection};
use crate::scene::CityScene;

/// Ground heights for the slope checks, supplied by whoever holds the terrain.
pub struct Terrain {
    /// Ground height in metres at a city-local point.
    pub height_m: Box<dyn Fn(f32, f32) -> f32 + Sync>,
}

impl Terrain {
    /// Slope (rise over run) at a local point, from heights `reach` metres apart.
    pub fn slope(&self, x: f32, z: f32, reach: f32) -> f32 {
        let h = &self.height_m;
        let dx = (h(x + reach, z) - h(x - reach, z)) / (2.0 * reach);
        let dz = (h(x, z + reach) - h(x, z - reach)) / (2.0 * reach);
        dx.hypot(dz)
    }
}

struct Bld {
    id: u32,
    ring: Vec<Vec2>,
    lo: Vec2,
    hi: Vec2,
    centre: Vec2,
    area: f32,
    height: f32,
    floors: u16,
    use_type: ParcelUse,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Tree,
    /// A post, sign or box that stands on the ground: height in metres.
    Post(f32),
}

struct Inst {
    kind: Kind,
    key: String,
    p: Vec2,
    y: f32,
    scale: f32,
}

struct Channel {
    line: Vec<Vec2>,
    half: f32,
}

pub struct Context<'a> {
    city: &'a ModernCity,
    network: &'a Network,
    scene: Option<&'a CityScene>,
    terrain: Option<&'a Terrain>,
    max_buildings: usize,
    blds: Vec<Bld>,
    bgrid: Grid,
    channels: Vec<Channel>,
    water_y: f32,
    roads: RoadIndex<'a>,
    junctions: JunctionIndex<'a>,
    insts: Vec<Inst>,
    podiums: Vec<Option<(f32, u16)>>,
    counts: Vec<usize>,
    found: Vec<Vec<Violation>>,
    inventory: Inventory,
}

const STORE_CAP: usize = 4000;

fn prefix(use_type: ParcelUse) -> &'static str {
    match use_type {
        ParcelUse::Residential => "residential",
        ParcelUse::MixedUse => "mixed-use",
        ParcelUse::Commercial => "commercial",
        ParcelUse::Civic => "civic",
        ParcelUse::Park => "park",
        ParcelUse::Villa => "villa",
        ParcelUse::Farmstead => "farmstead",
        ParcelUse::Village => "village",
        ParcelUse::Industrial => "industrial",
    }
}

impl<'a> Context<'a> {
    pub fn new(
        city: &'a ModernCity,
        network: &'a Network,
        scene: Option<&'a CityScene>,
        terrain: Option<&'a Terrain>,
        max_buildings: usize,
    ) -> Self {
        let local = |p: &urban::Point| {
            let [x, z] = city.frame.to_local(*p);
            Vec2::new(x, z)
        };
        let mut blds = Vec::new();
        let mut bgrid = Grid::new(40.0);
        for b in &city.buildings {
            let ring: Vec<Vec2> = b.footprint.iter().map(local).collect();
            if ring.is_empty() {
                continue;
            }
            let (lo, hi) = bounds(&ring);
            bgrid.insert(blds.len() as u32, lo, hi);
            blds.push(Bld {
                id: b.id,
                centre: centroid(&ring),
                area: signed_area(&ring).abs(),
                ring,
                lo,
                hi,
                height: b.height_metres,
                floors: b.floors,
                use_type: b.use_type,
            });
        }
        let mut channels = Vec::new();
        if let Some(river) = &city.river {
            channels.push(Channel {
                line: river.iter().map(local).collect(),
                half: city.river_width_metres * 0.5,
            });
        }
        for t in &city.tributaries {
            channels.push(Channel {
                line: t.line.iter().map(local).collect(),
                half: t.width_metres * 0.5,
            });
        }
        let width = if city.river.is_some() {
            city.river_width_metres
        } else {
            city.tributaries.first().map_or(40.0, |t| t.width_metres)
        };
        let water_y = water_level(&city.river_setting, width);
        let podiums = crate::buildings::far::podiums_of(&city.buildings);
        let mut cx = Self {
            city,
            network,
            scene,
            terrain,
            max_buildings,
            blds,
            bgrid,
            channels,
            water_y,
            roads: RoadIndex::new(network),
            junctions: JunctionIndex::new(&network.junctions),
            insts: Vec::new(),
            podiums,
            counts: vec![0; CHECKS.len()],
            found: vec![Vec::new(); CHECKS.len()],
            inventory: Inventory::default(),
        };
        cx.decode_instances();
        cx.inventory = Inventory {
            buildings: cx.blds.len(),
            roads: network.roads.len(),
            junctions: network.junctions.len(),
            trees: cx.insts.iter().filter(|i| i.kind == Kind::Tree).count(),
            furniture: cx.insts.iter().filter(|i| i.kind != Kind::Tree).count(),
            vehicles: scene.map_or(0, |s| s.traffic.agents.len()),
            fields: city.fields.len(),
            compounds: city.compounds.len(),
            bridges: network.roads.iter().filter(|r| r.bridge || r.layer > 0).count(),
            ramps: network.roads.iter().filter(|r| r.oneway).count(),
            signalised: network.junctions.iter().filter(|j| j.kind == JunctionKind::Signalized).count(),
            rivers: cx.channels.len(),
            scene_checked: scene.is_some(),
            terrain_checked: terrain.is_some(),
        };
        cx
    }

    fn decode_instances(&mut self) {
        let Some(scene) = self.scene else { return };
        for list in &scene.instances {
            let kind = if list.key.starts_with("tree/") {
                Kind::Tree
            } else if let Some(name) = list.key.strip_prefix("furniture/") {
                let height = match name {
                    "lamp" | "lamp.double" => 8.0,
                    "pole" => 6.0,
                    "sign.guide" | "sign.crossing" | "sign.roundabout" | "sign.glyph" => 3.0,
                    "shelter" => 3.0,
                    "bin" | "bollard" => 1.0,
                    _ => continue,
                };
                Kind::Post(height)
            } else {
                continue;
            };
            let Ok(bytes) = STANDARD.decode(&list.data) else { continue };
            for chunk in bytes.chunks_exact(40) {
                let f: Vec<f32> = chunk
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                    .collect();
                self.insts.push(Inst {
                    kind,
                    key: list.key.clone(),
                    p: Vec2::new(f[0], f[2]),
                    y: f[1],
                    scale: f[5],
                });
            }
        }
    }

    // ---- reporting -------------------------------------------------------

    fn push(&mut self, id: &'static str, at: Vec2, ids: Vec<String>, magnitude: f32, detail: String) {
        let index = CHECKS.iter().position(|c| c.id == id).expect("known check");
        self.counts[index] += 1;
        if self.found[index].len() < STORE_CAP {
            self.found[index].push(Violation {
                check: id,
                x: at.x,
                z: at.y,
                ids,
                magnitude,
                detail,
            });
        }
    }

    pub fn finish(mut self) -> Report {
        let mut summaries = Vec::new();
        for (i, c) in CHECKS.iter().enumerate() {
            let mut worst = std::mem::take(&mut self.found[i]);
            worst.sort_by(|a, b| b.magnitude.total_cmp(&a.magnitude));
            worst.truncate(KEEP_WORST);
            summaries.push(Summary {
                id: c.id,
                group: c.group,
                title: c.title,
                owner: c.owner,
                severity: c.severity,
                visibility: c.visibility,
                count: self.counts[i],
                gate: c.gate,
                score: self.counts[i] * c.severity as usize * c.visibility as usize,
                worst,
            });
        }
        Report {
            inventory: self.inventory,
            summaries,
        }
    }

    pub fn run_all(&mut self) {
        self.water();
        self.levels();
        self.building_checks();
        self.instance_checks();
        self.vehicle_checks();
        self.fence_checks();
        self.road_crossings();
        self.junction_checks();
        self.road_checks();
        self.graph_checks();
        self.lod_checks();
        self.field_checks();
    }

    // ---- helpers ---------------------------------------------------------

    fn local(&self, p: &urban::Point) -> Vec2 {
        let [x, z] = self.city.frame.to_local(*p);
        Vec2::new(x, z)
    }

    /// Metres inside the nearest water channel (negative outside it).
    fn channel_depth(&self, p: Vec2) -> f32 {
        let mut best = f32::MIN;
        for c in &self.channels {
            if c.line.len() >= 2 {
                best = best.max(c.half - distance_to_polyline(p, &c.line));
            }
        }
        best
    }

    fn channel_distance(&self, p: Vec2) -> f32 {
        self.channels
            .iter()
            .filter(|c| c.line.len() >= 2)
            .map(|c| distance_to_polyline(p, &c.line))
            .fold(f32::MAX, f32::min)
    }

    /// The junction whose paved box holds `p`; the island at the middle of a
    /// roundabout is not pavement.
    fn junction_pavement(&self, p: Vec2) -> Option<&'a crate::network::Junction> {
        self.junctions.containing(p).filter(|j| !(j.roundabout && p.distance(j.centre) < j.island + 0.5))
    }

    fn building_at(&self, p: Vec2) -> Option<&Bld> {
        self.bgrid
            .query(p, p)
            .into_iter()
            .map(|i| &self.blds[i as usize])
            .find(|b| point_in_ring(p, &b.ring))
    }

    // ---- (a) water -------------------------------------------------------

    fn water(&mut self) {
        if self.channels.is_empty() {
            return;
        }
        for i in 0..self.blds.len() {
            let b = &self.blds[i];
            let mut depth = f32::MIN;
            for p in ring_samples(&b.ring, 3.0).into_iter().chain([b.centre]) {
                depth = depth.max(self.channel_depth(p));
            }
            if depth > 1.0 {
                let (c, id, h) = (b.centre, b.id, b.height);
                self.push(
                    "water.building",
                    c,
                    vec![format!("building:{id}")],
                    depth,
                    format!("{h:.0} m building {depth:.1} m into the channel"),
                );
            }
        }
        for ri in 0..self.network.roads.len() {
            let road = &self.network.roads[ri];
            if road.bridge || road.layer > 0 {
                continue;
            }
            let length = road.carriageway.length();
            let mut worst: Option<(f32, Vec2)> = None;
            let mut s = 0.0;
            while s <= length {
                let (p3, _) = road.carriageway.sample(s);
                let p = Vec2::new(p3.x, p3.z);
                if at_grade(p3.y) {
                    let d = self.channel_depth(p);
                    if d > 2.0 && worst.is_none_or(|w| d > w.0) {
                        worst = Some((d, p));
                    }
                }
                s += 4.0;
            }
            if let Some((d, p)) = worst {
                let id = road.id;
                self.push(
                    "water.road",
                    p,
                    vec![format!("road:{id}")],
                    d,
                    format!("at-grade road {d:.1} m into the channel, not flagged as a bridge"),
                );
            }
        }
        for ri in 0..self.network.roads.len() {
            let road = &self.network.roads[ri];
            if !road.bridge {
                continue;
            }
            let length = road.carriageway.length();
            let mut worst: Option<(f32, Vec2)> = None;
            let mut s = 0.0;
            while s <= length {
                let (p3, _) = road.carriageway.sample(s);
                let p = Vec2::new(p3.x, p3.z);
                if self.channel_depth(p) > 0.0 {
                    let clear = p3.y - road.deck_thickness - self.water_y;
                    if worst.is_none_or(|w| clear < w.0) {
                        worst = Some((clear, p));
                    }
                }
                s += 3.0;
            }
            if let Some((clear, p)) = worst
                && clear < 1.5
            {
                let id = road.id;
                self.push(
                    "bridge.clear_water",
                    p,
                    vec![format!("road:{id}")],
                    1.5 - clear,
                    format!("deck underside {clear:.2} m over the water"),
                );
            }
        }
        for fi in 0..self.city.fields.len() {
            let ring: Vec<Vec2> = self.city.fields[fi].ring.iter().map(|p| self.local(p)).collect();
            let mut depth = f32::MIN;
            let mut at = centroid(&ring);
            for p in ring_samples(&ring, 6.0) {
                let d = self.channel_depth(p);
                if d > depth {
                    depth = d;
                    at = p;
                }
            }
            if depth > 3.0 {
                let id = self.city.fields[fi].id;
                self.push(
                    "water.field",
                    at,
                    vec![format!("field:{id}")],
                    depth,
                    format!("field edge {depth:.1} m into the channel"),
                );
            }
        }
        for i in 0..self.insts.len() {
            let inst = &self.insts[i];
            if !at_grade(inst.y) {
                continue;
            }
            let d = self.channel_depth(inst.p);
            if d > 0.5 {
                let (p, key) = (inst.p, inst.key.clone());
                let id = if inst.kind == Kind::Tree { "water.tree" } else { "water.furniture" };
                self.push(id, p, vec![key], d, format!("{d:.1} m inside the channel"));
            }
        }
    }

    // ---- (a) surfaces ----------------------------------------------------

    /// An object must stand on the surface under it: the ground, or the deck of
    /// the road it is on.
    fn levels(&mut self) {
        for i in 0..self.insts.len() {
            let inst = &self.insts[i];
            let height = match inst.kind {
                Kind::Tree => inst.scale,
                Kind::Post(h) => h,
            };
            let mut deck: Option<(f32, f32, u32)> = None;
            for near in self.roads.near(inst.p, 40.0) {
                let road = &self.network.roads[near.road];
                if near.y > 1.0 && near.distance < road.half_width() + 0.3 {
                    deck = Some((near.y, road.deck_thickness, road.id));
                    break;
                }
            }
            let (p, key, y) = (inst.p, inst.key.clone(), inst.y);
            match deck {
                Some((deck_y, thick, road)) => {
                    let on_deck = (y - deck_y).abs() < 0.6;
                    let pierces = y < deck_y - 0.6 && y + height > deck_y - thick + 0.5;
                    if !on_deck && pierces {
                        self.push(
                            "ground.level",
                            p,
                            vec![key, format!("road:{road}")],
                            deck_y - y,
                            format!("base {y:.2} m, deck {deck_y:.2} m: stands through the deck"),
                        );
                    } else if !on_deck && y > deck_y + 0.6 {
                        self.push(
                            "ground.level",
                            p,
                            vec![key, format!("road:{road}")],
                            y - deck_y,
                            format!("base {y:.2} m over a deck at {deck_y:.2} m: floats"),
                        );
                    }
                }
                None => {
                    if (y - level::GROUND).abs() > 0.6 {
                        self.push(
                            "ground.level",
                            p,
                            vec![key],
                            (y - level::GROUND).abs(),
                            format!("base at {y:.2} m on level ground"),
                        );
                    }
                }
            }
        }
    }

    // ---- (b)(e)(h)(j) buildings -------------------------------------------

    fn building_checks(&mut self) {
        let n = self.blds.len();
        for i in 0..n {
            let (id, centre, area, height, floors, use_type) = {
                let b = &self.blds[i];
                (b.id, b.centre, b.area, b.height, b.floors, b.use_type)
            };
            let tag = format!("building:{id}");
            // degenerate
            if area < 12.0 || self.blds[i].ring.len() < 3 {
                self.push("bldg.degenerate", centre, vec![tag.clone()], 12.0 - area, format!("footprint {area:.1} m2"));
            }
            // storey height
            if floors > 0 {
                let ftf = height / floors as f32;
                let (lo, hi) = match use_type {
                    ParcelUse::Residential | ParcelUse::Villa | ParcelUse::Village | ParcelUse::Farmstead => (2.7, 4.5),
                    ParcelUse::MixedUse | ParcelUse::Commercial | ParcelUse::Civic => (3.0, 6.5),
                    ParcelUse::Industrial => (3.0, 12.0),
                    ParcelUse::Park => (2.4, 12.0),
                };
                if ftf < lo || ftf > hi {
                    let off = if ftf < lo { lo - ftf } else { ftf - hi };
                    self.push(
                        "bldg.storey",
                        centre,
                        vec![tag.clone()],
                        off,
                        format!("{} {height:.1} m / {floors} floors = {ftf:.2} m per floor (allowed {lo}-{hi})", prefix(use_type)),
                    );
                }
            }
            // height and storeys against use, and tall buildings away from the core
            let (max_floors, max_h) = match use_type {
                ParcelUse::Villa | ParcelUse::Farmstead => (4, 16.0),
                // The urban village (城中村) of handshake buildings: up to 15 storeys.
                ParcelUse::Village => (15, 52.0),
                ParcelUse::Industrial => (5, 30.0),
                ParcelUse::Civic => (12, 60.0),
                ParcelUse::Residential => (45, 140.0),
                ParcelUse::MixedUse => (50, 180.0),
                ParcelUse::Commercial => (80, 400.0),
                ParcelUse::Park => (2, 10.0),
            };
            let from_centre = centre.length();
            if floors > max_floors || height > max_h || (height > 60.0 && from_centre > 1500.0) {
                self.push(
                    "bldg.height_use",
                    centre,
                    vec![tag.clone()],
                    (height - max_h).max(floors as f32 - max_floors as f32).max(1.0),
                    format!("{} {height:.0} m, {floors} floors, {from_centre:.0} m from the centre", prefix(use_type)),
                );
            }
            // street access: some road or footway close enough to reach
            let reach = match use_type {
                ParcelUse::Village | ParcelUse::Farmstead | ParcelUse::Villa => 220.0,
                _ => 70.0,
            };
            let radius = area.sqrt() * 0.5;
            let near = self.roads.near(centre, reach + radius + 40.0);
            let best = near
                .iter()
                .filter(|o| at_grade(o.y))
                .map(|o| o.distance - self.network.roads[o.road].half_width())
                .fold(f32::MAX, f32::min);
            let nearest_junction = self
                .network
                .junctions
                .iter()
                .map(|j| (j.centre.distance(centre) - j.radius).max(0.0))
                .fold(f32::MAX, f32::min);
            let gap = best.min(nearest_junction) - radius;
            if gap > reach {
                self.push(
                    "bldg.street_access",
                    centre,
                    vec![tag.clone()],
                    gap - reach,
                    format!("{} {:.0} m from the nearest street", prefix(use_type), gap.max(0.0)),
                );
            }
            // pairs
            let (lo, hi) = (self.blds[i].lo, self.blds[i].hi);
            let margin = 13.0;
            for j in self.bgrid.query(
                Vec2::new(lo.x - margin, lo.y - margin),
                Vec2::new(hi.x + margin, hi.y + margin),
            ) {
                let j = j as usize;
                if j <= i {
                    continue;
                }
                let (verdict, mid, ids, detail) = {
                    let (a, b) = (&self.blds[i], &self.blds[j]);
                    let boxes_touch = a.lo.x <= b.hi.x
                        && a.hi.x >= b.lo.x
                        && a.lo.y <= b.hi.y
                        && a.hi.y >= b.lo.y;
                    let overlap = if boxes_touch { overlap_area(&a.ring, &b.ring) } else { 0.0 };
                    let mid = (a.centre + b.centre) * 0.5;
                    let ids = vec![format!("building:{}", a.id), format!("building:{}", b.id)];
                    // A tower standing on a lower, larger building (a shop podium)
                    // is one building in two parts, not an overlap.
                    let (low, high) = if a.height <= b.height { (a, b) } else { (b, a) };
                    let on_podium = (overlap >= 0.8 * high.area && low.area >= high.area * 0.95 && high.height > low.height + 3.0)
                        || self.podiums[i].is_some()
                        || self.podiums[j].is_some();
                    let stacked = on_podium;
                    if overlap > 1.0 && overlap > 0.03 * a.area.min(b.area) {
                        let detail = format!(
                            "{} {:.0} m and {} {:.0} m share {overlap:.0} m2",
                            prefix(a.use_type), a.height, prefix(b.use_type), b.height
                        );
                        (if stacked { None } else { Some(("overlap.building_building", overlap)) }, mid, ids, detail)
                    } else if overlap > 0.0 || stacked {
                        (None, mid, ids, String::new())
                    } else {
                        let gap = ring_gap(&a.ring, &b.ring);
                        if a.height > 24.0 && b.height > 24.0 && gap < 13.0 && gap >= 1.0 {
                            let detail = format!("{:.0} m and {:.0} m towers {gap:.1} m apart", a.height, b.height);
                            (Some(("bldg.spacing_highrise", 13.0 - gap)), mid, ids, detail)
                        } else if gap > 0.3 && gap < 4.0 && a.area > 30.0 && b.area > 30.0 {
                            (Some(("bldg.sliver", 4.0 - gap)), mid, ids, format!("{gap:.1} m between two buildings"))
                        } else {
                            (None, mid, ids, String::new())
                        }
                    }
                };
                if let Some((check, magnitude)) = verdict {
                    self.push(check, mid, ids, magnitude, detail);
                }
            }
            // building against roads, junctions and fields
            self.building_vs_streets(i);
            // industrial amid a residential core
            if use_type == ParcelUse::Industrial {
                let (mut dense, mut other) = (0, 0);
                for j in self.bgrid.query(
                    Vec2::new(centre.x - 90.0, centre.y - 90.0),
                    Vec2::new(centre.x + 90.0, centre.y + 90.0),
                ) {
                    let o = &self.blds[j as usize];
                    if o.id == id || o.centre.distance(centre) > 90.0 {
                        continue;
                    }
                    match o.use_type {
                        ParcelUse::Residential | ParcelUse::MixedUse if o.height >= 18.0 => dense += 1,
                        ParcelUse::Industrial | ParcelUse::Farmstead | ParcelUse::Park => other += 1,
                        _ => other += 1,
                    }
                }
                if dense >= 6 && dense as f32 >= 0.6 * (dense + other) as f32 {
                    self.push(
                        "land.industrial_core",
                        centre,
                        vec![tag.clone()],
                        dense as f32,
                        format!("{dense} residential towers and {other} other buildings within 90 m"),
                    );
                }
            }
            // slope
            if let Some(t) = self.terrain {
                let s = t.slope(centre.x, centre.y, 120.0);
                if s > 0.15 && !matches!(use_type, ParcelUse::Village | ParcelUse::Farmstead | ParcelUse::Villa) {
                    self.push(
                        "ground.slope_building",
                        centre,
                        vec![tag.clone()],
                        s,
                        format!("{} on {:.0} percent slope (terrain sampled over 240 m)", prefix(use_type), s * 100.0),
                    );
                }
            }
        }
        // villages and water
        if !self.channels.is_empty() {
            let mut seen: HashSet<(i32, i32)> = HashSet::new();
            for i in 0..self.blds.len() {
                let b = &self.blds[i];
                if b.use_type != ParcelUse::Village {
                    continue;
                }
                let cell = ((b.centre.x / 400.0).floor() as i32, (b.centre.y / 400.0).floor() as i32);
                if !seen.insert(cell) {
                    continue;
                }
                let dist = self.channel_distance(b.centre);
                if dist > 2500.0 {
                    let (c, id) = (b.centre, b.id);
                    self.push(
                        "land.village_water",
                        c,
                        vec![format!("building:{id}")],
                        dist,
                        format!("village {dist:.0} m from the river"),
                    );
                }
            }
        }
    }

    fn building_vs_streets(&mut self, i: usize) {
        let (id, centre, height, ring) = {
            let b = &self.blds[i];
            (b.id, b.centre, b.height, b.ring.clone())
        };
        let tag = format!("building:{id}");
        // intrusion into an at-grade carriageway, or under an elevated deck
        let mut worst: Option<(f32, Vec2, u32, bool)> = None;
        for p in ring_samples(&ring, 1.5) {
            for near in self.roads.near(p, 30.0) {
                let road = &self.network.roads[near.road];
                let half = road.half_width();
                if near.distance >= half {
                    continue;
                }
                if at_grade(near.y) {
                    let depth = half - near.distance;
                    if depth > 0.3 && worst.is_none_or(|w| depth > w.0) {
                        worst = Some((depth, p, road.id, false));
                    }
                } else {
                    // The building stands in the deck's footprint: it must stay
                    // 4.5 m under the underside.
                    let room = near.y - road.deck_thickness - height;
                    if room < 4.5 {
                        let depth = 4.5 - room;
                        if worst.is_none_or(|w| depth > w.0) {
                            worst = Some((depth, p, road.id, true));
                        }
                    }
                }
            }
            if let Some(j) = self.junction_pavement(p) {
                let depth = distance_to_ring(p, &j.ring);
                if depth > 0.3 && worst.is_none_or(|w| depth > w.0) {
                    worst = Some((depth, p, 0, false));
                }
            }
        }
        // a road running through the building
        let (lo, hi) = (self.blds[i].lo, self.blds[i].hi);
        for near in self.roads.near(centre, (hi - lo).length() * 0.5 + 12.0) {
            let road = &self.network.roads[near.road];
            if !at_grade(near.y) {
                continue;
            }
            let pts = road.carriageway.points();
            for w in pts.windows(2) {
                let (a, b) = (Vec2::new(w[0].x, w[0].z), Vec2::new(w[1].x, w[1].z));
                for t in [0.0, 0.5] {
                    let p = a.lerp(b, t);
                    if p.x >= lo.x && p.x <= hi.x && p.y >= lo.y && p.y <= hi.y && point_in_ring(p, &ring) {
                        let depth = road.half_width() + 1.0;
                        if worst.is_none_or(|w| depth > w.0) {
                            worst = Some((depth, p, road.id, false));
                        }
                    }
                }
            }
        }
        if let Some((depth, p, road, deck)) = worst {
            let mut ids = vec![tag.clone()];
            if road != 0 {
                ids.push(format!("road:{road}"));
            }
            let what = if deck { "under a viaduct with too little headroom" } else { "into the carriageway" };
            let class = if road == 0 { String::new() } else { self.network.road(road).map_or(String::new(), |r| format!(" ({:?}, layer {})", r.class, r.layer)) };
            self.push("overlap.building_road", p, ids, depth, format!("{height:.0} m building {depth:.1} m {what}{class}"));
        }
        // fields
        for fi in 0..self.city.fields.len() {
            let f = &self.city.fields[fi];
            let fr: Vec<Vec2> = f.ring.iter().map(|p| self.local(p)).collect();
            let (flo, fhi) = bounds(&fr);
            if flo.x > hi.x || fhi.x < lo.x || flo.y > hi.y || fhi.y < lo.y {
                continue;
            }
            let area = overlap_area(&ring, &fr);
            if area > 4.0 {
                let fid = f.id;
                self.push(
                    "overlap.building_field",
                    centre,
                    vec![tag.clone(), format!("field:{fid}")],
                    area,
                    format!("{area:.0} m2 of the footprint lies in a field"),
                );
            }
        }
    }

    // ---- (b)(f) trees and furniture ----------------------------------------

    fn instance_checks(&mut self) {
        if self.insts.is_empty() {
            return;
        }
        let prototypes: HashMap<String, (f32, f32, f32)> = crate::trees::prototype_set_all()
            .into_iter()
            .map(|p| (p.key.clone(), (p.height, p.crown, p.trunk)))
            .collect();
        let allowed: HashSet<&str> = {
            let env = crate::trees::zones::environment_for(&self.city.climate);
            let pl = &env.zone.planting;
            [pl.street, pl.motorway, pl.median, pl.park, pl.compound, pl.waterfront, pl.farm, pl.orchard, pl.island]
                .iter()
                .flat_map(|share| share.iter().map(|(k, _)| *k))
                .collect()
        };
        let zone = crate::trees::zones::environment_for(&self.city.climate).zone.key;
        let mut off_zone: HashMap<String, (usize, Vec2)> = HashMap::new();
        for i in 0..self.insts.len() {
            let inst = &self.insts[i];
            let (p, key, y, kind, scale) = (inst.p, inst.key.clone(), inst.y, inst.kind, inst.scale);
            let trunk = match kind {
                Kind::Tree => prototypes.get(&key).map_or(0.2, |&(h, _, t)| t / h * scale),
                Kind::Post(_) => 0.1,
            };
            // inside a building
            if let Some(b) = self.building_at(p) {
                let (bid, bh) = (b.id, b.height);
                let (id, what) = match kind {
                    Kind::Tree => ("overlap.tree_building", "trunk inside"),
                    Kind::Post(_) => ("overlap.furniture_building", "stands inside"),
                };
                self.push(id, p, vec![key.clone(), format!("building:{bid}")], 1.0, format!("{what} a {bh:.0} m building"));
                continue;
            }
            // in a motor lane or a junction box, on the surface the object stands on
            let mut hit: Option<(f32, String)> = None;
            for near in self.roads.near(p, 30.0) {
                let road = &self.network.roads[near.road];
                if (near.y - y).abs() > 1.5 {
                    continue;
                }
                let (inner, outer) = lane_band(road);
                let lat = near.lateral.abs();
                if lat > inner && lat - trunk < outer - 0.05 {
                    let depth = outer - lat + trunk;
                    if hit.as_ref().is_none_or(|h| depth > h.0) {
                        hit = Some((depth, format!("road:{}", road.id)));
                    }
                }
            }
            if hit.is_none()
                && at_grade(y)
                && let Some(j) = self.junction_pavement(p)
            {
                hit = Some((distance_to_ring(p, &j.ring), format!("junction:{}", j.node)));
            }
            if let Some((depth, road)) = hit {
                let id = if kind == Kind::Tree { "overlap.tree_road" } else { "overlap.furniture_lane" };
                self.push(id, p, vec![key.clone(), road], depth, format!("{depth:.1} m into the carriageway"));
            }
            if kind == Kind::Tree {
                let species = key.split('/').nth(1).unwrap_or("");
                if !allowed.contains(species) {
                    let e = off_zone.entry(species.to_owned()).or_insert((0, p));
                    e.0 += 1;
                }
                if let Some(&(h, crown, _)) = prototypes.get(&key)
                    && let Some(sp) = crate::species::SPECIES
                        .iter()
                        .chain(crate::species::PLANTS.iter())
                        .find(|s| s.key == species)
                {
                    let height = scale;
                    let crown_r = crown / h * scale;
                    if height > sp.height_m.1 * 1.3 || crown_r > sp.crown_m.1 * 1.3 {
                        self.push(
                            "veg.size",
                            p,
                            vec![key.clone()],
                            height / sp.height_m.1,
                            format!("{height:.1} m tall, crown {crown_r:.1} m (species up to {:.0} m, {:.1} m)", sp.height_m.1, sp.crown_m.1),
                        );
                    }
                }
            }
        }
        for (species, (count, at)) in off_zone {
            let index = CHECKS.iter().position(|c| c.id == "veg.species_climate").unwrap();
            self.counts[index] += count - 1;
            self.push(
                "veg.species_climate",
                at,
                vec![format!("species:{species}")],
                count as f32,
                format!("{count} trees of {species} where the site climate selects the {zone} zone"),
            );
        }
    }

    fn vehicle_checks(&mut self) {
        let Some(scene) = self.scene else { return };
        for (n, agent) in scene.traffic.agents.iter().enumerate() {
            let p = Vec2::new(agent.x, agent.z);
            if let Some(b) = self.building_at(p) {
                let bid = b.id;
                self.push("overlap.vehicle_building", p, vec![format!("vehicle:{n}"), format!("building:{bid}")], 1.0, "vehicle inside a building".into());
                continue;
            }
            let on_road = self
                .roads
                .near(p, 25.0)
                .iter()
                .any(|o| o.distance <= self.network.roads[o.road].half_width() + 1.0);
            if !on_road && self.junctions.containing(p).is_none() {
                self.push("overlap.vehicle_offroad", p, vec![format!("vehicle:{n}")], 1.0, "vehicle outside every carriageway and junction".into());
            }
        }
    }

    fn fence_checks(&mut self) {
        for ci in 0..self.city.compounds.len() {
            let c = &self.city.compounds[ci];
            let ring: Vec<Vec2> = c.boundary.iter().map(|p| self.local(p)).collect();
            let mut worst: Option<(f32, Vec2, u32)> = None;
            for p in ring_samples(&ring, 2.0) {
                for near in self.roads.near(p, 20.0) {
                    let road = &self.network.roads[near.road];
                    if !at_grade(near.y) {
                        continue;
                    }
                    let (_, outer) = lane_band(road);
                    let depth = outer - near.distance;
                    if depth > 0.3 && worst.is_none_or(|w| depth > w.0) {
                        worst = Some((depth, p, road.id));
                    }
                }
            }
            if let Some((depth, p, road)) = worst {
                let id = c.id;
                self.push(
                    "overlap.fence_road",
                    p,
                    vec![format!("compound:{id}"), format!("road:{road}")],
                    depth,
                    format!("fence line {depth:.1} m inside the motor lanes"),
                );
            }
        }
    }

    fn field_checks(&mut self) {
        for fi in 0..self.city.fields.len() {
            let f = &self.city.fields[fi];
            let fid = f.id;
            let ring: Vec<Vec2> = f.ring.iter().map(|p| self.local(p)).collect();
            let mut worst: Option<(f32, Vec2, u32)> = None;
            for p in ring_samples(&ring, 8.0) {
                for near in self.roads.near(p, 15.0) {
                    let road = &self.network.roads[near.road];
                    if !at_grade(near.y) {
                        continue;
                    }
                    let (_, outer) = lane_band(road);
                    let depth = outer - near.distance;
                    if depth > 0.5 && worst.is_none_or(|w| depth > w.0) {
                        worst = Some((depth, p, road.id));
                    }
                }
            }
            if let Some((depth, p, road)) = worst {
                self.push(
                    "overlap.field_road",
                    p,
                    vec![format!("field:{fid}"), format!("road:{road}")],
                    depth,
                    format!("field edge {depth:.1} m into the motor lanes"),
                );
            }
            if let Some(t) = self.terrain {
                let c = centroid(&ring);
                let s = t.slope(c.x, c.y, 120.0);
                // 25 degrees is the limit on cultivated land (Land Administration Law);
                // a terraced field may be steeper than a ploughed one.
                if s > 0.466 {
                    self.push(
                        "ground.slope_field",
                        c,
                        vec![format!("field:{fid}")],
                        s,
                        format!("{:.0} percent slope", s * 100.0),
                    );
                }
            }
        }
    }

    // ---- (c) grade separation ------------------------------------------------

    fn road_crossings(&mut self) {
        let mut grid = Grid::new(30.0);
        let mut segs: Vec<(usize, usize)> = Vec::new();
        for (ri, road) in self.network.roads.iter().enumerate() {
            let pts = road.carriageway.points();
            for s in 0..pts.len().saturating_sub(1) {
                let (a, b) = (pts[s], pts[s + 1]);
                grid.insert(
                    segs.len() as u32,
                    Vec2::new(a.x.min(b.x), a.z.min(b.z)),
                    Vec2::new(a.x.max(b.x), a.z.max(b.z)),
                );
                segs.push((ri, s));
            }
        }
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for (id, &(ri, si)) in segs.iter().enumerate() {
            let r = &self.network.roads[ri];
            let pts = r.carriageway.points();
            let (a3, b3) = (pts[si], pts[si + 1]);
            let (a, b) = (Vec2::new(a3.x, a3.z), Vec2::new(b3.x, b3.z));
            for other in grid.query(Vec2::new(a.x.min(b.x), a.y.min(b.y)), Vec2::new(a.x.max(b.x), a.y.max(b.y))) {
                if other as usize <= id {
                    continue;
                }
                let (rj, sj) = segs[other as usize];
                if rj == ri {
                    continue;
                }
                let q = &self.network.roads[rj];
                if r.from_node == q.from_node || r.from_node == q.to_node || r.to_node == q.from_node || r.to_node == q.to_node {
                    continue;
                }
                let qp = q.carriageway.points();
                let (c3, d3) = (qp[sj], qp[sj + 1]);
                let (c, d) = (Vec2::new(c3.x, c3.z), Vec2::new(d3.x, d3.z));
                let Some((t, u)) = segment_intersection(a, b, c, d) else { continue };
                if !seen.insert((ri.min(rj), ri.max(rj))) {
                    continue;
                }
                let y1 = a3.y + (b3.y - a3.y) * t;
                let y2 = c3.y + (d3.y - c3.y) * u;
                let at = a.lerp(b, t);
                let ids = vec![format!("road:{}", r.id), format!("road:{}", q.id)];
                let dy = (y1 - y2).abs();
                if dy < 1.0 {
                    self.push(
                        "bridge.at_grade_crossing",
                        at,
                        ids,
                        1.0 - dy,
                        format!("{:?} and {:?} cross at {y1:.1} m and {y2:.1} m with no junction", r.class, q.class),
                    );
                } else {
                    let upper = if y1 > y2 { r } else { q };
                    let clear = dy - upper.deck_thickness;
                    if clear < 4.5 - 0.05 {
                        self.push(
                            "bridge.clear_road",
                            at,
                            ids,
                            4.5 - clear,
                            format!("headroom {clear:.2} m under a deck {dy:.2} m above the road below"),
                        );
                    }
                }
            }
        }
    }

    fn junction_checks(&mut self) {
        let signalled: HashSet<u32> = self
            .scene
            .map(|s| s.signals.iter().map(|r| r.junction).collect())
            .unwrap_or_default();
        for j in &self.network.junctions {
            let at = j.centre;
            // ends meet the junction in plan and in height
            for port in &j.ports {
                let Some(road) = self.network.road(port.road) else { continue };
                let end = if port.at_start { road.centreline.start() } else { road.centreline.end() };
                let step = (end.y - j.y).abs();
                if step > 0.2 {
                    self.push(
                        "bridge.step",
                        at,
                        vec![format!("road:{}", road.id), format!("junction:{}", j.node)],
                        step,
                        format!("road end at {:.2} m, junction at {:.2} m", end.y, j.y),
                    );
                }
                let gap = Vec2::new(end.x, end.z).distance(j.centre);
                if gap > j.radius + 2.0 {
                    self.push(
                        "bridge.gap",
                        at,
                        vec![format!("road:{}", road.id), format!("junction:{}", j.node)],
                        gap - j.radius,
                        format!("road end {gap:.1} m from the junction centre (box radius {:.1})", j.radius),
                    );
                }
            }
            if j.ports.len() < 3 || j.roundabout || self.network.is_air(j.node) {
                continue;
            }
            let ramp_arm = j.ports.iter().any(|p| self.network.road(p.road).is_some_and(|r| r.oneway));
            // arms closer than 30 degrees
            if !ramp_arm && j.kind != JunctionKind::GradeSeparated {
                let mut angles: Vec<f32> = j.ports.iter().map(|p| p.dir.y.atan2(p.dir.x).to_degrees()).collect();
                angles.sort_by(|a, b| a.total_cmp(b));
                let mut min = f32::MAX;
                for k in 0..angles.len() {
                    let next = angles[(k + 1) % angles.len()];
                    let mut d = next - angles[k];
                    if d <= 0.0 {
                        d += 360.0;
                    }
                    min = min.min(d);
                }
                if min < 30.0 {
                    self.push(
                        "net.junction_angle",
                        at,
                        vec![format!("junction:{}", j.node)],
                        30.0 - min,
                        format!("{} arms, two only {min:.0} degrees apart", j.ports.len()),
                    );
                }
            }
            // control
            if self.scene.is_some() {
                let has_rig = signalled.contains(&j.node);
                let major = j
                    .ports
                    .iter()
                    .filter_map(|p| self.network.road(p.road))
                    .filter(|r| matches!(r.class, ModernRoadClass::Arterial | ModernRoadClass::Expressway))
                    .count();
                let id = format!("junction:{}", j.node);
                if j.kind == JunctionKind::Signalized && !has_rig {
                    self.push("net.junction_control", at, vec![id.clone()], 1.0, "signalised junction with no signal heads".into());
                } else if j.kind != JunctionKind::Signalized && has_rig {
                    self.push("net.junction_control", at, vec![id.clone()], 1.0, format!("{:?} junction carrying signal heads", j.kind));
                } else if major >= 3 && j.kind == JunctionKind::Priority {
                    self.push("net.junction_control", at, vec![id.clone()], 2.0, format!("{major} arterial arms meeting under priority control"));
                }
            }
            let footway_arms: Vec<_> = j
                .ports
                .iter()
                .filter_map(|p| self.network.road(p.road).map(|r| (r, p.at_start)))
                .filter(|(r, _)| r.layer == 0 && !r.oneway && r.section.sidewalk_metres >= 1.0)
                .collect();
            if footway_arms.len() >= 3
                && !matches!(j.kind, JunctionKind::RightInRightOut | JunctionKind::GradeSeparated)
                && !footway_arms.iter().any(|(r, start)| if *start { r.walk_start } else { r.walk_end })
            {
                self.push(
                    "net.junction_control",
                    at,
                    vec![format!("junction:{}", j.node)],
                    1.0,
                    format!("{} footway arms and no pedestrian crossing", footway_arms.len()),
                );
            }
        }
        for w in &self.network.warnings {
            self.push("network.warning", Vec2::new(0.0, 0.0), vec![w.chars().take(60).collect()], 1.0, w.clone());
        }
        if let Some(s) = self.scene {
            for w in s.stats.warnings.iter().skip(self.network.warnings.len()) {
                self.push("network.warning", Vec2::new(0.0, 0.0), vec!["scene".into()], 1.0, w.clone());
            }
        }
    }

    // ---- (d)(i) roads ---------------------------------------------------------

    fn road_checks(&mut self) {
        for ri in 0..self.network.roads.len() {
            let road = &self.network.roads[ri];
            let id = road.id;
            let mid = {
                let (p, _) = road.carriageway.sample(road.carriageway.length() * 0.5);
                Vec2::new(p.x, p.z)
            };
            let s = &road.section;
            if s.motor_lane_width < 3.0 || s.motor_lane_width > 3.75 {
                self.push(
                    "net.lane_width",
                    mid,
                    vec![format!("road:{id}")],
                    (s.motor_lane_width - 3.375).abs(),
                    format!("{:?} lane {:.2} m wide (3.0-3.75)", road.class, s.motor_lane_width),
                );
            } else if road.has_sidewalk() && s.sidewalk_metres < 1.5 {
                self.push(
                    "net.lane_width",
                    mid,
                    vec![format!("road:{id}")],
                    1.5 - s.sidewalk_metres,
                    format!("{:?} footway {:.2} m wide (at least 1.5)", road.class, s.sidewalk_metres),
                );
            }
            let grade = road.max_grade;
            if road.oneway {
                if grade > 0.052 {
                    self.push("ramp.grade", mid, vec![format!("road:{id}")], grade, format!("ramp reaches {:.1} percent", grade * 100.0));
                }
                let ends = [road.from_node, road.to_node];
                if ends.iter().any(|n| self.network.incident_roads(*n).len() < 2) {
                    self.push("ramp.continuity", mid, vec![format!("road:{id}")], 1.0, "a ramp end joins no other road".into());
                }
            } else {
                let limit = if road.class == ModernRoadClass::Expressway { 0.05 } else { 0.06 };
                if grade > limit + 0.002 {
                    self.push(
                        "bridge.grade",
                        mid,
                        vec![format!("road:{id}")],
                        grade,
                        format!("{:?} reaches {:.1} percent (limit {:.0})", road.class, grade * 100.0, limit * 100.0),
                    );
                }
            }
        }
    }

    fn graph_checks(&mut self) {
        let city = self.city;
        let n = city.nodes.len();
        if n == 0 {
            return;
        }
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        let index: HashMap<u32, usize> = city.nodes.iter().enumerate().map(|(i, nd)| (nd.id, i)).collect();
        let mut degree = vec![0usize; n];
        let mut sole: Vec<Option<u32>> = vec![None; n];
        for r in &city.sd_roads {
            let (Some(&a), Some(&b)) = (index.get(&r.from), index.get(&r.to)) else { continue };
            let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
            parent[ra] = rb;
            if a != b {
                degree[a] += 1;
                degree[b] += 1;
                sole[a] = Some(r.id);
                sole[b] = Some(r.id);
            }
        }
        let mut roads_in: HashMap<usize, usize> = HashMap::new();
        for r in &city.sd_roads {
            if let Some(&a) = index.get(&r.from) {
                *roads_in.entry(find(&mut parent, a)).or_default() += 1;
            }
        }
        let main = roads_in.iter().max_by_key(|(_, c)| **c).map(|(k, _)| *k);
        let mut reported: HashSet<usize> = HashSet::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            if Some(root) != main && reported.insert(root) {
                let count = roads_in.get(&root).copied().unwrap_or(0);
                let p = self.local(&city.nodes[i].point);
                self.push(
                    "net.orphan",
                    p,
                    vec![format!("node:{}", city.nodes[i].id)],
                    count as f32,
                    if count == 0 {
                        "road node that no road uses".to_owned()
                    } else {
                        format!("component of {count} roads not joined to the main network")
                    },
                );
            }
        }
        let by_id: HashMap<u32, &urban::HdRoad> = city.hd_roads.iter().map(|h| (h.sd_road, h)).collect();
        for i in 0..n {
            if degree[i] != 1 || city.nodes[i].role == "regional-gateway" {
                continue;
            }
            let Some(rid) = sole[i] else { continue };
            let Some(sd) = city.sd_roads.iter().find(|r| r.id == rid) else { continue };
            let len = by_id.get(&rid).map_or(0.0, |h| {
                h.centreline.windows(2).map(|w| self.local(&w[0]).distance(self.local(&w[1]))).sum::<f32>()
            });
            let plausible = match sd.class {
                ModernRoadClass::Local => len <= 320.0,
                _ => false,
            };
            if !plausible {
                let p = self.local(&city.nodes[i].point);
                self.push(
                    "net.dead_end",
                    p,
                    vec![format!("road:{rid}"), format!("node:{}", city.nodes[i].id)],
                    len,
                    format!("{:?} road ends in nothing after {len:.0} m", sd.class),
                );
            }
        }
    }

    // ---- (j) level of detail -----------------------------------------------------

    fn lod_checks(&mut self) {
        if self.city.buildings.len() > self.max_buildings {
            let drop = self.city.buildings.len() - self.max_buildings;
            self.push(
                "lod.count",
                Vec2::new(0.0, 0.0),
                vec!["scene".into()],
                drop as f32,
                format!("{} of {} buildings have a far form but no detail", drop, self.city.buildings.len()),
            );
        }
        for (i, b) in self.city.buildings.iter().enumerate() {
            let Some(look) = crate::buildings::far::far_look(b, self.podiums.get(i).copied().flatten()) else {
                continue;
            };
            let diff = (look.roof_m - b.height_metres).abs();
            if diff > 1.0 {
                let c = b.footprint.first().map_or(Vec2::new(0.0, 0.0), |p| self.local(p));
                let (id, use_type) = (b.id, b.use_type);
                let height = b.height_metres;
                self.push(
                    "lod.roof",
                    c,
                    vec![format!("building:{id}")],
                    diff,
                    format!("{} plan height {height:.1} m, detail roof {:.1} m", prefix(use_type), look.roof_m),
                );
            }
        }
    }
}
