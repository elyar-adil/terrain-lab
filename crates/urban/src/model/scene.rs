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
    /// Farmland around the town.
    #[serde(default)]
    pub fields: Vec<super::buildings::Field>,
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

/// The mean of a ring's corners: where a block, parcel or field is for clipping.
fn ring_centre(ring: &[Point]) -> Option<Point> {
    if ring.is_empty() {
        return None;
    }
    let n = ring.len() as f32;
    Some(Point {
        x_km: ring.iter().map(|p| p.x_km).sum::<f32>() / n,
        y_km: ring.iter().map(|p| p.y_km).sum::<f32>() / n,
    })
}

impl ModernCity {
    /// The part of the city within `radius_m` of a local point `(x, z)`: for looking at
    /// one neighbourhood of a large plan without building the whole of it. Roads are
    /// kept if any of their centreline is in range (nodes are all kept; unused ones
    /// are harmless); everything else is kept by where its centre lies.
    pub fn clipped(&self, centre: [f32; 2], radius_m: f32) -> ModernCity {
        let local = |p: &Point| self.frame.to_local(*p);
        let near = |p: &Point, extra: f32| {
            let [x, z] = local(p);
            (x - centre[0]).hypot(z - centre[1]) <= radius_m + extra
        };
        let middle = ring_centre;
        let mut out = self.clone();
        out.hd_roads
            .retain(|road| road.centreline.iter().any(|p| near(p, 60.0)));
        let kept: std::collections::HashSet<u32> =
            out.hd_roads.iter().map(|road| road.sd_road).collect();
        out.sd_roads.retain(|road| kept.contains(&road.id));
        out.blocks
            .retain(|b| middle(&b.boundary).is_some_and(|c| near(&c, 0.0)));
        out.parcels
            .retain(|b| middle(&b.ring).is_some_and(|c| near(&c, 0.0)));
        let parcels: std::collections::HashSet<u32> = out.parcels.iter().map(|p| p.id).collect();
        out.buildings.retain(|b| parcels.contains(&b.parcel_id));
        out.compounds.retain(|c| parcels.contains(&c.parcel_id));
        out.trees.retain(|t| near(&t.point, 0.0));
        out.fields
            .retain(|f| middle(&f.ring).is_some_and(|c| near(&c, 0.0)));
        // Waterways are cut to the neighbourhood too, or the water alone would be the
        // whole size of the plan.
        let cut = |line: &[Point]| -> Vec<Point> {
            line.iter().copied().filter(|p| near(p, 250.0)).collect()
        };
        out.river = out
            .river
            .as_ref()
            .map(|line| cut(line))
            .filter(|line| line.len() >= 2);
        for tributary in &mut out.tributaries {
            tributary.line = cut(&tributary.line);
        }
        out.tributaries.retain(|t| t.line.len() >= 2);
        out
    }

    /// The part of the city inside the square `[min, max]` (local metres).
    ///
    /// Every element is kept by where its centre lies: a block, parcel or field by
    /// its centroid, a building through its parcel, a road by its midpoint. The
    /// squares of a grid therefore partition the plan, and nothing is drawn in two
    /// tiles. Rivers are cut to the square, so each tile holds its own stretch.
    pub fn clipped_rect(&self, min: [f32; 2], max: [f32; 2]) -> ModernCity {
        let local = |p: &Point| self.frame.to_local(*p);
        let inside = |p: &Point| {
            let [x, z] = local(p);
            x >= min[0] && x < max[0] && z >= min[1] && z < max[1]
        };
        let mut out = self.clone();
        out.hd_roads.retain(|road| {
            road.centreline
                .get(road.centreline.len() / 2)
                .is_some_and(&inside)
        });
        let kept: std::collections::HashSet<u32> =
            out.hd_roads.iter().map(|road| road.sd_road).collect();
        out.sd_roads.retain(|road| kept.contains(&road.id));
        out.blocks
            .retain(|b| ring_centre(&b.boundary).is_some_and(|c| inside(&c)));
        out.parcels
            .retain(|b| ring_centre(&b.ring).is_some_and(|c| inside(&c)));
        let parcels: std::collections::HashSet<u32> = out.parcels.iter().map(|p| p.id).collect();
        out.buildings.retain(|b| parcels.contains(&b.parcel_id));
        out.compounds.retain(|c| parcels.contains(&c.parcel_id));
        out.trees.retain(|t| inside(&t.point));
        out.fields
            .retain(|f| ring_centre(&f.ring).is_some_and(|c| inside(&c)));
        let cut =
            |line: &[Point]| -> Vec<Point> { line.iter().copied().filter(|p| inside(p)).collect() };
        out.river = out
            .river
            .as_ref()
            .map(|line| cut(line))
            .filter(|line| line.len() >= 2);
        for tributary in &mut out.tributaries {
            tributary.line = cut(&tributary.line);
        }
        out.tributaries.retain(|t| t.line.len() >= 2);
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::{ModernChinaSpec, generate_modern_chinese_city};

    #[test]
    fn a_grid_of_squares_partitions_the_plan() {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            radius_km: 0.6,
            ..ModernChinaSpec::default()
        });
        let (mut buildings, mut roads, mut blocks) = (0, 0, 0);
        // A 4 by 4 grid of 500 m squares covers the whole 0.6 km plan.
        for i in -2..2 {
            for j in -2..2 {
                let min = [i as f32 * 500.0, j as f32 * 500.0];
                let max = [min[0] + 500.0, min[1] + 500.0];
                let tile = city.clipped_rect(min, max);
                buildings += tile.buildings.len();
                roads += tile.hd_roads.len();
                blocks += tile.blocks.len();
            }
        }
        assert_eq!(
            buildings,
            city.buildings.len(),
            "a building is missing or doubled"
        );
        assert_eq!(roads, city.hd_roads.len(), "a road is missing or doubled");
        assert_eq!(blocks, city.blocks.len(), "a block is missing or doubled");
    }
}
