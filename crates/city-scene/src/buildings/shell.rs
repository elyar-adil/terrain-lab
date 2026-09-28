//! The building shell: plinth, ground floor, podium, shaft, crown, roof.
//!
//! The shell is assembled from the dimensioned facade designs — the shaft's
//! storey height *is* its design's 层高, the podium's is the podium tile's, the
//! crown's is the crown tile's — so the painted floor lines and the built floor
//! lines are the same lines by construction, and the whole elevation reads as a
//! tripartite composition: an articulated base, a shaft with strict rhythm, and
//! a stepped crown over a parapet with a coping.
//!
//! Two UV rules are load-bearing here:
//!
//! * facade walls go through [`super::facade_wall`], whose `V` datum puts a
//!   floor line on every quarter tile;
//! * `roof` is a **textured** material, so every wall and deck emitted into it
//!   carries metre UVs (`wall_uv` / `quad_uv` / `ground_uv`).  A bare quad into
//!   a textured group desyncs the optional UV layer from the vertex count and
//!   the payload fails to load.

use urban::{ModernBuilding, RoofStyle, modern};

use super::details;
use super::roofscape::{self, ring_extent};
use super::{
    Jitter, Massing, crown_tile_for, facade_tile_for, ground_floor_material, jitter_for,
    massing_of, podium_storeys, podium_tile_for, rule,
};
use super::{facade_wall, level};
use crate::facades::{GROUND_STOREY_M, design};
use crate::math::{Vec2, Vec3, inset_ring, ring_centroid, signed_area};
use crate::mesh::MeshBuilder;

/// How a slab's plan departs from a pure rectangle.  A district of pure
/// rectangles is a trading estate; a Chinese 板楼 district is L-shaped ends,
/// twin prongs and H plans, with the odd stair-tower step thrown in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SlabVariant {
    /// The plain rectangle — still the majority.
    Rectangle,
    /// One end notched away: the L-plan slab.
    CarvedL,
    /// A mid notch: two prongs on a shared base, the connected pair.
    CarvedU,
    /// A notch from each end: the H plan.
    CarvedH,
    /// The rectangle kept, but the last bays rise two to three extra storeys —
    /// 板塔结合, a point block grown out of the slab's end.
    Stepped,
}

/// The slab variant for a building id.  Roughly 60% of slabs depart from the
/// rectangle, in a fixed blend so any seed shows all of them.
pub(crate) fn slab_variant(id: u32) -> SlabVariant {
    let roll = modern::hash_u32(id, 19, 43);
    if roll < 0.16 {
        SlabVariant::CarvedL
    } else if roll < 0.30 {
        SlabVariant::CarvedU
    } else if roll < 0.42 {
        SlabVariant::CarvedH
    } else if roll < 0.60 {
        SlabVariant::Stepped
    } else {
        SlabVariant::Rectangle
    }
}

/// Carve rectangular notches out of one long side of an (axis-aligned)
/// rectangle, as fractions of the long side, keeping the winding.
///
/// The plan generator emits exact rectangles, so the carve is built from the
/// bounding box and is guaranteed to stay inside the footprint — which is what
/// lets every placement test keep reasoning about the plan's ring.
fn carved_ring(ring: &[Vec2], notches: &[(f32, f32, f32)]) -> Vec<Vec2> {
    let Some([min_x, min_y, max_x, max_y]) = ring_extent(ring) else {
        return ring.to_vec();
    };
    let w = max_x - min_x;
    let d = max_y - min_y;
    let mut out: Vec<Vec2> = Vec::with_capacity(4 + notches.len() * 4);
    if w >= d {
        // Long axis along X: the notch cuts into the +Y side.
        let along = w;
        out.push(Vec2::new(min_x, min_y));
        out.push(Vec2::new(max_x, min_y));
        out.push(Vec2::new(max_x, max_y));
        for (t0, t1, depth) in notches.iter().rev() {
            let x1 = min_x + along * t1;
            let x0 = min_x + along * t0;
            let y_notch = max_y - d * depth;
            out.push(Vec2::new(x1, max_y));
            out.push(Vec2::new(x1, y_notch));
            out.push(Vec2::new(x0, y_notch));
            out.push(Vec2::new(x0, max_y));
        }
        out.push(Vec2::new(min_x, max_y));
    } else {
        // Long axis along Y: the notch cuts into the +X side.
        let along = d;
        out.push(Vec2::new(min_x, min_y));
        out.push(Vec2::new(max_x, min_y));
        for (t0, t1, depth) in notches {
            let y0 = min_y + along * t0;
            let y1 = min_y + along * t1;
            let x_notch = max_x - w * depth;
            out.push(Vec2::new(max_x, y0));
            out.push(Vec2::new(x_notch, y0));
            out.push(Vec2::new(x_notch, y1));
            out.push(Vec2::new(max_x, y1));
        }
        out.push(Vec2::new(max_x, max_y));
        out.push(Vec2::new(min_x, max_y));
    }
    // A carve that would fold the ring over (a notch deeper than the depth, or
    // notches that meet) produces an invalid polygon; fall back to the
    // rectangle rather than emit an inside-out building.
    if signed_area(&out).abs() < signed_area(ring).abs() * 0.45 {
        return ring.to_vec();
    }
    out
}

