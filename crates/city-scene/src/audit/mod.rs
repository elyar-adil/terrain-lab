//! Plausibility audit: does what the generator made make sense?
//!
//! `docs/methodology.md` principle 7 says a generated feature is acceptable only
//! if someone who knows the real thing would find it plausible in place,
//! function and relation to its neighbours, and that this is checked by rules.
//! This module is those rules. It takes a city plan (`urban::ModernCity`), the
//! street network derived from it, and optionally the built scene (for the
//! trees, lamps, signs and vehicles, which exist only as instances), and
//! returns a [`Report`]: for each check a count, the worst offenders with
//! coordinates and ids, and the module that most probably owns the defect.
//!
//! Run it from the command line with
//! `cargo run --release -p city-scene --example sanity_audit` (see
//! `docs/sanity-audit.md`), or from a test with [`audit_city`].
//!
//! All coordinates in a [`Violation`] are city-local metres (`x`, `z`) in the
//! frame of the plan, the same ones the scene uses.

mod checks;
pub(crate) mod geom;

use serde::Serialize;

use urban::ModernCity;

use crate::network::Network;
use crate::scene::CityScene;

pub use checks::Terrain;

#[cfg(test)]
mod tests;

/// What one check is, who owns the code it judges, and how much it matters.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct CheckInfo {
    pub id: &'static str,
    pub group: &'static str,
    pub title: &'static str,
    /// The module that most probably produces the defect.
    pub owner: &'static str,
    /// 1 cosmetic or unproven, 2 wrong but local, 3 plainly broken.
    pub severity: u8,
    /// 1 needs a close look, 2 visible from the street, 3 visible from afar.
    pub visibility: u8,
    /// The most violations the regression gate on the demo city accepts.
    pub gate: usize,
}

macro_rules! check {
    ($id:expr, $group:expr, $title:expr, $owner:expr, $sev:expr, $vis:expr, $gate:expr) => {
        CheckInfo {
            id: $id,
            group: $group,
            title: $title,
            owner: $owner,
            severity: $sev,
            visibility: $vis,
            gate: $gate,
        }
    };
}

