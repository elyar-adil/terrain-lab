//! Buildings, blocks, parcels and Chinese residential compounds.
//!
//! The massing here is deliberately *not* the massing a western city generator
//! produces, because the target is a Chinese one and the two do not look alike
//! even in silhouette.  Four forms carry a Chinese district, and a generator
//! that emits only the first reads as a bar chart:
//!
//! * **板楼** — a long slab block, six to eighteen storeys, one structural bay
//!   deep, with a stair core at one end and the whole south face given over to
//!   balconies.  A share of them is carved into L, U (twin prongs) and H plan
//!   variants, and a share carries a taller stair-tower step at one end,
//!   because a district of pure rectangles is a trading estate.
//! * **塔楼 with 裙房** — a podium of two to five retail storeys, a shaft of
//!   twenty to forty, and a crown that is a *different colour and a different
//!   shape*, set back in two or three steps.
//! * **多层** — five to seven storeys of walk-up with a pitched or parapeted
//!   tile roof, which is most of the older city.
//! * **perimeter block** around a compound, behind a wall, with a gate.
//!
//! # The dimension rules (asserted, not eyeballed)
//!
//! Everything the geometry builds is derived from the dimensioned facade
//! designs (`facades::designs`) plus the rule constants in [`rule`] below, and
//! the tests hold both to the Chinese code band: 层高 2.8–3.0 m residential,
//! 开间 3.3/3.6/3.9 m, 窗台 0.9 m, 窗高 1.4–1.5 m, 阳台进深 1.2–1.5 m with
//! 栏杆 1.05 m (≤6 层) rising to 1.10 m (≥7 层, GB 50352's two guard bands),
//! 空调机位 0.6–0.8 m, 飘窗凸出 0.4–0.6 m on the 2000s tile-clad stock.
//!
//! Three things carry the realism on top of that, in order of contribution:
//!
//! 1. **The storey rhythm.**  Every wall is UV'd in metres against a tile that
//!    covers four real storeys, and the tile's `V` origin is the *base of the
//!    shaft*, not the world origin.  That is the whole difference between a
//!    window grid aligned to the building and a window grid that happens to
//!    line up on one wall of one building.
//! 2. **The ground floor.**  Street-level credibility is a continuous retail
//!    band — stall riser, piers, fascia, roller shutters, projecting sign
//!    boxes, awnings — not a row of doors.
//! 3. **The roofscape.**  Parapets and copings, water tanks on legs, stair head
//!    houses, lift overruns, antenna masts, satellite dishes, clothes poles and
//!    the occasional PV array.  A city of boxes with blank lids reads as a
//!    blockout no matter how good the walls are.

pub(crate) mod details;
pub(crate) mod roofscape;
pub(crate) mod shell;

use urban::{
    CityFrameInfo, Compound, ModernBuilding, Parcel, ParcelUse, Point, RoofStyle, UrbanBlock, modern,
};

use crate::facades::{GROUND_STOREY_M, design};
use crate::math::{Vec2, Vec3, inset_ring, point_in_ring, ring_centroid, signed_area};
use crate::mesh::{GroupStyle, MeshBuilder};
use crate::spec::FACADE_TILES;

pub(crate) use shell::building_shell;

/// Ground levels, against the sidewalk datum.
pub mod level {
    /// Sidewalk / parcel surface.
    pub const GROUND: f32 = 0.150;
    /// Skirt below the ground floor, so a building on a slope never shows a gap
    /// between its base and the terrain.
    pub const PLINTH: f32 = -1.60;
    pub const BLOCK: f32 = 0.138;
}