fn carve_for(variant: SlabVariant, ring: &[Vec2]) -> Vec<Vec2> {
    // A notch needs a long side to cut into and enough depth to remain a slab
    // on both sides of the cut; small or awkward plans stay rectangles.
    let Some([min_x, min_y, max_x, max_y]) = ring_extent(ring) else {
        return ring.to_vec();
    };
    let (w, d) = (max_x - min_x, max_y - min_y);
    let (long, short) = if w >= d { (w, d) } else { (d, w) };
    let carvable = long > 30.0 && short > 10.0;
    match variant {
        SlabVariant::CarvedL if carvable => carved_ring(ring, &[(0.0, 0.30, 0.52)]),
        SlabVariant::CarvedU if carvable => carved_ring(ring, &[(0.36, 0.64, 0.55)]),
        SlabVariant::CarvedH if carvable => {
            carved_ring(ring, &[(0.05, 0.27, 0.50), (0.73, 0.95, 0.50)])
        }
        _ => ring.to_vec(),
    }
}

pub(crate) fn building_shell(building: &ModernBuilding, ring: &[Vec2], builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    let area = signed_area(ring).abs();
    let perimeter: f32 = ring
        .iter()
        .enumerate()
        .map(|(index, point)| point.distance(ring[(index + 1) % ring.len()]))
        .sum();
    let mut massing = massing_of(building, ring, area);
    let Jitter { mirror, value: jitter_value } = jitter_for(building.id);
    // Colour variation comes from the baked tiles and the material table; see
    // the note on vertex tints in `super` for why this layer writes no colours.
    let tint: Option<[f32; 3]> = None;

    // Outward-facing ring: `inset_ring` on a reversed ring flips the winding, so
    // work from an explicitly outward ring rather than trusting the plan's.
    let mut outward = ring.to_vec();
    if signed_area(&outward) < 0.0 {
        outward.reverse();
    }

    let shaft_index = facade_tile_for(building, massing);
    let shaft_material = format!("facade/{shaft_index:02}");
    let shaft_design = design(shaft_index);
    let storey = shaft_design.storey_m;
    let glass = shaft_design.cladding.is_glass();
    if glass && massing == Massing::Tower {
        massing = Massing::CurtainTower;
    }
    let pitched = building.roof == RoofStyle::Terracotta
        || (building.roof == RoofStyle::Mansard && building.floors <= 6);

    // The storey arithmetic.  A ground storey of `GROUND_STOREY_M` and a whole
    // number of the shaft design's 层高 above it, so the parapet lands on a
    // floor and the tile never has to stretch.
    let upper_storeys = (building.floors as f32 - 1.0).max(1.0);
    let shaft_base = level::GROUND + GROUND_STOREY_M;
    let mut top = shaft_base + upper_storeys * storey;

    // Slab plan variants.  A carved slab builds everything — plinth, ground
    // band, shaft, parapet, roofscape — on the carved ring, so the notch is a
    // real notch and not a decal.
    if massing == Massing::Slab {
        let carved = carve_for(slab_variant(building.id), &outward);
        if carved.len() != outward.len() {
            outward = carved;
        }
    }

    // Plinth: a skirt below the ground floor so the building meets the terrain
    // on a slope instead of hovering over it.
    for index in 0..outward.len() {
        let a = outward[index];
        let b = outward[(index + 1) % outward.len()];
        builder.wall("trim.dark", a, b, level::PLINTH, level::GROUND, None);
    }

    // --- the ground floor, as a continuous band ---------------------------
    let ground_material = ground_floor_material(building, shaft_index);
    // The shopfront band.  Its own storey height, and its own UV datum: `V` runs
    // from zero at the *head* of the ground storey downwards, so the tile — which
    // is authored as one 4.2 m bay by one 4.5 m storey at true scale — is drawn
    // exactly once with no vertical repeat.  A door in the bake is 2.1 m tall and
    // it is 2.1 m tall on the wall.
    let ground_h = GROUND_STOREY_M;
    for index in 0..outward.len() {
        let a = outward[index];
        let b = outward[(index + 1) % outward.len()];
        let length = a.distance(b);
        if length <= 1.0e-3 {
            continue;
        }
        let u1 = if mirror { -length } else { length };
        builder.quad_uv(
            ground_material,
            Vec3::from_plan(a, level::GROUND),
            Vec3::from_plan(b, level::GROUND),
            Vec3::from_plan(b, level::GROUND + ground_h),
            Vec3::from_plan(a, level::GROUND + ground_h),
            [(0.0, 1.0), (u1, 1.0), (u1, 0.0), (0.0, 0.0)],
            None,
        );
    }
    let retail = ground_material == "ground/shop";
    if area > 60.0 {
        details::retail_band(&outward, centre, building, retail, builder);
    }
    // A stone portal at the entrance of a lobby building: two piers and a head,
    // which is what makes a tower door read as an address and not as a gap.
    if ground_material == "ground/lobby" && area > 90.0 {
        entrance_portal(&outward, builder);
    }

    // --- the shaft ---------------------------------------------------------
    let tower = matches!(massing, Massing::Tower | Massing::CurtainTower);
    // A tower's shaft steps back from its podium; a slab's does not, because a
    // 板楼 with a setback is a different building.
    let shaft_ring = if tower {
        inset_ring(&outward, 2.2 + (building.floors as f32 * 0.08).min(3.0))
    } else {
        outward.clone()
    };
    // Crowns: two setbacks for most towers, three for the tallest, each one
    // storey of the *crown design's* own 层高.
    let crown_stages = if tower && building.floors >= 30 {
        3.0_f32
    } else if tower {
        2.0
    } else {
        0.0
    };
    let crown_index = if tower { crown_tile_for(shaft_index, building.id) } else { shaft_index };
    let crown_storey = design(crown_index).storey_m;

    // The podium: a 裙房 of two to four retail storeys, in its own material, with
    // its own shopfront band at the pavement and a deck where the shaft lands.
    let mut shaft_base_local = shaft_base;
    let mut shaft_top = top;
    if tower {
        let podium_index = podium_tile_for(shaft_index);
        let podium_storey = design(podium_index).storey_m;
        let storeys = podium_storeys(building);
        let podium_top = shaft_base + storeys * podium_storey;
        for index in 0..shaft_ring.len() {
            let a = shaft_ring[index];
            let b = shaft_ring[(index + 1) % shaft_ring.len()];
            facade_wall(
                builder,
                &format!("facade/{podium_index:02}"),
                a,
                b,
                shaft_base,
                podium_top,
                storeys,
                mirror,
                tint,
            );
        }
        // Pier relief on the podium too: a tile-clad podium with shadowed
        // pilasters is the base a glass tower needs to not float.
        details::wall_relief(
            builder,
            &format!("facade/{podium_index:02}"),
            &shaft_ring,
            shaft_base,
            podium_top,
            storeys,
            mirror,
            design(podium_index),
        );
        podium_deck(&outward, &shaft_ring, podium_top, builder);
        parapet(&shaft_ring, podium_top, 0.55, builder);
        // Podium roofs are working roofs: plant and a greening patch, not a lid.
        roofscape::podium_deck_props(building, &shaft_ring, podium_top, builder);

        // The shaft starts on the deck and ends a whole number of storeys below
        // the plan top, leaving the crown stages out of the shaft's count.
        shaft_base_local = podium_top;
        let total = ((top - podium_top) / storey).round().max(crown_stages + 3.0);
        shaft_top = podium_top + (total - crown_stages) * storey;
        top = podium_top + total * storey;
    }

    let shaft_storeys = (shaft_top - shaft_base_local) / storey;
    for index in 0..shaft_ring.len() {
        let a = shaft_ring[index];
        let b = shaft_ring[(index + 1) % shaft_ring.len()];
        facade_wall(
            builder,
            &shaft_material,
            a,
            b,
            shaft_base_local,
            shaft_top,
            shaft_storeys,
            mirror,
            tint,
        );
    }

    // Real relief on the shaft walls: projecting pier strips on masonry, a
    // cluster of full-height fins on curtain wall.  This is the geometry half
    // of the vertical rhythm — the tile paints it, the wall now casts it.
    if glass {
        details::curtain_fins(
            builder,
            &shaft_material,
            &shaft_ring,
            shaft_base_local,
            shaft_top,
            shaft_storeys,
            mirror,
            shaft_design,
        );
    } else {
        details::wall_relief(
            builder,
            &shaft_material,
            &shaft_ring,
            shaft_base_local,
            shaft_top,
            shaft_storeys,
            mirror,
            shaft_design,
        );
    }

    // Sill courses at every floor, and 飘窗 bay courses on the 2000s tile-clad
    // stock: the horizontal relief a storey rhythm needs to survive raking
    // light.  Masonry only — a curtain wall carries its painted coping.
    if !glass {
        details::sill_courses(builder, &shaft_ring, shaft_base_local, shaft_top, shaft_design);
    }

    // String courses.  A projecting band every four storeys is the strongest
    // horizontal the elevation has, and it is real geometry, so it throws a real
    // shadow at every storey of the day instead of a painted one.
    if massing != Massing::LowRise && shaft_storeys >= 8.0 {
        string_courses(&shaft_ring, shaft_base_local, shaft_top, 4.0, storey, builder);
    }

    // --- the roof ----------------------------------------------------------
    let cap_ring = shaft_ring.clone();
    builder.ground_uv("roof", &cap_ring, shaft_top, None);
    parapet(&cap_ring, shaft_top, 0.55 + jitter_value * 0.2, builder);
    if area > 90.0 {
        roofscape::roofscape(building, &cap_ring, shaft_top, builder);
    }

    // --- the crown ---------------------------------------------------------
    if tower {
        // Setbacks and a crown in its own material.  A tower with one flat
        // top is a box; a tower with a stepped top is a skyline.
        let crown_material = format!("facade/{crown_index:02}");
        let mut stage = shaft_ring.clone();
        let mut stage_deck = shaft_top;
        let inset = [1.1_f32, 1.4, 1.1];
        for stage_number in 0..crown_stages as usize {
            let next = inset_ring(&stage, inset[stage_number.min(2)]);
            for index in 0..stage.len() {
                let a = stage[index];
                let b = stage[(index + 1) % stage.len()];
                facade_wall(
                    builder,
                    &crown_material,
                    a,
                    b,
                    stage_deck,
                    stage_deck + crown_storey,
                    1.0,
                    mirror,
                    tint,
                );
            }
            stage_deck += crown_storey;
            // The step's own deck and its shadowed fascia, so the setback reads
            // as a step rather than as a hole.
            podium_deck(&stage, &next, stage_deck, builder);
            builder.ground_uv("roof", &next, stage_deck, None);
            parapet(&next, stage_deck, 0.62, builder);
            stage = next;
        }
        if crown_stages >= 3.0 {
            // The topmost step is small, so it carries a mast and a condenser
            // rather than a full roofscape: the silhouette detail that says
            // "city" from a kilometre out.
            roofscape::crown_mast(building.id, &stage, stage_deck, builder);
        } else {
            roofscape::roofscape(building, &stage, stage_deck, builder);
        }
    }

    // --- the stepped slab end ----------------------------------------------
    if massing == Massing::Slab && slab_variant(building.id) == SlabVariant::Stepped {
        stepped_end(building, &outward, shaft_base, shaft_top, storey, &shaft_material, builder);
    }

    // --- the balconies, the condensers and the rainwater --------------------
    let balcony_ring = if tower { shaft_ring.clone() } else { outward.clone() };
    let balcony_base = shaft_base_local;
    let has_balconies = rule::has_balconies(massing, glass) && !pitched;
    if has_balconies && perimeter < 150.0 {
        // **One** frontage on a slab, two on a compact tower.  A 板楼 gives its
        // whole south face over to balconies and leaves its ends blank, which is
        // both what it looks like and half the geometry: the balcony stack is
        // the single most expensive element in this module.
        let fronts = front_edges(&balcony_ring, if tower { 2 } else { 1 });
        details::balconies(
            &balcony_ring,
            &fronts,
            building,
            balcony_base,
            shaft_top,
            storey,
            builder,
        );
        // Every balcony on a Chinese residential block carries a condenser unit,
        // and the faces without balconies carry wall-mounted units on a sparser
        // rhythm.  They are what stops a slab from reading as a stack of
        // stripes, and from a distance they are most of the texture of the wall.
        details::condensers(&balcony_ring, &fronts, building, balcony_base, shaft_top, storey, builder);
        details::wall_ac_units(
            &balcony_ring,
            &fronts,
            building,
            balcony_base,
            shaft_top,
            storey,
            builder,
        );
        // 晾衣杆 and the odd drying cage on the balcony line: half the year the
        // balcony is a laundry.
        details::drying_cages(&balcony_ring, &fronts, building, balcony_base, shaft_top, storey, builder);
    } else if !glass && !pitched && perimeter < 150.0 {
        // Masonry without balconies still grows condensers on a wall rhythm.
        details::wall_ac_units(&balcony_ring, &[], building, balcony_base, shaft_top, storey, builder);
    }
    // A drainpipe down one corner.  It is a single tube, and it is on every
    // building in China.
    if perimeter > 18.0 {
        details::downpipe(&outward, level::GROUND, top, building.id, builder);
    }
    if building.entrance_count > 0 && area > 90.0 {
        details::entrance_canopy(&outward, building, builder);
    }

    // --- the pitched roof, on a fully built shell ---------------------------
    // The old code returned before any wall existed, which left the pitched
    // walk-ups of the legacy city as floating pyramids.  They now get the whole
    // shell — plinth, ground band, shaft, balconies — and the roof on top.
    if pitched {
        pitched_roof(&outward, top, builder);
    }
}

