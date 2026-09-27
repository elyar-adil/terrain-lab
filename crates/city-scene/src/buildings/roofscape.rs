//! The roofscape.
//!
//! This is the single most recognisable thing about a Chinese skyline and the
//! thing a lazy port leaves out entirely: a flat lid with nothing on it.  A real
//! roof carries a stair head house, a lift overrun, one to four **water tanks on
//! legs** (the stainless drum every block has, because the mains pressure is not
//! reliable), a satellite dish, an antenna mast, a run of clothes poles on a
//! low-rise block, and on a newer one a PV array — plus the paved service strips
//! and greening patches that tell you someone walks up here.  All of it is
//! placed by rejection sampling against the roof outline, so nothing lands off
//! the roof.

use urban::ModernBuilding;

use crate::math::{Rng, Vec2, Vec3, point_in_ring, ring_centroid};
use crate::mesh::MeshBuilder;

/// The roofscape of one building.  Rejection sampling inside the roof polygon,
/// with a margin for each prop's own half-extent — the test
/// `nothing_a_roof_prop_is_placed_off_the_roof` asserts this, because a tank
/// hanging in the air beside a tower is the single most obvious tell of a
/// procedural city.
pub fn roofscape(building: &ModernBuilding, ring: &[Vec2], deck: f32, builder: &mut MeshBuilder) {
    let Some(extent) = ring_extent(ring) else {
        return;
    };
    let width = extent[2] - extent[0];
    let depth = extent[3] - extent[1];
    let mut rng = Rng::new(building.id ^ 0x9e37_79b9);

    let place = |rng: &mut Rng, half_x: f32, half_z: f32| -> Option<Vec2> {
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
    let place_box = |rng: &mut Rng, w: f32, d: f32, rotation: f32| -> Option<Vec2> {
        let (sin, cos) = rotation.sin_cos();
        let (sin, cos) = (sin.abs(), cos.abs());
        place(
            rng,
            w * 0.5 * cos + d * 0.5 * sin,
            w * 0.5 * sin + d * 0.5 * cos,
        )
    };

    // A paved service strip around the stair head and a greening patch: someone
    // walks up here, and the walkway is where the boots wear the membrane.  Both
    // are `ground_uv` quads so the textured materials keep their UV layer whole.
    roof_patches(&mut rng, ring, deck, builder);

    // The stair head house (楼梯间) and the lift overrun beside it.  Two boxes and
    // a dark door: the most reliable way to tell a Chinese roof from a western
    // one at two hundred metres.
    let rotation = rng.range(-0.35, 0.35);
    let body_w = 3.2 + rng.unit() * 1.6;
    let body_d = 2.6 + rng.unit() * 0.8;
    let cap_w = body_w + 0.40;
    let cap_d = body_d + 0.40;
    let mut stair_house: Option<Vec2> = None;
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
        // The door facing the deck, and a flue beside the house: the kitchen and
        // bathroom exhausts of the top floor go straight out here.
        let outward = (point - ring_centroid(ring)).normalize();
        let door = point - outward * (body_d * 0.5 - 0.02);
        crate::mesh::box_at(builder, "trim.dark", door, deck + 1.05, 0.95, 2.1, 0.08, rotation);
        stair_house = Some(point);
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
    // Exhaust flues (排烟道): a pair of stubby stacks with caps, usually against
    // the stair house, on most roofs.
    if rng.chance(0.7) {
        let anchor = stair_house
            .or_else(|| place(&mut rng, 0.45, 0.45))
            .unwrap_or_else(|| Vec2::new((extent[0] + extent[2]) * 0.5, (extent[1] + extent[3]) * 0.5));
        for (dx, dz, height) in [(0.55_f32, 0.30_f32, 1.1_f32), (0.95, -0.25, 1.5)] {
            let base = Vec2::new(anchor.x + dx, anchor.y + dz);
            if point_in_ring(base, ring) {
                builder.tube(
                    "trim.light",
                    Vec3::new(base.x, deck, base.y),
                    Vec3::new(base.x, deck + height, base.y),
                    0.22,
                    0.20,
                    4,
                    None,
                );
                // The cap slab, and the dark throat under it.
                builder.wall("trim.dark", base, base, deck + height - 0.16, deck + height, None);
                builder.quad(
                    "trim.dark",
                    Vec3::new(base.x - 0.24, deck + height, base.y - 0.24),
                    Vec3::new(base.x + 0.24, deck + height, base.y - 0.24),
                    Vec3::new(base.x + 0.24, deck + height, base.y + 0.24),
                    Vec3::new(base.x - 0.24, deck + height, base.y + 0.24),
                    None,
                );
            }
        }
    }

    // Water tanks (水箱): a stainless drum on four short legs, which is the
    // shape almost every block in the country has on its roof.
    let tanks = 1 + rng.int(3);
    for _ in 0..tanks {
        if let Some(point) = place(&mut rng, 0.85, 0.85) {
            water_tank(&mut rng, point, deck, builder);
        }
    }

    // Condenser units, and a satellite dish on most roofs.  The condensers are
    // placed as an aligned **bank** along one edge — plant lines up against the
    // parapet, it is not scattered — with the odd stray unit beside them.
    condenser_bank(&mut rng, ring, deck, builder);
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
    if building.floors >= 8 {
        // A guyed mast's anchors reach 2.2 m out from the mast, so the mast is
        // placed far enough in that the wires land on the deck, not in the air
        // beside it.
        let guy_margin = if building.floors >= 14 { 2.6 } else { 0.0 };
        if let Some(point) = place(&mut rng, 0.70 + guy_margin, 0.25 + guy_margin) {
            mast(
                building.id,
                point,
                deck,
                4.0 + rng.unit() * 5.0,
                building.floors >= 14,
                builder,
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

/// A stainless water tank on four short legs, with its lid collar.  Split out so
/// the stepped slab ends can grow one too.
fn water_tank(rng: &mut Rng, point: Vec2, deck: f32, builder: &mut MeshBuilder) {
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
            3,
            None,
        );
    }
    let base = deck + leg;
    // Six sides.  A stainless drum is 1.1 m across and the closest a viewer ever
    // gets is a hundred metres, so seven sides buys nothing and every roof in
    // the city pays for it.
    builder.tube(
        "metal.ac",
        Vec3::new(point.x, base, point.y),
        Vec3::new(point.x, base + drum, point.y),
        radius,
        radius * 0.94,
        6,
        None,
    );
    // A collar at the top, so the drum reads as a tank with a lid rather than as
    // an open pipe.
    builder.tube(
        "metal.ac",
        Vec3::new(point.x, base + drum - 0.10, point.y),
        Vec3::new(point.x, base + drum, point.y),
        radius * 1.06,
        radius * 1.02,
        6,
        None,
    );
}

/// An antenna mast with a cross-arm, three whips and — on tall buildings — three
/// guy wires back to the deck.  The wires are what a real mast has and what keeps
/// it from reading as a flagpole.
fn mast(id: u32, point: Vec2, deck: f32, height: f32, guys: bool, builder: &mut MeshBuilder) {
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
    if guys {
        // Three anchors 120° apart at 2.2 m radius; the wires are hair-thin
        // tubes because anything thicker reads as rope from the street.
        let mut seed = id;
        for index in 0..3 {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let angle = (index as f32) * std::f32::consts::TAU / 3.0 + (seed % 97) as f32 * 0.01;
            let anchor = point + Vec2::new(angle.cos(), angle.sin()) * 2.2;
            builder.tube(
                "trim.dark",
                Vec3::new(point.x, deck + height * 0.92, point.y),
                Vec3::new(anchor.x, deck + 0.12, anchor.y),
                0.014,
                0.010,
                3,
                None,
            );
        }
    }
}

/// Paved service strips and greening patches: the marks of a roof people walk
/// on.  Both are inset well inside the parapet and emitted with `ground_uv` so
/// the textured `parcel.paving` / `parcel.green` groups keep every UV.
fn roof_patches(rng: &mut Rng, ring: &[Vec2], deck: f32, builder: &mut MeshBuilder) {
    let Some([min_x, min_y, max_x, max_y]) = ring_extent(ring) else {
        return;
    };
    let (w, d) = (max_x - min_x, max_y - min_y);
    if w < 6.0 || d < 5.0 {
        return;
    }
    let centre = ring_centroid(ring);
    // A walkway strip 0.9 m wide across the short axis, offset by a third.
    let strip_at = min_y + d * (0.3 + rng.unit() * 0.2);
    let strip = vec![
        Vec2::new(min_x + w * 0.2, strip_at),
        Vec2::new(max_x - w * 0.2, strip_at),
        Vec2::new(max_x - w * 0.2, strip_at + 0.9),
        Vec2::new(min_x + w * 0.2, strip_at + 0.9),
    ];
    if strip.iter().all(|point| point_in_ring(*point, ring)) {
        builder.ground_uv("parcel.paving", &strip, deck + 0.015, None);
    }
    // A greening patch on a third of the roofs: planter beds on sleepers.
    if rng.chance(0.35) {
        let (pw, pd) = ((w * 0.22).min(3.2), (d * 0.24).min(2.6));
        let at = centre
            + Vec2::new((rng.unit() - 0.5) * w * 0.3, (rng.unit() - 0.5) * d * 0.3);
        let patch = vec![
            Vec2::new(at.x - pw, at.y - pd),
            Vec2::new(at.x + pw, at.y - pd),
            Vec2::new(at.x + pw, at.y + pd),
            Vec2::new(at.x - pw, at.y + pd),
        ];
        if patch.iter().all(|point| point_in_ring(*point, ring)) {
            builder.ground_uv("parcel.green", &patch, deck + 0.05, None);
            // The sleeper edges that hold the bed.
            for index in 0..4 {
                let a = patch[index];
                let b = patch[(index + 1) % 4];
                builder.wall("trim.dark", a, b, deck, deck + 0.18, None);
            }
        }
    }
}

/// A row of condenser units lined along one roof edge — rooftop plant is banked
/// against the parapet, never scattered — plus the odd stray.
fn condenser_bank(rng: &mut Rng, ring: &[Vec2], deck: f32, builder: &mut MeshBuilder) {
    let Some([min_x, min_y, max_x, max_y]) = ring_extent(ring) else {
        return;
    };
    let (w, d) = (max_x - min_x, max_y - min_y);
    if w < 7.0 || d < 4.0 {
        return;
    }
    // Along the long axis, in the first unit-width inside the parapet.
    let count = 3 + rng.int(3);
    let pitch = 1.1_f32;
    let start = if w >= d { min_x + 1.2 } else { min_y + 1.2 };
    for index in 0..count {
        let at = start + index as f32 * pitch;
        let unit = if w >= d {
            if at > max_x - 1.2 {
                break;
            }
            Vec2::new(at, min_y + 0.65)
        } else {
            if at > max_y - 1.2 {
                break;
            }
            Vec2::new(min_x + 0.65, at)
        };
        if !point_in_ring(unit, ring) {
            continue;
        }
        crate::mesh::box_at(
            builder,
            "metal.ac",
            unit,
            deck + 0.42,
            0.92,
            0.84,
            0.40,
            if w >= d { 0.0 } else { std::f32::consts::FRAC_PI_2 },
        );
    }
}

/// Working plant on a tower's podium deck: an AC bank, a couple of tanks' worth
/// of pipework and a greening patch — the 裙房 roof is where the residents walk
/// the dog.
pub fn podium_deck_props(building: &ModernBuilding, ring: &[Vec2], deck: f32, builder: &mut MeshBuilder) {
    let mut rng = Rng::new(building.id ^ 0x51f3_1e0d);
    condenser_bank(&mut rng, ring, deck, builder);
    roof_patches(&mut rng, ring, deck, builder);
}

/// A mast and a condenser on a small topmost crown step, for the tallest towers.
pub fn crown_mast(id: u32, ring: &[Vec2], deck: f32, builder: &mut MeshBuilder) {
    let Some(extent) = ring_extent(ring) else {
        return;
    };
    let centre = Vec2::new((extent[0] + extent[2]) * 0.5, (extent[1] + extent[3]) * 0.5);
    if point_in_ring(centre, ring) {
        mast(id, centre, deck, 5.5, false, builder);
    }
    let mut rng = Rng::new(id ^ 0x1f2e_3d4c);
    condenser_bank(&mut rng, ring, deck, builder);
}

pub(crate) fn ring_extent(ring: &[Vec2]) -> Option<[f32; 4]> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::signed_area;
    use crate::mesh::MeshBuilder;
    use crate::buildings::{level, ring_of};
    use crate::facades::{GROUND_STOREY_M, STOREY_M};
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city() -> urban::ModernCity {
        generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        })
    }

    /// The roofscape, asserted on the built geometry.  A Chinese roof is never
    /// empty, and a city whose roofs are bare lids reads as a blockout no matter
    /// how the walls are textured.
    #[test]
    fn a_city_has_water_tanks_stair_houses_and_masts() {
        let city = city();
        let mut builder = MeshBuilder::new();
        crate::buildings::build(
            &city.blocks,
            &city.parcels,
            &city.buildings,
            &city.compounds,
            city.frame,
            &mut builder,
        );
        let groups = builder.build().meshes;
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

    #[test]
    fn roofs_carry_plant_and_a_parapet() {
        let city = city();
        let mut builder = MeshBuilder::new();
        crate::buildings::build(
            &city.blocks,
            &city.parcels,
            &city.buildings,
            &city.compounds,
            city.frame,
            &mut builder,
        );
        let groups = builder.build().meshes;
        let roof = groups.iter().find(|g| g.material == "roof").expect("no roof");
        assert!(roof.positions.len() > 0);
        assert!(
            groups.iter().any(|g| g.material == "metal.ac"),
            "no rooftop plant on a city of towers"
        );
        // A roof people walk on: service paving and the odd greening patch.
        assert!(
            groups.iter().any(|g| g.material == "parcel.paving"),
            "no paved service strip on any roof"
        );
    }
}
