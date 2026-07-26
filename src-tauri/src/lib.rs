use base64::{Engine, engine::general_purpose::STANDARD};
use image::{ImageFormat, Rgba, RgbaImage};
use infrastructure::{CrossingKind, InfrastructureData, generate_infrastructure};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, path::PathBuf, time::Instant};
use tauri::{AppHandle, Emitter};
use terrain_core::{
    SimulationConfig, TerrainData, TerrainStats, downsample_height, generate, render_satellite,
    save_png,
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProgressEvent {
    stage: String,
    progress: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerationResult {
    preview_data_url: String,
    width: u32,
    height: u32,
    world_size_km: f32,
    elapsed_ms: u128,
    stats: TerrainStats,
    mesh_size: usize,
    water_data_size: usize,
    height_data_base64: String,
    forest_data_base64: String,
    vegetation_exclusion_data_base64: String,
    urban_data_base64: String,
    cultivated_data_base64: String,
    crop_data_base64: String,
    road_data_base64: String,
    roads: Vec<RenderRoad>,
    cities: serde_json::Value,
    water_height_data_base64: String,
    water_mask_base64: String,
    water_kind_base64: String,
    flow_direction_base64: String,
    flow_strength_base64: String,
    analysis_previews: AnalysisPreviews,
    infrastructure_summary: InfrastructureSummary,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RenderRoad {
    id: u32,
    class: infrastructure::RoadClass,
    profile: infrastructure::RoadProfile,
    length_km: f32,
    /// Continuous renderers consume kilometres; grid indices must never leak
    /// into a street-level width calculation.
    path_km: Vec<[f32; 2]>,
}

fn render_roads(infrastructure: &InfrastructureData) -> Vec<RenderRoad> {
    let grid = infrastructure.urban_land.grid;
    let denominator = (grid.size - 1) as f32;
    infrastructure
        .roads
        .iter()
        .map(|road| RenderRoad {
            id: road.id,
            class: road.class,
            profile: road.class.profile(),
            length_km: road.length_km,
            path_km: road
                .path
                .iter()
                .map(|point| {
                    [
                        point.x as f32 / denominator * grid.world_size_km,
                        point.y as f32 / denominator * grid.world_size_km,
                    ]
                })
                .collect(),
        })
        .collect()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InfrastructureSummary {
    settlements: usize,
    roads: usize,
    bridges: usize,
    tunnels: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnalysisPreviews {
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDocument {
    schema_version: u32,
    name: String,
    config: SimulationConfig,
}

fn emit_progress(app: &AppHandle, progress: f32, stage: &str) {
    let _ = app.emit(
        "terrain-progress",
        ProgressEvent {
            stage: stage.to_owned(),
            progress,
        },
    );
}

fn image_data_url(image: &RgbaImage) -> Result<String, String> {
    let mut png = Cursor::new(Vec::new());
    image
        .write_to(&mut png, ImageFormat::Png)
        .map_err(|error| error.to_string())?;
    Ok(format!(
        "data:image/png;base64,{}",
        STANDARD.encode(png.into_inner())
    ))
}

fn analysis_image<F>(terrain: &TerrainData, size: usize, color: F) -> RgbaImage
where
    F: Fn(usize) -> [u8; 3],
{
    let mut image = RgbaImage::new(size as u32, size as u32);
    for y in 0..size {
        for x in 0..size {
            let sx = x * (terrain.size - 1) / (size - 1);
            let sy = y * (terrain.size - 1) / (size - 1);
            let rgb = color(sy * terrain.size + sx);
            image.put_pixel(x as u32, y as u32, Rgba([rgb[0], rgb[1], rgb[2], 255]));
        }
    }
    image
}

fn draw_line(image: &mut RgbaImage, from: (i32, i32), to: (i32, i32), color: Rgba<u8>) {
    let (mut x, mut y) = from;
    let dx = (to.0 - x).abs();
    let sx = if x < to.0 { 1 } else { -1 };
    let dy = -(to.1 - y).abs();
    let sy = if y < to.1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        if x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32 {
            image.put_pixel(x as u32, y as u32, color);
        }
        if (x, y) == to {
            break;
        }
        let doubled = error * 2;
        if doubled >= dy {
            error += dy;
            x += sx;
        }
        if doubled <= dx {
            error += dx;
            y += sy;
        }
    }
}

fn draw_line_alpha(
    image: &mut RgbaImage,
    from: (i32, i32),
    to: (i32, i32),
    color: [f32; 3],
    alpha: f32,
) {
    let (mut x, mut y) = from;
    let dx = (to.0 - x).abs();
    let sx = if x < to.0 { 1 } else { -1 };
    let dy = -(to.1 - y).abs();
    let sy = if y < to.1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        if x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32 {
            blend_surface(image.get_pixel_mut(x as u32, y as u32), color, alpha);
        }
        if (x, y) == to {
            break;
        }
        let doubled = error * 2;
        if doubled >= dy {
            error += dy;
            x += sx;
        }
        if doubled <= dx {
            error += dx;
            y += sy;
        }
    }
}

fn draw_line_width(
    image: &mut RgbaImage,
    from: (i32, i32),
    to: (i32, i32),
    radius: i32,
    color: Rgba<u8>,
) {
    for offset_y in -radius..=radius {
        for offset_x in -radius..=radius {
            if offset_x * offset_x + offset_y * offset_y <= radius * radius {
                draw_line(
                    image,
                    (from.0 + offset_x, from.1 + offset_y),
                    (to.0 + offset_x, to.1 + offset_y),
                    color,
                );
            }
        }
    }
}

fn vegetation_exclusion_mask(infrastructure: &InfrastructureData, mesh_size: usize) -> Vec<u8> {
    let source_size = infrastructure.urban_land.grid.size;
    let mut mask = vec![0_u8; mesh_size * mesh_size];

    for y in 0..mesh_size {
        for x in 0..mesh_size {
            let sx = x * (source_size - 1) / (mesh_size - 1);
            let sy = y * (source_size - 1) / (mesh_size - 1);
            let source_index = sy * source_size + sx;
            let occupied = infrastructure.urban_land.values[source_index] > 0.04
                || infrastructure.cultivated_land.values[source_index] > 0.06;
            if occupied {
                mask[y * mesh_size + x] = 255;
            }
        }
    }

    rasterize_road_coverage(
        &mut mask,
        mesh_size,
        source_size,
        infrastructure.urban_land.grid.world_size_km,
        &infrastructure.roads,
        true,
    );
    mask
}

fn infrastructure_surface_masks(
    infrastructure: &InfrastructureData,
    mesh_size: usize,
) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let source_size = infrastructure.urban_land.grid.size;
    let mut urban = vec![0_u8; mesh_size * mesh_size];
    let mut cultivated = vec![0_u8; mesh_size * mesh_size];
    // Semantic crop type, sampled by the near-field vegetation LOD. Values
    // deliberately describe a field rather than individual pixels so that a
    // wheat field cannot turn into corn from one plant row to the next.
    // 0 = not cultivated, 1 = wheat, 2 = maize, 3 = other rotation crop.
    let mut crops = vec![0_u8; mesh_size * mesh_size];
    let mut roads = vec![0_u8; mesh_size * mesh_size];
    for y in 0..mesh_size {
        for x in 0..mesh_size {
            let source_x = x * (source_size - 1) / (mesh_size - 1);
            let source_y = y * (source_size - 1) / (mesh_size - 1);
            let source = source_y * source_size + source_x;
            let target = y * mesh_size + x;
            urban[target] =
                (infrastructure.urban_land.values[source].clamp(0.0, 1.0) * 255.0) as u8;
            cultivated[target] =
                (infrastructure.cultivated_land.values[source].clamp(0.0, 1.0) * 255.0) as u8;
            if cultivated[target] > 18 {
                let world_x = x as f32 / (mesh_size - 1) as f32
                    * infrastructure.cultivated_land.grid.world_size_km;
                let world_y = y as f32 / (mesh_size - 1) as f32
                    * infrastructure.cultivated_land.grid.world_size_km;
                let (_, _, parcel_x, parcel_y) = farm_parcel(world_x, world_y);
                let selector = surface_hash(parcel_x, parcel_y, 0x51c7);
                crops[target] = if selector < 0.48 {
                    1
                } else if selector < 0.76 {
                    2
                } else {
                    3
                };
            }
        }
    }
    rasterize_road_coverage(
        &mut roads,
        mesh_size,
        source_size,
        infrastructure.urban_land.grid.world_size_km,
        &infrastructure.roads,
        false,
    );
    (urban, cultivated, crops, roads)
}

fn rasterize_road_coverage(
    target: &mut [u8],
    target_size: usize,
    source_size: usize,
    world_size_km: f32,
    roads: &[infrastructure::Road],
    right_of_way: bool,
) {
    let scale = (target_size - 1) as f32 / (source_size - 1) as f32;
    let metres_per_pixel = world_size_km * 1000.0 / (target_size - 1) as f32;
    for road in roads {
        let profile = road.class.profile();
        let width_metres = if right_of_way {
            profile.right_of_way_width_metres
        } else {
            profile.carriageway_width_metres
        };
        let width_pixels = width_metres / metres_per_pixel;
        for segment in road.path.windows(2) {
            let ax = segment[0].x as f32 * scale;
            let ay = segment[0].y as f32 * scale;
            let bx = segment[1].x as f32 * scale;
            let by = segment[1].y as f32 * scale;
            let reach = (width_pixels * 0.5 + 1.0).max(1.0);
            let min_x = (ax.min(bx) - reach).floor().max(0.0) as usize;
            let max_x = (ax.max(bx) + reach).ceil().min((target_size - 1) as f32) as usize;
            let min_y = (ay.min(by) - reach).floor().max(0.0) as usize;
            let max_y = (ay.max(by) + reach).ceil().min((target_size - 1) as f32) as usize;
            let vx = bx - ax;
            let vy = by - ay;
            let length_squared = vx * vx + vy * vy;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let t = if length_squared > 1.0e-6 {
                        (((x as f32 - ax) * vx + (y as f32 - ay) * vy) / length_squared)
                            .clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    let distance = ((x as f32 - (ax + vx * t)).powi(2)
                        + (y as f32 - (ay + vy * t)).powi(2))
                    .sqrt();
                    let coverage = if width_pixels < 1.0 {
                        width_pixels * (1.0 - distance).clamp(0.0, 1.0)
                    } else {
                        (width_pixels * 0.5 + 0.5 - distance).clamp(0.0, 1.0)
                    };
                    let index = y * target_size + x;
                    target[index] = target[index].max((coverage * 255.0).round() as u8);
                }
            }
        }
    }
}

fn blend_surface(pixel: &mut Rgba<u8>, color: [f32; 3], alpha: f32) {
    let alpha = alpha.clamp(0.0, 1.0);
    for channel in 0..3 {
        pixel[channel] = (pixel[channel] as f32 * (1.0 - alpha) + color[channel] * alpha)
            .clamp(0.0, 255.0) as u8;
    }
}

fn surface_hash(x: i32, y: i32, salt: u32) -> f32 {
    let mut value = (x as u32)
        .wrapping_mul(0x9e37_79b9)
        .wrapping_add((y as u32).wrapping_mul(0x85eb_ca6b))
        .wrapping_add(salt.wrapping_mul(0xc2b2_ae35));
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^= value >> 16;
    value as f32 / u32::MAX as f32
}

fn farm_parcel_spacing_km(world_x: f32, world_y: f32) -> f32 {
    let region_x = (world_x / 8.0).floor() as i32;
    let region_y = (world_y / 8.0).floor() as i32;
    let farming_system = surface_hash(region_x, region_y, 0x6a31);
    if farming_system < 0.28 {
        0.16 // small irrigated holdings, commonly 1--4 ha
    } else if farming_system < 0.78 {
        0.34 // mixed fields, commonly 5--18 ha
    } else {
        0.68 // mechanised fields, commonly 20--80 ha
    }
}

/// Returns nearest distance, second-nearest distance and stable parcel id.
fn farm_parcel(world_x: f32, world_y: f32) -> (f32, f32, i32, i32) {
    let warped_x = world_x + (world_y * 0.23).sin() * 0.31;
    let warped_y = world_y + (world_x * 0.19).sin() * 0.27;
    let spacing = farm_parcel_spacing_km(world_x, world_y);
    let cell_x = (warped_x / spacing).floor() as i32;
    let cell_y = (warped_y / spacing).floor() as i32;
    let mut nearest = (f32::INFINITY, 0_i32, 0_i32);
    let mut second_distance = f32::INFINITY;
    for candidate_y in cell_y - 2..=cell_y + 2 {
        for candidate_x in cell_x - 2..=cell_x + 2 {
            let seed_x =
                (candidate_x as f32 + 0.12 + surface_hash(candidate_x, candidate_y, 101) * 0.76)
                    * spacing;
            let seed_y =
                (candidate_y as f32 + 0.12 + surface_hash(candidate_x, candidate_y, 137) * 0.76)
                    * spacing;
            let distance = (warped_x - seed_x).hypot(warped_y - seed_y);
            if distance < nearest.0 {
                second_distance = nearest.0;
                nearest = (distance, candidate_x, candidate_y);
            } else if distance < second_distance {
                second_distance = distance;
            }
        }
    }
    (nearest.0, second_distance, nearest.1, nearest.2)
}

fn composite_world_surface(
    image: &mut RgbaImage,
    terrain: &TerrainData,
    infrastructure: &InfrastructureData,
) {
    let width = image.width().max(2);
    let height = image.height().max(2);
    let world_km = infrastructure.cultivated_land.grid.world_size_km;
    for output_y in 0..height {
        for output_x in 0..width {
            let source_x = output_x as usize * (terrain.size - 1) / (width as usize - 1);
            let source_y = output_y as usize * (terrain.size - 1) / (height as usize - 1);
            let index = source_y * terrain.size + source_x;
            let cultivated = infrastructure.cultivated_land.values[index];
            let urban = infrastructure.urban_land.values[index];
            let pixel = image.get_pixel_mut(output_x, output_y);

            if cultivated > 0.07 {
                let world_x = output_x as f32 / (width - 1) as f32 * world_km;
                let world_y = output_y as f32 / (height - 1) as f32 * world_km;
                let (nearest_distance, second_distance, parcel_x, parcel_y) =
                    farm_parcel(world_x, world_y);
                let crop = surface_hash(parcel_x, parcel_y, 19);
                let season = surface_hash(parcel_x, parcel_y, 43);
                let field_color = if crop < 0.22 {
                    [126.0 + season * 28.0, 116.0 + season * 24.0, 62.0]
                } else if crop < 0.55 {
                    [91.0 + season * 24.0, 126.0 + season * 31.0, 63.0]
                } else if crop < 0.80 {
                    [154.0 + season * 24.0, 145.0 + season * 22.0, 76.0]
                } else {
                    [113.0 + season * 18.0, 142.0 + season * 22.0, 85.0]
                };
                let boundary = second_distance - nearest_distance;
                let field_alpha = cultivated.powf(0.72) * 0.76;
                blend_surface(pixel, field_color, field_alpha);
                if boundary < 0.055 {
                    // Hedgerows, drainage ditches and unploughed parcel margins are among the most
                    // legible agricultural signals in satellite imagery.
                    let margin_color = if crop < 0.55 {
                        [54.0, 79.0, 43.0]
                    } else {
                        [82.0, 72.0, 44.0]
                    };
                    blend_surface(pixel, margin_color, cultivated * 0.74);
                }
            }

            if urban > 0.055 {
                let world_x = output_x as f32 / (width - 1) as f32 * world_km;
                let world_y = output_y as f32 / (height - 1) as f32 * world_km;
                let warped_x = world_x + (world_y * 0.43).sin() * 0.075;
                let warped_y = world_y + (world_x * 0.37).sin() * 0.065;
                let block_width = 0.31;
                let block_height = 0.24;
                let block_x = (warped_x / block_width).floor() as i32;
                let block_y = (warped_y / block_height).floor() as i32;
                let roof = surface_hash(block_x, block_y, 71);
                let local_x = (warped_x / block_width).rem_euclid(1.0);
                let local_y = (warped_y / block_height).rem_euclid(1.0);
                let street_distance = local_x.min(1.0 - local_x).min(local_y.min(1.0 - local_y));
                if street_distance < 0.105 {
                    blend_surface(pixel, [91.0, 91.0, 86.0], urban.powf(0.68) * 0.63);
                } else {
                    let urban_color =
                        [108.0 + roof * 61.0, 103.0 + roof * 48.0, 94.0 + roof * 39.0];
                    blend_surface(pixel, urban_color, urban.powf(0.68) * 0.84);
                }
            }
        }
    }

    let scale_x = (width - 1) as f32 / (terrain.size - 1) as f32;
    let scale_y = (height - 1) as f32 / (terrain.size - 1) as f32;
    let metres_per_pixel = world_km * 1000.0 / width.max(height) as f32;
    for road in &infrastructure.roads {
        let width_metres = road.class.profile().carriageway_width_metres;
        let road_color = match road.class {
            infrastructure::RoadClass::Motorway => [145.0, 143.0, 136.0],
            infrastructure::RoadClass::Arterial => [138.0, 136.0, 129.0],
            infrastructure::RoadClass::Collector => [129.0, 127.0, 120.0],
            infrastructure::RoadClass::Local => [124.0, 122.0, 115.0],
            infrastructure::RoadClass::Rural => [118.0, 109.0, 91.0],
        };
        let width_pixels = width_metres / metres_per_pixel;
        let mut centreline: Vec<(f32, f32)> = road
            .path
            .iter()
            .map(|point| (point.x as f32 * scale_x, point.y as f32 * scale_y))
            .collect();
        for _ in 0..3 {
            if centreline.len() < 3 {
                break;
            }
            let mut refined = Vec::with_capacity(centreline.len() * 2);
            refined.push(centreline[0]);
            for points in centreline.windows(2) {
                refined.push((
                    points[0].0 * 0.75 + points[1].0 * 0.25,
                    points[0].1 * 0.75 + points[1].1 * 0.25,
                ));
                refined.push((
                    points[0].0 * 0.25 + points[1].0 * 0.75,
                    points[0].1 * 0.25 + points[1].1 * 0.75,
                ));
            }
            refined.push(*centreline.last().unwrap());
            centreline = refined;
        }
        for points in centreline.windows(2) {
            let from = (points[0].0.round() as i32, points[0].1.round() as i32);
            let to = (points[1].0.round() as i32, points[1].1.round() as i32);
            if width_pixels < 1.0 {
                draw_line_alpha(image, from, to, road_color, width_pixels * 0.72);
            } else {
                let radius = (width_pixels * 0.5).floor().max(0.0) as i32;
                draw_line_width(
                    image,
                    from,
                    to,
                    radius,
                    Rgba([
                        road_color[0] as u8,
                        road_color[1] as u8,
                        road_color[2] as u8,
                        255,
                    ]),
                );
            }
        }
    }
}

fn infrastructure_image(
    terrain: &TerrainData,
    data: &InfrastructureData,
    size: usize,
) -> RgbaImage {
    let mut image = analysis_image(terrain, size, |index| {
        let farms = data.cultivated_land.values[index];
        let hazard = data.hazard.values[index];
        let urban = data.urban_land.values[index];
        [
            (22.0 + farms * 126.0 + hazard * 72.0 + urban * 92.0) as u8,
            (28.0 + farms * 105.0 + urban * 78.0) as u8,
            (30.0 + hazard * 28.0 + urban * 72.0) as u8,
        ]
    });
    let scale = (size - 1) as f32 / (terrain.size - 1) as f32;
    for road in &data.roads {
        let color = match road.class {
            infrastructure::RoadClass::Motorway => Rgba([250, 220, 92, 255]),
            infrastructure::RoadClass::Arterial => Rgba([239, 190, 78, 255]),
            infrastructure::RoadClass::Collector => Rgba([222, 157, 70, 255]),
            infrastructure::RoadClass::Local => Rgba([205, 139, 69, 255]),
            infrastructure::RoadClass::Rural => Rgba([181, 124, 76, 255]),
        };
        for points in road.path.windows(2) {
            let from = (
                (points[0].x as f32 * scale).round() as i32,
                (points[0].y as f32 * scale).round() as i32,
            );
            let to = (
                (points[1].x as f32 * scale).round() as i32,
                (points[1].y as f32 * scale).round() as i32,
            );
            draw_line(&mut image, from, to, color);
        }
    }
    for settlement in &data.settlements {
        let cx = (settlement.location.x as f32 * scale).round() as i32;
        let cy = (settlement.location.y as f32 * scale).round() as i32;
        let radius = match settlement.class {
            infrastructure::SettlementClass::RegionalCentre => 5,
            infrastructure::SettlementClass::Town => 4,
            infrastructure::SettlementClass::Village => 3,
        };
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y <= radius * radius {
                    let xx = cx + x;
                    let yy = cy + y;
                    if xx >= 0 && yy >= 0 && xx < size as i32 && yy < size as i32 {
                        image.put_pixel(xx as u32, yy as u32, Rgba([236, 244, 226, 255]));
                    }
                }
            }
        }
    }
    image
}

