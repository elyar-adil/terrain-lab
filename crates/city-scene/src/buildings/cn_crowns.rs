//! How Chinese residential blocks end: the roof forms of each building
//! culture, and what stands on a flat roof.
//!
//! The crown is the part of a block seen from furthest away, and it is where
//! the cultures differ most: the white beam frame (构架) of the 2000s and 2010s
//! tower, the clay hip of the walk-up, the mansard with dormers of the 欧式
//! compound, the grey-tile hips with deep eaves of 新中式, and the rows of solar
//! water heaters on every 1990s walk-up roof.

use super::cn_kit::{
    Frame, PAINT, TRIM, edge_out, face, hip_roof, notched_plan, parapet, projection, roof_clay,
};
use super::hash_u32;
use crate::math::{Vec2, Vec3, inset_ring, ring_centroid};
use crate::mesh::{MeshBuilder, box_at};

/// The lift machine room over the core, in the body colour, and its frame.
fn core_of(frame: &Frame) -> Frame {
    let core_w = (frame.len * 0.32).clamp(6.0, 12.0);
    let core_d = (frame.depth * 0.40).clamp(4.5, 8.0);
    let cx = frame.len * 0.5;
    let cy = frame.depth * 0.55;
    Frame {
        o: frame.at(cx - core_w * 0.5, cy - core_d * 0.5),
        ea: frame.ea,
        eb: frame.eb,
        len: core_w,
        depth: core_d,
    }
}

fn walls(
    ring: &[Vec2],
    y0: f32,
    y1: f32,
    material: &str,
    colour: Option<[f32; 3]>,
    builder: &mut MeshBuilder,
) {
    let centre = ring_centroid(ring);
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let out = edge_out(a, b, centre);
        face(
            builder,
            material,
            [
                Vec3::from_plan(a, y0),
                Vec3::from_plan(b, y0),
                Vec3::from_plan(b, y1),
                Vec3::from_plan(a, y1),
            ],
            Vec3::new(out.x, 0.0, out.y),
            None,
            colour,
        );
    }
}

/// The machine room box over the core.  Returns its frame and roof height.
fn machine_room(
    frame: &Frame,
    top: f32,
    paint: [f32; 3],
    builder: &mut MeshBuilder,
) -> (Frame, f32) {
    let core = core_of(frame);
    let ring = notched_plan(&core, &[], &[]);
    let h = 4.2;
    walls(&ring, top, top + h, PAINT, Some(paint), builder);
    (core, top + h)
}

/// A white beam frame round a tower's roof, over a flat machine room.
pub(super) fn frame_crown(
    plan: &[Vec2],
    frame: &Frame,
    top: f32,
    id: u32,
    paint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let (core, core_top) = machine_room(frame, top, paint, builder);
    let core_ring = notched_plan(&core, &[], &[]);
    builder.ground_uv("roof", &core_ring, core_top, None);
    parapet(&core_ring, core_top, 0.6, paint, builder);
    let h = 3.6 + 1.2 * hash_u32(id, 259, 5);
    let edge = inset_ring(plan, 0.35);
    let y1 = top + 1.2 + h;
    for index in 0..edge.len() {
        let a = edge[index];
        let b = edge[(index + 1) % edge.len()];
        let length = a.distance(b);
        if length < 1.5 {
            continue;
        }
        let along = (b - a) / length;
        let angle = along.y.atan2(along.x);
        let posts = (length / 3.2).ceil().max(1.0) as u32;
        for p in 0..=posts {
            let at = a + along * (length * p as f32 / posts as f32);
            box_at(builder, TRIM, at, top + 1.2 + h * 0.5, 0.35, h, 0.35, angle);
        }
        let mid = (a + b) * 0.5;
        box_at(
            builder,
            TRIM,
            mid,
            y1 - 0.3,
            length + 0.35,
            0.6,
            0.35,
            angle,
        );
    }
    let beams = (frame.len / 3.0).floor() as u32;
    for k in 1..beams {
        let x = frame.len * k as f32 / beams as f32;
        let mid = frame.at(x, frame.depth * 0.5);
        let angle = frame.eb.y.atan2(frame.eb.x);
        box_at(
            builder,
            TRIM,
            mid,
            y1 - 0.25,
            frame.depth - 0.9,
            0.5,
            0.3,
            angle,
        );
    }
}

/// A hipped clay roof over the machine room of a tower.
pub(super) fn hipped_core(
    frame: &Frame,
    top: f32,
    terracotta: bool,
    paint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let (core, core_top) = machine_room(frame, top, paint, builder);
    hip_roof(&core, core_top, roof_clay(terracotta), 0.6, 0.5, builder);
}

