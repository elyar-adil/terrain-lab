//! The signs of a Chinese street, as geometry: the shop board over every unit
//! (and a second one on the first floor for restaurants and gyms), the hanging
//! vertical sign, the barber's pole at the door, and a tower's company name in
//! separate letters on its crown.
//!
//! What goes on a sign is decided by [`crate::facades::cn_signs`]; this file
//! decides only where it stands.  The rules are the street's own: signs live on
//! the ground storey, a few on the first floor, and never higher; a tower
//! carries its owner's name as lettering with no board behind it.

use super::cn_kit::{edge_out, face};
use super::{hash_u32, level};
use crate::facades::GROUND_STOREY_M;
use crate::facades::cn_shopfront::{SIGN_Y0, SIGN_Y1};
use crate::facades::cn_signs::{self, BOARD, CROWN, Kind, POLE, VERT};
use crate::math::{Vec2, Vec3, ring_centroid};
use crate::mesh::{GroupStyle, MeshBuilder, box_at};

/// Declare the sign groups.  Done here, where they are used, so a city without
/// Chinese streets carries no empty sign groups.
fn declare(builder: &mut MeshBuilder) {
    let wall = GroupStyle {
        cast_shadow: true,
        receive_shadow: true,
        alpha_cutout: false,
        dynamic: false,
    };
    for key in [BOARD, VERT, POLE] {
        builder.style(key, wall);
    }
    builder.style(
        CROWN,
        GroupStyle {
            cast_shadow: false,
            receive_shadow: false,
            alpha_cutout: true,
            dynamic: false,
        },
    );
}

/// A vertical quad on the plan segment `a`–`b`, facing `outward`, whose texture
/// rectangle `(u0, v0, u1, v1)` reads left to right as seen from outside.
#[allow(clippy::too_many_arguments)]
fn oriented_quad(
    builder: &mut MeshBuilder,
    material: &str,
    a: Vec2,
    b: Vec2,
    y0: f32,
    y1: f32,
    outward: Vec2,
    uv: (f32, f32, f32, f32),
    tint: Option<[f32; 3]>,
) {
    // Seen from outside, right = forward x up = (outward.y, -outward.x).
    let right = Vec2::new(outward.y, -outward.x);
    let (l, r) = if (b - a).dot(right) >= 0.0 {
        (a, b)
    } else {
        (b, a)
    };
    let (u0, v0, u1, v1) = uv;
    face(
        builder,
        material,
        [
            Vec3::from_plan(l, y0),
            Vec3::from_plan(r, y0),
            Vec3::from_plan(r, y1),
            Vec3::from_plan(l, y1),
        ],
        Vec3::new(outward.x, 0.0, outward.y),
        Some([(u0, v1), (u1, v1), (u1, v0), (u0, v0)]),
        tint,
    );
}

/// A sign board: a shallow box with the design on its front face.
#[allow(clippy::too_many_arguments)]
fn board_box(
    builder: &mut MeshBuilder,
    p0: Vec2,
    p1: Vec2,
    out: Vec2,
    depth: f32,
    y0: f32,
    y1: f32,
    uv: (f32, f32, f32, f32),
    tint: [f32; 3],
) {
    let q0 = p0 + out * depth;
    let q1 = p1 + out * depth;
    oriented_quad(builder, BOARD, q0, q1, y0, y1, out, uv, Some(tint));
    let along = (p1 - p0).normalize();
    let a3 = Vec3::new(along.x, 0.0, along.y);
    let sides = [([p0, q0], a3 * -1.0), ([p1, q1], a3)];
    for ([s, e], n) in sides {
        face(
            builder,
            "trim.dark",
            [
                Vec3::from_plan(s, y0),
                Vec3::from_plan(e, y0),
                Vec3::from_plan(e, y1),
                Vec3::from_plan(s, y1),
            ],
            n,
            None,
            None,
        );
    }
    for (y, n) in [(y1, 1.0_f32), (y0, -1.0)] {
        face(
            builder,
            "trim.dark",
            [
                Vec3::from_plan(p0, y),
                Vec3::from_plan(p1, y),
                Vec3::from_plan(q1, y),
                Vec3::from_plan(q0, y),
            ],
            Vec3::new(0.0, n, 0.0),
            None,
            None,
        );
    }
}

