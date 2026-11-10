//! Numeric check of every road against the river's water surface and banks:
//! for each bridge the deck, soffit and clearance over the water; for every
//! road, whether any carriageway runs over the channel or on the bank slope
//! below its own surface.
//!
//!     cargo run --release -p city-scene --example river_audit -- [--seed 42] [--radius-km 0.5]

use city_scene::buildings::{drop_depth, water_level};
use urban::{ModernChinaSpec, generate_modern_chinese_city};

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let i = args.iter().position(|v| v == name)?;
    args.get(i + 1).cloned()
}

fn dist_to_polyline(p: [f32; 2], line: &[[f32; 2]]) -> f32 {
    let mut best = f32::MAX;
    for w in line.windows(2) {
        let (a, b) = (w[0], w[1]);
        let (dx, dz) = (b[0] - a[0], b[1] - a[1]);
        let l2 = (dx * dx + dz * dz).max(1e-6);
        let t = (((p[0] - a[0]) * dx + (p[1] - a[1]) * dz) / l2).clamp(0.0, 1.0);
        let (qx, qz) = (a[0] + dx * t, a[1] + dz * t);
        best = best.min((p[0] - qx).hypot(p[1] - qz));
    }
    best
}

fn main() {
    let seed = flag("--seed").and_then(|v| v.parse().ok()).unwrap_or(42_u32);
    let radius_km = flag("--radius-km").and_then(|v| v.parse().ok()).unwrap_or(0.5_f32);
    let city = generate_modern_chinese_city(ModernChinaSpec {
        seed,
        radius_km,
        block_size_metres: 110.0,
        ..ModernChinaSpec::default()
    });
    let network = city_scene::derive_city(&city);
    let Some(river) = city.river.as_deref() else {
        println!("no river");
        return;
    };
    let line: Vec<[f32; 2]> = river.iter().map(|p| city.frame.to_local(*p)).collect();
    let half = city.river_width_metres * 0.5;
    let setting = city.river_setting;
    let water = water_level(&setting, city.river_width_metres);
    println!(
        "river width {:.1} m, drop {:.2} m, water surface y = {:.2}; setting {:?}",
        city.river_width_metres,
        drop_depth(&setting, city.river_width_metres),
        water,
        setting
    );
    println!("river line (local m): {:?}", line.iter().map(|p| [p[0].round(), p[1].round()]).collect::<Vec<_>>());
    let mut bad = 0;
    for road in &network.roads {
        let length = road.carriageway.length();
        let mut over = 0;
        let mut bank = 0;
        let (mut min_deck, mut min_over) = (f32::MAX, f32::MAX);
        let mut at_xz = [0.0_f32; 2];
        let mut station = 0.0;
        while station <= length {
            let p = road.carriageway.sample(station).0;
            let d = dist_to_polyline([p.x, p.z], &line);
            if d < half {
                over += 1;
                min_over = min_over.min(p.y);
                at_xz = [p.x, p.z];
                min_deck = min_deck.min(p.y);
            } else if d < half + 4.6 {
                bank += 1;
            }
            station += 2.0;
        }
        if over == 0 && bank == 0 {
            continue;
        }
        let girder = if road.bridge {
            city_scene::street::river_girder_depth(road.deck_thickness)
        } else {
            road.deck_thickness
        };
        let verdict = if over > 0 && !road.bridge && road.layer == 0 {
            bad += 1;
            "ROAD OVER WATER, NOT A BRIDGE"
        } else if over > 0 && min_over - girder < water + 1.5 {
            bad += 1;
            "LOW CLEARANCE"
        } else {
            "ok"
        };
        println!(
            "road {:>4} bridge={:<5} layer={} len {:>6.1} m: {:>3} samples over water, {:>3} on bank strip; \
             at ({:.0},{:.0}) lowest deck over water {:>6.2} (soffit {:>6.2}, clearance above water {:>5.2}) {}",
            road.id,
            road.bridge,
            road.layer,
            length,
            over,
            bank,
            at_xz[0],
            at_xz[1],
            if over > 0 { min_over } else { f32::NAN },
            if over > 0 { min_over - girder } else { f32::NAN },
            if over > 0 { min_over - girder - water } else { f32::NAN },
            verdict
        );
    }
    println!("{bad} problems");
}
