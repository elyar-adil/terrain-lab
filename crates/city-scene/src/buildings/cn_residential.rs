//! The Chinese residential block: 板楼, 点式塔楼 and 多层, built from the flat
//! elevations of [`crate::facades::cn_elevation`].
//!
//! The generic shell (`shell.rs`) builds every building from one bay grammar
//! with balconies stuck on; this module builds the residential stock of a
//! `ChineseModern` city the way it is actually built, because that stock is
//! most of what a photograph of a Chinese district shows:
//!
//! * **The plan faces the sun.**  The long side whose outward normal is closest
//!   to the sun's azimuth is the living side (balconies, bedrooms, bay windows);
//!   the opposite side is the service side (kitchens, bathrooms, the stair and
//!   the unit entrances); the ends are gables.
//! * **The plan is notched.**  A point tower has a recess between its two flats
//!   on the living side and two light wells on the service side; a long slab has
//!   a light well at every other party wall.  Those notches are the deep
//!   vertical shadows that make a tower read as a tower and not as a box.
//! * **Everything that projects, projects at the painted span.**  Balconies,
//!   bay windows and AC platforms are built at exactly the spans the elevation
//!   paints, per storey, and their faces sample the painted opening they stand
//!   in front of — so the glazing on a projecting bay window is the same pane,
//!   with the same curtain, as the one painted on the wall behind it.
//! * **The colour is the building's, not the tile's.**  The elevation's paint
//!   is near-white and the wall's vertex colour carries the building's render
//!   colour, drawn from the palette measured on the reference photographs.
//! * **The top is a crown.**  A tower ends in a frame (构架) or a hipped roof
//!   over its lift machine room; a walk-up ends in a parapet or a tiled roof.

use urban::{ModernBuilding, ParcelUse};

use super::{hash_u32, level, roofscape};
use crate::facades::GROUND_STOREY_M;
use crate::facades::cn_elevation::{
    BAY_HEAD_M, BAY_SILL_M, CN_STOREY_M, DOOR_HEAD_M, LAYOUTS, LOUVRE_TOP_M, Layout, PAINT_BASE,
    Part, Side, layout,
};
use crate::math::{Vec2, Vec3, inset_ring, ring_centroid, signed_area};
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

/// The render colours of Chinese residential stock, as final linear
/// reflectance, with their weights.  Sampled from the reference photographs
/// (Beijing 1990s towers, Shenzhen 2000s high-rise, Tianjin walk-ups): cream
/// and beige lead, off-white and peach follow, grey and tan are the minority.
const PALETTE: [([f32; 3], f32); 8] = [
    ([0.66, 0.55, 0.36], 3.0), // cream yellow
    ([0.62, 0.56, 0.45], 3.0), // light beige
    ([0.70, 0.69, 0.64], 3.0), // off-white
    ([0.66, 0.48, 0.38], 1.5), // peach
    ([0.55, 0.56, 0.56], 1.0), // pale grey
    ([0.52, 0.42, 0.31], 1.0), // warm tan
    ([0.70, 0.58, 0.33], 1.0), // light ochre
    ([0.52, 0.56, 0.60], 0.5), // pale grey-blue
];

/// Whether this module builds a building.
pub(crate) fn applies(building: &ModernBuilding) -> bool {
    match building.use_type {
        ParcelUse::Residential => building.floors >= 3,
        // Shops with flats over them, however low.
        ParcelUse::MixedUse => building.floors >= 2,
        _ => false,
    }
}

/// The building's render colour, final linear reflectance.
pub(crate) fn paint_colour(id: u32) -> [f32; 3] {
    let total: f32 = PALETTE.iter().map(|(_, w)| w).sum();
    let mut pick = hash_u32(id, 211, 7) * total;
    let mut colour = PALETTE[0].0;
    for (candidate, weight) in PALETTE {
        if pick < weight {
            colour = candidate;
            break;
        }
        pick -= weight;
    }
    let light = 0.94 + 0.10 * hash_u32(id, 213, 7);
    [colour[0] * light, colour[1] * light, colour[2] * light]
}

/// The vertex colour for an elevation wall: the paint colour divided by the
/// paint field's own reflectance, so the product is the paint colour.
fn wall_tint(paint: [f32; 3]) -> [f32; 3] {
    [
        (paint[0] / PAINT_BASE).min(1.0),
        (paint[1] / PAINT_BASE).min(1.0),
        (paint[2] / PAINT_BASE).min(1.0),
    ]
}

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
    x0: f32,
    x1: f32,
    depth: f32,
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