/// The two longest edges of a ring, which is where a Chinese block puts its
/// balconies and its entrance: the frontage, not the flanks.
pub(crate) fn front_edges(ring: &[Vec2], count: usize) -> Vec<usize> {
    let mut order: Vec<(usize, f32)> = (0..ring.len())
        .map(|index| {
            let a = ring[index];
            let b = ring[(index + 1) % ring.len()];
            (index, a.distance(b))
        })
        .collect();
    order.sort_by(|a, b| b.1.total_cmp(&a.1));
    let mut edges: Vec<usize> = order.iter().take(count).map(|(index, _)| *index).collect();
    edges.sort_unstable();
    edges
}

fn podium_deck(outer: &[Vec2], inner: &[Vec2], y: f32, builder: &mut MeshBuilder) {
    if outer.len() != inner.len() {
        return;
    }
    for index in 0..outer.len() {
        let a = outer[index];
        let b = outer[(index + 1) % outer.len()];
        let c = inner[(index + 1) % inner.len()];
        let d = inner[index];
        // The deck is `roof` — a textured material — so the quad carries the
        // world-space metre UVs `ground_uv` would give it.
        builder.quad_uv(
            "roof",
            Vec3::from_plan(a, y),
            Vec3::from_plan(d, y),
            Vec3::from_plan(c, y),
            Vec3::from_plan(b, y),
            [(a.x, a.y), (d.x, d.y), (c.x, c.y), (b.x, b.y)],
            None,
        );
        // The vertical face of the step itself.
        builder.wall("trim.dark", d, c, y - 0.20, y + 0.05, None);
    }
}

