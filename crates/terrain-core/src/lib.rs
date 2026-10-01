use image::{ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
};
use thiserror::Error;

pub mod evolution;
pub mod fluvial;
pub mod geology;
pub mod sites;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TerrainPreset {
    Arid,
    Temperate,
    Glacial,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Landform {
    #[default]
    MountainRange,
    Hills,
    Plains,
    Plateau,
    Coastal,
    Archipelago,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationConfig {
    pub seed: u32,
    pub preset: TerrainPreset,
    #[serde(default)]
    pub landform: Landform,
    pub grid_size: usize,
    pub world_size_km: f32,
    pub rainfall: f32,
    pub evaporation: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub sun_azimuth: f32,
    pub sun_elevation: f32,
    pub haze: f32,
    #[serde(default = "default_cloud_coverage")]
    pub cloud_coverage: f32,
    #[serde(default = "default_cloud_speed")]
    pub cloud_speed: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerrainStats {
    pub min_elevation: f32,
    pub max_elevation: f32,
    pub mean_elevation: f32,
    pub mean_slope: f32,
    pub water_coverage: f32,
    pub snow_coverage: f32,
    pub forest_coverage: f32,
}

#[derive(Debug, Error)]
pub enum TerrainError {
    #[error("grid size must be between 128 and 2048")]
    InvalidGridSize,
    #[error("world size must be between 5 and 500 km")]
    InvalidWorldSize,
    #[error("output size must be between 256 and 8192 pixels")]
    InvalidOutputSize,
    #[error("generated terrain contains invalid numeric values")]
    InvalidTerrain,
    #[error("image encoding failed: {0}")]
    Image(#[from] image::ImageError),
    #[error("file operation failed: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Clone)]
pub struct TerrainData {
    pub size: usize,
    pub height: Vec<f32>,
    pub moisture: Vec<f32>,
    pub temperature: Vec<f32>,
    pub flow: Vec<f32>,
    pub flow_direction_x: Vec<f32>,
    pub flow_direction_y: Vec<f32>,
    pub filled_height: Vec<f32>,
    pub vegetation: Vec<f32>,
    pub geology: Vec<f32>,
    pub lithology: Vec<geology::Lithology>,
    pub bedding_strike: Vec<f32>,
    pub bedding_dip: Vec<f32>,
    pub fold_phase: Vec<f32>,
    pub stratigraphic_phase: Vec<f32>,
    pub fracture_intensity: Vec<f32>,
    pub weathering_potential: Vec<f32>,
    pub erosion_resistance: Vec<f32>,
    pub soil_depth: Vec<f32>,
    pub forest: Vec<f32>,
    pub grassland: Vec<f32>,
    pub shrubland: Vec<f32>,
    pub bare_ground: Vec<f32>,
    pub sediment: Vec<f32>,
    pub snow: Vec<f32>,
    pub water: Vec<f32>,
    pub lake: Vec<f32>,
    pub wetland: Vec<f32>,
    pub floodplain: Vec<f32>,
    pub river_order: Vec<u8>,
    pub basin_id: Vec<u32>,
    pub stats: TerrainStats,
}

#[derive(Clone, Copy)]
struct HeapCell {
    elevation: f32,
    index: usize,
}

impl PartialEq for HeapCell {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.elevation.to_bits() == other.elevation.to_bits()
    }
}

impl Eq for HeapCell {}

impl PartialOrd for HeapCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapCell {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .elevation
            .total_cmp(&self.elevation)
            .then_with(|| other.index.cmp(&self.index))
    }
}

#[derive(Clone, Copy)]
struct PresetParameters {
    relief: f32,
    ridge_power: f32,
    roughness: f32,
    tree_line: f32,
    vegetation_gain: f32,
    snow_gain: f32,
    erosion: f32,
    base_tint: [f32; 3],
}

impl TerrainPreset {
    fn parameters(self) -> PresetParameters {
        match self {
            Self::Arid => PresetParameters {
                relief: 1.15,
                ridge_power: 3.7,
                roughness: 1.2,
                tree_line: 0.33,
                vegetation_gain: 0.55,
                snow_gain: 0.55,
                erosion: 0.82,
                base_tint: [0.53, 0.43, 0.29],
            },
            Self::Temperate => PresetParameters {
                relief: 0.95,
                ridge_power: 3.15,
                roughness: 0.92,
                tree_line: 0.24,
                vegetation_gain: 1.28,
                snow_gain: 0.5,
                erosion: 1.22,
                base_tint: [0.34, 0.39, 0.25],
            },
            Self::Glacial => PresetParameters {
                relief: 1.28,
                ridge_power: 3.45,
                roughness: 1.08,
                tree_line: 0.2,
                vegetation_gain: 0.48,
                snow_gain: 1.5,
                erosion: 0.92,
                base_tint: [0.42, 0.43, 0.39],
            },
        }
    }
}

static FLUVIAL_EROSION: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

/// Switch the fluvial landscape evolution pass on or off (on by default). Probes
/// and tests use it to compare a landscape with and without it.
pub fn set_fluvial_erosion(enabled: bool) {
    FLUVIAL_EROSION.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

pub fn generate<F>(config: &SimulationConfig, mut progress: F) -> Result<TerrainData, TerrainError>
where
    F: FnMut(f32, &str),
{
    validate_config(config)?;
    let n = config.grid_size;
    let len = n * n;
    let preset = config.preset.parameters();
    let mut geology_model = geology::generate_geology(geology::GeologyConfig {
        size: n,
        world_size_km: config.world_size_km,
        seed: config.seed,
    })
    .map_err(|_| TerrainError::InvalidTerrain)?;
    progress(0.04, "构造区域地貌骨架");

    let mut height = vec![0.0_f32; len];
    height
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            let x = index % n;
            let y = index / n;
            let nx = x as f32 / (n - 1) as f32;
            let ny = y as f32 / (n - 1) as f32;
            let warp_x = fbm(nx * 2.0 + 13.7, ny * 2.0 - 8.1, config.seed ^ 0x91e1, 5) * 0.095;
            let warp_y = fbm(nx * 2.0 - 4.2, ny * 2.0 + 17.3, config.seed ^ 0x7ab3, 5) * 0.095;
            let px = nx + warp_x;
            let py = ny + warp_y;
            let belt_axis =
                0.48 + fbm(px * 1.15 + 9.0, py * 0.72 - 5.0, config.seed ^ 0xac71, 5) * 0.12;
            let belt_distance = (py - belt_axis).abs();
            let belt = 1.0 - smoothstep(0.10, 0.38, belt_distance);
            let continental = fbm(px * 1.35 + 27.0, py * 1.35 - 19.0, config.seed ^ 0x41c6, 6);
            let ridge_noise =
                1.0 - fbm(px * 3.7 + 17.0, py * 3.7 - 11.0, config.seed ^ 0xda21, 6).abs();
            let ridge = clamp01(ridge_noise).powf(preset.ridge_power);
            let secondary =
                clamp01(1.0 - fbm(px * 8.4 - 3.0, py * 8.4 + 12.0, config.seed ^ 0x2f41, 5).abs())
                    .powf(3.6);
            let spurs = clamp01(
                1.0 - fbm(px * 14.0 + 21.0, py * 7.0 - 13.0, config.seed ^ 0xb991, 4).abs(),
            )
            .powf(4.1)
                * belt;
            let strata = fbm(px * 24.0 + 31.0, py * 24.0 - 19.0, config.seed ^ 0x16ef, 4) * 115.0;
            let hills = fbm(px * 72.0 - 17.0, py * 72.0 + 29.0, config.seed ^ 0x84dd, 3) * 24.0;
            let broad = fbm(px * 5.2 - 41.0, py * 5.2 + 33.0, config.seed ^ 0x619d, 5);
            let coast_field = continental * 0.42
                + fbm(px * 2.35 - 11.0, py * 2.35 + 7.0, config.seed ^ 0x4b29, 5) * 0.22
                + (0.47 - nx) * 1.12;
            let island_distance = ((nx - 0.5).powi(2) + (ny - 0.5).powi(2)).sqrt();
            let island_field = continental * 0.58
                + fbm(px * 3.4 + 4.0, py * 3.4 - 12.0, config.seed ^ 0xf2a7, 5) * 0.38
                - island_distance * 0.48;
            let meander_axis = 0.50
                + fbm(nx * 1.65 + 7.0, 0.4, config.seed ^ 0x93d1, 6) * 0.19
                + fbm(nx * 5.4 - 3.0, 1.7, config.seed ^ 0x51a9, 4) * 0.045;
            let meander_distance = (ny - meander_axis).abs();
            let flood_basin = (-((meander_distance / 0.075).powi(2))).exp();
            let main_channel = (-((meander_distance / 0.0085).powi(2))).exp();
            let elevation = match config.landform {
                Landform::MountainRange => {
                    85.0 + continental * 310.0
                        + belt * (520.0 + ridge * 2650.0 + secondary * 720.0 + spurs * 360.0)
                        + (strata + hills) * belt
                }
                Landform::Hills => {
                    105.0
                        + continental * 145.0
                        + broad * 95.0
                        + belt * (90.0 + ridge * 620.0 + secondary * 230.0 + spurs * 95.0)
                        + strata * 0.28
                        + hills * 1.55
                }
                Landform::Plains => {
                    78.0 + continental * 52.0
                        + broad * 36.0
                        + ridge * belt * 72.0
                        + strata * 0.08
                        + hills * 0.42
                        + (1.0 - nx) * 34.0
                        - flood_basin * 24.0
                        - main_channel * 12.0
                }
                Landform::Plateau => {
                    let warped_distance = ((px - 0.5).powi(2) + ((py - 0.5) * 1.08).powi(2)).sqrt();
                    let rim_noise = broad * 0.055 + continental * 0.035;
                    let table =
                        1.0 - smoothstep(0.28 + rim_noise, 0.47 + rim_noise, warped_distance);
                    let escarpment =
                        smoothstep(0.08, 0.38, table) * (1.0 - smoothstep(0.62, 0.94, table));
                    145.0
                        + table * (1120.0 + continental * 210.0 + broad * 85.0)
                        + escarpment * (ridge * 420.0 + secondary * 150.0)
                        + strata * 0.22
                        + hills * 0.65
                }
                Landform::Coastal => {
                    let land = smoothstep(-0.08, 0.12, coast_field);
                    16.0 + land
                        * (70.0
                            + clamp01(coast_field + 0.12).powf(0.72) * 620.0
                            + belt * ridge * 760.0
                            + secondary * 120.0
                            + hills * 0.72)
                }
                Landform::Archipelago => {
                    let land = smoothstep(-0.04, 0.18, island_field);
                    12.0 + land
                        * (58.0
                            + clamp01(island_field + 0.18).powf(0.68) * 780.0
                            + ridge * 260.0
                            + secondary * 85.0
                            + hills * 0.45)
                }
            };
            let margin = x.min(y).min(n - 1 - x).min(n - 1 - y) as f32 / n as f32;
            let edge_width = match config.landform {
                Landform::Coastal | Landform::Archipelago => 0.002,
                _ => 0.010,
            };
            let edge = smoothstep(0.0, edge_width, margin);
            let climate_relief = match config.landform {
                Landform::MountainRange => preset.relief,
                _ => 0.82 + preset.relief * 0.18,
            };
            *value = (elevation * climate_relief * edge + 24.0 * (1.0 - edge)).max(0.0);
        });
    geology_model.expose_surface(&height);

    let fluvial_enabled = FLUVIAL_EROSION.load(std::sync::atomic::Ordering::Relaxed);
    if fluvial_enabled {
        // The formulas above only plan the large-scale shape; rivers carve the rest.
        progress(0.10, "河流侵蚀塑造山谷与山脊");
        let cell_metres = config.world_size_km * 1000.0 / n as f32;
        fluvial::evolve(
            &mut height,
            &geology_model.erosion_resistance,
            n,
            cell_metres,
            config.seed,
            &mut |fraction| progress(0.10 + 0.10 * fraction, "河流侵蚀塑造山谷与山脊"),
        );
        geology_model.expose_surface(&height);
    }

    progress(0.20, "计算迎风降水、温度与地表湿度");
    let mut moisture = vec![0.0_f32; len];
    let mut temperature = vec![0.0_f32; len];
    let wind_angle = config.wind_direction.to_radians();
    let wind_x = wind_angle.cos();
    let wind_y = wind_angle.sin();
    let rainfall_balance =
        ((config.rainfall - config.evaporation * 0.55) / 1200.0).clamp(-0.55, 1.2);
    moisture
        .par_iter_mut()
        .zip(temperature.par_iter_mut())
        .enumerate()
        .for_each(|(index, (wet, temp))| {
            let x = index % n;
            let y = index / n;
            let h = height[index];
            let up_x = (x as isize - wind_x.signum() as isize).clamp(0, n as isize - 1) as usize;
            let up_y = (y as isize - wind_y.signum() as isize).clamp(0, n as isize - 1) as usize;
            let uplift = (h - height[up_y * n + up_x]).max(0.0) / 180.0;
            let regional = fbm(
                x as f32 / n as f32 * 3.2 + 90.0,
                y as f32 / n as f32 * 3.2 - 40.0,
                config.seed ^ 0xc184,
                5,
            );
            *wet = clamp01(
                0.42 + rainfall_balance * 0.42 + regional * 0.25 + uplift * 0.34 - h / 9500.0,
            );
            let latitude = (y as f32 / (n - 1) as f32 - 0.5).abs();
            *temp = 21.0 - latitude * 10.0 - h * 0.0062;
        });

    let mut receiver = vec![usize::MAX; len];
    let mut receiver_secondary = vec![usize::MAX; len];
    let mut receiver_weight = vec![1.0_f32; len];
    let mut filled_height = height.clone();
    let mut flow = vec![0.0_f32; len];
    for iteration in 0..4 {
        progress(
            0.30 + iteration as f32 * 0.10,
            if iteration == 0 {
                "推演河网与汇流结构"
            } else {
                "侵蚀河谷并稳定水系"
            },
        );
        priority_flood(&height, n, &mut filled_height);
        compute_routing(
            &filled_height,
            n,
            &mut receiver,
            &mut receiver_secondary,
            &mut receiver_weight,
        );
        accumulate_flow(
            &filled_height,
            &moisture,
            &receiver,
            &receiver_secondary,
            &receiver_weight,
            &mut flow,
        );
        if iteration < 3 {
            geology_model.expose_surface(&height);
            if !fluvial_enabled {
                erode_channels(
                    &mut height,
                    &flow,
                    &receiver,
                    &geology_model.erosion_resistance,
                    n,
                    preset.erosion,
                );
            }
            breach_overflowing_spillways(&mut height, &filled_height, &flow, &receiver, n);
            diffuse_slopes(&mut height, n, 0.06 + preset.roughness * 0.015);
        }
    }
    geology_model.expose_surface(&height);

    progress(0.72, "生成植被、积雪与水体覆盖");
    let max_flow = flow.par_iter().copied().reduce(|| 0.0, f32::max).max(1.0);
    let mut vegetation = vec![0.0_f32; len];
    let mut snow = vec![0.0_f32; len];
    let mut water = vec![0.0_f32; len];
    let mut lake = vec![0.0_f32; len];
    let mut floodplain = vec![0.0_f32; len];
    let mut wetland = vec![0.0_f32; len];
    let mut river_order = vec![0_u8; len];
    lake.par_iter_mut().enumerate().for_each(|(index, value)| {
        let fill_depth = filled_height[index] - height[index];
        let climate_persistence = clamp01((moisture[index] - 0.38) / 0.42);
        // Elevation is not a lake-deletion criterion. Tarns, glacial lakes
        // and structural basins may persist at altitude; actively overflowing
        // basins are instead opened by the spillway evolution pass.
        *value = clamp01((fill_depth - 4.0) / 30.0) * climate_persistence;
    });
    floodplain
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            let x = index % n;
            let y = index / n;
            let slope = local_slope(&height, n, x, y);
            let flow_signal = clamp01(
                (flow[index].ln_1p() - max_flow.ln_1p() * 0.43) / (max_flow.ln_1p() * 0.34),
            );
            *value = flow_signal * clamp01(1.0 - slope * 16.0);
        });
    wetland
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            *value = clamp01(
                floodplain[index] * moisture[index] * 1.25
                    + lake[index] * (1.0 - lake[index]) * 0.7,
            );
        });
    compute_river_order(&filled_height, &flow, &receiver, max_flow, &mut river_order);
    let basin_id = compute_basins(&receiver, n);
    let mut flow_direction_x = vec![0.0_f32; len];
    let mut flow_direction_y = vec![0.0_f32; len];
    flow_direction_x
        .par_iter_mut()
        .zip(flow_direction_y.par_iter_mut())
        .enumerate()
        .for_each(|(index, (dx, dy))| {
            let target = receiver[index];
            if target == usize::MAX {
                return;
            }
            let offset_x = target as isize % n as isize - index as isize % n as isize;
            let offset_y = target as isize / n as isize - index as isize / n as isize;
            let length = ((offset_x * offset_x + offset_y * offset_y) as f32)
                .sqrt()
                .max(1.0);
            *dx = offset_x as f32 / length;
            *dy = offset_y as f32 / length;
        });
    vegetation
        .par_iter_mut()
        .zip(snow.par_iter_mut())
        .zip(water.par_iter_mut())
        .enumerate()
        .for_each(|(index, ((veg, snow_value), water_value))| {
            let x = index % n;
            let y = index / n;
            let slope = local_slope(&height, n, x, y);
            let river = clamp01(
                (flow[index].ln_1p() - max_flow.ln_1p() * 0.78) / (max_flow.ln_1p() * 0.14),
            );
            let river_moisture = river * 0.3;
            let texture = fbm(x as f32 * 0.028, y as f32 * 0.028, config.seed ^ 0xf231, 4) * 0.12;
            *veg = clamp01(
                (moisture[index] + river_moisture - preset.tree_line - slope * 0.72 + texture)
                    * preset.vegetation_gain
                    * 1.65,
            );
            // Snow lies where the climate allows it, but it slides off steep
            // ground, leaving bare rock on ridge flanks and snow on benches and
            // in gullies. A uniform white mountain is a cone drawn with a rule.
            let shedding = 1.0 - smoothstep(0.30, 0.80, slope);
            *snow_value = clamp01(
                ((-temperature[index] + 1.5) / 10.0 * preset.snow_gain + height[index] / 9000.0)
                    * (0.12 + 0.88 * shedding),
            );
            let boundary_water = if height[index] < 58.0 {
                clamp01((58.0 - height[index]) / 30.0)
            } else {
                0.0
            };
            *water_value = river.max(boundary_water).max(lake[index]);
        });

    progress(0.80, "生成岩性、土壤与地表覆盖群落");
    let min_height = height
        .par_iter()
        .copied()
        .reduce(|| f32::INFINITY, f32::min);
    let max_height = height
        .par_iter()
        .copied()
        .reduce(|| f32::NEG_INFINITY, f32::max);
    let height_range = (max_height - min_height).max(1.0);
    let cell_metres = config.world_size_km * 1000.0 / (n - 1) as f32;
    let mut geology = vec![0.0_f32; len];
    let mut soil_depth = vec![0.0_f32; len];
    let mut forest = vec![0.0_f32; len];
    let mut grassland = vec![0.0_f32; len];
    let mut shrubland = vec![0.0_f32; len];
    let mut bare_ground = vec![0.0_f32; len];
    let mut sediment = vec![0.0_f32; len];
    geology
        .par_iter_mut()
        .zip(soil_depth.par_iter_mut())
        .zip(forest.par_iter_mut())
        .zip(grassland.par_iter_mut())
        .zip(shrubland.par_iter_mut())
        .zip(bare_ground.par_iter_mut())
        .zip(sediment.par_iter_mut())
        .enumerate()
        .for_each(
            |(index, ((((((geology_value, soil), trees), grass), shrubs), bare), deposit))| {
                let x = index % n;
                let y = index / n;
                let nx = x as f32 / (n - 1) as f32;
                let ny = y as f32 / (n - 1) as f32;
                let slope = physical_slope(&height, n, x, y, cell_metres);
                let elevation = clamp01((height[index] - min_height) / height_range);
                let resistance = geology_model.erosion_resistance[index];
                let weathering = geology_model.weathering_potential[index];
                let fracture = geology_model.fracture_intensity[index];
                let strata = geology_model.fold_phase[index].sin();
                *geology_value = clamp01((resistance - 0.34) / (2.35 - 0.34));

                let land = 1.0 - clamp01((58.0 - height[index]) / 18.0);
                let stable_surface = clamp01(1.0 - slope * 3.8);
                let exposed_rock = clamp01(
                    (slope - 0.16) * 3.0 + elevation * 0.28 + strata.abs() * 0.12 + fracture * 0.18
                        - floodplain[index] * 0.72,
                ) * land;
                *soil = clamp01(
                    stable_surface * (0.30 + moisture[index] * 0.30 + weathering * 0.18)
                        + floodplain[index] * 0.72
                        + wetland[index] * 0.25
                        - exposed_rock * 0.82,
                ) * land;

                let warm_enough = smoothstep(-8.0, 12.0, temperature[index]);
                let not_too_hot = 1.0 - smoothstep(27.0, 38.0, temperature[index]);
                // Forests are regional communities, rather than independent noisy cells.  A
                // broad, domain-warped field gives them coherent watersheds and recognisable
                // margins while the second field breaks up otherwise smooth boundaries.
                let forest_warp_x =
                    fbm(nx * 2.1 + 4.0, ny * 2.1 - 7.0, config.seed ^ 0x8e31, 4) * 0.10;
                let forest_warp_y =
                    fbm(nx * 2.1 - 9.0, ny * 2.1 + 3.0, config.seed ^ 0x52bd, 4) * 0.10;
                let forest_region = fbm(
                    (nx + forest_warp_x) * 4.2 + 19.0,
                    (ny + forest_warp_y) * 4.2 - 31.0,
                    config.seed ^ 0xa137,
                    5,
                );
                let forest_edge = fbm(nx * 13.0 - 17.0, ny * 13.0 + 23.0, config.seed ^ 0x6d47, 4);
                let grass_patch = clamp01(
                    0.62 + fbm(nx * 12.0 - 7.0, ny * 12.0 + 18.0, config.seed ^ 0xd283, 4) * 0.58,
                );
                let habitable = land * (1.0 - water[index]) * (1.0 - snow[index] * 0.88);
                let biome_bias = match config.preset {
                    TerrainPreset::Temperate => 0.10,
                    TerrainPreset::Arid => -0.19,
                    TerrainPreset::Glacial => -0.10,
                };
                let forest_score = moisture[index]
                    + forest_region * 0.27
                    + forest_edge * 0.070
                    + *soil * 0.10
                    + biome_bias
                    - slope * 0.62
                    - wetland[index] * 0.12;
                let forest_footprint = smoothstep(0.76, 1.04, forest_score);
                let canopy_density = clamp01(
                    0.52 + moisture[index] * 0.43 + forest_edge * 0.09 - exposed_rock * 0.62,
                );
                *trees = clamp01(
                    forest_footprint
                        * canopy_density
                        * warm_enough
                        * not_too_hot
                        * (0.72 + stable_surface * 0.28)
                        * (0.72 + preset.vegetation_gain * 0.22),
                ) * habitable;
                *grass = clamp01(
                    (1.0 - *trees * 0.78)
                        * smoothstep(0.20, 0.54, moisture[index])
                        * (1.0 - smoothstep(0.82, 1.0, moisture[index]))
                        * warm_enough
                        * grass_patch
                        * (0.50 + stable_surface * 0.50),
                ) * habitable;
                *shrubs = clamp01(
                    (1.0 - *trees)
                        * (1.0 - *grass * 0.62)
                        * smoothstep(0.08, 0.34, moisture[index])
                        * (1.0 - smoothstep(0.58, 0.82, moisture[index]))
                        * smoothstep(-2.0, 18.0, temperature[index]),
                ) * habitable;
                *deposit = clamp01(
                    floodplain[index] * (0.48 + flow[index].ln_1p() / max_flow.ln_1p() * 0.52)
                        + wetland[index] * 0.22
                        + clamp01(1.0 - (height[index] - 58.0).abs() / 12.0)
                            * stable_surface
                            * 0.52,
                ) * land;
                *bare = clamp01(
                    exposed_rock.max(
                        (1.0 - *trees - *grass * 0.82 - *shrubs * 0.72)
                            * (0.48 + (1.0 - *soil) * 0.52),
                    ) + snow[index] * 0.12,
                ) * habitable;
            },
        );
    vegetation
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            *value = clamp01(forest[index] + grassland[index] * 0.72 + shrubland[index] * 0.58);
        });

    if height.iter().any(|value| !value.is_finite()) {
        return Err(TerrainError::InvalidTerrain);
    }

    progress(0.86, "计算地貌质量与覆盖统计");
    let stats = calculate_stats(n, &height, &forest, &snow, &water, config.world_size_km);
    progress(0.90, "地貌模拟完成");
    Ok(TerrainData {
        size: n,
        height,
        moisture,
        temperature,
        flow,
        flow_direction_x,
        flow_direction_y,
        filled_height,
        vegetation,
        geology,
        lithology: geology_model.lithology,
        bedding_strike: geology_model.bedding_strike,
        bedding_dip: geology_model.bedding_dip,
        fold_phase: geology_model.fold_phase,
        stratigraphic_phase: geology_model.stratigraphic_phase,
        fracture_intensity: geology_model.fracture_intensity,
        weathering_potential: geology_model.weathering_potential,
        erosion_resistance: geology_model.erosion_resistance,
        soil_depth,
        forest,
        grassland,
        shrubland,
        bare_ground,
        sediment,
        snow,
        water,
        lake,
        wetland,
        floodplain,
        river_order,
        basin_id,
        stats,
    })
}