/// How a tower or a slab is notched.
fn notches_for(frame: &Frame, tower: bool, id: u32) -> (Vec<Notch>, Vec<Notch>) {
    let (l, d) = (frame.len, frame.depth);
    let mut front = Vec::new();
    let mut back = Vec::new();
    if d < 9.0 {
        return (front, back);
    }
    if tower {
        if l >= 20.0 {
            let w = 2.2 + 0.6 * hash_u32(id, 221, 3);
            front.push(Notch {
                x0: l * 0.5 - w * 0.5,
                x1: l * 0.5 + w * 0.5,
                depth: (1.4 + 0.8 * hash_u32(id, 223, 3)).min(d * 0.18),
            });
        }
        if l >= 18.0 {
            let w = 2.0 + 0.4 * hash_u32(id, 227, 3);
            let depth = (3.0 + 1.6 * hash_u32(id, 229, 3)).min(d * 0.32);
            for t in [0.30_f32, 0.70] {
                back.push(Notch {
                    x0: l * t - w * 0.5,
                    x1: l * t + w * 0.5,
                    depth,
                });
            }
        }
    } else if l >= 34.0 && hash_u32(id, 231, 3) < 0.65 {
        // A light well at every other party wall of a long slab.
        let flats = (l / 12.0).round().max(2.0);
        let pitch = l / flats;
        let mut k = 1.0;
        while k < flats {
            let x = k * pitch;
            back.push(Notch {
                x0: x - 1.0,
                x1: x + 1.0,
                depth: (2.2_f32).min(d * 0.25),
            });
            k += 2.0;
        }
    }
    (front, back)
}

/// Which elevations a building wears.
struct Dress {
    front: &'static Layout,
    back: &'static Layout,
    gable: &'static Layout,
    /// Living-room balconies glazed in (封阳台).
    enclosed: bool,
    /// Balcony parapets in white trim rather than body paint.
    white_parapet: bool,
    balcony_depth: f32,
}

fn layout_of(key: &str) -> &'static Layout {
    layout(key).unwrap_or(&LAYOUTS[0])
}

fn dress_for(id: u32, tower: bool) -> Dress {
    let roll = hash_u32(id, 233, 5);
    let front = if tower {
        if roll < 0.40 {
            "cnres/front-b"
        } else if roll < 0.75 {
            "cnres/front-c"
        } else {
            "cnres/front-a"
        }
    } else if roll < 0.45 {
        "cnres/front-a"
    } else if roll < 0.80 {
        "cnres/front-b"
    } else {
        "cnres/front-c"
    };
    let modern = front == "cnres/front-c";
    let back = if tower || hash_u32(id, 235, 5) < 0.3 {
        "cnres/back-b"
    } else {
        "cnres/back-a"
    };
    Dress {
        front: layout_of(front),
        back: layout_of(back),
        gable: layout_of("cnres/gable"),
        enclosed: !modern && hash_u32(id, 237, 5) < 0.5,
        white_parapet: modern || hash_u32(id, 239, 5) < 0.35,
        balcony_depth: if modern { 1.5 } else { 1.3 },
    }
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
/// lands, and the `V` of a height.
pub(super) struct WallMap {
    pub(super) a: Vec2,
    pub(super) along: Vec2,
    pub(super) out: Vec2,
    /// `u` at `a`, and metres of `u` per metre of wall.
    pub(super) u0: f32,
    pub(super) u_per_m: f32,
    /// Whole tiles on this wall; zero on a gable (no projections).
    pub(super) units: u32,
    /// Storey count and 层高, for `V`.
    pub(super) storeys: f32,
    pub(super) storey_m: f32,
}

impl WallMap {
    pub(super) fn point(&self, u: f32) -> Vec2 {
        self.a + self.along * ((u - self.u0) / self.u_per_m)
    }
    /// `V` at a height `ml` above the floor of storey `s` (0 = lowest).
    pub(super) fn v(&self, s: f32, ml: f32) -> f32 {
        (self.storeys - s - ml / self.storey_m) / 4.0
    }
}

/// Everything a building needs to know to dress a wall.
struct Ctx<'a> {
    id: u32,
    dress: &'a Dress,
    base: f32,
    storeys: u32,
    bands: Bands,
}

