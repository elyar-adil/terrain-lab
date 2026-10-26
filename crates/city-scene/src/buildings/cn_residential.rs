//! The Chinese residential block — 板楼, 点式塔楼, 多层 and the urban village —
//! built from a development's [`Design`] in the facade grammar.
//!
//! Every block of a development (a compound, or a street block's plot) shares
//! one design, as the blocks of a real 小区 do; the next development draws a
//! new one from its own building culture.  Within that:
//!
//! * **The plan faces the sun.**  The long side nearest the sun's azimuth is the
//!   living side (balconies, bedrooms, bay windows), the opposite one the
//!   service side (kitchens, bathrooms, the stair and the unit entrances), the
//!   ends are gables.
//! * **The plan is notched**, by culture: light wells and a central recess on a
//!   tower, light wells at the party walls of a long slab, staggered flats on
//!   some slabs, cut corners on some towers.  Those are the deep vertical
//!   shadows that make a block read as built rather than extruded.
//! * **Everything that projects, projects at the span the elevation draws**:
//!   balconies (solid, glass, bars or glazed in), bay windows, louvred AC
//!   platforms and window cages are built per storey at exactly those spans,
//!   and their outer faces are drawn by the same procedural facade, so the
//!   pane on a bay window is the pane the wall behind it would have shown.
//! * **The colour is the building's**: the facade draws a near-white skin and
//!   the wall's vertex colour carries the development's render colour, banded
//!   三段式 into base, body and top.
//! * **The top is the culture's crown** (see `cn_crowns`).

use urban::{ModernBuilding, ParcelUse};

use super::cn_crowns;
use super::cn_kit::{
    Frame, Notch, PAINT, RAIL_GLASS, SUN_PLAN, TRIM, WallMap, edge_out, facade_wall, face,
    ground_shops, hip_roof, hung_condenser, notched_plan, parapet, projection, roof_clay,
    wall_tint,
};
use super::{hash_u32, level, roofscape};
use crate::facades::GROUND_STOREY_M;
use crate::facades::cn_grammar::{
    BAY_SILL_M, Crown, Design, Elevation, Era, FACADE, FacadeTable, LOUVRE_TOP_M, Part, Rail, Side,
    bay_head, door_head,
};
use crate::math::{Rng, Vec2, Vec3, inset_ring, ring_centroid, signed_area};
use crate::mesh::{MeshBuilder, box_at};

/// Whether this module builds a building.
pub(crate) fn applies(building: &ModernBuilding) -> bool {
    match building.use_type {
        ParcelUse::Residential => building.floors >= 3,
        // Shops with flats over them, however low.
        ParcelUse::MixedUse => building.floors >= 2,
        // The self-built houses of a 城中村.
        ParcelUse::Village => true,
        _ => false,
    }
}

/// The development a building belongs to: its plot.  A compound's blocks all
/// stand on the compound's parcel, so they share a design.
fn development_seed(building: &ModernBuilding) -> u32 {
    building.parcel_id.wrapping_mul(0x9e37_79b1) ^ 0x5bd1_e995
}

/// Whether a block is a point tower.
fn is_tower(building: &ModernBuilding) -> bool {
    building.floors >= 19
}

/// The design of a building's development, and its table rows.
fn design_of(building: &ModernBuilding, table: &mut FacadeTable) -> (Design, [u8; 3]) {
    // Every village house was built by its own family, but the elevation table
    // has 256 rows for a whole tile: a village draws from ten designs, and its
    // houses differ by those, their widths and their heights.
    if building.use_type == ParcelUse::Village {
        let variant = building.id.wrapping_mul(0x9e37_79b1) >> 28;
        let seed = development_seed(building) ^ (variant % 10).wrapping_mul(0x2c1b_3c6d);
        return table.village(seed, building.floors);
    }
    table.residential(
        development_seed(building),
        building.floors,
        is_tower(building),
    )
}

/// Whether the ground floor is a shop: a mixed-use block, or a village house on
/// a wide lane or the village's edge (the plan gives it a second entrance).
fn shop_storey(building: &ModernBuilding) -> bool {
    building.use_type == ParcelUse::MixedUse
        || (building.use_type == ParcelUse::Village && building.entrance_count >= 2)
}

