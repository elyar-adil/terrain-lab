//! Factories and warehouses: the portal-frame shed and the standard-factory block.
//!
//! A Chinese industrial park is two kinds of building, and they are easy to tell
//! apart from the air:
//!
//! - the **portal-frame shed** (门式刚架厂房): a single storey with eaves at
//!   8 - 12 m, a shallow steel roof at about 1 : 10 with translucent skylight
//!   strips and a ridge vent, walls of a 1.2 m block dado under corrugated
//!   cladding, a ribbon of windows under the eaves, and roller doors in the
//!   gable ends with a canopy over each;
//! - the **standard factory** (标准厂房): four or five storeys of concrete frame
//!   with a band of windows on every floor and a flat roof.
//!
//! Every choice is made from the building's own hash, so two sheds in one yard
//! differ in cladding colour, door count and skylight spacing.

use urban::{ModernBuilding, ParcelUse};

use super::{hash_u32, level};
use crate::math::{Vec2, Vec3, signed_area};
use crate::mesh::{MeshBuilder, box_at, face_normal};

/// Height of the block-work dado under the cladding.
const DADO_M: f32 = 1.2;
/// Roof pitch of a portal frame, rise over run.
const SHED_PITCH: f32 = 0.1;
/// How far the roof sheet runs past the wall.
const EAVE_OVERHANG_M: f32 = 0.45;

/// Cladding colours (linear tints on a near-white sheet): white, pale grey,
/// the light blue-grey and the cream that most sheds are clad in.
const CLADDING: [[f32; 3]; 5] = [
    [0.86, 0.87, 0.87],
    [0.74, 0.76, 0.78],
    [0.62, 0.70, 0.78],
    [0.84, 0.80, 0.70],
    [0.70, 0.72, 0.70],
];

/// Whether this module builds a building.
pub(crate) fn applies(building: &ModernBuilding) -> bool {
    building.use_type == ParcelUse::Industrial
}

fn roll(building: &ModernBuilding, salt: i32) -> f32 {
    hash_u32(building.id, salt, 0x1d57)
}

/// Build one industrial building on its footprint.
pub(crate) fn shell(building: &ModernBuilding, ring: &[Vec2], builder: &mut MeshBuilder) {
    if ring.len() != 4 {
        return;
    }
    // Clockwise, so every wall faces out (see `building_shell`).
    let mut outward = ring.to_vec();
    if signed_area(&outward) > 0.0 {
        outward.reverse();
    }
    if building.floors <= 1 {
        shed(building, &outward, builder);
    } else {
        block(building, &outward, builder);
    }
}

/// A quad whose face is turned toward `out`, whatever order its corners came in.
fn quad_facing(
    builder: &mut MeshBuilder,
    material: &str,
    q: [Vec3; 4],
    out: Vec3,
    color: Option<[f32; 3]>,
) {
    if face_normal(q[0], q[1], q[2]).dot(out) >= 0.0 {
        builder.quad(material, q[0], q[1], q[2], q[3], color);
    } else {
        builder.quad(material, q[3], q[2], q[1], q[0], color);
    }
}

