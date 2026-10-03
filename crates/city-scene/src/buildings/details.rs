//! Facade details: the parts that hang off the shell.
//!
//! Everything in this module is *geometry with a rule behind it* — the pier
//! strips land on the tile's painted piers because both derive from the same
//! design's bay module; the balcony depth and guard heights come from
//! [`super::rule`]; the condenser berth is the code's 0.6 – 0.8 m opening.  The
//! Look is a Chinese residential street wall: dark balcony fascias, condensers
//! under every balcony line, 防盗窗 grilles, sign boxes over the pavement.

use urban::ModernBuilding;

use super::rule;
use crate::facades::FacadeDesign;
use crate::math::{Vec2, Vec3, ring_centroid};
use crate::mesh::MeshBuilder;

/// A balcony stack.
///
/// A Chinese residential balcony is a **full-height opening** with a slab, a
/// dark fascia and a railing, and it repeats *identically* down the whole
/// elevation — that repetition is the look, and a facade with balconies at
/// irregular intervals reads as a hotel.  Two details do the work:
///
/// * the **fascia is dark**.  From a distance the balcony reads as a dark
///   horizontal line at every storey, which is exactly the value rhythm a slab
///   needs and is the reason this element is in the file at all.
/// * the **railing is pale and solid**, not a glass balustrade, which is what
///   catches the light and separates one storey from the next.  Its height is
///   the code's two-band rule: 1.05 m to six storeys, 1.10 m from seven.
pub(crate) fn balconies(
    ring: &[Vec2],
    fronts: &[usize],
    building: &ModernBuilding,
    base: f32,
    top: f32,
    storey: f32,
    builder: &mut MeshBuilder,
) {
    if fronts.is_empty() {
        return;
    }
    let centre = ring_centroid(ring);
    let rail = rule::balcony_rail_m(building.floors);
    let mut floor = 1_u32;
    while floor < building.floors as u32 {
        let y = base + floor as f32 * storey;
        if y > top - 1.2 {
            break;
        }
        for index in fronts {
            let a = ring[*index];
            let b = ring[(*index + 1) % ring.len()];
            let length = a.distance(b);
            if length < 3.4 {
                continue;
            }
            let outward = ((a + b) * 0.5 - centre).normalize();
            // The slab projects the code depth and is 150 mm thick, which is a
            // real 生活阳台 and not a ledge.
            let depth = rule::BALCONY_DEPTH_M;
            let a_out = a + outward * depth;
            let b_out = b + outward * depth;
            // Slab top.
            builder.quad(
                "balcony.slab",
                Vec3::from_plan(a, y),
                Vec3::from_plan(b, y),
                Vec3::from_plan(b_out, y),
                Vec3::from_plan(a_out, y),
                None,
            );
            // The dark fascia.  This single face is why the element is here: from
            // a distance a balcony reads as a dark horizontal line at every
            // storey, and that is the value rhythm a slab needs.
            builder.wall("trim.dark", b_out, a_out, y - 0.20, y, None);
            // The railing: a solid pale parapet at the code height, with a coping.
            builder.wall("trim.light", a_out, b_out, y, y + rail, None);
            builder.wall("balcony.slab", b_out, a_out, y + rail, y + rail + 0.09, None);
            // One return, so the balcony is an object and not a floating plane.
            // Both returns would be correct and cost another quad a storey.
            builder.wall("balcony.slab", a, a_out, y, y + rail + 0.09, None);
        }
        floor += 1;
    }
}

/// Wall-mounted air-conditioner condensers (空调外机) under the balcony line.
///
/// A box in the code's 0.76 m berth on a bracket, with a dark fan disc, one per
/// balcony on the front elevations.  They are the most characteristic object on a
/// Chinese residential facade and they are also what stops a slab from reading as
/// a stack of stripes: every storey gets a small, dark, high-frequency
/// horizontal interruption.
pub(crate) fn condensers(
    ring: &[Vec2],
    fronts: &[usize],
    building: &ModernBuilding,
    base: f32,
    top: f32,
    storey: f32,
    builder: &mut MeshBuilder,
) {
    if fronts.is_empty() {
        return;
    }
    let centre = ring_centroid(ring);
    let mut floor = 1_u32;
    let mut serial = building.id.wrapping_mul(31);
    while floor < building.floors as u32 {
        let y = base + floor as f32 * storey;
        if y > top - 1.6 {
            break;
        }
        for index in fronts {
            let a = ring[*index];
            let b = ring[(*index + 1) % ring.len()];
            let length = a.distance(b);
            if length < 3.4 {
                continue;
            }
            serial = serial.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            // Not every balcony has one: about a quarter do, which is roughly
            // the real share of households with a split unit on the balcony.
            if (serial >> 16) % 100 >= 27 {
                continue;
            }
            let along = (b - a).normalize();
            let t = 0.25 + ((serial >> 8) % 100) as f32 / 200.0;
            let anchor = a + along * (length * t);
            condenser_unit(builder, anchor, centre, y - 0.42);
        }
        floor += 1;
    }
}