pub fn render_satellite<F>(
    terrain: &TerrainData,
    config: &SimulationConfig,
    output_size: usize,
    mut progress: F,
) -> Result<RgbaImage, TerrainError>
where
    F: FnMut(f32, &str),
{
    if !(256..=8192).contains(&output_size) {
        return Err(TerrainError::InvalidOutputSize);
    }
    progress(0.91, "合成多尺度地表材质");
    let pixels = output_size
        .checked_mul(output_size)
        .and_then(|v| v.checked_mul(4))
        .ok_or(TerrainError::InvalidOutputSize)?;
    let mut data = vec![0_u8; pixels];
    let max_flow = terrain
        .flow
        .par_iter()
        .copied()
        .reduce(|| 0.0, f32::max)
        .max(1.0);
    let min_height = terrain.stats.min_elevation;
    let height_range = (terrain.stats.max_elevation - min_height).max(1.0);
    let parameters = config.preset.parameters();
    let sun_azimuth = config.sun_azimuth.to_radians();
    let sun_elevation = config.sun_elevation.to_radians();
    let sun = [
        sun_azimuth.sin() * sun_elevation.cos(),
        sun_azimuth.cos() * sun_elevation.cos(),
        sun_elevation.sin(),
    ];
    let source_size = terrain.size;
    let cell_metres = config.world_size_km * 1000.0 / source_size as f32;

    data.par_chunks_mut(4)
        .enumerate()
        .for_each(|(index, pixel)| {
            let x = index % output_size;
            let y = index / output_size;
            let sx = x as f32 / (output_size - 1) as f32 * (source_size - 1) as f32;
            let sy = y as f32 / (output_size - 1) as f32 * (source_size - 1) as f32;
            let h = bilinear(&terrain.height, source_size, sx, sy);
            let wet = bilinear(&terrain.moisture, source_size, sx, sy);
            let temp = bilinear(&terrain.temperature, source_size, sx, sy);
            let veg = bilinear(&terrain.vegetation, source_size, sx, sy);
            let geology = bilinear(&terrain.geology, source_size, sx, sy);
            let fold_phase = bilinear(&terrain.fold_phase, source_size, sx, sy);
            let stratigraphic_phase = bilinear(&terrain.stratigraphic_phase, source_size, sx, sy);
            let bedding_dip = bilinear(&terrain.bedding_dip, source_size, sx, sy);
            let fracture = bilinear(&terrain.fracture_intensity, source_size, sx, sy);
            let weathering = bilinear(&terrain.weathering_potential, source_size, sx, sy);
            let source_x = sx.round().clamp(0.0, (source_size - 1) as f32) as usize;
            let source_y = sy.round().clamp(0.0, (source_size - 1) as f32) as usize;
            let lithology = terrain.lithology[source_y * source_size + source_x];
            let soil_depth = bilinear(&terrain.soil_depth, source_size, sx, sy);
            let forest_cover = bilinear(&terrain.forest, source_size, sx, sy);
            let grass_cover = bilinear(&terrain.grassland, source_size, sx, sy);
            let shrub_cover = bilinear(&terrain.shrubland, source_size, sx, sy);
            let bare_cover = bilinear(&terrain.bare_ground, source_size, sx, sy);
            let sediment_cover = bilinear(&terrain.sediment, source_size, sx, sy);
            let snow = bilinear(&terrain.snow, source_size, sx, sy);
            let water = bilinear(&terrain.water, source_size, sx, sy);
            let semantic_floodplain = bilinear(&terrain.floodplain, source_size, sx, sy);
            let wetland = bilinear(&terrain.wetland, source_size, sx, sy);
            let river_warp_x = fbm(sx * 0.025, sy * 0.025, config.seed ^ 0x3c91, 3) * 1.8;
            let river_warp_y = fbm(sx * 0.025, sy * 0.025, config.seed ^ 0x8d47, 3) * 1.8;
            let warped_sx = (sx + river_warp_x).clamp(0.0, (source_size - 1) as f32);
            let warped_sy = (sy + river_warp_y).clamp(0.0, (source_size - 1) as f32);
            let flow = bilinear(&terrain.flow, source_size, warped_sx, warped_sy);
            let flood_radius = 3.2;
            let nearby_flow = [
                flow,
                bilinear(
                    &terrain.flow,
                    source_size,
                    (warped_sx - flood_radius).max(0.0),
                    warped_sy,
                ),
                bilinear(
                    &terrain.flow,
                    source_size,
                    (warped_sx + flood_radius).min((source_size - 1) as f32),
                    warped_sy,
                ),
                bilinear(
                    &terrain.flow,
                    source_size,
                    warped_sx,
                    (warped_sy - flood_radius).max(0.0),
                ),
                bilinear(
                    &terrain.flow,
                    source_size,
                    warped_sx,
                    (warped_sy + flood_radius).min((source_size - 1) as f32),
                ),
            ]
            .into_iter()
            .fold(0.0_f32, f32::max);
            let px_world = x as f32 / output_size as f32 * config.world_size_km * 1000.0;
            let py_world = y as f32 / output_size as f32 * config.world_size_km * 1000.0;
            let detail = fbm(px_world / 680.0, py_world / 680.0, config.seed ^ 0x72e1, 5);
            let fine = fbm(px_world / 95.0, py_world / 95.0, config.seed ^ 0x5ac3, 3);
            let canopy_region = fbm(
                px_world / 4200.0 + 11.0,
                py_world / 4200.0 - 7.0,
                config.seed ^ 0x4b17,
                4,
            );
            let canopy_texture = fbm(
                px_world / 72.0 - 13.0,
                py_world / 72.0 + 19.0,
                config.seed ^ 0xc953,
                3,
            );
            let sample_step = (source_size as f32 / output_size as f32).max(0.75);
            let hx0 = bilinear(
                &terrain.height,
                source_size,
                (sx - sample_step).max(0.0),
                sy,
            );
            let hx1 = bilinear(
                &terrain.height,
                source_size,
                (sx + sample_step).min((source_size - 1) as f32),
                sy,
            );
            let hy0 = bilinear(
                &terrain.height,
                source_size,
                sx,
                (sy - sample_step).max(0.0),
            );
            let hy1 = bilinear(
                &terrain.height,
                source_size,
                sx,
                (sy + sample_step).min((source_size - 1) as f32),
            );
            let dzdx = (hx1 - hx0) / (cell_metres * sample_step * 2.0);
            let dzdy = (hy1 - hy0) / (cell_metres * sample_step * 2.0);
            let normal_length = (dzdx * dzdx + dzdy * dzdy + 1.0).sqrt();
            let normal = [
                -dzdx / normal_length,
                -dzdy / normal_length,
                1.0 / normal_length,
            ];
            let direct = (normal[0] * sun[0] + normal[1] * sun[1] + normal[2] * sun[2]).max(0.0);
            let hillshade = 0.34 + direct * 0.66;
            let elev = clamp01((h - min_height) / height_range);
            let slope = (dzdx * dzdx + dzdy * dzdy).sqrt();
            let rock =
                clamp01(((slope - 0.16) * 2.7 + elev * 0.22 - veg * 0.20).max(bare_cover * 0.72));
            let dry = clamp01(1.0 - wet);
            let open_water = clamp01((58.0 - h) / 18.0);
            let river_terrain_factor = match config.landform {
                Landform::Plains => 0.72,
                Landform::Coastal | Landform::Archipelago => 0.48,
                Landform::Hills => clamp01(slope * 10.0 + 0.16),
                _ => clamp01(slope * 14.0 + 0.08),
            };
            let river =
                clamp01((flow.ln_1p() - max_flow.ln_1p() * 0.70) / (max_flow.ln_1p() * 0.22))
                    * river_terrain_factor
                    * (1.0 - open_water);
            let floodplain = clamp01(
                (nearby_flow.ln_1p() - max_flow.ln_1p() * 0.48) / (max_flow.ln_1p() * 0.30),
            ) * clamp01(1.0 - slope * 22.0)
                * (0.42 + wet * 0.58);
            let lithology_shift = geology - 0.5;
            let soil = [
                parameters.base_tint[0]
                    + lithology_shift * 0.038
                    + dry * 0.07
                    + detail * 0.048
                    + fine * 0.016,
                parameters.base_tint[1]
                    + lithology_shift * 0.020
                    + soil_depth * 0.035
                    + detail * 0.038
                    + fine * 0.012,
                parameters.base_tint[2] - lithology_shift * 0.012
                    + soil_depth * 0.018
                    + detail * 0.024,
            ];
            let cold_forest = clamp01((12.0 - temp) / 18.0 + elev * 0.24);
            let deciduous = [
                0.125 + wet * 0.025 + canopy_region * 0.018,
                0.205 + wet * 0.060 + canopy_region * 0.028,
                0.095 + wet * 0.024,
            ];
            let conifer = [
                0.085 + geology * 0.012,
                0.155 + wet * 0.040,
                0.095 + wet * 0.022,
            ];
            let mut forest = mix3(deciduous, conifer, cold_forest);
            // At satellite scale individual crowns read mainly as mottled luminance and a
            // slightly darker north-facing inter-crown shadow, not bright green noise.
            let crown_luminance = 0.96 + canopy_texture * 0.105 + fine * 0.025;
            for channel in &mut forest {
                *channel *= crown_luminance;
            }
            let grass = [
                0.245 + dry * 0.17 + detail * 0.035,
                0.335 + wet * 0.14 + fine * 0.018,
                0.145 + dry * 0.075,
            ];
            let shrub = [
                0.315 + dry * 0.13 + detail * 0.04,
                0.335 + wet * 0.07 + fine * 0.014,
                0.185 + dry * 0.055,
            ];
            let lithology_color = match lithology {
                geology::Lithology::Shale => [0.315, 0.305, 0.285],
                geology::Lithology::Sandstone => [0.455, 0.395, 0.315],
                geology::Lithology::Limestone => [0.485, 0.475, 0.425],
                geology::Lithology::Granite => [0.435, 0.415, 0.395],
                geology::Lithology::Basalt => [0.255, 0.270, 0.275],
                geology::Lithology::Metamorphic => [0.355, 0.345, 0.335],
            };
            // This is the actual intersection of a finite-thickness 3D bed
            // with the topographic surface. Only the contact receives a small
            // weathering shadow; no sinusoidal colour bands are painted over
            // the mountain. `fold_phase` contributes structural microvariation
            // but cannot invent additional beds.
            let within_bed = stratigraphic_phase.rem_euclid(1.0);
            let contact_distance = within_bed.min(1.0 - within_bed);
            let phase_dx = (bilinear(
                &terrain.stratigraphic_phase,
                source_size,
                (sx + sample_step).min((source_size - 1) as f32),
                sy,
            ) - bilinear(
                &terrain.stratigraphic_phase,
                source_size,
                (sx - sample_step).max(0.0),
                sy,
            ))
            .abs()
                * 0.5;
            let phase_dy = (bilinear(
                &terrain.stratigraphic_phase,
                source_size,
                sx,
                (sy + sample_step).min((source_size - 1) as f32),
            ) - bilinear(
                &terrain.stratigraphic_phase,
                source_size,
                sx,
                (sy - sample_step).max(0.0),
            ))
            .abs()
                * 0.5;
            let resolvable_contact = 1.0 - smoothstep(0.22, 0.62, phase_dx.max(phase_dy));
            let bedding_contact =
                (1.0 - smoothstep(0.0, 0.075, contact_distance)) * resolvable_contact;
            let bedding_tone = 1.0 - bedding_contact * (0.055 + bedding_dip * 0.025)
                + fine * 0.012
                + fold_phase.sin() * 0.004;
            let rock_color = [
                (lithology_color[0] + geology * 0.012 + weathering * 0.010) * bedding_tone,
                (lithology_color[1] + weathering * 0.016) * bedding_tone,
                (lithology_color[2] + (1.0 - geology) * 0.008) * bedding_tone,
            ];
            let sediment_color = if matches!(config.preset, TerrainPreset::Arid) {
                [0.54, 0.46, 0.31]
            } else {
                [0.39, 0.355, 0.245]
            };
            let mut color = soil;
            color = mix3(color, sediment_color, sediment_cover * 0.62);
            color = mix3(color, shrub, shrub_cover * 0.78);
            color = mix3(color, grass, grass_cover * 0.88);
            let visible_canopy = smoothstep(0.12, 0.84, forest_cover);
            let canopy_opacity = visible_canopy
                * (0.58 + forest_cover * 0.16 + clamp01(canopy_texture + 0.5) * 0.07);
            color = mix3(color, forest, canopy_opacity);
            let floodplain_color = if matches!(config.preset, TerrainPreset::Arid) {
                [0.41, 0.38, 0.24]
            } else {
                [0.18, 0.32, 0.17]
            };
            color = mix3(
                color,
                floodplain_color,
                floodplain.max(semantic_floodplain) * 0.24,
            );
            color = mix3(color, [0.12, 0.255, 0.19], wetland * 0.34);
            color = mix3(color, rock_color, rock * (0.58 + (1.0 - soil_depth) * 0.42));
            color = mix3(
                color,
                [0.25, 0.245, 0.225],
                fracture * (0.025 + rock * 0.08),
            );
            color = mix3(color, [0.82, 0.845, 0.84], clamp01(snow * 1.18));

            let water_amount = water.max(river * (0.48 + wet * 0.42));
            if water_amount > 0.32 {
                let sediment = clamp01(river * dry * 0.8);
                let water_color = mix3([0.065, 0.135, 0.145], [0.22, 0.265, 0.19], sediment);
                color = mix3(color, water_color, clamp01((water_amount - 0.29) * 1.08));
            }

            let terrain_shade = hillshade * (0.92 - clamp01(slope * 1.7) * 0.09 + detail * 0.025);
            let surface_shade = lerp(terrain_shade, 0.94 + fine * 0.015, open_water);
            let haze = clamp01(config.haze / 10.0) * 0.12;
            for channel in 0..3 {
                color[channel] *= surface_shade;
            }
            color = mix3(color, [0.58, 0.62, 0.60], haze * (0.35 + elev * 0.65));
            if temp < -8.0 {
                color = mix3(
                    color,
                    [0.72, 0.76, 0.77],
                    clamp01((-temp - 8.0) / 25.0) * 0.12,
                );
            }
            pixel[0] = color_to_u8(color[0]);
            pixel[1] = color_to_u8(color[1]);
            pixel[2] = color_to_u8(color[2]);
            pixel[3] = 255;
        });

    progress(0.98, "编码正射自然色影像");
    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(output_size as u32, output_size as u32, data)
        .ok_or(TerrainError::InvalidTerrain)
}