/// Wall colours of village houses: linear albedos measured from photographs of
/// Shenzhen and Guangzhou villages, by clustering the lit wall pixels (k-means on
/// an oblique Shenzhen view; greys from an overcast Guangzhou one), weighted by
/// the share each cluster covers. One tile colour up the whole house, unrelated
/// to the next house's: about 70 % warm pastels, 30 % grey and off-white, a few
/// per cent blue, teal and mint.
const VILLAGE_PAINTS: [([f32; 3], f32); 14] = [
    ([0.62, 0.46, 0.36], 11.7),
    ([0.53, 0.34, 0.27], 11.2),
    ([0.41, 0.27, 0.23], 9.9),
    ([0.68, 0.56, 0.46], 9.5),
    ([0.36, 0.19, 0.17], 7.6),
    ([0.72, 0.67, 0.60], 7.5),
    ([0.55, 0.53, 0.50], 10.0),
    ([0.36, 0.34, 0.32], 7.0),
    ([0.75, 0.74, 0.71], 6.0),
    ([0.27, 0.14, 0.12], 4.9),
    ([0.53, 0.25, 0.18], 3.9),
    ([0.15, 0.25, 0.30], 2.4),
    ([0.41, 0.50, 0.55], 2.1),
    ([0.40, 0.55, 0.45], 1.5),
];

fn village_paint(id: u32) -> [f32; 3] {
    let total: f32 = VILLAGE_PAINTS.iter().map(|(_, w)| w).sum();
    let mut r = hash_u32(id, 211, 7) * total;
    for (paint, weight) in VILLAGE_PAINTS {
        if r < weight {
            return paint;
        }
        r -= weight;
    }
    VILLAGE_PAINTS[0].0
}

/// Where the elevations start and how many storeys they carry.  A mixed-use
/// block stands on a 4.5 m shop storey; a compound block on a raised ground
/// floor (首层抬高 0.45 m) behind a stone plinth; a block on another building's
/// podium starts on its roof.
fn storeys_of(building: &ModernBuilding, podium: Option<(f32, u16)>) -> (f32, u32) {
    match podium {
        Some((top, floors)) => (top, (building.floors.saturating_sub(floors) as u32).max(3)),
        None if shop_storey(building) => (
            level::GROUND + GROUND_STOREY_M,
            (building.floors as u32).saturating_sub(1).max(1),
        ),
        None => (level::GROUND + 0.45, (building.floors as u32).max(1)),
    }
}

/// The roof deck height of a block this module builds on the ground.
pub(crate) fn roof_height(building: &ModernBuilding, table: &mut FacadeTable) -> f32 {
    let (design, _) = design_of(building, table);
    let (base, storeys) = storeys_of(building, None);
    base + storeys as f32 * design.front.storey_m
}

