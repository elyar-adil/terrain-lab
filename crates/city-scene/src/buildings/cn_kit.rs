//! Geometry shared by the Chinese building builders (`cn_residential`,
//! `cn_commercial`): the plan frame and its notches, faces and projecting
//! boxes, the wall-to-elevation mapping, parapets, pitched roofs and the shop
//! storey.

use urban::ModernBuilding;

use super::{hash_u32, level};
use crate::facades::GROUND_STOREY_M;
use crate::facades::cn_grammar::FACADE;
use crate::math::{Vec2, Vec3, ring_centroid, signed_area};
use crate::mesh::{MeshBuilder, box_at};

/// The sun's azimuth in plan, matching `SUN_DIR` in `src/city/sky.ts`.  The
/// living side of a block faces it.
pub(super) const SUN_PLAN: Vec2 = Vec2 {
    x: -0.694,
    y: -0.718,
};

/// Body paint and its trim.
pub(crate) const PAINT: &str = "cnres.paint";
pub(crate) const TRIM: &str = "cnres.trim";

/// The plan's frame: an origin, the long axis `ea`, the axis `eb` pointing from
/// the living side to the service side, and the extents along each.
#[derive(Debug, Clone, Copy)]
pub(super) struct Frame {
    pub(super) o: Vec2,
    pub(super) ea: Vec2,
    pub(super) eb: Vec2,
    pub(super) len: f32,
    pub(super) depth: f32,
}

impl Frame {
    pub(super) fn at(&self, x: f32, y: f32) -> Vec2 {
        self.o + self.ea * x + self.eb * y
    }

    /// Fit a frame to a ring that is (nearly) a rectangle.
    pub(super) fn fit(ring: &[Vec2]) -> Option<Self> {
        if ring.len() < 4 {
            return None;
        }
        let (mut best, mut best_len) = (0, 0.0_f32);
        for index in 0..ring.len() {
            let l = ring[index].distance(ring[(index + 1) % ring.len()]);
            if l > best_len {
                best = index;
                best_len = l;
            }
        }
        let ea = (ring[(best + 1) % ring.len()] - ring[best]).normalize();
        let mut eb = Vec2::new(-ea.y, ea.x);
        // The living side is y = 0, its outward normal is -eb: point it at the sun.
        if (-eb).dot(SUN_PLAN) < 0.0 {
            eb = -eb;
        }
        let (mut x0, mut x1, mut y0, mut y1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for p in ring {
            let x = p.dot(ea);
            let y = p.dot(eb);
            x0 = x0.min(x);
            x1 = x1.max(x);
            y0 = y0.min(y);
            y1 = y1.max(y);
        }
        let (len, depth) = (x1 - x0, y1 - y0);
        if len < 4.0 || depth < 4.0 {
            return None;
        }
        // A rectangle fills its own frame; anything else keeps its ring.
        if signed_area(ring).abs() < 0.93 * len * depth {
            return None;
        }
        Some(Self {
            o: ea * x0 + eb * y0,
            ea,
            eb,
            len,
            depth,
        })
    }
}

/// A notch cut into one long side: start and end along the long axis, depth.
#[derive(Debug, Clone, Copy)]
pub(super) struct Notch {
    pub(super) x0: f32,
    pub(super) x1: f32,
    pub(super) depth: f32,
}

/// The plan, clockwise (outward walls), from a frame and its notches.
pub(super) fn notched_plan(frame: &Frame, front: &[Notch], back: &[Notch]) -> Vec<Vec2> {
    let (l, d) = (frame.len, frame.depth);
    let mut local: Vec<(f32, f32)> = vec![(0.0, 0.0)];
    for n in front {
        local.extend([(n.x0, 0.0), (n.x0, n.depth), (n.x1, n.depth), (n.x1, 0.0)]);
    }
    local.extend([(l, 0.0), (l, d)]);
    for n in back.iter().rev() {
        local.extend([
            (n.x1, d),
            (n.x1, d - n.depth),
            (n.x0, d - n.depth),
            (n.x0, d),
        ]);
    }
    local.push((0.0, d));
    let mut ring: Vec<Vec2> = local.iter().map(|(x, y)| frame.at(*x, *y)).collect();
    if signed_area(&ring) > 0.0 {
        ring.reverse();
    }
    ring
}

/// Emit a quad whose face normal points along `outward`, whatever order its
/// corners arrive in.
pub(super) fn face(
    builder: &mut MeshBuilder,
    material: &str,
    q: [Vec3; 4],
    outward: Vec3,
    uv: Option<[(f32, f32); 4]>,
    colour: Option<[f32; 3]>,
) {
    let n = (q[1] - q[0]).cross(q[2] - q[0]);
    let flip = n.dot(outward) < 0.0;
    let (q, uv) = if flip {
        (
            [q[0], q[3], q[2], q[1]],
            uv.map(|uv| [uv[0], uv[3], uv[2], uv[1]]),
        )
    } else {
        (q, uv)
    };
    match uv {
        Some(uv) => builder.quad_uv(material, q[0], q[1], q[2], q[3], uv, colour),
        None => builder.quad(material, q[0], q[1], q[2], q[3], colour),
    }
}

/// The outer face of a projection drawn in an elevation material: the
/// material key, the face's UVs and its vertex colour.
pub(super) type FrontFace<'a> = (&'a str, [(f32, f32); 4], [f32; 3]);

