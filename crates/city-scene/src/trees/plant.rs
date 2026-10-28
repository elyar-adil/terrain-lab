//! Planting: where the city's trees go, and which species each site gets.
//!
//! A planting list is a *designed order*, not a uniform bag. The order is the
//! conventional one a Chinese landscape bureau plants in — the workhorse
//! avenue species first, the specials after — so a street reads as designed
//! rather than as a shuffle. Every list is drawn from the palette in
//! [`crate::species`].
//!
//! Only ChineseModern has lists of its own. Every other style plants from the
//! legacy lists below, exactly as the planting always did, so a WesternMixed
//! city does not move a tree.

use urban::{CityFrameInfo, CityStyle, Parcel, ParcelUse, Point, TreeSpecies, modern};

use super::{TAU, TREE_BUDGET, TUFT_BUDGET, TreePrototype, two_variant_count};
use crate::buildings::level;
use crate::math::{Rng, Vec2};
use crate::mesh::{Instance, MeshBuilder, box_at};
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
    /// A fruit tree in an orchard row.
    Orchard,
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeOutput {
    pub instances: usize,
    pub by_role: Vec<(String, usize)>,
    /// The audit table: `(role, species key, instance count)`, one row per
    /// species planted in each role. The species and role of every placed tree,
    /// in a form a report can read without the instance buffers.
    pub by_role_species: Vec<(String, String, usize)>,
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
const COMPOUND: &[&str] = &["xiang-zhang", "gui-hua", "yu-shu", "tao-shu", "he-huan"];

/// A riverbank in a Chinese city is a willow or a metasequoia avenue. The
/// Wuhan and Nanjing embankments are lined with them and so is half the
/// country.
const WATERFRONT: &[&str] = &["liu-shu", "shui-shan", "yang-shu", "yu-shu"];

/// An orchard is peach and its cousins; the grower knows more fruit than the old
/// prototype set does, and will say so when the set is widened.
const ORCHARD: &[&str] = &["tao-shu"];

/// A farmyard: poplars and willows, the locust and the elm, a peach by the door.
const FARM: &[&str] = &[
    "yang-shu",
    "liu-shu",
    "huai-shu",
    "yu-shu",
    "tao-shu",
    "ci-huai",
    "xiang-zhang",
];

/// A roundabout island is visible from every approach, so it gets the showy
/// small trees and the ginkgo.
const ISLAND: &[&str] = &["tao-shu", "he-huan", "gui-hua", "yin-xing"];

/// ChineseModern kerbside avenue and its median: the street-tree set a mainland
/// avenue is actually planted with. The plane (`wu-tong`, whose record is a
/// London plane, though its Chinese label is 梧桐) and the scholar tree carry
/// most of it; 香樟 and 银杏 are the evergreen and the gold; 白蜡 and 杨树 are the
/// hardy trees and the northern poplar. The poplar is planted city-wide because
/// a plan carries no latitude that would say which districts are north.
const CN_STREET: &[(&str, u32)] = &[
    ("huai-shu", 32),
    ("wu-tong", 28),
    ("xiang-zhang", 14),
    ("yin-xing", 9),
    ("bai-la", 9),
    ("yang-shu", 8),
];

/// A ChineseModern motorway verge: the columnar poplar, and the two hardiest
/// street trees for a hot verge and a sign's clearance.
const CN_MOTORWAY: &[(&str, u32)] = &[("yang-shu", 4), ("huai-shu", 3), ("bai-la", 3)];

/// ChineseModern parks: the scholar tree, the ginkgo and the camphor. The
/// magnolia that a Chinese park would also plant is not in the palette, so the
/// camphor takes its share.
const CN_PARK: &[(&str, u32)] = &[("huai-shu", 3), ("yin-xing", 3), ("xiang-zhang", 4)];

/// ChineseModern compound courtyards: the deodar and the juniper as the
/// conifers, the camphor and the osmanthus as the evergreen, and the two
/// flowering accents. 紫薇 is the crape myrtle; 海棠 has no record of its own,
/// and the peach is the nearest small spring-blossom tree the palette has.
const CN_COMPOUND: &[(&str, u32)] = &[
    ("xue-song", 3),
    ("long-bai", 3),
    ("xiang-zhang", 3),
    ("gui-hua", 4),
    ("zi-wei", 2),
    ("tao-shu", 2),
];

/// ChineseModern roundabout islands: the showy small trees and the ginkgo, with
/// the crape myrtle for the summer where the silk tree used to stand.
const CN_ISLAND: &[(&str, u32)] = &[
    ("gui-hua", 3),
    ("tao-shu", 2),
    ("yin-xing", 2),
    ("zi-wei", 2),
];

/// What a role plants from. The legacy lists are ordered, so the front of a list
/// is planted most. The ChineseModern lists are weighted: a real avenue is an
/// allocation, with its planes and scholar trees as shares of the whole, not a
/// ranking.
#[derive(Clone, Copy)]
enum Planting {
    Ordered(&'static [&'static str]),
    Weighted(&'static [(&'static str, u32)]),
}