/// The semantic ground cover the terrain shader builds its materials from, as
/// RGBA8 at `mesh_size` squared: R forest, G grass and shrub, B snow, A how
/// natural the ground is (255 = paint it procedurally, 0 = keep the baked image,
/// which is where water, roads, fields and towns are painted). `keep` is an
/// optional 0..255 mask of such built or open-water surfaces to preserve.
pub fn material_control_map(terrain: &TerrainData, mesh_size: usize, keep: Option<&[u8]>) -> Vec<u8> {
    let source = terrain.size;
    let scale = (source - 1) as f32 / (mesh_size - 1) as f32;
    let mut out = vec![0_u8; mesh_size * mesh_size * 4];
    out.par_chunks_mut(4).enumerate().for_each(|(i, px)| {
        let sx = (i % mesh_size) as f32 * scale;
        let sy = (i / mesh_size) as f32 * scale;
        let forest = bilinear(&terrain.forest, source, sx, sy);
        let grass = bilinear(&terrain.grassland, source, sx, sy)
            .max(bilinear(&terrain.shrubland, source, sx, sy) * 0.85);
        let snow = bilinear(&terrain.snow, source, sx, sy);
        let water = bilinear(&terrain.water, source, sx, sy);
        let kept = keep.map_or(0.0, |k| k[i] as f32 / 255.0);
        let natural = clamp01(1.0 - (water * 1.5).max(kept * 1.5));
        px[0] = (clamp01(forest) * 255.0) as u8;
        px[1] = (clamp01(grass) * 255.0) as u8;
        px[2] = (clamp01(snow) * 255.0) as u8;
        px[3] = (natural * 255.0) as u8;
    });
    out
}

