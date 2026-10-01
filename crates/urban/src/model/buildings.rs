use serde::{Deserialize, Serialize};

use super::core::{Point, RoofStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParcelUse {
    Residential,
    MixedUse,
    Commercial,
    Civic,
    Park,
    /// A detached house with its own garden: the suburban belt around a town.
    Villa,
    /// A farmhouse and its yard, standing alone beside a country road.
    Farmstead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcel {
    pub id: u32,
    pub block_id: u32,
    pub ring: Vec<Point>,
    pub use_type: ParcelUse,
    pub compound: bool,
    /// Index into `ring` of the edge fronting the widest adjacent street —
    /// where a gated compound puts its entrance.
    pub gate_edge: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModernBuilding {
    pub id: u32,
    pub parcel_id: u32,
    pub footprint: Vec<Point>,
    pub height_metres: f32,
    pub floors: u16,
    pub use_type: ParcelUse,
    pub roof: RoofStyle,
    pub facade: BuildingFacade,
    pub podium_height_metres: f32,
    pub window_bays: u16,
    pub balcony_bays: u16,
    pub entrance_count: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BuildingFacade {
    CurtainWall,
    ConcreteGlass,
    StoneCivic,
    BrickResidential,
    MetalIndustrial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compound {
    pub id: u32,
    pub parcel_id: u32,
    pub boundary: Vec<Point>,
    pub courtyard: Option<Vec<Point>>,
    pub gate_points: Vec<Point>,
    /// Internal drive loops (fire lane), drawn as light ribbons.
    pub paths: Vec<Vec<Point>>,
    pub road_width_metres: f32,
    pub fence_height_metres: f32,
    pub planted_ratio: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TreeSpecies {
    ChinesePlane,
    Ginkgo,
    Cedar,
    Bamboo,
    Willow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeInstance {
    pub id: u32,
    pub point: Point,
    pub species: TreeSpecies,
    /// Which L-System prototype variant of the species this tree instances;
    /// the prototype geometry set is emitted once per generation request.
    pub variant: u16,
    pub height_metres: f32,
    pub crown_radius_metres: f32,
    pub trunk_radius_metres: f32,
}