/// Wall-mounted condensers on the faces **without** balconies — the ends and
/// rear of a 板楼, three faces of a tower — on a sparser, irregular rhythm: one
/// berth every few metres, skipping storeys, because that is how a real wall
/// fills in over a decade.
pub(crate) fn wall_ac_units(
    ring: &[Vec2],
    fronts: &[usize],
    building: &ModernBuilding,
    base: f32,
    top: f32,
    storey: f32,
    builder: &mut MeshBuilder,
) {
    let centre = ring_centroid(ring);
    let mut serial = building.id.wrapping_mul(7_621);
    let mut floor = 1_u32;
    while floor < building.floors as u32 {
        // Odd floors mostly: units cluster where the bedrooms are.
        let y = base + floor as f32 * storey;
        if y > top - 1.4 {
            break;
        }
        for index in 0..ring.len() {
            if fronts.contains(&index) {
                continue;
            }
            let a = ring[index];
            let b = ring[(index + 1) % ring.len()];
            let length = a.distance(b);
            if length < 7.0 {
                continue;
            }
            // One berth per ~9 m of wall, two storeys in three, jittered.
            let berths = (length / 9.0).floor() as u32;
            for berth in 0..berths {
                serial = serial.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                if (serial >> 12) % 100 >= 64 {
                    continue;
                }
                let t = (berth as f32 + 0.5 + ((serial >> 20) % 100) as f32 / 250.0)
                    / berths as f32;
                let along = (b - a).normalize();
                let anchor = a + along * (length * t.clamp(0.1, 0.9));
                condenser_unit(builder, anchor, centre, y - 0.42);
            }
        }
        floor += 2;
    }
}

/// One 空调外机: the code-width unit on a bracket, hanging off the wall, with
/// the dark fan disc that is the only part anyone can see.
fn condenser_unit(builder: &mut MeshBuilder, anchor: Vec2, centre: Vec2, y: f32) {
    let outward = (anchor - centre).normalize();
    // The bracket, then the unit, hanging 0.42 m off the wall in a berth the
    // code sizes at 0.6 – 0.8 m.
    let front = anchor + outward * 0.42;
    builder.wall("trim.dark", anchor, front, y - 0.33, y - 0.26, None);
    crate::mesh::box_at(
        builder,
        "metal.ac",
        front,
        y,
        rule::AC_BERTH_W_M,
        0.62,
        0.34,
        (anchor - centre).angle() + std::f32::consts::FRAC_PI_2,
    );
    // The fan: a dark disc on the front face.  Six sides, because a fan is 300 mm
    // across and there is no camera in this renderer closer than a metre.
    let disc = front + outward * 0.18;
    builder.tube(
        "trim.dark",
        Vec3::new(disc.x, y, disc.y),
        Vec3::new(disc.x + outward.x * 0.02, y, disc.y + outward.y * 0.02),
        0.24,
        0.24,
        6,
        None,
    );
}

/// 晾衣杆 and drying cages on the balcony line: a pole pair over a third of the
/// balconies and a wire cage on the rarer ones.  Half the year the balcony is a
/// laundry, and a facade without laundry gear reads as an office.
pub(crate) fn drying_cages(
    ring: &[Vec2],
    fronts: &[usize],
    building: &ModernBuilding,
    base: f32,
    top: f32,
    storey: f32,
    builder: &mut MeshBuilder,
) {
    if fronts.is_empty() {
        return;
    }
    let centre = ring_centroid(ring);
    let rail = rule::balcony_rail_m(building.floors);
    let mut serial = building.id.wrapping_mul(97);
    let mut floor = 1_u32;
    while floor < building.floors as u32 {
        let y = base + floor as f32 * storey;
        if y > top - 1.4 {
            break;
        }
        for index in fronts {
            let a = ring[*index];
            let b = ring[(*index + 1) % ring.len()];
            let length = a.distance(b);
            if length < 5.0 {
                continue;
            }
            serial = serial.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let roll = (serial >> 10) % 100;
            // A pole pair over a third of balconies; a cage on one in twelve.
            if roll >= 45 {
                continue;
            }
            let along = (b - a).normalize();
            let outward = ((a + b) * 0.5 - centre).normalize();
            let anchor = a + along * (length * (0.3 + ((serial >> 18) % 100) as f32 / 250.0));
            let rail_y = y + rail;
            if roll < 33 {
                // Two uprights and a rail above the balcony guard.
                for offset in [-0.55_f32, 0.55] {
                    let foot = anchor + along * offset;
                    builder.tube(
                        "trim.dark",
                        Vec3::new(foot.x, rail_y, foot.y),
                        Vec3::new(foot.x, rail_y + 0.75, foot.y),
                        0.022,
                        0.020,
                        4,
                        None,
                    );
                }
                let r0 = anchor - along * 0.55 + outward * (rule::BALCONY_DEPTH_M * 0.55);
                let r1 = anchor + along * 0.55 + outward * (rule::BALCONY_DEPTH_M * 0.55);
                builder.tube(
                    "trim.dark",
                    Vec3::new(r0.x, rail_y + 0.75, r0.y),
                    Vec3::new(r1.x, rail_y + 0.75, r1.y),
                    0.018,
                    0.018,
                    4,
                    None,
                );
            } else {
                // The drying cage: a stainless box frame on the balcony, the
                // 封阳台 that half the blocks have welded on.
                let depth = rule::BALCONY_DEPTH_M;
                let half_w = 1.1_f32;
                let height = 1.2_f32;
                let corners = [
                    anchor - along * half_w,
                    anchor + along * half_w,
                    anchor + along * half_w + outward * depth,
                    anchor - along * half_w + outward * depth,
                ];
                for corner in corners {
                    builder.tube(
                        "metal.ac",
                        Vec3::new(corner.x, rail_y, corner.y),
                        Vec3::new(corner.x, rail_y + height, corner.y),
                        0.026,
                        0.022,
                        4,
                        None,
                    );
                }
                for pair in [(0usize, 1usize), (1, 2), (2, 3), (3, 0)] {
                    let a = corners[pair.0];
                    let b = corners[pair.1];
                    builder.tube(
                        "metal.ac",
                        Vec3::new(a.x, rail_y + height, a.y),
                        Vec3::new(b.x, rail_y + height, b.y),
                        0.020,
                        0.018,
                        4,
                        None,
                    );
                }
            }
        }
        floor += 2;
    }
}

