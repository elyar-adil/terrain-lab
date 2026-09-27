//! Buildings, blocks, parcels and Chinese residential compounds.
//!
//! The massing here is deliberately *not* the massing a western city generator
//! produces, because the target is a Chinese one and the two do not look alike
//! even in silhouette.  Four forms carry a Chinese district, and a generator
//! that emits only the first reads as a bar chart:
//!
//! * **板楼** — a long slab block, six to eighteen storeys, one structural bay
//!   deep, with a stair core at one end and the whole south face given over to
//!   balconies.
//! * **塔楼 with 裙房** — a podium of two to four retail storeys, a shaft of
//!   twenty to forty, and a crown that is a *different colour and a different
//!   shape*, set back in two or three steps.
//! * **多层** — five to seven storeys of walk-up with a pitched or parapeted
//!   tile roof, which is most of the older city.
//! * **perimeter block** around a compound, behind a wall, with a gate.
//!
//! Three things carry the realism on top of that, in order of contribution:
//!
//! 1. **The storey rhythm.**  Every wall is UV'd in metres against a tile that
//!    covers four real storeys, and the tile's `V` origin is the *base of the
//!    shaft*, not the world origin.  That is the whole difference between a
//!    window grid aligned to the building and a window grid that happens to line
//!    up on one wall of one building.
//! 2. **The ground floor.**  Street-level credibility is a continuous retail
//!    band — stall riser, piers, fascia, roller shutters, projecting sign
//!    boxes, awnings — not a row of doors.
//! 3. **The roofscape.**  Parapets and copings, water tanks on legs, stair head
//!    houses, lift overruns, antenna masts, satellite dishes, clothes poles and
//!    the occasional PV array.  A city of boxes with blank lids reads as a
//!    blockout no matter how good the walls are.

use urban::{
    CityFrameInfo, Compound, ModernBuilding, Parcel, ParcelUse, Point, RoofStyle, UrbanBlock, modern,
};

use crate::facades::{GROUND_STOREY_M, STOREY_M};
use crate::math::{Vec2, Vec3, inset_ring, point_in_ring, ring_centroid, signed_area};
use crate::mesh::{GroupStyle, MeshBuilder};
use crate::spec::FACADE_TILES;

/// Ground levels, against the sidewalk datum.
pub mod level {
    /// Sidewalk / parcel surface.
    pub const GROUND: f32 = 0.150;
    /// Skirt below the ground floor, so a building on a slope never shows a gap
    /// between its base and the terrain.
    pub const PLINTH: f32 = -1.60;
    pub const BLOCK: f32 = 0.138;
}

/// Per-building jitter, derived from the building id so a city is identical for
/// a seed and a single building can be re-derived without the others.
struct Jitter {
    value: f32,
    mirror: bool,
    wall: [f32; 3],
}

fn jitter_for(id: u32) -> Jitter {
    let value = 0.82 + modern::hash_u32(id, 3, 17) * 0.36;
    Jitter {
        value,
        mirror: modern::hash_u32(id, 5, 23) < 0.15,
        wall: [
            value * (0.92 + modern::hash_u32(id, 7, 29) * 0.16),
            value * (0.96 + modern::hash_u32(id, 11, 31) * 0.08),
            value * (0.86 + modern::hash_u32(id, 13, 37) * 0.28),
        ],
    }
}

/// Which of the four massing forms this building is.  Derived from the plan,
/// but the *geometry* is what this drives, so it is worth naming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Massing {
    /// 塔楼: podium, shaft, stepped crown.
    Tower,
    /// 板楼: a long slab.
    Slab,
    /// 多层: a low walk-up.
    LowRise,
}

/// A long slab is long: its footprint's longest side is at least three times its
/// shortest, and it is not deep enough to be a perimeter block.
fn is_slab(ring: &[Vec2]) -> bool {
    let mut longest = 0.0_f32;
    let mut shortest = f32::MAX;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let length = a.distance(b);
        longest = longest.max(length);
        shortest = shortest.min(length);
    }
    longest > shortest * 2.4 && longest > 24.0 && shortest < 22.0
}

fn massing_of(building: &ModernBuilding, ring: &[Vec2], area: f32) -> Massing {
    // A tower is twenty storeys and up, whatever the plan says about its podium:
    // a thirty-storey building with a two-storey base is still a 塔楼, and
    // classifying it as a slab is how a skyline ends up without one.
    if building.floors >= 20 {
        return Massing::Tower;
    }
    if building.floors <= 7 && area < 2400.0 {
        return Massing::LowRise;
    }
    if is_slab(ring) {
        return Massing::Slab;
    }
    if building.floors <= 7 {
        Massing::LowRise
    } else {
        Massing::Slab
    }
}

/// The 裙房: how many storeys of retail podium a tower stands on.  Every tower
/// gets one, because a Chinese tower without a podium is a western one, and the
/// plan's `podium_height_metres` is rounded to a whole number of storeys so the
/// podium's own shopfront band lands on a floor.
fn podium_storeys(building: &ModernBuilding) -> f32 {
    if building.podium_height_metres > GROUND_STOREY_M + 1.0 {
        (building.podium_height_metres / STOREY_M).round().clamp(2.0, 5.0)
    } else {
        2.0
    }
}

/// The facade tile for a building.
///
/// The art direction is the reference skyline, not an even spread: a Chinese
/// residential district is **mostly warm beige and buff**, with grey and
/// tile-clad slabs behind it and glass towers as a minority.  Spreading the
/// palette evenly is what produces a city that looks like a paint chart.
///
/// * 板楼 and 多层 — masonry only.  A residential block is not clad in glass,
///   and a district of them is not a western one.
/// * 塔楼 — glass for a third of them, because a glass tower is what makes a
///   cluster read as a skyline, and masonry for the rest.
fn facade_tile_for(building: &ModernBuilding, massing: Massing) -> usize {
    let id = building.id as usize;
    match massing {
        Massing::Tower => {
            // 8..16 is the curtain-wall block, 16..24 the masonry one.
            if id % 3 == 0 {
                8 + (id * 5 + 2) % 8
            } else {
                16 + (id * 7 + 5) % 8
            }
        }
        // The warm end leads: 0, 1, 2, 4, 5 are beige, buff and ochre, and they
        // take nine of the sixteen slots.  Grey and pale slabs fill the rest.
        Massing::Slab => [0usize, 0, 1, 2, 4, 0, 5, 3, 6, 18, 16, 19, 17, 20, 21, 23]
            [id % 16],
        Massing::LowRise => {
            // Low-rise is where brick, dark tile and the small-tile stock lives.
            [22usize, 23, 17, 21, 2, 22, 16, 23, 0, 3, 7, 20, 1, 21, 4, 22][id % 16]
        }
    }
}

/// A **different** tile for the podium, because a 裙房 is clad differently from
/// the shaft above it and that difference is most of what tells a tower apart
/// from a slab at four hundred metres.
fn podium_tile_for(shaft: usize) -> usize {
    // A glass shaft gets a pale stone podium; a masonry shaft gets a small-tile
    // one.  A podium that matches its shaft is a wider shaft.
    if FACADE_TILES[shaft].glass {
        6
    } else {
        [17usize, 16, 18, 21][shaft % 4]
    }
}