pub fn save_png(image: &RgbaImage, path: impl AsRef<std::path::Path>) -> Result<(), TerrainError> {
    image.save_with_format(path, image::ImageFormat::Png)?;
    Ok(())
}

pub fn downsample_height(terrain: &TerrainData, output_size: usize) -> Vec<f32> {
    let output_size = output_size.clamp(2, terrain.size);
    let mut output = vec![0.0_f32; output_size * output_size];
    output
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            let x = index % output_size;
            let y = index / output_size;
            let sx = x as f32 / (output_size - 1) as f32 * (terrain.size - 1) as f32;
            let sy = y as f32 / (output_size - 1) as f32 * (terrain.size - 1) as f32;
            *value = bilinear(&terrain.height, terrain.size, sx, sy);
        });
    output
}

fn validate_config(config: &SimulationConfig) -> Result<(), TerrainError> {
    if !(128..=2048).contains(&config.grid_size) {
        return Err(TerrainError::InvalidGridSize);
    }
    if !(5.0..=500.0).contains(&config.world_size_km) {
        return Err(TerrainError::InvalidWorldSize);
    }
    Ok(())
}

fn default_cloud_coverage() -> f32 {
    35.0
}

fn default_cloud_speed() -> f32 {
    24.0
}

fn priority_flood(height: &[f32], n: usize, filled: &mut [f32]) {
    filled.copy_from_slice(height);
    let mut visited = vec![false; height.len()];
    let mut queue = BinaryHeap::new();
    for x in 0..n {
        for index in [x, (n - 1) * n + x] {
            if !visited[index] {
                visited[index] = true;
                queue.push(HeapCell {
                    elevation: filled[index],
                    index,
                });
            }
        }
    }
    for y in 1..n - 1 {
        for index in [y * n, y * n + n - 1] {
            if !visited[index] {
                visited[index] = true;
                queue.push(HeapCell {
                    elevation: filled[index],
                    index,
                });
            }
        }
    }
    while let Some(cell) = queue.pop() {
        let x = cell.index % n;
        let y = cell.index / n;
        for oy in -1_isize..=1 {
            for ox in -1_isize..=1 {
                if ox == 0 && oy == 0 {
                    continue;
                }
                let xx = x as isize + ox;
                let yy = y as isize + oy;
                if xx < 0 || yy < 0 || xx >= n as isize || yy >= n as isize {
                    continue;
                }
                let candidate = yy as usize * n + xx as usize;
                if visited[candidate] {
                    continue;
                }
                visited[candidate] = true;
                if filled[candidate] <= cell.elevation {
                    filled[candidate] = cell.elevation + 0.002;
                }
                queue.push(HeapCell {
                    elevation: filled[candidate],
                    index: candidate,
                });
            }
        }
    }
}

