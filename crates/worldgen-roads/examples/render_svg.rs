//! Draw a window of the road network as SVG, for looking at.
//!
//!   cargo run -p worldgen-roads --example render_svg -- OUT.svg [x y size_m [seed [min_class]]]

use std::fmt::Write as _;
use std::sync::Arc;

use worldgen_contracts::{PinnedRoad, PinnedSet, Polyline, PolylineRiver, RoadClass, Setting, V2, v2};
use worldgen_core::{Cell, Frame, Seed};
use worldgen_roads::{Fields, HashedTowns, RoadsConfig, engine, ROADS};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "roads.svg".into());
    let num = |i: usize, d: f64| args.get(i).and_then(|s| s.parse().ok()).unwrap_or(d);
    let seed = num(5, 7.0) as u64;
    let (mut cx, mut cy, size) = (num(2, 20000.0), num(3, 20000.0), num(4, 6000.0));
    if args.get(2).map(String::as_str) == Some("city") {
        // Find the n-th densest place within 40 km of (20 km, 20 km) and centre on it.
        let towns = HashedTowns::new(Seed::new(seed));
        let nth = num(3, 0.0) as usize;
        let mut best: Vec<(f64, f64, f64)> = Vec::new();
        for gy in 0..200 {
            for gx in 0..200 {
                let p = v2(gx as f64 * 400.0, gy as f64 * 400.0);
                let u = worldgen_contracts::UrbanField::urbanness(&towns, p);
                if u > 0.97 && best.iter().all(|b| (b.0 - p.x).hypot(b.1 - p.y) > 6000.0) {
                    best.push((p.x, p.y, u));
                }
            }
        }
        let (x, y, _) = best[nth.min(best.len() - 1)];
        eprintln!("{} dense places found; using ({x}, {y})", best.len());
        cx = x;
        cy = y;
    }
    let min_class = match args.get(6).map(String::as_str) {
        Some("arterial") => RoadClass::Arterial,
        Some("collector") => RoadClass::Collector,
        Some("local") => RoadClass::Local,
        Some("service") => RoadClass::Service,
        _ => RoadClass::Track,
    };
    let config = RoadsConfig { min_class, ..RoadsConfig::default() };
    let frame = Frame::new([0.0, 0.0], 1_048_576.0);
    let river = PolylineRiver {
        id: 1,
        line: (0..60).map(|k| v2(cx - size * 0.7 + k as f64 * size * 0.025, cy + (k as f64 * 0.35).sin() * size * 0.08 - size * 0.1)).collect(),
        width_m: 38.0,
    };
    // A long arterial a router might have laid, bending across the window.
    let given = PinnedRoad {
        id: 0xA11,
        class: RoadClass::Arterial,
        path: worldgen_roads::shape::round_corners(
            &Polyline(vec![
                v2(cx - size * 0.9, cy - size * 0.35),
                v2(cx - size * 0.3, cy - size * 0.12),
                v2(cx + size * 0.05, cy + size * 0.02),
                v2(cx + size * 0.35, cy + size * 0.22),
                v2(cx + size * 0.9, cy + size * 0.3),
            ]),
            450.0,
            20.0,
        ),
    };
    let mut fields = Fields::new(HashedTowns::shared(Seed::new(seed))).with_water(Arc::new(river.clone()));
    if std::env::var("GIVEN").is_ok() {
        fields = fields.with_pinned(Arc::new(PinnedSet::new(vec![given])));
    }
    let e = engine(Seed::new(seed), frame, config, fields).unwrap();

    // The window, as tiles at a level that fits a few of them.
    let level = ((frame.root_size_m / (size / 2.0)).log2().floor() as u8).clamp(1, 20);
    let (x0, y0, x1, y1) = (cx - size / 2.0, cy - size / 2.0, cx + size / 2.0, cy + size / 2.0);
    let (lo, hi) = (Cell::containing(&frame, [x0, y0], level), Cell::containing(&frame, [x1, y1], level));
    let mut svg = String::new();
    let px = 1600.0;
    let k = px / size;
    let tx = |p: V2| ((p.x - x0) * k, (y1 - p.y) * k);
    writeln!(svg, r##"<svg xmlns="http://www.w3.org/2000/svg" width="{px}" height="{px}" viewBox="0 0 {px} {px}"><rect width="{px}" height="{px}" fill="#e9e4d6"/>"##).unwrap();
    // River.
    let pts: Vec<String> = river.line.iter().map(|p| { let (x, y) = tx(*p); format!("{x:.1},{y:.1}") }).collect();
    writeln!(svg, r##"<polyline points="{}" fill="none" stroke="#9ec3dd" stroke-width="{:.1}" stroke-linecap="round"/>"##, pts.join(" "), river.width_m * k).unwrap();
    let mut count = 0;
    let mut layers: Vec<(RoadClass, String)> = Vec::new();
    for j in lo.y..=hi.y {
        for i in lo.x..=hi.x {
            let tile = e.get::<worldgen_contracts::RoadTile>(ROADS, Cell::new(level, i, j)).unwrap();
            for edge in &tile.edges {
                count += 1;
                for piece in &edge.pieces {
                    let pts: Vec<String> = piece.0.iter().map(|p| { let (x, y) = tx(*p); format!("{x:.1},{y:.1}") }).collect();
                    let w = edge.class.cross_section(edge.setting).width() * k;
                    let colour = match (edge.class, edge.setting) {
                        (RoadClass::Arterial | RoadClass::Motorway, _) => "#c9573b",
                        (RoadClass::Collector, Setting::Urban) => "#d89a3c",
                        (RoadClass::Collector, _) => "#b88c5a",
                        (RoadClass::Local, _) => "#ffffff",
                        _ => "#f6f1e2",
                    };
                    let bridge = !edge.spans.is_empty();
                    let casing = if bridge { "#2f3b52" } else { "#8d867a" };
                    layers.push((edge.class, format!(
                        r##"<polyline points="{pts}" fill="none" stroke="{casing}" stroke-width="{:.2}" stroke-linecap="round" stroke-linejoin="round"/><polyline points="{pts}" fill="none" stroke="{colour}" stroke-width="{:.2}" stroke-linecap="round" stroke-linejoin="round"/>"##,
                        (w + 1.4).max(1.2), w.max(0.8), pts = pts.join(" ")
                    )));
                }
            }
        }
    }
    layers.sort_by_key(|(c, _)| *c);
    for (_, s) in layers { svg.push_str(&s); svg.push('\n'); }
    svg.push_str("</svg>\n");
    std::fs::write(&out, svg).unwrap();
    eprintln!("{out}: {count} edges, tile level {level}, computed {} products", e.computed());
}
