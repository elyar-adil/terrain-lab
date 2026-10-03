//! Render a hillshade of a generated landscape, with or without the fluvial
//! evolution pass, to judge whether it looks like terrain.
//!
//! `cargo run --release -p terrain-core --example erosion_probe -- out.png [off] [landform] [grid]`
use image::{GrayImage, Luma};
use std::{env, time::Instant};
use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate, set_fluvial_erosion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "probe.png".into());
    let off = args.get(2).map(|s| s == "off").unwrap_or(false);
    let landform = match args.get(3).map(String::as_str) {
        Some("hills") => Landform::Hills,
        Some("plateau") => Landform::Plateau,
        Some("coastal") => Landform::Coastal,
        _ => Landform::MountainRange,
    };
    let grid: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(512);
    set_fluvial_erosion(!off);
    let config = SimulationConfig {
        seed: 284_735,
        preset: TerrainPreset::Temperate,
        landform,
        grid_size: grid,
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
    let started = Instant::now();
    let terrain = generate(&config, |_, _| {})?;
    println!("generate {:?} (fluvial {})", started.elapsed(), !off);
    let n = terrain.size;
    let cell = config.world_size_km * 1000.0 / n as f32;
    let (min, max) = terrain
        .height
        .iter()
        .fold((f32::MAX, f32::MIN), |(a, b), &h| (a.min(h), b.max(h)));
    println!("height {min:.0}..{max:.0} m, cell {cell:.0} m");
    // Sun from the north-west, 30 degrees up, like the renderer's.
    let (lx, ly, lz) = (-0.55_f32, -0.66, 0.52);
    let norm = (lx * lx + ly * ly + lz * lz).sqrt();
    let mut img = GrayImage::new(n as u32, n as u32);
    for y in 1..n - 1 {
        for x in 1..n - 1 {
            let h = |dx: isize, dy: isize| {
                terrain.height[(y as isize + dy) as usize * n + (x as isize + dx) as usize]
            };
            let gx = (h(1, 0) - h(-1, 0)) / (2.0 * cell);
            let gy = (h(0, 1) - h(0, -1)) / (2.0 * cell);
            let (nx, ny, nz) = (-gx, -gy, 1.0);
            let nl = (nx * nx + ny * ny + nz * nz).sqrt();
            let lambert = ((nx * lx + ny * ly + nz * lz) / (nl * norm)).max(0.0);
            img.put_pixel(x as u32, y as u32, Luma([(30.0 + 225.0 * lambert) as u8]));
        }
    }
    img.save(&out)?;
    println!("wrote {out}");
    Ok(())
}