/// How a block's plan is notched, by its culture.
fn notches_for(frame: &Frame, tower: bool, era: Era, id: u32) -> (Vec<Notch>, Vec<Notch>) {
    let (l, d) = (frame.len, frame.depth);
    let mut rng = Rng::new(id ^ 0x0c1f_2a57);
    let mut front = Vec::new();
    let mut back = Vec::new();
    if d < 9.0 || era == Era::Village {
        return (front, back);
    }
    if tower {
        if l >= 20.0 && rng.chance(0.75) {
            let w = rng.range(2.2, 3.2);
            front.push(Notch {
                x0: l * 0.5 - w * 0.5,
                x1: l * 0.5 + w * 0.5,
                depth: rng.range(1.4, 2.6).min(d * 0.2),
            });
        }
        if l >= 18.0 && rng.chance(0.85) {
            let w = rng.range(2.0, 2.6);
            let depth = rng.range(3.0, 4.8).min(d * 0.34);
            let t = rng.range(0.26, 0.34);
            for at in [t, 1.0 - t] {
                back.push(Notch {
                    x0: l * at - w * 0.5,
                    x1: l * at + w * 0.5,
                    depth,
                });
            }
        }
        // Cut corners on the living side: the stepped, cross-like towers.
        if rng.chance(0.35) && front.len() <= 1 {
            let c = rng.range(1.8, 3.0).min(l * 0.15);
            let depth = rng.range(1.5, 2.6).min(d * 0.2);
            front.insert(
                0,
                Notch {
                    x0: 0.0,
                    x1: c,
                    depth,
                },
            );
            front.push(Notch {
                x0: l - c,
                x1: l,
                depth,
            });
            front.sort_by(|a, b| a.x0.total_cmp(&b.x0));
        }
    } else if l >= 34.0 {
        let flats = (l / 12.0).round().max(2.0);
        let pitch = l / flats;
        if rng.chance(0.6) {
            // A light well at every other party wall.
            let mut k = 1.0;
            while k < flats {
                let x = k * pitch;
                back.push(Notch {
                    x0: x - 1.0,
                    x1: x + 1.0,
                    depth: 2.2_f32.min(d * 0.25),
                });
                k += 2.0;
            }
        }
        if rng.chance(0.3) && matches!(era, Era::Modern | Era::Euro | Era::Tile) {
            // Staggered flats: every other unit steps back from the street.
            let mut k = 1.0;
            while k < flats {
                front.push(Notch {
                    x0: (k * pitch + 0.05).min(l - 0.1),
                    x1: ((k + 1.0) * pitch - 0.05).min(l - 0.05),
                    depth: 1.2_f32.min(d * 0.15),
                });
                k += 2.0;
            }
        }
    }
    (front, back)
}

/// Everything a building needs to dress its walls.
struct Ctx<'a> {
    id: u32,
    design: &'a Design,
    /// A colour that replaces the design's, top to bottom: a village house.
    paint: Option<[f32; 3]>,
    base: f32,
    storeys: u32,
}

impl Ctx<'_> {
    /// The render colour of storey `s` (0 = lowest): 三段式.
    fn paint_at(&self, s: u32) -> [f32; 3] {
        if let Some(paint) = self.paint {
            return paint;
        }
        let (base, base_n) = self.design.base;
        let (top, top_n) = self.design.top;
        if s < base_n && self.storeys > base_n + 2 {
            base
        } else if s + top_n >= self.storeys && self.storeys > top_n + 2 {
            top
        } else {
            self.design.paint
        }
    }
    fn tint_at(&self, s: u32) -> [f32; 3] {
        wall_tint(self.paint_at(s))
    }
    /// The storey boundaries where the colour changes.
    fn bands(&self) -> [u32; 4] {
        let n = self.storeys;
        let (_, base_n) = self.design.base;
        let (_, top_n) = self.design.top;
        let b = if n > base_n + 2 { base_n } else { 0 };
        let t = if n > top_n + 2 { n - top_n } else { n };
        [0, b.min(n), t.max(b.min(n)), n]
    }
}