fn shed(building: &ModernBuilding, ring: &[Vec2], builder: &mut MeshBuilder) {
    let tint = CLADDING[(roll(building, 1) * CLADDING.len() as f32) as usize % CLADDING.len()];
    let eave = level::GROUND + building.height_metres;
    // The ridge runs along the long axis.
    let (e0, e1) = (ring[1] - ring[0], ring[2] - ring[1]);
    let long_first = e0.length() >= e1.length();
    // `a0..a1` and `b0..b1` are the two long walls, eave side; the short walls are gables.
    let (a0, a1, b1, b0) = if long_first {
        (ring[0], ring[1], ring[2], ring[3])
    } else {
        (ring[1], ring[2], ring[3], ring[0])
    };
    let span = a0.distance(b0);
    let length = a0.distance(a1);
    let ridge = eave + span * 0.5 * SHED_PITCH;
    let centre = (a0 + a1 + b0 + b1) * 0.25;

    // Walls: the dado, the cladding, and on the long walls the ribbon window.
    for index in 0..4 {
        let (p, q) = (ring[index], ring[(index + 1) % 4]);
        builder.wall("trim.dark", p, q, level::PLINTH, level::GROUND, None);
        builder.wall(
            "wall.render",
            p,
            q,
            level::GROUND,
            level::GROUND + DADO_M,
            Some([0.78, 0.77, 0.74]),
        );
        builder.wall_uv(
            "metal.clad",
            p,
            q,
            level::GROUND + DADO_M,
            eave,
            false,
            Some(tint),
        );
        let along = (q - p).length();
        let long_wall = (along - length).abs() < 0.5;
        if long_wall && along > 12.0 {
            // Set a hair proud of the cladding so it never fights it.
            let out = (q - p).normalize().left_normal() * 0.04;
            let (pw, qw) = (p.lerp(q, 0.04) + out, p.lerp(q, 0.96) + out);
            builder.wall("cnres.glass", pw, qw, eave - 2.3, eave - 1.2, None);
        }
    }

    // The gables, up to the ridge, and the roller doors in them.
    for (p, q) in [(a1, b1), (b0, a0)] {
        let mid = p.lerp(q, 0.5);
        let out = Vec3::from_plan((mid - centre).normalize(), 0.0);
        let tri = [
            Vec3::from_plan(p, eave),
            Vec3::from_plan(q, eave),
            Vec3::from_plan(mid, ridge),
        ];
        if face_normal(tri[0], tri[1], tri[2]).dot(out) >= 0.0 {
            builder.tri_flat("metal.clad", tri[0], tri[1], tri[2], Some(tint));
        } else {
            builder.tri_flat("metal.clad", tri[2], tri[1], tri[0], Some(tint));
        }
        let width = p.distance(q);
        let doors = if width > 36.0 { 2 } else { 1 };
        let dir = (q - p).normalize();
        let normal = (mid - centre).normalize();
        for k in 0..doors {
            let t = (k as f32 + 1.0) / (doors as f32 + 1.0);
            let c = p.lerp(q, t) + normal * 0.05;
            let (d0, d1) = (c - dir * 2.1, c + dir * 2.1);
            builder.wall(
                "metal.door",
                d0,
                d1,
                level::GROUND,
                level::GROUND + 4.5,
                None,
            );
            builder.wall(
                "metal.door",
                d1,
                d0,
                level::GROUND,
                level::GROUND + 4.5,
                None,
            );
            let yaw = dir.y.atan2(dir.x);
            box_at(
                builder,
                "trim.dark",
                c + normal * 0.6,
                level::GROUND + 4.9,
                5.4,
                0.12,
                1.3,
                yaw,
            );
        }
    }

    // The roof: two sheets from the eaves, a little past the walls, to the ridge.
    let across = (b0 - a0).normalize();
    let lengthwise = (a1 - a0).normalize();
    let over = |p: Vec2, side: f32, end: f32| {
        p - across * (EAVE_OVERHANG_M * side) + lengthwise * (EAVE_OVERHANG_M * end)
    };
    let drop = EAVE_OVERHANG_M * SHED_PITCH;
    let (ma0, ma1) = (a0.lerp(b0, 0.5), a1.lerp(b1, 0.5));
    let up = Vec3::new(0.0, 1.0, 0.0);
    let sheet_a = [
        Vec3::from_plan(over(a0, 1.0, -1.0), eave - drop),
        Vec3::from_plan(over(a1, 1.0, 1.0), eave - drop),
        Vec3::from_plan(ma1 + lengthwise * EAVE_OVERHANG_M, ridge),
        Vec3::from_plan(ma0 - lengthwise * EAVE_OVERHANG_M, ridge),
    ];
    let sheet_b = [
        Vec3::from_plan(over(b0, -1.0, -1.0), eave - drop),
        Vec3::from_plan(over(b1, -1.0, 1.0), eave - drop),
        Vec3::from_plan(ma1 + lengthwise * EAVE_OVERHANG_M, ridge),
        Vec3::from_plan(ma0 - lengthwise * EAVE_OVERHANG_M, ridge),
    ];
    quad_facing(builder, "roof.steel", sheet_a, up, None);
    quad_facing(builder, "roof.steel", sheet_b, up, None);

    // Skylight strips across each sheet, every few bays, and the ridge vent.
    let bay = 6.0 + 3.0 * roll(building, 3);
    let strips = ((length / bay) as usize).max(1);
    for k in 0..strips {
        let t0 = (k as f32 + 0.35) / strips as f32;
        let t1 = (k as f32 + 0.65) / strips as f32;
        if roll(building, 40 + k as i32) < 0.35 {
            continue;
        }
        for (eave_line, s) in [((a0, a1), 1.0_f32), ((b0, b1), -1.0)] {
            let (e0p, e1p) = eave_line;
            let lift = 0.03;
            let p0 = e0p.lerp(e1p, t0) + across * (1.2 * s);
            let p1 = e0p.lerp(e1p, t1) + across * (1.2 * s);
            let r0 = ma0.lerp(ma1, t0) - across * (1.0 * s);
            let r1 = ma0.lerp(ma1, t1) - across * (1.0 * s);
            // On the sheet: the eave height plus the pitch times the run from its eave.
            let height = |p: Vec2| {
                let run = if s > 0.0 {
                    (p - a0).dot(across)
                } else {
                    (b0 - p).dot(across)
                };
                eave + run.max(0.0) * SHED_PITCH + lift
            };
            let q = [
                Vec3::from_plan(p0, height(p0)),
                Vec3::from_plan(p1, height(p1)),
                Vec3::from_plan(r1, height(r1)),
                Vec3::from_plan(r0, height(r0)),
            ];
            quad_facing(builder, "roof.skylight", q, up, None);
        }
    }
    if length > 24.0 && roll(building, 5) < 0.7 {
        let yaw = lengthwise.y.atan2(lengthwise.x);
        box_at(
            builder,
            "metal.clad",
            (ma0 + ma1) * 0.5,
            ridge + 0.35,
            length * 0.8,
            0.7,
            1.6,
            yaw,
        );
    }
}