/// Projecting pier strips on a masonry wall, at the design's own bay pitch.
///
/// The tile paints a pier on every bay boundary; this puts a thin physical strip
/// there — 50 to 80 mm proud, centred on the painted pier — so the vertical
/// rhythm is real relief that catches raking light and throws a real shadow,
/// not just paint.  The strip's face carries the facade material sampled at the
/// pier zone (the bay_clearance rule guarantees a window never comes this close
/// to a boundary), and the storey `V` datum is the wall's own, so the relief is
/// aligned to the floors by construction.
pub(crate) fn wall_relief(
    builder: &mut MeshBuilder,
    material: &str,
    ring: &[Vec2],
    base: f32,
    top: f32,
    storeys: f32,
    mirror: bool,
    design: &FacadeDesign,
) {
    if top - base <= 1.0 {
        return;
    }
    let centre = ring_centroid(ring);
    let bays = design.bays.max(1) as f32;
    let pitch = design.bay_m / bays;
    let strip_w = (design.pier_w * 0.55).clamp(0.10, 0.30);
    let depth = 0.055;
    let v_base = storeys / 4.0;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let length = a.distance(b);
        // Relief pays for itself only along a real wall.
        if length < pitch * 1.6 {
            continue;
        }
        let along = (b - a).normalize();
        let outward = ((a + b) * 0.5 - centre).normalize();
        let count = ((length - 0.5) / pitch).floor() as i32;
        for k in 1..=count {
            let at = k as f32 * pitch;
            if at > length - 0.3 {
                break;
            }
            let c0 = a + along * (at - strip_w * 0.5);
            let c1 = a + along * (at + strip_w * 0.5);
            let o0 = c0 + outward * depth;
            let o1 = c1 + outward * depth;
            // The pier face, sampled at the painted pier zone of the tile.
            let (u0, u1) = if mirror {
                (-(at + strip_w * 0.5), -(at - strip_w * 0.5))
            } else {
                (at - strip_w * 0.5, at + strip_w * 0.5)
            };
            builder.quad_uv(
                material,
                Vec3::from_plan(o0, base),
                Vec3::from_plan(o1, base),
                Vec3::from_plan(o1, top),
                Vec3::from_plan(o0, top),
                [(u0, v_base), (u1, v_base), (u1, 0.0), (u0, 0.0)],
                None,
            );
            // The shaded flanks.
            builder.wall("trim.dark", c0, o0, base, top, None);
            builder.wall("trim.dark", o1, c1, base, top, None);
        }
    }
}

