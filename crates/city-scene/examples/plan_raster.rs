//! Rasterise the street layer (and the block plates) straight down, flat
//! colours per material, so the way surfaces meet can be checked in seconds
//! instead of the ten minutes a software-rendered city shot takes.
//!
//! cargo run --release -p city-scene --example plan_raster -- \
//!     [--radius-km 1.6] [--seed 42] [--centre x,z] [--span 160] [--px 800]
//!     [--plan plan.json]          a plan from `plan_svg` (PLAN_JSON) instead
//!     [--out plan.ppm] [--list]   print junctions and air nodes, and stop
//!     [--layer N]                 draw only what is at or below N metres up

use city_scene::math::Vec2;
use city_scene::{JunctionSpec, MeshBuilder};
use urban::{ModernChinaSpec, generate_modern_chinese_city};

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let index = args.iter().position(|value| value == name)?;
    args.get(index + 1).cloned()
}

fn colour(material: &str) -> Option<[f32; 3]> {
    Some(match material {
        "asphalt" => [0.20, 0.20, 0.22],
        "asphalt.cycle" => [0.30, 0.28, 0.27],
        "asphalt.pavement" => [0.12, 0.12, 0.12],
        "sidewalk" => [0.62, 0.60, 0.56],
        "kerb" => [0.85, 0.85, 0.80],
        "median.plant" => [0.20, 0.45, 0.18],
        "block.ground" => [0.36, 0.50, 0.30],
        "marking.white" | "marking.crosswalk" | "marking.dashed-3-5" | "marking.dashed-6-9" => {
            [0.97, 0.97, 0.97]
        }
        "marking.yellow" => [0.95, 0.75, 0.10],
        "viaduct.concrete" => [0.66, 0.65, 0.62],
        "viaduct.planter" => [0.30, 0.55, 0.25],
        "viaduct.glass" => [0.55, 0.70, 0.75],
        "bridge.concrete" => [0.7, 0.68, 0.64],
        "water" => [0.20, 0.35, 0.50],
        _ => return None,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let radius_km = flag("--radius-km")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1.6_f32);
    let seed = flag("--seed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(42_u32);
    // `--plan plan.json`: a plan written elsewhere (`plan_svg` with `PLAN_JSON`,
    // the towns the game builds) instead of a generated test city.
    let city = match flag("--plan") {
        Some(path) => serde_json::from_slice::<urban::ModernCity>(&std::fs::read(path)?)?,
        None => generate_modern_chinese_city(ModernChinaSpec {
            seed,
            radius_km,
            block_size_metres: flag("--block-m")
                .and_then(|v| v.parse().ok())
                .unwrap_or(ModernChinaSpec::default().block_size_metres),
            ..ModernChinaSpec::default()
        }),
    };
    let network = city_scene::derive(
        &city.nodes,
        &city.sd_roads,
        &city.hd_roads,
        city.frame,
        JunctionSpec::default(),
        city.seed,
    );
    if let Some(at) = flag("--near") {
        let v: Vec<f32> = at.split(',').filter_map(|s| s.parse().ok()).collect();
        roads_near(&city, v[0], v[1], v[2]);
        return Ok(());
    }
    if std::env::args().any(|a| a == "--deviation") {
        deviation_report(&city);
        return Ok(());
    }
    if let Some(id) = flag("--road").and_then(|v| v.parse().ok()) {
        road_report(&network, id);
        return Ok(());
    }
    if let Some(node) = flag("--junction").and_then(|v| v.parse().ok()) {
        junction_report(&network, node);
        return Ok(());
    }
    // `--connectors node`: every movement through a junction, one line each:
    // movement then the plan points, for drawing over a render.
    if let Some(node) = flag("--connectors").and_then(|v| v.parse::<u32>().ok()) {
        for c in network.connectors.iter().filter(|c| c.node == node) {
            let points: Vec<String> = (0..=40)
                .map(|k| {
                    let p = c.path.plan_at(c.path.length() * k as f32 / 40.0);
                    format!("{:.2},{:.2}", p.x, p.y)
                })
                .collect();
            println!("{:?} {}", c.movement, points.join(" "));
        }
        return Ok(());
    }
    if std::env::args().any(|a| a == "--acute") {
        for j in &network.junctions {
            let n = j.ports.len();
            if n < 3 {
                continue;
            }
            for k in 0..n {
                let a = -j.ports[k].dir;
                let b = -j.ports[(k + 1) % n].dir;
                let g = city_scene::corner::gap(a, b).to_degrees();
                if g < 50.0 {
                    let cls = |i: usize| {
                        network
                            .road(j.ports[i].road)
                            .map(|r| format!("{:?}", r.class))
                            .unwrap_or_default()
                    };
                    println!(
                        "junction {} at {:.0},{:.0}: corner {:.0} between road {} {} and road {} {}",
                        j.node,
                        j.centre.x,
                        j.centre.y,
                        g,
                        j.ports[k].road,
                        cls(k),
                        j.ports[(k + 1) % n].road,
                        cls((k + 1) % n)
                    );
                }
            }
        }
        return Ok(());
    }
    if std::env::args().any(|a| a == "--list") {
        for junction in &network.junctions {
            let classes: Vec<String> = junction
                .ports
                .iter()
                .filter_map(|p| network.road(p.road))
                .map(|r| format!("{:?}{}", r.class, if r.oneway { "/ramp" } else { "" }))
                .collect();
            println!(
                "junction {} at {:.0},{:.0} {:?} r={:.1} [{}]",
                junction.node,
                junction.centre.x,
                junction.centre.y,
                junction.kind,
                junction.radius,
                classes.join(" ")
            );
        }
        for road in network.roads.iter().filter(|r| r.oneway) {
            let p = road.carriageway.start();
            let q = road.carriageway.end();
            println!(
                "ramp {} {:.0},{:.0} -> {:.0},{:.0} air {} {}",
                road.id,
                p.x,
                p.z,
                q.x,
                q.z,
                network.is_air(road.from_node),
                network.is_air(road.to_node)
            );
        }
        for warning in &network.warnings {
            println!("warning: {warning}");
        }
        return Ok(());
    }
    let centre: Vec<f32> = flag("--centre")
        .unwrap_or_else(|| "0,0".into())
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let centre = Vec2::new(centre[0], centre[1]);
    let span: f32 = flag("--span").and_then(|v| v.parse().ok()).unwrap_or(160.0);
    let px: usize = flag("--px").and_then(|v| v.parse().ok()).unwrap_or(800);
    let ceiling: f32 = flag("--layer")
        .and_then(|v| v.parse().ok())
        .unwrap_or(f32::MAX);
    let out = flag("--out").unwrap_or_else(|| "plan.ppm".into());

    let mut builder = MeshBuilder::new();
    city_scene::build_street_layer(&network, &mut builder, city.seed);
    let streets =
        city_scene::street::footprints(&network, city_scene::buildings::PLATE_UNDER_PAVING);
    for block in city
        .blocks
        .iter()
        .filter(|_| !std::env::args().any(|a| a == "--no-blocks"))
    {
        let ring: Vec<Vec2> = block
            .boundary
            .iter()
            .map(|p| {
                let [x, z] = city.frame.to_local(*p);
                Vec2::new(x, z)
            })
            .collect();
        let mut ring = ring;
        if city_scene::math::signed_area(&ring) < 0.0 {
            ring.reverse();
        }
        for piece in city_scene::buildings::cut_streets(ring, &streets) {
            if piece.len() >= 3 {
                builder.ground_uv(
                    "block.ground",
                    &piece,
                    city_scene::street::level::BLOCK,
                    None,
                );
            }
        }
    }
    let geometry = builder.build();
    if let Some(at) = flag("--probe") {
        let v: Vec<f32> = at.split(',').filter_map(|s| s.parse().ok()).collect();
        let (px, pz) = (v[0], v[1]);
        let at = Vec2::new(px, pz);
        for j in &network.junctions {
            if city_scene::math::point_in_ring(at, &j.ring) {
                println!("inside junction {} box (even-odd)", j.node);
            }
            for piece in city_scene::buildings::normalise(&j.ring) {
                if city_scene::math::point_in_ring(at, &piece) {
                    println!("inside junction {} box (normalised)", j.node);
                }
            }
        }
        for r in &network.roads {
            let plan: Vec<Vec2> = r
                .carriageway
                .points()
                .iter()
                .map(|p| Vec2::new(p.x, p.z))
                .collect();
            let d = city_scene::math::distance_to_polyline(at, &plan);
            if d < r.half_width() + r.section.sidewalk_metres + 1.0 {
                println!(
                    "road {} ({:?}) centreline {:.1} m away, half {:.1}, walk {:.1}, from {} to {}",
                    r.id,
                    r.class,
                    d,
                    r.half_width(),
                    r.half_width() + r.section.sidewalk_metres,
                    r.from_node,
                    r.to_node
                );
            }
        }
        for group in &geometry.meshes {
            if group.instance_of.is_some() {
                continue;
            }
            let p = &group.positions;
            for tri in group.indices.chunks(3) {
                let q: Vec<[f32; 3]> = tri
                    .iter()
                    .map(|&i| {
                        [
                            p[i as usize * 3],
                            p[i as usize * 3 + 1],
                            p[i as usize * 3 + 2],
                        ]
                    })
                    .collect();
                let area = (q[1][0] - q[0][0]) * (q[2][2] - q[0][2])
                    - (q[2][0] - q[0][0]) * (q[1][2] - q[0][2]);
                if area.abs() < 1e-6 {
                    continue;
                }
                let w0 = ((q[1][0] - px) * (q[2][2] - pz) - (q[2][0] - px) * (q[1][2] - pz)) / area;
                let w1 = ((q[2][0] - px) * (q[0][2] - pz) - (q[0][0] - px) * (q[2][2] - pz)) / area;
                let w2 = 1.0 - w0 - w1;
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let y = w0 * q[0][1] + w1 * q[1][1] + w2 * q[2][1];
                    println!("{} y={y:.3} tri {:?}", group.material, q);
                }
            }
        }
        return Ok(());
    }

    let mut image = vec![[0.10_f32, 0.30, 0.12]; px * px];
    let mut depth = vec![f32::MIN; px * px];
    let scale = px as f32 / span;
    let to_px = |x: f32, z: f32| {
        (
            (x - centre.x) * scale + px as f32 * 0.5,
            (z - centre.y) * scale + px as f32 * 0.5,
        )
    };
    let light = [0.35_f32, 0.85, 0.40];
    for group in &geometry.meshes {
        if group.instance_of.is_some() {
            continue;
        }
        let Some(base) = colour(&group.material) else {
            continue;
        };
        let p = &group.positions;
        for tri in group.indices.chunks(3) {
            let v: Vec<[f32; 3]> = tri
                .iter()
                .map(|&i| {
                    let i = i as usize * 3;
                    [p[i], p[i + 1], p[i + 2]]
                })
                .collect();
            if v.iter().any(|q| q[1] > ceiling) {
                continue;
            }
            let e1 = [v[1][0] - v[0][0], v[1][1] - v[0][1], v[1][2] - v[0][2]];
            let e2 = [v[2][0] - v[0][0], v[2][1] - v[0][1], v[2][2] - v[0][2]];
            let n = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];
            let nl = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-9);
            let shade =
                0.55 + 0.45 * ((n[0] * light[0] + n[1] * light[1] + n[2] * light[2]) / nl).abs();
            let c = [base[0] * shade, base[1] * shade, base[2] * shade];
            let s: Vec<(f32, f32)> = v.iter().map(|q| to_px(q[0], q[2])).collect();
            let (x0, x1) = (
                s.iter()
                    .map(|q| q.0)
                    .fold(f32::MAX, f32::min)
                    .floor()
                    .max(0.0) as i64,
                s.iter()
                    .map(|q| q.0)
                    .fold(f32::MIN, f32::max)
                    .ceil()
                    .min(px as f32 - 1.0) as i64,
            );
            let (y0, y1) = (
                s.iter()
                    .map(|q| q.1)
                    .fold(f32::MAX, f32::min)
                    .floor()
                    .max(0.0) as i64,
                s.iter()
                    .map(|q| q.1)
                    .fold(f32::MIN, f32::max)
                    .ceil()
                    .min(px as f32 - 1.0) as i64,
            );
            let area =
                (s[1].0 - s[0].0) * (s[2].1 - s[0].1) - (s[2].0 - s[0].0) * (s[1].1 - s[0].1);
            if area.abs() < 1e-6 {
                continue;
            }
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
                    let w0 = ((s[1].0 - fx) * (s[2].1 - fy) - (s[2].0 - fx) * (s[1].1 - fy)) / area;
                    let w1 = ((s[2].0 - fx) * (s[0].1 - fy) - (s[0].0 - fx) * (s[2].1 - fy)) / area;
                    let w2 = 1.0 - w0 - w1;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                        continue;
                    }
                    let h = w0 * v[0][1] + w1 * v[1][1] + w2 * v[2][1];
                    let k = y as usize * px + x as usize;
                    if h >= depth[k] {
                        depth[k] = h;
                        image[k] = c;
                    }
                }
            }
        }
    }
    let mut bytes = format!("P6 {px} {px} 255\n").into_bytes();
    for c in image {
        for channel in c {
            bytes.push((channel.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0) as u8);
        }
    }
    std::fs::write(&out, bytes)?;
    println!("wrote {out}");
    Ok(())
}

