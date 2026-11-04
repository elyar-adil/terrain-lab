//! Print the railways and airport of generated worlds: the statistics the
//! transport design study names for validation.
//!
//! cargo run --release -p infrastructure --example transport_report -- [seed] [plains|plateau|coastal] [grid]

use infrastructure::generate_infrastructure;
use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate};
use urban::RailGround;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let seed: u32 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(284_735);
    let landform = match args.get(2).map(String::as_str) {
        Some("plateau") => Landform::Plateau,
        Some("coastal") => Landform::Coastal,
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
    println!("seed {seed} {landform:?} grid {grid_size}: {} settlements", data.settlements.len());
    for r in &data.railways {
        let mut crossings = [0; 2];
        for c in &r.crossings {
            crossings[c.treatment as usize] += 1;
        }
        let straight = ((r.points[0].x_km - r.points.last().unwrap().x_km).hypot(r.points[0].y_km - r.points.last().unwrap().y_km))
            / r.length_km.max(1e-3);
        println!(
            "rail {} {:?} {}->{} len {:.1} km circ {:.2} Rmin {:.0} grade {:.2}% stations {} crossings level {} over {} | viaduct {:.0}% bridge {:.0}% tunnel {:.0}% emb {:.0}% cut {:.0}%",
            r.id, r.kind, r.from_settlement, r.to_settlement, r.length_km, 1.0 / straight.max(1e-3),
            r.min_radius_m, r.max_grade * 100.0, r.stations.len(), crossings[0], crossings[1],
            r.share(RailGround::Viaduct) * 100.0, r.share(RailGround::Bridge) * 100.0,
            r.share(RailGround::Tunnel) * 100.0, r.share(RailGround::Embankment) * 100.0,
            r.share(RailGround::Cutting) * 100.0
        );
    }
    match &data.airport {
        Some(a) => println!(
            "airport {} runway {}/{} {:.0}x{:.0} m bearing {:.0} usability {:.4} (13 kt: {:.4}, perpendicular {:.4}) city {:.1} km relief {:.1} m slope {:.2}% elev {:.0} m access pts {}",
            a.plan.code.name(), a.plan.designator_lo, a.plan.designator_hi, a.plan.runway_length_m,
            a.plan.runway_width_m, a.plan.bearing_deg(), a.wind_usability, a.wind_usability_13kt, a.perpendicular_usability_13kt,
            a.distance_to_city_km, a.relief_m, a.longitudinal_slope * 100.0, a.plan.elevation_m, a.access_path_km.len()
        ),
        None => println!("no airport site"),
    }
    for (i, c) in data.modern_cities.iter().enumerate().take(4) {
        println!("city {i}: {} rail corridors, {} buildings", c.rail.len(), c.buildings.len());
    }
    Ok(())
}
