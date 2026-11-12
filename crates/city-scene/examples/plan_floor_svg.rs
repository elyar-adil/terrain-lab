//! Draw typical-floor plans from the seeded plan generator as one SVG sheet.
//!
//! cargo run --release -p city-scene --example plan_floor_svg -- \
//!     [--typology slab|tower|walkup|office] [--era soviet|estate|millennium|contemporary]
//!     [--seeds 4] [--first 1] [--storeys 11] [--lat 31] [--out plans.svg]
//!
//! North is up.  Windows are blue bars on the wall, balcony doors darker blue,
//! air-conditioner ledges grey, doors brown; each room is labelled with its
//! function and use area.

use city_scene::buildings::plan::{
    Func, OpeningKind, Plan, PlanEra, PlanSpec, Rect, Side, Typology,
};
use std::fmt::Write;

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let i = args.iter().position(|v| v == name)?;
    args.get(i + 1).cloned()
}

fn fill(f: Func) -> &'static str {
    match f {
        Func::Living => "#f6e3b4",
        Func::Dining => "#f1d9a0",
        Func::MasterBed => "#cfe3c4",
        Func::Bed => "#d9ead0",
        Func::Study => "#d9ead0",
        Func::Kitchen => "#f3c9b8",
        Func::Bath | Func::Wc | Func::Toilets => "#bcd7ea",
        Func::Hall | Func::Lobby => "#efefef",
        Func::Store | Func::Shaft => "#d8d4cc",
        Func::Balcony | Func::ServiceBalcony => "#f7f7fb",
        Func::Stair => "#e5c9e8",
        Func::Lift => "#c9c9d6",
        Func::Open => "#f4f1e8",
    }
}

fn label(f: Func) -> &'static str {
    match f {
        Func::Living => "living",
        Func::Dining => "dining",
        Func::MasterBed => "master",
        Func::Bed => "bed",
        Func::Study => "study",
        Func::Kitchen => "kitchen",
        Func::Bath => "bath",
        Func::Wc => "wc",
        Func::Hall => "hall",
        Func::Store => "store",
        Func::Balcony => "balcony",
        Func::ServiceBalcony => "svc bal",
        Func::Stair => "stair",
        Func::Lift => "lift",
        Func::Lobby => "lobby",
        Func::Shaft => "",
        Func::Open => "open plan",
        Func::Toilets => "wc",
    }
}

fn draw(out: &mut String, plan: &Plan, ox: f32, oy: f32, s: f32) {
    let h = plan.plate.d();
    let px = |x: f32| ox + x * s;
    let py = |y: f32| oy + (h - y) * s;
    let rect = |out: &mut String, r: &Rect, fillc: &str, stroke: &str, sw: f32| {
        let _ = write!(
            out,
            r#"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" fill="{fillc}" stroke="{stroke}" stroke-width="{sw}"/>"#,
            px(r.x0),
            py(r.y1),
            r.w() * s,
            r.d() * s
        );
    };
    for r in plan.rooms.iter().filter(|r| r.func.is_balcony()) {
        rect(
            out,
            &r.rect,
            if r.enclosed { "#dff0f7" } else { "#fafafc" },
            "#8a8a99",
            1.0,
        );
    }
    for r in plan.rooms.iter().filter(|r| !r.func.is_balcony()) {
        rect(out, &r.rect, fill(r.func), "#555", 1.4);
    }
    for (k, r) in plan.rooms.iter().enumerate() {
        let c = r.rect.centre();
        let t = label(r.func);
        if t.is_empty() {
            continue;
        }
        let fs = (s * 0.42).clamp(5.0, 11.0);
        let _ = write!(
            out,
            r##"<text x="{:.1}" y="{:.1}" font-size="{fs:.1}" text-anchor="middle" fill="#222">{t}</text>"##,
            px(c[0]),
            py(c[1]) - 1.0
        );
        if !r.func.is_balcony()
            && !matches!(r.func, Func::Open | Func::Lobby | Func::Stair | Func::Lift)
        {
            let _ = write!(
                out,
                r##"<text x="{:.1}" y="{:.1}" font-size="{:.1}" text-anchor="middle" fill="#666">{:.1}</text>"##,
                px(c[0]),
                py(c[1]) + fs,
                fs * 0.85,
                r.rect.area()
            );
        }
        let _ = k;
    }
    // The tower core annotations.
    for st in &plan.core.stairs {
        rect(out, st, "none", "#9c27b0", 1.0);
    }
    for lf in &plan.core.lifts {
        rect(out, lf, "none", "#555599", 1.0);
    }
    // Doors.
    for d in &plan.doors {
        let w = d.width * s;
        let (x, y) = (px(d.at[0]), py(d.at[1]));
        let colour = if d.kind == city_scene::buildings::plan::DoorKind::Entry {
            "#b00020"
        } else {
            "#8d5a2b"
        };
        if d.vertical_wall {
            let _ = write!(
                out,
                r#"<line x1="{x:.1}" y1="{:.1}" x2="{x:.1}" y2="{:.1}" stroke="{colour}" stroke-width="3"/>"#,
                y - w / 2.0,
                y + w / 2.0
            );
        } else {
            let _ = write!(
                out,
                r#"<line x1="{:.1}" y1="{y:.1}" x2="{:.1}" y2="{y:.1}" stroke="{colour}" stroke-width="3"/>"#,
                x - w / 2.0,
                x + w / 2.0
            );
        }
    }
    // Openings, drawn on the face.
    for o in &plan.openings {
        let (x, y, horizontal) = match o.side {
            Side::South => (px(o.along), py(plan.plate.y0), true),
            Side::North => (px(o.along), py(plan.plate.y1), true),
            Side::East => (px(plan.plate.x1), py(o.along), false),
            Side::West => (px(plan.plate.x0), py(o.along), false),
        };
        let w = o.width * s;
        let (colour, thick) = match o.kind {
            OpeningKind::Window | OpeningKind::Ribbon => ("#1e88e5", 5.0),
            OpeningKind::BalconyDoor => ("#0d47a1", 5.0),
            OpeningKind::Bay => ("#00acc1", 7.0),
            OpeningKind::StairWindow => ("#8e24aa", 5.0),
            OpeningKind::AcLedge => ("#757575", 6.0),
            OpeningKind::Curtain { .. } => ("#4fc3f7", 4.0),
        };
        if horizontal {
            let _ = write!(
                out,
                r#"<line x1="{:.1}" y1="{y:.1}" x2="{:.1}" y2="{y:.1}" stroke="{colour}" stroke-width="{thick}"/>"#,
                x - w / 2.0,
                x + w / 2.0
            );
        } else {
            let _ = write!(
                out,
                r#"<line x1="{x:.1}" y1="{:.1}" x2="{x:.1}" y2="{:.1}" stroke="{colour}" stroke-width="{thick}"/>"#,
                y - w / 2.0,
                y + w / 2.0
            );
        }
    }
    // Office column grid.
    if plan.spec.typology == Typology::Office {
        let g = if (plan.plate.w() / 9.0).fract().abs() < 0.01
            || (plan.plate.w() / 9.0).fract() > 0.99
        {
            9.0
        } else {
            8.4
        };
        let mut x = 0.0;
        while x <= plan.plate.w() + 0.01 {
            let mut y = 0.0;
            while y <= plan.plate.d() + 0.01 {
                let _ = write!(
                    out,
                    r##"<rect x="{:.1}" y="{:.1}" width="4" height="4" fill="#222"/>"##,
                    px(x) - 2.0,
                    py(y) - 2.0
                );
                y += g;
            }
            x += g;
        }
    }
}