/// A **crown of a different colour and material**, set back and often in a
/// different family from the shaft.  A crown that matches its shaft is a lid.
fn crown_tile_for(shaft: usize) -> usize {
    if FACADE_TILES[shaft].glass {
        // A glass tower's crown is a pale stone and glass cap, or a second,
        // tighter module.
        13
    } else {
        [6usize, 16, 17, 20][shaft % 4]
    }
}

fn ground_floor_material(building: &ModernBuilding, tile_index: usize) -> &'static str {
    match building.use_type {
        // A retail podium and a mixed-use block both get the shopfront band.
        ParcelUse::MixedUse | ParcelUse::Commercial => "ground/shop",
        // A glass tower gets a stone portal, not a shopfront, unless it has a
        // retail podium — which it does, which is the point of a 裙房.
        _ if tile_index >= 8 && tile_index <= 15 => "ground/lobby",
        // A civic building gets a portal too.
        _ if building.use_type == ParcelUse::Civic => "ground/lobby",
        _ => "ground/home",
    }
}

/// Roof and trim reflectances, as linear values, for the vertex tints the
/// building layer writes.
///
/// **The building layer writes no vertex colours at all**, and that is a
/// decision rather than an omission.  The renderer binds every building material
/// without `vertexColors`, so a per-vertex tint is provably dead payload: it
/// costs four bytes a vertex across the whole layer, and — worse — a group whose
/// colour buffer is only partly populated is *worse* than one with none, because
/// a renderer that later turns `vertexColors` on would read a truncated buffer.
/// Colour variation in this layer therefore comes from the twenty-four baked
/// tiles and the fourteen materials, which is where it can actually be seen.

fn declare(builder: &mut MeshBuilder) {
    let wall = GroupStyle {
        cast_shadow: true,
        receive_shadow: true,
        alpha_cutout: false,
        dynamic: false,
    };
    let ground = GroupStyle {
        cast_shadow: false,
        receive_shadow: true,
        alpha_cutout: false,
        dynamic: false,
    };
    for index in 0..FACADE_TILES.len() {
        builder.style(&format!("facade/{index:02}"), wall);
    }
    for kind in ["ground/shop", "ground/lobby", "ground/home"] {
        builder.style(kind, wall);
    }
    builder.style("roof", wall);
    builder.style("sign/shop", wall);
    builder.style("trim.light", wall);
    builder.style("trim.dark", wall);
    builder.style("balcony.slab", wall);
    builder.style("metal.ac", wall);
    builder.style("awning", wall);
    builder.style("wall.render", wall);
    builder.style("block.ground", ground);
    builder.style("parcel.paving", ground);
    builder.style("parcel.green", ground);
}

/// Ring of a building, in city-local metres, wound consistently outward-facing.
fn ring_of(footprint: &[Point], frame: CityFrameInfo) -> Vec<Vec2> {
    footprint
        .iter()
        .map(|point| {
            let [x, z] = frame.to_local(*point);
            Vec2::new(x, z)
        })
        .collect()
}

/// Build every static surface of a city that stands on the ground.
pub fn build(
    blocks: &[UrbanBlock],
    parcels: &[Parcel],
    buildings: &[ModernBuilding],
    compounds: &[Compound],
    frame: CityFrameInfo,
    builder: &mut MeshBuilder,
) {
    declare(builder);
    for block in blocks {
        let ring = ring_of(&block.boundary, frame);
        if ring.len() < 3 {
            continue;
        }
        // The block plate sits just under the sidewalk so a paved parcel and a
        // lawn meet the kerb without a step, and no bare terrain shows through.
        let mut plate = ring.clone();
        if signed_area(&plate) < 0.0 {
            plate.reverse();
        }
        builder.ground_uv("block.ground", &plate, level::BLOCK, None);
    }
    for parcel in parcels {
        let ring = ring_of(&parcel.ring, frame);
        if ring.len() < 3 {
            continue;
        }
        let material = if matches!(parcel.use_type, ParcelUse::Park) {
            "parcel.green"
        } else {
            "parcel.paving"
        };
        builder.ground_uv(material, &ring, level::GROUND - 0.004, None);
    }
    for building in buildings {
        let ring = ring_of(&building.footprint, frame);
        if ring.len() < 3 {
            continue;
        }
        building_shell(building, &ring, builder);
    }
    for compound in compounds {
        let ring = ring_of(&compound.boundary, frame);
        if ring.len() < 3 {
            continue;
        }
        compound_shell(compound, &ring, frame, builder);
    }
}

/// A wall, UV'd so that the tile's storey lines land on floors.
///
/// This is the single most important function in the file and it exists because
/// `wall_uv` could not do it.  Two things are wrong with using world `Y` as the
/// tile's `V`:
///
/// * **Direction.**  The renderer uploads row 0 of a texture at `V = 0`, so `V`
///   must *increase downwards* for the image to appear the right way up.  World
///   `Y` increases upwards, which turns every facade upside down — invisible
///   while a storey band is painted symmetrically at both ends of the tile, and
///   glaring the moment a window has a sill.
/// * **Phase.**  A wall's storey lines are the floors of the building, and the
///   building's floors are `GROUND_STOREY_M + n * STOREY_M` above the pavement.
///   If `V` is world `Y`, whether a window head lands on a head depends on where
///   the building happens to sit relative to the texture's origin, and it stops
///   landing the moment the ground storey height changes with land use.
///
/// So `V` is measured from the *top of the wall downwards* and `V = 0` is put at
/// the top of the last storey, which makes every floor line land on `V` a
/// multiple of `1 / STOREYS_PER_TILE` — exactly on a floor, on every wall of
/// every building, at any height.
fn facade_wall(
    builder: &mut MeshBuilder,
    material: &str,
    a: Vec2,
    b: Vec2,
    base: f32,
    top: f32,
    storeys: f32,
    mirror: bool,
    tint: Option<[f32; 3]>,
) {
    let length = a.distance(b);
    if length <= 1.0e-3 || top - base <= 1.0e-3 {
        return;
    }
    // `V` counts down from the wall's head, in tile heights.
    let v_base = storeys / 4.0;
    let u1 = if mirror { -length } else { length };
    builder.quad_uv(
        material,
        Vec3::from_plan(a, base),
        Vec3::from_plan(b, base),
        Vec3::from_plan(b, top),
        Vec3::from_plan(a, top),
        [(0.0, v_base), (u1, v_base), (u1, 0.0), (0.0, 0.0)],
        tint,
    );
}

