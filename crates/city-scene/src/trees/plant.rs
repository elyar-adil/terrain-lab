//! Planting: where the city's trees go, and which species each site gets.
//!
//! A planting list is a *designed order*, not a uniform bag. The order is the
//! conventional one a Chinese landscape bureau plants in — the workhorse
//! avenue species first, the specials after — so a street reads as designed
//! rather than as a shuffle. Every list is drawn from the reference set of
//! sixteen in [`crate::species`].

use urban::{CityFrameInfo, Parcel, ParcelUse, Point, TreeSpecies, modern};

use super::{TAU, TREE_BUDGET, TUFT_BUDGET, TreePrototype, two_variant_count};
use crate::buildings::level;
use crate::math::{Rng, Vec2};
use crate::mesh::{Instance, MeshBuilder};
use crate::network::Network;
use crate::species::by_key;

/// Where a tree came from. Reported so the renderer can pick a foliage LOD and
/// so placement can be audited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TreeRole {
    /// Kerbside avenue tree.
    Street,
    /// Median planting.
    Median,
    /// Park or green parcel.
    Park,
    /// Inside a residential compound.
    Compound,
    /// Riverfront promenade.
    Waterfront,
    /// Roundabout island.
    Island,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeOutput {
    pub instances: usize,
    pub by_role: Vec<(String, usize)>,
    pub tufts: usize,
}

/// The avenue: camphor and elm as the evergreen and deciduous workhorses, the
/// fastigiate poplar for a northern boulevard's colonnade, scholar and locust
/// where the footway is wide, ginkgo and parasol as the specials, the little
/// osmanthus where the kerb is narrow, and the two conifers where a road
/// swings wide.
const AVENUE: &[&str] = &[
    "xiang-zhang",
    "yu-shu",
    "yang-shu",
    "huai-shu",
    "yin-xing",
    "wu-tong",
    "ci-huai",
    "gui-hua",
    "sha-shu",
];

/// A motorway's trees have to clear a sign and survive a hot verge: the
/// columnar poplar and the conifers.
const MOTORWAY: &[&str] = &["yang-shu", "shui-shan", "sha-shu", "xue-song"];

/// A median is a narrow strip at 0.5 m, so it gets the small trees: the
/// osmanthus, the peach in blossom, and the silk tree where the strip widens.
const MEDIAN: &[&str] = &["gui-hua", "tao-shu", "he-huan"];

/// A park is where every species in the reference set can appear, and a city
/// with a park has all sixteen in it.
const PARK: &[&str] = &[
    "xiang-zhang",
    "yu-shu",
    "yang-shu",
    "huai-shu",
    "yin-xing",
    "wu-tong",
    "ci-huai",
    "gui-hua",
    "xue-song",
    "liu-shu",
    "song-shu",
    "shui-shan",
    "rong-shu",
    "tao-shu",
    "sha-shu",
    "he-huan",
];

/// A compound courtyard is evergreen, sheltering and small — with one peach
/// for the blossom and one silk tree for the shade, because a courtyard is
/// where a Chinese city keeps its small pleasures.
const COMPOUND: &[&str] = &[
    "xiang-zhang",
    "gui-hua",
    "yu-shu",
    "tao-shu",
    "he-huan",
];

/// A riverbank in a Chinese city is a willow or a metasequoia avenue. The
/// Wuhan and Nanjing embankments are lined with them and so is half the
/// country.
const WATERFRONT: &[&str] = &["liu-shu", "shui-shan", "yang-shu", "yu-shu"];

/// A farmyard: poplars and willows, the locust and the elm, a peach by the door.
const FARM: &[&str] = &["yang-shu", "liu-shu", "huai-shu", "yu-shu", "tao-shu", "ci-huai", "xiang-zhang"];

/// A roundabout island is visible from every approach, so it gets the showy
/// small trees and the ginkgo.
const ISLAND: &[&str] = &["tao-shu", "he-huan", "gui-hua", "yin-xing"];

