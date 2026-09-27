//! A quick structural report on a generated Chinese city: graph shape, degree
//! distribution and morphology score.  Used to check a generator change without
//! booting the renderer.

use std::collections::HashMap;

use urban::{ModernChinaSpec, Point, generate_modern_chinese_city, measure};

fn main() {
    let city = generate_modern_chinese_city(ModernChinaSpec {
        centre: Point {
            x_km: 5.0,
            y_km: 4.0,
        },
        radius_km: 1.4,
        rotation_radians: 0.17,
        seed: 42,
        density: 0.82,
        block_size_metres: 100.0,
        organic: 0.68,
        river_width_metres: 64.0,
    });
    let mut degrees: HashMap<u32, u32> = HashMap::new();
    for road in &city.sd_roads {
        *degrees.entry(road.from).or_insert(0) += 1;
        *degrees.entry(road.to).or_insert(0) += 1;
    }
    let crossings = degrees.values().filter(|degree| **degree >= 3).count();
    let bridges = city.hd_roads.iter().filter(|road| road.bridge).count();
    println!("nodes={} roads={} crossings={} bridges={}", city.nodes.len(), city.sd_roads.len(), crossings, bridges);
    println!(
        "blocks={} parcels={} buildings={} compounds={} trees={}",
        city.blocks.len(),
        city.parcels.len(),
        city.buildings.len(),
        city.compounds.len(),
        city.trees.len()
    );
    println!("morphology score: {:.3}", city.morphology_score);
    println!("stats: {:?}", measure(&city));
}