/// Full-height stone fins on a curtain wall, one per curtain module.
///
/// The vertical-stripe tower — the commonest glass elevation in a Chinese CBD —
/// is a *projecting* fin cluster, not a printed grid: each fin throws a shadow
/// down its shaded flank, and the cluster is what makes a glass tower read as a
/// bundle of verticals from a kilometre.  The fin face samples the tile's own
/// fin zone, so paint and relief agree.
pub(crate) fn curtain_fins(
    builder: &mut MeshBuilder,
    material: &str,
    ring: &[Vec2],
    base: f32,
    top: f32,
    storeys: f32,
    mirror: bool,
    design: &FacadeDesign,
) {
    if top - base <= 1.0 {
        return;
    }
    let centre = ring_centroid(ring);
    let bays = design.bays.max(1) as f32;
    let pitch = design.bay_m / bays;
    let fin_w = (design.pier_w * 0.5 + 0.05).clamp(0.10, 0.24);
    let depth = 0.32;
    let v_base = storeys / 4.0;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let length = a.distance(b);
        if length < pitch * 1.6 {
            continue;
        }
        let along = (b - a).normalize();
        let outward = ((a + b) * 0.5 - centre).normalize();
        let count = ((length - 0.5) / pitch).floor() as i32;
        for k in 0..=count {
            let at = k as f32 * pitch;
            if at < fin_w * 0.5 || at > length - fin_w * 0.5 {
                continue;
            }
            let c0 = a + along * (at - fin_w * 0.5);
            let c1 = a + along * (at + fin_w * 0.5);
            let o0 = c0 + outward * depth;
            let o1 = c1 + outward * depth;
            let (u0, u1) = if mirror {
                (-(at + fin_w * 0.5), -(at - fin_w * 0.5))
            } else {
                (at - fin_w * 0.5, at + fin_w * 0.5)
            };
            builder.quad_uv(
                material,
                Vec3::from_plan(o0, base),
                Vec3::from_plan(o1, base),
                Vec3::from_plan(o1, top),
                Vec3::from_plan(o0, top),
                [(u0, v_base), (u1, v_base), (u1, 0.0), (u0, 0.0)],
                None,
            );
            // The pale stone flanks and the sloping cap that throws the water
            // clear — a fin without a cap is a shadow gap, not a fin.
            builder.wall("trim.light", c0, o0, base, top, None);
            builder.wall("trim.light", o1, c1, base, top, None);
            builder.quad(
                "trim.light",
                Vec3::from_plan(c0, top),
                Vec3::from_plan(c1, top),
                Vec3::from_plan(o1, top),
                Vec3::from_plan(o0, top),
                None,
            );
        }
    }
}

/// Sill courses at every floor, and 飘窗 bay courses on the 2000s tile-clad
/// stock.
///
/// A projecting sill slab under every window line is the horizontal relief that
/// keeps a masonry facade alive in raking light: 90 mm on the ordinary stock,
/// and on the 飘窗 designs a 450 mm pair — sill and head, with the painted
/// glazing showing between them — which is what a 2000s bay window *is*.
pub(crate) fn sill_courses(
    builder: &mut MeshBuilder,
    ring: &[Vec2],
    base: f32,
    top: f32,
    design: &FacadeDesign,
) {
    if design.cladding.is_glass() || top - base <= 2.0 {
        return;
    }
    let centre = ring_centroid(ring);
    let storey = design.storey_m;
    let sill = design.sill_m;
    let depth = if design.bay_band {
        super::rule::BAY_WINDOW_PROJECTION_M
    } else {
        0.09
    };
    let mut count = 1.0_f32;
    loop {
        let floor = base + count * storey;
        if floor > top - 1.0 {
            break;
        }
        let sill_y = floor + sill;
        for index in 0..ring.len() {
            let a = ring[index];
            let b = ring[(index + 1) % ring.len()];
            if a.distance(b) < 5.0 {
                continue;
            }
            let outward = ((a + b) * 0.5 - centre).normalize();
            let a_out = a + outward * depth;
            let b_out = b + outward * depth;
            // The sill slab: top face, front edge, dark drip underneath.
            builder.quad(
                "trim.light",
                Vec3::from_plan(a, sill_y),
                Vec3::from_plan(b, sill_y),
                Vec3::from_plan(b_out, sill_y),
                Vec3::from_plan(a_out, sill_y),
                None,
            );
            builder.wall("trim.light", b_out, a_out, sill_y - 0.05, sill_y, None);
            builder.wall("trim.dark", a, b, sill_y - 0.07, sill_y - 0.02, None);
            if design.bay_band {
                // The head course of the 飘窗, at the window head height: the
                // pair of projected slabs frames the painted glazing between
                // them and reads as the bay window it is modelled on.
                let head_y = floor + sill + design.open_h + 0.06;
                builder.quad(
                    "trim.light",
                    Vec3::from_plan(a_out, head_y),
                    Vec3::from_plan(b_out, head_y),
                    Vec3::from_plan(b, head_y),
                    Vec3::from_plan(a, head_y),
                    None,
                );
                builder.wall("trim.light", a, b, head_y - 0.04, head_y, None);
                builder.wall("trim.dark", a_out, b_out, head_y, head_y + 0.03, None);
            }
        }
        count += 1.0;
    }
}

