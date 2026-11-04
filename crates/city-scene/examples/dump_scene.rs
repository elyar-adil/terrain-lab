//! Write one city scene as the JSON array `city-harness.html` loads, without
//! needing the Tauri crate (and its native webview dependencies) to build.
//!
//! cargo run --release -p city-scene --example dump_scene -- \
//!     [--radius-km 0.5] [--block-m 110] [--seed 42] [--out public/city-scenes.json]
//!     [--bin city.bin]   also write the binary container the desktop app streams
//!     [--disc]           the old round, roadless-outside standalone city
//!     [--lite]           small budgets, for headless software rendering
//!     [--plan plan.json] write the city plan itself and stop, for checking a layout
//!     [--from plan.json] build from a plan written elsewhere (`plan_svg` with
//!                        `PLAN_JSON`: the towns the game builds)

use city_scene::{SceneBudget, build_city_scene};
use urban::{
    CityOptions, ModernChinaSpec, ModernRoadClass, Point, RegionalApproach,
    generate_modern_chinese_city_with_options,
};

/// The standalone city has no world under it, so its edge and its roads are
/// seeded: four regional roads leave the town in different directions (an
/// arterial, two collectors and a local lane) and the built-up area is the
/// organic, lumpy footprint that runs out along them, not a disc. For a town
/// laid on real terrain (roads through valleys and over bridges, fields,
/// terraces and shelterbelts that follow slope, soil and climate) write a plan
/// with `cargo run --release -p infrastructure --example plan_svg` (with
/// `PLAN_JSON=plan.json`) and load it with `--from`.
fn seeded_approaches(seed: u32, radius_km: f32) -> Vec<RegionalApproach> {
    let classes = [
        ModernRoadClass::Arterial,
        ModernRoadClass::Collector,
        ModernRoadClass::Collector,
        ModernRoadClass::Local,
    ];
    classes
        .iter()
        .enumerate()
        .map(|(i, class)| {
            let h = urban::hash_u32(seed, i as i32, 77);
            let angle = (i as f32 + 0.15 + 0.7 * h) * std::f32::consts::TAU / 4.0;
            let (c, s) = (angle.cos(), angle.sin());
            // A gently bowed line from well outside the town to its middle.
            let path_km = (0..=16)
                .map(|k| {
                    let t = k as f32 / 16.0;
                    let r = radius_km * (1.7 - 1.7 * t);
                    let bow = (t * std::f32::consts::PI).sin() * radius_km * 0.08 * (h - 0.5);
                    Point {
                        x_km: r * c - bow * s,
                        y_km: r * s + bow * c,
                    }
                })
                .collect();
            RegionalApproach {
                class: *class,
                path_km,
            }
        })
        .collect()
}

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
        // `--world SEED [--town i] [--landform plains|plateau|coastal] [--grid 256]`:
        // a town of a generated world, planned on its terrain (the game's own path).
        (None, None) if flag("--world").is_some() => {
            use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate};
            let config = SimulationConfig {
                seed: flag("--world").and_then(|v| v.parse().ok()).unwrap_or(284_735),
                preset: TerrainPreset::Temperate,
                landform: match flag("--landform").as_deref() {
                    Some("plateau") => Landform::Plateau,
                    Some("coastal") => Landform::Coastal,
                    _ => Landform::Plains,
                },
                grid_size: flag("--grid").and_then(|v| v.parse().ok()).unwrap_or(256),
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
            let data = infrastructure::generate_infrastructure(&terrain, &config)?;
            let index: usize = flag("--town").and_then(|v| v.parse().ok()).unwrap_or(0);
            data.modern_cities
                .get(index)
                .cloned()
                .ok_or("no such town in the world")?
        }
        (None, fixture) => match fixture {
            Some(path) => {
                let index: usize = flag("--city").and_then(|v| v.parse().ok()).unwrap_or(0);
                let document: serde_json::Value = serde_json::from_slice(&std::fs::read(&path)?)?;
                let plan = document["result"]["modernCities"][index].clone();
                serde_json::from_value::<urban::ModernCity>(plan)?
            }
            None => {
                let spec = ModernChinaSpec {
                    seed,
                    radius_km,
                    block_size_metres: block,
                    ..ModernChinaSpec::default()
                };
                let disc = std::env::args().any(|a| a == "--disc");
                let approaches = if disc {
                    Vec::new()
                } else {
                    seeded_approaches(seed, radius_km)
                };
                generate_modern_chinese_city_with_options(
                    spec,
                    &approaches,
                    CityOptions {
                        organic_footprint: !disc,
                    },
                )
            }
        },
    };
    // `--climate temp_c,moisture,elevation_m,coast_km[,snow]`: plant as if the town
    // stood in that climate (a plan made on real terrain carries its own).
    let city = match flag("--climate") {
        Some(text) => {
            let v: Vec<f32> = text.split(',').filter_map(|s| s.parse().ok()).collect();
            let mut city = city;
            city.climate = urban::SiteClimate {
                temperature_c: v[0],
                moisture: v[1],
                elevation_m: v[2],
                coast_km: v[3],
                snow: v.get(4).copied().unwrap_or(0.0),
            };
            city
        }
        None => city,
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
    // `--river-setting moisture,discharge,slope,relief_m,sediment`: the terrain's account
    // of the river, for checking how the valley and banks follow it.
    let mut city = city;
    if let Some(text) = flag("--river-setting") {
        let v: Vec<f32> = text.split(',').filter_map(|s| s.parse().ok()).collect();
        if v.len() == 5 {
            city.river_setting = urban::RiverSetting {
                moisture: v[0],
                discharge: v[1],
                slope: v[2],
                relief_m: v[3],
                sediment: v[4],
                ..city.river_setting
            };
        }
    }
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
