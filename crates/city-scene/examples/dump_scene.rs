//! Write one city scene as the JSON array `city-harness.html` loads, without
//! needing the Tauri crate (and its native webview dependencies) to build.
//!
//! cargo run --release -p city-scene --example dump_scene -- \
//!     [--radius-km 0.5] [--block-m 110] [--seed 42] [--out public/city-scenes.json]

use city_scene::{SceneBudget, build_city_scene};
use urban::{ModernChinaSpec, generate_modern_chinese_city};

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let index = args.iter().position(|value| value == name)?;
    args.get(index + 1).cloned()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let radius_km = flag("--radius-km").and_then(|v| v.parse().ok()).unwrap_or(0.5_f32);
    let block = flag("--block-m").and_then(|v| v.parse().ok()).unwrap_or(110.0_f32);
    let seed = flag("--seed").and_then(|v| v.parse().ok()).unwrap_or(42_u32);
    let out = flag("--out").unwrap_or_else(|| "public/city-scenes.json".into());
    let city = generate_modern_chinese_city(ModernChinaSpec {
        seed,
        radius_km,
        block_size_metres: block,
        ..ModernChinaSpec::default()
    });
    let scene = build_city_scene(&city, SceneBudget::default());
    let s = &scene.stats;
    println!(
        "{} buildings, {} tree instances, {} draws, {} tris",
        s.buildings, s.tree_instances, s.draw_calls, s.triangles
    );
    std::fs::write(&out, serde_json::to_vec(&vec![scene])?)?;
    println!("wrote {out}");
    Ok(())
}
