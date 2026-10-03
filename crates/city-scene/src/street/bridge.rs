//! The structure that carries a street over something else: parapets with a
//! capping rail, and piers on real ground.

use crate::math::{Vec2, Vec3};
use crate::mesh::{GroupStyle, MeshBuilder};
use crate::network::Road;
use worldgen_core::hash::cell01;

use super::{Carriageway, offset_path, sweep};

/// Parapets, piers and abutments for an elevated road.
///
/// The previous renderer lifted a bridge ribbon six metres and left it floating.
/// A viaduct without a deck edge and piers reads as a road pasted onto the sky,
/// which is worse than not drawing the bridge at all.
pub(super) fn bridge_structure(road: &Road, builder: &mut MeshBuilder) {
    if road.layer == 0 {
        return;
    }
    let path = road.carriageway.clone();
    declare_materials(builder);
    let length = path.length();
    if length < 4.0 {
        return;
    }
    let surface = Carriageway::on_path(path, road.half_width());
    let half = road.half_width();
    // Parapet: a capping ribbon plus its full-height face on both sides.
    for side in [-1.0_f32, 1.0] {
        let edge = side * (half - 0.3);
        let cap = offset_path(&surface, edge);
        builder.ribbon(
            "bridge.concrete",
            &cap,
            -0.18,
            0.18,
            0.0,
            length,
            0.85,
            None,
        );
        sweep(
            builder,
            "bridge.concrete",
            &Carriageway::on_path(cap, 0.2),
            -0.18,
            0.18,
            0.0,
            length,
            0.85,
        );
    }
    if road.bridge {
        river_bridge(road, &surface, length, builder);
        return;
    }
    // Piers every 28 m, skipping any that would land in the carriageway of
    // another road — the same avoidance the source kernel applies.
    let mut station = 14.0;
    while station < length - 12.0 {
        let (position, tangent) = surface.path.sample(station);
        let top = position.y - road.deck_thickness;
        if top > 1.2 {
            let height = top;
            let normal = tangent.left_normal();
            let base = crate::math::Vec3::new(
                position.x - normal.x * 0.7,
                0.0,
                position.z - normal.y * 0.7,
            );
            let top_point = crate::math::Vec3::new(
                position.x - normal.x * 0.7,
                height,
                position.z - normal.y * 0.7,
            );
            builder.tube("bridge.concrete", base, top_point, 0.7, 0.6, 6, None);
            // Pier cap, spread to carry the deck.
            let cap_a = crate::math::Vec3::new(
                position.x - normal.x * 1.1,
                height,
                position.z - normal.y * 1.1,
            );
            let cap_b = crate::math::Vec3::new(
                position.x + normal.x * 1.1,
                height,
                position.z + normal.y * 1.1,
            );
            builder.tube("bridge.concrete", cap_a, cap_b, 0.6, 0.6, 4, None);
        }
        station += 28.0;
    }
}

const STONE: [f32; 3] = [0.60, 0.56, 0.50];
const PIER: [f32; 3] = [0.50, 0.49, 0.46];
const DARK: [f32; 3] = [0.10, 0.11, 0.12];
const STEEL_M: &str = "bridge.steel";
const STONE_M: &str = "bridge.stone";

/// Painted-steel colours a bridge can wear: international orange, aluminium
/// grey, sea green, off white, graphite.
const PAINTS: [[f32; 3]; 5] = [
    [0.62, 0.16, 0.10],
    [0.66, 0.68, 0.70],
    [0.24, 0.42, 0.34],
    [0.84, 0.84, 0.80],
    [0.30, 0.33, 0.37],
];