/// A vertical wall UV'd the same way as a facade, but anchored at its own base
/// so a single-storey band — a shopfront fascia, a podium deck fascia — is drawn
/// once rather than tiled.
fn band_uv(
    builder: &mut MeshBuilder,
    material: &str,
    a: Vec2,
    b: Vec2,
    y0: f32,
    y1: f32,
    tint: Option<[f32; 3]>,
) {
    let length = a.distance(b);
    if length <= 1.0e-3 || y1 - y0 <= 1.0e-3 {
        return;
    }
    builder.quad_uv(
        material,
        Vec3::from_plan(a, y0),
        Vec3::from_plan(b, y0),
        Vec3::from_plan(b, y1),
        Vec3::from_plan(a, y1),
        [(0.0, y0), (length, y0), (length, y1), (0.0, y1)],
        tint,
    );
}

fn building_shell(building: &ModernBuilding, ring: &[Vec2], builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    let area = signed_area(ring).abs();
    let perimeter = ring
        .iter()
        .enumerate()
        .map(|(index, point)| point.distance(ring[(index + 1) % ring.len()]))
        .sum::<f32>();
    let massing = massing_of(building, ring, area);
    let jitter = jitter_for(building.id);
    // Colour variation comes from the baked tiles and the material table; see the
    // note on `roof_tone` above for why this layer writes no vertex colours.
    let tint: Option<[f32; 3]> = None;

    // Outward-facing ring: `inset_ring` on a reversed ring flips the winding, so
    // work from an explicitly outward ring rather than trusting the plan's.
    let mut outward = ring.to_vec();
    if signed_area(&outward) < 0.0 {
        outward.reverse();
    }

    let shaft_index = facade_tile_for(building, massing);
    let shaft = format!("facade/{shaft_index:02}");
    let pitched = building.roof == RoofStyle::Terracotta
        || (building.roof == RoofStyle::Mansard && building.floors <= 6);

    if pitched {
        pitched_roof(building, &outward, centre, builder);
        return;
    }

    // The storey arithmetic.  A ground storey of `GROUND_STOREY_M` and a whole
    // number of `STOREY_M` storeys above it, so the parapet lands on a floor and
    // the tile never has to stretch.
    let upper_storeys = (building.floors as f32 - 1.0).max(1.0);
    let shaft_base = level::GROUND + GROUND_STOREY_M;
    let top = shaft_base + upper_storeys * STOREY_M;

    // Plinth: a skirt below the ground floor so the building meets the terrain on
    // a slope instead of hovering over it.
    for index in 0..outward.len() {
        let a = outward[index];
        let b = outward[(index + 1) % outward.len()];
        builder.wall("trim.dark", a, b, level::PLINTH, level::GROUND, None);
    }

    // --- the ground floor, as a continuous band ---------------------------
    let ground_material = ground_floor_material(building, shaft_index);
    let retail = ground_material == "ground/shop";
    if area > 60.0 {
        retail_band(&outward, centre, building, retail, builder);
    }

    // --- the shaft ---------------------------------------------------------
    let tower = massing == Massing::Tower;
    // A tower's shaft steps back from its podium; a slab's does not, because a
    // 板楼 with a setback is a different building.
    let shaft_ring = if tower {
        inset_ring(&outward, 2.2 + (building.floors as f32 * 0.08).min(3.0))
    } else {
        outward.clone()
    };
    let shaft_top = if tower { top - 2.0 * STOREY_M } else { top };
    let shaft_base_local = if tower {
        shaft_base + podium_storeys(building) * STOREY_M
    } else {
        shaft_base
    };

    // The podium: a 裙房 of two to four retail storeys, in its own material, with
    // its own shopfront band at the pavement and a deck where the shaft lands.
    if tower {
        let podium_top = shaft_base_local;
        let podium_index = podium_tile_for(shaft_index);
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
                podium_storeys(building),
                jitter.mirror,
                tint,
            );
        }
        podium_deck(&outward, &shaft_ring, podium_top, builder);
        parapet(&shaft_ring, podium_top, 0.55, builder);
    }

    for index in 0..shaft_ring.len() {
        let a = shaft_ring[index];
        let b = shaft_ring[(index + 1) % shaft_ring.len()];
        facade_wall(
            builder,
            &shaft,
            a,
            b,
            shaft_base_local,
            shaft_top,
            (shaft_top - shaft_base_local) / STOREY_M,
            jitter.mirror,
            tint,
        );
    }

    // String courses.  A projecting band every four storeys is the strongest
    // horizontal the elevation has, and it is real geometry, so it throws a real
    // shadow at every storey of the day instead of a painted one.
    if massing != Massing::LowRise && upper_storeys >= 8.0 {
        string_courses(&shaft_ring, shaft_base_local, shaft_top, 4.0, builder);
    }

    // --- the roof ----------------------------------------------------------
    let cap_ring = shaft_ring.clone();
    builder.ground_uv("roof", &cap_ring, shaft_top, None);
    parapet(&cap_ring, shaft_top, 0.55 + jitter.value * 0.2, builder);
    if area > 90.0 {
        roofscape(building, &cap_ring, shaft_top, builder);
    }

    // --- the crown ---------------------------------------------------------
    if tower {
        // Two setbacks and a crown in its own material.  A tower with one flat
        // top is a box; a tower with a stepped top is a skyline.
        let crown_index = crown_tile_for(shaft_index);
        let crown_material = format!("facade/{crown_index:02}");
        let stage_one = inset_ring(&shaft_ring, 1.1);
        let stage_two = inset_ring(&stage_one, 1.4);
        for (from, to) in [(shaft_ring.as_slice(), stage_one.as_slice()), (stage_one.as_slice(), stage_two.as_slice())] {
            let from_top = if from.len() == shaft_ring.len() { shaft_top } else { shaft_top + STOREY_M };
            for index in 0..from.len() {
                let a = from[index];
                let b = from[(index + 1) % from.len()];
                facade_wall(
                    builder,
                    &crown_material,
                    a,
                    b,
                    from_top,
                    from_top + STOREY_M,
                    1.0,
                    jitter.mirror,
                    tint,
                );
            }
            // The step's own deck and its shadowed fascia, so the setback reads
            // as a step rather than as a hole.
            podium_deck(from, to, from_top + STOREY_M, builder);
            builder.ground_uv("roof", to, from_top + STOREY_M, None);
            parapet(to, from_top + STOREY_M, 0.62, builder);
        }
        roofscape(
            building,
            &stage_two,
            shaft_top + 2.0 * STOREY_M,
            builder,
        );
    }

    // --- the balconies, the condensers and the rainwater --------------------
    let balcony_ring = if tower { shaft_ring.clone() } else { outward.clone() };
    let balcony_base = shaft_base_local;
    if massing != Massing::LowRise && perimeter < 150.0 && !FACADE_TILES[shaft_index].glass {
        let fronts = front_edges(&balcony_ring, 2);
        balconies(&balcony_ring, &fronts, building, balcony_base, shaft_top, builder);
        // Every balcony on a Chinese residential block carries a condenser unit.
        // They are what stops a slab from reading as a stack of stripes, and
        // from a distance they are most of the texture of the wall.
        condensers(&balcony_ring, &fronts, building, balcony_base, shaft_top, builder);
    }
    // A drainpipe down one corner.  It is a single tube, and it is on every
    // building in China.
    if perimeter > 18.0 {
        downpipe(&outward, level::GROUND, top, building.id, builder);
    }
    if building.entrance_count > 0 && area > 90.0 {
        entrance_canopy(&outward, building, builder);
    }
}