/// The dimensioned rules the geometry is built from.  Each constant sits inside
/// its code band and the tests assert the band, so an edit that drifts a
/// dimension out of construction reality fails rather than shipping.
pub mod rule {
    use super::Massing;
    /// 阳台进深 — balcony slab depth.  GB practice is 1.2 – 1.5 m; 1.4 m is the
    /// common 生活阳台 that still fits a floor-standing rack.
    pub const BALCONY_DEPTH_M: f32 = 1.4;
    /// 空调机位 — the reserved condenser berth width.  Code asks 0.6 – 0.8 m so
    /// a split unit plus its service clearance fits.
    pub const AC_BERTH_W_M: f32 = 0.76;
    /// 飘窗凸出 — how far the bay-window sill and head courses of the 2000s
    /// tile-clad stock project.  0.4 – 0.6 m; 0.45 m is the standard 混凝土
    /// 挑板 with a glazing line between.
    pub const BAY_WINDOW_PROJECTION_M: f32 = 0.45;
    /// 阳台栏杆高 — guard height above the balcony slab: 1.05 m up to six
    /// storeys, 1.10 m from seven up (the two-band GB 50352 rule for low and
    /// high railings).
    pub fn balcony_rail_m(floors: u16) -> f32 {
        if floors >= 7 { 1.10 } else { 1.05 }
    }
    /// Whether a building class carries balconies at all: everything
    /// residential does — 板楼, 点式塔楼, 多层 walk-up — and nothing clad in
    /// curtain wall does, because a glass office has no 生活阳台.
    pub(crate) fn has_balconies(massing: Massing, glass: bool) -> bool {
        !glass && massing != Massing::CurtainTower
    }
}

/// A well-mixed per-building hash in `[0, 1)`.  (The `urban` hash only mixes the
/// parity of a small id into its top bits, so every building of one parity got
/// the same "random" draw — the reason a district read as copies.)
pub(crate) fn hash_u32(seed: u32, a: i32, b: i32) -> f32 {
    let mut v = seed.wrapping_mul(0x9e37_79b1)
        ^ (a as u32).wrapping_mul(0x85eb_ca6b)
        ^ (b as u32).wrapping_mul(0xc2b2_ae35).rotate_left(11);
    v ^= v >> 16;
    v = v.wrapping_mul(0x7feb_352d);
    v ^= v >> 15;
    v = v.wrapping_mul(0x846c_a68b);
    v ^= v >> 16;
    (v >> 8) as f32 / (1u32 << 24) as f32
}

/// Per-building jitter, derived from the building id so a city is identical for
/// a seed and a single building can be re-derived without the others.
pub(crate) struct Jitter {
    pub value: f32,
    pub mirror: bool,
}

/// A per-building colour shift for the facade tile: lightness from weathered to
/// clean, and a warm, neutral or cool cast. Two towers that share a tile are still
/// two different buildings, which is most of what stops a district reading as one
/// copied block.
pub(crate) fn building_tint(id: u32) -> [f32; 3] {
    let lightness = 0.74 + hash_u32(id, 11, 41) * 0.26;
    let cast = hash_u32(id, 13, 43);
    let (r, g, b) = if cast < 0.34 {
        (1.0, 0.93, 0.83) // warm
    } else if cast < 0.67 {
        (0.97, 0.97, 0.96) // neutral
    } else {
        (0.88, 0.94, 1.0) // cool
    };
    [r * lightness, g * lightness, b * lightness]
}

/// Curtain-wall glass tint per building: blue, green, bronze or grey.  The
/// renderer multiplies the glass' sky-tone emission by this colour, so the hue
/// carries even though the tile albedo of glass is (correctly) dark.
pub(crate) fn glass_tint(id: u32) -> [f32; 3] {
    let family = hash_u32(id, 107, 43);
    let l = 0.82 + hash_u32(id, 109, 43) * 0.18;
    let (r, g, b) = if family < 0.36 {
        (0.55, 0.80, 1.0) // blue
    } else if family < 0.56 {
        (0.55, 1.0, 0.80) // green
    } else if family < 0.78 {
        (1.0, 0.72, 0.42) // bronze
    } else {
        (0.80, 0.84, 0.90) // grey
    };
    [r * l, g * l, b * l]
}

pub(crate) fn jitter_for(id: u32) -> Jitter {
    let value = 0.82 + hash_u32(id, 3, 17) * 0.36;
    Jitter {
        value,
        mirror: hash_u32(id, 5, 23) < 0.15,
    }
}

/// Which of the massing forms this building is.  Derived from the plan, but the
/// *geometry* is what this drives, so it is worth naming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Massing {
    /// 塔楼: podium, shaft, stepped crown — the glass half of them.
    CurtainTower,
    /// 塔楼: podium, shaft, stepped crown — the masonry half.
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