/// A projecting band at every `every`-th storey: a string course, a drip edge and
/// the shadow under it.  Cheap, and it is the one detail that gives a tall slab
/// a horizontal scale.
fn string_courses(ring: &[Vec2], base: f32, top: f32, every: f32, storey: f32, builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    let mut count = every;
    while base + count * storey < top - 1.0 {
        let y = base + count * storey;
        for index in 0..ring.len() {
            let a = ring[index];
            let b = ring[(index + 1) % ring.len()];
            if a.distance(b) < 1.5 {
                continue;
            }
            let outward = ((a + b) * 0.5 - centre).normalize();
            // A 220 mm projection: enough to throw a shadow line, not enough to
            // read as a ledge.
            let d = 0.22;
            let a_out = a + outward * d;
            let b_out = b + outward * d;
            // Top face, front face, and the shadowed underside of the overhang.
            builder.quad(
                "trim.light",
                Vec3::from_plan(a, y + 0.26),
                Vec3::from_plan(b, y + 0.26),
                Vec3::from_plan(b_out, y + 0.26),
                Vec3::from_plan(a_out, y + 0.26),
                None,
            );
            builder.wall("trim.light", b_out, a_out, y, y + 0.26, None);
            builder.wall("trim.dark", a, b, y - 0.10, y, None);
        }
        count += every;
    }
}