impl Ctx<'_> {
    /// The render colour of storey `s` (0 = lowest).
    fn paint_at(&self, s: u32) -> [f32; 3] {
        self.bands.paint_at(s, self.storeys)
    }
    /// The elevation vertex colour of storey `s`.
    fn tint_at(&self, s: u32) -> [f32; 3] {
        wall_tint(self.paint_at(s))
    }
}

/// 三段式: most Chinese residential blocks are painted in three bands — a
/// darker base of one to three storeys (stone-coloured or a deeper shade of the
/// body), the body, and on many a lighter or contrasting top storey under the
/// roof.  It is the commonest colour scheme in the reference photographs and
/// the cheapest thing that stops a block reading as one extruded colour.
#[derive(Debug, Clone, Copy)]
struct Bands {
    body: [f32; 3],
    base: [f32; 3],
    base_storeys: u32,
    top: [f32; 3],
    top_storeys: u32,
}

impl Bands {
    fn of(id: u32, body: [f32; 3], storeys: u32, tower: bool) -> Self {
        let roll = hash_u32(id, 281, 5);
        let base_storeys = if storeys < 5 {
            0
        } else if tower {
            2 + (roll * 2.0) as u32
        } else if roll < 0.3 {
            0
        } else if roll < 0.75 {
            1
        } else {
            2
        };
        let shade = |c: [f32; 3], k: f32| [c[0] * k, c[1] * k, c[2] * k];
        let base = match (hash_u32(id, 283, 5) * 4.0) as u32 {
            0 => [0.40, 0.34, 0.28],
            1 => [0.46, 0.44, 0.41],
            2 => [0.52, 0.38, 0.29],
            _ => shade(body, 0.72),
        };
        let top_roll = hash_u32(id, 285, 5);
        let top_storeys = if storeys >= 6 && top_roll < 0.55 {
            1
        } else {
            0
        };
        let top = if top_roll < 0.3 {
            [0.72, 0.71, 0.68]
        } else {
            shade(body, 0.84)
        };
        Self {
            body,
            base,
            base_storeys,
            top,
            top_storeys,
        }
    }

    fn paint_at(&self, s: u32, storeys: u32) -> [f32; 3] {
        if s < self.base_storeys {
            self.base
        } else if s + self.top_storeys >= storeys && s >= self.base_storeys {
            self.top
        } else {
            self.body
        }
    }
}

/// A storey layout: where the elevations start and how many storeys they carry.
/// A mixed-use block stands on a 4.5 m shop storey; a compound block on a
/// raised ground floor (首层抬高 0.45 m) behind a stone plinth; a block that
/// stands on another building's podium starts on its roof.
fn storeys_of(building: &ModernBuilding, podium: Option<(f32, u16)>) -> (f32, u32) {
    match podium {
        Some((top, floors)) => (top, (building.floors.saturating_sub(floors) as u32).max(3)),
        None if building.use_type == ParcelUse::MixedUse => (
            level::GROUND + GROUND_STOREY_M,
            (building.floors as u32).saturating_sub(1).max(1),
        ),
        None => (level::GROUND + 0.45, (building.floors as u32).max(1)),
    }
}

/// The roof deck height of a block this module builds on the ground.
pub(crate) fn roof_height(building: &ModernBuilding) -> f32 {
    let (base, storeys) = storeys_of(building, None);
    base + storeys as f32 * CN_STOREY_M
}