/// Plant every tree a city should have: kerbside avenues, planted medians, park
/// groves, compound interiors, the river promenade and roundabout islands.
pub fn plant(
    network: &Network,
    parcels: &[Parcel],
    buildings: &[urban::ModernBuilding],
    river: Option<&[Point]>,
    river_width: f32,
    frame: CityFrameInfo,
    prototypes: &[TreePrototype],
    builder: &mut MeshBuilder,
    seed: u32,
) -> TreeOutput {
    super::declare(builder);
    let mut rng = Rng::new(seed ^ 0x74ee);
    let mut output = TreeOutput::default();
    let mut counts: Vec<(TreeRole, usize)> = Vec::new();
    // A free function rather than a closure: the planting loops need the same
    // stream for their own jitter, and a closure capturing it would force those
    // draws through `place` instead.
    fn place(
        role: TreeRole,
        point: Vec2,
        list: &'static [&'static str],
        prototypes: &[TreePrototype],
        rng: &mut Rng,
        builder: &mut MeshBuilder,
        output: &mut TreeOutput,
        counts: &mut Vec<(TreeRole, usize)>,
    ) {
        if output.instances >= TREE_BUDGET {
            return;
        }
        // A planting list is a designed order, not a uniform bag: the front of
        // the list is what the city actually plants, and `u^1.35` is a gentle
        // enough skew that the rest still turns up.
        let roll = rng.unit().powf(1.35) * list.len() as f32;
        let key = list[(roll as usize).min(list.len() - 1)];
        let Some(species) = by_key(key) else {
            return;
        };
        let variants = variants_of(species);
        let variant = rng.int(variants as u32) as u16;
        let index = prototypes
            .iter()
            .position(|prototype| prototype.species.key == species.key && prototype.variant == variant)
            .unwrap_or(0);
        let prototype = &prototypes[index];
        let jitter = 0.90 + rng.unit() * 0.22;
        // Per-tree variation, and only downwards: an instance tint above 1.0
        // clips in the payload's `u8` tint and the tree is brighter than the
        // species, which is the one direction that has to be impossible.
        let tint = [
            0.84 + rng.unit() * 0.16,
            0.86 + rng.unit() * 0.14,
            0.82 + rng.unit() * 0.18,
        ];
        builder.add_instance(
            &prototype.key,
            Instance {
                x: point.x,
                y: level::GROUND - 0.05,
                z: point.y,
                rotation_y: rng.unit() * TAU,
                scale_x: prototype.height * jitter,
                scale_y: prototype.height * jitter,
                scale_z: prototype.height * jitter,
                tint_r: tint[0],
                tint_g: tint[1],
                tint_b: tint[2],
            },
        );
        output.instances += 1;
        match counts.iter_mut().find(|(existing, _)| *existing == role) {
            Some(entry) => entry.1 += 1,
            None => counts.push((role, 1)),
        }
    }

    // --- kerbside avenues and planted medians -------------------------------
    for road in &network.roads {
        if road.layer != 0 || !road.has_sidewalk() {
            continue;
        }
        let path = road.carriageway.clone();
        let length = path.length();
        if length < 24.0 {
            continue;
        }
        let section = road.section;
        let half = section.half_width();
        // A boulevard gets a double row; a local street gets one on the sunnier
        // side only, which is both what is built and what keeps the count sane.
        let double_row = matches!(
            road.class,
            urban::ModernRoadClass::Arterial | urban::ModernRoadClass::Expressway
        );
        let spacing = if road.is_motorway() { 22.0 } else { 15.0 };
        let lateral = half + section.sidewalk_metres * 0.6;
        let list = if road.is_motorway() { MOTORWAY } else { AVENUE };
        let mut station = 12.0;
        let mut side = 1.0_f32;
        while station < length - 12.0 {
            // Longitudinally jittered so an avenue does not read as a comb.
            let offset = station + (rng.unit() - 0.5) * 5.0;
            if offset < 10.0 || offset > length - 10.0 {
                station += spacing;
                continue;
            }
            let sides: &[f32] = if double_row { &[-1.0, 1.0] } else { &[side] };
            for direction in sides {
                let wobble = (rng.unit() - 0.5) * 0.9;
                let point = path.offset_at(offset, *direction * (lateral + wobble), 0.0);
                place(
                    TreeRole::Street,
                    Vec2::new(point.x, point.z),
                    list,
                    prototypes,
                    &mut rng,
                    builder,
                    &mut output,
                    &mut counts,
                );
            }
            side = -side;
            station += spacing;
        }
        // Median belt on a divided street.
        if section.has_median() {
            let median_half = section.median_metres * 0.5;
            let mut at = 20.0;
            while at < length - 20.0 {
                let point =
                    path.offset_at(at + (rng.unit() - 0.5) * 4.0, (rng.unit() - 0.5) * median_half * 0.5, 0.0);
                place(
                    TreeRole::Median,
                    Vec2::new(point.x, point.z),
                    MEDIAN,
                    prototypes,
                    &mut rng,
                    builder,
                    &mut output,
                    &mut counts,
                );
                at += 24.0;
            }
        }
    }

    // --- park groves and compound interiors ---------------------------------
    // Where the houses of a garden stand, so a tree is not planted through one.
    let mut footprints: std::collections::HashMap<u32, Vec<Vec<Vec2>>> = std::collections::HashMap::new();
    for building in buildings {
        if matches!(building.use_type, ParcelUse::Villa | ParcelUse::Farmstead) {
            let ring: Vec<Vec2> = building
                .footprint
                .iter()
                .map(|point| {
                    let [x, z] = frame.to_local(*point);
                    Vec2::new(x, z)
                })
                .collect();
            footprints.entry(building.parcel_id).or_default().push(ring);
        }
    }
    for parcel in parcels {
        let ring: Vec<Vec2> = parcel
            .ring
            .iter()
            .map(|point| {
                let [x, z] = frame.to_local(*point);
                Vec2::new(x, z)
            })
            .collect();
        if ring.len() < 3 {
            continue;
        }
        let interior = crate::math::inset_ring(&ring, 3.0);
        if interior.len() < 3 {
            continue;
        }
        let bounds = bounds_of(&interior);
        let (role, count, list) = match parcel.use_type {
            ParcelUse::Park => (TreeRole::Park, 10, PARK),
            ParcelUse::Residential if parcel.compound => (TreeRole::Compound, 8, COMPOUND),
            ParcelUse::Civic => (TreeRole::Park, 5, PARK),
            ParcelUse::Villa => (TreeRole::Compound, 3 + (parcel.id % 4) as usize, COMPOUND),
            ParcelUse::Farmstead => (TreeRole::Compound, 2 + (parcel.id % 5) as usize, FARM),
            _ => continue,
        };
        let houses = footprints.get(&parcel.id);
        for _ in 0..count {
            // Rejection-sample inside the parcel: scatter trees in a bounding box
            // and keep the ones that actually land on green.
            for _ in 0..6 {
                let candidate = Vec2::new(
                    bounds.0 + rng.unit() * (bounds.2 - bounds.0),
                    bounds.1 + rng.unit() * (bounds.3 - bounds.1),
                );
                let blocked = houses.is_some_and(|rings| {
                    rings.iter().any(|house| {
                        crate::math::point_in_ring(candidate, house)
                            || (0..house.len()).any(|i| {
                                segment_distance(candidate, house[i], house[(i + 1) % house.len()]) < 3.2
                            })
                    })
                });
                if !blocked && crate::math::point_in_ring(candidate, &interior) {
                    place(
                        role,
                        candidate,
                        list,
                        prototypes,
                        &mut rng,
                        builder,
                        &mut output,
                        &mut counts,
                    );
                    break;
                }
            }
        }
    }

    // --- riverfront promenade ----------------------------------------------
    if let Some(river) = river {
        let path = crate::math::Path::from_plan(
            river
                .iter()
                .map(|point| {
                    let [x, z] = frame.to_local(*point);
                    [x, z]
                })
                .collect::<Vec<[f32; 2]>>(),
        );
        let mut station = 0.0;
        while station < path.length() {
            for side in [-1.0_f32, 1.0] {
                let point = path.offset_at(
                    station,
                    // Clear of the water: half the channel plus the bank promenade.
                    side * (river_width * 0.5 + 14.0),
                    0.0,
                );
                place(
                    TreeRole::Waterfront,
                    Vec2::new(point.x, point.z),
                    WATERFRONT,
                    prototypes,
                    &mut rng,
                    builder,
                    &mut output,
                    &mut counts,
                );
            }
            station += 30.0;
        }
    }

    // --- roundabout islands --------------------------------------------------
    for junction in &network.junctions {
        if !junction.roundabout {
            continue;
        }
        let island = junction.radius.max(18.0) * 0.42;
        for index in 0..5 {
            let angle = index as f32 / 5.0 * TAU + rng.unit() * 0.9;
            let radius = island * (0.25 + rng.unit() * 0.5);
            place(
                TreeRole::Island,
                junction.centre + Vec2::new(angle.cos(), angle.sin()) * radius,
                ISLAND,
                prototypes,
                &mut rng,
                builder,
                &mut output,
                &mut counts,
            );
        }
    }

    // --- low shrubs and grass tufts -----------------------------------------
    let mut tufts = 0;
    'tufts: for parcel in parcels {
        if !matches!(parcel.use_type, ParcelUse::Park) {
            continue;
        }
        let ring: Vec<Vec2> = parcel
            .ring
            .iter()
            .map(|point| {
                let [x, z] = frame.to_local(*point);
                Vec2::new(x, z)
            })
            .collect();
        if ring.len() < 3 {
            continue;
        }
        let bounds = bounds_of(&ring);
        for _ in 0..40 {
            if tufts >= TUFT_BUDGET {
                break 'tufts;
            }
            let candidate = Vec2::new(
                bounds.0 + rng.unit() * (bounds.2 - bounds.0),
                bounds.1 + rng.unit() * (bounds.3 - bounds.1),
            );
            if !crate::math::point_in_ring(candidate, &ring) {
                continue;
            }
            // The tuft's albedo is in its texture; the instance tint only
            // varies it, and only downwards, for the same reason the tree tint
            // does.
            let green = 0.82 + rng.unit() * 0.18;
            builder.add_instance(
                "tuft",
                Instance {
                    x: candidate.x,
                    y: level::GROUND - 0.02,
                    z: candidate.y,
                    rotation_y: rng.unit() * TAU,
                    scale_x: 0.55 + rng.unit() * 0.5,
                    scale_y: 0.55 + rng.unit() * 0.5,
                    scale_z: 0.55 + rng.unit() * 0.5,
                    tint_r: green * 0.96,
                    tint_g: green,
                    tint_b: green * 0.90,
                },
            );
            tufts += 1;
        }
    }
    output.tufts = tufts;
    output.by_role = counts
        .into_iter()
        .map(|(role, count)| (format!("{role:?}").to_lowercase(), count))
        .collect();
    output
}