#[allow(dead_code)]
fn deviation_report(city: &urban::ModernCity) {
    let nodes: Vec<[f32; 2]> = city
        .nodes
        .iter()
        .map(|n| city.frame.to_local(n.point))
        .collect();
    for (sd, hd) in city.sd_roads.iter().zip(&city.hd_roads) {
        let a = nodes[sd.from as usize];
        let b = nodes[sd.to as usize];
        let d = [b[0] - a[0], b[1] - a[1]];
        let l = (d[0] * d[0] + d[1] * d[1]).sqrt().max(1e-3);
        let mut worst: f32 = 0.0;
        for p in &hd.centreline {
            let q = city.frame.to_local(*p);
            let off = ((q[0] - a[0]) * d[1] - (q[1] - a[1]) * d[0]).abs() / l;
            worst = worst.max(off);
        }
        let first = city.frame.to_local(hd.centreline[0]);
        let gap = ((first[0] - a[0]).powi(2) + (first[1] - a[1]).powi(2))
            .sqrt()
            .min(((first[0] - b[0]).powi(2) + (first[1] - b[1]).powi(2)).sqrt());
        if worst > 1.0 || gap > 1.0 {
            println!(
                "road {} {:?} {:.0},{:.0}->{:.0},{:.0} off {:.1} endgap {:.1} pts {}",
                sd.id,
                sd.class,
                a[0],
                a[1],
                b[0],
                b[1],
                worst,
                gap,
                hd.centreline.len()
            );
        }
    }
}