fn main() {
    let typology = match flag("--typology").as_deref().unwrap_or("slab") {
        "tower" => Typology::TowerFourUnit,
        "walkup" => Typology::WalkUp,
        "office" => Typology::Office,
        _ => Typology::SlabTwoUnit,
    };
    let era = match flag("--era").as_deref().unwrap_or("contemporary") {
        "soviet" => PlanEra::Soviet,
        "estate" => PlanEra::Estate,
        "millennium" => PlanEra::Millennium,
        _ => PlanEra::Contemporary,
    };
    let count: u32 = flag("--seeds").and_then(|v| v.parse().ok()).unwrap_or(4);
    let first: u32 = flag("--first").and_then(|v| v.parse().ok()).unwrap_or(1);
    let storeys: u16 = flag("--storeys").and_then(|v| v.parse().ok()).unwrap_or(11);
    let latitude: f32 = flag("--lat").and_then(|v| v.parse().ok()).unwrap_or(31.0);
    let path = flag("--out").unwrap_or_else(|| "plans.svg".to_string());

    let plans: Vec<Plan> = (first..first + count)
        .map(|seed| {
            Plan::generate(PlanSpec {
                typology,
                era,
                latitude,
                storeys,
                seed,
            })
        })
        .collect();
    let widest = plans.iter().map(|p| p.plate.w()).fold(0.0, f32::max) + 6.0;
    let deepest = plans.iter().map(|p| p.plate.d()).fold(0.0, f32::max) + 9.0;
    let cols = if widest < 30.0 { 2 } else { 1 };
    let rows = (plans.len() as u32).div_ceil(cols);
    let s = if typology == Typology::Office {
        9.0
    } else {
        20.0
    };
    let (cw, ch) = (widest * s, deepest * s);
    let mut svg = String::new();
    let _ = write!(
        svg,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{:.0}" height="{:.0}" font-family="sans-serif"><rect width="100%" height="100%" fill="#fff"/>"##,
        cw * cols as f32,
        ch * rows as f32
    );
    for (k, p) in plans.iter().enumerate() {
        let (c, r) = (k as u32 % cols, k as u32 / cols);
        let (ox, oy) = (c as f32 * cw + 3.0 * s, r as f32 * ch + 5.0 * s);
        let title = format!(
            "{:?} {:?} seed {}  {:.1} x {:.1} m  floor {:.1} m  azimuth {:+.1}  redraws {}",
            p.spec.typology,
            p.spec.era,
            p.spec.seed,
            p.plate.w(),
            p.plate.d(),
            p.floor_height,
            p.azimuth_deg,
            p.attempts
        );
        let _ = write!(
            svg,
            r##"<text x="{ox:.0}" y="{:.0}" font-size="13" fill="#111">{title}</text>"##,
            oy - 2.2 * s
        );
        let units: Vec<String> = p
            .units
            .iter()
            .map(|u| format!("{}b {:.0}m2", u.bedrooms, u.use_area))
            .collect();
        let _ = write!(
            svg,
            r##"<text x="{ox:.0}" y="{:.0}" font-size="11" fill="#444">units: {}   evac {:.1} m   WWR S {:.2} N {:.2}   (N up)</text>"##,
            oy - 0.8 * s,
            units.join(", "),
            p.evac_max,
            p.wwr(Side::South),
            p.wwr(Side::North)
        );
        draw(&mut svg, p, ox, oy, s);
    }
    svg.push_str("</svg>");
    std::fs::write(&path, svg).expect("write svg");
    println!("wrote {path}");
}