/// A box standing out from a wall: `p0 → p1` along the wall, `out` the wall's
/// outward unit normal, `depth` the projection, `y0 .. y1` its height.  The face
/// against the wall is not built.  `front_uv` maps the outer face onto an
/// elevation texture, with `material_front`; the rest is `material`.
#[allow(clippy::too_many_arguments)]
pub(super) fn projection(
    builder: &mut MeshBuilder,
    material: &str,
    colour: Option<[f32; 3]>,
    p0: Vec2,
    p1: Vec2,
    out: Vec2,
    depth: f32,
    y0: f32,
    y1: f32,
    front: Option<FrontFace<'_>>,
) {
    let q0 = p0 + out * depth;
    let q1 = p1 + out * depth;
    let along = (p1 - p0).normalize();
    let o3 = Vec3::new(out.x, 0.0, out.y);
    let a3 = Vec3::new(along.x, 0.0, along.y);
    let front_q = [
        Vec3::from_plan(q0, y0),
        Vec3::from_plan(q1, y0),
        Vec3::from_plan(q1, y1),
        Vec3::from_plan(q0, y1),
    ];
    match front {
        Some((elevation, uv, tint)) => face(builder, elevation, front_q, o3, Some(uv), Some(tint)),
        None => face(builder, material, front_q, o3, None, colour),
    }
    face(
        builder,
        material,
        [
            Vec3::from_plan(p0, y0),
            Vec3::from_plan(q0, y0),
            Vec3::from_plan(q0, y1),
            Vec3::from_plan(p0, y1),
        ],
        a3 * -1.0,
        None,
        colour,
    );
    face(
        builder,
        material,
        [
            Vec3::from_plan(p1, y0),
            Vec3::from_plan(q1, y0),
            Vec3::from_plan(q1, y1),
            Vec3::from_plan(p1, y1),
        ],
        a3,
        None,
        colour,
    );
    face(
        builder,
        material,
        [
            Vec3::from_plan(p0, y1),
            Vec3::from_plan(p1, y1),
            Vec3::from_plan(q1, y1),
            Vec3::from_plan(q0, y1),
        ],
        Vec3::new(0.0, 1.0, 0.0),
        None,
        colour,
    );
    face(
        builder,
        material,
        [
            Vec3::from_plan(p0, y0),
            Vec3::from_plan(p1, y0),
            Vec3::from_plan(q1, y0),
            Vec3::from_plan(q0, y0),
        ],
        Vec3::new(0.0, -1.0, 0.0),
        None,
        colour,
    );
}