#[allow(dead_code)]
fn roads_near(city: &urban::ModernCity, x: f32, z: f32, r: f32) {
    let nodes: Vec<[f32; 2]> = city
        .nodes
        .iter()
        .map(|n| city.frame.to_local(n.point))
        .collect();
    for (sd, hd) in city.sd_roads.iter().zip(&city.hd_roads) {
        let pts: Vec<[f32; 2]> = hd
            .centreline
            .iter()
            .map(|p| city.frame.to_local(*p))
            .collect();
        if pts.iter().any(|p| (p[0] - x).hypot(p[1] - z) < r) {
            println!(
                "sd {} hd {} {:?}/{:?} oneway {} layer {} nodes {:?}->{:?} first {:?} last {:?} n {} pts {:?}",
                sd.id,
                hd.id,
                sd.class,
                hd.class,
                hd.oneway,
                hd.layer,
                nodes[sd.from as usize],
                nodes[sd.to as usize],
                pts[0],
                pts[pts.len() - 1],
                pts.len(),
                &pts[..pts.len().min(6)]
            );
        }
    }
}

#[allow(dead_code)]
fn junction_report(network: &city_scene::Network, node: u32) {
    let Some(j) = network.junction(node) else {
        return;
    };
    for p in &j.ports {
        let r = network.road(p.road).unwrap();
        println!(
            "port road {} at_start {} dir {:?} left {:?} right {:?} trims {:.1}/{:.1} len {:.1}",
            p.road,
            p.at_start,
            p.dir,
            p.left,
            p.right,
            r.trim_start,
            r.trim_end,
            r.centreline.length()
        );
    }
    for c in &j.corners {
        println!("corner kerb {:?}", c.kerb);
    }
    let ring = &j.ring;
    let area = city_scene::math::signed_area(ring).abs();
    let tri: f32 = city_scene::math::triangulate(ring)
        .iter()
        .map(|[a, b, c]| city_scene::math::signed_area(&[ring[*a], ring[*b], ring[*c]]).abs())
        .sum();
    println!(
        "ring points {} area {area:.1} triangulated {tri:.1}",
        ring.len()
    );
    for (i, p) in ring.iter().enumerate() {
        println!("  {i} {:.3} {:.3}", p.x, p.y);
    }
}