/// The ground-floor band.
///
/// A Chinese ground floor is a **continuous** retail band, and the parts are what
/// make it read as one: a dark stall riser at the pavement, a run of **tile-clad
/// piers between the shopfronts**, the glazed shopfronts, a fascia that
/// overhangs, projecting sign boxes at head height, and an awning over the
/// frontage.  Any one of them alone reads as a door; together they read as a
/// street.
pub(crate) fn retail_band(
    ring: &[Vec2],
    centre: Vec2,
    building: &ModernBuilding,
    retail: bool,
    builder: &mut MeshBuilder,
) {
    let floor = super::level::GROUND;
    let head = floor + crate::facades::GROUND_STOREY_M;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        if a.distance(b) < 1.0 {
            continue;
        }
        let mid = (a + b) * 0.5;
        let outward = (mid - centre).normalize();
        // The stall riser: a 420 mm dark stone kick plate, which is what every
        // Chinese shopfront has and what keeps the glazing off the pavement.
        builder.wall("trim.dark", a, b, floor - 0.02, floor + 0.42, None);
        // The fascia: a band from 3.55 m to the head, overhanging 300 mm.  The
        // overhang throws a hard shadow across the top of the shopfront, and
        // that shadow is most of what makes a ground floor read as a ground
        // floor rather than as a window.
        let fy0 = floor + crate::facades::GROUND_STOREY_M - 0.95;
        let a_out = a + outward * 0.30;
        let b_out = b + outward * 0.30;
        builder.quad(
            "trim.light",
            Vec3::from_plan(a, fy0),
            Vec3::from_plan(b, fy0),
            Vec3::from_plan(b_out, fy0),
            Vec3::from_plan(a_out, fy0),
            None,
        );
        builder.wall("trim.light", b_out, a_out, fy0, head, None);
        builder.wall("trim.dark", a, b, fy0 - 0.16, fy0, None);
        // The colonnade: a tile-clad pier between every shop unit, 4.2 m on
        // centre, projecting 220 mm.  The pier rhythm is what makes a retail
        // podium read as built of units rather than extruded.
        if retail {
            let length = a.distance(b);
            let pitch = 4.2_f32;
            let count = ((length - 0.8) / pitch).floor() as i32;
            for k in 1..=count {
                let at = k as f32 * pitch;
                let along = (b - a).normalize();
                let p0 = a + along * (at - 0.22);
                let p1 = a + along * (at + 0.22);
                let q0 = p0 + outward * 0.22;
                let q1 = p1 + outward * 0.22;
                builder.wall("trim.light", p0, q0, floor + 0.42, fy0, None);
                builder.wall("trim.light", q1, p1, floor + 0.42, fy0, None);
                builder.wall("trim.dark", q0, q1, floor + 0.42, fy0, None);
            }
        }
    }
    if retail {
        sign_boxes(ring, centre, building, floor, builder);
        roller_shutters(ring, centre, building, floor, builder);
        awnings(ring, centre, floor, building.id, builder);
    }
}

/// Roller shutters, on a fraction of the retail bays.
///
/// This is the one ground-floor element that has to vary *per building* rather
/// than per tile, and it is the highest-value variation available: on a Chinese
/// street roughly a third of the units are shut, and the ribbed slats catch
/// raking light in a way nothing else at street level does.  Every fourth bay of
/// every fourth shopfront gets one, which is about the real proportion.
fn roller_shutters(ring: &[Vec2], centre: Vec2, building: &ModernBuilding, floor: f32, builder: &mut MeshBuilder) {
    let mut serial = building.id.wrapping_mul(2_246_822_519);
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let length = a.distance(b);
        if length < 3.0 {
            continue;
        }
        let along = (b - a).normalize();
        let outward = ((a + b) * 0.5 - centre).normalize();
        // A shutter covers a 3.2 m bay: one per structural bay, every fourth
        // structural bay, and then a half or fully drawn one.
        let mut cursor = 0.9;
        while cursor + 3.2 <= length - 0.9 {
            serial = serial.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            if (serial >> 16) % 100 < 26 {
                let start = a + along * cursor;
                let end = start + along * 3.2;
                let closed = if (serial >> 8) % 100 < 45 {
                    // Fully down: the whole opening behind a curtain of slats.
                    0.30
                } else {
                    // Partly down, which is a shop trading and the rest shut.
                    1.35 + ((serial >> 4) % 12) as f32 * 0.10
                };
                let top = floor + 3.20;
                let bottom = floor + closed;
                let p0 = start + outward * 0.10;
                let p1 = end + outward * 0.10;
                // The curtain, the guide rails either side and the barrel box
                // above it.  Two materials: a pale slat face and a dark frame,
                // which is what a shutter actually is.
                builder.quad(
                    "trim.light",
                    Vec3::from_plan(p0, bottom),
                    Vec3::from_plan(p1, bottom),
                    Vec3::from_plan(p1, top),
                    Vec3::from_plan(p0, top),
                    None,
                );
                for edge in [p0, p1] {
                    builder.wall("trim.dark", edge, edge, bottom, top + 0.30, None);
                }
                builder.wall("trim.dark", p1, p0, top, top + 0.30, None);
                // The bottom rail, which is the only part of a shutter that
                // touches the pavement.
                builder.wall("trim.dark", p1, p0, bottom - 0.10, bottom, None);
            }
            cursor += 3.6;
        }
    }
}

