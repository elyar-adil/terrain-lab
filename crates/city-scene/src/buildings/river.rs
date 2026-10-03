//! The river: a smooth water surface with a depth gradient and soft foam, and
//! stone quay walls with a promenade, railing, steps and bollards wherever the
//! bank is urban. Park stretches keep a plain grass bank.

use urban::{CityFrameInfo, Point};

use crate::math::{Vec2, Vec3, signed_area};
use crate::mesh::{GroupStyle, MeshBuilder};
use worldgen_core::hash::cell01;
use worldgen_core::smooth01;

use super::ring_of;

const QUAY: &str = "quay.stone";
const FOAM: &str = "water.foam";
/// Walkway height above the water plane and the ground plate.
const TOP: f32 = 0.85;
const COPING: f32 = 0.98;
const WATER_Y: f32 = 0.03;

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// The river as a water surface plus its banks. `crossings` are (plan point,
/// radius) discs along every river bridge: the quay stops short of them so it
/// never buries a bridge approach.
pub fn build_water(
    river: &[Point],
    width: f32,
    frame: CityFrameInfo,
    crossings: &[(Vec2, f32)],
    builder: &mut MeshBuilder,
) {
    builder.style(
        "water",
        GroupStyle {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    builder.style(
        FOAM,
        GroupStyle {
            cast_shadow: false,
            receive_shadow: false,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    builder.style(
        QUAY,
        GroupStyle {
            cast_shadow: false,
            receive_shadow: true,
            alpha_cutout: false,
            dynamic: false,
        },
    );
    let mut line = ring_of(river, frame);
    if line.len() < 2 {
        return;
    }
    // Chaikin corner cutting: the plan gives a coarse polyline, and offsetting a
    // coarse polyline makes angular banks. Four passes round every bend into a
    // smooth curve while keeping both ends where they were.
    for _ in 0..4 {
        let mut next = Vec::with_capacity(line.len() * 2);
        next.push(line[0]);
        for pair in line.windows(2) {
            next.push(pair[0] * 0.75 + pair[1] * 0.25);
            next.push(pair[0] * 0.25 + pair[1] * 0.75);
        }
        next.push(line[line.len() - 1]);
        line = next;
    }
    let n = line.len();
    let half = width * 0.5;

    // Arc length and left normals.
    let mut arc = vec![0.0_f32; n];
    let mut normal = vec![Vec2::new(0.0, 1.0); n];
    for i in 0..n {
        if i > 0 {
            let d = line[i] - line[i - 1];
            arc[i] = arc[i - 1] + d.x.hypot(d.y);
        }
        let a = line[i.saturating_sub(1)];
        let b = line[(i + 1).min(n - 1)];
        let (dx, dz) = (b.x - a.x, b.y - a.y);
        let len = dx.hypot(dz).max(1.0e-4);
        normal[i] = Vec2::new(-dz / len, dx / len);
    }
    let at = |i: usize, off: f32| {
        Vec2::new(line[i].x + normal[i].x * off, line[i].y + normal[i].y * off)
    };

    // ---- water surface ------------------------------------------------------
    // Lateral bands, tight near the bank and wide in the middle, each with its
    // own colour so the shallows fade smoothly into the channel.
    let shallow = [0.20, 0.31, 0.29];
    let mid = [0.13, 0.26, 0.30];
    let deep = [0.08, 0.19, 0.27];
    let murk = [0.24, 0.30, 0.22];
    let dists = [
        0.0_f32, 0.3, 0.6, 1.0, 1.5, 2.1, 2.8, 3.6, 4.6, 5.8, 7.2, 9.0, 11.0, 13.5, 16.0, 19.0,
        23.0, 28.0,
    ];
    let mut offsets: Vec<f32> = Vec::new();
    for d in dists.iter().filter(|d| **d < half - 0.5) {
        offsets.push(half - d);
    }
    offsets.push(0.0);
    for d in dists.iter().rev().filter(|d| **d < half - 0.5) {
        offsets.push(-(half - d));
    }
    offsets.dedup_by(|a, b| (*a - *b).abs() < 0.05);
    offsets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let depth_colour = |off: f32, s: f32| {
        // 0 at the bank, 1 in the centre.
        let from_bank = (half - off.abs()).max(0.0);
        let t = smooth01(from_bank / (half * 0.85).max(4.0));
        let base = mix(
            shallow,
            mix(mid, deep, smooth01((t - 0.45) / 0.55)),
            smooth01(t * 1.6),
        );
        // Slowly varying turbidity along the reach: sediment-brown-green patches.
        let turb = 0.5 + 0.5 * (s * 0.013 + 1.3).sin() * (s * 0.037).sin();
        mix(base, murk, turb * 0.32 * (1.0 - t * 0.5))
    };
    let put = |builder: &mut MeshBuilder,
               mut quad: Vec<Vec2>,
               y: f32,
               mat: &str,
               colour: Option<[f32; 3]>| {
        if signed_area(&quad) < 0.0 {
            quad.reverse();
        }
        builder.ground_uv(mat, &quad, y, colour);
    };
    for i in 0..n - 1 {
        let s = (arc[i] + arc[i + 1]) * 0.5;
        for w in offsets.windows(2) {
            let colour = depth_colour((w[0] + w[1]) * 0.5, s);
            put(
                builder,
                vec![at(i, w[0]), at(i + 1, w[0]), at(i + 1, w[1]), at(i, w[1])],
                WATER_Y,
                "water",
                Some(colour),
            );
        }
    }

    // ---- bank foam ----------------------------------------------------------
    // Three translucent strips of growing width stack to a soft, fading edge.
    for (k, (w, y)) in [(0.3_f32, 0.05_f32), (0.8, 0.055), (1.6, 0.06)]
        .into_iter()
        .enumerate()
    {
        let _ = k;
        for sg in [-1.0_f32, 1.0] {
            let edge = sg * half;
            let inner = sg * (half - w);
            for i in 0..n - 1 {
                put(
                    builder,
                    vec![at(i, edge), at(i + 1, edge), at(i + 1, inner), at(i, inner)],
                    y,
                    FOAM,
                    None,
                );
            }
        }
    }

    // ---- quay walls ---------------------------------------------------------
    let near_crossing = |p: Vec2| {
        crossings
            .iter()
            .any(|(c, r)| (p.x - c.x).hypot(p.y - c.y) < *r)
    };
    let stone = [0.56, 0.54, 0.50];
    let wet = [0.16, 0.19, 0.18];
    let coping_col = [0.70, 0.68, 0.63];
    let paving = [0.50, 0.49, 0.47];
    for sg in [-1.0_f32, 1.0] {
        let side_id = if sg > 0.0 { 1 } else { 2 };
        let chunk = |s: f32| cell01(0xA11, (s / 70.0) as i32, side_id, 7);
        let urban = |i: usize| {
            let s = (arc[i] + arc[(i + 1).min(n - 1)]) * 0.5;
            let mid_pt = (line[i] + line[(i + 1).min(n - 1)]) * 0.5;
            chunk(s) > 0.24 && !near_crossing(at(i, sg * half)) && !near_crossing(mid_pt)
        };
        let stair_seg = |i: usize| {
            let s = arc[i];
            let cell = (s / 55.0) as i32;
            let centre = (cell as f32 + 0.5) * 55.0;
            cell01(0xB22, cell, side_id, 3) < 0.45 && (s - centre).abs() < 2.4
        };
        let oe = sg * half;
        let mut prev_active = false;
        let mut rail_acc = 0.0_f32;
        let mut bollard_acc = 12.0_f32;
        for i in 0..n - 1 {
            let active = urban(i);
            let a_w = at(i, oe);
            let b_w = at(i + 1, oe);
            let seg_len = arc[i + 1] - arc[i];
            if active != prev_active {
                // End cap so the slab is closed where the quay starts or stops.
                let j = i;
                let o = |off: f32| at(j, oe + sg * off);
                let cap = |builder: &mut MeshBuilder, off0: f32, off1: f32, y0: f32, y1: f32| {
                    builder.wall(QUAY, o(off0), o(off1), y0, y1, Some(stone));
                    builder.wall(QUAY, o(off1), o(off0), y0, y1, Some(stone));
                };
                cap(builder, 0.0, 4.0, -0.2, TOP);
            }
            prev_active = active;
            if !active {
                continue;
            }
            let stairs = stair_seg(i);
            let out = |p: usize, off: f32| at(p, oe + sg * off);
            // Water face: a dark wet base under the waterline, dressed stone above.
            builder.wall(QUAY, a_w, b_w, -1.0, WATER_Y, Some(wet));
            builder.wall(QUAY, b_w, a_w, -1.0, WATER_Y, Some(wet));
            builder.wall(QUAY, a_w, b_w, WATER_Y, TOP, Some(stone));
            builder.wall(QUAY, b_w, a_w, WATER_Y, TOP, Some(stone));
            let s = arc[i];
            let tone = 0.94 + 0.12 * cell01(0xC33, (s / 4.0) as i32, side_id, 1);
            let top_col = [paving[0] * tone, paving[1] * tone, paving[2] * tone];
            if stairs {
                // Landing flush with the walkway, then steps down into the water.
                put(
                    builder,
                    vec![out(i, 0.0), out(i + 1, 0.0), out(i + 1, 3.4), out(i, 3.4)],
                    TOP,
                    QUAY,
                    Some(top_col),
                );
                for k in 0..5 {
                    let d0 = 0.42 * k as f32;
                    let d1 = 0.42 * (k + 1) as f32;
                    let y = TOP - 0.19 * (k + 1) as f32;
                    put(
                        builder,
                        vec![out(i, -d0), out(i + 1, -d0), out(i + 1, -d1), out(i, -d1)],
                        y,
                        QUAY,
                        Some(coping_col),
                    );
                    let (p, q) = (out(i, -d1), out(i + 1, -d1));
                    builder.wall(QUAY, p, q, y - 0.19, y, Some(stone));
                    builder.wall(QUAY, q, p, y - 0.19, y, Some(stone));
                    // Side cheek walls, kept out of the water above the last tread.
                }
            } else {
                // Coping: a proud, lighter stone edge.
                builder.wall(
                    QUAY,
                    out(i, -0.06),
                    out(i + 1, -0.06),
                    TOP,
                    COPING,
                    Some(coping_col),
                );
                builder.wall(
                    QUAY,
                    out(i + 1, -0.06),
                    out(i, -0.06),
                    TOP,
                    COPING,
                    Some(coping_col),
                );
                put(
                    builder,
                    vec![
                        out(i, -0.06),
                        out(i + 1, -0.06),
                        out(i + 1, 0.42),
                        out(i, 0.42),
                    ],
                    COPING,
                    QUAY,
                    Some(coping_col),
                );
                builder.wall(
                    QUAY,
                    out(i, 0.42),
                    out(i + 1, 0.42),
                    TOP,
                    COPING,
                    Some(coping_col),
                );
                builder.wall(
                    QUAY,
                    out(i + 1, 0.42),
                    out(i, 0.42),
                    TOP,
                    COPING,
                    Some(coping_col),
                );
                put(
                    builder,
                    vec![out(i, 0.42), out(i + 1, 0.42), out(i + 1, 3.4), out(i, 3.4)],
                    TOP,
                    QUAY,
                    Some(top_col),
                );
            }
            // Landward edge: two risers step down to the ground.
            for (o0, y0, y1) in [(3.4_f32, 0.42_f32, TOP), (4.0, 0.0, 0.42)] {
                let (p, q) = (out(i, o0), out(i + 1, o0));
                builder.wall(QUAY, p, q, y0, y1, Some(stone));
                builder.wall(QUAY, q, p, y0, y1, Some(stone));
            }
            put(
                builder,
                vec![out(i, 3.4), out(i + 1, 3.4), out(i + 1, 4.0), out(i, 4.0)],
                0.42,
                QUAY,
                Some(top_col),
            );

            if !stairs {
                // Railing: posts every ~3 m, a top rail and a mid rail.
                let (p, q) = (out(i, 0.9), out(i + 1, 0.9));
                let rail = [0.55, 0.57, 0.60];
                for (h, r) in [(TOP + 1.02, 0.035_f32), (TOP + 0.55, 0.025)] {
                    builder.tube(
                        "rail.steel",
                        Vec3::from_plan(p, h),
                        Vec3::from_plan(q, h),
                        r,
                        r,
                        5,
                        Some(rail),
                    );
                }
                rail_acc -= seg_len;
                if rail_acc <= 0.0 {
                    builder.tube(
                        "rail.steel",
                        Vec3::from_plan(p, TOP),
                        Vec3::from_plan(p, TOP + 1.05),
                        0.045,
                        0.04,
                        5,
                        Some(rail),
                    );
                    rail_acc += 3.0;
                }
            }
            // Bollards along the water side of the walkway.
            bollard_acc -= seg_len;
            if bollard_acc <= 0.0 && !stairs {
                let p = out(i, 1.5);
                let dark = [0.08, 0.09, 0.10];
                builder.tube(
                    "rail.steel",
                    Vec3::from_plan(p, TOP),
                    Vec3::from_plan(p, TOP + 0.52),
                    0.13,
                    0.10,
                    8,
                    Some(dark),
                );
                builder.tube(
                    "rail.steel",
                    Vec3::from_plan(p, TOP + 0.52),
                    Vec3::from_plan(p, TOP + 0.6),
                    0.15,
                    0.15,
                    8,
                    Some(dark),
                );
                bollard_acc += 26.0;
            }
        }
    }
}