#[allow(dead_code)]
fn road_report(network: &city_scene::Network, id: u32) {
    let Some(r) = network.road(id) else { return };
    println!(
        "road {} from {} (air {}) to {} (air {}) len {:.1}",
        id,
        r.from_node,
        network.is_air(r.from_node),
        r.to_node,
        network.is_air(r.to_node),
        r.carriageway.length()
    );
    let p = r.carriageway.points();
    // Each vertex with how sharply the carriageway turns there.
    let mut station = 0.0;
    for k in 1..p.len().saturating_sub(1) {
        station += ((p[k].x - p[k - 1].x).powi(2) + (p[k].z - p[k - 1].z).powi(2)).sqrt();
        let (ax, az) = (p[k].x - p[k - 1].x, p[k].z - p[k - 1].z);
        let (bx, bz) = (p[k + 1].x - p[k].x, p[k + 1].z - p[k].z);
        let turn = (ax * bz - az * bx).atan2(ax * bx + az * bz).to_degrees();
        if turn.abs() > 8.0 {
            println!(
                "  vertex at {station:.1} m ({:.0},{:.0}) turns {turn:.0} deg",
                p[k].x, p[k].z
            );
        }
    }
    println!(
        "  first {:?} {:?} last {:?}",
        p[0],
        p.get(1),
        p[p.len() - 1]
    );
    for lid in &r.lane_ids {
        let l = network.lane(lid).unwrap();
        println!(
            "  lane {} succ {:?} pred {:?} start {:?} end {:?}",
            lid,
            l.successors,
            l.predecessors,
            l.path.start(),
            l.path.end()
        );
    }
    for node in [r.from_node, r.to_node] {
        for other in network.incident_roads(node) {
            let o = network.road(other).unwrap();
            println!(
                "  at node {node}: road {other} oneway {} start {:?} end {:?}",
                o.oneway,
                o.carriageway.start(),
                o.carriageway.end()
            );
        }
    }
}
