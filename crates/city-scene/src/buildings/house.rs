//! Houses: the detached villa of the suburb and the farmhouse of the country.
//!
//! A house here is a few rectangular volumes (a main block, perhaps a wing, a
//! garage, a shed), each built from the same facade tiles as the rest of the
//! city but with what a flat-roofed block of flats does not have: a pitched roof
//! with eaves, a ridge and gable ends, a door with a canopy, a chimney, the solar
//! water heater every Chinese roof carries.
//!
//! Every choice (the wall, the covering, the pitch, whether there is a chimney)
//! is made from the *parcel's* hash, so the volumes of one house agree with each
//! other and two houses do not.

use urban::{ModernBuilding, ParcelUse, RoofStyle};

use super::{facade_wall, hash_u32, level};
use crate::facades::{RoofCovering, design};
use crate::math::{Vec2, Vec3, signed_area};
use crate::mesh::MeshBuilder;

/// Wall tiles a house may wear: render in the warm family, cream, brick.
const VILLA_WALLS: [usize; 9] = [0, 1, 2, 4, 5, 6, 18, 17, 22];
const FARM_WALLS: [usize; 7] = [1, 4, 5, 6, 22, 23, 18];

fn roll(building: &ModernBuilding, salt: u32) -> f32 {
    hash_u32(building.parcel_id, salt as i32, 0x4801)
}

/// The covering of this house's main roof; sheds may differ.
fn covering(building: &ModernBuilding, shed: bool) -> RoofCovering {
    let r = roll(building, 11);
    match building.use_type {
        ParcelUse::Villa => {
            if r < 0.50 {
                RoofCovering::GreyClay
            } else if r < 0.88 {
                RoofCovering::Terracotta
            } else {
                RoofCovering::BlueSteel
            }
        }
        _ => {
            if shed && r < 0.75 {
                RoofCovering::BlueSteel
            } else if r < 0.62 {
                RoofCovering::GreyClay
            } else if r < 0.82 {
                RoofCovering::Terracotta
            } else {
                RoofCovering::BlueSteel
            }
        }
    }
}

struct Rect {
    o: Vec2,
    eu: Vec2,
    ev: Vec2,
    w: f32,
    d: f32,
}

impl Rect {
    fn at(&self, u: f32, v: f32) -> Vec2 {
        self.o + self.eu * u + self.ev * v
    }
    fn p(&self, u: f32, v: f32, y: f32) -> Vec3 {
        Vec3::from_plan(self.at(u, v), y)
    }
}

/// A triangle whose normal faces away from `inside`, whatever order it is given.
fn tri_away(builder: &mut MeshBuilder, material: &str, a: Vec3, b: Vec3, c: Vec3, inside: Vec3, uv: [(f32, f32); 3]) {
    let n = (b - a).cross(c - a);
    let centre = (a + b + c) * (1.0 / 3.0);
    if n.dot(centre - inside) < 0.0 {
        builder.tri_uv(material, a, c, b, [uv[0], uv[2], uv[1]], None);
    } else {
        builder.tri_uv(material, a, b, c, uv, None);
    }
}

/// A quadrilateral roof or wall piece facing away from `inside`.
fn quad_away(builder: &mut MeshBuilder, material: &str, q: [Vec3; 4], inside: Vec3, uv: [(f32, f32); 4]) {
    let n = (q[1] - q[0]).cross(q[2] - q[0]);
    let centre = (q[0] + q[1] + q[2] + q[3]) * 0.25;
    if n.dot(centre - inside) < 0.0 {
        builder.quad_uv(material, q[0], q[3], q[2], q[1], [uv[0], uv[3], uv[2], uv[1]], None);
    } else {
        builder.quad_uv(material, q[0], q[1], q[2], q[3], uv, None);
    }
}