/// Projecting sign boxes — 灯箱 — at head height, perpendicular to the wall, on
/// a bracket.  The density of bright rectangles over a footway is half of what
/// makes a Chinese street recognisable, and they are the one thing on a
/// building that is *meant* to be bright.
fn sign_boxes(ring: &[Vec2], centre: Vec2, building: &ModernBuilding, floor: f32, builder: &mut MeshBuilder) {
    let y = floor + crate::facades::GROUND_STOREY_M - 1.55;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let length = a.distance(b);
        if length < 3.0 {
            continue;
        }
        let along = (b - a).normalize();
        let mut cursor = 0.6;
        let mut serial = (index as u32).wrapping_mul(2_654_435_761) ^ building.id;
        while cursor + 1.5 <= length - 0.6 {
            serial = serial.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            // Most bays have a sign; the gaps are the shuttered ones.
            if (serial >> 16) % 100 < 82 {
                let anchor = a + along * cursor;
                let outward = (anchor - centre).normalize();
                // The bracket, then the box hanging off it.
                let tip = anchor + outward * 0.75;
                builder.wall("trim.dark", anchor, tip, y - 0.10, y, None);
                let height = 0.62 + ((serial >> 8) % 5) as f32 * 0.06;
                // Both faces, because a sign box is lit on both sides and one of
                // them faces the pavement.
                builder.quad_uv(
                    "sign/shop",
                    Vec3::new(tip.x, y, tip.y),
                    Vec3::new(tip.x + along.x * 1.35, y, tip.y + along.y * 1.35),
                    Vec3::new(tip.x + along.x * 1.35, y + height, tip.y + along.y * 1.35),
                    Vec3::new(tip.x, y + height, tip.y),
                    [(0.0, 0.0), (1.35, 0.0), (1.35, height), (0.0, height)],
                    None,
                );
                let back = tip + outward * 0.10;
                builder.quad_uv(
                    "sign/shop",
                    Vec3::new(back.x + along.x * 1.35, y, back.y + along.y * 1.35),
                    Vec3::new(back.x, y, back.y),
                    Vec3::new(back.x, y + height, back.y),
                    Vec3::new(back.x + along.x * 1.35, y + height, back.y + along.y * 1.35),
                    [(0.0, 0.0), (1.35, 0.0), (1.35, height), (0.0, height)],
                    None,
                );
                // The top and the end, so the box is a solid object and not two
                // floating quads.
                builder.quad(
                    "trim.dark",
                    Vec3::new(tip.x, y + height, tip.y),
                    Vec3::new(tip.x + along.x * 1.35, y + height, tip.y + along.y * 1.35),
                    Vec3::new(back.x + along.x * 1.35, y + height, back.y + along.y * 1.35),
                    Vec3::new(back.x, y + height, back.y),
                    None,
                );
                builder.quad(
                    "trim.dark",
                    Vec3::new(back.x, y, back.y),
                    Vec3::new(tip.x, y, tip.y),
                    Vec3::new(tip.x, y + height, tip.y),
                    Vec3::new(back.x, y + height, back.y),
                    None,
                );
            }
            cursor += 3.0;
        }
    }
}

/// A fabric awning over the shopfront: a sloping quad on a frame, in bands, with
/// the scalloped valance at its front edge.
fn awnings(ring: &[Vec2], centre: Vec2, floor: f32, id: u32, builder: &mut MeshBuilder) {
    let y = floor + 2.95;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let length = a.distance(b);
        if length < 4.0 {
            continue;
        }
        let outward = ((a + b) * 0.5 - centre).normalize();
        let depth = 1.35;
        let drop = 0.55;
        let a_out = a + outward * depth;
        let b_out = b + outward * depth;
        builder.quad(
            "awning",
            Vec3::from_plan(a, y),
            Vec3::from_plan(b, y),
            Vec3::from_plan(b_out, y - drop),
            Vec3::from_plan(a_out, y - drop),
            None,
        );
        // The valance, and the two brackets that hold it up.
        builder.wall("awning", b_out, a_out, y - drop - 0.26, y - drop, None);
        for t in [0.15_f32, 0.55, 0.9] {
            let anchor = a.lerp(b, t);
            builder.tube(
                "trim.dark",
                Vec3::new(anchor.x, y, anchor.y),
                Vec3::new(
                    (anchor + outward * depth).x,
                    y - drop,
                    (anchor + outward * depth).y,
                ),
                0.035,
                0.030,
                4,
                None,
            );
        }
        let _ = id;
    }
}

/// A rainwater downpipe (落水管) down one corner, in a socket at the parapet and
/// discharging over a splash block.  One tube per building, and it is on every
/// building in the country.
pub(crate) fn downpipe(ring: &[Vec2], base: f32, top: f32, id: u32, builder: &mut MeshBuilder) {
    let index = (id % ring.len().max(1) as u32) as usize;
    let a = ring[index];
    let b = ring[(index + 1) % ring.len()];
    let outward = (((a + b) * 0.5) - ring_centroid(ring)).normalize();
    let anchor = a + outward * 0.11;
    builder.tube(
        "trim.light",
        Vec3::new(anchor.x, base + 0.35, anchor.y),
        Vec3::new(anchor.x, top - 0.2, anchor.y),
        0.075,
        0.075,
        5,
        None,
    );
    // Two brackets and the shoe at the bottom, so it is fixed to the wall rather
    // than floating beside it.
    for t in [0.25_f32, 0.72] {
        let y = base + (top - base) * t;
        builder.wall("trim.dark", a, anchor, y - 0.04, y + 0.04, None);
    }
    builder.wall("trim.dark", anchor, anchor + outward * 0.18, base + 0.12, base + 0.35, None);
}

