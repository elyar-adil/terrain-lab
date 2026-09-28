//! Dump a full generation payload (terrain + infrastructure + city scenes) as
//! the render-harness fixture, so the browser harness can screenshot the real
//! product renderer without launching the desktop shell.
//!
//! Usage: cargo run --release -p wind-water-terrain-lab --example city_fixture
//!        [--cities N] [--radius-km R]

use std::env;
use std::path::PathBuf;
use city_scene::CityScene;
use terrain_core::{Landform, SimulationConfig, TerrainPreset};

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let index = args.iter().position(|value| value == name)?;
    args.get(index + 1).cloned()
}

fn public_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("public")
}

/// Print what a city scene actually contains.
///
/// This is the only way to tell a real defect from an imagined one: every claim
/// about draw calls, tree counts or junction geometry is checked here against the
/// scene that was built, so a regression is a changed number rather than a
/// changed opinion.
fn report_scene(index: usize, scene: &CityScene) {
    let s = &scene.stats;
    println!(
        "  city {index}: {} draws, {} tris, {} verts, {} buildings, {} trees/{} instances, \
         {} vehicles, {} junctions, {} connectors, {} signals, {} parked",
        s.draw_calls,
        s.triangles,
        s.vertices,
        s.buildings,
        s.trees,
        s.tree_instances,
        s.vehicles,
        s.junctions,
        s.connectors,
        s.signals,
        s.parked_cars,
    );
    println!(
        "    extent {:.0}..{:.0} x {:.0}..{:.0} m, roles {:?}",
        scene.extent_m[0], scene.extent_m[2], scene.extent_m[1], scene.extent_m[3], s.tree_roles,
    );
    for warning in &s.warnings {
        println!("    warning: {warning}");
    }
    for stopped in &scene.traffic.stalled {
        println!("    stalled: {stopped}");
    }
    // Where the bytes are, largest first.
    let mut meshes: Vec<(&str, usize, usize)> = scene
        .meshes
        .iter()
        .map(|mesh| {
            let bytes = mesh.positions.len() + mesh.normals.len() + mesh.indices.len();
            (mesh.material.as_str(), mesh.vertex_count, bytes)
        })
        .collect();
    meshes.sort_by_key(|(_, _, bytes)| std::cmp::Reverse(*bytes));
    for (material, vertices, bytes) in meshes.iter().take(10) {
        println!(
            "      {material:<24} {vertices:>8} verts {:>7.2} MB",
            *bytes as f32 / 1_048_576.0
        );
    }
    println!(
        "    textures: {}",
        scene
            .textures
            .iter()
            .map(|texture| format!(
                "{} {}x{} @ {:.1}x{:.1}m",
                texture.name,
                texture.width,
                texture.height,
                texture.tile_width_m,
                texture.tile_height_m
            ))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SimulationConfig {
        seed: 284_735,
        preset: TerrainPreset::Temperate,
        landform: Landform::Plains,
        grid_size: 512,
        world_size_km: 40.0,
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
    let keep = flag("--cities")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(3);
    // `--scene-only` writes just the city scene, with no terrain payload.
    //
    // The full fixture is dominated by the heightfield, the water stack and the
    // sixteen analysis rasters, which are hundreds of megabytes and several
    // minutes under software WebGL — for a city that is a flat local frame, all
    // of it is dead weight.  This is what the city viewer loads, so iterating on
    // the city costs seconds instead of minutes.
    let scene_only = env::args().any(|argument| argument == "--scene-only");
    if scene_only {
        let (_result, infrastructure) =
            wind_water_terrain_lab_lib::build_payload(config.clone(), &|_, _| {})?;
        let scenes: Vec<_> = infrastructure
            .city_scenes
            .iter()
            .take(keep)
            .cloned()
            .collect();
        for (index, scene) in scenes.iter().enumerate() {
            report_scene(index, scene);
        }
        let bytes = serde_json::to_vec(&scenes)?;
        let path = public_dir().join("city-scenes.json");
        std::fs::write(&path, &bytes)?;
        println!(
            "scene-only: {} cities, {} ({:.1} MB)",
            scenes.len(),
            path.display(),
            bytes.len() as f32 / 1_048_576.0
        );
        return Ok(());
    }
    let (result, infrastructure) =
        wind_water_terrain_lab_lib::build_payload(config.clone(), &|_, _| {})?;
    // Work on the serialised form so the fixture is byte-identical to what the
    // desktop command sends, and so truncating the city list also truncates the
    // scene list the renderer actually reads.  `GenerationResult` is camelCase.
    let mut payload = serde_json::to_value(&result)?;
    for key in ["modernCities", "cities"] {
        if let Some(list) = payload
            .get_mut(key)
            .and_then(|value| value.as_array_mut())
        {
            list.truncate(keep);
        }
    }
    let fixture = serde_json::json!({ "config": config, "result": payload });
    let path = public_dir().join("render-fixture.json");
    let bytes = serde_json::to_vec(&fixture)?;
    std::fs::write(&path, &bytes)?;
    println!(
        "cities={keep} fixture={} ({:.1} MB)",
        path.display(),
        bytes.len() as f32 / 1_048_576.0
    );
    // Attribute the payload to a field rather than guessing at it.
    if let Some(object) = payload.as_object() {
        let mut report: Vec<(String, usize)> = object
            .iter()
            .map(|(key, value)| (key.clone(), serde_json::to_vec(value).map(|v| v.len()).unwrap_or(0)))
            .collect();
        report.sort_by_key(|(_, size)| std::cmp::Reverse(*size));
        for (key, size) in report.iter().take(12) {
            println!("  {key:<32} {:>8.1} MB", *size as f32 / 1_048_576.0);
        }
    }
    for (index, scene) in infrastructure.city_scenes.iter().take(keep).enumerate() {
        report_scene(index, scene);
    }
    Ok(())
}