/// Every list a city's trees are planted from, for one vocabulary.
#[derive(Clone, Copy)]
struct Lists {
    street: Planting,
    motorway: Planting,
    median: Planting,
    park: Planting,
    compound: Planting,
    waterfront: Planting,
    orchard: Planting,
    farm: Planting,
    island: Planting,
}

impl Lists {
    /// The lists every style planted from before ChineseModern had its own.
    /// WesternMixed keeps them exactly, and so does any style without lists of
    /// its own.
    const LEGACY: Lists = Lists {
        street: Planting::Ordered(AVENUE),
        motorway: Planting::Ordered(MOTORWAY),
        median: Planting::Ordered(MEDIAN),
        park: Planting::Ordered(PARK),
        compound: Planting::Ordered(COMPOUND),
        waterfront: Planting::Ordered(WATERFRONT),
        orchard: Planting::Ordered(ORCHARD),
        farm: Planting::Ordered(FARM),
        island: Planting::Ordered(ISLAND),
    };

    fn for_style(style: CityStyle) -> Lists {
        match style {
            CityStyle::ChineseModern => Lists {
                street: Planting::Weighted(CN_STREET),
                motorway: Planting::Weighted(CN_MOTORWAY),
                median: Planting::Weighted(CN_STREET),
                park: Planting::Weighted(CN_PARK),
                compound: Planting::Weighted(CN_COMPOUND),
                island: Planting::Weighted(CN_ISLAND),
                ..Lists::LEGACY
            },
            _ => Lists::LEGACY,
        }
    }
}

/// Plant every tree a city should have, from the legacy lists.
///
/// Kept for callers that do not carry the city's style. A caller that has the
/// style should use [`plant_styled`], which is the only way a ChineseModern
/// city gets its own species.
pub fn plant(
    network: &Network,
    parcels: &[Parcel],
    buildings: &[urban::ModernBuilding],
    fields: &[urban::Field],
    river: Option<&[Point]>,
    river_width: f32,
    frame: CityFrameInfo,
    prototypes: &[TreePrototype],
    builder: &mut MeshBuilder,
    seed: u32,
) -> TreeOutput {
    plant_with(
        network,
        parcels,
        buildings,
        fields,
        river,
        river_width,
        frame,
        prototypes,
        builder,
        seed,
        Lists::LEGACY,
    )
}

/// Plant every tree a city should have, from the lists of its style.
///
/// ChineseModern plants the street-tree set on its avenues and medians, the
/// scholar tree and the camphor in its parks, and the conifers and flowering
/// trees of a courtyard in its compounds. Every other style plants exactly what
/// [`plant`] plants.
pub fn plant_styled(
    network: &Network,
    parcels: &[Parcel],
    buildings: &[urban::ModernBuilding],
    fields: &[urban::Field],
    river: Option<&[Point]>,
    river_width: f32,
    frame: CityFrameInfo,
    prototypes: &[TreePrototype],
    builder: &mut MeshBuilder,
    seed: u32,
    style: CityStyle,
) -> TreeOutput {
    plant_with(
        network,
        parcels,
        buildings,
        fields,
        river,
        river_width,
        frame,
        prototypes,
        builder,
        seed,
        Lists::for_style(style),
    )
}