/// Every check. The gate numbers are what the demo city is held to in the
/// test suite; lower them as defects are fixed, never raise them.
pub const CHECKS: &[CheckInfo] = &[
    // (a) ground and water
    check!(
        "water.building",
        "ground",
        "building footprint in the river channel",
        "urban parcels / buildings::river",
        3,
        3,
        0
    ),
    check!(
        "water.road",
        "ground",
        "road at grade through the river channel",
        "urban roads + network deck",
        3,
        3,
        0
    ),
    check!(
        "water.tree",
        "ground",
        "tree standing in the river channel",
        "trees::plant",
        2,
        2,
        0
    ),
    check!(
        "water.furniture",
        "ground",
        "lamp, pole or sign standing in the river channel",
        "furniture",
        2,
        2,
        1
    ),
    check!(
        "water.field",
        "ground",
        "field ring in the river channel",
        "urban landscape / crops",
        2,
        3,
        0
    ),
    check!(
        "ground.level",
        "ground",
        "object not on the surface under it (floating, or sunk into a deck)",
        "furniture / trees",
        2,
        2,
        0
    ),
    check!(
        "ground.slope_building",
        "ground",
        "building on slope steeper than 15 percent",
        "urban parcels",
        2,
        2,
        0
    ),
    check!(
        "ground.slope_field",
        "ground",
        "cultivated field on slope steeper than 25 degrees",
        "urban landscape",
        2,
        2,
        0
    ),
    // (b) overlaps
    check!(
        "overlap.building_building",
        "overlap",
        "two buildings overlapping",
        "urban parcels",
        3,
        3,
        0
    ),
    check!(
        "overlap.building_road",
        "overlap",
        "building inside a carriageway, junction or under a deck",
        "urban parcels vs network",
        3,
        3,
        0
    ),
    check!(
        "overlap.building_field",
        "overlap",
        "building inside a field",
        "urban landscape",
        2,
        2,
        0
    ),
    check!(
        "overlap.field_road",
        "overlap",
        "field ring crossing a carriageway",
        "urban landscape",
        2,
        2,
        0
    ),
    check!(
        "overlap.tree_road",
        "overlap",
        "tree trunk in a motor lane or junction",
        "trees::plant",
        3,
        2,
        0
    ),
    check!(
        "overlap.tree_building",
        "overlap",
        "tree trunk inside a building (on its roof)",
        "trees::plant",
        3,
        2,
        0
    ),
    check!(
        "overlap.furniture_lane",
        "overlap",
        "lamp, pole, sign, bin or shelter in a motor lane or junction",
        "furniture",
        3,
        2,
        0
    ),
    check!(
        "overlap.furniture_building",
        "overlap",
        "lamp, pole or sign inside a building",
        "furniture",
        3,
        2,
        0
    ),
    check!(
        "overlap.vehicle_building",
        "overlap",
        "vehicle inside a building",
        "traffic",
        3,
        2,
        0
    ),
    check!(
        "overlap.vehicle_offroad",
        "overlap",
        "vehicle not on any carriageway",
        "traffic",
        3,
        2,
        0
    ),
    check!(
        "overlap.fence_road",
        "overlap",
        "compound fence or gate on a carriageway",
        "urban parcels (compounds)",
        2,
        2,
        0
    ),
    // (c) bridges and grade separation
    check!(
        "bridge.clear_water",
        "bridge",
        "bridge deck less than 1.5 m over the water",
        "network deck",
        3,
        3,
        0
    ),
    check!(
        "bridge.clear_road",
        "bridge",
        "deck over a road with less than 4.5 m headroom",
        "network deck",
        3,
        3,
        0
    ),
    check!(
        "bridge.at_grade_crossing",
        "bridge",
        "two roads cross at the same level with no junction",
        "urban roads / expressway",
        3,
        3,
        0
    ),
    check!(
        "bridge.step",
        "bridge",
        "road end not level with its junction (a step)",
        "network",
        2,
        2,
        0
    ),
    check!(
        "bridge.gap",
        "bridge",
        "road end not meeting its junction in plan (a gap)",
        "network",
        2,
        2,
        0
    ),
    check!(
        "bridge.grade",
        "bridge",
        "grade above the standard for the road",
        "network deck",
        2,
        3,
        0
    ),
    check!(
        "network.warning",
        "bridge",
        "engineering problem the network derivation itself reported",
        "network",
        2,
        2,
        0
    ),
    // (d) road network
    check!(
        "net.orphan",
        "network",
        "road component not connected to the main network",
        "urban graph",
        3,
        3,
        0
    ),
    check!(
        "net.dead_end",
        "network",
        "dead end implausible for its class or length",
        "urban graph",
        2,
        2,
        2
    ),
    check!(
        "net.lane_width",
        "network",
        "lane or footway width outside the standard",
        "worldgen-contracts",
        2,
        2,
        0
    ),
    check!(
        "net.junction_angle",
        "network",
        "junction arms closer than 30 degrees",
        "urban graph / tidy",
        2,
        2,
        0
    ),
    check!(
        "net.junction_control",
        "network",
        "junction with the wrong or no control for its class",
        "network / street::signals",
        2,
        2,
        0
    ),
    // (e) buildings
    check!(
        "bldg.storey",
        "building",
        "floor-to-floor height outside the range for the use",
        "urban parcels / buildings",
        2,
        2,
        1
    ),
    check!(
        "bldg.height_use",
        "building",
        "height or storeys beyond what the use allows",
        "urban parcels",
        2,
        3,
        0
    ),
    check!(
        "bldg.street_access",
        "building",
        "no street or footway within reach of the building",
        "urban parcels",
        2,
        1,
        0
    ),
    check!(
        "bldg.spacing_highrise",
        "building",
        "two high-rises closer than 13 m (GB 50016)",
        "urban parcels",
        2,
        2,
        0
    ),
    check!(
        "bldg.sliver",
        "building",
        "gap between two buildings narrower than 4 m but not touching",
        "urban parcels",
        1,
        2,
        0
    ),
    check!(
        "bldg.degenerate",
        "building",
        "footprint under 12 square metres or self-crossing",
        "urban parcels",
        1,
        1,
        0
    ),
    // (f) vegetation
    check!(
        "veg.species_climate",
        "vegetation",
        "species not in the planting zone of the site climate",
        "trees::zones / plant",
        2,
        2,
        0
    ),
    check!(
        "veg.size",
        "vegetation",
        "tree height or crown outside the species range",
        "trees::plant",
        2,
        2,
        0
    ),
    // (h) land use
    check!(
        "land.industrial_core",
        "landuse",
        "industrial building amid a dense residential core",
        "urban parcels",
        2,
        2,
        0
    ),
    check!(
        "land.village_water",
        "landuse",
        "village far from any water",
        "urban suburb",
        1,
        1,
        0
    ),
    // (i) interchange
    check!(
        "ramp.grade",
        "interchange",
        "ramp grade above 5 percent",
        "network deck / expressway",
        3,
        3,
        0
    ),
    check!(
        "ramp.continuity",
        "interchange",
        "ramp end that joins no road",
        "urban expressway",
        3,
        3,
        0
    ),
    // (j) level of detail
    check!(
        "lod.roof",
        "lod",
        "far roof height differs from the detail roof",
        "buildings::far",
        2,
        3,
        9
    ),
    check!(
        "lod.count",
        "lod",
        "buildings dropped from the detail but present in the far mass",
        "scene budget",
        2,
        3,
        0
    ),
];

