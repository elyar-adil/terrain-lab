mod blocks;
mod geom;
mod graph;
mod landscape;
mod parcels;
mod roads;

use graph::{GraphOutput, build_graph, modern_hash, modern_phase};
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
    modern_hash(seed, a, b, 0x5f37_1d1b)
}

struct CityFrame {
    spec: ModernChinaSpec,
    radius_m: f32,
    block_m: f32,
    density: f32,
    organic: f32,
    river_half: f32,
}

impl CityFrame {
    fn new(spec: ModernChinaSpec) -> Self {
        Self {
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

    fn river_x(&self, z_m: f32) -> f32 {
        let radius_m = self.radius_m;
        let phase = modern_phase(self.spec.seed);
        radius_m
            * (0.075 * (z_m / radius_m * 2.7 + phase).sin()
                + 0.032 * (z_m / radius_m * 6.2 + phase * 0.4).sin())
    }
}

pub fn generate_modern_chinese_city(spec: ModernChinaSpec) -> ModernCity {
    let frame = CityFrame::new(spec);
    let GraphOutput {
        nodes,
        sd_roads,
        hd_roads,
        river,
        morphology_score,
    } = build_graph(&frame);
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
