//! Writes the car prototypes as one JSON file for the standalone viewer
//! (`scripts/car-viewer`): every part of every design, positions, normals and
//! indices, so a car can be judged alone and close without building a city.
//!   cargo run --release -p city-scene --example dump_cars -- cars.json [fleet-seed]
use std::fmt::Write as _;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| "cars.json".to_owned());
    let fleet: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let started = std::time::Instant::now();
    let mut builder = city_scene::MeshBuilder::new();
    city_scene::car::build_prototypes(&mut builder, fleet);
    let elapsed = started.elapsed();
    let scene = builder.build();
    let mut out = String::from("{\"variants\":");
    write!(out, "{},\"tints\":[", city_scene::car::CAR_VARIANTS).unwrap();
    for variant in 0..city_scene::car::CAR_VARIANTS {
        if variant > 0 {
            out.push(',');
        }
        out.push('{');
        for (index, (part, tint)) in city_scene::car::part_tints(fleet, variant)
            .iter()
            .enumerate()
        {
            if index > 0 {
                out.push(',');
            }
            write!(out, "\"{part}\":[{},{},{}]", tint[0], tint[1], tint[2]).unwrap();
        }
        out.push('}');
    }
    out.push_str("],\"meshes\":{");
    let mut total = 0;
    let mut bytes = 0;
    let mut first = true;
    for mesh in &scene.meshes {
        if !mesh.material.starts_with("car/") {
            continue;
        }
        if !first {
            out.push(',');
        }
        first = false;
        let join = |values: &mut dyn Iterator<Item = String>| values.collect::<Vec<_>>().join(",");
        write!(
            out,
            "\"{}\":{{\"p\":[{}],\"n\":[{}],\"i\":[{}]}}",
            mesh.material,
            join(&mut mesh.positions.iter().map(|v| format!("{v:.5}"))),
            join(&mut mesh.normals.iter().map(|v| format!("{v:.4}"))),
            join(&mut mesh.indices.iter().map(|v| v.to_string())),
        )
        .unwrap();
        bytes += mesh.positions.len() * 8 + mesh.indices.len() * 4;
        if !mesh.material.contains("far.") {
            total += mesh.indices.len() / 3;
        }
        eprintln!(
            "{:28} {:7} triangles",
            mesh.material,
            mesh.indices.len() / 3
        );
    }
    out.push_str("}}");
    eprintln!(
        "near models: {} triangles in all, {} per design; {:.1} MB of buffers; built in {elapsed:.0?}",
        total,
        total / city_scene::car::CAR_VARIANTS,
        bytes as f32 / 1.0e6
    );
    std::fs::write(&path, out).expect("write json");
}