pub fn info(id: &str) -> &'static CheckInfo {
    CHECKS
        .iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("unknown audit check {id}"))
}

/// One finding.
#[derive(Debug, Clone, Serialize)]
pub struct Violation {
    pub check: &'static str,
    /// City-local metres.
    pub x: f32,
    pub z: f32,
    /// Ids of the objects involved (`building:12`, `road:7`, ...).
    pub ids: Vec<String>,
    /// How bad, in the check's unit (metres of intrusion, grade, ...).
    pub magnitude: f32,
    pub detail: String,
}

/// Counts for one check.
#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub id: &'static str,
    pub group: &'static str,
    pub title: &'static str,
    pub owner: &'static str,
    pub severity: u8,
    pub visibility: u8,
    pub count: usize,
    pub gate: usize,
    /// Count times severity times visibility: the ranking key.
    pub score: usize,
    pub worst: Vec<Violation>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Inventory {
    pub buildings: usize,
    pub roads: usize,
    pub junctions: usize,
    pub trees: usize,
    pub furniture: usize,
    pub vehicles: usize,
    pub fields: usize,
    pub compounds: usize,
    pub bridges: usize,
    pub ramps: usize,
    pub signalised: usize,
    pub rivers: usize,
    pub scene_checked: bool,
    pub terrain_checked: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub inventory: Inventory,
    pub summaries: Vec<Summary>,
}

impl Report {
    pub fn count(&self, id: &str) -> usize {
        self.summaries
            .iter()
            .find(|s| s.id == id)
            .map_or(0, |s| s.count)
    }

    pub fn total(&self) -> usize {
        self.summaries.iter().map(|s| s.count).sum()
    }

    /// Checks over their gate, as `(id, count, gate)`.
    pub fn over_gate(&self) -> Vec<(&'static str, usize, usize)> {
        self.summaries
            .iter()
            .filter(|s| s.count > s.gate)
            .map(|s| (s.id, s.count, s.gate))
            .collect()
    }

    /// Summaries with findings, most important first.
    pub fn ranked(&self) -> Vec<&Summary> {
        let mut list: Vec<&Summary> = self.summaries.iter().filter(|s| s.count > 0).collect();
        list.sort_by(|a, b| b.score.cmp(&a.score));
        list
    }

    pub fn print(&self, top: usize) {
        let i = &self.inventory;
        println!(
            "audited {} buildings, {} roads ({} bridges, {} ramps), {} junctions ({} signalised), {} trees, {} furniture, {} vehicles, {} fields, {} waterways (scene {}, terrain {})",
            i.buildings,
            i.roads,
            i.bridges,
            i.ramps,
            i.junctions,
            i.signalised,
            i.trees,
            i.furniture,
            i.vehicles,
            i.fields,
            i.rivers,
            if i.scene_checked { "yes" } else { "no" },
            if i.terrain_checked { "yes" } else { "no" }
        );
        println!(
            "{:<28} {:>6} {:>5} {:>3}  {}",
            "check", "count", "gate", "sev", "title"
        );
        for s in &self.summaries {
            let mark = if s.count > s.gate {
                "FAIL"
            } else if s.count > 0 {
                "ok"
            } else {
                ""
            };
            println!(
                "{:<28} {:>6} {:>5} {:>3}  {} {}",
                s.id, s.count, s.gate, s.severity, s.title, mark
            );
        }
        for s in self.ranked() {
            println!(
                "\n{} ({}, {} found, owner: {})",
                s.id, s.title, s.count, s.owner
            );
            for v in s.worst.iter().take(top) {
                println!(
                    "  at ({:.1}, {:.1})  {:.2}  {}  [{}]",
                    v.x,
                    v.z,
                    v.magnitude,
                    v.detail,
                    v.ids.join(", ")
                );
            }
        }
    }
}

/// How many worst offenders each summary keeps.
pub const KEEP_WORST: usize = 40;

/// Audit a plan. `network` is `scene::derive_city(city)`; `scene` is the built
/// scene when the trees, furniture and vehicles should be judged too;
/// `terrain` supplies ground heights for the slope checks.
pub fn audit(
    city: &ModernCity,
    network: &Network,
    scene: Option<&CityScene>,
    terrain: Option<&Terrain>,
    max_buildings: usize,
) -> Report {
    let mut cx = checks::Context::new(city, network, scene, terrain, max_buildings);
    cx.run_all();
    cx.finish()
}

/// Derive the network, build the scene and audit both: the gate used by tests.
pub fn audit_city(city: &ModernCity, budget: crate::scene::SceneBudget) -> Report {
    let network = crate::scene::derive_city(city);
    let scene = crate::scene::build_city_scene(city, budget);
    audit(city, &network, Some(&scene), None, budget.max_buildings)
}