/// A key from where a shop stands, so a unit has the same sign in every build.
fn place_key(p: Vec2) -> u32 {
    let x = (p.x * 2.0).round() as i32 as u32;
    let z = (p.y * 2.0).round() as i32 as u32;
    let mut v = x.wrapping_mul(0x9e37_79b1) ^ z.wrapping_mul(0x85eb_ca6b) ^ 0x2545_f491;
    v ^= v >> 15;
    v = v.wrapping_mul(0x2c1b_3c6d);
    v ^ (v >> 13)
}

fn hash01(key: u32, salt: i32) -> f32 {
    hash_u32(key, salt, 17)
}

/// Dress one shop unit `p0 → p1` on a wall facing `out` with its signs.
/// Returns `false` when the unit is too narrow for a legible board, in which
/// case the caller keeps the shopfront tile's own lettering.
pub(super) fn shop_signs(
    builder: &mut MeshBuilder,
    p0: Vec2,
    p1: Vec2,
    out: Vec2,
    tint: [f32; 3],
) -> bool {
    let width = p0.distance(p1);
    if width < 2.6 {
        return false;
    }
    declare(builder);
    let along = (p1 - p0) / width;
    let key = place_key((p0 + p1) * 0.5);
    let bays = if width > 6.4 { 2 } else { 1 };
    let board = cn_signs::pick_board(bays, key, false);
    let (u0, v0, u1, v1) = cn_signs::board_uv(board);
    board_box(
        builder,
        p0,
        p1,
        out,
        0.22,
        level::GROUND + SIGN_Y0,
        level::GROUND + SIGN_Y1,
        (u0, v0, u1, v1),
        tint,
    );
    // A restaurant, game room or gym often takes the first floor too.
    if hash01(key, 3) < 0.30 {
        let upper = cn_signs::pick_board(bays, key ^ 0x5bd1_e995, true);
        let (u0, v0, u1, v1) = cn_signs::board_uv(upper);
        let y0 = level::GROUND + GROUND_STOREY_M + 0.55;
        board_box(
            builder,
            p0 + along * 0.15,
            p1 - along * 0.15,
            out,
            0.16,
            y0,
            y0 + (SIGN_Y1 - SIGN_Y0),
            (u0, v0, u1, v1),
            tint,
        );
    }
    // The vertical hanging sign at the unit's left-hand pier.
    if hash01(key, 5) < 0.17 && width > 3.4 {
        let vert = cn_signs::pick_vert(key ^ 0x1b87_3593);
        let (u0, v0, u1, v1) = cn_signs::vert_uv(vert);
        let at = p0 + along * 0.32;
        let (y0, y1) = (level::GROUND + 2.75, level::GROUND + 4.55);
        let near = at + out * 0.26;
        let far = at + out * 0.96;
        for side in [along, -along] {
            // Both faces; the one turned away reads mirrored from behind, so
            // each face carries the design seen from its own side.
            oriented_quad(
                builder,
                VERT,
                near + side * 0.025,
                far + side * 0.025,
                y0,
                y1,
                side,
                (u0, v0, u1, v1),
                Some(tint),
            );
        }
        // The bracket arm and top cap.
        box_at(
            builder,
            "trim.dark",
            at + out * 0.61,
            y1 + 0.03,
            0.06,
            0.06,
            0.72,
            along.y.atan2(along.x),
        );
    }
    if board.kind == Kind::Barber {
        barber_pole(builder, p1 - along * 0.36, out, along);
    }
    true
}

