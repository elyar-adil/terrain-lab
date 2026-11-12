//! Plausibility audit of one generated city: runs `city_scene::audit` over the
//! plan, its street network and the built scene, prints the count per check and
//! the worst offenders with local coordinates and ids, optionally writes a JSON
//! report, and exits non-zero when a check is over its gate.
//!
//! cargo run --release -p city-scene --example sanity_audit -- [inputs] [options]
//!
//! Inputs are those of `dump_scene`: the demo city (`--seed`, `--radius-km`,
//! `--block-m`, `--disc`), `--from plan.json`, `--fixture f.json --city i`, or
//! `--world SEED [--town i] [--landform plains|plateau|coastal] [--grid 256]`
//! (a town of a generated world, which also lets the slope checks read the
//! terrain), with `--climate temp_c,moisture,elevation_m,coast_km[,snow]`.
//!
//! Options:
//!   --json report.json    write the full report
//!   --top N               worst offenders listed per check (default 5)
//!   --gate                exit 1 when any check is over its gate (default)
//!   --no-gate             always exit 0
//!   --lite                small scene budget (fewer trees and buildings)

use city_scene::audit::{self, Terrain};
use city_scene::{SceneBudget, build_city_scene, derive_city};
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
    // `--fixture plan.json --city i`: the plan of city i from a render fixture, so the
    // tiles match the far form the terrain view draws for that same city.
    let mut world_terrain: Option<(Vec<f32>, usize, f32)> = None;
    let city = match (flag("--from"), flag("--fixture")) {
        (Some(path), _) => serde_json::from_slice::<urban::ModernCity>(&std::fs::read(path)?)?,
        // `--world SEED [--town i] [--landform plains|plateau|coastal] [--grid 256]`:
        // a town of a generated world, planned on its terrain (the game's own path).
        (None, None) if flag("--world").is_some() => {
            use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate};
            let config = SimulationConfig {
                seed: flag("--world")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(284_735),
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
            world_terrain = Some((terrain.height.clone(), terrain.size, config.world_size_km));
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
            facade_texture_size: 64,
            ground_texture_size: 64,
            vehicles: 6,
            ..SceneBudget::default()
        }
    } else {
        SceneBudget {
            facade_texture_size: 64,
            ground_texture_size: 64,
            ..SceneBudget::default()
        }
    };
    let network = derive_city(&city);
    let scene = build_city_scene(&city, budget);
    let terrain = world_terrain.map(|(heights, n, size_km)| {
        let frame = city.frame;
        Terrain {
            height_m: Box::new(move |x, z| {
                let p = frame.to_world(x, z);
                let gx = (p.x_km / size_km * (n - 1) as f32).clamp(0.0, (n - 1) as f32 - 1.001);
                let gy = (p.y_km / size_km * (n - 1) as f32).clamp(0.0, (n - 1) as f32 - 1.001);
                let (ix, iy) = (gx as usize, gy as usize);
                let (fx, fy) = (gx - ix as f32, gy - iy as f32);
                let at = |i: usize, j: usize| heights[j * n + i];
                let top = at(ix, iy) * (1.0 - fx) + at(ix + 1, iy) * fx;
                let bottom = at(ix, iy + 1) * (1.0 - fx) + at(ix + 1, iy + 1) * fx;
                top * (1.0 - fy) + bottom * fy
            }),
        }
    });
    let report = audit::audit(
        &city,
        &network,
        Some(&scene),
        terrain.as_ref(),
        budget.max_buildings,
    );
    let top: usize = flag("--top").and_then(|v| v.parse().ok()).unwrap_or(5);
    report.print(top);
    if let Some(path) = flag("--json") {
        std::fs::write(&path, serde_json::to_vec_pretty(&report)?)?;
        println!("wrote {path}");
    }
    let over = report.over_gate();
    if !over.is_empty() && !std::env::args().any(|a| a == "--no-gate") {
        eprintln!("over the gate:");
        for (id, count, gate) in over {
            eprintln!("  {id}: {count} (gate {gate})");
        }
        std::process::exit(1);
    }
    Ok(())
}