/// Build one Chinese residential block.  `podium` is the roof height and
/// storey count of a larger, lower building it stands on, if any.  Returns
/// the roof deck height.
pub(crate) fn shell(
    building: &ModernBuilding,
    ring: &[Vec2],
    podium: Option<(f32, u16)>,
    builder: &mut MeshBuilder,
) -> f32 {
    let id = building.id;
    let on_ground = podium.is_none();
    let retail = on_ground && building.use_type == ParcelUse::MixedUse;
    let tower = building.floors >= 19;
    let paint = paint_colour(id);
    let dress = dress_for(id, tower);

    // Plan: inset so balconies stay inside the plot, then notched.
    let mut outward = ring.to_vec();
    if signed_area(&outward) > 0.0 {
        outward.reverse();
    }
    let inset = inset_ring(&outward, 0.6);
    let frame = Frame::fit(&inset);
    let plan = match &frame {
        Some(frame) => {
            let (front, back) = notches_for(frame, tower, id);
            notched_plan(frame, &front, &back)
        }
        None => {
            let mut plan = inset.clone();
            if signed_area(&plan) > 0.0 {
                plan.reverse();
            }
            plan
        }
    };
    let centre = ring_centroid(&plan);

    let (base, storeys) = storeys_of(building, podium);
    let top = base + storeys as f32 * CN_STOREY_M;
    let bands = Bands::of(id, paint, storeys, tower);
    let ctx = Ctx {
        id,
        dress: &dress,
        base,
        storeys,
        bands,
    };

    // Plinth and ground storey.
    for index in 0..plan.len() {
        if !on_ground {
            break;
        }
        let a = plan[index];
        let b = plan[(index + 1) % plan.len()];
        builder.wall(
            "wall.render",
            a,
            b,
            level::PLINTH,
            level::GROUND + 0.45,
            None,
        );
    }
    if retail {
        ground_shops(&plan, building, builder);
    }

    // The elevations.
    let mut u_run = [0.0_f32; 3];
    for index in 0..plan.len() {
        let a = plan[index];
        let b = plan[(index + 1) % plan.len()];
        let length = a.distance(b);
        if length < 0.3 {
            continue;
        }
        let along = (b - a) / length;
        let mut out = Vec2::new(along.y, -along.x);
        if out.dot((a + b) * 0.5 - centre) < 0.0 {
            out = -out;
        }
        let side = match &frame {
            Some(frame) => {
                let facing = out.dot(-frame.eb);
                // A tower has windows all round, so its flanks take the
                // service elevation; a slab's flanks are gables.
                if facing > 0.7 {
                    Side::Front
                } else if facing < -0.7 || tower {
                    Side::Back
                } else {
                    Side::Gable
                }
            }
            None => {
                let facing = out.dot(SUN_PLAN);
                if facing > 0.5 {
                    Side::Front
                } else if facing < -0.5 || tower {
                    Side::Back
                } else {
                    Side::Gable
                }
            }
        };
        let mut layout = match side {
            Side::Front => dress.front,
            Side::Back => dress.back,
            Side::Gable => dress.gable,
        };
        if layout.side != Side::Gable && length < 0.7 * layout.width {
            layout = dress.gable;
        }
        let slot = match layout.side {
            Side::Front => 0,
            Side::Back => 1,
            Side::Gable => 2,
        };
        let map = if layout.side == Side::Gable {
            // Unstretched and centred on the gable's window, so a short end wall
            // shows one window in its middle rather than a cut one at a corner.
            let centre_u = layout.width * 0.5;
            WallMap {
                a,
                along,
                out,
                u0: centre_u - length * 0.5,
                u_per_m: 1.0,
                units: 0,
                storeys: storeys as f32,
                storey_m: CN_STOREY_M,
            }
        } else {
            let units = (length / layout.width).round().max(1.0) as u32;
            let u0 = u_run[slot];
            u_run[slot] += units as f32 * layout.width;
            WallMap {
                a,
                along,
                out,
                u0,
                u_per_m: units as f32 * layout.width / length,
                units,
                storeys: storeys as f32,
                storey_m: CN_STOREY_M,
            }
        };
        let u1 = map.u0 + length * map.u_per_m;
        // One quad per colour band, each UV'd on the same storey datum.
        let splits = [
            0,
            bands.base_storeys.min(storeys),
            storeys - bands.top_storeys.min(storeys),
            storeys,
        ];
        for pair in splits.windows(2) {
            let (s0, s1) = (pair[0], pair[1].max(pair[0]));
            if s1 == s0 {
                continue;
            }
            let (y0, y1) = (
                base + s0 as f32 * CN_STOREY_M,
                base + s1 as f32 * CN_STOREY_M,
            );
            let (v0, v1) = (map.v(s0 as f32, 0.0), map.v(s1 as f32, 0.0));
            builder.quad_uv(
                layout.key,
                Vec3::from_plan(a, y0),
                Vec3::from_plan(b, y0),
                Vec3::from_plan(b, y1),
                Vec3::from_plan(a, y1),
                [(map.u0, v0), (u1, v0), (u1, v1), (map.u0, v1)],
                Some(ctx.tint_at(s0)),
            );
        }
        if map.units > 0 {
            dress_wall(&ctx, layout, &map, builder);
        }
        if layout.side == Side::Back && on_ground && !retail {
            entrances(&ctx, layout, &map, tower, builder);
        }
        if layout.side == Side::Gable && length > 6.0 {
            gable_units(&ctx, &map, length, builder);
        }
    }

    // The roof.
    builder.ground_uv("roof", &plan, top, None);
    let pitched = !tower && building.floors <= 7 && hash_u32(id, 241, 5) < 0.45;
    if pitched {
        if let Some(frame) = &frame {
            hip_roof(frame, top, id, builder);
        } else {
            parapet(&plan, top, 1.1, paint, builder);
        }
    } else {
        parapet(&plan, top, 1.2, paint, builder);
        if signed_area(&plan).abs() > 90.0 {
            roofscape::roofscape(building, true, &plan, top, builder);
        }
        if tower {
            crown(&plan, frame.as_ref(), top, id, paint, builder);
        }
    }
    let foot = if on_ground { level::GROUND } else { base };
    super::details::downpipe(&plan, foot, top, id, builder);
    top
}