fn declare_materials(builder: &mut MeshBuilder) {
    builder.style(
        STONE_M,
        GroupStyle {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Girder,
    Truss,
    Arch,
    Stayed,
    Suspension,
}

fn choose_kind(length: f32, roll: f32) -> Kind {
    if length < 45.0 {
        Kind::Girder
    } else if length < 75.0 {
        if roll < 0.5 { Kind::Truss } else { Kind::Arch }
    } else if roll < 0.15 {
        Kind::Truss
    } else if roll < 0.42 {
        Kind::Suspension
    } else if roll < 0.72 {
        Kind::Stayed
    } else {
        Kind::Arch
    }
}

fn scale(c: [f32; 3], k: f32) -> [f32; 3] {
    [c[0] * k, c[1] * k, c[2] * k]
}

/// A box in its own frame. `yaw` turns local +x onto the deck tangent, so `w`
/// runs along the deck and `d` across it.
#[allow(clippy::too_many_arguments)]
fn cuboid(
    b: &mut MeshBuilder,
    mat: &str,
    c: Vec2,
    y0: f32,
    y1: f32,
    w: f32,
    d: f32,
    yaw: f32,
    col: [f32; 3],
) {
    if y1 - y0 < 1.0e-3 {
        return;
    }
    let (sin, cos) = yaw.sin_cos();
    let pt = |x: f32, z: f32| Vec2::new(c.x + x * cos - z * sin, c.y + x * sin + z * cos);
    let (hw, hd) = (w * 0.5, d * 0.5);
    let k = [pt(-hw, -hd), pt(hw, -hd), pt(hw, hd), pt(-hw, hd)];
    for i in 0..4 {
        b.wall(mat, k[i], k[(i + 1) % 4], y0, y1, Some(col));
    }
    let top = |p: Vec2| Vec3::from_plan(p, y1);
    b.quad(mat, top(k[3]), top(k[2]), top(k[1]), top(k[0]), Some(col));
}

/// A convex six-point prism (a lens) from a CCW outline in the local frame.
#[allow(clippy::too_many_arguments)]
fn lens_prism(
    b: &mut MeshBuilder,
    mat: &str,
    c: Vec2,
    yaw: f32,
    len: f32,
    wid: f32,
    y0: f32,
    y1: f32,
    col: [f32; 3],
) {
    let (l, w) = (len * 0.5, wid * 0.5);
    let tip = (wid * 1.1).min(l * 0.5);
    let pts = [
        (-l, 0.0),
        (-l + tip, -w),
        (l - tip, -w),
        (l, 0.0),
        (l - tip, w),
        (-l + tip, w),
    ];
    let (sin, cos) = yaw.sin_cos();
    let k: Vec<Vec2> = pts
        .iter()
        .map(|&(x, z)| Vec2::new(c.x + x * cos - z * sin, c.y + x * sin + z * cos))
        .collect();
    for i in 0..6 {
        b.wall(mat, k[i], k[(i + 1) % 6], y0, y1, Some(col));
    }
    let top = |p: Vec2| Vec3::from_plan(p, y1);
    b.quad(mat, top(k[2]), top(k[1]), top(k[0]), top(k[5]), Some(col));
    b.quad(mat, top(k[5]), top(k[4]), top(k[3]), top(k[2]), Some(col));
}

fn tube(b: &mut MeshBuilder, mat: &str, a: Vec3, c: Vec3, r: f32, col: [f32; 3]) {
    b.tube(mat, a, c, r, r, 5, Some(col));
}

/// A deck over water. Kind, colour and proportions come from a hash of where the
/// bridge is and how long it is, so a city has a mix of structures and the same
/// seed always gives the same ones.
fn river_bridge(road: &Road, surface: &Carriageway, length: f32, builder: &mut MeshBuilder) {
    let half = road.half_width();
    let depth = (road.deck_thickness * 2.2).max(1.6);
    let start = surface.path.sample(0.0).0;
    let (ha, hb) = ((start.x * 0.5) as i32, (start.z * 0.5) as i32);
    let lu = length as u32;
    let kind = choose_kind(length, cell01(0x5B1D, ha, hb, lu));
    let paint = PAINTS[(cell01(0x77A1, ha, hb, lu) * PAINTS.len() as f32) as usize % PAINTS.len()];
    let stone_towers = cell01(0x1234, ha, hb, lu) < 0.5;
    let a_frame = cell01(0x9911, ha, hb, lu) < 0.5;
    let dy = |st: f32| surface.path.sample(st.clamp(0.0, length)).0.y;
    let yaw_at = |st: f32| {
        let t = surface.path.sample(st.clamp(0.0, length)).1;
        t.y.atan2(t.x)
    };
    let plan = |st: f32, off: f32| {
        let p = surface.point(st.clamp(0.0, length), off, 0.0);
        Vec2::new(p.x, p.z)
    };
    // A point at signed lateral offset `off` and absolute height `y`.
    let atx = |st: f32, off: f32, y: f32| {
        let p = surface.point(st.clamp(0.0, length), off, 0.0);
        Vec3::new(p.x, y, p.z)
    };

    // Land approach: where the deck is low it stands on a solid retaining-wall
    // fill instead of a girder, so the ramp meets the ground and never floats.
    let steps = ((length / 3.0).ceil() as usize).max(2);
    let step = length / steps as f32;
    let low = |st: f32| dy(st) - depth < 1.4;
    let mut fill_lo = 0usize;
    while fill_lo < steps
        && (fill_lo as f32 + 1.0) * step <= length * 0.25
        && low((fill_lo as f32 + 1.0) * step)
    {
        fill_lo += 1;
    }
    let mut fill_hi = steps;
    while fill_hi > fill_lo
        && (fill_hi as f32 - 1.0) * step >= length * 0.75
        && low((fill_hi as f32 - 1.0) * step)
    {
        fill_hi -= 1;
    }
    let is_fill = |seg: usize| seg < fill_lo || seg >= fill_hi;

    let mut prev: Option<(Vec3, Vec3)> = None;
    for k in 0..=steps {
        let st = step * k as f32;
        let l = surface.point(st, -half, 0.0);
        let r = surface.point(st, half, 0.0);
        if let Some((pl, pr)) = prev {
            let seg = k - 1;
            if is_fill(seg) {
                let s_prev = step * seg as f32;
                for (a, b, sgn) in [(pl, l, -1.0_f32), (pr, r, 1.0)] {
                    for (p, q) in [(a, b), (b, a)] {
                        builder.quad(
                            STONE_M,
                            Vec3::new(p.x, -0.4, p.z),
                            Vec3::new(q.x, -0.4, q.z),
                            Vec3::new(q.x, q.y - 0.4, q.z),
                            Vec3::new(p.x, p.y - 0.4, p.z),
                            Some(STONE),
                        );
                    }
                    // Coping: a proud band under the parapet, seen from both sides.
                    let push = |v: Vec3, s: f32| {
                        let n = surface.path.sample(s).1.left_normal();
                        Vec3::new(v.x + n.x * 0.16 * sgn, v.y, v.z + n.y * 0.16 * sgn)
                    };
                    let (a2, b2) = (push(a, s_prev), push(b, st));
                    let lo = |v: Vec3| Vec3::new(v.x, v.y - 0.4, v.z);
                    let cope = scale(STONE, 1.12);
                    builder.quad(STONE_M, lo(a2), lo(b2), b2, a2, Some(cope));
                    builder.quad(STONE_M, a2, b2, lo(b2), lo(a2), Some(cope));
                    builder.quad(STONE_M, a2, b2, b, a, Some(cope));
                    builder.quad(STONE_M, a, b, b2, a2, Some(cope));
                    builder.quad(
                        STONE_M,
                        lo(a),
                        lo(b),
                        lo(b2),
                        lo(a2),
                        Some(scale(STONE, 0.7)),
                    );
                    builder.quad(
                        STONE_M,
                        lo(a2),
                        lo(b2),
                        lo(b),
                        lo(a),
                        Some(scale(STONE, 0.7)),
                    );
                }
            } else {
                for (a, b) in [(pl, l), (pr, r)] {
                    // Sloped top and bottom: the fascia follows the ramp instead of stepping.
                    let (a0, b0) = (
                        Vec3::new(a.x, a.y - depth, a.z),
                        Vec3::new(b.x, b.y - depth, b.z),
                    );
                    builder.quad(STEEL_M, a0, b0, b, a, Some(paint));
                    builder.quad(STEEL_M, b0, a0, a, b, Some(paint));
                }
                builder.quad(
                    STEEL_M,
                    Vec3::new(pl.x, pl.y - depth, pl.z),
                    Vec3::new(pr.x, pr.y - depth, pr.z),
                    Vec3::new(r.x, r.y - depth, r.z),
                    Vec3::new(l.x, l.y - depth, l.z),
                    Some(scale(paint, 0.45)),
                );
            }
        }
        prev = Some((l, r));
    }
    // Abutment faces where the fill meets the girder: closes the gap beneath.
    for (edge, on) in [(fill_lo, fill_lo > 0), (fill_hi, fill_hi < steps)] {
        if !on {
            continue;
        }
        let st = step * edge as f32;
        let (l, r) = (surface.point(st, -half, 0.0), surface.point(st, half, 0.0));
        let (bl, br) = (Vec3::new(l.x, -0.4, l.z), Vec3::new(r.x, -0.4, r.z));
        let (tl, tr) = (
            Vec3::new(l.x, l.y - depth, l.z),
            Vec3::new(r.x, r.y - depth, r.z),
        );
        builder.quad(STONE_M, bl, br, tr, tl, Some(PIER));
        builder.quad(STONE_M, br, bl, tl, tr, Some(PIER));
    }

    // Expansion joints: dark strips across the full width, just proud of the crown.
    let mut st = 12.0;
    while st < length - 8.0 {
        let n = 10;
        for i in 0..n {
            let o0 = -half + 0.3 + (2.0 * half - 0.6) * i as f32 / n as f32;
            let o1 = -half + 0.3 + (2.0 * half - 0.6) * (i + 1) as f32 / n as f32;
            builder.quad(
                STEEL_M,
                surface.point(st, o0, 0.06),
                surface.point(st, o1, 0.06),
                surface.point(st + 0.35, o1, 0.06),
                surface.point(st + 0.35, o0, 0.06),
                Some(DARK),
            );
        }
        st += 24.0;
    }
    // Lamp posts on the parapet, arms reaching over the carriageway.
    let lamp_h = if kind == Kind::Truss { 6.5 } else { 8.0 };
    let post = [0.34, 0.36, 0.38];
    let mut st = 10.0;
    while st < length - 6.0 {
        for sgn in [-1.0_f32, 1.0] {
            let base = surface.point(st, sgn * (half - 0.3), 0.85);
            let n = surface.path.sample(st).1.left_normal();
            let top = Vec3::new(base.x, base.y + lamp_h, base.z);
            builder.tube(STEEL_M, base, top, 0.11, 0.07, 6, Some(post));
            let arm = Vec3::new(
                top.x - n.x * sgn * 1.8,
                top.y + 0.35,
                top.z - n.y * sgn * 1.8,
            );
            tube(builder, STEEL_M, top, arm, 0.05, post);
            cuboid(
                builder,
                STEEL_M,
                Vec2::new(arm.x, arm.z),
                arm.y - 0.12,
                arm.y + 0.02,
                0.9,
                0.35,
                yaw_at(st),
                [0.95, 0.93, 0.80],
            );
        }
        st += 22.0;
    }

    // Piers with cutwaters under the girder types; cable and arch bridges span
    // clear.
    if kind == Kind::Girder || kind == Kind::Truss {
        let spans = ((length / 34.0).round() as usize).max(1);
        for i in 1..spans {
            let st = length * i as f32 / spans as f32;
            if st < step * fill_lo as f32 + 2.0 || st > step * fill_hi as f32 - 2.0 {
                continue;
            }
            let (pos, tan) = surface.path.sample(st);
            let n = tan.left_normal();
            let top = pos.y - depth;
            if top < 1.5 {
                continue;
            }
            let yaw = n.y.atan2(n.x); // local x runs across the deck, along the flow
            for side in [-1.0_f32, 1.0] {
                let c = Vec2::new(
                    pos.x + n.x * half * 0.5 * side,
                    pos.z + n.y * half * 0.5 * side,
                );
                lens_prism(
                    builder,
                    STONE_M,
                    c,
                    yaw,
                    half * 0.85,
                    2.6,
                    -4.0,
                    top - 1.1,
                    PIER,
                );
                // Wider collar at the waterline; foam collects here.
                lens_prism(
                    builder,
                    STONE_M,
                    c,
                    yaw,
                    half * 0.85 + 1.6,
                    3.6,
                    -4.0,
                    0.9,
                    STONE,
                );
                if st > length * 0.3 && st < length * 0.7 {
                    // Soft foam where the current splits on the cutwater.
                    for (grow, y) in [(0.6_f32, 0.05_f32), (1.5, 0.055), (2.8, 0.06)] {
                        let (l, w) = (
                            (half * 0.85 + 1.6 + grow * 2.0) * 0.5,
                            (3.6 + grow * 2.0) * 0.5,
                        );
                        let tip = (w * 2.0 * 1.1).min(l * 0.5);
                        let (sin, cos) = yaw.sin_cos();
                        let ring: Vec<Vec2> = [
                            (-l, 0.0),
                            (-l + tip, -w),
                            (l - tip, -w),
                            (l, 0.0),
                            (l - tip, w),
                            (-l + tip, w),
                        ]
                        .iter()
                        .map(|&(x, z)| Vec2::new(c.x + x * cos - z * sin, c.y + x * sin + z * cos))
                        .collect();
                        let mut ring = ring;
                        if crate::math::signed_area(&ring) < 0.0 {
                            ring.reverse();
                        }
                        builder.ground_uv("water.foam", &ring, y, None);
                    }
                }
            }
            cuboid(
                builder,
                STONE_M,
                Vec2::new(pos.x, pos.z),
                top - 1.1,
                top,
                3.8,
                half * 2.0 + 1.0,
                yaw_at(st),
                PIER,
            );
        }
    }

    // Everything above the deck hangs from a plane just outside the parapet, with
    // a small cantilever bracket at each anchor.
    let c_off = half + 2.0;
    let bracket = |builder: &mut MeshBuilder, st: f32, sgn: f32| -> Vec3 {
        let a = half - 0.2;
        let b = c_off + 0.4;
        let mid = surface.point(st, sgn * (a + b) * 0.5, 0.0);
        cuboid(
            builder,
            STEEL_M,
            Vec2::new(mid.x, mid.z),
            mid.y - 0.8,
            mid.y - 0.15,
            0.7,
            b - a,
            yaw_at(st),
            paint,
        );
        surface.point(st, sgn * c_off, -0.15)
    };
    let hanger_col = scale(paint, 0.9);
    let cable_col = scale(paint, 0.8);

    match kind {
        Kind::Girder => {}
        Kind::Suspension => {
            let tower_h = (length * 0.28).clamp(20.0, 36.0);
            let towers = [length * 0.24, length * 0.76];
            let mut tops = [0.0_f32; 2];
            for (ti, &ts) in towers.iter().enumerate() {
                let base_y = dy(ts);
                let top = base_y + tower_h;
                tops[ti] = top;
                let yaw = yaw_at(ts);
                for sgn in [-1.0_f32, 1.0] {
                    let c = plan(ts, sgn * c_off);
                    if stone_towers {
                        cuboid(builder, STONE_M, c, -8.0, top, 3.2, 3.6, yaw, STONE);
                        cuboid(
                            builder,
                            STONE_M,
                            c,
                            top - 0.7,
                            top + 0.5,
                            4.0,
                            4.4,
                            yaw,
                            scale(STONE, 0.9),
                        );
                    } else {
                        builder.tube(
                            STEEL_M,
                            Vec3::new(c.x, -8.0, c.y),
                            Vec3::new(c.x, top + 0.5, c.y),
                            1.7,
                            1.05,
                            8,
                            Some(paint),
                        );
                    }
                }
                if stone_towers {
                    // Lintel with an arched portal below it.
                    let inner = c_off - 1.8;
                    cuboid(
                        builder,
                        STONE_M,
                        plan(ts, 0.0),
                        top - 2.4,
                        top - 0.7,
                        3.0,
                        c_off * 2.0,
                        yaw,
                        STONE,
                    );
                    let rise = inner.min(9.0) * 0.55;
                    let spring = top - 2.4 - rise;
                    let n_seg = 16;
                    let mut prev_p: Option<Vec3> = None;
                    for i in 0..=n_seg {
                        let a = std::f32::consts::PI * i as f32 / n_seg as f32;
                        let pt = atx(ts, -a.cos() * inner, spring + a.sin() * rise);
                        if let Some(q) = prev_p {
                            builder.tube(STONE_M, q, pt, 0.7, 0.7, 6, Some(STONE));
                        }
                        prev_p = Some(pt);
                    }
                } else {
                    for f in [0.62_f32, 0.93] {
                        tube(
                            builder,
                            STEEL_M,
                            atx(ts, -c_off, base_y + tower_h * f),
                            atx(ts, c_off, base_y + tower_h * f),
                            0.75,
                            paint,
                        );
                    }
                    tube(
                        builder,
                        STEEL_M,
                        atx(ts, -c_off, base_y + tower_h * 0.62),
                        atx(ts, c_off, base_y + tower_h * 0.93),
                        0.35,
                        paint,
                    );
                    tube(
                        builder,
                        STEEL_M,
                        atx(ts, c_off, base_y + tower_h * 0.62),
                        atx(ts, -c_off, base_y + tower_h * 0.93),
                        0.35,
                        paint,
                    );
                }
            }
            // Main cable: straight back stays, parabolic main span.
            let mid_y = dy(length * 0.5) + 2.4;
            let sag = (tops[0] + tops[1]) * 0.5 - mid_y;
            let cable_y = |s: f32| -> f32 {
                if s <= towers[0] {
                    let a0 = dy(0.0) + 1.4;
                    a0 + (tops[0] - 0.4 - a0) * (s / towers[0])
                } else if s >= towers[1] {
                    let a1 = dy(length) + 1.4;
                    a1 + (tops[1] - 0.4 - a1) * ((length - s) / (length - towers[1]))
                } else {
                    let u = (s - towers[0]) / (towers[1] - towers[0]);
                    (tops[0] + (tops[1] - tops[0]) * u) - 0.4 - sag * 4.0 * u * (1.0 - u)
                }
            };
            for sgn in [-1.0_f32, 1.0] {
                let n = 72;
                let mut prev_c: Option<Vec3> = None;
                for i in 0..=n {
                    let s = length * i as f32 / n as f32;
                    let pt = atx(s, sgn * c_off, cable_y(s));
                    if let Some(q) = prev_c {
                        builder.tube(STEEL_M, q, pt, 0.36, 0.36, 6, Some(cable_col));
                    }
                    prev_c = Some(pt);
                }
                let mut s = 4.0;
                while s < length - 3.0 {
                    let near_tower = towers.iter().any(|t| (s - t).abs() < 2.4);
                    if !near_tower && cable_y(s) - dy(s) > 1.6 {
                        let anchor = bracket(builder, s, sgn);
                        tube(
                            builder,
                            STEEL_M,
                            atx(s, sgn * c_off, cable_y(s)),
                            anchor,
                            0.07,
                            hanger_col,
                        );
                    }
                    s += 4.5;
                }
            }
        }
        Kind::Stayed => {
            let tower_h = (length * 0.30).clamp(22.0, 40.0);
            let towers = [length * 0.27, length * 0.73];
            let inset = if a_frame { 0.72 } else { 0.0 };
            for &ts in &towers {
                let base_y = dy(ts);
                let top = base_y + tower_h;
                // f = 0 at the deck, 1 at the top; A-frames lean inward.
                let leg_at = |sgn: f32, f: f32| {
                    atx(ts, sgn * c_off * (1.0 - inset * f), base_y + tower_h * f)
                };
                for sgn in [-1.0_f32, 1.0] {
                    let b0 = leg_at(sgn, 0.0);
                    builder.tube(
                        STEEL_M,
                        Vec3::new(b0.x, -8.0, b0.z),
                        leg_at(sgn, 1.0),
                        1.5,
                        0.85,
                        8,
                        Some(paint),
                    );
                }
                let _ = top;
                if a_frame {
                    tube(
                        builder,
                        STEEL_M,
                        leg_at(-1.0, 1.0),
                        leg_at(1.0, 1.0),
                        0.6,
                        paint,
                    );
                } else {
                    for f in [0.55_f32, 0.9] {
                        tube(
                            builder,
                            STEEL_M,
                            leg_at(-1.0, f),
                            leg_at(1.0, f),
                            0.7,
                            paint,
                        );
                    }
                }
                // Fans of stays per side: toward the bank and toward mid-span.
                let toward_mid = if ts < length * 0.5 { 1.0_f32 } else { -1.0 };
                for sgn in [-1.0_f32, 1.0] {
                    for dir in [-toward_mid, toward_mid] {
                        let limit = if dir == toward_mid {
                            length * 0.5 - 3.0
                        } else if dir < 0.0 {
                            3.5
                        } else {
                            length - 3.5
                        };
                        let mut k = 0;
                        let mut s = ts + dir * 6.0;
                        while (dir > 0.0 && s < limit) || (dir < 0.0 && s > limit) {
                            // Anchors farther out attach higher, so the fan clears itself.
                            let f = 0.58 + 0.38 * (k as f32 / 6.0).min(1.0);
                            let anchor = bracket(builder, s, sgn);
                            tube(builder, STEEL_M, leg_at(sgn, f), anchor, 0.09, hanger_col);
                            s += dir * 5.0;
                            k += 1;
                        }
                    }
                }
            }
        }
        Kind::Arch => {
            let s0 = length * 0.07;
            let s1 = length * 0.93;
            let y0 = dy(s0) + 0.6;
            let y1 = dy(s1) + 0.6;
            let rise = ((s1 - s0) * 0.22).clamp(10.0, 26.0);
            let rib_y = |s: f32| -> f32 {
                let u = ((s - s0) / (s1 - s0)).clamp(0.0, 1.0);
                y0 + (y1 - y0) * u + rise * 4.0 * u * (1.0 - u)
            };
            for sgn in [-1.0_f32, 1.0] {
                let n = 40;
                let mut prev_p: Option<Vec3> = None;
                for i in 0..=n {
                    let s = s0 + (s1 - s0) * i as f32 / n as f32;
                    let pt = atx(s, sgn * c_off, rib_y(s));
                    if let Some(q) = prev_p {
                        builder.tube(STEEL_M, q, pt, 0.75, 0.75, 6, Some(paint));
                    }
                    prev_p = Some(pt);
                }
                for s in [s0, s1] {
                    cuboid(
                        builder,
                        STONE_M,
                        plan(s, sgn * c_off),
                        -6.0,
                        dy(s) + 1.0,
                        3.4,
                        3.4,
                        yaw_at(s),
                        STONE,
                    );
                }
                let mut s = s0 + 4.0;
                while s < s1 - 3.0 {
                    if rib_y(s) - dy(s) > 1.8 {
                        let anchor = bracket(builder, s, sgn);
                        tube(
                            builder,
                            STEEL_M,
                            atx(s, sgn * c_off, rib_y(s)),
                            anchor,
                            0.07,
                            hanger_col,
                        );
                    }
                    s += 4.5;
                }
            }
            // Wind bracing across the top where it clears traffic.
            let mut s = s0 + 9.0;
            while s < s1 - 8.0 {
                if rib_y(s) - dy(s) > 8.0 {
                    tube(
                        builder,
                        STEEL_M,
                        atx(s, -c_off, rib_y(s) - 0.3),
                        atx(s, c_off, rib_y(s) - 0.3),
                        0.32,
                        paint,
                    );
                }
                s += 9.0;
            }
        }
        Kind::Truss => {
            let panels = ((length / 6.0).round() as usize).max(4);
            let h = 8.2;
            for sgn in [-1.0_f32, 1.0] {
                let mut prev_top: Option<Vec3> = None;
                let mut prev_bot: Option<Vec3> = None;
                for i in 0..=panels {
                    let s = length * i as f32 / panels as f32;
                    let bot = atx(s, sgn * c_off, dy(s) + 0.2);
                    let top = atx(s, sgn * c_off, dy(s) + h);
                    tube(builder, STEEL_M, bot, top, 0.22, paint);
                    if let (Some(pt), Some(pb)) = (prev_top, prev_bot) {
                        tube(builder, STEEL_M, pt, top, 0.42, paint);
                        tube(builder, STEEL_M, pb, bot, 0.34, paint);
                        let sp = length * (i - 1) as f32 / panels as f32;
                        // Pratt: diagonals lean toward mid-span.
                        let (lo, hi) = if s < length * 0.5 { (sp, s) } else { (s, sp) };
                        tube(
                            builder,
                            STEEL_M,
                            atx(lo, sgn * c_off, dy(lo) + 0.2),
                            atx(hi, sgn * c_off, dy(hi) + h),
                            0.2,
                            paint,
                        );
                        let a = bracket(builder, s, sgn);
                        tube(builder, STEEL_M, bot, a, 0.12, paint);
                    }
                    if i % 2 == 0 && i > 0 && i < panels && sgn > 0.0 {
                        tube(
                            builder,
                            STEEL_M,
                            atx(s, -c_off, dy(s) + h),
                            atx(s, c_off, dy(s) + h),
                            0.26,
                            paint,
                        );
                    }
                    prev_top = Some(top);
                    prev_bot = Some(bot);
                }
            }
        }
    }
}
