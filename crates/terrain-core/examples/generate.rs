use std::{env, time::Instant};
use terrain_core::{
    Landform, SimulationConfig, TerrainPreset, generate, render_satellite, save_png,
};

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "terrain-preview.png".to_owned());
    let size = env::args()
        .nth(2)
        .and_then(|value| value.parse().ok())
        .unwrap_or(1024);
    let landform = match env::args().nth(3).as_deref() {
        Some("hills") => Landform::Hills,
        Some("plains") => Landform::Plains,
        Some("plateau") => Landform::Plateau,
        Some("coastal") => Landform::Coastal,
        Some("archipelago") => Landform::Archipelago,
        _ => Landform::MountainRange,
    };
    let config = SimulationConfig {
        seed: 284735,
        preset: TerrainPreset::Temperate,
        landform,
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
    let started = Instant::now();
    let terrain = generate(&config, |progress, stage| {
        eprintln!("{:>3}% {stage}", (progress * 100.0) as u32)
    })
    .expect("terrain generation failed");
    let image = render_satellite(&terrain, &config, size, |progress, stage| {
        eprintln!("{:>3}% {stage}", (progress * 100.0) as u32)
    })
    .expect("satellite rendering failed");
    save_png(&image, &path).expect("image export failed");
    println!("saved {path} in {:.2}s", started.elapsed().as_secs_f32());
}