fn analysis_previews(
    terrain: &TerrainData,
    infrastructure: &InfrastructureData,
) -> Result<AnalysisPreviews, String> {
    let size = 512_usize.min(terrain.size);
    let max_flow = terrain.flow.iter().copied().fold(1.0_f32, f32::max);
    let flow_log = max_flow.ln_1p();
    let discharge = analysis_image(terrain, size, |index| {
        let value = (terrain.flow[index].ln_1p() / flow_log).clamp(0.0, 1.0);
        [
            (8.0 + value * 42.0) as u8,
            (14.0 + value * 166.0) as u8,
            (22.0 + value * 233.0) as u8,
        ]
    });
    let lake = analysis_image(terrain, size, |index| {
        let value = terrain.lake[index].clamp(0.0, 1.0);
        [
            (10.0 + value * 35.0) as u8,
            (18.0 + value * 135.0) as u8,
            (24.0 + value * 225.0) as u8,
        ]
    });
    let wetland = analysis_image(terrain, size, |index| {
        let value = terrain.wetland[index].clamp(0.0, 1.0);
        [
            (13.0 + value * 76.0) as u8,
            (18.0 + value * 183.0) as u8,
            (20.0 + value * 113.0) as u8,
        ]
    });
    let floodplain = analysis_image(terrain, size, |index| {
        let value = terrain.floodplain[index].clamp(0.0, 1.0);
        [
            (15.0 + value * 214.0) as u8,
            (18.0 + value * 169.0) as u8,
            (20.0 + value * 48.0) as u8,
        ]
    });
    let basin = analysis_image(terrain, size, |index| {
        let id = terrain.basin_id[index];
        let hash = id.wrapping_mul(0x9e37_79b9);
        [
            45 + ((hash >> 16) & 127) as u8,
            45 + ((hash >> 8) & 127) as u8,
            45 + (hash & 127) as u8,
        ]
    });
    let river_order = analysis_image(terrain, size, |index| {
        let value = terrain.river_order[index].min(8) as f32 / 8.0;
        [
            (12.0 + value * 48.0) as u8,
            (17.0 + value * 186.0) as u8,
            (22.0 + value * 229.0) as u8,
        ]
    });
    let geology = analysis_image(terrain, size, |index| {
        let base: [f32; 3] = match terrain.lithology[index] {
            terrain_core::geology::Lithology::Shale => [82.0, 77.0, 69.0],
            terrain_core::geology::Lithology::Sandstone => [174.0, 137.0, 91.0],
            terrain_core::geology::Lithology::Limestone => [190.0, 187.0, 165.0],
            terrain_core::geology::Lithology::Granite => [151.0, 142.0, 137.0],
            terrain_core::geology::Lithology::Basalt => [58.0, 66.0, 70.0],
            terrain_core::geology::Lithology::Metamorphic => [112.0, 106.0, 104.0],
        };
        let within_bed = terrain.stratigraphic_phase[index].rem_euclid(1.0);
        let contact_distance = within_bed.min(1.0 - within_bed);
        let contact = (1.0 - (contact_distance / 0.075).clamp(0.0, 1.0)) * 0.22;
        let fault = terrain.fracture_intensity[index] * 0.18;
        let tone = 1.0 - contact - fault;
        [
            (base[0] * tone) as u8,
            (base[1] * tone) as u8,
            (base[2] * tone) as u8,
        ]
    });
    let soil_depth = analysis_image(terrain, size, |index| {
        let value = terrain.soil_depth[index].clamp(0.0, 1.0);
        [
            (53.0 + value * 92.0) as u8,
            (43.0 + value * 105.0) as u8,
            (35.0 + value * 42.0) as u8,
        ]
    });
    let land_cover = analysis_image(terrain, size, |index| {
        if terrain.water[index] > 0.35 {
            return [25, 88, 125];
        }
        if terrain.snow[index] > 0.45 {
            return [220, 232, 234];
        }
        let covers = [
            (terrain.forest[index], [28, 92, 48]),
            (terrain.grassland[index], [125, 158, 66]),
            (terrain.shrubland[index], [158, 139, 69]),
            (terrain.bare_ground[index], [142, 124, 106]),
            (terrain.sediment[index], [204, 174, 105]),
        ];
        covers
            .into_iter()
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, color)| color)
            .unwrap_or([45, 48, 45])
    });
    let travel_cost = analysis_image(terrain, size, |index| {
        let value = infrastructure.travel_cost.values[index];
        [
            (24.0 + value * 218.0) as u8,
            (35.0 + (1.0 - value) * 155.0) as u8,
            (31.0 + (1.0 - value) * 72.0) as u8,
        ]
    });
    let hazard = analysis_image(terrain, size, |index| {
        let value = infrastructure.hazard.values[index];
        [
            (24.0 + value * 224.0) as u8,
            (26.0 + value * 69.0) as u8,
            (31.0 + value * 35.0) as u8,
        ]
    });
    let settlement_suitability = analysis_image(terrain, size, |index| {
        let value = infrastructure.settlement_suitability.values[index];
        [
            (20.0 + value * 112.0) as u8,
            (27.0 + value * 211.0) as u8,
            (32.0 + value * 93.0) as u8,
        ]
    });
    let agricultural_suitability = analysis_image(terrain, size, |index| {
        let value = infrastructure.agricultural_suitability.values[index];
        [
            (27.0 + value * 190.0) as u8,
            (29.0 + value * 180.0) as u8,
            (29.0 + value * 45.0) as u8,
        ]
    });
    let cultivated_land = analysis_image(terrain, size, |index| {
        let value = infrastructure.cultivated_land.values[index];
        [
            (27.0 + value * 190.0) as u8,
            (31.0 + value * 154.0) as u8,
            (29.0 + value * 58.0) as u8,
        ]
    });
    let urban_land = analysis_image(terrain, size, |index| {
        let value = infrastructure.urban_land.values[index];
        [
            (30.0 + value * 205.0) as u8,
            (32.0 + value * 188.0) as u8,
            (35.0 + value * 157.0) as u8,
        ]
    });
    let infrastructure_map = infrastructure_image(terrain, infrastructure, size);
    Ok(AnalysisPreviews {
        discharge: image_data_url(&discharge)?,
        lake: image_data_url(&lake)?,
        wetland: image_data_url(&wetland)?,
        floodplain: image_data_url(&floodplain)?,
        basin: image_data_url(&basin)?,
        river_order: image_data_url(&river_order)?,
        geology: image_data_url(&geology)?,
        soil_depth: image_data_url(&soil_depth)?,
        land_cover: image_data_url(&land_cover)?,
        travel_cost: image_data_url(&travel_cost)?,
        hazard: image_data_url(&hazard)?,
        settlement_suitability: image_data_url(&settlement_suitability)?,
        agricultural_suitability: image_data_url(&agricultural_suitability)?,
        urban_land: image_data_url(&urban_land)?,
        cultivated_land: image_data_url(&cultivated_land)?,
        infrastructure: image_data_url(&infrastructure_map)?,
    })
}

