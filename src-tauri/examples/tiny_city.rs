//! Dump one small city scene on its own, so a single element can be looked at.
//!
//! The full fixture is dominated by the terrain payload, and a 1.8 km city
//! scene is over a hundred megabytes of vertex data — neither is a usable
//! feedback loop when the question is "does this arrow stencil read as an
//! arrow".  This writes the smallest city that still contains every element the
//! scene layer produces: a signalised arterial, a crosswalk, a tree avenue, a
//! tower, a roundabout, compound walls and a traffic fleet.
//!
//! Usage: cargo run --release -p wind-water-terrain-lab --example tiny_city
//!        [--radius-km 0.4] [--block-m 120] [--out public/tiny-city.json]

use std::env;
use std::path::PathBuf;

use city_scene::{SceneBudget, build_city_scene};
use urban::{ModernChinaSpec, generate_modern_chinese_city};

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let index = args.iter().position(|value| value == name)?;
    args.get(index + 1).cloned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let radius_km = flag("--radius-km")
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.42);
    let block = flag("--block-m")
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(120.0);
    let seed = flag("--seed")
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(42);
    let out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("public")
        .join(
            flag("--out")
                .map(|value| PathBuf::from(value))
                .unwrap_or_else(|| PathBuf::from("tiny-city.json")),
        );

    let city = generate_modern_chinese_city(ModernChinaSpec {
        seed,
        radius_km,
        block_size_metres: block,
        ..ModernChinaSpec::default()
    });
    println!(
        "plan: {} blocks, {} parcels, {} buildings, {} hd roads, {} sd roads",
        city.blocks.len(),
        city.parcels.len(),
        city.buildings.len(),
        city.hd_roads.len(),
        city.sd_roads.len()
    );

    let scene = build_city_scene(&city, SceneBudget::default());
    let s = &scene.stats;
    println!(
        "scene: {} draws, {} tris, {} verts | {} buildings, {} tree instances, \
         {} vehicles, {} junctions, {} connectors, {} signals, {} parked cars",
        s.draw_calls,
        s.triangles,
        s.vertices,
        s.buildings,
        s.tree_instances,
        s.vehicles,
        s.junctions,
        s.connectors,
        s.signals,
        s.parked_cars,
    );
    println!(
        "extent {:.0}..{:.0} x {:.0}..{:.0} m; tree roles {:?}",
        scene.extent_m[0], scene.extent_m[2], scene.extent_m[1], scene.extent_m[3], s.tree_roles,
    );
    for warning in &s.warnings {
        println!("  warning: {warning}");
    }
    for stalled in &scene.traffic.stalled {
        println!("  stalled: {stalled}");
    }

    let mut meshes: Vec<(&str, usize, usize)> = scene
        .meshes
        .iter()
        .map(|mesh| {
            let bytes = mesh.positions.len() + mesh.normals.len() + mesh.indices.len();
            (mesh.material.as_str(), mesh.vertex_count, bytes)
        })
        .collect();
    meshes.sort_by_key(|(_, _, bytes)| std::cmp::Reverse(*bytes));
    println!("largest groups:");
    for (material, vertices, bytes) in meshes.iter().take(12) {
        println!(
            "  {material:<26} {vertices:>8} verts {:>7.2} MB",
            *bytes as f32 / 1_048_576.0
        );
    }

    let bytes = serde_json::to_vec(&scene)?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&out, &bytes)?;
    println!(
        "wrote {} ({:.2} MB)",
        out.display(),
        bytes.len() as f32 / 1_048_576.0
    );
    Ok(())
}