/// One wall's mapping onto an elevation: where along the wall a tile `u`
/// lands, and the `V` of a height.  `V` counts storeys up from the
/// elevation's base, which is the procedural facade's contract.
pub(super) struct WallMap {
    pub(super) a: Vec2,
    pub(super) along: Vec2,
    pub(super) out: Vec2,
    /// `u` at `a`, and metres of `u` per metre of wall.
    pub(super) u0: f32,
    pub(super) u_per_m: f32,
    /// Whole tiles on this wall; zero on a gable (no projections).
    pub(super) units: u32,
    /// 层高, for `V`.
    pub(super) storey_m: f32,
}

impl WallMap {
    pub(super) fn point(&self, u: f32) -> Vec2 {
        self.a + self.along * ((u - self.u0) / self.u_per_m)
    }
    /// `V` at a height `ml` above the floor of storey `s` (0 = lowest).
    pub(super) fn v(&self, s: f32, ml: f32) -> f32 {
        s + ml / self.storey_m
    }
    /// A `u` span shrunk by `inset` metres of wall at both ends.
    pub(super) fn shrink(&self, ua: f32, ub: f32, inset: f32) -> (f32, f32) {
        let d = inset * self.u_per_m;
        (ua + d, ub - d)
    }
}

/// The paint field's own reflectance in the facade shader: a wall's vertex
/// colour is its render colour divided by this.
pub(super) const PAINT_BASE: f32 = 0.86;

/// The vertex colour of a facade wall painted `paint` (final reflectance).
pub(super) fn wall_tint(paint: [f32; 3]) -> [f32; 3] {
    [
        (paint[0] / PAINT_BASE).min(1.0),
        (paint[1] / PAINT_BASE).min(1.0),
        (paint[2] / PAINT_BASE).min(1.0),
    ]
}

/// Laminated glass of a balcony rail: dark, smooth, reflective.
pub(super) const RAIL_GLASS: &str = "cnres.glass";

/// A facade wall from `a` to `b`, storeys `s0..s1` of an elevation whose base
/// is at `base`, drawn from table row `row` in `tint`.
#[allow(clippy::too_many_arguments)]
pub(super) fn facade_wall(
    builder: &mut MeshBuilder,
    map: &WallMap,
    b: Vec2,
    base: f32,
    s0: u32,
    s1: u32,
    row: u8,
    tint: [f32; 3],
) {
    if s1 <= s0 {
        return;
    }
    let length = map.a.distance(b);
    let u1 = map.u0 + length * map.u_per_m;
    let (y0, y1) = (
        base + s0 as f32 * map.storey_m,
        base + s1 as f32 * map.storey_m,
    );
    builder.facade_row = Some(row);
    builder.quad_uv(
        FACADE,
        Vec3::from_plan(map.a, y0),
        Vec3::from_plan(b, y0),
        Vec3::from_plan(b, y1),
        Vec3::from_plan(map.a, y1),
        [
            (map.u0, s0 as f32),
            (u1, s0 as f32),
            (u1, s1 as f32),
            (map.u0, s1 as f32),
        ],
        Some(tint),
    );
    builder.facade_row = None;
}

/// The outward normal of a ring edge.
pub(super) fn edge_out(a: Vec2, b: Vec2, centre: Vec2) -> Vec2 {
    let along = (b - a).normalize();
    let out = Vec2::new(along.y, -along.x);
    if out.dot((a + b) * 0.5 - centre) < 0.0 {
        -out
    } else {
        out
    }
}

/// A split-unit condenser on a bracket, hung on the wall at `u`.
pub(super) fn hung_condenser(map: &WallMap, u: f32, y: f32, builder: &mut MeshBuilder) {
    let at = map.point(u) + map.out * 0.22;
    let angle = map.along.y.atan2(map.along.x);
    box_at(builder, "metal.ac", at, y + 0.27, 0.80, 0.54, 0.30, angle);
}