/// A rotating barber's pole: a red, white and blue helix on a glass cylinder
/// on two wall brackets beside the door.  The stripes are a scrolling texture
/// (`sign/pole`), so the renderer turns the pole by moving the texture.
fn barber_pole(builder: &mut MeshBuilder, base: Vec2, out: Vec2, along: Vec2) {
    const SEGMENTS: usize = 12;
    let (radius, y0, y1) = (0.095_f32, level::GROUND + 1.42, level::GROUND + 2.38);
    let centre = base + out * 0.26;
    let circumference = std::f32::consts::TAU * radius;
    let v_top = (y1 - y0) / circumference;
    let ring = |k: usize, r: f32| {
        let t = std::f32::consts::TAU * k as f32 / SEGMENTS as f32;
        (
            centre + Vec2::new(t.cos(), t.sin()) * r,
            Vec3::new(t.cos(), 0.0, t.sin()),
        )
    };
    for k in 0..SEGMENTS {
        let (pa, na) = ring(k, radius);
        let (pb, nb) = ring(k + 1, radius);
        let (ua, ub) = (k as f32 / SEGMENTS as f32, (k + 1) as f32 / SEGMENTS as f32);
        // `pb` first: the ring runs the other way round the cylinder.
        builder.quad_uv_shaded(
            POLE,
            [
                (Vec3::from_plan(pb, y0), nb),
                (Vec3::from_plan(pa, y0), na),
                (Vec3::from_plan(pa, y1), na),
                (Vec3::from_plan(pb, y1), nb),
            ],
            [(ub, v_top), (ua, v_top), (ua, 0.0), (ub, 0.0)],
            None,
        );
    }
    // Dark metal caps, top and bottom.
    for y in [y0 - 0.04, y1 + 0.04] {
        box_at(builder, "trim.dark", centre, y, 0.25, 0.08, 0.25, 0.0);
    }
    let angle = along.y.atan2(along.x);
    for y in [y0 + 0.12, y1 - 0.12] {
        box_at(
            builder,
            "trim.dark",
            base + out * 0.13,
            y,
            0.045,
            0.045,
            0.26,
            angle,
        );
    }
}

/// A tower's company name: separate letters standing on the parapet with no
/// board behind them, on a slim steel rail.
pub(super) fn crown_lettering(
    ring: &[Vec2],
    top: f32,
    parapet_h: f32,
    id: u32,
    floors: u16,
    builder: &mut MeshBuilder,
) {
    let chance = match floors {
        0..=5 => 0.0,
        6..=11 => 0.35,
        _ => 0.9,
    };
    if hash_u32(id, 911, 3) >= chance {
        return;
    }
    declare(builder);
    let crown = cn_signs::pick_crown(id.wrapping_mul(0x9e37_79b1) ^ 0x7f4a_7c15);
    let centre = ring_centroid(ring);
    let mut edges: Vec<(f32, usize)> = (0..ring.len())
        .map(|i| (ring[i].distance(ring[(i + 1) % ring.len()]), i))
        .collect();
    edges.sort_by(|a, b| b.0.total_cmp(&a.0));
    let tall = floors >= 20;
    let mut h = if tall {
        4.2
    } else if floors >= 12 {
        3.0
    } else {
        1.8
    };
    h *= 0.9 + 0.2 * hash_u32(id, 913, 3);
    let faces = if floors >= 12 && hash_u32(id, 915, 3) < 0.45 {
        2
    } else {
        1
    };
    let mut placed = 0;
    for &(length, index) in &edges {
        if placed == faces {
            break;
        }
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let mut size = h;
        let mut width = cn_signs::crown_width_m(crown, size);
        if width > length * 0.78 {
            size = h * (length * 0.78 / width);
            width = length * 0.78;
        }
        if size < 1.05 {
            continue;
        }
        placed += 1;
        let along = (b - a) / length;
        let out = edge_out(a, b, centre);
        let mid = (a + b) * 0.5 + out * 0.02;
        let (l, r) = (mid - along * width * 0.5, mid + along * width * 0.5);
        let y0 = top + parapet_h + 0.28;
        let (u0, v0, u1, v1) = cn_signs::crown_uv(crown);
        oriented_quad(
            builder,
            CROWN,
            l,
            r,
            y0,
            y0 + size,
            out,
            (u0, v0, u1, v1),
            None,
        );
        // The steel rail the letters are bolted to.
        box_at(
            builder,
            "trim.dark",
            mid,
            top + parapet_h + 0.20,
            width,
            0.16,
            0.18,
            along.y.atan2(along.x),
        );
    }
}