/// The projections a wall's elevation calls for, storey by storey.
fn dress_wall(ctx: &Ctx, layout: &Layout, map: &WallMap, builder: &mut MeshBuilder) {
    let w = layout.width;
    let storeys = ctx.storeys;
    for k in 0..map.units {
        let tile_u = map.u0 + k as f32 * w;
        let mirrored = k % 2 == 1;
        for (span_index, span) in layout.spans.iter().enumerate() {
            let (x0, x1) = if mirrored {
                (w - span.x1, w - span.x0)
            } else {
                (span.x0, span.x1)
            };
            let (ua, ub) = (tile_u + x0, tile_u + x1);
            let p0 = map.point(ua);
            let p1 = map.point(ub);
            match span.part {
                Part::Balcony => balcony_stack(ctx, layout, map, p0, p1, ua, ub, builder),
                Part::Bay => bay_stack(ctx, layout, map, p0, p1, ua, ub, builder),
                Part::AcNiche => louvre_stack(ctx, layout, map, p0, p1, ua, ub, builder),
                Part::Wall if layout.side == Side::Front && span.x1 - span.x0 >= 0.55 => {
                    // The 1990s tower hangs its condensers on the wall between
                    // windows, wherever the tenant drilled.
                    for s in 0..storeys {
                        if hash_u32(ctx.id ^ (k << 8), span_index as i32, s as i32) < 0.55 {
                            let y = ctx.base + s as f32 * CN_STOREY_M + 0.35;
                            hung_condenser(map, (ua + ub) * 0.5, y, builder);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// The `u` span of a part, shrunk by `inset` metres of wall at both ends.
fn shrink(ua: f32, ub: f32, inset: f32, map: &WallMap) -> (f32, f32) {
    let d = inset * map.u_per_m;
    (ua + d, ub - d)
}

#[allow(clippy::too_many_arguments)]
fn balcony_stack(
    ctx: &Ctx,
    layout: &Layout,
    map: &WallMap,
    p0: Vec2,
    p1: Vec2,
    ua: f32,
    ub: f32,
    builder: &mut MeshBuilder,
) {
    let depth = ctx.dress.balcony_depth;
    let rail_mat = if ctx.dress.white_parapet { TRIM } else { PAINT };
    let out = map.out;
    let q0 = p0 + out * depth;
    let q1 = p1 + out * depth;
    let along = (p1 - p0).normalize();
    // The door wall's glazing, which the enclosed front samples.
    let (da, db) = shrink(ua, ub, 0.30, map);
    for s in 0..ctx.storeys {
        let y = ctx.base + s as f32 * CN_STOREY_M;
        let sf = s as f32;
        // The floor slab, with its white edge.
        projection(builder, TRIM, None, p0, p1, out, depth, y - 0.14, y, None);
        // The parapet: a front and two cheeks, 1.1 m.
        let rail_colour = if ctx.dress.white_parapet {
            None
        } else {
            Some(ctx.paint_at(s))
        };
        let rail = 1.10;
        let t = 0.10;
        projection(
            builder,
            rail_mat,
            rail_colour,
            p0 + out * (depth - t),
            p1 + out * (depth - t),
            out,
            t,
            y,
            y + rail,
            None,
        );
        for (p, q, inward) in [(p0, q0, along), (p1, q1, -along)] {
            let a = p + inward * t * 0.5;
            let b = q - out * t + inward * t * 0.5;
            projection(
                builder,
                rail_mat,
                rail_colour,
                a,
                b,
                -inward,
                t * 0.5,
                y,
                y + rail,
                None,
            );
        }
        if ctx.dress.enclosed {
            // 封阳台: aluminium glazing from the parapet to the slab above,
            // sampled from the door glazing painted on the wall behind.
            let gy0 = y + rail;
            let gy1 = y + CN_STOREY_M - 0.14;
            let front_uv = [
                (da, map.v(sf, 1.0)),
                (db, map.v(sf, 1.0)),
                (db, map.v(sf, DOOR_HEAD_M)),
                (da, map.v(sf, DOOR_HEAD_M)),
            ];
            face(
                builder,
                layout.key,
                [
                    Vec3::from_plan(q0 - out * 0.06, gy0),
                    Vec3::from_plan(q1 - out * 0.06, gy0),
                    Vec3::from_plan(q1 - out * 0.06, gy1),
                    Vec3::from_plan(q0 - out * 0.06, gy1),
                ],
                Vec3::new(out.x, 0.0, out.y),
                Some(front_uv),
                Some(ctx.tint_at(s)),
            );
            let side_w = depth * map.u_per_m;
            for (p, q, normal) in [(p0, q0, -along), (p1, q1, along)] {
                let (sa, sb) = if normal.dot(along) < 0.0 {
                    (da, da + side_w)
                } else {
                    (db - side_w, db)
                };
                face(
                    builder,
                    layout.key,
                    [
                        Vec3::from_plan(p, gy0),
                        Vec3::from_plan(q - out * 0.06, gy0),
                        Vec3::from_plan(q - out * 0.06, gy1),
                        Vec3::from_plan(p, gy1),
                    ],
                    Vec3::new(normal.x, 0.0, normal.y),
                    Some([
                        (sa, map.v(sf, 1.0)),
                        (sb, map.v(sf, 1.0)),
                        (sb, map.v(sf, DOOR_HEAD_M)),
                        (sa, map.v(sf, DOOR_HEAD_M)),
                    ]),
                    Some(ctx.tint_at(s)),
                );
            }
        }
    }
    // The roof over the top balcony.
    let y = ctx.base + ctx.storeys as f32 * CN_STOREY_M;
    projection(
        builder,
        TRIM,
        None,
        p0,
        p1,
        out,
        depth,
        y - 0.14,
        y + 0.05,
        None,
    );
}

#[allow(clippy::too_many_arguments)]
fn bay_stack(
    ctx: &Ctx,
    layout: &Layout,
    map: &WallMap,
    p0: Vec2,
    p1: Vec2,
    ua: f32,
    ub: f32,
    builder: &mut MeshBuilder,
) {
    let depth = 0.5;
    let out = map.out;
    let along = (p1 - p0).normalize();
    let (ga, gb) = shrink(ua, ub, 0.06, map);
    let g0 = p0 + along * 0.06;
    let g1 = p1 - along * 0.06;
    for s in 0..ctx.storeys {
        let y = ctx.base + s as f32 * CN_STOREY_M;
        let sf = s as f32;
        // The head of this storey's bay and the sill of the next are one slab
        // of concrete, so they are one box.
        let below = if s == 0 {
            y
        } else {
            y - (CN_STOREY_M - BAY_HEAD_M)
        };
        projection(
            builder,
            PAINT,
            Some(ctx.paint_at(s)),
            p0,
            p1,
            out,
            depth,
            below,
            y + BAY_SILL_M,
            None,
        );
        // The glazing, pushed out to the face of the box.
        let front_uv = [
            (ga, map.v(sf, BAY_SILL_M)),
            (gb, map.v(sf, BAY_SILL_M)),
            (gb, map.v(sf, BAY_HEAD_M)),
            (ga, map.v(sf, BAY_HEAD_M)),
        ];
        face(
            builder,
            layout.key,
            [
                Vec3::from_plan(g0 + out * (depth - 0.04), y + BAY_SILL_M),
                Vec3::from_plan(g1 + out * (depth - 0.04), y + BAY_SILL_M),
                Vec3::from_plan(g1 + out * (depth - 0.04), y + BAY_HEAD_M),
                Vec3::from_plan(g0 + out * (depth - 0.04), y + BAY_HEAD_M),
            ],
            Vec3::new(out.x, 0.0, out.y),
            Some(front_uv),
            Some(ctx.tint_at(s)),
        );
        let side_w = depth * map.u_per_m;
        for (p, normal, (sa, sb)) in [
            (p0, -along, (ga, ga + side_w)),
            (p1, along, (gb - side_w, gb)),
        ] {
            face(
                builder,
                layout.key,
                [
                    Vec3::from_plan(p, y + BAY_SILL_M),
                    Vec3::from_plan(p + out * depth, y + BAY_SILL_M),
                    Vec3::from_plan(p + out * depth, y + BAY_HEAD_M),
                    Vec3::from_plan(p, y + BAY_HEAD_M),
                ],
                Vec3::new(normal.x, 0.0, normal.y),
                Some([
                    (sa, map.v(sf, BAY_SILL_M)),
                    (sb, map.v(sf, BAY_SILL_M)),
                    (sb, map.v(sf, BAY_HEAD_M)),
                    (sa, map.v(sf, BAY_HEAD_M)),
                ]),
                Some(ctx.tint_at(s)),
            );
        }
    }
    let y = ctx.base + ctx.storeys as f32 * CN_STOREY_M;
    projection(
        builder,
        PAINT,
        Some(ctx.paint_at(ctx.storeys.saturating_sub(1))),
        p0,
        p1,
        out,
        depth,
        y - (CN_STOREY_M - BAY_HEAD_M),
        y,
        None,
    );
}

#[allow(clippy::too_many_arguments)]
fn louvre_stack(
    ctx: &Ctx,
    layout: &Layout,
    map: &WallMap,
    p0: Vec2,
    p1: Vec2,
    ua: f32,
    ub: f32,
    builder: &mut MeshBuilder,
) {
    let depth = 0.55;
    for s in 0..ctx.storeys {
        let y = ctx.base + s as f32 * CN_STOREY_M;
        let sf = s as f32;
        let uv = [
            (ua, map.v(sf, 0.0)),
            (ub, map.v(sf, 0.0)),
            (ub, map.v(sf, LOUVRE_TOP_M)),
            (ua, map.v(sf, LOUVRE_TOP_M)),
        ];
        projection(
            builder,
            TRIM,
            None,
            p0,
            p1,
            map.out,
            depth,
            y,
            y + LOUVRE_TOP_M,
            Some((layout.key, uv, ctx.tint_at(s))),
        );
    }
}

/// A split-unit condenser on a bracket, hung on the wall at `u`.
fn hung_condenser(map: &WallMap, u: f32, y: f32, builder: &mut MeshBuilder) {
    let at = map.point(u) + map.out * 0.22;
    let angle = map.along.y.atan2(map.along.x);
    box_at(builder, "metal.ac", at, y + 0.27, 0.80, 0.54, 0.30, angle);
}

/// Air conditioners on a gable: a column beside each window.
fn gable_units(ctx: &Ctx, map: &WallMap, length: f32, builder: &mut MeshBuilder) {
    // The gable's window sits on the tile centre; the condenser hangs 1.1 m to
    // one side of it, in the wall, on most storeys.
    let centre_u = map.u0 + length * 0.5 * map.u_per_m;
    let side = if hash_u32(ctx.id, 251, 5) < 0.5 {
        -1.0
    } else {
        1.0
    };
    for s in 0..ctx.storeys {
        if hash_u32(ctx.id, 253, s as i32) < 0.7 {
            let y = ctx.base + s as f32 * CN_STOREY_M + 0.4;
            hung_condenser(map, centre_u + side * 1.25, y, builder);
        }
    }
}

/// Unit entrances (单元门) on the service side: a door, a canopy, two steps.
fn entrances(ctx: &Ctx, layout: &Layout, map: &WallMap, tower: bool, builder: &mut MeshBuilder) {
    let w = layout.width;
    let mut doors: Vec<f32> = Vec::new();
    if tower || map.units < 2 {
        if map.units >= 1 && hash_u32(ctx.id, 255, 5) < if tower { 1.0 } else { 0.5 } {
            doors.push(map.u0 + map.units as f32 * w * 0.5);
        }
    } else {
        let mut k = 1;
        while k < map.units {
            doors.push(map.u0 + k as f32 * w);
            k += 2;
        }
    }
    let angle = map.along.y.atan2(map.along.x);
    for u in doors {
        let at = map.point(u);
        let y = level::GROUND;
        // The door: a dark glazed leaf in a stone surround.
        box_at(
            builder,
            "trim.dark",
            at + map.out * 0.03,
            y + 1.35,
            1.6,
            2.4,
            0.06,
            angle,
        );
        box_at(
            builder,
            TRIM,
            at + map.out * 0.05,
            y + 2.62,
            2.0,
            0.16,
            0.10,
            angle,
        );
        // The canopy.
        box_at(
            builder,
            TRIM,
            at + map.out * 0.75,
            y + 2.95,
            2.8,
            0.16,
            1.5,
            angle,
        );
        // Steps up to the raised ground floor.
        box_at(
            builder,
            "wall.render",
            at + map.out * 0.9,
            y + 0.15,
            2.8,
            0.30,
            1.8,
            angle,
        );
        box_at(
            builder,
            "wall.render",
            at + map.out * 0.6,
            y + 0.375,
            2.8,
            0.15,
            1.2,
            angle,
        );
    }
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

/// The crown of a residential tower: a white frame (构架) around the roof, or
/// a hipped roof over the lift machine room.
fn crown(
    plan: &[Vec2],
    frame: Option<&Frame>,
    top: f32,
    id: u32,
    paint: [f32; 3],
    builder: &mut MeshBuilder,
) {
    let roll = hash_u32(id, 257, 5);
    let Some(frame) = frame else {
        return;
    };
    // The machine room: a box over the core, in the body colour.
    let core_w = (frame.len * 0.32).clamp(6.0, 12.0);
    let core_d = (frame.depth * 0.40).clamp(4.5, 8.0);
    let cx = frame.len * 0.5;
    let cy = frame.depth * 0.55;
    let core = Frame {
        o: frame.at(cx - core_w * 0.5, cy - core_d * 0.5),
        ea: frame.ea,
        eb: frame.eb,
        len: core_w,
        depth: core_d,
    };
    let core_ring = notched_plan(&core, &[], &[]);
    let core_h = 4.2;
    for index in 0..core_ring.len() {
        let a = core_ring[index];
        let b = core_ring[(index + 1) % core_ring.len()];
        let centre = ring_centroid(&core_ring);
        let mut out = (b - a).normalize();
        out = Vec2::new(out.y, -out.x);
        if out.dot((a + b) * 0.5 - centre) < 0.0 {
            out = -out;
        }
        face(
            builder,
            PAINT,
            [
                Vec3::from_plan(a, top),
                Vec3::from_plan(b, top),
                Vec3::from_plan(b, top + core_h),
                Vec3::from_plan(a, top + core_h),
            ],
            Vec3::new(out.x, 0.0, out.y),
            None,
            Some(paint),
        );
    }
    if roll < 0.4 {
        hip_roof(&core, top + core_h, id, builder);
    } else {
        builder.ground_uv("roof", &core_ring, top + core_h, None);
        parapet(&core_ring, top + core_h, 0.6, paint, builder);
        // The frame: posts on the roof edge and a beam grid over them, which is
        // the white lattice that ends nearly every 2000s tower in the country.
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
        // Cross beams over the roof, every 3 m along the long axis.
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
}

/// A hipped roof of grey or red clay tile over a rectangle.
pub(super) fn hip_roof(frame: &Frame, top: f32, id: u32, builder: &mut MeshBuilder) {
    let material = if hash_u32(id, 261, 5) < 0.6 {
        "roof.tile"
    } else {
        "roof.terracotta"
    };
    let eave = 0.6;
    let pitch = 0.50;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(w: f32, d: f32) -> Vec<Vec2> {
        vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, d),
            Vec2::new(w, d),
            Vec2::new(w, 0.0),
        ]
    }

    #[test]
    fn the_living_side_faces_the_sun() {
        let frame = Frame::fit(&rect(40.0, 12.0)).expect("a rectangle fits");
        assert!((-frame.eb).dot(SUN_PLAN) > 0.0);
        assert!((frame.len - 40.0).abs() < 1e-3 && (frame.depth - 12.0).abs() < 1e-3);
    }

    #[test]
    fn a_notched_plan_stays_inside_its_frame_and_keeps_most_of_its_area() {
        let frame = Frame::fit(&rect(26.0, 18.0)).unwrap();
        let (front, back) = notches_for(&frame, true, 7);
        assert_eq!(front.len(), 1);
        assert_eq!(back.len(), 2);
        let plan = notched_plan(&frame, &front, &back);
        assert!(signed_area(&plan) < 0.0, "plan must be clockwise");
        let area = signed_area(&plan).abs();
        assert!(area > 0.8 * 26.0 * 18.0 && area < 26.0 * 18.0);
        for p in &plan {
            assert!(p.x > -1e-3 && p.x < 26.0 + 1e-3 && p.y > -1e-3 && p.y < 18.0 + 1e-3);
        }
    }

    #[test]
    fn the_palette_is_pale() {
        for id in 0..200 {
            let c = paint_colour(id);
            let luma = 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
            assert!(
                (0.38..0.75).contains(&luma),
                "paint {c:?} too dark or too bright"
            );
        }
    }
}