/// The shop storey (底商): the shopfront run of [`crate::facades::cn_shopfront`]
/// all round the plan, continuing round corners, with each shop's signboard
/// built as a board standing off the wall whose face carries the lettering.
pub(super) fn ground_shops(plan: &[Vec2], building: &ModernBuilding, builder: &mut MeshBuilder) {
    use crate::facades::cn_shopfront::{SHOPFRONT, SIGN_Y0, SIGN_Y1, shop_spans};
    let tile_w = crate::facades::GROUND_FLOOR_TILE_W;
    // A per-building seed for the shop interiors, carried as a near-white
    // vertex colour (the tile has no paint class, so it tints nothing).
    let seed = [1.0, 0.9 + 0.1 * hash_u32(building.id, 271, 5), 1.0];
    let centre = ring_centroid(plan);
    let v_sign0 = (GROUND_STOREY_M - SIGN_Y1) / GROUND_STOREY_M;
    let v_sign1 = (GROUND_STOREY_M - SIGN_Y0) / GROUND_STOREY_M;
    let mut u_run = hash_u32(building.id, 273, 5) * tile_w;
    for index in 0..plan.len() {
        let a = plan[index];
        let b = plan[(index + 1) % plan.len()];
        let length = a.distance(b);
        if length <= 1.0e-3 {
            continue;
        }
        let along = (b - a) / length;
        let mut out = Vec2::new(along.y, -along.x);
        if out.dot((a + b) * 0.5 - centre) < 0.0 {
            out = -out;
        }
        let u0 = u_run;
        let u1 = u0 + length;
        u_run = u1;
        builder.quad_uv(
            SHOPFRONT,
            Vec3::from_plan(a, level::GROUND),
            Vec3::from_plan(b, level::GROUND),
            Vec3::from_plan(b, level::GROUND + GROUND_STOREY_M),
            Vec3::from_plan(a, level::GROUND + GROUND_STOREY_M),
            [(u0, 1.0), (u1, 1.0), (u1, 0.0), (u0, 0.0)],
            Some(seed),
        );
        if length < 2.5 {
            continue;
        }
        // The signboards, shop by shop, clipped to this wall.
        let first = (u0 / tile_w).floor() as i32;
        let last = (u1 / tile_w).floor() as i32;
        for tile in first..=last {
            for (x0, x1) in shop_spans() {
                let sa = (tile as f32 * tile_w + x0 + 0.04).max(u0 + 0.3);
                let sb = (tile as f32 * tile_w + x1 - 0.04).min(u1 - 0.3);
                if sb - sa < 1.2 {
                    continue;
                }
                let p0 = a + along * (sa - u0);
                let p1 = a + along * (sb - u0);
                projection(
                    builder,
                    "trim.dark",
                    None,
                    p0,
                    p1,
                    out,
                    0.22,
                    level::GROUND + SIGN_Y0,
                    level::GROUND + SIGN_Y1,
                    Some((
                        SHOPFRONT,
                        [(sa, v_sign1), (sb, v_sign1), (sb, v_sign0), (sa, v_sign0)],
                        seed,
                    )),
                );
            }
        }
    }
}

/// A parapet in the body colour with a white coping.
pub(super) fn parapet(
    ring: &[Vec2],
    deck: f32,
    height: f32,
    paint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let centre = ring_centroid(ring);
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let mut out = (b - a).normalize();
        out = Vec2::new(out.y, -out.x);
        if out.dot((a + b) * 0.5 - centre) < 0.0 {
            out = -out;
        }
        let o3 = Vec3::new(out.x, 0.0, out.y);
        face(
            builder,
            PAINT,
            [
                Vec3::from_plan(a, deck),
                Vec3::from_plan(b, deck),
                Vec3::from_plan(b, deck + height),
                Vec3::from_plan(a, deck + height),
            ],
            o3,
            None,
            Some(paint),
        );
        let ai = a - out * 0.2;
        let bi = b - out * 0.2;
        face(
            builder,
            PAINT,
            [
                Vec3::from_plan(ai, deck),
                Vec3::from_plan(bi, deck),
                Vec3::from_plan(bi, deck + height),
                Vec3::from_plan(ai, deck + height),
            ],
            o3 * -1.0,
            None,
            Some(paint),
        );
        // The coping: white, projecting 60 mm.
        projection(
            builder,
            TRIM,
            None,
            ai,
            bi,
            out,
            0.26,
            deck + height,
            deck + height + 0.12,
            None,
        );
    }
}