/// A parapet: one wall per roof edge, with a capping band on top and the shadow
/// its overhang throws on the roof deck.  The parapet walls are `roof` — a
/// textured material — so they carry metre UVs.
pub(crate) fn parapet(ring: &[Vec2], deck: f32, height: f32, builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        builder.wall_uv("roof", a, b, deck, deck + height, false, None);
        // A coping that projects: 60 mm, and a drip under it.  A flush parapet
        // reads as a cut edge; an overhanging coping reads as a built one.
        let outward = ((a + b) * 0.5 - centre).normalize();
        let a_out = a + outward * 0.06;
        let b_out = b + outward * 0.06;
        builder.quad(
            "trim.light",
            Vec3::from_plan(a, deck + height),
            Vec3::from_plan(b, deck + height),
            Vec3::from_plan(b_out, deck + height),
            Vec3::from_plan(a_out, deck + height),
            None,
        );
        builder.wall("trim.light", b_out, a_out, deck + height, deck + height + 0.09, None);
        builder.wall("trim.dark", a, b, deck, deck + 0.05, None);
    }
}

fn pitched_roof(ring: &[Vec2], top: f32, builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    let eaves = 0.55_f32;
    // An eave overhang first: a tiled roof that stops flush with the wall has no
    // shadow line under it and reads as a paper cone.
    let eave_ring: Vec<Vec2> = ring.iter().map(|p| centre + (*p - centre) * (1.0 + eaves / 40.0)).collect();
    for index in 0..ring.len() {
        let a = eave_ring[index];
        let b = eave_ring[(index + 1) % ring.len()];
        // The soffit.
        builder.quad(
            "trim.light",
            Vec3::from_plan(ring[index], top),
            Vec3::from_plan(ring[(index + 1) % ring.len()], top),
            Vec3::from_plan(b, top - 0.10),
            Vec3::from_plan(a, top - 0.10),
            None,
        );
        builder.wall("trim.light", b, a, top - 0.22, top - 0.10, None);
    }
    let rise = 1.6_f32 + 0.7;
    let apex = Vec3::new(centre.x, top + rise, centre.y);
    for index in 0..ring.len() {
        let a = eave_ring[index];
        let b = eave_ring[(index + 1) % ring.len()];
        builder.tri_flat(
            "roof",
            Vec3::from_plan(a, top - 0.10),
            Vec3::from_plan(b, top - 0.10),
            apex,
            None,
        );
    }
    // A ridge tile along the top, which is what actually catches the light on a
    // pitched roof and stops it reading as a fold.
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let mid = (a + b) * 0.5;
        let along = (b - a).normalize();
        let half = a.distance(b) * 0.5 + 0.12;
        let r0 = mid - along * half;
        let r1 = mid + along * half;
        builder.tube(
            "trim.dark",
            Vec3::new(r0.x, top + rise - 0.06, r0.y),
            Vec3::new(r1.x, top + rise - 0.06, r1.y),
            0.14,
            0.14,
            4,
            None,
        );
    }
}

