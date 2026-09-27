use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x_km: f32,
    pub y_km: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CityStyle {
    /// Contemporary mainland Chinese city: hierarchical roads, compact
    /// mixed-use blocks, residential compounds and a planted river corridor.
    ChineseModern,
    Parisian,
    BarcelonaEixample,
    Manhattan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StreetClass {
    Boulevard,
    Avenue,
    Street,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoofStyle {
    Mansard,
    Terracotta,
    Flat,
    SetbackTower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreetSegment {
    pub from: Point,
    pub to: Point,
    pub class: StreetClass,
    pub width_metres: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanBlock {
    pub boundary: Vec<Point>,
    pub courtyard: Option<Vec<Point>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingMass {
    pub footprint: Vec<Point>,
    pub courtyard: Option<Vec<Point>>,
    pub height_metres: f32,
    pub roof: RoofStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanModel {
    pub style: CityStyle,
    pub streets: Vec<StreetSegment>,
    pub blocks: Vec<UrbanBlock>,
    pub buildings: Vec<BuildingMass>,
}

#[derive(Debug, Clone, Copy)]
pub struct CitySpec {
    pub centre: Point,
    pub radius_km: f32,
    pub rotation_radians: f32,
    pub seed: u32,
    pub density: f32,
}

/// Parameters for the production Chinese-city generator.  Distances are
/// expressed in metres inside the generator and converted to the crate's
/// kilometre coordinate convention at the boundary.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModernChinaSpec {
    pub centre: Point,
    pub radius_km: f32,
    pub rotation_radians: f32,
    pub seed: u32,
    pub density: f32,
    pub block_size_metres: f32,
    pub organic: f32,
    pub river_width_metres: f32,
}

impl Default for ModernChinaSpec {
    fn default() -> Self {
        Self {
            centre: Point {
                x_km: 0.0,
                y_km: 0.0,
            },
            // Match the source city's default 1.8 km planning extent.  A
            // settlement is a neighbourhood-scale city layer; using a 4.8 km
            // radius here stamped an oversized grid across the terrain.
            radius_km: 0.9,
            rotation_radians: 0.0,
            seed: 42,
            density: 0.82,
            block_size_metres: 120.0,
            organic: 0.68,
            river_width_metres: 64.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModernRoadClass {
    Expressway,
    Arterial,
    Collector,
    Local,
}

impl ModernRoadClass {
    /// Ribbon width in metres, matching the CJJ 37 cross-sections encoded in
    /// `modern::roads::cross_section`: 快速路 36 m, 主干路 32 m, 次干路 22 m,
    /// 支路 12 m.
    pub const fn width_metres(self) -> f32 {
        match self {
            Self::Expressway => 36.0,
            Self::Arterial => 32.0,
            Self::Collector => 22.0,
            Self::Local => 12.0,
        }
    }

    /// Physical central median; divided streets are the Chinese norm for
    /// expressways and arterials, never for collectors or locals.
    pub const fn median_metres(self) -> f32 {
        match self {
            Self::Expressway => 2.5,
            Self::Arterial => 2.0,
            Self::Collector | Self::Local => 0.0,
        }
    }
}
