//! The numbers that make a road network look like one place and not another.

use std::sync::Arc;

use worldgen_contracts::{
    DryLand, FlatGround, HeightField, NoPinnedRoads, PinnedRoads, UrbanField, WaterField,
};

/// One rung of the street hierarchy. A rung exists where the place is built up
/// at least `min_urban`, and its streets lie `base_spacing_m` apart in the dense
/// core, up to three times that on the thin edge of a town.
#[derive(Debug, Clone, Copy)]
pub struct LevelSpec {
    pub level: u8,
    pub base_spacing_m: f64,
    pub min_urban: f64,
    /// A rung's streets are placed where the place, *or anywhere this near it*, is
    /// built up. The sides of a block can lie in the fields while its middle is
    /// already town; without this the town would stop at the first block whose
    /// sides are outside it, and have a rectangular edge.
    pub reach_m: f64,
}

/// From the arterial grid down to the alleys between buildings: 主干路, 次干路,
/// 支路, 小区路, 巷.
pub const LEVELS: [LevelSpec; 5] = [
    LevelSpec {
        level: 0,
        base_spacing_m: 78.0,
        min_urban: 0.7,
        reach_m: 50.0,
    },
    LevelSpec {
        level: 1,
        base_spacing_m: 150.0,
        min_urban: 0.5,
        reach_m: 110.0,
    },
    LevelSpec {
        level: 2,
        base_spacing_m: 290.0,
        min_urban: 0.3,
        reach_m: 240.0,
    },
    LevelSpec {
        level: 3,
        base_spacing_m: 520.0,
        min_urban: 0.15,
        reach_m: 500.0,
    },
    LevelSpec {
        level: 4,
        base_spacing_m: 1000.0,
        min_urban: 0.0,
        reach_m: 0.0,
    },
];

/// The coarsest rung: the lattice chords themselves, and the arterial grid.
pub const TOP_RUNG: u8 = LEVELS.len() as u8 - 1;

#[derive(Debug, Clone)]
pub struct RoadsConfig {
    /// Engine level whose cells are the lattice the road grid hangs from.
    pub lattice_level: u8,
    /// How far a lattice corner may stray from its cell corner, as a fraction of the cell.
    pub corner_jitter: f64,
    /// Low-frequency bending of the whole grid (metres, and the wavelength it varies over).
    pub warp_amp_m: f64,
    pub warp_scale_m: f64,
    /// Fine wiggle, so a long street is not a ruled line.
    pub wiggle_amp_m: f64,
    pub wiggle_scale_m: f64,
    /// A place built up at least this much has urban streets, below it country roads.
    pub urban_threshold: f64,
    /// How finely roads are sampled when bent, metres.
    pub sample_step_m: f64,
    /// Smallest class a tile carries: the level of detail.
    pub min_class: worldgen_contracts::RoadClass,
}

impl Default for RoadsConfig {
    fn default() -> Self {
        Self {
            lattice_level: 9,
            corner_jitter: 0.2,
            warp_amp_m: 55.0,
            warp_scale_m: 1400.0,
            wiggle_amp_m: 5.0,
            wiggle_scale_m: 230.0,
            urban_threshold: 0.3,
            sample_step_m: 20.0,
            min_class: worldgen_contracts::RoadClass::Track,
        }
    }
}

/// What the roads layer reads from the rest of the world.
#[derive(Clone)]
pub struct Fields {
    pub urban: Arc<dyn UrbanField>,
    pub water: Arc<dyn WaterField>,
    pub height: Arc<dyn HeightField>,
    /// Roads laid down by a planner; the layer generates the rest around them.
    pub pinned: Arc<dyn PinnedRoads>,
}

impl Fields {
    pub fn new(urban: Arc<dyn UrbanField>) -> Self {
        Self {
            urban,
            water: Arc::new(DryLand),
            height: Arc::new(FlatGround(0.0)),
            pinned: Arc::new(NoPinnedRoads),
        }
    }
    pub fn with_water(mut self, water: Arc<dyn WaterField>) -> Self {
        self.water = water;
        self
    }
    pub fn with_pinned(mut self, pinned: Arc<dyn PinnedRoads>) -> Self {
        self.pinned = pinned;
        self
    }
    pub fn with_height(mut self, height: Arc<dyn HeightField>) -> Self {
        self.height = height;
        self
    }
}

pub(crate) fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Spacing of a rung's streets at a place of the given urbanness; `None` where the
/// rung does not exist.
pub(crate) fn spacing(spec: &LevelSpec, urbanness: f64) -> Option<f64> {
    if urbanness < spec.min_urban {
        return None;
    }
    let k = smoothstep(spec.min_urban, 1.0, urbanness);
    Some(spec.base_spacing_m * (3.0 - 2.0 * k))
}