/// Build one volume of a house.
pub(crate) fn house_volume(building: &ModernBuilding, ring: &[Vec2], builder: &mut MeshBuilder) {
    if ring.len() != 4 {
        return;
    }
    let tiles: &[usize] = if building.use_type == ParcelUse::Villa { &VILLA_WALLS } else { &FARM_WALLS };
    let tile = tiles[(roll(building, 3) * tiles.len() as f32) as usize % tiles.len()];
    let storey = design(tile).storey_m;
    let floors = building.floors.max(1) as f32;
    let top = level::GROUND + floors * storey;
    let wall_material = format!("facade/{tile:02}");

    let mut outward = ring.to_vec();
    if signed_area(&outward) > 0.0 {
        outward.reverse();
    }
    // The skirt, so the house meets sloping ground instead of hovering over it.
    for index in 0..4 {
        let a = outward[index];
        let b = outward[(index + 1) % 4];
        builder.wall("trim.dark", a, b, level::PLINTH, level::GROUND, None);
        facade_wall(builder, &wall_material, a, b, level::GROUND, top, floors, false, None);
    }

    let e = ring[1] - ring[0];
    let f = ring[3] - ring[0];
    let (w, d) = (e.length(), f.length());
    if w < 1.0 || d < 1.0 {
        return;
    }
    let r = Rect { o: ring[0], eu: e * (1.0 / w), ev: f * (1.0 / d), w, d };
    let inside = r.p(w * 0.5, d * 0.5, top - 1.0);

    door(&r, building, design(tile).bay_m, builder);

    match building.roof {
        RoofStyle::Gable | RoofStyle::Hip | RoofStyle::Terracotta | RoofStyle::Mansard => {
            pitched(&r, building, top, &wall_material, inside, builder);
        }
        _ => flat(&r, building, top, ring, builder),
    }
}

/// A door on the front wall (the first edge) and a canopy over it.
fn door(r: &Rect, building: &ModernBuilding, bay_m: f32, builder: &mut MeshBuilder) {
    if r.w < 4.0 {
        return;
    }
    // In the middle of a bay, so the door stands where a window of the grid would.
    let bays = (r.w / bay_m).round().max(1.0);
    let k = ((roll(building, 5) * bays).floor()).min(bays - 1.0);
    let u = (k + 0.5) * r.w / bays;
    let y = level::GROUND;
    // The front is the edge `v = 0`; its outward direction is `-ev`.
    let out = r.ev * -1.0;
    let (a, b) = (r.at(u - 0.55, 0.0) + out * 0.03, r.at(u + 0.55, 0.0) + out * 0.03);
    // `wall` faces left of travel; walking from the `+u` end to the `-u` end puts
    // the exterior on the left of a front edge whose outward is `-ev`.
    builder.wall("trim.dark", b, a, y, y + 2.1, None);
    builder.wall("trim.dark", a, b, y, y + 2.1, None);
    // A shallow canopy and a step.
    let centre = r.at(u, 0.0) + out * 0.7;
    let yaw = r.eu.y.atan2(r.eu.x);
    crate::mesh::box_at(builder, "trim.light", centre, y + 2.45, 2.2, 0.12, 1.4, yaw);
    crate::mesh::box_at(builder, "trim.light", r.at(u, 0.0) + out * 0.5, y + 0.06, 1.8, 0.12, 1.0, yaw);
}

fn flat(r: &Rect, building: &ModernBuilding, top: f32, ring: &[Vec2], builder: &mut MeshBuilder) {
    let mut cap = ring.to_vec();
    if signed_area(&cap) < 0.0 {
        cap.reverse();
    }
    builder.ground_uv("roof", &cap, top, None);
    super::shell::parapet(&cap, top, 0.55, builder);
    // A stairhead and a water heater: a flat roof is somebody's terrace.
    if r.w > 6.0 && r.d > 6.0 && roll(building, 7) < 0.6 {
        let c = r.at(r.w * (0.25 + 0.5 * roll(building, 8)), r.d * 0.7);
        let yaw = r.eu.y.atan2(r.eu.x);
        crate::mesh::box_at(builder, "trim.light", c, top + 1.3, 2.6, 2.6, 2.4, yaw);
    }
    solar(r, building, r.d * 0.30, r.d * 0.30 + 1.2, top + 0.1, top + 0.1, builder);
}