fn compute_routing(
    height: &[f32],
    n: usize,
    receiver: &mut [usize],
    secondary: &mut [usize],
    primary_weight: &mut [f32],
) {
    receiver
        .par_iter_mut()
        .zip(secondary.par_iter_mut())
        .zip(primary_weight.par_iter_mut())
        .enumerate()
        .for_each(|(index, ((target, alternate), weight))| {
            let x = index % n;
            let y = index / n;
            if x == 0 || y == 0 || x + 1 == n || y + 1 == n {
                *target = usize::MAX;
                *alternate = usize::MAX;
                *weight = 1.0;
                return;
            }
            let mut best = usize::MAX;
            let mut second = usize::MAX;
            let mut best_slope = 0.0_f32;
            let mut second_slope = 0.0_f32;
            for oy in -1_isize..=1 {
                for ox in -1_isize..=1 {
                    if ox == 0 && oy == 0 {
                        continue;
                    }
                    let candidate = (y as isize + oy) as usize * n + (x as isize + ox) as usize;
                    let distance = if ox != 0 && oy != 0 {
                        std::f32::consts::SQRT_2
                    } else {
                        1.0
                    };
                    let slope = (height[index] - height[candidate]) / distance;
                    if slope > best_slope {
                        second = best;
                        second_slope = best_slope;
                        best = candidate;
                        best_slope = slope;
                    } else if slope > second_slope {
                        second = candidate;
                        second_slope = slope;
                    }
                }
            }
            *target = best;
            *alternate = second;
            let sum = best_slope + second_slope;
            *weight = if second == usize::MAX || sum <= f32::EPSILON {
                1.0
            } else {
                (best_slope / sum).clamp(0.55, 0.92)
            };
        });
}