/// A stair-tower step grown out of one end of a slab: the last bays rise two to
/// three storeys above the roofline, with their own parapet, deck and tank —
/// 板塔结合, and the roofline variation that keeps a slab district from being a
/// bar chart.  The block is inset a joint's width from the slab face so the two
/// shells never z-fight where they meet.
fn stepped_end(
    building: &ModernBuilding,
    ring: &[Vec2],
    base: f32,
    slab_top: f32,
    storey: f32,
    material: &str,
    builder: &mut MeshBuilder,
) {
    let Some([min_x, min_y, max_x, max_y]) = ring_extent(ring) else {
        return;
    };
    let (w, d) = (max_x - min_x, max_y - min_y);
    if w < 30.0 || d < 8.0 {
        return;
    }
    let along_x = w >= d;
    let extra = 2.0 + (building.id % 2) as f32;
    let top = slab_top + extra * storey;
    let frac = 0.24;
    let block: Vec<Vec2> = if along_x {
        // Alternate which end grows, so steps stagger down a street.
        let (x0, x1) = if building.id % 4 < 2 {
            (min_x, min_x + w * frac)
        } else {
            (max_x - w * frac, max_x)
        };
        vec![Vec2::new(x0, min_y), Vec2::new(x1, min_y), Vec2::new(x1, max_y), Vec2::new(x0, max_y)]
    } else {
        let (y0, y1) = if building.id % 4 < 2 {
            (min_y, min_y + d * frac)
        } else {
            (max_y - d * frac, max_y)
        };
        vec![Vec2::new(min_x, y0), Vec2::new(max_x, y0), Vec2::new(max_x, y1), Vec2::new(min_x, y1)]
    };
    let block = inset_ring(&block, 0.07);
    let storeys = (top - base) / storey;
    for index in 0..block.len() {
        let a = block[index];
        let b = block[(index + 1) % block.len()];
        facade_wall(builder, material, a, b, base, top, storeys, false, None);
    }
    builder.ground_uv("roof", &block, top, None);
    parapet(&block, top, 0.6, builder);
    roofscape::roofscape(building, &block, top, builder);
}