/// Pick one species from a planting list.
fn pick(list: Planting, rng: &mut Rng) -> &'static str {
    match list {
        Planting::Ordered(keys) => {
            // A planting list is a designed order, not a uniform bag: the front of
            // the list is what the city actually plants, and `u^1.35` is a gentle
            // enough skew that the rest still turns up.
            let roll = rng.unit().powf(1.35) * keys.len() as f32;
            keys[(roll as usize).min(keys.len() - 1)]
        }
        Planting::Weighted(entries) => {
            let total: u32 = entries.iter().map(|&(_, weight)| weight).sum();
            let mut edge = rng.unit() * total as f32;
            for &(key, weight) in entries {
                if edge < weight as f32 {
                    return key;
                }
                edge -= weight as f32;
            }
            entries[entries.len() - 1].0
        }
    }
}

fn role_name(role: TreeRole) -> String {
    format!("{role:?}").to_lowercase()
}

fn plant_with(
    network: &Network,
    parcels: &[Parcel],
    buildings: &[urban::ModernBuilding],
    fields: &[urban::Field],
    river: Option<&[Point]>,
    river_width: f32,
    frame: CityFrameInfo,
    prototypes: &[TreePrototype],
    builder: &mut MeshBuilder,
    seed: u32,
    lists: Lists,
) -> TreeOutput {
    super::declare(builder);
    let planted_from = builder.mark();
    let mut rng = Rng::new(seed ^ 0x74ee);
    let mut output = TreeOutput::default();
    let mut counts: Vec<(TreeRole, &'static str, usize)> = Vec::new();
    // A free function rather than a closure: the planting loops need the same
    // stream for their own jitter, and a closure capturing it would force those
    // draws through `place` instead.
    fn place(
        role: TreeRole,
        point: Vec2,
        list: Planting,
        prototypes: &[TreePrototype],
        rng: &mut Rng,
        builder: &mut MeshBuilder,
        output: &mut TreeOutput,
        counts: &mut Vec<(TreeRole, &'static str, usize)>,
    ) {
        if output.instances >= TREE_BUDGET {
            return;
        }
        let key = pick(list, rng);
        let Some(species) = by_key(key) else {
            return;
        };
        let variants = variants_of(species);
        let variant = rng.int(variants as u32) as u16;
        let index = prototypes
            .iter()
            .position(|prototype| {
                prototype.species.key == species.key && prototype.variant == variant
            })
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
        match counts
            .iter_mut()
            .find(|(existing, existing_key, _)| *existing == role && *existing_key == species.key)
        {
            Some(entry) => entry.2 += 1,
            None => counts.push((role, species.key, 1)),
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
        let half = crate::network::kerb_half(&section);
        // Every street with a footway wide enough for a pit is lined both
        // sides, at the 6 - 8 m Chinese streets are planted at (CJJ 75 §3.2),
        // in a straight line of pits a metre in from the kerb: never on the
        // paving itself, and clear of the lamps and poles in the same line.
        let sidewalk = section.sidewalk_metres;
        let pit = if sidewalk >= 4.5 {
            1.5
        } else if sidewalk >= 3.2 {
            1.25
        } else {
            1.0
        };
        let style = pit_style(road.class, sidewalk, road.id);
        let spacing = if road.is_motorway() { 10.0 } else { 8.0 };
        let lateral = half + 0.3 + pit * 0.5;
        let list = if road.is_motorway() {
            lists.motorway
        } else {
            lists.street
        };
        let obstacles = crate::furniture::footway_obstacles(road);
        // A footway too narrow for a pit carries no trees.
        let mut station = if sidewalk >= 2.4 {
            10.0 + (road.id % 4) as f32
        } else {
            f32::INFINITY
        };
        while station < length - 10.0 {
            for direction in [-1.0_f32, 1.0] {
                if obstacles
                    .iter()
                    .any(|(at, side)| *side == direction && (at - station).abs() < pit * 0.5 + 0.7)
                {
                    continue;
                }
                let point =
                    path.offset_at(station, road.kerbside(station, direction * lateral), 0.0);
                let at = Vec2::new(point.x, point.z);
                let before = output.instances;
                place(
                    TreeRole::Street,
                    at,
                    list,
                    prototypes,
                    &mut rng,
                    builder,
                    &mut output,
                    &mut counts,
                );
                if output.instances > before {
                    tree_pit(builder, at, path.tangent_at(station), pit, style);
                }
            }
            station += spacing;
        }
        // Median belt on a divided street: one line down the middle of the
        // hedge, clear of the median's lamps.
        if section.has_median() {
            let mut at = 20.0;
            while at < length - 20.0 {
                if crate::furniture::near_median_lamp(at, 2.0) {
                    at += 24.0;
                    continue;
                }
                let point = path.offset_at(at, 0.0, 0.0);
                place(
                    TreeRole::Median,
                    Vec2::new(point.x, point.z),
                    lists.median,
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
    let mut footprints: std::collections::HashMap<u32, Vec<Vec<Vec2>>> =
        std::collections::HashMap::new();
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
            ParcelUse::Park => (TreeRole::Park, 10, lists.park),
            ParcelUse::Residential if parcel.compound => (TreeRole::Compound, 8, lists.compound),
            ParcelUse::Civic => (TreeRole::Park, 5, lists.park),
            ParcelUse::Villa => (
                TreeRole::Compound,
                3 + (parcel.id % 4) as usize,
                lists.compound,
            ),
            ParcelUse::Farmstead => (TreeRole::Compound, 2 + (parcel.id % 5) as usize, lists.farm),
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
                                segment_distance(candidate, house[i], house[(i + 1) % house.len()])
                                    < 3.2
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

    // --- orchards: fruit trees in rows ------------------------------------------
    for field in fields.iter().filter(|f| f.crop == urban::CropKind::Orchard) {
        let ring: Vec<Vec2> = field
            .ring
            .iter()
            .map(|point| {
                let [x, z] = frame.to_local(*point);
                Vec2::new(x, z)
            })
            .collect();
        let inner = crate::math::inset_ring(&ring, 2.0);
        if inner.len() < 3 {
            continue;
        }
        let (sin, cos) = field.row_angle.sin_cos();
        let (mut u0, mut u1, mut v0, mut v1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for p in &ring {
            let (u, v) = (p.x * cos + p.y * sin, -p.x * sin + p.y * cos);
            u0 = u0.min(u);
            u1 = u1.max(u);
            v0 = v0.min(v);
            v1 = v1.max(v);
        }
        let mut planted = 0;
        let mut v = v0 + 3.0;
        while v < v1 - 1.5 && planted < 70 {
            let mut u = u0 + 3.0 + rng.unit() * 1.5;
            while u < u1 - 1.5 && planted < 70 {
                let spot = Vec2::new(
                    (u + (rng.unit() - 0.5) * 0.8) * cos - v * sin,
                    (u + (rng.unit() - 0.5) * 0.8) * sin + v * cos,
                );
                if crate::math::point_in_ring(spot, &inner) {
                    place(
                        TreeRole::Orchard,
                        spot,
                        lists.orchard,
                        prototypes,
                        &mut rng,
                        builder,
                        &mut output,
                        &mut counts,
                    );
                    planted += 1;
                }
                u += 5.0;
            }
            v += 5.5;
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
                    lists.waterfront,
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
                lists.island,
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
    // Nothing grows under a viaduct's deck: in the dark and dry between its
    // piers a tree would also stand up through the road above it. Each deck
    // as its plan line, with its half-width and a crown's reach.
    let decks: Vec<(Vec<Vec2>, f32)> = network
        .roads
        .iter()
        .filter(|road| road.layer != 0)
        .map(|road| {
            let length = road.carriageway.length();
            let steps = ((length / 4.0).ceil() as usize).max(1);
            let plan: Vec<Vec2> = (0..=steps)
                .map(|k| road.carriageway.plan_at(length * k as f32 / steps as f32))
                .collect();
            (plan, road.half_width() + 4.0)
        })
        .collect();
    if !decks.is_empty() {
        let dropped = builder.retain_instances_since(&planted_from, |i| {
            let p = Vec2::new(i.x, i.z);
            !decks
                .iter()
                .any(|(plan, reach)| crate::math::distance_to_polyline(p, plan) < *reach)
        });
        output.instances = output.instances.saturating_sub(dropped);
    }
    // Roles in the order they were first planted, as they always were.
    let mut by_role: Vec<(TreeRole, usize)> = Vec::new();
    for (role, _, count) in &counts {
        match by_role.iter_mut().find(|(existing, _)| existing == role) {
            Some(entry) => entry.1 += count,
            None => by_role.push((*role, *count)),
        }
    }
    output.by_role = by_role
        .into_iter()
        .map(|(role, count)| (role_name(role), count))
        .collect();
    output.by_role_species = counts
        .into_iter()
        .map(|(role, key, count)| (role_name(role), key.to_string(), count))
        .collect();
    output
}

/// Distance from a point to a segment, plan metres.
/// How a street's tree pits are finished (树池), one way down a street, from
/// photographs of Chinese footways: a raised granite planter of grass on a
/// wide boulevard footway, a cast-iron grate over gravel on a busy street, a
/// pit of grass in a granite edge elsewhere.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PitStyle {
    Grass,
    Grate,
    Planter,
}

fn pit_style(class: urban::ModernRoadClass, sidewalk: f32, id: u32) -> PitStyle {
    use urban::ModernRoadClass::*;
    match class {
        Arterial if sidewalk >= 5.0 && id.is_multiple_of(2) => PitStyle::Planter,
        Arterial | Collector if !id.is_multiple_of(3) => PitStyle::Grate,
        _ => PitStyle::Grass,
    }
}

/// A tree pit `size` square round `at`, its sides along `along`, set into
/// the footway (whose top is at kerb height).
fn tree_pit(builder: &mut MeshBuilder, at: Vec2, along: Vec2, size: f32, style: PitStyle) {
    let walk = crate::street::level::KERB;
    let across = Vec2::new(-along.y, along.x);
    let square = |h: f32| -> Vec<Vec2> {
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .iter()
            .map(|(u, v)| at + along * (u * h) + across * (v * h))
            .collect()
    };
    let rotation = along.y.atan2(along.x);
    let edge = 0.1;
    let frame = |builder: &mut MeshBuilder, material: &str, h: f32, y: f32, tall: f32| {
        for k in 0..4 {
            let (u, v) = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)][k];
            let centre = at + along * (u * (h - edge * 0.5)) + across * (v * (h - edge * 0.5));
            let (w, d) = if u != 0.0 {
                (edge, h * 2.0)
            } else {
                (h * 2.0 - 2.0 * edge, edge)
            };
            box_at(builder, material, centre, y, w, tall, d, rotation);
        }
    };
    let h = size * 0.5;
    match style {
        PitStyle::Planter => {
            let (h, tall) = (h + 0.1, 0.45);
            frame(builder, "treepit.planter", h, walk + tall * 0.5, tall);
            builder.ground_uv("median.plant", &square(h - edge), walk + tall - 0.04, None);
        }
        PitStyle::Grate => {
            frame(builder, "treepit.edge", h, walk + 0.005, 0.03);
            builder.ground_uv("treepit.grate", &square(h - edge), walk + 0.006, None);
            let disc: Vec<Vec2> = (0..14)
                .map(|k| {
                    let a = k as f32 / 14.0 * TAU;
                    at + Vec2::new(a.cos(), a.sin()) * (h * 0.42)
                })
                .collect();
            builder.ground_uv("treepit.gravel", &disc, walk + 0.012, None);
        }
        PitStyle::Grass => {
            frame(builder, "treepit.edge", h, walk + 0.01, 0.04);
            builder.ground_uv("median.plant", &square(h - edge), walk + 0.008, None);
        }
    }
}

fn segment_distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let len2 = ab.x * ab.x + ab.y * ab.y;
    let t = if len2 < 1.0e-9 {
        0.0
    } else {
        (((p.x - a.x) * ab.x + (p.y - a.y) * ab.y) / len2).clamp(0.0, 1.0)
    };
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