/// Build one Chinese residential block.  `podium` is the roof height and
/// storey count of a larger, lower building it stands on, if any.  Returns the
/// roof deck height.
pub(crate) fn shell(
    building: &ModernBuilding,
    ring: &[Vec2],
    podium: Option<(f32, u16)>,
    table: &mut FacadeTable,
    builder: &mut MeshBuilder,
) -> f32 {
    let id = building.id;
    let on_ground = podium.is_none();
    let retail = on_ground && shop_storey(building);
    let village = building.use_type == ParcelUse::Village;
    let tower = is_tower(building);
    let (design, rows) = design_of(building, table);
    let storey = design.front.storey_m;

    // Plan: inset so balconies stay inside the plot, then notched.
    let mut outward = ring.to_vec();
    if signed_area(&outward) > 0.0 {
        outward.reverse();
    }
    // A village house stands nearly against its neighbours (握手楼): only a
    // slit, so it is barely inset.
    let inset = inset_ring(&outward, if village { 0.15 } else { 0.6 });
    let frame = Frame::fit(&inset);
    let plan = match &frame {
        Some(frame) => {
            let (front, back) = notches_for(frame, tower, design.era, id);
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
    let top = base + storeys as f32 * storey;
    let ctx = Ctx {
        id,
        design: &design,
        paint: village.then(|| village_paint(id)),
        base,
        storeys,
    };

    if on_ground {
        for index in 0..plan.len() {
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
    }
    if retail {
        ground_shops(&plan, building, builder);
    }

    // The elevations.
    let mut u_run = [0.0_f32; 2];
    for index in 0..plan.len() {
        let a = plan[index];
        let b = plan[(index + 1) % plan.len()];
        let length = a.distance(b);
        if length < 0.3 {
            continue;
        }
        let along = (b - a) / length;
        let out = edge_out(a, b, centre);
        let facing = match &frame {
            Some(frame) => out.dot(-frame.eb),
            None => out.dot(SUN_PLAN),
        };
        let side = if facing > 0.7 {
            Side::Front
        } else if facing < -0.7 || tower {
            // A tower has windows all round, so its flanks take the service
            // elevation; a slab's flanks are gables.
            Side::Back
        } else {
            Side::Gable
        };
        let (mut elevation, mut row) = match side {
            Side::Front => (&design.front, rows[0]),
            Side::Back => (&design.back, rows[1]),
            Side::Gable => (&design.gable, rows[2]),
        };
        if side != Side::Gable && length < 0.7 * elevation.width {
            elevation = &design.gable;
            row = rows[2];
        }
        let map = if elevation.side == Side::Gable {
            // Unstretched and centred on the gable's window, so a short end
            // wall shows one window in its middle rather than a cut one.
            let centre_u = elevation
                .spans
                .iter()
                .find(|span| matches!(span.part, Part::Window { .. }))
                .map_or(elevation.width * 0.5, |span| (span.x0 + span.x1) * 0.5);
            WallMap {
                a,
                along,
                out,
                u0: centre_u - length * 0.5,
                u_per_m: 1.0,
                units: 0,
                storey_m: storey,
            }
        } else {
            let slot = (elevation.side == Side::Back) as usize;
            let units = (length / elevation.width).round().max(1.0) as u32;
            let u0 = u_run[slot];
            u_run[slot] += units as f32 * elevation.width;
            WallMap {
                a,
                along,
                out,
                u0,
                u_per_m: units as f32 * elevation.width / length,
                units,
                storey_m: storey,
            }
        };
        let bands = ctx.bands();
        for pair in bands.windows(2) {
            facade_wall(
                builder,
                &map,
                b,
                base,
                pair[0],
                pair[1],
                row,
                ctx.tint_at(pair[0]),
            );
        }
        builder.facade_row = Some(row);
        if map.units > 0 {
            dress_wall(&ctx, elevation, &map, builder);
        } else if length > 4.0 {
            gable_units(&ctx, &map, length, builder);
        }
        builder.facade_row = None;
        if elevation.side == Side::Back && on_ground && !retail {
            entrances(&ctx, elevation, &map, tower, builder);
        }
    }

    // The roof and the crown.
    builder.ground_uv("roof", &plan, top, None);
    let crown = design.crown;
    let top_paint = ctx.paint_at(storeys.saturating_sub(1));
    match (crown, &frame) {
        (Crown::Hip, Some(frame)) if !tower => {
            hip_roof(frame, top, roof_clay(design.terracotta), 0.6, 0.5, builder);
        }
        (Crown::Mansard, _) => {
            cn_crowns::mansard(&plan, frame.as_ref(), top, id, builder);
        }
        (Crown::NewChinese, Some(frame)) if !tower && building.floors <= 7 => {
            let (len, depth) = (frame.len, frame.depth);
            let wide = Frame {
                o: frame.o - frame.ea * 0.2 - frame.eb * 0.2,
                ea: frame.ea,
                eb: frame.eb,
                len: len + 0.4,
                depth: depth + 0.4,
            };
            hip_roof(&wide, top, "roof.tile", 1.0, 0.42, builder);
        }
        _ => {
            parapet(&plan, top, 1.2, top_paint, builder);
            if signed_area(&plan).abs() > 90.0 {
                roofscape::roofscape(building, true, &plan, top, builder);
            }
            if let Some(frame) = &frame {
                match crown {
                    Crown::Frame if building.floors >= 9 => {
                        cn_crowns::frame_crown(&plan, frame, top, id, top_paint, builder);
                    }
                    Crown::Hip => {
                        cn_crowns::hipped_core(frame, top, design.terracotta, top_paint, builder);
                    }
                    Crown::NewChinese => {
                        cn_crowns::new_chinese(&plan, Some(frame), top, top_paint, builder);
                    }
                    _ => {}
                }
                if design.solar {
                    cn_crowns::solar_heaters(frame, top, id, SUN_PLAN, builder);
                }
            }
        }
    }
    let foot = if on_ground { level::GROUND } else { base };
    super::details::downpipe(&plan, foot, top, id, builder);
    top
}

/// The projections a wall's elevation calls for, storey by storey.
fn dress_wall(ctx: &Ctx, elevation: &Elevation, map: &WallMap, builder: &mut MeshBuilder) {
    let w = elevation.width;
    for k in 0..map.units {
        let tile_u = map.u0 + k as f32 * w;
        let mirrored = k % 2 == 1;
        for (span_index, span) in elevation.spans.iter().enumerate() {
            let (x0, x1) = if mirrored {
                (w - span.x1, w - span.x0)
            } else {
                (span.x0, span.x1)
            };
            let (ua, ub) = (tile_u + x0, tile_u + x1);
            let p0 = map.point(ua);
            let p1 = map.point(ub);
            match span.part {
                Part::Balcony { rail } => {
                    balcony_stack(ctx, map, rail, p0, p1, ua, ub, builder);
                }
                Part::Bay { .. } => bay_stack(ctx, map, p0, p1, ua, ub, builder),
                Part::AcNiche => louvre_stack(ctx, map, p0, p1, ua, ub, builder),
                Part::Window { sill, head, .. } if ctx.design.cages => {
                    cage_stack(ctx, elevation, map, sill, head, p0, p1, ua, ub, builder);
                }
                Part::Wall if span.x1 - span.x0 >= 0.55 && ctx.design.hung_ac > 0.0 => {
                    // Condensers hung on the wall between windows, wherever
                    // the tenant drilled.
                    for s in 0..ctx.storeys {
                        let roll = hash_u32(ctx.id ^ (k << 8), span_index as i32, s as i32);
                        if roll < ctx.design.hung_ac * 0.6 {
                            let y = ctx.base + s as f32 * map.storey_m + 0.35;
                            hung_condenser(map, (ua + ub) * 0.5, y, builder);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn balcony_stack(
    ctx: &Ctx,
    map: &WallMap,
    rail: Rail,
    p0: Vec2,
    p1: Vec2,
    ua: f32,
    ub: f32,
    builder: &mut MeshBuilder,
) {
    let depth = ctx.design.balcony_depth;
    let storey = map.storey_m;
    let out = map.out;
    let q0 = p0 + out * depth;
    let q1 = p1 + out * depth;
    let along = (p1 - p0).normalize();
    let angle = along.y.atan2(along.x);
    let (da, db) = map.shrink(ua, ub, 0.30);
    let head = door_head(storey);
    let rail_h = if ctx.storeys >= 7 { 1.10 } else { 1.05 };
    for s in 0..ctx.storeys {
        let y = ctx.base + s as f32 * storey;
        let sf = s as f32;
        // The floor slab, with its white edge.
        projection(builder, TRIM, None, p0, p1, out, depth, y - 0.14, y, None);
        let (solid_mat, solid_colour) = if ctx.design.trim_parapet {
            (TRIM, None)
        } else {
            (PAINT, Some(ctx.paint_at(s)))
        };
        let t = 0.10;
        match rail {
            Rail::Solid | Rail::Enclosed => {
                projection(
                    builder,
                    solid_mat,
                    solid_colour,
                    p0 + out * (depth - t),
                    p1 + out * (depth - t),
                    out,
                    t,
                    y,
                    y + rail_h,
                    None,
                );
                for (p, q, inward) in [(p0, q0, along), (p1, q1, -along)] {
                    let a = p + inward * t * 0.5;
                    let b = q - out * t + inward * t * 0.5;
                    projection(
                        builder,
                        solid_mat,
                        solid_colour,
                        a,
                        b,
                        -inward,
                        t * 0.5,
                        y,
                        y + rail_h,
                        None,
                    );
                }
            }
            Rail::Glass => {
                // A glass panel between a white top rail and the slab, on posts.
                projection(
                    builder,
                    RAIL_GLASS,
                    None,
                    p0 + out * (depth - 0.04),
                    p1 + out * (depth - 0.04),
                    out,
                    0.02,
                    y + 0.06,
                    y + rail_h - 0.06,
                    None,
                );
                projection(
                    builder,
                    TRIM,
                    None,
                    p0 + out * (depth - 0.07),
                    p1 + out * (depth - 0.07),
                    out,
                    0.07,
                    y + rail_h - 0.06,
                    y + rail_h,
                    None,
                );
                for (p, q) in [(p0, q0), (p1, q1)] {
                    box_at(
                        builder,
                        TRIM,
                        (p + q) * 0.5,
                        y + rail_h - 0.03,
                        0.05,
                        0.06,
                        depth,
                        angle,
                    );
                }
            }
            Rail::Bars => {
                // A top rail and balusters.
                projection(
                    builder,
                    TRIM,
                    None,
                    p0 + out * (depth - 0.08),
                    p1 + out * (depth - 0.08),
                    out,
                    0.08,
                    y + rail_h - 0.08,
                    y + rail_h,
                    None,
                );
                let width = p0.distance(p1);
                let count = (width / 0.45).floor().max(2.0) as u32;
                for b in 0..=count {
                    let at = p0 + along * (width * b as f32 / count as f32) + out * (depth - 0.05);
                    box_at(
                        builder,
                        TRIM,
                        at,
                        y + rail_h * 0.5,
                        0.05,
                        rail_h,
                        0.05,
                        angle,
                    );
                }
            }
        }
        if rail == Rail::Enclosed {
            // 封阳台: aluminium glazing from the parapet to the slab above, drawn
            // from the door glazing of the wall behind.
            let gy0 = y + rail_h;
            let gy1 = y + storey - 0.14;
            let v0 = map.v(sf, 1.0);
            let v1 = map.v(sf, head);
            face(
                builder,
                FACADE,
                [
                    Vec3::from_plan(q0 - out * 0.06, gy0),
                    Vec3::from_plan(q1 - out * 0.06, gy0),
                    Vec3::from_plan(q1 - out * 0.06, gy1),
                    Vec3::from_plan(q0 - out * 0.06, gy1),
                ],
                Vec3::new(out.x, 0.0, out.y),
                Some([(da, v0), (db, v0), (db, v1), (da, v1)]),
                Some(ctx.tint_at(s)),
            );
            let side_w = depth * map.u_per_m;
            for (p, q, normal, (sa, sb)) in [
                (p0, q0, -along, (da, da + side_w)),
                (p1, q1, along, (db - side_w, db)),
            ] {
                face(
                    builder,
                    FACADE,
                    [
                        Vec3::from_plan(p, gy0),
                        Vec3::from_plan(q - out * 0.06, gy0),
                        Vec3::from_plan(q - out * 0.06, gy1),
                        Vec3::from_plan(p, gy1),
                    ],
                    Vec3::new(normal.x, 0.0, normal.y),
                    Some([(sa, v0), (sb, v0), (sb, v1), (sa, v1)]),
                    Some(ctx.tint_at(s)),
                );
            }
        }
    }
    let y = ctx.base + ctx.storeys as f32 * storey;
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
    map: &WallMap,
    p0: Vec2,
    p1: Vec2,
    ua: f32,
    ub: f32,
    builder: &mut MeshBuilder,
) {
    let depth = ctx.design.bay_depth;
    let storey = map.storey_m;
    let head = bay_head(storey);
    let out = map.out;
    let along = (p1 - p0).normalize();
    let (ga, gb) = map.shrink(ua, ub, 0.06);
    let g0 = p0 + along * 0.06;
    let g1 = p1 - along * 0.06;
    for s in 0..ctx.storeys {
        let y = ctx.base + s as f32 * storey;
        let sf = s as f32;
        // The head of this storey's bay and the sill of the next are one slab
        // of concrete, so they are one box.
        let below = if s == 0 { y } else { y - (storey - head) };
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
        let (v0, v1) = (map.v(sf, BAY_SILL_M), map.v(sf, head));
        face(
            builder,
            FACADE,
            [
                Vec3::from_plan(g0 + out * (depth - 0.04), y + BAY_SILL_M),
                Vec3::from_plan(g1 + out * (depth - 0.04), y + BAY_SILL_M),
                Vec3::from_plan(g1 + out * (depth - 0.04), y + head),
                Vec3::from_plan(g0 + out * (depth - 0.04), y + head),
            ],
            Vec3::new(out.x, 0.0, out.y),
            Some([(ga, v0), (gb, v0), (gb, v1), (ga, v1)]),
            Some(ctx.tint_at(s)),
        );
        let side_w = depth * map.u_per_m;
        for (p, normal, (sa, sb)) in [
            (p0, -along, (ga, ga + side_w)),
            (p1, along, (gb - side_w, gb)),
        ] {
            face(
                builder,
                FACADE,
                [
                    Vec3::from_plan(p, y + BAY_SILL_M),
                    Vec3::from_plan(p + out * depth, y + BAY_SILL_M),
                    Vec3::from_plan(p + out * depth, y + head),
                    Vec3::from_plan(p, y + head),
                ],
                Vec3::new(normal.x, 0.0, normal.y),
                Some([(sa, v0), (sb, v0), (sb, v1), (sa, v1)]),
                Some(ctx.tint_at(s)),
            );
        }
    }
    let y = ctx.base + ctx.storeys as f32 * storey;
    projection(
        builder,
        PAINT,
        Some(ctx.paint_at(ctx.storeys.saturating_sub(1))),
        p0,
        p1,
        out,
        depth,
        y - (storey - head),
        y,
        None,
    );
}

fn louvre_stack(
    ctx: &Ctx,
    map: &WallMap,
    p0: Vec2,
    p1: Vec2,
    ua: f32,
    ub: f32,
    builder: &mut MeshBuilder,
) {
    let depth = 0.55;
    for s in 0..ctx.storeys {
        let y = ctx.base + s as f32 * map.storey_m;
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
            Some((FACADE, uv, ctx.tint_at(s))),
        );
    }
}

/// Security cages (防盗笼) over the windows of the grilled storeys: a box
/// standing 250 mm off the wall whose faces are the grilled window itself,
/// under a little sloping rain cap.
#[allow(clippy::too_many_arguments)]
fn cage_stack(
    ctx: &Ctx,
    elevation: &Elevation,
    map: &WallMap,
    sill: f32,
    head: f32,
    p0: Vec2,
    p1: Vec2,
    ua: f32,
    ub: f32,
    builder: &mut MeshBuilder,
) {
    let depth = 0.25;
    let out = map.out;
    let along = (p1 - p0).normalize();
    let angle = along.y.atan2(along.x);
    let caged = (elevation.grille_storeys as u32).min(ctx.storeys);
    for s in 0..caged {
        // Not every flat bothers.
        if hash_u32(ctx.id, (ua * 10.0) as i32, s as i32 + 77) < 0.3 {
            continue;
        }
        let y = ctx.base + s as f32 * map.storey_m;
        let sf = s as f32;
        let (v0, v1) = (map.v(sf, sill), map.v(sf, head));
        face(
            builder,
            FACADE,
            [
                Vec3::from_plan(p0 + out * depth, y + sill),
                Vec3::from_plan(p1 + out * depth, y + sill),
                Vec3::from_plan(p1 + out * depth, y + head),
                Vec3::from_plan(p0 + out * depth, y + head),
            ],
            Vec3::new(out.x, 0.0, out.y),
            Some([(ua, v0), (ub, v0), (ub, v1), (ua, v1)]),
            Some(ctx.tint_at(s)),
        );
        let side_w = depth * map.u_per_m;
        for (p, normal, (sa, sb)) in [
            (p0, -along, (ua, ua + side_w)),
            (p1, along, (ub - side_w, ub)),
        ] {
            face(
                builder,
                FACADE,
                [
                    Vec3::from_plan(p, y + sill),
                    Vec3::from_plan(p + out * depth, y + sill),
                    Vec3::from_plan(p + out * depth, y + head),
                    Vec3::from_plan(p, y + head),
                ],
                Vec3::new(normal.x, 0.0, normal.y),
                Some([(sa, v0), (sb, v0), (sb, v1), (sa, v1)]),
                Some(ctx.tint_at(s)),
            );
        }
        // The cage floor and its rain cap.
        projection(
            builder,
            "trim.dark",
            None,
            p0,
            p1,
            out,
            depth,
            y + sill - 0.04,
            y + sill,
            None,
        );
        box_at(
            builder,
            "roof.steel",
            (p0 + p1) * 0.5 + out * (depth * 0.5),
            y + head + 0.06,
            p0.distance(p1) + 0.1,
            0.04,
            depth + 0.1,
            angle,
        );
    }
}

/// Air conditioners on a gable: a column beside its window.
fn gable_units(ctx: &Ctx, map: &WallMap, length: f32, builder: &mut MeshBuilder) {
    let centre_u = map.u0 + length * 0.5 * map.u_per_m;
    let side = if hash_u32(ctx.id, 251, 5) < 0.5 {
        -1.0
    } else {
        1.0
    };
    let p = 0.3 + 0.6 * ctx.design.hung_ac;
    for s in 0..ctx.storeys {
        if hash_u32(ctx.id, 253, s as i32) < p {
            let y = ctx.base + s as f32 * map.storey_m + 0.4;
            hung_condenser(map, centre_u + side * 1.25, y, builder);
        }
    }
}

/// Unit entrances (单元门) on the service side: a door, a canopy, two steps.
/// They stand at the stairs, which the back elevation puts at every other
/// tile edge.
fn entrances(
    ctx: &Ctx,
    elevation: &Elevation,
    map: &WallMap,
    tower: bool,
    builder: &mut MeshBuilder,
) {
    let w = elevation.width;
    let has_stair = elevation.spans.iter().any(|span| span.part == Part::Stair);
    let mut doors: Vec<f32> = Vec::new();
    if tower || map.units < 2 || !has_stair {
        if map.units >= 1 && (tower || hash_u32(ctx.id, 255, 5) < 0.5) {
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
    fn notched_plans_stay_inside_their_frame_and_keep_most_of_their_area() {
        let frame = Frame::fit(&rect(26.0, 18.0)).unwrap();
        for id in 0..40 {
            for era in [Era::Modern, Era::Tile, Era::Euro] {
                let (front, back) = notches_for(&frame, true, era, id);
                let plan = notched_plan(&frame, &front, &back);
                assert!(signed_area(&plan) < 0.0, "plan must be clockwise");
                let area = signed_area(&plan).abs();
                assert!(area > 0.7 * 26.0 * 18.0 && area <= 26.0 * 18.0 + 1e-3);
                for p in &plan {
                    assert!(p.x > -1e-3 && p.x < 26.0 + 1e-3 && p.y > -1e-3 && p.y < 18.0 + 1e-3);
                }
            }
        }
    }

    #[test]
    fn a_development_shares_one_design() {
        let mut table = FacadeTable::new();
        let mut a = ModernBuilding {
            id: 1,
            parcel_id: 9,
            footprint: Vec::new(),
            height_metres: 30.0,
            floors: 11,
            use_type: ParcelUse::Residential,
            roof: urban::RoofStyle::Flat,
            facade: urban::BuildingFacade::PlasterResidential,
            podium_height_metres: 0.0,
            window_bays: 0,
            balcony_bays: 0,
            entrance_count: 1,
        };
        let (_, rows_a) = design_of(&a, &mut table);
        a.id = 2;
        let (_, rows_b) = design_of(&a, &mut table);
        assert_eq!(rows_a, rows_b);
        a.parcel_id = 10;
        let (_, rows_c) = design_of(&a, &mut table);
        assert_ne!(rows_a, rows_c);
    }
}