/// The clay a hipped roof is covered in: grey 小青瓦 or red tile.
pub(super) fn roof_clay(terracotta: bool) -> &'static str {
    if terracotta {
        "roof.terracotta"
    } else {
        "roof.tile"
    }
}

/// A hipped roof of `material` over a rectangle, `eave` metres out, rising at
/// `pitch` (rise over run).
pub(super) fn hip_roof(
    frame: &Frame,
    top: f32,
    material: &str,
    eave: f32,
    pitch: f32,
    builder: &mut MeshBuilder,
) {
    let (len, depth) = (frame.len, frame.depth);
    let half = depth * 0.5 + eave;
    let rise = half * pitch;
    let y0 = top + 0.05;
    let y1 = y0 + rise;
    let r0 = (half - eave).min(len * 0.5);
    let r1 = (len - (half - eave)).max(len * 0.5);
    let p = |x: f32, y: f32, h: f32| Vec3::from_plan(frame.at(x, y), h);
    let mid = depth * 0.5;
    let slope = (half * half + rise * rise).sqrt();
    let up = Vec3::new(0.0, 1.0, 0.0);
    let eb3 = Vec3::new(frame.eb.x, 0.0, frame.eb.y);
    let ea3 = Vec3::new(frame.ea.x, 0.0, frame.ea.y);
    for (y_eave, sign) in [(-eave, -1.0_f32), (depth + eave, 1.0)] {
        face(
            builder,
            material,
            [
                p(-eave, y_eave, y0),
                p(len + eave, y_eave, y0),
                p(r1, mid, y1),
                p(r0, mid, y1),
            ],
            up + eb3 * sign,
            Some([(-eave, slope), (len + eave, slope), (r1, 0.0), (r0, 0.0)]),
            None,
        );
    }
    for (x_eave, x_ridge, sign) in [(-eave, r0, -1.0_f32), (len + eave, r1, 1.0)] {
        let a = p(x_eave, -eave, y0);
        let b = p(x_eave, depth + eave, y0);
        let c = p(x_ridge, mid, y1);
        let n = (b - a).cross(c - a);
        let want = up + ea3 * sign;
        let (b, c, uvb, uvc) = if n.dot(want) < 0.0 {
            (c, b, (half, 0.0), (2.0 * half, slope))
        } else {
            (b, c, (2.0 * half, slope), (half, 0.0))
        };
        builder.tri_uv(material, a, b, c, [(0.0, slope), uvb, uvc], None);
    }
    // The soffit and a fascia, so the eave is a thing and not a fold.
    let ring = [
        frame.at(-eave, -eave),
        frame.at(len + eave, -eave),
        frame.at(len + eave, depth + eave),
        frame.at(-eave, depth + eave),
    ];
    for index in 0..4 {
        let a = ring[index];
        let b = ring[(index + 1) % 4];
        let mut out = (b - a).normalize();
        out = Vec2::new(out.y, -out.x);
        let centre = frame.at(len * 0.5, mid);
        if out.dot((a + b) * 0.5 - centre) < 0.0 {
            out = -out;
        }
        face(
            builder,
            TRIM,
            [
                Vec3::from_plan(a, y0 - 0.22),
                Vec3::from_plan(b, y0 - 0.22),
                Vec3::from_plan(b, y0),
                Vec3::from_plan(a, y0),
            ],
            Vec3::new(out.x, 0.0, out.y),
            None,
            None,
        );
    }
    let soffit = [
        frame.at(-eave, -eave),
        frame.at(len + eave, -eave),
        frame.at(len + eave, depth + eave),
        frame.at(-eave, depth + eave),
    ];
    face(
        builder,
        TRIM,
        [
            Vec3::from_plan(soffit[0], y0 - 0.22),
            Vec3::from_plan(soffit[1], y0 - 0.22),
            Vec3::from_plan(soffit[2], y0 - 0.22),
            Vec3::from_plan(soffit[3], y0 - 0.22),
        ],
        Vec3::new(0.0, -1.0, 0.0),
        None,
        None,
    );
}