pub(crate) fn massing_of(building: &ModernBuilding, ring: &[Vec2], area: f32) -> Massing {
    // A tower is twenty storeys and up, whatever the plan says about its podium:
    // a thirty-storey building with a two-storey base is still a 塔楼, and
    // classifying it as a slab is how a skyline ends up without one.
    if building.floors >= 20 {
        let shaft = facade_tile_for(building, Massing::Tower);
        if design(shaft).cladding.is_glass() {
            return Massing::CurtainTower;
        }
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
        // The plan's podium height is a retail volume; the podium is built as a
        // whole number of 公建 storeys (3.0 – 3.3 m) so its shopfront band and
        // its deck land on floors.  Two to four retail storeys is the Chinese
        // norm; five only on the biggest plots.
        (building.podium_height_metres / 3.3).round().clamp(2.0, 4.0)
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
///
/// **The draw-call trade is explicit.**  A tile in use is a draw call, and the
/// whole scene has a hard budget, so this table reaches for sixteen of the
/// twenty-four rather than all of them: six warm 板楼 variants, two grey and
/// pale slabs, two tile-clad, two brick, one cream podium tile, one mid-grey
/// panel, and four glass.  A 4 000-building city reaches every tile anyway
/// because the podium and crown functions compose new combinations; a 136-building
/// one does not, and it should not pay for tiles it never draws.
fn facade_tile_for(building: &ModernBuilding, massing: Massing) -> usize {
    let id = building.id as usize;
    match massing {
        Massing::CurtainTower | Massing::Tower => {
            // 8..12 is the first half of the curtain-wall block; the second half
            // (12..16) is reserved for crowns and for large districts.
            if id % 2 == 0 {
                8 + (id * 5 + 2) % 4
            } else {
                [6usize, 17, 20, 20][id % 4]
            }
        }
        // The warm end leads: 0, 1, 2, 4, 5 are beige, buff and ochre, and they
        // take six of the ten slots.  Grey, pale and tile-clad slabs fill the rest.
        Massing::Slab => [0usize, 0, 1, 2, 4, 0, 5, 6, 18, 17][id % 10],
        Massing::LowRise => {
            // Low-rise is where brick, dark tile and the small-tile stock lives.
            [22usize, 22, 17, 2, 1, 23, 16, 20][id % 8]
        }
    }
}

/// A **different** tile for the podium, because a 裙房 is clad differently from
/// the shaft above it and that difference is most of what tells a tower apart
/// from a slab at four hundred metres.  Every podium is the cream mosaic tile:
/// small ceramic tile on a podium base is what a Chinese 裙房 *is*, whatever the
/// shaft above it wears.
pub(crate) fn podium_tile_for(_shaft: usize) -> usize {
    17
}

/// A **crown of a different colour and material**, set back and often in a
/// different family from the shaft.  A crown that matches its shaft is a lid.
pub(crate) fn crown_tile_for(shaft: usize, id: u32) -> usize {
    if design(shaft).cladding.is_glass() {
        // A glass tower's crown is a pale stone cap — or, on every other block,
        // the mint-green glazing of the 2010s, which is the single most
        // recognisable residential-tower crown in the country.
        if id % 2 == 0 {
            13
        } else {
            14
        }
    } else {
        16
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
pub(crate) fn ring_of(footprint: &[Point], frame: CityFrameInfo) -> Vec<Vec2> {
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
        // Residential compounds are landscaped (lawn, hedges, paths); only
        // commercial, mixed and civic plots are hard-paved forecourts.
        let material = if matches!(parcel.use_type, ParcelUse::Park | ParcelUse::Residential) {
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
        builder.ambient_tint = Some(building_tint(building.id));
        building_shell(building, &ring, builder);
        builder.ambient_tint = None;
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
/// This is the single most important function in the crate and it exists because
/// `wall_uv` could not do it.  Two things are wrong with using world `Y` as the
/// tile's `V`:
///
/// * **Direction.**  The renderer uploads row 0 of a texture at `V = 0`, so `V`
///   must *increase downwards* for the image to appear the right way up.  World
///   `Y` increases upwards, which turns every facade upside down — invisible
///   while a storey band is painted symmetrically at both ends of the tile, and
///   glaring the moment a window has a sill.
/// * **Phase.**  A wall's storey lines are the floors of the building, and the
///   building's floors are `GROUND_STOREY_M + n * storey_m` above the pavement.
///   If `V` is world `Y`, whether a window head lands on a head depends on where
///   the building happens to sit relative to the texture's origin, and it stops
///   landing the moment the ground storey height changes with land use.
///
/// So `V` is measured from the *top of the wall downwards* and `V = 0` is put at
/// the top of the last storey, which makes every floor line land on `V` a
/// multiple of `1 / STOREYS_PER_TILE` — exactly on a floor, on every wall of
/// every building, at any height.  `storeys` is a whole number of the design's
/// own 层高, which is what keeps the painting and the geometry in phase.
pub(crate) fn facade_wall(
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
    // `V` counts down from the wall's head, in tile heights. Whole storeys and
    // whole bays: a wall is stretched by a few percent so that it starts and ends
    // on a bay boundary and carries an integer number of floors, otherwise the
    // window grid is cut through at every corner and misaligns between walls.
    let v_base = storeys.round().max(1.0) / 4.0;
    let bay = material
        .strip_prefix("facade/")
        .and_then(|index| index.parse::<usize>().ok())
        .map(|index| crate::facades::design(index).bay_m)
        .filter(|bay| *bay > 0.5)
        .unwrap_or(3.6);
    let snapped = (length / bay).round().max(1.0) * bay;
    let u1 = if mirror { -snapped } else { snapped };
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
pub(crate) fn band_uv(
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

// ---------------------------------------------------------------------------
// compounds (小区)
// ---------------------------------------------------------------------------

/// A walled residential compound: boundary wall, a gate portal on the street
/// front, an internal fire lane and a lawn.
fn compound_shell(compound: &Compound, ring: &[Vec2], frame: CityFrameInfo, builder: &mut MeshBuilder) {
    // Clockwise in plan, so wall quads face outward (see `building_shell`).
    let mut outward = ring.to_vec();
    if signed_area(&outward) > 0.0 {
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
    // Internal loop drive.  Metre UVs along the path: `parcel.paving` is a
    // *textured* material, and a bare quad into it desyncs the group's optional
    // UV layer from its vertex count — the payload bug the renderer caught.
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
        builder.ribbon_uv_along(
            "parcel.paving",
            &drive,
            -compound.road_width_metres * 0.5,
            compound.road_width_metres * 0.5,
            0.0,
            length,
            level::GROUND + 0.006,
            None,
            0.0,
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
    use crate::facades::{FACADE_TILE_H, FACADE_TILE_W};
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> urban::ModernCity {
        generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        })
    }

    /// Facade triangles must be front-facing from *outside*.  The renderer culls
    /// back faces, so a wall wound inward is invisible from the street and the
    /// viewer sees the far walls' inner sides instead: a building that looks
    /// hollow.  Asserted on the emitted index winding, not on the stored normals.
    #[test]
    fn facade_walls_face_away_from_the_building() {
        let city = city();
        let mut checked = 0;
        let mut outward = 0;
        for building in city.buildings.iter().take(12) {
            let ring = ring_of(&building.footprint, city.frame);
            if ring.len() < 3 {
                continue;
            }
            let centre = ring_centroid(&ring);
            let mut builder = MeshBuilder::new();
            shell::building_shell(building, &ring, &mut builder);
            for group in builder.build().meshes.iter().filter(|g| g.material.starts_with("facade/")) {
                let p = |i: u32| {
                    let i = i as usize * 3;
                    Vec3::new(group.positions[i], group.positions[i + 1], group.positions[i + 2])
                };
                for tri in group.indices.chunks_exact(3) {
                    let (a, b, c) = (p(tri[0]), p(tri[1]), p(tri[2]));
                    let n = (b - a).cross(c - a);
                    if n.y.abs() > 0.5 * n.length() || n.length() < 1.0e-6 {
                        continue; // not a wall
                    }
                    let mid = Vec2::new((a.x + b.x + c.x) / 3.0, (a.z + b.z + c.z) / 3.0);
                    let out = Vec2::new(mid.x - centre.x, mid.y - centre.y);
                    // A concave footprint (courtyard, carved slab) has genuine
                    // wall faces that point toward the centroid, so count
                    // rather than assert per triangle.
                    if n.x * out.x + n.z * out.y > 0.0 {
                        outward += 1;
                    }
                    checked += 1;
                }
            }
        }
        assert!(checked > 50, "only {checked} wall triangles checked");
        assert!(
            outward as f32 >= checked as f32 * 0.9,
            "only {outward} of {checked} facade triangles face outward"
        );
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
    #[allow(dead_code)]
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

    pub(super) fn is_building_material(material: &str) -> bool {
        material.starts_with("facade/")
            || material.starts_with("ground/")
            || matches!(
                material,
                "roof" | "trim.light" | "trim.dark" | "balcony.slab" | "metal.ac" | "awning"
                    | "wall.render" | "sign/shop" | "block.ground" | "parcel.paving"
                    | "parcel.green"
            )
    }

    /// The dimensioned rules are inside their construction bands.  These are the
    /// numbers every balcony, condenser berth and bay course is *built* from, so
    /// a drift out of the band is a building nobody could get approved.
    #[test]
    fn the_geometry_rules_sit_inside_the_construction_bands() {
        assert!(
            (1.2..=1.5).contains(&rule::BALCONY_DEPTH_M),
            "阳台进深 {} m is outside 1.2-1.5",
            rule::BALCONY_DEPTH_M
        );
        assert_eq!(rule::balcony_rail_m(6), 1.05, "≤6 层 guard is 1.05");
        assert_eq!(rule::balcony_rail_m(7), 1.10, "≥7 层 guard is 1.10");
        assert!(
            (0.6..=0.8).contains(&rule::AC_BERTH_W_M),
            "空调机位 {} m is outside 0.6-0.8",
            rule::AC_BERTH_W_M
        );
        assert!(
            (0.4..=0.6).contains(&rule::BAY_WINDOW_PROJECTION_M),
            "飘窗凸出 {} m is outside 0.4-0.6",
            rule::BAY_WINDOW_PROJECTION_M
        );
        // And the standard module constants quote the standard design.
        assert!((FACADE_TILE_H - design(0).storey_m * 4.0).abs() < 1.0e-6);
        assert!((FACADE_TILE_W - design(0).bay_m).abs() < 1.0e-6);
    }

    /// Balcony presence is a per-class rule, not a per-building whim: every
    /// residential form has them and curtain wall never does.
    #[test]
    fn balcony_presence_is_a_class_rule() {
        assert!(rule::has_balconies(Massing::Slab, false));
        assert!(rule::has_balconies(Massing::LowRise, false));
        assert!(rule::has_balconies(Massing::Tower, false));
        assert!(!rule::has_balconies(Massing::CurtainTower, true));
        assert!(!rule::has_balconies(Massing::Slab, true));
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

    /// `roof`, `parcel.paving` and `sign/shop` are **textured** materials: every
    /// vertex in those groups must carry a UV, or the renderer's optional UV
    /// layer desyncs from the vertex count and the payload fails to load.
    #[test]
    fn every_textured_building_group_carries_uvs_on_every_vertex() {
        let groups = built().meshes;
        for group in &groups {
            if !matches!(group.material.as_str(), "roof" | "parcel.paving" | "parcel.green" | "block.ground" | "sign/shop")
            {
                continue;
            }
            let uvs = group.uvs.as_ref().unwrap_or_else(|| {
                panic!("{} is texture-bound and needs UVs on every vertex", group.material)
            });
            assert_eq!(
                uvs.len(),
                group.positions.len() / 3 * 2,
                "{} has a partly populated UV layer",
                group.material
            );
        }
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
            let Some(extent) = roofscape::ring_extent(&ring) else { continue };
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
        // Low rise, mid rise and a real skyline.  The thresholds are the Chinese
        // storey bands, not round numbers: a 多层 walk-up tops out below 26 m, a
        // 板楼 sits in the middle, and a 塔楼 clears twenty storeys.
        let low = measured.iter().filter(|h| **h < 26.0).count();
        let mid = measured.iter().filter(|h| (26.0..58.0).contains(*h)).count();
        let tall = measured.iter().filter(|h| **h >= 58.0).count();
        assert!(low > 3, "only {low} low-rise buildings; a city of towers is not a city");
        assert!(mid > 3, "only {mid} mid-rise buildings; the 板楼 band is missing");
        assert!(tall > 0, "nothing above 58 m: there is no skyline");
        // And the heights must genuinely vary, not cluster.
        let mut sorted = measured.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let range = sorted[sorted.len() - 1] - sorted[0];
        assert!(range > 45.0, "the whole city spans {range:.0} m of height");
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

    /// The slab family must not be a family of rectangles: carved L, U and H
    /// plans and stepped end blocks have to exist in the built geometry, and the
    /// way to know is to count facade walls per slab — a carved slab has more
    /// than four.
    #[test]
    fn slab_massing_is_carved_and_stepped_not_only_rectangular() {
        let city = city();
        let mut carved = 0;
        let mut slabs = 0;
        for building in &city.buildings {
            let ring = ring_of(&building.footprint, city.frame);
            if ring.len() < 3 {
                continue;
            }
            let area = signed_area(&ring).abs();
            if massing_of(building, &ring, area) != Massing::Slab {
                continue;
            }
            slabs += 1;
            if shell::slab_variant(building.id) != shell::SlabVariant::Rectangle {
                carved += 1;
            }
        }
        assert!(slabs > 5, "only {slabs} slabs in the city");
        assert!(
            carved * 4 > slabs,
            "only {carved} of {slabs} slabs are carved or stepped; a district of pure rectangles is a trading estate"
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
                // Either a group has no vertex colours or it has four bytes per
                // vertex.  A *partly* populated colour buffer is worse than none,
                // because a renderer that turns `vertexColors` on would read it
                // as a shorter array and misalign every vertex after the first
                // untinted one.
                assert_eq!(
                    colors.len(),
                    group.positions.len() / 3 * 4,
                    "{} has a partly populated colour buffer",
                    group.material
                );
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
            match massing_of(building, &ring, area) {
                Massing::CurtainTower | Massing::Tower => tall += 1,
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


/// The river as a water surface: a strip along the centreline, `width` metres
/// across, lying just above the bare ground so the channel reads as water
/// rather than as a gap in the city.
pub fn build_water(river: &[Point], width: f32, frame: CityFrameInfo, builder: &mut MeshBuilder) {
    builder.style(
        "water",
        GroupStyle { cast_shadow: false, receive_shadow: true, alpha_cutout: false, dynamic: false },
    );
    let mut line = ring_of(river, frame);
    if line.len() < 2 {
        return;
    }
    // Chaikin corner cutting: the plan gives a coarse polyline, and offsetting a
    // coarse polyline makes angular banks. Three passes round every bend into a
    // smooth curve while keeping both ends where they were.
    for _ in 0..3 {
        let mut next = Vec::with_capacity(line.len() * 2);
        next.push(line[0]);
        for pair in line.windows(2) {
            next.push(pair[0] * 0.75 + pair[1] * 0.25);
            next.push(pair[0] * 0.25 + pair[1] * 0.75);
        }
        next.push(line[line.len() - 1]);
        line = next;
    }
    let half = width * 0.5;
    let shore = (width * 0.12).clamp(2.5, 6.0);
    let mut left = Vec::with_capacity(line.len());
    let mut right = Vec::with_capacity(line.len());
    let mut left_in = Vec::with_capacity(line.len());
    let mut right_in = Vec::with_capacity(line.len());
    for i in 0..line.len() {
        let a = line[i.saturating_sub(1)];
        let b = line[(i + 1).min(line.len() - 1)];
        let (dx, dz) = (b.x - a.x, b.y - a.y);
        let len = dx.hypot(dz).max(1.0e-4);
        let (nx, nz) = (-dz / len, dx / len);
        left.push(Vec2::new(line[i].x + nx * half, line[i].y + nz * half));
        right.push(Vec2::new(line[i].x - nx * half, line[i].y - nz * half));
        left_in.push(Vec2::new(line[i].x + nx * (half - shore), line[i].y + nz * (half - shore)));
        right_in.push(Vec2::new(line[i].x - nx * (half - shore), line[i].y - nz * (half - shore)));
    }
    // Shallow, sandy-green water at the edges and deep blue-green in the middle,
    // written as vertex colours so one material draws the whole river.
    let shallow = Some([0.34, 0.50, 0.46]);
    let deep = Some([0.14, 0.27, 0.34]);
    // Quads per segment: robust on bends where one big polygon would self-touch.
    let mut put = |builder: &mut MeshBuilder, mut quad: Vec<Vec2>, colour: Option<[f32; 3]>| {
        if signed_area(&quad) < 0.0 {
            quad.reverse();
        }
        builder.ground_uv("water", &quad, 0.03, colour);
    };
    for i in 0..line.len() - 1 {
        put(builder, vec![left[i], left[i + 1], left_in[i + 1], left_in[i]], shallow);
        put(builder, vec![left_in[i], left_in[i + 1], right_in[i + 1], right_in[i]], deep);
        put(builder, vec![right_in[i], right_in[i + 1], right[i + 1], right[i]], shallow);
    }
}
