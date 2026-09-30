use base64::{Engine, engine::general_purpose::STANDARD};
use image::ImageFormat;
use serde::Serialize;
use std::{env, fs, io::Cursor, path::PathBuf};
use terrain_core::{
    Landform, SimulationConfig, TerrainPreset, TerrainStats, downsample_height, generate,
    render_satellite,
    sites::{CitySite, exclude_vegetation, flatten_heights},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    config: SimulationConfig,
    result: FixtureResult,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FixtureResult {
    preview_data_url: String,
    width: u32,
    height: u32,
    world_size_km: f32,
    elapsed_ms: u32,
    stats: TerrainStats,
    mesh_size: usize,
    water_data_size: usize,
    height_data_base64: String,
    forest_data_base64: String,
    vegetation_exclusion_data_base64: String,
    city_sites: Vec<CitySite>,
    far_trees: Vec<city_scene::trees::FarTreePayload>,
    urban_data_base64: String,
    cultivated_data_base64: String,
    crop_data_base64: String,
    road_data_base64: String,
    roads: Vec<()>,
    cities: Vec<()>,
    water_height_data_base64: String,
    water_mask_base64: String,
    water_kind_base64: String,
    flow_direction_base64: String,
    flow_strength_base64: String,
    analysis_previews: EmptyAnalysis,
    infrastructure_summary: EmptyInfrastructureSummary,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmptyInfrastructureSummary {
    settlements: usize,
    roads: usize,
    bridges: usize,
    tunnels: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmptyAnalysis {
    discharge: String,
    lake: String,
    wetland: String,
    floodplain: String,
    basin: String,
    river_order: String,
    geology: String,
    soil_depth: String,
    land_cover: String,
    travel_cost: String,
    hazard: String,
    settlement_suitability: String,
    agricultural_suitability: String,
    urban_land: String,
    cultivated_land: String,
    infrastructure: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("render-fixture.json"));
    let config = SimulationConfig {
        seed: 284_735,
        preset: TerrainPreset::Temperate,
        landform: Landform::Coastal,
        // The validation harness exercises shaders and scene evolution with a
        // representative mesh. Product generation still uses the 512-cell LOD;
        // software WebGL does not need four full-resolution renders per check.
        grid_size: 128,
        world_size_km: 80.0,
        rainfall: 1275.0,
        evaporation: 600.0,
        wind_speed: 7.0,
        wind_direction: 225.0,
        sun_azimuth: 235.0,
        sun_elevation: 42.0,
        haze: 2.5,
        cloud_coverage: 48.0,
        cloud_speed: 24.0,
    };
    let terrain = generate(&config, |_, _| {})?;
    let mesh_size = 128_usize.min(terrain.size);
    // `FIXTURE_CITY_RADIUS_M=900` declares one city site at the world centre (where
    // the harness's `?city=1` mounts a scene) and levels/clears the ground under
    // it exactly as the desktop payload does.
    let city_sites: Vec<CitySite> = env::var("FIXTURE_CITY_RADIUS_M")
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .map(|radius_m| {
            vec![CitySite {
                x_km: config.world_size_km / 2.0,
                y_km: config.world_size_km / 2.0,
                radius_m,
            }]
        })
        .unwrap_or_default();
    let mut mesh_heights = downsample_height(&terrain, mesh_size);
    flatten_heights(&mut mesh_heights, mesh_size, config.world_size_km, &city_sites);
    let mut exclusion = vec![0_u8; mesh_size * mesh_size];
    exclude_vegetation(&mut exclusion, mesh_size, config.world_size_km, &city_sites);
    let height_bytes: Vec<u8> = mesh_heights
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect();
    let forest_bytes: Vec<u8> = terrain
        .forest
        .iter()
        .map(|value| (value.clamp(0.0, 1.0) * 255.0) as u8)
        .collect();

    let mut water_height_bytes = Vec::with_capacity(mesh_size * mesh_size * 4);
    for y in 0..mesh_size {
        for x in 0..mesh_size {
            let sx = x * (terrain.size - 1) / (mesh_size - 1);
            let sy = y * (terrain.size - 1) / (mesh_size - 1);
            let index = sy * terrain.size + sx;
            let surface = if terrain.height[index] < 58.0 {
                58.0
            } else if terrain.lake[index] > 0.05 {
                terrain.filled_height[index]
            } else {
                terrain.height[index] + 0.35
            };
            water_height_bytes.extend_from_slice(&surface.to_le_bytes());
        }
    }

    let water_data_size = 128_usize.min(terrain.size);
    let max_flow = terrain.flow.iter().copied().fold(1.0_f32, f32::max);
    let max_flow_log = max_flow.ln_1p();
    let mut water_mask = Vec::with_capacity(water_data_size * water_data_size);
    let mut water_kind = Vec::with_capacity(water_data_size * water_data_size);
    let mut flow_direction = Vec::with_capacity(water_data_size * water_data_size * 2);
    let mut flow_strength = Vec::with_capacity(water_data_size * water_data_size);
    for y in 0..water_data_size {
        for x in 0..water_data_size {
            let sx = x * (terrain.size - 1) / (water_data_size - 1);
            let sy = y * (terrain.size - 1) / (water_data_size - 1);
            let index = sy * terrain.size + sx;
            let ocean = terrain.height[index] < 58.0;
            let lake = terrain.lake[index] > 0.05;
            water_mask.push((terrain.water[index].clamp(0.0, 1.0) * 255.0) as u8);
            water_kind.push(if ocean {
                255
            } else if lake {
                128
            } else {
                0
            });
            flow_direction
                .push(((terrain.flow_direction_x[index].clamp(-1.0, 1.0) * 127.0) as i8) as u8);
            flow_direction
                .push(((terrain.flow_direction_y[index].clamp(-1.0, 1.0) * 127.0) as i8) as u8);
            flow_strength
                .push(((terrain.flow[index].ln_1p() / max_flow_log).clamp(0.0, 1.0) * 255.0) as u8);
        }
    }

    let image = render_satellite(&terrain, &config, 768, |_, _| {})?;
    let dimensions = image.dimensions();
    let mut png = Cursor::new(Vec::new());
    image.write_to(&mut png, ImageFormat::Png)?;
    let empty = String::new();
    let fixture = Fixture {
        config: config.clone(),
        result: FixtureResult {
            preview_data_url: format!(
                "data:image/png;base64,{}",
                STANDARD.encode(png.into_inner())
            ),
            width: dimensions.0,
            height: dimensions.1,
            world_size_km: config.world_size_km,
            elapsed_ms: 0,
            stats: terrain.stats,
            mesh_size,
            water_data_size,
            height_data_base64: STANDARD.encode(height_bytes),
            forest_data_base64: STANDARD.encode(forest_bytes),
            vegetation_exclusion_data_base64: STANDARD.encode(exclusion),
            city_sites,
            far_trees: city_scene::trees::far_tree_payload(),
            urban_data_base64: STANDARD.encode(vec![0_u8; mesh_size * mesh_size]),
            cultivated_data_base64: STANDARD.encode(vec![0_u8; mesh_size * mesh_size]),
            crop_data_base64: STANDARD.encode(vec![0_u8; mesh_size * mesh_size]),
            road_data_base64: STANDARD.encode(vec![0_u8; mesh_size * mesh_size]),
            roads: Vec::new(),
            cities: Vec::new(),
            water_height_data_base64: STANDARD.encode(water_height_bytes),
            water_mask_base64: STANDARD.encode(water_mask),
            water_kind_base64: STANDARD.encode(water_kind),
            flow_direction_base64: STANDARD.encode(flow_direction),
            flow_strength_base64: STANDARD.encode(flow_strength),
            analysis_previews: EmptyAnalysis {
                discharge: empty.clone(),
                lake: empty.clone(),
                wetland: empty.clone(),
                floodplain: empty.clone(),
                basin: empty.clone(),
                river_order: empty.clone(),
                geology: empty.clone(),
                soil_depth: empty.clone(),
                land_cover: empty.clone(),
                travel_cost: empty.clone(),
                hazard: empty.clone(),
                settlement_suitability: empty.clone(),
                agricultural_suitability: empty.clone(),
                urban_land: empty.clone(),
                cultivated_land: empty.clone(),
                infrastructure: empty,
            },
            infrastructure_summary: EmptyInfrastructureSummary {
                settlements: 0,
                roads: 0,
                bridges: 0,
                tunnels: 0,
            },
        },
    };
    fs::write(&output, serde_json::to_vec(&fixture)?)?;
    println!("saved {}", output.display());
    Ok(())
}
