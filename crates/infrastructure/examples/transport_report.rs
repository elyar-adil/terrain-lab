//! Print the railways and airport of generated worlds: the statistics the
//! transport design study names for validation.
//!
//! cargo run --release -p infrastructure --example transport_report -- [seed] [plains|plateau|coastal|mountain] [grid]

use infrastructure::generate_infrastructure;
use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate};
use urban::RailGround;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let seed: u32 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(284_735);
    let landform = match args.get(2).map(String::as_str) {
        Some("plateau") => Landform::Plateau,
        Some("coastal") => Landform::Coastal,
        Some("mountain") => Landform::MountainRange,
        _ => Landform::Plains,
    };
    let grid_size: usize = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(256);
    let config = SimulationConfig {
        seed,
        preset: TerrainPreset::Temperate,
        landform,
        grid_size,
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
    let data = generate_infrastructure(&terrain, &config)?;
    println!(
        "seed {seed} {landform:?} grid {grid_size}: {} settlements",
        data.settlements.len()
    );
    for r in &data.railways {
        let mut crossings = [0; 3];
        for c in &r.crossings {
            crossings[c.treatment as usize] += 1;
        }
        let straight = ((r.points[0].x_km - r.points.last().unwrap().x_km)
            .hypot(r.points[0].y_km - r.points.last().unwrap().y_km))
            / r.length_km.max(1e-3);
        println!(
            "rail {} {:?} {}->{} len {:.1} km circ {:.2} Rmin {:.0} grade {:.2}% stations {} crossings level {} over {} under {} | mountain {:.1} km fill {:.0} m depth {:.0} m | viaduct {:.0}% bridge {:.0}% tunnel {:.0}% emb {:.0}% cut {:.0}%",
            r.id,
            r.kind,
            r.from_settlement,
            r.to_settlement,
            r.length_km,
            1.0 / straight.max(1e-3),
            r.min_radius_m,
            r.max_grade * 100.0,
            r.stations.len(),
            crossings[0],
            crossings[1],
            crossings[2],
            r.mountain_m / 1000.0,
            r.points
                .iter()
                .map(|p| p.design_m - p.ground_m)
                .fold(0.0_f32, f32::max),
            r.points
                .iter()
                .map(|p| p.ground_m - p.design_m)
                .fold(0.0_f32, f32::max),
            r.share(RailGround::Viaduct) * 100.0,
            r.share(RailGround::Bridge) * 100.0,
            r.share(RailGround::Tunnel) * 100.0,
            r.share(RailGround::Embankment) * 100.0,
            r.share(RailGround::Cutting) * 100.0
        );
    }
    match &data.airport {
        Some(a) => println!(
            "airport {} runway {}/{} {:.0}x{:.0} m bearing {:.0} usability {:.4} (13 kt: {:.4}, perpendicular {:.4}) city {:.1} km relief {:.1} m slope {:.2}% elev {:.0} m access pts {}",
            a.plan.code.name(),
            a.plan.designator_lo,
            a.plan.designator_hi,
            a.plan.runway_length_m,
            a.plan.runway_width_m,
            a.plan.bearing_deg(),
            a.wind_usability,
            a.wind_usability_13kt,
            a.perpendicular_usability_13kt,
            a.distance_to_city_km,
            a.relief_m,
            a.longitudinal_slope * 100.0,
            a.plan.elevation_m,
            a.access_path_km.len()
        ),
        None => println!("no airport site"),
    }
    for r in &data.railways {
        for st in &r.stations {
            let p = r
                .points
                .iter()
                .min_by(|a, b| {
                    (a.s_m - st.plan.s_m)
                        .abs()
                        .total_cmp(&(b.s_m - st.plan.s_m).abs())
                })
                .unwrap();
            println!(
                "  rail {} station of settlement {} at ({:.2}, {:.2}) km, {} tracks, ground {:?}",
                r.id, st.settlement, p.x_km, p.y_km, st.plan.tracks, p.ground
            );
        }
    }
    if let Some(a) = &data.airport {
        println!(
            "  airport centre ({:.2}, {:.2}) km axis ({:.2}, {:.2})",
            a.plan.centre.x_km, a.plan.centre.y_km, a.plan.axis[0], a.plan.axis[1]
        );
    }
    for s in data.settlements.iter().take(4) {
        println!(
            "  settlement {} {:?} at grid {:?}",
            s.id, s.class, s.location
        );
    }
    for (i, c) in data.modern_cities.iter().enumerate().take(4) {
        for corridor in &c.rail {
            let line = corridor.local_line(c);
            let st = corridor.stations.first();
            let at = |s: f32| {
                let k = corridor
                    .vertices
                    .iter()
                    .position(|v| v.s_m >= s)
                    .unwrap_or(0);
                line[k]
            };
            if let Some(st) = st {
                println!(
                    "  city {i} station local {:?}, -300 m {:?}, +300 m {:?}, side {}",
                    at(st.s_m),
                    at(st.s_m - 300.0),
                    at(st.s_m + 300.0),
                    st.building_side
                );
            }
            let mid = corridor.vertices[corridor.vertices.len() / 2].s_m;
            println!(
                "  city {i} corridor mid {:?} ground {:?} above {:.1}",
                at(mid),
                corridor.vertices[corridor.vertices.len() / 2].ground,
                corridor.vertices[corridor.vertices.len() / 2].above_ground_m
            );
        }
        println!(
            "city {i}: {} rail corridors, {} buildings",
            c.rail.len(),
            c.buildings.len()
        );
    }
    Ok(())
}
