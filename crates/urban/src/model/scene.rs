use serde::{Deserialize, Serialize};

use super::buildings::TreeInstance;
use super::buildings::{Compound, ModernBuilding, Parcel};
use super::core::{CityStyle, Point, UrbanBlock};
use super::roads::{HdRoad, SdNode, SdRoad};

/// City-local metric frame.  `urban` generates in kilometres with a rotation,
/// but every downstream consumer (street geometry, traffic, the renderer) wants
/// metres around the city centre.  The frame travels with the payload so the
/// inverse transform is exact instead of re-derived from the specification.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityFrameInfo {
    /// City centre in world kilometres.
    pub origin: Point,
    pub rotation_radians: f32,
}

impl CityFrameInfo {
    /// World kilometre point → city-local metres, `(x, z)`, y up.
    pub fn to_local(self, point: Point) -> [f32; 2] {
        let (sin, cos) = self.rotation_radians.sin_cos();
        let dx = (point.x_km - self.origin.x_km) * 1000.0;
        let dy = (point.y_km - self.origin.y_km) * 1000.0;
        [dx * cos + dy * sin, -dx * sin + dy * cos]
    }

    /// City-local metres → world kilometre point.
    pub fn to_world(self, x_m: f32, z_m: f32) -> Point {
        let (sin, cos) = self.rotation_radians.sin_cos();
        Point {
            x_km: self.origin.x_km + (x_m * cos - z_m * sin) / 1000.0,
            y_km: self.origin.y_km + (x_m * sin + z_m * cos) / 1000.0,
        }
    }
}

/// Rich output used by the Rust renderer.  `sd` is the city-scale graph,
/// A waterway besides the main river.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tributary {
    pub line: Vec<Point>,
    pub width_metres: f32,
}

/// `hd` contains physical road cross-sections and centre lines, while blocks,
/// parcels and buildings retain stable ids for incremental rendering and
/// inspection.  All fields are deterministic for the same specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModernCity {
    pub version: u32,
    pub jurisdiction: String,
    pub style: CityStyle,
    pub seed: u32,
    pub frame: CityFrameInfo,
    pub nodes: Vec<SdNode>,
    pub sd_roads: Vec<SdRoad>,
    pub hd_roads: Vec<HdRoad>,
    pub blocks: Vec<UrbanBlock>,
    pub parcels: Vec<Parcel>,
    pub buildings: Vec<ModernBuilding>,
    pub compounds: Vec<Compound>,
    pub trees: Vec<TreeInstance>,
    pub river: Option<Vec<Point>>,
    pub river_width_metres: f32,
    /// Other waterways through the town, each with its own width: a plan made on a
    /// real landscape has more than one.
    #[serde(default)]
    pub tributaries: Vec<Tributary>,
    /// Squared deviation of the SD graph from the realistic Chinese morphology
    /// prior (`MorphologyPrior::default`).  Exposed so callers and tests can
    /// assert a generated plan actually reads like a Chinese street network.
    pub morphology_score: f64,
}