#[tauri::command]
async fn generate_terrain(
    app: AppHandle,
    config: SimulationConfig,
) -> Result<GenerationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let started = Instant::now();
        let terrain = generate(&config, |progress, stage| {
            emit_progress(&app, progress, stage)
        })
        .map_err(|error| error.to_string())?;
        let stats = terrain.stats.clone();
        emit_progress(&app, 0.90, "计算聚落、农田与道路网络");
        let infrastructure =
            generate_infrastructure(&terrain, &config).map_err(|error| error.to_string())?;
        let infrastructure_summary = InfrastructureSummary {
            settlements: infrastructure.settlements.len(),
            roads: infrastructure.roads.len(),
            bridges: infrastructure
                .crossings
                .iter()
                .filter(|crossing| crossing.kind == CrossingKind::Bridge)
                .count(),
            tunnels: infrastructure
                .crossings
                .iter()
                .filter(|crossing| crossing.kind == CrossingKind::Tunnel)
                .count(),
        };
        let analysis_previews = analysis_previews(&terrain, &infrastructure)?;
        let mesh_size = 512_usize.min(terrain.size);
        let height_bytes: Vec<u8> = downsample_height(&terrain, mesh_size)
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect();
        let mut forest_bytes = Vec::with_capacity(mesh_size * mesh_size);
        for y in 0..mesh_size {
            for x in 0..mesh_size {
                let sx = x * (terrain.size - 1) / (mesh_size - 1);
                let sy = y * (terrain.size - 1) / (mesh_size - 1);
                forest_bytes
                    .push((terrain.forest[sy * terrain.size + sx].clamp(0.0, 1.0) * 255.0) as u8);
            }
        }
        let vegetation_exclusion_bytes = vegetation_exclusion_mask(&infrastructure, mesh_size);
        let (urban_bytes, cultivated_bytes, crop_bytes, road_bytes) =
            infrastructure_surface_masks(&infrastructure, mesh_size);
        let max_flow = terrain.flow.iter().copied().fold(1.0_f32, f32::max);
        let max_flow_log = max_flow.ln_1p();
        let mut water_height_bytes = Vec::with_capacity(mesh_size * mesh_size * 4);
        for y in 0..mesh_size {
            for x in 0..mesh_size {
                let sx = x * (terrain.size - 1) / (mesh_size - 1);
                let sy = y * (terrain.size - 1) / (mesh_size - 1);
                let index = sy * terrain.size + sx;
                let open_ocean = terrain.height[index] < 58.0;
                let lake = terrain.lake[index] > 0.05;
                let surface_height = if open_ocean {
                    58.0
                } else if lake {
                    terrain.filled_height[index]
                } else {
                    terrain.height[index] + 0.35
                };
                water_height_bytes.extend_from_slice(&surface_height.to_le_bytes());
            }
        }
        let water_data_size = 512_usize.min(terrain.size);
        let mut water_mask_bytes = Vec::with_capacity(water_data_size * water_data_size);
        let mut water_kind_bytes = Vec::with_capacity(water_data_size * water_data_size);
        let mut flow_direction_bytes = Vec::with_capacity(water_data_size * water_data_size * 2);
        let mut flow_strength_bytes = Vec::with_capacity(water_data_size * water_data_size);
        for y in 0..water_data_size {
            for x in 0..water_data_size {
                let sx = x * (terrain.size - 1) / (water_data_size - 1);
                let sy = y * (terrain.size - 1) / (water_data_size - 1);
                let index = sy * terrain.size + sx;
                let open_ocean = terrain.height[index] < 58.0;
                let lake = terrain.lake[index] > 0.05;
                water_mask_bytes.push((terrain.water[index].clamp(0.0, 1.0) * 255.0) as u8);
                water_kind_bytes.push(if open_ocean {
                    255
                } else if lake {
                    128
                } else {
                    0
                });
                flow_direction_bytes
                    .push(((terrain.flow_direction_x[index].clamp(-1.0, 1.0) * 127.0) as i8) as u8);
                flow_direction_bytes
                    .push(((terrain.flow_direction_y[index].clamp(-1.0, 1.0) * 127.0) as i8) as u8);
                flow_strength_bytes.push(
                    ((terrain.flow[index].ln_1p() / max_flow_log).clamp(0.0, 1.0) * 255.0) as u8,
                );
            }
        }
        let preview_size = 1024_usize.min(config.grid_size.max(512));
        let mut image = render_satellite(&terrain, &config, preview_size, |progress, stage| {
            emit_progress(&app, progress, stage)
        })
        .map_err(|error| error.to_string())?;
        composite_world_surface(&mut image, &terrain, &infrastructure);
        let mut png = Cursor::new(Vec::new());
        image
            .write_to(&mut png, ImageFormat::Png)
            .map_err(|error| error.to_string())?;
        emit_progress(&app, 1.0, "卫星影像生成完成");
        Ok(GenerationResult {
            preview_data_url: format!(
                "data:image/png;base64,{}",
                STANDARD.encode(png.into_inner())
            ),
            width: image.width(),
            height: image.height(),
            world_size_km: config.world_size_km,
            elapsed_ms: started.elapsed().as_millis(),
            stats,
            mesh_size,
            water_data_size,
            height_data_base64: STANDARD.encode(height_bytes),
            forest_data_base64: STANDARD.encode(forest_bytes),
            vegetation_exclusion_data_base64: STANDARD.encode(vegetation_exclusion_bytes),
            urban_data_base64: STANDARD.encode(urban_bytes),
            cultivated_data_base64: STANDARD.encode(cultivated_bytes),
            crop_data_base64: STANDARD.encode(crop_bytes),
            road_data_base64: STANDARD.encode(road_bytes),
            roads: render_roads(&infrastructure),
            cities: serde_json::to_value(&infrastructure.cities)
                .map_err(|error| error.to_string())?,
            water_height_data_base64: STANDARD.encode(water_height_bytes),
            water_mask_base64: STANDARD.encode(water_mask_bytes),
            water_kind_base64: STANDARD.encode(water_kind_bytes),
            flow_direction_base64: STANDARD.encode(flow_direction_bytes),
            flow_strength_base64: STANDARD.encode(flow_strength_bytes),
            analysis_previews,
            infrastructure_summary,
        })
    })
    .await
    .map_err(|error| format!("native generation task failed: {error}"))?
}

