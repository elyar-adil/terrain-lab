use image::{Rgba, RgbaImage};
use infrastructure::{RoadClass, SettlementClass, generate_infrastructure};
use std::{env, error::Error, path::PathBuf};
use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate};

fn line(image: &mut RgbaImage, from: (i32, i32), to: (i32, i32), color: Rgba<u8>) {
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

fn main() -> Result<(), Box<dyn Error>> {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("infrastructure.png"));
    let config = SimulationConfig {
        seed: 284_735,
        preset: TerrainPreset::Temperate,
        landform: Landform::Plains,
        grid_size: 512,
        world_size_km: 80.0,
        rainfall: 1275.0,
        evaporation: 600.0,
        wind_speed: 7.0,
        wind_direction: 225.0,
        sun_azimuth: 235.0,
        sun_elevation: 42.0,
        haze: 2.5,
        cloud_coverage: 35.0,
        cloud_speed: 24.0,
    };
    let terrain = generate(&config, |_, _| {})?;
    let data = generate_infrastructure(&terrain, &config)?;
    let size = terrain.size;
    let mut image = RgbaImage::new(size as u32, size as u32);
    let mut urban_diagnostic = RgbaImage::new(size as u32, size as u32);
    for index in 0..size * size {
        let water = terrain.water[index];
        let farms = data.cultivated_land.values[index];
        let suitability = data.settlement_suitability.values[index];
        let risk = data.hazard.values[index];
        let urban = data.urban_land.values[index];
        let color = if water > 0.35 {
            [25, 76, 103]
        } else {
            [
                (29.0 + farms * 150.0 + risk * 38.0 + urban * 110.0) as u8,
                (35.0 + farms * 118.0 + suitability * 48.0 + urban * 96.0) as u8,
                (31.0 + suitability * 32.0 + urban * 86.0) as u8,
            ]
        };
        image.put_pixel(
            (index % size) as u32,
            (index / size) as u32,
            Rgba([color[0], color[1], color[2], 255]),
        );
        urban_diagnostic.put_pixel(
            (index % size) as u32,
            (index / size) as u32,
            Rgba([
                (18.0 + farms * 25.0 + urban * 211.0) as u8,
                (23.0 + farms * 35.0 + urban * 197.0) as u8,
                (26.0 + farms * 18.0 + urban * 176.0) as u8,
                255,
            ]),
        );
    }
    for road in &data.roads {
        let color = match road.class {
            RoadClass::Motorway => Rgba([252, 220, 91, 255]),
            RoadClass::Arterial => Rgba([242, 191, 78, 255]),
            RoadClass::Collector => Rgba([225, 152, 66, 255]),
            RoadClass::Local => Rgba([205, 137, 68, 255]),
            RoadClass::Rural => Rgba([181, 121, 74, 255]),
        };
        for pair in road.path.windows(2) {
            line(
                &mut image,
                (pair[0].x as i32, pair[0].y as i32),
                (pair[1].x as i32, pair[1].y as i32),
                color,
            );
            line(
                &mut urban_diagnostic,
                (pair[0].x as i32, pair[0].y as i32),
                (pair[1].x as i32, pair[1].y as i32),
                color,
            );
        }
    }
    for site in &data.settlements {
        let radius = match site.class {
            SettlementClass::RegionalCentre => 6,
            SettlementClass::Town => 4,
            SettlementClass::Village => 3,
        };
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y <= radius * radius {
                    let xx = site.location.x as i32 + x;
                    let yy = site.location.y as i32 + y;
                    if xx >= 0 && yy >= 0 && xx < size as i32 && yy < size as i32 {
                        image.put_pixel(xx as u32, yy as u32, Rgba([245, 245, 232, 255]));
                    }
                }
            }
        }
    }
    image.save(&output)?;
    let diagnostic_name = format!(
        "{}-urban.{}",
        output
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("infrastructure"),
        output
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("png")
    );
    let diagnostic_path = output.with_file_name(diagnostic_name);
    urban_diagnostic.save(&diagnostic_path)?;
    let farm_coverage = data
        .cultivated_land
        .values
        .iter()
        .filter(|value| **value > 0.25)
        .count() as f32
        / data.cultivated_land.values.len() as f32;
    let urban_coverage = data
        .urban_land
        .values
        .iter()
        .filter(|value| **value > 0.18)
        .count() as f32
        / data.urban_land.values.len() as f32;
    println!(
        "settlements={} roads={} crossings={} farms={:.1}% urban={:.2}% saved={}",
        data.settlements.len(),
        data.roads.len(),
        data.crossings.len(),
        farm_coverage * 100.0,
        urban_coverage * 100.0,
        output.display()
    );
    Ok(())
}