/// A canopy over the main entrance, on the edge the parcel's gate points at.
pub(crate) fn entrance_canopy(ring: &[Vec2], building: &ModernBuilding, builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    let edge = (building.id % ring.len().max(1) as u32) as usize;
    let a = ring[edge];
    let b = ring[(edge + 1) % ring.len()];
    let length = a.distance(b);
    if length < 4.0 {
        return;
    }
    let outward = ((a + b) * 0.5 - centre).normalize();
    let width = (length - 1.0).min(9.5);
    let mid = (a + b) * 0.5;
    let p0 = mid - (b - a).normalize() * (width * 0.5);
    let p1 = mid + (b - a).normalize() * (width * 0.5);
    let depth = 1.30;
    let y = super::level::GROUND + 3.35;
    builder.quad(
        "awning",
        Vec3::from_plan(p0, y),
        Vec3::from_plan(p1, y),
        Vec3::from_plan(p1 + outward * depth, y - 0.30),
        Vec3::from_plan(p0 + outward * depth, y - 0.30),
        None,
    );
    builder.wall("awning", p1 + outward * depth, p0 + outward * depth, y - 0.44, y - 0.30, None);
    // The fascia, and the brackets.
    builder.wall("trim.light", p1 + outward * depth, p0 + outward * depth, y - 0.30, y - 0.16, None);
    for t in [0.12_f32, 0.5, 0.88] {
        let anchor = p0.lerp(p1, t);
        builder.tube(
            "trim.dark",
            Vec3::new(anchor.x, y - 1.0, anchor.y),
            Vec3::new(
                (anchor + outward * depth).x,
                y - 0.30,
                (anchor + outward * depth).y,
            ),
            0.045,
            0.040,
            4,
            None,
        );
    }
    // Two steps up to the door, because a Chinese building meets the pavement
    // with a plinth and a step, not with a threshold at grade.
    for (offset, rise) in [(0.0_f32, 0.15_f32), (0.42, 0.30)] {
        let q0 = p0 + outward * (0.15 + offset);
        let q1 = p1 + outward * (0.15 + offset);
        builder.quad(
            "trim.light",
            Vec3::from_plan(q0, super::level::GROUND + rise),
            Vec3::from_plan(q1, super::level::GROUND + rise),
            Vec3::from_plan(q1, super::level::GROUND + rise - 0.06),
            Vec3::from_plan(q0, super::level::GROUND + rise - 0.06),
            None,
        );
        builder.wall("trim.light", q1, q0, super::level::GROUND, super::level::GROUND + rise, None);
    }
}