fn accumulate_flow(
    height: &[f32],
    moisture: &[f32],
    receiver: &[usize],
    secondary: &[usize],
    primary_weight: &[f32],
    flow: &mut [f32],
) {
    flow.par_iter_mut()
        .enumerate()
        .for_each(|(i, value)| *value = 0.15 + moisture[i] * 0.85);
    let mut order: Vec<usize> = (0..height.len()).collect();
    order.par_sort_unstable_by(|a, b| height[*b].total_cmp(&height[*a]));
    for index in order {
        let target = receiver[index];
        if target != usize::MAX {
            let weight = primary_weight[index];
            flow[target] += flow[index] * weight;
            let alternate = secondary[index];
            if alternate != usize::MAX {
                flow[alternate] += flow[index] * (1.0 - weight);
            }
        }
    }
}

fn compute_basins(receiver: &[usize], n: usize) -> Vec<u32> {
    let mut basins = vec![0_u32; receiver.len()];
    for start in 0..receiver.len() {
        if basins[start] != 0 {
            continue;
        }
        let mut path = Vec::new();
        let mut current = start;
        let basin = loop {
            if basins[current] != 0 {
                break basins[current];
            }
            path.push(current);
            let next = receiver[current];
            if next == usize::MAX {
                let x = current % n;
                let y = current / n;
                let outlet = if x == 0 || y == 0 || x + 1 == n || y + 1 == n {
                    current
                } else {
                    start
                };
                break outlet as u32 + 1;
            }
            if path.len() > receiver.len() {
                break start as u32 + 1;
            }
            current = next;
        };
        for index in path {
            basins[index] = basin;
        }
    }
    basins
}

fn compute_river_order(
    height: &[f32],
    flow: &[f32],
    receiver: &[usize],
    max_flow: f32,
    river_order: &mut [u8],
) {
    river_order.fill(0);
    let mut strongest_upstream = vec![0_u8; receiver.len()];
    let mut strongest_count = vec![0_u8; receiver.len()];
    let mut order: Vec<usize> = (0..height.len()).collect();
    order.par_sort_unstable_by(|a, b| height[*b].total_cmp(&height[*a]));
    let channel_threshold = max_flow * 0.0008;

    for index in order {
        if flow[index] < channel_threshold {
            continue;
        }
        let upstream_order = strongest_upstream[index];
        let current_order = if upstream_order == 0 {
            1
        } else if strongest_count[index] >= 2 {
            upstream_order.saturating_add(1)
        } else {
            upstream_order
        };
        river_order[index] = current_order;

        let target = receiver[index];
        if target == usize::MAX {
            continue;
        }
        if current_order > strongest_upstream[target] {
            strongest_upstream[target] = current_order;
            strongest_count[target] = 1;
        } else if current_order == strongest_upstream[target] {
            strongest_count[target] = strongest_count[target].saturating_add(1);
        }
    }
}

fn erode_channels(
    height: &mut [f32],
    flow: &[f32],
    receiver: &[usize],
    erosion_resistance: &[f32],
    n: usize,
    strength: f32,
) {
    let original = height.to_vec();
    height
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            let target = receiver[index];
            if target == usize::MAX {
                return;
            }
            let slope = ((original[index] - original[target]).max(0.0) / 100.0).sqrt();
            let power = flow[index].ln_1p() * slope;
            let incision =
                (power * strength * 0.42 / erosion_resistance[index].max(0.25)).min(18.0);
            *value = (original[index] - incision).max(0.0);
            let x = index % n;
            let y = index / n;
            if x < 2 || y < 2 || x + 2 >= n || y + 2 >= n {
                *value = original[index];
            }
        });
}

fn breach_overflowing_spillways(
    height: &mut [f32],
    filled_height: &[f32],
    flow: &[f32],
    receiver: &[usize],
    n: usize,
) {
    let max_flow = flow.iter().copied().fold(1.0_f32, f32::max);
    let mut cuts = vec![0.0_f32; height.len()];
    let mut labels = vec![0_u32; height.len()];
    let mut next_label = 1_u32;
    let mut queue = VecDeque::new();

    for seed in 0..height.len() {
        if labels[seed] != 0 || filled_height[seed] - height[seed] < 5.0 {
            continue;
        }
        let label = next_label;
        next_label = next_label.saturating_add(1);
        labels[seed] = label;
        queue.push_back(seed);
        let mut members = Vec::new();
        while let Some(index) = queue.pop_front() {
            members.push(index);
            let x = index % n;
            let y = index / n;
            for offset_y in -1_isize..=1 {
                for offset_x in -1_isize..=1 {
                    if offset_x == 0 && offset_y == 0 {
                        continue;
                    }
                    let xx = x as isize + offset_x;
                    let yy = y as isize + offset_y;
                    if xx < 1 || yy < 1 || xx + 1 >= n as isize || yy + 1 >= n as isize {
                        continue;
                    }
                    let neighbour = yy as usize * n + xx as usize;
                    if labels[neighbour] == 0 && filled_height[neighbour] - height[neighbour] >= 5.0
                    {
                        labels[neighbour] = label;
                        queue.push_back(neighbour);
                    }
                }
            }
        }

        let Some(&bottom) = members
            .iter()
            .min_by(|a, b| height[**a].total_cmp(&height[**b]))
        else {
            continue;
        };
        let mut outlet = None;
        for &index in &members {
            let target = receiver[index];
            if target == usize::MAX || labels[target] == label {
                continue;
            }
            if outlet.is_none_or(|current: usize| {
                filled_height[index] < filled_height[current]
                    || (filled_height[index] == filled_height[current]
                        && flow[index] > flow[current])
            }) {
                outlet = Some(index);
            }
        }
        let Some(outlet) = outlet else { continue };
        let spill_height = filled_height[outlet];
        let head = spill_height - height[bottom];
        let discharge = flow[outlet] / max_flow;
        let basin_scale = (members.len() as f32).sqrt();
        if head < 9.0 || discharge < 0.000_08 || basin_scale < 1.7 {
            continue;
        }

        // Overflow progressively saws through a sill. Incision depends on
        // hydraulic head and discharge, opposed by local sill relief as a
        // proxy for resistant rock. Closed or weakly supplied lakes can remain
        // at any elevation because there is no altitude threshold here.
        let discharge_gain = (flow[outlet].ln_1p() / max_flow.ln_1p()).clamp(0.0, 1.0);
        let sill_relief = local_slope(height, n, outlet % n, outlet / n);
        let resistance = 1.0 + (sill_relief / 180.0).clamp(0.0, 2.5);
        // A generation pass represents a long geomorphic interval rather
        // than one storm. A strongly supplied weak sill can therefore retreat
        // by a substantial fraction of its remaining head; repeated routing
        // passes converge toward a shallow outlet without a depth clamp.
        let incision = (head * (0.20 + discharge_gain * 0.35) / resistance).clamp(2.0, head * 0.62);
        let target_spill = (spill_height - incision).max(height[bottom] + 3.0);

        // Cut only the sill and a short reach below it. Re-running flood fill
        // on the next evolution step lets the knickpoint migrate upstream;
        // carving the entire D8 path at once creates ruler-straight trenches.
        let mut cursor = outlet;
        for downstream_step in 0..=6 {
            if cursor == usize::MAX {
                break;
            }
            let bed = target_spill - downstream_step as f32 * 0.55;
            let centre_x = cursor % n;
            let centre_y = cursor / n;
            for offset_y in -2_isize..=2 {
                for offset_x in -2_isize..=2 {
                    let xx = centre_x as isize + offset_x;
                    let yy = centre_y as isize + offset_y;
                    if xx < 1 || yy < 1 || xx + 1 >= n as isize || yy + 1 >= n as isize {
                        continue;
                    }
                    let distance = ((offset_x * offset_x + offset_y * offset_y) as f32).sqrt();
                    if distance > 2.25 {
                        continue;
                    }
                    let neighbour = yy as usize * n + xx as usize;
                    let valley_bed = bed + distance * 5.5;
                    cuts[neighbour] =
                        cuts[neighbour].max((height[neighbour] - valley_bed).max(0.0));
                }
            }
            cursor = receiver[cursor];
        }
    }
    for (value, cut) in height.iter_mut().zip(cuts) {
        *value -= cut;
    }
}

fn diffuse_slopes(height: &mut [f32], n: usize, amount: f32) {
    let original = height.to_vec();
    height
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            let x = index % n;
            let y = index / n;
            if x == 0 || y == 0 || x + 1 == n || y + 1 == n {
                return;
            }
            let average = (original[index - 1]
                + original[index + 1]
                + original[index - n]
                + original[index + n])
                * 0.25;
            *value = original[index] + (average - original[index]) * amount;
        });
}

