//! Write one city scene as the JSON array `city-harness.html` loads, without
//! needing the Tauri crate (and its native webview dependencies) to build.
//!
//! cargo run --release -p city-scene --example dump_scene -- \
//!     [--radius-km 0.5] [--block-m 110] [--seed 42] [--out public/city-scenes.json]
//!     [--bin city.bin]   also write the binary container the desktop app streams
//!     [--lite]           small budgets, for headless software rendering
//!     [--plan plan.json] write the city plan itself and stop, for checking a layout
//!     [--from plan.json] build from a plan written elsewhere (`plan_svg` with
//!                        `PLAN_JSON`: the towns the game builds)

use city_scene::{SceneBudget, build_city_scene};
use urban::{ModernChinaSpec, generate_modern_chinese_city};

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let index = args.iter().position(|value| value == name)?;
    args.get(index + 1).cloned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let radius_km = flag("--radius-km")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.5_f32);
    let block = flag("--block-m")
        .and_then(|v| v.parse().ok())
        .unwrap_or(110.0_f32);
    let seed = flag("--seed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(42_u32);
    let out = flag("--out").unwrap_or_else(|| "public/city-scenes.json".into());
    // `--fixture plan.json --city i`: the plan of city i from a render fixture, so the
    // tiles match the far form the terrain view draws for that same city.
    let city = match (flag("--from"), flag("--fixture")) {
        (Some(path), _) => serde_json::from_slice::<urban::ModernCity>(&std::fs::read(path)?)?,
        (None, fixture) => match fixture {
            Some(path) => {
                let index: usize = flag("--city").and_then(|v| v.parse().ok()).unwrap_or(0);
                let document: serde_json::Value = serde_json::from_slice(&std::fs::read(&path)?)?;
                let plan = document["result"]["modernCities"][index].clone();
                serde_json::from_value::<urban::ModernCity>(plan)?
            }
            None => generate_modern_chinese_city(ModernChinaSpec {
                seed,
                radius_km,
                block_size_metres: block,
                ..ModernChinaSpec::default()
            }),
        },
    };
    let budget = if std::env::args().any(|a| a == "--lite") {
        SceneBudget {
            max_buildings: 150,
            max_trees: 120,
            facade_texture_size: 128,
            ground_texture_size: 128,
            vehicles: 6,
            ..SceneBudget::default()
        }
    } else {
        SceneBudget::default()
    };
    // `--clip-x m --clip-z m --clip-r m`: only the neighbourhood of one local point,
    // the unit a streamed scene is built from.
    // `--square x0,z0,x1,z1`: one square tile, built as the desktop app builds
    // it (`build_city_scene_tile`).
    let square: Option<([f32; 2], [f32; 2])> = flag("--square").map(|text| {
        let v: Vec<f32> = text.split(',').filter_map(|s| s.parse().ok()).collect();
        ([v[0], v[1]], [v[2], v[3]])
    });
    let city = match (flag("--clip-x"), flag("--clip-z"), flag("--clip-r")) {
        (Some(x), Some(z), Some(r)) => city.clipped(
            [x.parse().unwrap_or(0.0), z.parse().unwrap_or(0.0)],
            r.parse().unwrap_or(200.0),
        ),
        _ => city,
    };
    println!("{} buildings in the plan", city.buildings.len());
    if let Some(plan) = flag("--plan") {
        std::fs::write(&plan, serde_json::to_vec(&city)?)?;
        println!("wrote {plan}");
        return Ok(());
    }
    let scene = match square {
        Some((min, max)) => city_scene::build_city_scene_tile(
            &city,
            &city_scene::derive_city(&city),
            min,
            max,
            budget,
        ),
        None => build_city_scene(&city, budget),
    };
    let s = &scene.stats;
    println!(
        "{} buildings, {} tree instances, {} draws, {} tris",
        s.buildings, s.tree_instances, s.draw_calls, s.triangles
    );
    if let Some(binary) = flag("--bin") {
        std::fs::write(&binary, city_scene::scene::encode_binary(&scene)?)?;
        println!("wrote {binary}");
    }
    std::fs::write(&out, serde_json::to_vec(&vec![scene])?)?;
    println!("wrote {out}");
    Ok(())
}