/// The solar water heater on a roof: a dark panel lying on the surface from
/// `(v0, y0)` to `(v1, y1)` (the plane's profile across the ridge) and its tank
/// above the high edge.
fn solar(r: &Rect, building: &ModernBuilding, v0: f32, v1: f32, y0: f32, y1: f32, builder: &mut MeshBuilder) {
    if roll(building, 9) > 0.5 || r.w < 7.0 {
        return;
    }
    let u = r.w * (0.20 + 0.40 * roll(building, 10));
    let lift = 0.12;
    let q = [
        r.p(u, v0, y0 + lift),
        r.p(u + 1.7, v0, y0 + lift),
        r.p(u + 1.7, v1, y1 + lift),
        r.p(u, v1, y1 + lift),
    ];
    quad_away(
        builder,
        "trim.dark",
        q,
        r.p(u + 0.85, 0.5 * (v0 + v1), 0.5 * (y0 + y1) - 2.0),
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
    );
    // The tank, lying along the panel's top edge.
    let tank = Vec3::new(0.0, 0.0, 0.0);
    let _ = tank;
    let (tv, ty) = (v1 + 0.02, y1 + lift + 0.20);
    builder.tube("trim.light", r.p(u + 0.1, tv, ty), r.p(u + 1.6, tv, ty), 0.17, 0.17, 8, None);
}