/// Distance from a point to a segment, plan metres.
fn segment_distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let len2 = ab.x * ab.x + ab.y * ab.y;
    let t = if len2 < 1.0e-9 { 0.0 } else { (((p.x - a.x) * ab.x + (p.y - a.y) * ab.y) / len2).clamp(0.0, 1.0) };
    let q = Vec2::new(a.x + ab.x * t, a.y + ab.y * t);
    ((p.x - q.x).powi(2) + (p.y - q.y).powi(2)).sqrt()
}

fn variants_of(species: &crate::species::Species) -> usize {
    two_variant_count(species)
}

/// A clipped shrub, instanced along medians and park edges, lives in
/// [`super::shrub`]; this function is kept next to the planting code it
/// belongs with.
fn bounds_of(ring: &[Vec2]) -> (f32, f32, f32, f32) {
    let mut bounds = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for point in ring {
        bounds.0 = bounds.0.min(point.x);
        bounds.1 = bounds.1.min(point.y);
        bounds.2 = bounds.2.max(point.x);
        bounds.3 = bounds.3.max(point.y);
    }
    bounds
}

/// Deterministic species choice for a parcel, shared with the plan generator so
/// a tree's species and its planting position cannot disagree.
///
/// This is the *plan* layer's species, from `urban`'s own three-variant enum;
/// the scene layer plants the sixteen-species palette in [`crate::species`]
/// and picks per role from a designed list. It stays because `urban` still
/// carries it on every `TreeInstance`, and a plan whose tree species cannot be
/// named is not auditable.
pub fn species_for_parcel(parcel: &Parcel, index: usize) -> TreeSpecies {
    let roll = modern::hash_u32(parcel.id, index as i32, 917);
    match parcel.use_type {
        ParcelUse::Park => {
            if roll < 0.34 {
                TreeSpecies::Willow
            } else {
                TreeSpecies::Ginkgo
            }
        }
        ParcelUse::Residential => {
            if roll < 0.5 {
                TreeSpecies::ChinesePlane
            } else {
                TreeSpecies::Ginkgo
            }
        }
        _ => TreeSpecies::ChinesePlane,
    }
}
