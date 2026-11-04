//! Prints a generated world's statistics and counts, to compare a native build
//! with the browser's WebAssembly build (scripts/web-smoke.mjs prints the same).
//!   cargo run --release -p app-backend --example payload_stats -- [grid] [seed]
use terrain_core::{Landform, SimulationConfig, TerrainPreset};

fn main() {
    let mut args = std::env::args().skip(1);
    let grid: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(512);
    let seed: u32 = args.next().and_then(|v| v.parse().ok()).unwrap_or(284735);
    let config = SimulationConfig {
        seed,
        preset: TerrainPreset::Temperate,
        landform: Landform::MountainRange,
        grid_size: grid,
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
    let started = std::time::Instant::now();
    let result = app_backend::build_generation_result(config, &|_, _| {}).unwrap();
    println!("generate {grid}: {:.1} s", started.elapsed().as_secs_f32());
    println!("stats {}", serde_json::to_string(&result.stats).unwrap());
}