fn calculate_stats(
    n: usize,
    height: &[f32],
    forest: &[f32],
    snow: &[f32],
    water: &[f32],
    world_size_km: f32,
) -> TerrainStats {
    let min_elevation = height
        .par_iter()
        .copied()
        .reduce(|| f32::INFINITY, f32::min);
    let max_elevation = height
        .par_iter()
        .copied()
        .reduce(|| f32::NEG_INFINITY, f32::max);
    let mean_elevation = height.par_iter().sum::<f32>() / height.len() as f32;
    let cell = world_size_km * 1000.0 / n as f32;
    let slopes: f32 = (n..height.len() - n)
        .into_par_iter()
        .filter(|i| i % n > 0 && i % n + 1 < n)
        .map(|i| {
            let dx = (height[i + 1] - height[i - 1]) / (2.0 * cell);
            let dy = (height[i + n] - height[i - n]) / (2.0 * cell);
            (dx * dx + dy * dy).sqrt().atan().to_degrees()
        })
        .sum();
    let slope_samples = ((n - 2) * (n - 2)) as f32;
    TerrainStats {
        min_elevation,
        max_elevation,
        mean_elevation,
        mean_slope: slopes / slope_samples.max(1.0),
        water_coverage: water.par_iter().filter(|value| **value > 0.35).count() as f32
            / water.len() as f32,
        snow_coverage: snow.par_iter().filter(|value| **value > 0.45).count() as f32
            / snow.len() as f32,
        forest_coverage: forest.par_iter().filter(|value| **value > 0.48).count() as f32
            / forest.len() as f32,
    }
}

fn physical_slope(height: &[f32], n: usize, x: usize, y: usize, cell_metres: f32) -> f32 {
    let left = height[y * n + x.saturating_sub(1)];
    let right = height[y * n + (x + 1).min(n - 1)];
    let up = height[y.saturating_sub(1) * n + x];
    let down = height[(y + 1).min(n - 1) * n + x];
    let dx = (right - left) / (2.0 * cell_metres);
    let dy = (down - up) / (2.0 * cell_metres);
    (dx * dx + dy * dy).sqrt()
}

fn local_slope(height: &[f32], n: usize, x: usize, y: usize) -> f32 {
    let left = height[y * n + x.saturating_sub(1)];
    let right = height[y * n + (x + 1).min(n - 1)];
    let up = height[y.saturating_sub(1) * n + x];
    let down = height[(y + 1).min(n - 1) * n + x];
    ((right - left).powi(2) + (down - up).powi(2)).sqrt() / 360.0
}

fn bilinear(data: &[f32], n: usize, x: f32, y: f32) -> f32 {
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(n - 1);
    let y1 = (y0 + 1).min(n - 1);
    let tx = x - x0 as f32;
    let ty = y - y0 as f32;
    let top = lerp(data[y0 * n + x0], data[y0 * n + x1], tx);
    let bottom = lerp(data[y1 * n + x0], data[y1 * n + x1], tx);
    lerp(top, bottom, ty)
}

fn hash(x: i32, y: i32, seed: u32) -> f32 {
    let mut value =
        (x as u32).wrapping_mul(0x9e37_79b1) ^ (y as u32).wrapping_mul(0x85eb_ca77) ^ seed;
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^= value >> 16;
    value as f32 / u32::MAX as f32 * 2.0 - 1.0
}

fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let tx = smooth_curve(x - x.floor());
    let ty = smooth_curve(y - y.floor());
    let a = hash(x0, y0, seed);
    let b = hash(x0 + 1, y0, seed);
    let c = hash(x0, y0 + 1, seed);
    let d = hash(x0 + 1, y0 + 1, seed);
    lerp(lerp(a, b, tx), lerp(c, d, tx), ty)
}

fn fbm(x: f32, y: f32, seed: u32, octaves: usize) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.55;
    let mut frequency = 1.0;
    let mut total = 0.0;
    for octave in 0..octaves {
        value += value_noise(
            x * frequency,
            y * frequency,
            seed.wrapping_add(octave as u32 * 7919),
        ) * amplitude;
        total += amplitude;
        amplitude *= 0.5;
        frequency *= 2.03;
    }
    value / total.max(f32::EPSILON)
}