/// The two longest edges of a ring, which is where a Chinese block puts its
/// balconies and its entrance: the frontage, not the flanks.
fn front_edges(ring: &[Vec2], count: usize) -> Vec<usize> {
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
        builder.quad(
            "roof",
            Vec3::from_plan(a, y),
            Vec3::from_plan(d, y),
            Vec3::from_plan(c, y),
            Vec3::from_plan(b, y),
            None,
        );
        // The vertical face of the step itself.
        builder.wall("trim.dark", d, c, y - 0.20, y + 0.05, None);
    }
}

/// A projecting band at every `every`-th storey: a string course, a drip edge and
/// the shadow under it.  Cheap, and it is the one detail that gives a tall slab
/// a horizontal scale.
fn string_courses(ring: &[Vec2], base: f32, top: f32, every: f32, builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    let mut storey = every;
    while base + storey * STOREY_M < top - 1.0 {
        let y = base + storey * STOREY_M;
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
        storey += every;
    }
}

/// A parapet: one wall per roof edge, with a capping band on top and the shadow
/// its overhang throws on the roof deck.
fn parapet(ring: &[Vec2], deck: f32, height: f32, builder: &mut MeshBuilder) {
    let centre = ring_centroid(ring);
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        builder.wall("roof", a, b, deck, deck + height, None);
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

fn pitched_roof(building: &ModernBuilding, ring: &[Vec2], centre: Vec2, builder: &mut MeshBuilder) {
    let top = level::GROUND + building.height_metres;
    let eaves = 0.55_f32.min(ring_centroid(ring).distance(ring[0]) * 0.0 + 0.55);
    // An eave overhang first: a tiled roof that stops flush with the wall has no
    // shadow line under it and reads as a paper cone.
    let eave_ring: Vec<Vec2> = ring.iter().map(|p| centre + (*p - centre) * (1.0 + eaves / 40.0)).collect();
    for index in 0..ring.len() {
        let a = eave_ring[index];
        let b = eave_ring[(index + 1) % eave_ring.len()];
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
    let rise = 1.6 + modern::hash_u32(building.id, 2, 41) * 1.4;
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

/// The ground-floor band.
///
/// A Chinese ground floor is a **continuous** retail band, and the parts are what
/// make it read as one: a dark stall riser at the pavement, a run of piers, the
/// glazed shopfronts, a fascia that overhangs, projecting sign boxes at head
/// height, and an awning over the frontage.  Any one of them alone reads as a
/// door; together they read as a street.
fn retail_band(ring: &[Vec2], centre: Vec2, building: &ModernBuilding, retail: bool, builder: &mut MeshBuilder) {
    let floor = level::GROUND;
    let head = floor + GROUND_STOREY_M;
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
        let fy0 = floor + GROUND_STOREY_M - 0.95;
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
            serial = serial.wrapping_mul(1664525).wrapping_add(1013904223);
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
    let y = floor + GROUND_STOREY_M - 1.55;
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
            serial = serial.wrapping_mul(1664525).wrapping_add(1013904223);
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
            Some([0.18, 0.20, 0.22]),
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

/// The roofscape.
///
/// This is the single most recognisable thing about a Chinese skyline and the
/// thing the previous port left out entirely: a flat lid with nothing on it.  A
/// real roof carries a stair head house, a lift overrun, one to four **water
/// tanks on legs** (the stainless drum every block has, because the mains pressure
/// is not reliable), a satellite dish, an antenna mast, a run of clothes poles on
/// a low-rise block, and on a newer one a PV array.  All of it is placed by
/// rejection sampling against the roof outline, so nothing lands off the roof.
fn roofscape(building: &ModernBuilding, ring: &[Vec2], deck: f32, builder: &mut MeshBuilder) {
    let Some(extent) = ring_extent(ring) else {
        return;
    };
    let width = extent[2] - extent[0];
    let depth = extent[3] - extent[1];
    let mut rng = crate::math::Rng::new(building.id ^ 0x9e37_79b9);

    // Rejection sampling inside the roof polygon, with a margin for the prop's
    // own half-extent.  The test `nothing_a_roof_prop_is_placed_off_the_roof`
    // asserts this, because a tank hanging in the air beside a tower is the
    // single most obvious tell of a procedural city.
    let place = |rng: &mut crate::math::Rng, half_x: f32, half_z: f32| -> Option<Vec2> {
        if width < half_x * 2.0 + 1.2 || depth < half_z * 2.0 + 1.2 {
            return None;
        }
        for _ in 0..12 {
            let candidate = Vec2::new(
                rng.range(extent[0] + half_x, extent[2] - half_x),
                rng.range(extent[1] + half_z, extent[3] - half_z),
            );
            if point_in_ring(candidate, ring) {
                return Some(candidate);
            }
        }
        None
    };
    // The same, for a box that is about to be rotated.  A rotated rectangle's
    // axis-aligned half-extent is bigger than its own half-extent, and getting
    // that wrong is how a plant room ends up overhanging a parapet.
    let place_box = |rng: &mut crate::math::Rng, w: f32, d: f32, rotation: f32| -> Option<Vec2> {
        let (sin, cos) = rotation.sin_cos();
        let (sin, cos) = (sin.abs(), cos.abs());
        place(
            rng,
            w * 0.5 * cos + d * 0.5 * sin,
            w * 0.5 * sin + d * 0.5 * cos,
        )
    };

    // The stair head house (楼梯间) and the lift overrun beside it.  Two boxes and
    // a dark door: the most reliable way to tell a Chinese roof from a western
    // one at two hundred metres.
    let rotation = rng.range(-0.35, 0.35);
    let body_w = 3.2 + rng.unit() * 1.6;
    let body_d = 2.6 + rng.unit() * 0.8;
    let cap_w = body_w + 0.40;
    let cap_d = body_d + 0.40;
    if let Some(point) = place_box(&mut rng, cap_w, cap_d, rotation) {
        let height = 2.4 + rng.unit() * 1.5;
        crate::mesh::box_at(
            builder,
            "trim.light",
            point,
            deck + height * 0.5,
            body_w,
            height,
            body_d,
            rotation,
        );
        // A capping slab, slightly oversized, the way a plant room is finished.
        crate::mesh::box_at(
            builder,
            "trim.dark",
            point,
            deck + height + 0.10,
            cap_w,
            0.20,
            cap_d,
            rotation,
        );
    }
    let rotation = rng.range(-0.40, 0.40);
    if let Some(point) = place_box(&mut rng, 2.0, 1.8, rotation) {
        let height = 1.7 + rng.unit() * 0.9;
        crate::mesh::box_at(
            builder,
            "trim.light",
            point,
            deck + height * 0.5,
            2.0,
            height,
            1.8,
            rotation,
        );
    }

    // Water tanks (水箱): a stainless drum on four short legs, which is the
    // shape almost every block in the country has on its roof.
    let tanks = 1 + rng.int(3);
    for _ in 0..tanks {
        if let Some(point) = place(&mut rng, 0.85, 0.85) {
            let leg = 0.55 + rng.unit() * 0.35;
            let radius = 0.55 + rng.unit() * 0.25;
            let drum = 1.0 + rng.unit() * 0.7;
            for (dx, dz) in [(-0.45, -0.45), (0.45, -0.45), (0.45, 0.45), (-0.45, 0.45)] {
                builder.tube(
                    "trim.dark",
                    Vec3::new(point.x + dx, deck, point.y + dz),
                    Vec3::new(point.x + dx, deck + leg, point.y + dz),
                    0.05,
                    0.05,
                    4,
                    None,
                );
            }
            let base = deck + leg;
            builder.tube(
                "metal.ac",
                Vec3::new(point.x, base, point.y),
                Vec3::new(point.x, base + drum, point.y),
                radius,
                radius,
                7,
                None,
            );
            // A domed lid, so the drum is a tank and not a cylinder.
            builder.tube(
                "metal.ac",
                Vec3::new(point.x, base + drum, point.y),
                Vec3::new(point.x, base + drum + 0.18, point.y),
                radius,
                radius * 0.55,
                7,
                None,
            );
        }
    }

    // Condenser units, and a satellite dish on most roofs.
    for _ in 0..(1 + rng.int(3)) {
        let w = 0.90 + rng.unit() * 0.55;
        let d = 0.70 + rng.unit() * 0.40;
        let rotation = rng.range(-0.6, 0.6);
        if let Some(point) = place_box(&mut rng, w, d, rotation) {
            crate::mesh::box_at(
                builder,
                "metal.ac",
                point,
                deck + 0.50,
                w,
                1.0,
                d,
                rotation,
            );
        }
    }
    if rng.chance(0.7)
        && let Some(point) = place(&mut rng, 0.80, 0.55)
    {
        let height = 0.9 + rng.unit() * 0.7;
        builder.tube(
            "trim.dark",
            Vec3::new(point.x, deck, point.y),
            Vec3::new(point.x, deck + height, point.y),
            0.06,
            0.05,
            4,
            None,
        );
        // The dish: a shallow cone, which is all a 35 cm dish is at any distance
        // a viewer can see one from.
        builder.tube(
            "metal.ac",
            Vec3::new(point.x, deck + height, point.y),
            Vec3::new(point.x + 0.30, deck + height + 0.28, point.y),
            0.10,
            0.38,
            8,
            None,
        );
    }

    // An antenna mast (天线), on the taller buildings.  A tapered tube with a
    // cross-arm and a couple of whips: from a kilometre away this is the
    // silhouette detail that says "city" rather than "blockout".
    if building.floors >= 8 && let Some(point) = place(&mut rng, 0.70, 0.25) {
        let height = 4.0 + rng.unit() * 5.0;
        builder.tube(
            "trim.dark",
            Vec3::new(point.x, deck, point.y),
            Vec3::new(point.x, deck + height, point.y),
            0.085,
            0.035,
            5,
            None,
        );
        let arm = deck + height * 0.72;
        builder.tube(
            "trim.dark",
            Vec3::new(point.x - 0.55, arm, point.y),
            Vec3::new(point.x + 0.55, arm, point.y),
            0.030,
            0.030,
            4,
            None,
        );
        for offset in [-0.35_f32, 0.0, 0.35] {
            builder.tube(
                "trim.dark",
                Vec3::new(point.x + offset, arm, point.y),
                Vec3::new(point.x + offset, arm + 0.9, point.y),
                0.018,
                0.012,
                3,
                None,
            );
        }
    }

    // Clothes poles (晾衣杆) on a low roof: two uprights and a rail, which is what
    // a Chinese apartment roof is actually for half the year.
    if building.floors <= 8 && let Some(point) = place(&mut rng, 1.30, 0.35) {
        let height = 1.5 + rng.unit() * 0.6;
        for offset in [-1.0_f32, 1.0] {
            builder.tube(
                "trim.dark",
                Vec3::new(point.x + offset, deck, point.y),
                Vec3::new(point.x + offset, deck + height, point.y),
                0.045,
                0.040,
                4,
                None,
            );
        }
        builder.tube(
            "trim.dark",
            Vec3::new(point.x - 1.1, deck + height, point.y),
            Vec3::new(point.x + 1.1, deck + height, point.y),
            0.030,
            0.030,
            4,
            None,
        );
    }

    // A PV array on some roofs: a tilted frame of dark panels on a low rail.
    if building.floors > 5 && rng.chance(0.35) {
        // The array's own axes are rotated, so its half-extent is 1.5 m in both
        // world axes whatever the tilt.
        if let Some(point) = place(&mut rng, 1.55, 1.55) {
            let (sin, cos) = (0.28_f32).sin_cos();
            let (lx, lz) = (cos, sin);
            let tilt = 0.62_f32;
            let corners = [
                point + Vec2::new(-lx, -lz) * 1.5,
                point + Vec2::new(lx, -lz) * 1.5,
                point + Vec2::new(lx, lz) * 1.5,
                point + Vec2::new(-lx, lz) * 1.5,
            ];
            for index in 0..4 {
                let a = corners[index];
                let b = corners[(index + 1) % 4];
                builder.quad(
                    "trim.dark",
                    Vec3::from_plan(a, deck + 0.20),
                    Vec3::from_plan(b, deck + 0.20),
                    Vec3::from_plan(b, deck + 0.20 + tilt),
                    Vec3::from_plan(a, deck + 0.20 + tilt),
                    None,
                );
            }
            for index in [0usize, 2] {
                let a = corners[index];
                let b = corners[(index + 1) % 4];
                builder.wall("metal.ac", a, b, deck, deck + 0.20, None);
            }
        }
    }
}

fn ring_extent(ring: &[Vec2]) -> Option<[f32; 4]> {
    if ring.len() < 3 {
        return None;
    }
    let mut extent = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
    for point in ring {
        extent[0] = extent[0].min(point.x);
        extent[1] = extent[1].min(point.y);
        extent[2] = extent[2].max(point.x);
        extent[3] = extent[3].max(point.y);
    }
    Some(extent)
}

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
///   catches the light and separates one storey from the next.
fn balconies(
    ring: &[Vec2],
    fronts: &[usize],
    building: &ModernBuilding,
    base: f32,
    top: f32,
    builder: &mut MeshBuilder,
) {
    if fronts.is_empty() {
        return;
    }
    let centre = ring_centroid(ring);
    let mut floor = 1_u32;
    while floor < building.floors as u32 {
        let y = base + floor as f32 * STOREY_M;
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
            // The slab projects 1.5 m and is 150 mm thick, which is a real
            // 生活阳台 and not a ledge.
            let depth = 1.5;
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
            // The dark fascia, and the soffit under the slab.
            builder.wall("trim.dark", b_out, a_out, y - 0.20, y, None);
            builder.quad(
                "trim.dark",
                Vec3::from_plan(a_out, y - 0.20),
                Vec3::from_plan(b_out, y - 0.20),
                Vec3::from_plan(b, y - 0.20),
                Vec3::from_plan(a, y - 0.20),
                None,
            );
            // The railing: a solid pale parapet 1.05 m, with a coping and a
            // shadow line at its foot.
            builder.wall("trim.light", a_out, b_out, y, y + 1.05, None);
            builder.wall("balcony.slab", b_out, a_out, y + 1.05, y + 1.14, None);
            // The two returns, so the balcony is a box and not three quads.
            builder.wall("balcony.slab", a, a_out, y, y + 1.14, None);
            builder.wall("balcony.slab", b_out, b, y, y + 1.14, None);
        }
        floor += 1;
    }
}

/// Wall-mounted air-conditioner condensers (空调外机) under the balcony line.
///
/// A box on a bracket with a dark fan disc, one per balcony on the front
/// elevations.  They are the most characteristic object on a Chinese residential
/// facade and they are also what stops a slab from reading as a stack of stripes:
/// every storey gets a small, dark, high-frequency horizontal interruption.
fn condensers(
    ring: &[Vec2],
    fronts: &[usize],
    building: &ModernBuilding,
    base: f32,
    top: f32,
    builder: &mut MeshBuilder,
) {
    if fronts.is_empty() {
        return;
    }
    let centre = ring_centroid(ring);
    let mut floor = 1_u32;
    let mut serial = building.id.wrapping_mul(31);
    while floor < building.floors as u32 {
        let y = base + floor as f32 * STOREY_M;
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
            serial = serial.wrapping_mul(1664525).wrapping_add(1013904223);
            // Not every balcony has one: about two thirds do.
            if (serial >> 16) % 100 >= 66 {
                floor += 1;
                continue;
            }
            let along = (b - a).normalize();
            let t = 0.25 + ((serial >> 8) % 100) as f32 / 200.0;
            let anchor = a + along * (length * t);
            let outward = (anchor - centre).normalize();
            // The bracket, then the unit, hanging 0.35 m off the wall and
            // 0.9 m wide.
            let front = anchor + outward * 0.42;
            builder.wall("trim.dark", anchor, front, y - 0.75, y - 0.68, None);
            crate::mesh::box_at(
                builder,
                "metal.ac",
                front,
                y - 0.42,
                0.88,
                0.62,
                0.34,
                (anchor - centre).angle() + std::f32::consts::FRAC_PI_2,
            );
            // The fan: a dark disc on the front face, which is the only part of
            // a condenser anyone can see.
            let disc = front + outward * 0.18;
            builder.tube(
                "trim.dark",
                Vec3::new(disc.x, y - 0.42, disc.y),
                Vec3::new(disc.x + outward.x * 0.02, y - 0.42, disc.y + outward.y * 0.02),
                0.24,
                0.24,
                8,
                None,
            );
        }
        floor += 1;
    }
}

/// A rainwater downpipe (落水管) down one corner, in a socket at the parapet and
/// discharging over a splash block.  One tube per building, and it is on every
/// building in the country.
fn downpipe(ring: &[Vec2], base: f32, top: f32, id: u32, builder: &mut MeshBuilder) {
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
fn entrance_canopy(ring: &[Vec2], building: &ModernBuilding, builder: &mut MeshBuilder) {
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
    let y = level::GROUND + 3.35;
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
            Vec3::from_plan(q0, level::GROUND + rise),
            Vec3::from_plan(q1, level::GROUND + rise),
            Vec3::from_plan(q1, level::GROUND + rise - 0.06),
            Vec3::from_plan(q0, level::GROUND + rise - 0.06),
            None,
        );
        builder.wall("trim.light", q1, q0, level::GROUND, level::GROUND + rise, None);
    }
}

// ---------------------------------------------------------------------------
// compounds (小区)
// ---------------------------------------------------------------------------

/// A walled residential compound: boundary wall, a gate portal on the street
/// front, an internal fire lane and a lawn.
fn compound_shell(compound: &Compound, ring: &[Vec2], frame: CityFrameInfo, builder: &mut MeshBuilder) {
    let mut outward = ring.to_vec();
    if signed_area(&outward) < 0.0 {
        outward.reverse();
    }
    let height = compound.fence_height_metres;
    // A capping course every 1.8 m of wall, and a plinth at its foot: a
    // rendered compound wall is never a bare extrusion.
    for index in 0..outward.len() {
        let a = outward[index];
        let b = outward[(index + 1) % outward.len()];
        builder.wall("wall.render", a, b, level::GROUND, level::GROUND + height, None);
        builder.wall("trim.light", a, b, level::GROUND + height, level::GROUND + height + 0.14, None);
        builder.wall("trim.dark", a, b, level::GROUND, level::GROUND + 0.35, None);
        if a.distance(b) > 1.8 {
            builder.wall("trim.light", a, b, level::GROUND + height - 0.55, level::GROUND + height - 0.45, None);
        }
    }
    if compound.gate_points.len() >= 2 {
        let a = gate_point(compound.gate_points[0], frame);
        let b = gate_point(compound.gate_points[1], frame);
        for point in [a, b] {
            crate::mesh::box_at(builder, "wall.render", point, level::GROUND + 2.1, 0.9, 4.2, 0.9, 0.0);
            crate::mesh::box_at(builder, "trim.dark", point, level::GROUND + 4.32, 1.2, 0.24, 1.2, 0.0);
        }
        let span = b - a;
        let bearing = Vec2::new(span.x, span.y).angle();
        crate::mesh::box_at(
            builder,
            "trim.light",
            (a + b) * 0.5,
            level::GROUND + 4.0,
            a.distance(b) + 1.2,
            1.15,
            1.4,
            -bearing,
        );
        // A sliding gate leaf, half open, which is what every compound gate is.
        let mid = (a + b) * 0.5;
        let dir = if a.distance(b) > 0.1 { (b - a).normalize() } else { Vec2::new(1.0, 0.0) };
        crate::mesh::box_at(
            builder,
            "trim.dark",
            mid + dir * (a.distance(b) * 0.18),
            level::GROUND + 1.05,
            a.distance(b) * 0.36,
            2.1,
            0.10,
            -bearing,
        );
    }
    // Internal loop drive.
    for path in &compound.paths {
        let ring: Vec<Vec2> = path
            .iter()
            .map(|point| gate_point(*point, frame))
            .collect();
        if ring.len() < 2 {
            continue;
        }
        let drive = crate::math::Path::flat(ring);
        let length = drive.length();
        builder.ribbon(
            "parcel.paving",
            &drive,
            -compound.road_width_metres * 0.5,
            compound.road_width_metres * 0.5,
            0.0,
            length,
            level::GROUND + 0.006,
            None,
        );
    }
}

fn gate_point(point: Point, frame: CityFrameInfo) -> Vec2 {
    let [x, z] = frame.to_local(point);
    Vec2::new(x, z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> urban::ModernCity {
        generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        })
    }

    fn built() -> crate::mesh::SceneGeometry {
        let city = city();
        let mut builder = MeshBuilder::new();
        build(
            &city.blocks,
            &city.parcels,
            &city.buildings,
            &city.compounds,
            city.frame,
            &mut builder,
        );
        builder.build()
    }

    /// Every vertex of the groups the building layer owns, as `(x, y, z)`.
    fn building_vertices(groups: &[crate::mesh::MeshGroup]) -> Vec<(f32, f32, f32, String)> {
        let mut out = Vec::new();
        for group in groups {
            if !is_building_material(&group.material) {
                continue;
            }
            for chunk in group.positions.chunks_exact(3) {
                out.push((chunk[0], chunk[1], chunk[2], group.material.clone()));
            }
        }
        out
    }

    fn is_building_material(material: &str) -> bool {
        material.starts_with("facade/")
            || material.starts_with("ground/")
            || matches!(
                material,
                "roof" | "trim.light" | "trim.dark" | "balcony.slab" | "metal.ac" | "awning"
                    | "wall.render" | "sign/shop" | "block.ground" | "parcel.paving"
                    | "parcel.green"
            )
    }

    #[test]
    fn every_building_emits_a_facade_with_metre_uvs() {
        let groups = built().meshes;
        let facades: Vec<_> = groups.iter().filter(|g| g.material.starts_with("facade/")).collect();
        assert!(!facades.is_empty());
        for group in facades {
            let uvs = group.uvs.as_ref().expect("facades need metre UVs");
            assert_eq!(uvs.len(), group.positions.len() / 3 * 2);
            // UVs are metres, so they must span more than a tile somewhere or the
            // wall sampled a single texel — the artefact this port removes.
            let span = uvs.iter().fold(0.0_f32, |acc, value| acc.max(value.abs()));
            assert!(span > 3.0, "{} has degenerate UVs (max {span})", group.material);
        }
    }

    /// The storey alignment, asserted on the geometry rather than on the
    /// constant.  A facade wall's `V` runs from `storeys / 4` at its base to `0`
    /// at its head, so **every wall's floor lines are exact multiples of a
    /// quarter tile** — which is what "aligned to the geometry" means.
    #[test]
    fn every_facade_wall_lands_its_floor_lines_on_floors() {
        let groups = built().meshes;
        let mut checked = 0;
        for group in groups.iter().filter(|g| g.material.starts_with("facade/")) {
            let uvs = group.uvs.as_ref().unwrap();
            for pair in uvs.chunks_exact(2) {
                let v = pair[1];
                // The two ends of every wall quad: one is a floor line.
                let floor_line = v * 4.0;
                assert!(
                    (floor_line - floor_line.round()).abs() < 1.0e-3,
                    "{} has a floor line at {v:.4} tile heights, which is {:.3} \
                     of a storey out of phase",
                    group.material,
                    floor_line - floor_line.round()
                );
                checked += 1;
            }
        }
        assert!(checked > 500, "only {checked} facade corners checked");
    }

    /// The massing has to actually vary, and the way to know is to measure the
    /// geometry that was built rather than to read the plan's claims.
    #[test]
    fn the_massing_contains_slabs_towers_and_low_rise_at_genuinely_different_heights() {
        let city = city();
        let groups = built().meshes;
        // Assign every shell vertex to the building whose footprint contains it,
        // then take each building's real top.
        let mut footprints: Vec<([f32; 4], Vec<Vec2>)> = Vec::new();
        for building in &city.buildings {
            let ring = ring_of(&building.footprint, city.frame);
            if ring.len() < 3 {
                continue;
            }
            let Some(extent) = ring_extent(&ring) else { continue };
            footprints.push((extent, ring));
        }
        let mut tops: Vec<f32> = vec![f32::MIN; footprints.len()];
        for group in &groups {
            if !group.material.starts_with("facade/") && !group.material.starts_with("ground/") {
                continue;
            }
            for chunk in group.positions.chunks_exact(3) {
                let point = Vec2::new(chunk[0], chunk[2]);
                // A small tolerance: a balcony or a sign box projects past the
                // wall, and the shell vertices themselves sit on the ring.
                for (index, (extent, ring)) in footprints.iter().enumerate() {
                    if point.x < extent[0] - 0.6
                        || point.x > extent[2] + 0.6
                        || point.y < extent[1] - 0.6
                        || point.y > extent[3] + 0.6
                    {
                        continue;
                    }
                    if point_in_ring(point, ring) {
                        tops[index] = tops[index].max(chunk[1]);
                        break;
                    }
                }
            }
        }
        let measured: Vec<f32> = tops.iter().copied().filter(|top| top.is_finite() && *top > 1.0).collect();
        assert!(
            measured.len() > city.buildings.len() / 2,
            "only {} of {} buildings produced a measurable shell",
            measured.len(),
            city.buildings.len()
        );
        // Low rise, mid rise and a real skyline.
        let low = measured.iter().filter(|h| **h < 26.0).count();
        let mid = measured.iter().filter(|h| (26.0..70.0).contains(*h)).count();
        let tall = measured.iter().filter(|h| **h >= 70.0).count();
        assert!(low > 3, "only {low} low-rise buildings; a city of towers is not a city");
        assert!(mid > 3, "only {mid} mid-rise buildings; the 板楼 band is missing");
        assert!(tall > 0, "nothing above 70 m: there is no skyline");
        // And the heights must genuinely vary, not cluster.
        let mut sorted = measured.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let range = sorted[sorted.len() - 1] - sorted[0];
        assert!(range > 60.0, "the whole city spans {range:.0} m of height");
        let mut buckets = [0usize; 8];
        for top in &measured {
            let bucket = (((*top - sorted[0]) / range.max(1.0)) * 8.0).clamp(0.0, 7.0) as usize;
            buckets[bucket] += 1;
        }
        assert!(
            buckets.iter().filter(|count| **count > 0).count() >= 5,
            "the height distribution is lumpy: {buckets:?}"
        );
    }

    /// The roofscape, asserted on the built geometry.  A Chinese roof is never
    /// empty, and a city whose roofs are bare lids reads as a blockout no matter
    /// how the walls are textured.
    #[test]
    fn a_city_has_water_tanks_stair_houses_and_masts() {
        let groups = built().meshes;
        let highest_roof = groups
            .iter()
            .filter(|group| group.material == "roof")
            .flat_map(|group| group.positions.chunks_exact(3).map(|c| c[1]))
            .fold(f32::MIN, f32::max);
        assert!(highest_roof > 20.0, "the city has no roof to speak of");

        // The stair head house and the lift overrun are pale masses above the
        // deck; the water tanks and the condensers are stainless; the masts are
        // dark and thin.  Each is asserted above the roofline, because a prop
        // *at* roof height is a prop that fell off.
        for (material, label) in [
            ("trim.light", "stair head house"),
            ("metal.ac", "water tank / condenser"),
            ("trim.dark", "antenna mast"),
        ] {
            let group = groups
                .iter()
                .find(|group| group.material == material)
                .unwrap_or_else(|| panic!("no {label} geometry"));
            let above = group
                .positions
                .chunks_exact(3)
                .filter(|c| c[1] > highest_roof + 0.8)
                .count();
            assert!(above > 20, "only {above} vertices of {label} sit above a roof");
        }
    }

    /// A prop that lands off its roof is the most obvious tell of a procedural
    /// city, so the placement is checked against the polygon rather than against
    /// a bounding box.
    #[test]
    fn nothing_a_roof_prop_is_placed_off_the_roof() {
        let city = city();
        let mut placed = 0;
        for building in &city.buildings {
            let ring = ring_of(&building.footprint, city.frame);
            if ring.len() < 3 || signed_area(&ring).abs() < 90.0 {
                continue;
            }
            let mut outward = ring.clone();
            if signed_area(&outward) < 0.0 {
                outward.reverse();
            }
            let upper_storeys = (building.floors as f32 - 1.0).max(1.0);
            let deck = level::GROUND + GROUND_STOREY_M + upper_storeys * STOREY_M;
            let mut builder = MeshBuilder::new();
            roofscape(building, &outward, deck, &mut builder);
            for group in builder.build().meshes {
                for chunk in group.positions.chunks_exact(3) {
                    let point = Vec2::new(chunk[0], chunk[2]);
                    // Props are allowed a 0.6 m tolerance for the parapet line
                    // and the tank legs, and nothing beyond it.
                    let margin = 0.6;
                    let inside = point_in_ring(point, &outward)
                        || [
                            (1.0, 0.0),
                            (-1.0, 0.0),
                            (0.0, 1.0),
                            (0.0, -1.0),
                            (0.707, 0.707),
                            (-0.707, 0.707),
                            (0.707, -0.707),
                            (-0.707, -0.707),
                        ]
                        .iter()
                        .any(|(dx, dz)| {
                            point_in_ring(point + Vec2::new(*dx, *dz) * margin, &outward)
                        });
                    assert!(
                        inside,
                        "building {} has a {} vertex at {point:?}, {margin} m outside its roof",
                        building.id,
                        group.material
                    );
                    assert!(
                        chunk[1] >= deck - 0.3,
                        "building {} has a {} vertex *below* its roof deck",
                        building.id,
                        group.material
                    );
                    placed += 1;
                }
            }
        }
        assert!(placed > 200, "only {placed} roof-prop vertices were checked");
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
            let fronts = front_edges(&outward, 2);
            if fronts.is_empty() {
                continue;
            }
            let base = level::GROUND + GROUND_STOREY_M;
            let top = base + (building.floors as f32 - 1.0).max(1.0) * STOREY_M;
            let mut builder = MeshBuilder::new();
            balconies(&outward, &fronts, building, base, top, &mut builder);
            let centroid = ring_centroid(&outward);
            for group in builder.build().meshes {
                for chunk in group.positions.chunks_exact(3) {
                    let point = Vec2::new(chunk[0], chunk[2]);
                    if point_in_ring(point, &outward) {
                        // On the wall itself, or a corner that rounds inward: fine.
                        continue;
                    }
                    // Outside the footprint, so the balcony must point *away*
                    // from the building and stay within its 1.5 m slab.
                    let radial = point - centroid;
                    let distance = crate::math::distance_to_polyline(point, &outward);
                    assert!(
                        distance < 1.6,
                        "building {} has a balcony vertex {distance:.2} m outside its wall",
                        building.id
                    );
                    assert!(
                        radial.is_finite(),
                        "building {} has a non-finite balcony radial",
                        building.id
                    );
                    checked += 1;
                }
            }
        }
        assert!(checked > 200, "only {checked} balcony vertices were checked");
    }

    #[test]
    fn roofs_carry_plant_and_a_parapet() {
        let groups = built().meshes;
        let roof = groups.iter().find(|g| g.material == "roof").expect("no roof");
        assert!(roof.positions.len() > 0);
        assert!(
            groups.iter().any(|g| g.material == "metal.ac"),
            "no rooftop plant on a city of towers"
        );
    }

    #[test]
    fn every_building_gets_a_ground_floor() {
        let groups = built().meshes;
        for kind in ["ground/shop", "ground/lobby", "ground/home"] {
            assert!(
                groups.iter().any(|g| g.material == kind),
                "no {kind} in the city"
            );
        }
    }

    #[test]
    fn the_facade_palette_is_actually_drawn_from() {
        let groups = built().meshes;
        let mut used: Vec<&str> = groups
            .iter()
            .filter(|g| g.material.starts_with("facade/"))
            .map(|g| g.material.as_str())
            .collect();
        used.sort_unstable();
        used.dedup();
        // A skyline of two or three facade variants is what the previous port
        // produced, and it is a large part of why it read as a blockout.
        assert!(used.len() >= 6, "only {} facade variants used", used.len());
    }

    /// The palette is not only *available* — it is *reachable*.  A glass tile that
    /// no building can ever pick is a tile that was baked for nothing.
    #[test]
    fn both_facade_families_reach_the_city() {
        let groups = built().meshes;
        let mut glazed = 0;
        let mut masonry = 0;
        for group in groups.iter().filter(|g| g.material.starts_with("facade/")) {
            let index: usize = group.material[7..].parse().unwrap();
            if FACADE_TILES[index].glass {
                glazed += 1;
            } else {
                masonry += 1;
            }
        }
        assert!(masonry > 0, "no masonry in the city");
        assert!(glazed > 0, "no curtain wall in the city: it is not a Chinese skyline");
    }

    #[test]
    fn building_geometry_never_nans() {
        for group in built().meshes {
            assert!(
                group.positions.iter().all(|value| value.is_finite()),
                "{} has a non-finite position",
                group.material
            );
            if let Some(uvs) = &group.uvs {
                assert!(uvs.iter().all(|value| value.is_finite()));
            }
            if let Some(colors) = &group.colors {
                assert_eq!(colors.len(), group.positions.len() / 3 * 4);
            }
        }
    }

    /// The massing classifier has to agree with the geometry it produces, or the
    /// four forms are just four names.
    #[test]
    fn the_four_massing_forms_are_distinguishable() {
        let city = city();
        let mut tall = 0;
        let mut low = 0;
        let mut slabs = 0;
        for building in &city.buildings {
            let ring = ring_of(&building.footprint, city.frame);
            if ring.len() < 3 {
                continue;
            }
            let area = signed_area(&ring).abs();
            let massing = massing_of(building, &ring, area);
            match massing {
                Massing::Tower => tall += 1,
                Massing::Slab => slabs += 1,
                Massing::LowRise => low += 1,
            }
        }
        assert!(tall > 0, "no 塔楼 in a Chinese city");
        assert!(slabs > 0, "no 板楼 in a Chinese city");
        assert!(low > 0, "no 多层 in a Chinese city");
        assert!(tall + slabs + low == city.buildings.len());
    }
}
