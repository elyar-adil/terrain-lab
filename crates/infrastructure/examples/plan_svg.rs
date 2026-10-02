//! Draw one town's plan as SVG: roads by class, blocks, building footprints, the river.
//!
//!   PLANNER=legacy|fabric cargo run --release -p infrastructure --example plan_svg -- OUT.svg [settlement-index]

use std::{env, error::Error, fmt::Write as _};

use infrastructure::{CityPlanner, generate_infrastructure_with};
use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate};
use urban::ModernRoadClass;

fn main() -> Result<(), Box<dyn Error>> {
    let out = env::args().nth(1).unwrap_or_else(|| "plan.svg".into());
    let index: usize = env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let planner = if env::var("PLANNER").as_deref() == Ok("legacy") { CityPlanner::Legacy } else { CityPlanner::Fabric };
    let grid: usize = env::var("GRID").ok().and_then(|s| s.parse().ok()).unwrap_or(512);
    let config = SimulationConfig {
        seed: 284_735,
        preset: TerrainPreset::Temperate,
        landform: match env::var("LANDFORM").as_deref() {
            Ok("coastal") => Landform::Coastal,
            _ => Landform::Plains,
        },
        grid_size: grid,
        world_size_km: env::var("WORLD_KM").ok().and_then(|s| s.parse().ok()).unwrap_or(80.0),
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
    let t = std::time::Instant::now();
    let terrain = generate(&config, |_, _| {})?;
    eprintln!("terrain {:.1}s", t.elapsed().as_secs_f32());
    let t = std::time::Instant::now();
    let infra = generate_infrastructure_with(&terrain, &config, planner)?;
    eprintln!("infrastructure {:.1}s ({:?}): {} settlements, {} roads", t.elapsed().as_secs_f32(), planner, infra.settlements.len(), infra.roads.len());
    for (i, (s, c)) in infra.settlements.iter().zip(&infra.modern_cities).enumerate() {
        let at = s.location.y * terrain.size + s.location.x;
        eprintln!(
            "  {i:2} {:?} @({:3},{:3}) h={:.0}m: {} roads, {} blocks, {} parcels ({} villa, {} farm), {} buildings",
            s.class, s.location.x, s.location.y, terrain.height[at], c.sd_roads.len(), c.blocks.len(), c.parcels.len(),
            c.parcels.iter().filter(|p| p.use_type == urban::ParcelUse::Villa).count(),
            c.parcels.iter().filter(|p| p.use_type == urban::ParcelUse::Farmstead).count(),
            c.buildings.len()
        );
    }
    let city = &infra.modern_cities[index.min(infra.modern_cities.len() - 1)];
    eprintln!(
        "town {index}: {} nodes, {} roads, {} blocks, {} parcels, {} buildings",
        city.nodes.len(), city.sd_roads.len(), city.blocks.len(), city.parcels.len(), city.buildings.len()
    );

    eprintln!("  fields: {}", city.fields.len());
    for fl in city.fields.iter().step_by(97).take(6) {
        let l: Vec<[f32; 2]> = fl.ring.iter().map(|q| city.frame.to_local(*q)).collect();
        let c = [l.iter().map(|q| q[0]).sum::<f32>() / 4.0, l.iter().map(|q| q[1]).sum::<f32>() / 4.0];
        eprintln!("  field {:?}.{} at {:.0},{:.0}", fl.crop, fl.variant, c[0], c[1]);
    }
    let count = |u: urban::ParcelUse| city.parcels.iter().filter(|p| p.use_type == u).count();
    eprintln!(
        "  parcels: villa {}, farmstead {}, residential {}, commercial {}, mixed {}, civic {}, park {}",
        count(urban::ParcelUse::Villa),
        count(urban::ParcelUse::Farmstead),
        count(urban::ParcelUse::Residential),
        count(urban::ParcelUse::Commercial),
        count(urban::ParcelUse::MixedUse),
        count(urban::ParcelUse::Civic),
        count(urban::ParcelUse::Park)
    );

    // The densest knot of villas and of farmsteads, in local metres, for `--clip`.
    for (name, kind) in [("villa", urban::ParcelUse::Villa), ("farmstead", urban::ParcelUse::Farmstead)] {
        let at: Vec<[f32; 2]> = city
            .parcels
            .iter()
            .filter(|p| p.use_type == kind)
            .map(|p| {
                let n = p.ring.len() as f32;
                city.frame.to_local(urban::Point {
                    x_km: p.ring.iter().map(|q| q.x_km).sum::<f32>() / n,
                    y_km: p.ring.iter().map(|q| q.y_km).sum::<f32>() / n,
                })
            })
            .collect();
        let best = at.iter().max_by_key(|a| at.iter().filter(|b| (a[0] - b[0]).hypot(a[1] - b[1]) < 150.0).count());
        if let Some(b) = best {
            let n = at.iter().filter(|c| (b[0] - c[0]).hypot(b[1] - c[1]) < 150.0).count();
            eprintln!("  densest {name} knot: {:.0},{:.0} ({n} within 150 m)", b[0], b[1]);
        }
    }

    // A camera in front of a few villas and farmsteads, for the city viewer's `cam=`.
    for (name, kind) in [("villa", urban::ParcelUse::Villa), ("farmstead", urban::ParcelUse::Farmstead)] {
        let knot: Option<Vec<f32>> = env::var("KNOT").ok().map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect());
        for p in city
            .parcels
            .iter()
            .filter(|p| p.use_type == kind)
            .filter(|p| {
                knot.as_ref().is_none_or(|k| {
                    let q = city.frame.to_local(p.ring[0]);
                    k.len() == 2 && (q[0] - k[0]).hypot(q[1] - k[1]) < 260.0
                })
            })
            .step_by(3)
            .take(6)
        {
            let l: Vec<[f32; 2]> = p.ring.iter().map(|q| city.frame.to_local(*q)).collect();
            let n = l.len() as f32;
            let c = [l.iter().map(|q| q[0]).sum::<f32>() / n, l.iter().map(|q| q[1]).sum::<f32>() / n];
            let m = [(l[0][0] + l[1][0]) / 2.0, (l[0][1] + l[1][1]) / 2.0];
            let d = [m[0] - c[0], m[1] - c[1]];
            let len = d[0].hypot(d[1]).max(1.0);
            let at = [c[0] + d[0] / len * 34.0, c[1] + d[1] / len * 34.0];
            eprintln!("  {name} cam={:.0},2.6,{:.0},{:.0},3.5,{:.0},50", at[0], at[1], c[0], c[1]);
        }
    }

    // Bounds from the roads.
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for n in &city.nodes {
        x0 = x0.min(n.point.x_km);
        x1 = x1.max(n.point.x_km);
        y0 = y0.min(n.point.y_km);
        y1 = y1.max(n.point.y_km);
    }
    let size = (x1 - x0).max(y1 - y0) * 1.05;
    let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    let px = 1600.0_f32;
    let k = px / size;
    let tx = |p: &urban::Point| ((p.x_km - cx) * k + px / 2.0, (cy - p.y_km) * k + px / 2.0);
    let mut svg = String::new();
    writeln!(svg, r##"<svg xmlns="http://www.w3.org/2000/svg" width="{px}" height="{px}"><rect width="{px}" height="{px}" fill="#e8e4d4"/>"##)?;
    if let Some(river) = &city.river {
        let pts: Vec<String> = river.iter().map(|p| { let (x, y) = tx(p); format!("{x:.1},{y:.1}") }).collect();
        writeln!(svg, r##"<polyline points="{}" fill="none" stroke="#9ec3dd" stroke-width="{:.1}" stroke-linecap="round"/>"##, pts.join(" "), city.river_width_metres / 1000.0 * k)?;
    }
    for b in &city.blocks {
        let pts: Vec<String> = b.boundary.iter().map(|p| { let (x, y) = tx(p); format!("{x:.1},{y:.1}") }).collect();
        writeln!(svg, r##"<polygon points="{}" fill="#cfd8b8" stroke="none"/>"##, pts.join(" "))?;
    }
    for t in &city.tributaries {
        let pts: Vec<String> = t.line.iter().map(|p| { let (x, y) = tx(p); format!("{x:.1},{y:.1}") }).collect();
        writeln!(svg, r##"<polyline points="{}" fill="none" stroke="#9ec3dd" stroke-width="{:.1}" stroke-linecap="round"/>"##, pts.join(" "), t.width_metres / 1000.0 * k)?;
    }
    for fl in &city.fields {
        let pts: Vec<String> = fl.ring.iter().map(|p| { let (x, y) = tx(p); format!("{x:.1},{y:.1}") }).collect();
        let fill = match fl.crop {
            urban::CropKind::Wheat => "#c9c46a",
            urban::CropKind::Rice => "#7fb36a",
            urban::CropKind::Rapeseed => "#f0d83a",
            urban::CropKind::Corn => "#8fa84a",
            urban::CropKind::Vegetables => "#5f9a4a",
            urban::CropKind::Fallow => "#a58a68",
            urban::CropKind::Orchard => "#3f7a4a",
        };
        writeln!(svg, r##"<polygon points="{}" fill="{fill}" stroke="#777" stroke-width="0.3"/>"##, pts.join(" "))?;
    }
    for b in &city.buildings {
        let pts: Vec<String> = b.footprint.iter().map(|p| { let (x, y) = tx(p); format!("{x:.1},{y:.1}") }).collect();
        let fill = match b.use_type {
            urban::ParcelUse::Villa => "#d9895a",
            urban::ParcelUse::Farmstead => "#a8552e",
            _ => "#b9a99a",
        };
        writeln!(svg, r##"<polygon points="{}" fill="{fill}" stroke="#8d7f73" stroke-width="0.3"/>"##, pts.join(" "))?;
    }
    let mut roads: Vec<_> = city.hd_roads.iter().collect();
    roads.sort_by_key(|r| std::cmp::Reverse(r.class as u8));
    for r in roads {
        let pts: Vec<String> = r.centreline.iter().map(|p| { let (x, y) = tx(p); format!("{x:.1},{y:.1}") }).collect();
        let colour = match r.class {
            ModernRoadClass::Expressway | ModernRoadClass::Arterial => "#c9573b",
            ModernRoadClass::Collector => "#d89a3c",
            ModernRoadClass::Local => "#ffffff",
        };
        let w = r.width_metres / 1000.0 * k;
        let casing = if r.bridge { "#2f3b52" } else { "#8d867a" };
        writeln!(svg, r##"<polyline points="{pts}" fill="none" stroke="{casing}" stroke-width="{:.2}" stroke-linecap="round"/><polyline points="{pts}" fill="none" stroke="{colour}" stroke-width="{:.2}" stroke-linecap="round"/>"##, w.max(1.0) + 1.0, w.max(0.8), pts = pts.join(" "))?;
    }
    svg.push_str("</svg>\n");
    std::fs::write(&out, svg)?;
    eprintln!("wrote {out} ({:.1} km across)", size);
    Ok(())
}
