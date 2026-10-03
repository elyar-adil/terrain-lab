use urban::{ModernChinaSpec, Point, generate_modern_chinese_city};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let seed: u32 = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(42);
    let spec = match (a.get(2), a.get(3)) {
        (Some(r), Some(b)) => ModernChinaSpec {
            seed,
            radius_km: r.parse().unwrap(),
            block_size_metres: b.parse().unwrap(),
            ..ModernChinaSpec::default()
        },
        _ => ModernChinaSpec {
            centre: Point {
                x_km: 5.0,
                y_km: 4.0,
            },
            radius_km: 1.4,
            rotation_radians: 0.17,
            seed,
            density: 0.82,
            block_size_metres: 100.0,
            organic: 0.68,
            river_width_metres: 64.0,
            ..ModernChinaSpec::default()
        },
    };
    let city = generate_modern_chinese_city(spec);
    let pts = |p: &Vec<Point>| {
        p.iter()
            .map(|q| format!("{:.1},{:.1}", q.x_km * 1000.0, q.y_km * 1000.0))
            .collect::<Vec<_>>()
            .join(" ")
    };
    for r in &city.hd_roads {
        println!(
            "R\t{:?}\t{:.1}\t{}\t{}",
            r.class,
            r.width_metres,
            r.bridge as u8,
            pts(&r.centreline)
        );
    }
    for b in &city.blocks {
        println!("K\t{}", pts(&b.boundary));
    }
    for p in &city.parcels {
        println!("P\t{:?}\t{}", p.use_type, pts(&p.ring));
    }
    for b in &city.buildings {
        println!(
            "B\t{:.1}\t{:?}\t{:?}\t{}\t{}",
            b.height_metres,
            b.facade,
            b.roof,
            pts(&b.footprint),
            b.parcel_id
        );
    }
    if let Some(r) = &city.river {
        println!("W\t{}", pts(r));
    }
    eprintln!(
        "blocks={} parcels={} buildings={} compounds={} trees={}",
        city.blocks.len(),
        city.parcels.len(),
        city.buildings.len(),
        city.compounds.len(),
        city.trees.len()
    );
}