/// A stone portal at a lobby entrance: two piers and a head, placed on the
/// middle of the longest edge.  The ground bake paints the deep portal shadow;
/// this gives the shadow something to be about.
fn entrance_portal(ring: &[Vec2], builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    let Some(edge) = front_edges(ring, 1).first().copied() else {
        return;
    };
    let a = ring[edge];
    let b = ring[(edge + 1) % ring.len()];
    if a.distance(b) < 8.0 {
        return;
    }
    let outward = ((a + b) * 0.5 - centre).normalize();
    let along = (b - a).normalize();
    let mid = (a + b) * 0.5;
    let height = 4.3;
    let yaw = along.angle();
    // Two piers 0.7 m square, 4.5 m apart, and the head beam over them.
    for side in [-2.25_f32, 2.25] {
        let pier = mid + along * side + outward * 0.18;
        crate::mesh::box_at(builder, "trim.light", pier, level::GROUND + height * 0.5, 0.7, height, 0.55, yaw);
    }
    let head_centre = mid + outward * 0.18;
    crate::mesh::box_at(
        builder,
        "trim.light",
        head_centre,
        level::GROUND + height + 0.30,
        5.9,
        0.60,
        0.65,
        yaw,
    );
    // The address band over the portal, in the shop-sign material so it reads as
    // the building's name board.
    let sign_y = level::GROUND + height + 0.72;
    let half = 2.6_f32;
    let s0 = head_centre - along * half;
    let s1 = head_centre + along * half;
    let so0 = s0 + outward * 0.10;
    let so1 = s1 + outward * 0.10;
    builder.quad_uv(
        "sign/shop",
        Vec3::from_plan(s0, sign_y),
        Vec3::from_plan(s1, sign_y),
        Vec3::from_plan(s1, sign_y + 0.75),
        Vec3::from_plan(s0, sign_y + 0.75),
        [(0.0, 0.0), (2.0 * half, 0.0), (2.0 * half, 0.75), (0.0, 0.75)],
        None,
    );
    builder.quad(
        "trim.dark",
        Vec3::from_plan(so0, sign_y + 0.75),
        Vec3::from_plan(so1, sign_y + 0.75),
        Vec3::from_plan(so1, sign_y),
        Vec3::from_plan(so0, sign_y),
        None,
    );
}