/// The 欧式 crown: a mansard of dark slate-grey tile rising from the parapet,
/// pierced by dormers with white surrounds, under a flat top.
pub(super) fn mansard(
    plan: &[Vec2],
    frame: Option<&Frame>,
    top: f32,
    id: u32,
    builder: &mut MeshBuilder,
) {
    let h = 3.0 + 0.6 * hash_u32(id, 263, 5);
    let inset = 1.1;
    let upper = inset_ring(plan, inset);
    if upper.len() != plan.len() {
        return;
    }
    let roof = if hash_u32(id, 265, 5) < 0.5 {
        "roof.steel"
    } else {
        "roof.tile"
    };
    let centre = ring_centroid(plan);
    for index in 0..plan.len() {
        let a = plan[index];
        let b = plan[(index + 1) % plan.len()];
        let c = upper[(index + 1) % upper.len()];
        let d = upper[index];
        let out = edge_out(a, b, centre);
        let length = a.distance(b);
        let slope = (h * h + inset * inset).sqrt();
        face(
            builder,
            roof,
            [
                Vec3::from_plan(a, top),
                Vec3::from_plan(b, top),
                Vec3::from_plan(c, top + h),
                Vec3::from_plan(d, top + h),
            ],
            Vec3::new(out.x, inset / slope * 3.0, out.y),
            Some([(0.0, slope), (length, slope), (length, 0.0), (0.0, 0.0)]),
            None,
        );
        // Dormers: a white-framed box with a dark window, every 3.6 m.
        if length < 4.0 {
            continue;
        }
        let along = (b - a) / length;
        let count = ((length - 1.2) / 3.6).floor().max(1.0) as u32;
        let pitch = length / count as f32;
        for k in 0..count {
            let at = a + along * (pitch * (k as f32 + 0.5)) - out * 0.3;
            let p0 = at - along * 0.6;
            let p1 = at + along * 0.6;
            projection(
                builder,
                TRIM,
                None,
                p0,
                p1,
                out,
                0.6,
                top + 0.5,
                top + 2.2,
                None,
            );
            box_at(
                builder,
                "trim.dark",
                at + out * 0.62,
                top + 1.3,
                0.8,
                1.1,
                0.04,
                along.y.atan2(along.x),
            );
            // The dormer's own little pitched cap.
            box_at(
                builder,
                roof,
                at + out * 0.25,
                top + 2.3,
                1.5,
                0.2,
                1.0,
                along.y.atan2(along.x),
            );
        }
    }
    builder.ground_uv("roof", &upper, top + h, None);
    // The cornice under the mansard.
    for index in 0..plan.len() {
        let a = plan[index];
        let b = plan[(index + 1) % plan.len()];
        let out = edge_out(a, b, centre);
        projection(
            builder,
            TRIM,
            None,
            a,
            b,
            out,
            0.35,
            top - 0.35,
            top + 0.05,
            None,
        );
    }
    if let Some(frame) = frame {
        let core = core_of(frame);
        let core_ring = notched_plan(&core, &[], &[]);
        let ok = core_ring
            .iter()
            .all(|p| crate::math::point_in_ring(*p, &upper));
        if ok {
            hip_roof(&core, top + h, roof, 0.4, 0.45, builder);
        }
    }
}

/// 新中式: a flat roof under a deep dark eave board, and a grey-tile hipped
/// pavilion roof with wide eaves over the core.
pub(super) fn new_chinese(
    plan: &[Vec2],
    frame: Option<&Frame>,
    top: f32,
    paint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let centre = ring_centroid(plan);
    for index in 0..plan.len() {
        let a = plan[index];
        let b = plan[(index + 1) % plan.len()];
        let out = edge_out(a, b, centre);
        // The eave board: 600 mm out, dark grey, with its soffit.
        projection(
            builder,
            "trim.dark",
            None,
            a,
            b,
            out,
            0.6,
            top + 0.6,
            top + 0.95,
            None,
        );
    }
    if let Some(frame) = frame {
        let (core, core_top) = machine_room(frame, top, paint, builder);
        // A wider, flatter grey-tile roof than the clay hip.
        let wide = Frame {
            o: core.o - core.ea * 0.9 - core.eb * 0.9,
            ea: core.ea,
            eb: core.eb,
            len: core.len + 1.8,
            depth: core.depth + 1.8,
        };
        hip_roof(&wide, core_top, "roof.tile", 1.1, 0.42, builder);
    }
}

/// Solar water heaters (太阳能热水器) on a flat walk-up roof: one per flat,
/// in rows facing the sun, each a tilted rack of evacuated tubes under a white
/// tank on a steel frame.
pub(super) fn solar_heaters(
    frame: &Frame,
    top: f32,
    id: u32,
    sun: Vec2,
    builder: &mut MeshBuilder,
) {
    // Face the panels to the sun: along the frame axis that points at it.
    let toward = if frame.eb.dot(sun) < 0.0 {
        -frame.eb
    } else {
        frame.eb
    };
    let along = frame.ea;
    let angle = along.y.atan2(along.x);
    let count = (frame.len / 2.6).floor() as i32;
    let rows = if frame.depth > 11.0 { 2 } else { 1 };
    for r in 0..rows {
        let y = frame.depth * (0.3 + 0.4 * r as f32);
        for k in 0..count {
            if hash_u32(id, k * 7 + r, 269) < 0.25 {
                continue;
            }
            let x = 1.3 + k as f32 * 2.6;
            let at = frame.at(x, y);
            // The tank, white, 1.8 m long, 0.45 m across.
            box_at(
                builder,
                "metal.ac",
                at - toward * 0.4,
                top + 1.15,
                1.8,
                0.45,
                0.45,
                angle,
            );
            // The tube rack, sloping down towards the sun.
            let p0 = at - toward * 0.25 - along * 0.85;
            let p1 = at - toward * 0.25 + along * 0.85;
            let q0 = p0 + toward * 1.3;
            let q1 = p1 + toward * 1.3;
            face(
                builder,
                "solar.tubes",
                [
                    Vec3::from_plan(p0, top + 1.0),
                    Vec3::from_plan(p1, top + 1.0),
                    Vec3::from_plan(q1, top + 0.25),
                    Vec3::from_plan(q0, top + 0.25),
                ],
                Vec3::new(toward.x, 1.6, toward.y),
                None,
                None,
            );
            // Legs.
            for p in [q0, q1] {
                box_at(builder, "trim.dark", p, top + 0.12, 0.05, 0.25, 0.05, angle);
            }
        }
    }
}