fn smooth_curve(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}
fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    smooth_curve(clamp01((value - edge0) / (edge1 - edge0)))
}
fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}
fn lerp(a: f32, b: f32, amount: f32) -> f32 {
    a + (b - a) * amount
}
fn mix3(a: [f32; 3], b: [f32; 3], amount: f32) -> [f32; 3] {
    [
        lerp(a[0], b[0], amount),
        lerp(a[1], b[1], amount),
        lerp(a[2], b[2], amount),
    ]
}
fn color_to_u8(value: f32) -> u8 {
    (clamp01(value).powf(1.0 / 1.6) * 255.0 + 0.5) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> SimulationConfig {
        SimulationConfig {
            seed: 42,
            preset: TerrainPreset::Temperate,
            landform: Landform::MountainRange,
            grid_size: 128,
            world_size_km: 80.0,
            rainfall: 1200.0,
            evaporation: 600.0,
            wind_speed: 8.0,
            wind_direction: 220.0,
            sun_azimuth: 235.0,
            sun_elevation: 42.0,
            haze: 2.5,
            cloud_coverage: 35.0,
            cloud_speed: 24.0,
        }
    }

    #[test]
    fn generation_is_deterministic() {
        let first = generate(&config(), |_, _| {}).unwrap();
        let second = generate(&config(), |_, _| {}).unwrap();
        assert_eq!(first.height, second.height);
        assert_eq!(first.flow, second.flow);
    }

    #[test]
    fn generated_fields_are_finite_and_bounded() {
        let terrain = generate(&config(), |_, _| {}).unwrap();
        assert!(
            terrain
                .height
                .iter()
                .all(|value| value.is_finite() && *value >= 0.0)
        );
        assert!(
            terrain
                .vegetation
                .iter()
                .all(|value| (0.0..=1.0).contains(value))
        );
        for layer in [
            &terrain.geology,
            &terrain.soil_depth,
            &terrain.forest,
            &terrain.grassland,
            &terrain.shrubland,
            &terrain.bare_ground,
            &terrain.sediment,
        ] {
            assert!(layer.iter().all(|value| (0.0..=1.0).contains(value)));
        }
        assert!(
            terrain
                .flow_direction_x
                .iter()
                .chain(&terrain.flow_direction_y)
                .all(|value| value.is_finite() && (-1.0..=1.0).contains(value))
        );
        assert!(terrain.stats.max_elevation > terrain.stats.min_elevation);
    }

    #[test]
    fn land_cover_contains_multiple_distinct_communities() {
        let terrain = generate(&config(), |_, _| {}).unwrap();
        let covered = [
            terrain.forest.iter().filter(|value| **value > 0.25).count(),
            terrain
                .grassland
                .iter()
                .filter(|value| **value > 0.25)
                .count(),
            terrain
                .shrubland
                .iter()
                .filter(|value| **value > 0.20)
                .count(),
            terrain
                .bare_ground
                .iter()
                .filter(|value| **value > 0.25)
                .count(),
        ];
        assert!(covered.into_iter().filter(|count| *count > 64).count() >= 3);
    }

    #[test]
    fn forest_cover_responds_to_climate_without_greening_every_biome() {
        fn forest_fraction(terrain: &TerrainData) -> f32 {
            let habitable_land = terrain
                .water
                .iter()
                .zip(&terrain.snow)
                .filter(|(water, snow)| **water < 0.35 && **snow < 0.55)
                .count()
                .max(1);
            terrain
                .forest
                .iter()
                .zip(&terrain.water)
                .zip(&terrain.snow)
                .filter(|((forest, water), snow)| {
                    **forest > 0.48 && **water < 0.35 && **snow < 0.55
                })
                .count() as f32
                / habitable_land as f32
        }

        let mut wet_temperate = config();
        wet_temperate.landform = Landform::Hills;
        wet_temperate.rainfall = 1450.0;
        wet_temperate.evaporation = 500.0;
        let wet_fraction = forest_fraction(&generate(&wet_temperate, |_, _| {}).unwrap());

        let mut dry_temperate = wet_temperate.clone();
        dry_temperate.rainfall = 380.0;
        dry_temperate.evaporation = 900.0;
        let dry_fraction = forest_fraction(&generate(&dry_temperate, |_, _| {}).unwrap());

        let mut arid = wet_temperate.clone();
        arid.preset = TerrainPreset::Arid;
        arid.rainfall = 380.0;
        arid.evaporation = 1050.0;
        let arid_fraction = forest_fraction(&generate(&arid, |_, _| {}).unwrap());

        let mut glacial = wet_temperate.clone();
        glacial.preset = TerrainPreset::Glacial;
        glacial.landform = Landform::MountainRange;
        let glacial_fraction = forest_fraction(&generate(&glacial, |_, _| {}).unwrap());

        assert!(
            wet_fraction > 0.32,
            "wet temperate forest fraction was only {wet_fraction:.3}"
        );
        assert!(
            wet_fraction < 0.78,
            "wet temperate forest erased open habitats: {wet_fraction:.3}"
        );
        assert!(
            wet_fraction > dry_fraction + 0.18,
            "forest did not respond strongly to moisture: wet={wet_fraction:.3}, dry={dry_fraction:.3}"
        );
        assert!(
            arid_fraction < 0.18,
            "arid forest fraction was too high: {arid_fraction:.3}"
        );
        assert!(
            glacial_fraction < wet_fraction * 0.72,
            "glacial forest should stay below wet temperate cover: glacial={glacial_fraction:.3}, wet={wet_fraction:.3}"
        );
    }

    #[test]
    fn satellite_render_has_requested_dimensions() {
        let configuration = config();
        let terrain = generate(&configuration, |_, _| {}).unwrap();
        let image = render_satellite(&terrain, &configuration, 256, |_, _| {}).unwrap();
        assert_eq!(image.dimensions(), (256, 256));
    }

    #[test]
    fn landform_families_produce_distinct_relief() {
        let mut mountain_config = config();
        mountain_config.landform = Landform::MountainRange;
        let mountain = generate(&mountain_config, |_, _| {}).unwrap();
        let mut plains_config = config();
        plains_config.landform = Landform::Plains;
        let plains = generate(&plains_config, |_, _| {}).unwrap();
        let mountain_relief = mountain.stats.max_elevation - mountain.stats.min_elevation;
        let plains_relief = plains.stats.max_elevation - plains.stats.min_elevation;
        assert!(mountain_relief > plains_relief * 4.0);
    }

    #[test]
    fn coastal_and_archipelago_worlds_contain_open_water() {
        for landform in [Landform::Coastal, Landform::Archipelago] {
            let mut configuration = config();
            configuration.landform = landform;
            let terrain = generate(&configuration, |_, _| {}).unwrap();
            assert!(terrain.stats.water_coverage > 0.08);
        }
    }

    #[test]
    fn priority_flood_never_lowers_terrain_and_marks_depressions() {
        let n = 5;
        let mut height = vec![10.0; n * n];
        height[2 * n + 2] = 1.0;
        let mut filled = vec![0.0; height.len()];
        priority_flood(&height, n, &mut filled);

        assert!(
            filled
                .iter()
                .zip(&height)
                .all(|(filled, raw)| filled >= raw)
        );
        assert!(filled[2 * n + 2] > height[2 * n + 2]);
    }

    #[test]
    fn filled_surface_routes_every_interior_cell_without_cycles() {
        let terrain = generate(&config(), |_, _| {}).unwrap();
        let n = terrain.size;
        let mut receiver = vec![usize::MAX; n * n];
        let mut secondary = vec![usize::MAX; n * n];
        let mut weight = vec![1.0; n * n];
        compute_routing(
            &terrain.filled_height,
            n,
            &mut receiver,
            &mut secondary,
            &mut weight,
        );

        for y in 1..n - 1 {
            for x in 1..n - 1 {
                let start = y * n + x;
                assert_ne!(receiver[start], usize::MAX);
                let mut current = start;
                for step in 0..n * n {
                    let next = receiver[current];
                    if next == usize::MAX {
                        break;
                    }
                    assert!(terrain.filled_height[next] < terrain.filled_height[current]);
                    current = next;
                    assert!(step + 1 < n * n, "routing cycle from cell {start}");
                }
            }
        }
    }

    #[test]
    fn dual_flow_conserves_runoff_at_boundary_outlets() {
        let terrain = generate(&config(), |_, _| {}).unwrap();
        let n = terrain.size;
        let mut receiver = vec![usize::MAX; n * n];
        let mut secondary = vec![usize::MAX; n * n];
        let mut weight = vec![1.0; n * n];
        let mut flow = vec![0.0; n * n];
        compute_routing(
            &terrain.filled_height,
            n,
            &mut receiver,
            &mut secondary,
            &mut weight,
        );
        accumulate_flow(
            &terrain.filled_height,
            &terrain.moisture,
            &receiver,
            &secondary,
            &weight,
            &mut flow,
        );

        let rainfall: f32 = terrain
            .moisture
            .iter()
            .map(|value| 0.15 + value * 0.85)
            .sum();
        let outlet_flow: f32 = flow
            .iter()
            .zip(&receiver)
            .filter_map(|(value, target)| (*target == usize::MAX).then_some(*value))
            .sum();
        let relative_error = (outlet_flow - rainfall).abs() / rainfall;
        assert!(relative_error < 1.0e-4, "flow error was {relative_error}");
    }

    #[test]
    fn semantic_hydrology_layers_are_consistent() {
        let terrain = generate(&config(), |_, _| {}).unwrap();
        assert!(terrain.basin_id.iter().all(|id| *id != 0));
        assert!(
            terrain
                .lake
                .iter()
                .zip(&terrain.filled_height)
                .zip(&terrain.height)
                .all(|((lake, filled), raw)| *lake <= 0.0 || filled > raw)
        );
        assert!(terrain.river_order.iter().any(|order| *order >= 2));
    }

    fn overflowing_basin_fixture(
        elevation_offset: f32,
        outlet_flow: f32,
    ) -> (usize, Vec<f32>, Vec<f32>, Vec<f32>, Vec<usize>) {
        let n = 25;
        let mut height = vec![140.0 + elevation_offset; n * n];
        let mut filled = height.clone();
        let mut flow = vec![1.0; n * n];
        let mut receiver = vec![usize::MAX; n * n];
        let outlet = 11 * n + 11;

        for y in 9..=13 {
            for x in 7..=11 {
                let index = y * n + x;
                height[index] = 100.0 + elevation_offset;
                filled[index] = 130.0 + elevation_offset;
                receiver[index] = outlet;
            }
        }
        height[11 * n + 9] = 90.0 + elevation_offset;
        height[outlet] = 125.0 + elevation_offset;
        flow[outlet] = outlet_flow;

        // A short, non-collinear downstream reach represents the locally
        // selected valley. The breach pass must widen it, not dig single-cell
        // D8 holes or project a ruler-straight trench across the terrain.
        let reach = [outlet, 11 * n + 12, 12 * n + 13, 12 * n + 14, 13 * n + 15];
        for pair in reach.windows(2) {
            receiver[pair[0]] = pair[1];
        }
        flow[0] = 10_000.0;
        (n, height, filled, flow, receiver)
    }

    #[test]
    fn spillway_incision_is_driven_by_overflow_not_absolute_elevation() {
        let (n, mut low, low_filled, flow, receiver) = overflowing_basin_fixture(0.0, 10_000.0);
        let low_before = low.clone();
        breach_overflowing_spillways(&mut low, &low_filled, &flow, &receiver, n);

        let (n, mut high, high_filled, flow, receiver) =
            overflowing_basin_fixture(4_000.0, 10_000.0);
        let high_before = high.clone();
        breach_overflowing_spillways(&mut high, &high_filled, &flow, &receiver, n);

        for index in 0..low.len() {
            let low_cut = low_before[index] - low[index];
            let high_cut = high_before[index] - high[index];
            assert!((low_cut - high_cut).abs() < 1.0e-3);
        }
        assert!(
            low.iter()
                .zip(&low_before)
                .any(|(after, before)| after < before)
        );
    }

    #[test]
    fn weakly_supplied_closed_basin_is_not_forcibly_drained() {
        let (n, mut height, filled, flow, receiver) = overflowing_basin_fixture(3_200.0, 0.01);
        let before = height.clone();
        breach_overflowing_spillways(&mut height, &filled, &flow, &receiver, n);
        assert_eq!(height, before);
    }

    #[test]
    fn spillway_cut_is_wide_shallow_and_follows_local_valley() {
        let (n, mut height, filled, flow, receiver) = overflowing_basin_fixture(1_800.0, 10_000.0);
        let before = height.clone();
        breach_overflowing_spillways(&mut height, &filled, &flow, &receiver, n);
        let changed: Vec<_> = height
            .iter()
            .zip(&before)
            .enumerate()
            .filter_map(|(index, (after, old))| (after < old).then_some(index))
            .collect();
        assert!(
            changed.len() >= 12,
            "spillway was only {} cells wide",
            changed.len()
        );
        assert!(changed.iter().any(|index| index / n == 12));
        assert!(changed.iter().any(|index| index / n == 13));
        let deepest_cut = changed
            .iter()
            .map(|index| before[*index] - height[*index])
            .fold(0.0_f32, f32::max);
        assert!(deepest_cut < 35.0, "single-pass cut was {deepest_cut} m");
        for &index in &changed {
            let x = index % n;
            let y = index / n;
            let neighbours = changed.iter().filter(|other| {
                let xx = **other % n;
                let yy = **other / n;
                xx.abs_diff(x) <= 1 && yy.abs_diff(y) <= 1 && **other != index
            });
            assert!(neighbours.count() > 0, "isolated incision at {x},{y}");
        }
    }

    #[test]
    fn the_material_control_map_carries_cover_and_marks_built_and_wet_ground() {
        let config = SimulationConfig {
            seed: 11,
            preset: TerrainPreset::Temperate,
            landform: Landform::Hills,
            grid_size: 128,
            world_size_km: 40.0,
            rainfall: 1200.0,
            evaporation: 600.0,
            wind_speed: 8.0,
            wind_direction: 220.0,
            sun_azimuth: 235.0,
            sun_elevation: 42.0,
            haze: 2.5,
            cloud_coverage: 35.0,
            cloud_speed: 24.0,
        };
        let terrain = generate(&config, |_, _| {}).unwrap();
        let mesh = 64;
        let free = material_control_map(&terrain, mesh, None);
        assert_eq!(free.len(), mesh * mesh * 4);
        // Cover follows the terrain: some forest and some natural ground exist.
        assert!(free.chunks(4).any(|p| p[0] > 100), "no forest channel");
        assert!(free.chunks(4).any(|p| p[3] == 255), "no fully natural ground");
        // Open water is not painted procedurally.
        let wet = free.chunks(4).zip(0..).filter(|(p, _)| p[3] < 128).count();
        let any_water = terrain.water.iter().any(|w| *w > 0.7);
        assert_eq!(wet > 0, any_water, "natural flag must mirror open water");
        // A kept (built-up) region is excluded from procedural painting.
        let keep = vec![255_u8; mesh * mesh];
        let built = material_control_map(&terrain, mesh, Some(&keep));
        assert!(built.chunks(4).all(|p| p[3] == 0));
    }
}