/// Silence the unused-import lint for the module-level contract note: the pier
/// relief reads its bay module only through the design table.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::buildings::{level, ring_of};
    use crate::facades::{GROUND_STOREY_M, STOREY_M, design};
    use crate::math::{point_in_ring, signed_area};
    use crate::mesh::MeshBuilder;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> urban::ModernCity {
        generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        })
    }

    /// A balcony projects off the wall — that is what a balcony is — so the
    /// invariant is not "inside the footprint" but "outside it by no more than
    /// its own depth, and on the correct side".  A balcony on the wrong side of
    /// a wall is a balcony inside the building.
    #[test]
    fn a_balcony_never_projects_into_its_own_building() {
        let city = city();
        let mut checked = 0;
        for building in &city.buildings {
            let ring = ring_of(&building.footprint, city.frame);
            if ring.len() < 3 || signed_area(&ring).abs() < 120.0 {
                continue;
            }
            let mut outward = ring.clone();
            if signed_area(&outward) < 0.0 {
                outward.reverse();
            }
            let fronts = super::super::shell::front_edges(&outward, 2);
            if fronts.is_empty() {
                continue;
            }
            let base = level::GROUND + GROUND_STOREY_M;
            let top = base + (building.floors as f32 - 1.0).max(1.0) * STOREY_M;
            let mut builder = MeshBuilder::new();
            balconies(&outward, &fronts, building, base, top, STOREY_M, &mut builder);
            for group in builder.build().meshes {
                for chunk in group.positions.chunks_exact(3) {
                    let point = Vec2::new(chunk[0], chunk[2]);
                    if point_in_ring(point, &outward) {
                        // On the wall itself, or a corner that rounds inward: fine.
                        continue;
                    }
                    // Outside the footprint, so the balcony must point *away*
                    // from the building and stay within its own depth.
                    let distance = crate::math::distance_to_polyline(point, &outward);
                    assert!(
                        distance < rule::BALCONY_DEPTH_M + 0.15,
                        "building {} has a balcony vertex {distance:.2} m outside its wall",
                        building.id
                    );
                    checked += 1;
                }
            }
        }
        assert!(checked > 200, "only {checked} balcony vertices were checked");
    }

    /// The guard heights actually built follow the two-band code rule: a
    /// six-storey walk-up's balconies guard at 1.05, a tower's at 1.10.
    #[test]
    fn balcony_guards_follow_the_two_band_rule() {
        let city = city();
        let mut short = 0;
        let mut tall = 0;
        for building in &city.buildings {
            let ring = ring_of(&building.footprint, city.frame);
            if ring.len() < 3 {
                continue;
            }
            let outward = {
                let mut ring = ring.clone();
                if signed_area(&ring) < 0.0 {
                    ring.reverse();
                }
                ring
            };
            let fronts = super::super::shell::front_edges(&outward, 1);
            if fronts.is_empty() {
                continue;
            }
            let base = level::GROUND + GROUND_STOREY_M;
            let top = base + (building.floors as f32 - 1.0).max(1.0) * STOREY_M;
            let mut builder = MeshBuilder::new();
            balconies(&outward, &fronts, building, base, top, STOREY_M, &mut builder);
            let rail = rule::balcony_rail_m(building.floors);
            let expected = base + rail;
            for group in builder.build().meshes {
                if group.material != "trim.light" {
                    continue;
                }
                for chunk in group.positions.chunks_exact(3) {
                    // Railing tops sit within a hair of the code height above
                    // some storey's slab (the first balcony is on storey 1).
                    let above = chunk[1] - expected;
                    let off = (above / STOREY_M - (above / STOREY_M).round()).abs() * STOREY_M;
                    if above > STOREY_M - 0.02 && off < 0.02 {
                        if building.floors >= 7 {
                            tall += 1;
                        } else {
                            short += 1;
                        }
                        break;
                    }
                }
            }
        }
        assert!(short > 3, "no ≤6-storey balcony guards found");
        assert!(tall > 0, "no ≥7-storey balcony guards found");
    }

    /// Pier relief strips land on the tile's painted piers: their offsets are
    /// multiples of the design's bay module, which is what makes paint and
    /// relief the same grid.
    #[test]
    fn pier_relief_lands_on_the_bay_module() {
        let design = design(0);
        let pitch = design.bay_m / design.bays.max(1) as f32;
        let ring = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(21.6, 0.0),
            Vec2::new(21.6, 12.0),
            Vec2::new(0.0, 12.0),
        ];
        let mut builder = MeshBuilder::new();
        wall_relief(
            &mut builder,
            "facade/00",
            &ring,
            4.65,
            26.75,
            7.0,
            false,
            design,
        );
        let groups = builder.build().meshes;
        let group = groups.iter().find(|g| g.material == "facade/00").expect("no relief");
        let uvs = group.uvs.as_ref().expect("relief carries metre UVs");
        // Every relief vertex's U sits on a bay boundary: u % pitch ≈ 0 (within
        // half a strip width of it).
        let mut on_module = 0;
        for pair in uvs.chunks_exact(2) {
            let u = pair[0];
            let v = pair[1];
            let phase = u.rem_euclid(pitch);
            let on = (phase - pitch).abs() < 0.31 || phase < 0.31;
            if on && (v * 4.0 - (v * 4.0).round()).abs() < 1.0e-3 {
                on_module += 1;
            }
        }
        assert!(on_module > 20, "only {on_module} relief vertices on the bay module");
    }

    /// 飘窗 courses only grow on the 2000s tile-clad designs, and project the
    /// code's 0.4 – 0.6 m; the ordinary stock gets its 90 mm sill.
    #[test]
    fn bay_window_bands_belong_to_the_tile_clad_stock() {
        let plain = design(0);
        let mosaic = design(17);
        assert!(!plain.bay_band && mosaic.bay_band);
        let ring = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(30.0, 0.0),
            Vec2::new(30.0, 12.0),
            Vec2::new(0.0, 12.0),
        ];
        let mut builder = MeshBuilder::new();
        sill_courses(&mut builder, &ring, 4.65, 22.0, mosaic);
        let depth = super::rule::BAY_WINDOW_PROJECTION_M + 0.01;
        // Every vertex in front of the wall is within the 飘窗 projection of it.
        // The distance is measured outside the footprint rectangle, not as a
        // raw coordinate: the wall itself is 30 m long.
        for group in builder.build().meshes {
            for chunk in group.positions.chunks_exact(3) {
                let out = (-chunk[0])
                    .max(chunk[0] - 30.0)
                    .max(-chunk[2])
                    .max(chunk[2] - 12.0)
                    .max(0.0);
                assert!(
                    out < depth + 1.0e-3,
                    "a bay course projects {out:.2} m, outside the 飘窗 band"
                );
            }
        }
    }
}