fn block(building: &ModernBuilding, ring: &[Vec2], builder: &mut MeshBuilder) {
    let floors = building.floors.max(2);
    let storey = (building.height_metres - 1.0) / f32::from(floors);
    let top = level::GROUND + storey * f32::from(floors);
    let panel = [
        0.80 - 0.08 * roll(building, 7),
        0.79 - 0.08 * roll(building, 7),
        0.76 - 0.06 * roll(building, 8),
    ];
    for index in 0..4 {
        let (p, q) = (ring[index], ring[(index + 1) % 4]);
        builder.wall("trim.dark", p, q, level::PLINTH, level::GROUND, None);
        builder.wall("wall.render", p, q, level::GROUND, top, Some(panel));
        let out = (q - p).normalize().left_normal() * 0.04;
        if p.distance(q) > 8.0 {
            // A band of windows on every floor, between the spandrel panels.
            let (pw, qw) = (p.lerp(q, 0.03) + out, p.lerp(q, 0.97) + out);
            for f in 0..floors {
                let y = level::GROUND + storey * f32::from(f);
                builder.wall("cnres.glass", pw, qw, y + 1.0, y + storey - 0.7, None);
            }
        }
    }
    let mut cap = ring.to_vec();
    if signed_area(&cap) < 0.0 {
        cap.reverse();
    }
    builder.ground_uv("roof", &cap, top, None);
    super::shell::parapet(&cap, top, 0.9, builder);
    // A goods lift and a stair on the roof.
    let c = (ring[0] + ring[1] + ring[2] + ring[3]) * 0.25;
    let dir = (ring[1] - ring[0]).normalize();
    box_at(
        builder,
        "wall.render",
        c,
        top + 1.6,
        6.0,
        3.2,
        5.0,
        dir.y.atan2(dir.x),
    );
}