fn pitched(r: &Rect, building: &ModernBuilding, top: f32, wall_material: &str, inside: Vec3, builder: &mut MeshBuilder) {
    let hip = building.roof == RoofStyle::Hip || building.roof == RoofStyle::Terracotta;
    let shed = building.floors <= 1 && building.use_type == ParcelUse::Farmstead && r.w * r.d < 60.0;
    let cover = covering(building, shed);
    let material = cover.key();
    let along_u = r.w >= r.d;
    // The ridge runs along the long side; `a` is the long axis, `b` the short.
    let (len, span) = if along_u { (r.w, r.d) } else { (r.d, r.w) };
    let pitch = match cover {
        RoofCovering::BlueSteel => 0.27,
        _ => 0.46 + 0.14 * roll(building, 13),
    };
    let eave = 0.45 + 0.2 * roll(building, 14);
    let gable_oh = 0.30;
    let half = span * 0.5 + eave;
    let rise = half * pitch;
    let y_eave = top - 0.18;
    let y_ridge = y_eave + rise;
    // Map (a, b) in the ridge frame back to the rectangle's (u, v).
    let map = |a: f32, b: f32| if along_u { (a, b) } else { (b, a) };
    let pt = |a: f32, b: f32, y: f32| {
        let (u, v) = map(a, b);
        r.p(u, v, y)
    };
    let mid = span * 0.5;
    let (r0, r1) = if hip && len - span > 0.4 {
        (half - eave, len - (half - eave))
    } else if hip {
        (len * 0.5, len * 0.5)
    } else {
        (-gable_oh, len + gable_oh)
    };
    let slope_len = (half * half + rise * rise).sqrt();
    // The two long slopes (or four, for a hip).
    for side in [-1.0_f32, 1.0] {
        let b_eave = mid + side * half;
        let a_eave0 = if hip { -eave } else { -gable_oh };
        let a_eave1 = if hip { len + eave } else { len + gable_oh };
        let q = [pt(a_eave0, b_eave, y_eave), pt(a_eave1, b_eave, y_eave), pt(r1, mid, y_ridge), pt(r0, mid, y_ridge)];
        quad_away(
            builder,
            material,
            q,
            inside,
            [(a_eave0, slope_len), (a_eave1, slope_len), (r1, 0.0), (r0, 0.0)],
        );
        // The soffit under the eave, so the overhang is a thing and not a fold.
        let wall_b = mid + side * (span * 0.5);
        let soffit = [pt(a_eave0, b_eave, y_eave - 0.02), pt(a_eave1, b_eave, y_eave - 0.02), pt(a_eave1, wall_b, top - 0.02), pt(a_eave0, wall_b, top - 0.02)];
        let below = r.p(r.w * 0.5, r.d * 0.5, top + 4.0);
        quad_away(builder, "trim.light", soffit, below, [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        // A fascia board on the eave edge.
        let fascia = [pt(a_eave0, b_eave, y_eave - 0.22), pt(a_eave1, b_eave, y_eave - 0.22), pt(a_eave1, b_eave, y_eave), pt(a_eave0, b_eave, y_eave)];
        quad_away(builder, "trim.light", fascia, r.p(r.w * 0.5, r.d * 0.5, y_eave - 0.1), [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    }
    for end in [0.0_f32, 1.0] {
        let a_wall = end * len;
        let sgn = if end == 0.0 { -1.0 } else { 1.0 };
        if hip {
            // A hipped end: a triangle or trapezoid up to the ridge end.
            let a_eave = a_wall + sgn * eave;
            let a_ridge = if end == 0.0 { r0 } else { r1 };
            let q = [
                pt(a_eave, mid - half, y_eave),
                pt(a_eave, mid + half, y_eave),
                pt(a_ridge, mid, y_ridge),
                pt(a_ridge, mid, y_ridge),
            ];
            tri_away(
                builder,
                material,
                q[0],
                q[1],
                q[2],
                inside,
                [(mid - half, slope_len), (mid + half, slope_len), (mid, 0.0)],
            );
        } else {
            // A gable end: a triangle of wall under the roof, with its own eave edge.
            let a_at = a_wall;
            let t = [pt(a_at, mid - span * 0.5, top), pt(a_at, mid + span * 0.5, top), pt(a_at, mid, y_ridge)];
            // The gable wall shows the house's own render, plain.
            tri_away(builder, "trim.light", t[0], t[1], t[2], inside, [(0.0, 0.0), (span, 0.0), (span * 0.5, 1.0)]);
            let _ = wall_material;
            // Barge boards along the rake.
            for side in [-1.0_f32, 1.0] {
                let from = pt(a_at + sgn * gable_oh, mid + side * half, y_eave);
                let to = pt(a_at + sgn * gable_oh, mid, y_ridge);
                builder.tube("trim.light", from, to, 0.05, 0.05, 4, None);
            }
        }
    }
    // The ridge, and the hips: a dark tile line catches the light.
    builder.tube("trim.dark", pt(r0, mid, y_ridge + 0.03), pt(r1, mid, y_ridge + 0.03), 0.09, 0.09, 6, None);
    if hip {
        for end in [0.0_f32, 1.0] {
            let a_eave = if end == 0.0 { -eave } else { len + eave };
            let a_ridge = if end == 0.0 { r0 } else { r1 };
            for side in [-1.0_f32, 1.0] {
                builder.tube(
                    "trim.dark",
                    pt(a_eave, mid + side * half, y_eave + 0.02),
                    pt(a_ridge, mid, y_ridge + 0.03),
                    0.07,
                    0.07,
                    6,
                    None,
                );
            }
        }
    }
    // A chimney on a gabled house, and a heater on the sun-facing slope.
    if building.use_type == ParcelUse::Villa && roll(building, 15) < 0.45 && len > 6.0 {
        let a = len * (0.2 + 0.2 * roll(building, 16));
        let (u, v) = map(a, mid);
        let c = r.at(u, v);
        let yaw = r.eu.y.atan2(r.eu.x);
        crate::mesh::box_at(builder, "trim.light", c, y_ridge + 0.45, 0.7, 1.3, 0.7, yaw);
    }
    // The heater lies on the front slope (the one whose eave is at `v = -eave`).
    if along_u {
        let (b0, b1) = (mid - half * 0.70, mid - half * 0.26);
        let y_at = |bb: f32| y_ridge - (mid - bb) * pitch;
        solar(r, building, b0, b1, y_at(b0), y_at(b1), builder);
    }
}