#[tauri::command]
async fn export_terrain(
    app: AppHandle,
    config: SimulationConfig,
    output_path: String,
    output_size: usize,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let terrain = generate(&config, |progress, stage| {
            emit_progress(&app, progress * 0.82, stage)
        })
        .map_err(|error| error.to_string())?;
        let infrastructure =
            generate_infrastructure(&terrain, &config).map_err(|error| error.to_string())?;
        let mut image = render_satellite(&terrain, &config, output_size, |progress, stage| {
            emit_progress(&app, 0.82 + (progress - 0.90).max(0.0) * 1.8, stage)
        })
        .map_err(|error| error.to_string())?;
        composite_world_surface(&mut image, &terrain, &infrastructure);
        let path = PathBuf::from(&output_path);
        save_png(&image, &path).map_err(|error| error.to_string())?;
        emit_progress(&app, 1.0, "高分辨率影像导出完成");
        Ok(path.to_string_lossy().into_owned())
    })
    .await
    .map_err(|error| format!("native export task failed: {error}"))?
}

#[tauri::command]
async fn save_project(project: ProjectDocument, output_path: String) -> Result<String, String> {
    if project.schema_version != 1 {
        return Err("unsupported project schema version".to_owned());
    }
    let path = PathBuf::from(&output_path);
    let serialized = serde_json::to_vec_pretty(&project).map_err(|error| error.to_string())?;
    std::fs::write(&path, serialized).map_err(|error| error.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
async fn load_project(input_path: String) -> Result<ProjectDocument, String> {
    let bytes = std::fs::read(input_path).map_err(|error| error.to_string())?;
    let project: ProjectDocument =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    if project.schema_version != 1 {
        return Err(format!(
            "unsupported project schema version: {}",
            project.schema_version
        ));
    }
    Ok(project)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            generate_terrain,
            export_terrain,
            save_project,
            load_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running wind & water terrain lab");
}

#[cfg(test)]
mod tests {
    use super::*;
    use terrain_core::{Landform, TerrainPreset};

    #[test]
    fn subpixel_road_mask_preserves_physical_coverage() {
        let config = SimulationConfig {
            seed: 42,
            preset: TerrainPreset::Temperate,
            landform: Landform::Coastal,
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
        };
        let terrain = generate(&config, |_, _| {}).unwrap();
        let infrastructure = generate_infrastructure(&terrain, &config).unwrap();
        let size = 512;
        let mut mask = vec![0_u8; size * size];
        rasterize_road_coverage(
            &mut mask,
            size,
            terrain.size,
            config.world_size_km,
            &infrastructure.roads,
            false,
        );
        let metres_per_pixel = config.world_size_km * 1000.0 / (size - 1) as f32;
        let raster_area =
            mask.iter().map(|value| *value as f32 / 255.0).sum::<f32>() * metres_per_pixel.powi(2);
        let physical_area = infrastructure
            .roads
            .iter()
            .map(|road| road.length_km * 1000.0 * road.class.profile().carriageway_width_metres)
            .sum::<f32>();
        let ratio = raster_area / physical_area;
        assert!(
            (0.55..=1.65).contains(&ratio),
            "raster/physical road area ratio {ratio}"
        );
        assert!(
            mask.iter().copied().max().unwrap_or(0) < 128,
            "a sub-pixel road must not become a full terrain pixel"
        );
    }
}
