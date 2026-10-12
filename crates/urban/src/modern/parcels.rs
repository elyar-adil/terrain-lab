//! Blocks, lots and building massing derived from the *actual* road graph.
//!
//! Blocks are the bounded faces of the planar street graph.  Each block is set
//! back from every bounding street by that street's real right-of-way (half
//! carriageway + sidewalk + a building line), split into lots by recursive
//! oriented-box bisection, and massed inside the lot.  Because the envelope is
//! computed from the streets themselves, a building can never sit on a road,
//! whatever shape the network has.

use super::CityFrame;
use super::blocks::{Extraction, extract_faces, split_concave};
use super::geom::{
    Obb, V, centroid, clip_half, obb, point_in, point_seg_dist, ring_polyline_dist, seg_seg_dist,
    signed_area,
};
use super::graph::{QUAY_OFF_M, modern_hash};
use super::suburb;
use crate::model::cross_section;
use crate::{
    BuildingFacade, CityStyle, ModernBuilding, ModernRoadClass, Parcel, ParcelUse, Point,
    RoofStyle, SdNode, SdRoad, UrbanBlock,
};
use worldgen_core::smoothstep;

pub(super) struct ParcelOutput {
    pub blocks: Vec<UrbanBlock>,
    pub parcels: Vec<Parcel>,
    pub buildings: Vec<ModernBuilding>,
    pub fields: Vec<crate::Field>,
}

const PODIUM_FLOOR_M: f32 = 4.5;
/// Gap left between two lots that share a bisection line.
const LOT_GAP_M: f32 = 1.5;
/// Extra clearance between a footprint and its lot line.
const LOT_MARGIN_M: f32 = 0.8;
/// Building line: distance from the back of the sidewalk to the wall.
const BUILDING_LINE_M: f32 = 1.5;
/// Clearance kept between any building lot and the river bank.
const RIVER_BANK_M: f32 = 4.0;

/// Distance from a street centreline to the nearest permitted wall.
pub(super) fn right_of_way(class: ModernRoadClass) -> f32 {
    let s = cross_section(class);
    s.width_metres * 0.5 + s.sidewalk_metres + BUILDING_LINE_M
}

/// Setback of face edge `i`: a street's right-of-way, or half the lot gap along
/// a cut between two pieces of the same block.
fn edge_row(face: &super::blocks::Face, i: usize) -> f32 {
    if face.open[i] {
        LOT_GAP_M * 0.5
    } else {
        right_of_way(face.classes[i])
    }
}

pub(super) fn build_parcels(
    frame: &CityFrame,
    nodes: &[SdNode],
    sd_roads: &[SdRoad],
) -> ParcelOutput {
    let radius_m = frame.radius_m;
    let density = frame.density;
    let river_half = frame.river_half;
    let seed = frame.spec.seed;
    // The Chinese vocabulary (plaster slabs, a height that follows the centre)
    // is only for the Chinese style; `WesternMixed` keeps its brick stock.
    let chinese = frame.spec.style == CityStyle::ChineseModern;

    let pts: Vec<V> = nodes.iter().map(|n| frame.to_local(n.point)).collect();
    let edges: Vec<(usize, usize, ModernRoadClass)> = sd_roads
        .iter()
        .filter(|r| !r.bridge)
        .map(|r| (r.from as usize, r.to as usize, r.class))
        .collect();
    let Extraction { faces, spurs } = extract_faces(&pts, &edges);
    let faces: Vec<_> = faces.into_iter().flat_map(split_concave).collect();

    let river_local: Vec<V> = frame.river_line();

    let mut blocks: Vec<UrbanBlock> = Vec::new();
    let mut parcels: Vec<Parcel> = Vec::new();
    let mut buildings: Vec<ModernBuilding> = Vec::new();
    let mut next_parcel = 0_u32;
    let mut next_building = 0_u32;
    let mut fields: Vec<crate::Field> = Vec::new();
    let mut next_field = 0_u32;
    let to_world =
        |ring: &[V]| -> Vec<Point> { ring.iter().map(|p| frame.to_world(p.0, p.1)).collect() };

    for (face_index, face) in faces.iter().enumerate() {
        let centre = centroid(&face.ring);
        if centre.0.hypot(centre.1) > radius_m * 1.04 {
            continue;
        }
        // How built-up this block is: 1 in the town, thinning to scattered houses
        // at the edge, where buildings are also lower and more widely spaced.
        let built = frame.urbanness(centre.0, centre.1);
        // A big face of the country can have its middle in the fields and its
        // edge in the suburb: judge it by the most built-up place on its boundary.
        let face_built = if frame.external.is_some() {
            face.ring
                .iter()
                .fold(built, |m, p| m.max(frame.urbanness(p.0, p.1)))
        } else {
            built
        };
        // (Open country is not skipped when the streets come from outside: it is farmland.)
        if frame.organic_footprint && frame.external.is_none() && face_built < 0.06 {
            continue;
        }
        if signed_area(&face.ring) < 900.0 {
            continue;
        }
        let fi = face_index as i32;
        let centrality = frame.core_weight(centre.0, centre.1)
            * if frame.organic_footprint {
                0.35 + 0.65 * built
            } else {
                1.0
            };

        // ---- street setbacks -> buildable envelope ----
        let mut envelope = face.ring.clone();
        for i in 0..face.ring.len() {
            let a = face.ring[i];
            let b = face.ring[(i + 1) % face.ring.len()];
            let len = face.edge_len[i];
            if len < 1.0e-3 {
                continue;
            }
            let n = (-(b.1 - a.1) / len, (b.0 - a.0) / len);
            envelope = clip_half(&envelope, n, n.0 * a.0 + n.1 * a.1 + edge_row(face, i));
            if envelope.len() < 3 {
                break;
            }
        }
        if envelope.len() < 3 {
            continue;
        }
        // The block's ground plate stops under the kerb, not at the street
        // centre line: the plate sits above the carriageway datum, so a plate
        // that reached the centre line would paint grass over half of every road.
        let mut plate = face.ring.clone();
        for i in 0..face.ring.len() {
            let a = face.ring[i];
            let b = face.ring[(i + 1) % face.ring.len()];
            let len = face.edge_len[i];
            if len < 1.0e-3 {
                continue;
            }
            let n = (-(b.1 - a.1) / len, (b.0 - a.0) / len);
            let inset = if face.open[i] {
                0.0
            } else {
                (right_of_way(face.classes[i]) - BUILDING_LINE_M - 1.2).max(1.0)
            };
            plate = clip_half(&plate, n, n.0 * a.0 + n.1 * a.1 + inset);
            if plate.len() < 3 {
                break;
            }
        }
        if plate.len() < 3 {
            continue;
        }

        // ---- split off the river: each bank is its own block ----
        let near_river =
            ring_polyline_dist(&face.ring, &river_local) < river_half + RIVER_BANK_M + 4.0;
        let (block_rings, envelopes): (Vec<Vec<V>>, Vec<Vec<V>>) = if near_river {
            let bank = river_half + RIVER_BANK_M;
            let split = |ring: &[V]| -> Vec<Vec<V>> {
                let c = centroid(ring);
                let (n, d) = frame.river_split(c);
                [
                    clip_half(ring, n, d + bank),
                    clip_half(ring, (-n.0, -n.1), -d + bank),
                ]
                .into_iter()
                .filter(|p| p.len() >= 3 && signed_area(p) > 300.0)
                .collect()
            };
            let banks = split(&plate);
            let mut envs = split(&envelope);
            envs.retain(|e| signed_area(e) > 300.0);
            (banks, envs)
        } else {
            (vec![plate.clone()], vec![envelope])
        };
        for ring in &block_rings {
            blocks.push(UrbanBlock {
                boundary: to_world(ring),
                courtyard: None,
            });
        }
        if block_rings.is_empty() {
            continue;
        }
        // Parcels reference the block that contains their envelope.
        let block_base = (blocks.len() - block_rings.len()) as u32;

        // Outside the town proper, houses follow the streets instead of filling
        // the block: the suburb, then the farmsteads.
        if frame.external.is_some() && built < suburb::TOWN_BUILT {
            suburb::place_frontage(
                &suburb::Frontage {
                    frame,
                    face,
                    face_index: fi,
                    envelopes: &envelopes,
                    spurs: &spurs,
                    seed,
                    block_base,
                },
                &mut suburb::Sink {
                    parcels: &mut parcels,
                    buildings: &mut buildings,
                    next_parcel: &mut next_parcel,
                    next_building: &mut next_building,
                    fields: &mut fields,
                    next_field: &mut next_field,
                },
            );
            continue;
        }

        let block_noise = modern_hash(seed, fi, 0, 701);
        let face_area = signed_area(&face.ring);
        let park_block = block_noise < 0.05 + 0.06 * (1.0 - density)
            || (face_area > 14_000.0 && block_noise < 0.10);

        // Widest street around this block: where compound gates face.
        let widest = (0..face.ring.len())
            .filter(|i| !face.open[*i])
            .max_by_key(|i| face.classes[*i] as i32)
            .unwrap_or(0);
        let (wa, wb) = (face.ring[widest], face.ring[(widest + 1) % face.ring.len()]);

        // A few very large faces stay one lot: a walled compound or campus.
        let superblock =
            face_area > 13_000.0 && centrality < 0.55 && modern_hash(seed, fi, 4, 777) < 0.45;
        let target = if superblock {
            1.0e9
        } else {
            900.0 + 1500.0 * (1.0 - centrality) * (0.6 + 0.8 * modern_hash(seed, fi, 3, 761))
        };
        for (env_index, env) in envelopes.iter().enumerate() {
            let mut lots: Vec<Vec<V>> = Vec::new();
            subdivide(env, target, seed, fi * 8 + env_index as i32, 0, &mut lots);
            let block_id = block_base + (env_index as u32).min(block_rings.len() as u32 - 1);
            for (lot_index, lot) in lots.iter().enumerate() {
                let ob = obb(lot);
                let lot_area = signed_area(lot);
                if lot.len() < 4 || lot_area < 300.0 || ob.width().min(ob.depth()) < 10.0 {
                    continue;
                }
                let key = fi * 64 + (env_index as i32) * 16 + lot_index as i32;
                let n = modern_hash(seed, key, 1, 719);
                let noise = |salt: i32| modern_hash(seed, key, 2, salt);
                // The fringe is patchy: lots drop out more often the nearer the edge.
                if frame.organic_footprint && built < 1.0 {
                    let lot_c = centroid(lot);
                    let lot_built = frame.urbanness(lot_c.0, lot_c.1);
                    if noise(791) < (1.0 - lot_built).powf(0.8) * 0.9 {
                        continue;
                    }
                }
                let waterfront =
                    ring_polyline_dist(lot, &river_local) < river_half + QUAY_OFF_M + 45.0;
                let plaza = centrality > 0.45 && lot_area < 3_400.0 && noise(777) < 0.09;
                let use_type = if park_block || plaza || (waterfront && n > 0.82) {
                    ParcelUse::Park
                } else if n < 0.32 + 0.24 * centrality {
                    ParcelUse::Commercial
                } else if n < 0.55 {
                    ParcelUse::MixedUse
                } else if n < 0.60 {
                    ParcelUse::Civic
                } else {
                    ParcelUse::Residential
                };
                let use_type = if superblock && !matches!(use_type, ParcelUse::Park) {
                    ParcelUse::Residential
                } else {
                    use_type
                };
                let compound = matches!(use_type, ParcelUse::Residential)
                    && ob.width() > 55.0
                    && ob.depth() > 40.0
                    && (superblock || (centrality < 0.5 && noise(779) < 0.6));
                let parcel_id = next_parcel;
                next_parcel += 1;
                let gate_edge = (0..lot.len())
                    .min_by(|x, y| {
                        let mid = |i: usize| {
                            let (p, q) = (lot[i], lot[(i + 1) % lot.len()]);
                            ((p.0 + q.0) * 0.5, (p.1 + q.1) * 0.5)
                        };
                        point_seg_dist(mid(*x), wa, wb).total_cmp(&point_seg_dist(mid(*y), wa, wb))
                    })
                    .unwrap_or(0)
                    .min(255) as u8;
                parcels.push(Parcel {
                    id: parcel_id,
                    block_id,
                    ring: to_world(lot),
                    use_type,
                    compound,
                    gate_edge,
                });
                if matches!(use_type, ParcelUse::Park) {
                    continue;
                }
                // Buildable rectangle inside the lot, in the lot's own frame.
                let (bx0, bx1) = (ob.min_u + LOT_MARGIN_M, ob.max_u - LOT_MARGIN_M);
                let (bz0, bz1) = (ob.min_v + LOT_MARGIN_M, ob.max_v - LOT_MARGIN_M);
                let width = bx1 - bx0;
                let depth = bz1 - bz0;
                if width < 14.0 || depth < 14.0 {
                    continue;
                }
                let parcel_area = lot_area.min(width * depth * 1.15);
                // Clear of every dead-end street's right-of-way.
                let spur_clear = |ring: &[V]| -> bool {
                    spurs.iter().all(|(a, b, class)| {
                        let row = right_of_way(*class);
                        ring.iter().all(|p| point_seg_dist(*p, *a, *b) >= row)
                            && ring
                                .iter()
                                .zip(ring.iter().cycle().skip(1))
                                .all(|(p, q)| seg_seg_dist(*p, *q, *a, *b) >= row)
                    })
                };

                let mut emit = |footprint: Vec<V>,
                                floors: u16,
                                use_type: ParcelUse,
                                podium_floors: u16,
                                facade: BuildingFacade,
                                roof: RoofStyle,
                                balconies: bool| {
                    // Fit the footprint inside the lot; shrink about its
                    // centre until every corner is inside and it clears any
                    // dead-end street, else drop it.
                    let fc = centroid(&footprint);
                    let (fw, fd) = (width, depth);
                    let shifts = [
                        (0.0_f32, 0.0_f32),
                        (0.12, 0.0),
                        (-0.12, 0.0),
                        (0.0, 0.12),
                        (0.0, -0.12),
                        (0.25, 0.0),
                        (-0.25, 0.0),
                        (0.0, 0.25),
                        (0.0, -0.25),
                    ];
                    let placed = [1.0_f32, 0.92, 0.84, 0.76, 0.68, 0.58, 0.5]
                        .iter()
                        .find_map(|s| {
                            shifts.iter().find_map(|(sx, sz)| {
                                let ring: Vec<V> = footprint
                                    .iter()
                                    .map(|p| {
                                        ob.to_world(
                                            fc.0 + (p.0 - fc.0) * s + sx * fw,
                                            fc.1 + (p.1 - fc.1) * s + sz * fd,
                                        )
                                    })
                                    .collect();
                                (spur_clear(&ring) && ring.iter().all(|p| point_in(lot, *p)))
                                    .then_some(ring)
                            })
                        });
                    let Some(ring) = placed else {
                        return;
                    };
                    let max_side = ring
                        .iter()
                        .zip(ring.iter().cycle().skip(1))
                        .take(ring.len())
                        .map(|(a, b)| (a.0 - b.0).hypot(a.1 - b.1))
                        .fold(0.0_f32, f32::max);
                    let facade_kind = match use_type {
                        ParcelUse::Commercial => procedural::FacadeKind::CurtainWall,
                        ParcelUse::MixedUse => procedural::FacadeKind::ConcreteGlass,
                        ParcelUse::Civic => procedural::FacadeKind::StoneCivic,
                        ParcelUse::Residential
                        | ParcelUse::Villa
                        | ParcelUse::Farmstead
                        | ParcelUse::Park => procedural::FacadeKind::Residential,
                    };
                    let mut floors = floors;
                    // On a landscape the town grew on its own, a village is low and only a
                    // downtown is tall: the storeys a building may have follow how intense
                    // the development is where it stands.
                    if frame.external.is_some() {
                        floors = floors.min((2.0 + 40.0 * centrality.powi(3)) as u16).max(2);
                    }
                    let mut metrics =
                        procedural::facade_metrics(max_side, max_side * 0.8, floors, facade_kind);
                    let podium_height = podium_floors as f32 * PODIUM_FLOOR_M;
                    // No needles: height stays proportionate to the footprint's
                    // short side (about 4.4 : 1 at the very most).
                    let short_side = signed_area(&ring) / max_side.max(1.0);
                    let height_cap = 4.4 * short_side - podium_height - 1.2;
                    let cap_floors = (height_cap / metrics.storey_height_m).floor().max(3.0) as u16;
                    if floors > cap_floors {
                        floors = cap_floors;
                        metrics = procedural::facade_metrics(
                            max_side,
                            max_side * 0.8,
                            floors,
                            facade_kind,
                        );
                    }
                    let floor_height = metrics.storey_height_m;
                    buildings.push(ModernBuilding {
                        id: next_building,
                        parcel_id,
                        footprint: to_world(&ring),
                        height_metres: floors as f32 * floor_height + podium_height + 1.2,
                        floors,
                        use_type,
                        roof,
                        facade,
                        podium_height_metres: podium_height,
                        window_bays: metrics.bays,
                        balcony_bays: if balconies {
                            (metrics.bays.saturating_sub(1)).clamp(1, 30)
                        } else {
                            0
                        },
                        entrance_count: if matches!(
                            use_type,
                            ParcelUse::Commercial | ParcelUse::MixedUse
                        ) {
                            ((metrics.bays / 3).max(1) as u8).clamp(1, 8)
                        } else {
                            1
                        },
                    });
                    next_building += 1;
                };
                match use_type {
                    ParcelUse::Commercial => {
                        let podium_floors = if centrality < 0.3 || noise(731) < 0.5 {
                            3
                        } else {
                            4
                        };
                        emit(
                            rect(bx0, bx1, bz0, bz1, 0.06),
                            podium_floors,
                            ParcelUse::Commercial,
                            0,
                            BuildingFacade::CurtainWall,
                            RoofStyle::Flat,
                            false,
                        );
                        let tower_area = 24.0 * 20.0;
                        let _ = tower_area;
                        let tower_floors = ((6.0
                            + 30.0 * centrality.powf(1.15) * (0.7 + 0.3 * density))
                            * (0.72 + 0.56 * noise(745)))
                        .round()
                        .clamp(5.0, 38.0) as u16;
                        let size = 0.85 + 0.3 * centrality;
                        let tower_w = 30.0 * size * (0.85 + noise(737) * 0.3);
                        let tower_d = 26.0 * size * (0.85 + noise(739) * 0.3);
                        let towers = if parcel_area > 6_800.0 && noise(741) < 0.45 {
                            2
                        } else {
                            1
                        };
                        for t in 0..towers {
                            let (tx, tz) = if towers == 2 {
                                (
                                    if t == 0 {
                                        bx0 + width * 0.26
                                    } else {
                                        bx0 + width * 0.74
                                    },
                                    bz0 + depth * (0.30 + 0.4 * noise(743)),
                                )
                            } else {
                                (bx0 + width * 0.5, bz0 + depth * (0.32 + 0.36 * noise(743)))
                            };
                            emit(
                                rect(
                                    tx - tower_w * 0.5,
                                    tx + tower_w * 0.5,
                                    tz - tower_d * 0.5,
                                    tz + tower_d * 0.5,
                                    0.0,
                                ),
                                tower_floors,
                                ParcelUse::Commercial,
                                podium_floors,
                                BuildingFacade::CurtainWall,
                                if tower_floors >= 30 {
                                    RoofStyle::SetbackTower
                                } else {
                                    RoofStyle::Flat
                                },
                                false,
                            );
                        }
                    }
                    ParcelUse::MixedUse => {
                        let podium_floors = 3;
                        emit(
                            rect(bx0, bx1, bz0, bz1, 0.10),
                            podium_floors,
                            ParcelUse::MixedUse,
                            0,
                            BuildingFacade::ConcreteGlass,
                            RoofStyle::Flat,
                            false,
                        );
                        let tower_area = 22.0 * 18.0;
                        let _ = tower_area;
                        let tower_floors = ((5.0
                            + 24.0 * centrality.powf(1.15) * (0.7 + 0.3 * density))
                            * (0.72 + 0.56 * noise(755)))
                        .round()
                        .clamp(4.0, 32.0) as u16;
                        let tower_w = 22.0 * (0.9 + noise(747) * 0.2);
                        let tower_d = 18.0 * (0.9 + noise(749) * 0.2);
                        emit(
                            rect(
                                bx0 + width * 0.5 - tower_w * 0.5,
                                bx0 + width * 0.5 + tower_w * 0.5,
                                bz0 + depth * 0.5 - tower_d * 0.5,
                                bz0 + depth * 0.5 + tower_d * 0.5,
                                0.0,
                            ),
                            tower_floors,
                            ParcelUse::Residential,
                            podium_floors,
                            BuildingFacade::ConcreteGlass,
                            if tower_floors >= 30 {
                                RoofStyle::SetbackTower
                            } else {
                                RoofStyle::Flat
                            },
                            true,
                        );
                    }
                    ParcelUse::Civic => {
                        let slab_w = width * 0.62;
                        let slab_d = depth * 0.30;
                        emit(
                            rect(
                                bx0 + width * 0.5 - slab_w * 0.5,
                                bx0 + width * 0.5 + slab_w * 0.5,
                                bz0 + depth * 0.28 - slab_d * 0.5,
                                bz0 + depth * 0.28 + slab_d * 0.5,
                                0.0,
                            ),
                            (4.0 + noise(751) * 1.9).round() as u16,
                            ParcelUse::Civic,
                            0,
                            BuildingFacade::StoneCivic,
                            RoofStyle::Flat,
                            false,
                        );
                    }
                    ParcelUse::Residential if !compound => {
                        // A street-wall block: the building fills its lot and
                        // meets the pavement, as in any dense city centre.
                        let (floors, facade) = if chinese {
                            (
                                chinese_residential_storeys(centrality, density, noise(763)),
                                BuildingFacade::PlasterResidential,
                            )
                        } else {
                            let floors =
                                (4.0 + noise(763) * 5.0 + 11.0 * centrality.powf(1.2) * density)
                                    .round() as u16;
                            (floors.clamp(4, 18), BuildingFacade::BrickResidential)
                        };
                        emit(
                            rect(bx0, bx1, bz0, bz1, 0.03 + 0.2 * (1.0 - centrality)),
                            floors,
                            ParcelUse::Residential,
                            0,
                            facade,
                            RoofStyle::Flat,
                            true,
                        );
                    }
                    ParcelUse::Residential if chinese => {
                        // A walled compound of mixed blocks: see `chinese_compound_cells`.
                        let fits = |ring: &[V]| -> bool {
                            let world: Vec<V> =
                                ring.iter().map(|p| ob.to_world(p.0, p.1)).collect();
                            spur_clear(&world) && world.iter().all(|p| point_in(lot, *p))
                        };
                        let site = Site {
                            x0: bx0,
                            x1: bx1,
                            z0: bz0,
                            z1: bz1,
                        };
                        let lot_uv: Vec<V> = lot
                            .iter()
                            .map(|p| (p.0 * ob.u.0 + p.1 * ob.u.1, p.0 * ob.v.0 + p.1 * ob.v.1))
                            .collect();
                        for (ring, floors) in chinese_compound_cells(
                            site, &lot_uv, key, seed, centrality, density, &fits,
                        ) {
                            let roof = if floors >= 30 {
                                RoofStyle::SetbackTower
                            } else {
                                RoofStyle::Flat
                            };
                            emit(
                                ring,
                                floors,
                                ParcelUse::Residential,
                                0,
                                BuildingFacade::PlasterResidential,
                                roof,
                                true,
                            );
                        }
                    }
                    ParcelUse::Residential => {
                        // A walled compound: the lot is tiled into cells, each
                        // holding a slab or tower, a minority left as garden.
                        let slab_style = noise(753) < 0.5;
                        let (cw, cd) = if slab_style {
                            (46.0, 28.0)
                        } else {
                            (34.0, 30.0)
                        };
                        let nx = ((width / cw).floor() as usize).max(1);
                        let nz = ((depth / cd).floor() as usize).max(1);
                        let (cell_w, cell_d) = (width / nx as f32, depth / nz as f32);
                        for i in 0..nx {
                            for j in 0..nz {
                                let ck = key * 97 + (i * 7 + j) as i32;
                                let cn = |salt: i32| modern_hash(seed, ck, 3, salt);
                                if cn(781) < 0.05 + 0.12 * (1.0 - centrality) {
                                    continue;
                                }
                                let cx = bx0 + cell_w * (i as f32 + 0.5);
                                let cz = bz0 + cell_d * (j as f32 + 0.5);
                                let (floors, facade) = if chinese {
                                    (
                                        chinese_residential_storeys(centrality, density, cn(783)),
                                        BuildingFacade::PlasterResidential,
                                    )
                                } else {
                                    let floors = (6.0
                                        + (4.0 + 22.0 * centrality) * (0.5 + 0.7 * cn(783)))
                                    .round()
                                    .clamp(6.0, 34.0)
                                        as u16;
                                    (floors, BuildingFacade::BrickResidential)
                                };
                                let (w, d) = if slab_style {
                                    ((cell_w * 0.86).min(72.0), 12.5 + cn(785) * 2.5)
                                } else {
                                    (
                                        (23.0 * (0.88 + cn(787) * 0.24)).min(cell_w * 0.82),
                                        (19.0 * (0.88 + cn(789) * 0.24)).min(cell_d * 0.82),
                                    )
                                };
                                emit(
                                    rect(
                                        cx - w * 0.5,
                                        cx + w * 0.5,
                                        cz - d * 0.5,
                                        cz + d * 0.5,
                                        0.0,
                                    ),
                                    floors,
                                    ParcelUse::Residential,
                                    0,
                                    facade,
                                    if floors >= 30 {
                                        RoofStyle::SetbackTower
                                    } else {
                                        RoofStyle::Flat
                                    },
                                    true,
                                );
                            }
                        }
                    }
                    ParcelUse::Park | ParcelUse::Villa | ParcelUse::Farmstead => {}
                }
            }
        }
    }
    // Farmland along the country streets, around what is already there.
    let mut occupied: Vec<Vec<V>> = parcels
        .iter()
        .map(|p| p.ring.iter().map(|q| frame.to_local(*q)).collect())
        .chain(
            fields
                .iter()
                .map(|f| f.ring.iter().map(|q| frame.to_local(*q)).collect()),
        )
        .collect();
    if frame.external.is_some() {
        suburb::roadside_fields(
            frame,
            &pts,
            &edges,
            &river_local,
            river_half,
            seed,
            &mut occupied,
            &mut fields,
            &mut next_field,
        );
    }
    ParcelOutput {
        blocks,
        parcels,
        buildings,
        fields,
    }
}

/// Recursive oriented-box bisection into lots no larger than `target` m².
fn subdivide(poly: &[V], target: f32, seed: u32, key: i32, depth: u32, out: &mut Vec<Vec<V>>) {
    let area = signed_area(poly);
    let ob: Obb = obb(poly);
    let (w, d) = (ob.width(), ob.depth());
    if area <= target || depth >= 6 || w.max(d) < 30.0 {
        out.push(poly.to_vec());
        return;
    }
    let t = 0.42 + 0.16 * modern_hash(seed, key, depth as i32, 769 + depth as i32);
    // Cut across the longer side.
    let (axis, lo, hi) = if w >= d {
        (ob.u, ob.min_u, ob.max_u)
    } else {
        (ob.v, ob.min_v, ob.max_v)
    };
    let s = lo + (hi - lo) * t;
    let a = clip_half(poly, (-axis.0, -axis.1), -(s - LOT_GAP_M));
    let b = clip_half(poly, axis, s + LOT_GAP_M);
    if a.len() < 3 || b.len() < 3 || signed_area(&a) < 320.0 || signed_area(&b) < 320.0 {
        out.push(poly.to_vec());
        return;
    }
    subdivide(&a, target, seed, key * 2, depth + 1, out);
    subdivide(&b, target, seed, key * 2 + 1, depth + 1, out);
}

/// Storeys of a residential building in a Chinese city, from how central its lot
/// is, the city's density and a draw `u` in 0..1.  The town is a gradient rather
/// than a zoning map: the centre carries 18–28-storey slabs and point towers, the
/// planned rows at the edge are 6–11-storey slabs, and the band between is dense
/// mid-rise that rises towards the core.  Density scales only the tall part, as
/// it does for the commercial towers.  Storey height and the height cap are
/// applied by the caller, so a narrow slab still stops short of a needle.
fn chinese_residential_storeys(centrality: f32, density: f32, u: f32) -> u16 {
    let tall = 13.0 * (0.7 + 0.3 * density);
    let median = 8.5 + tall * smoothstep(0.20, 0.42, centrality);
    (median * (0.7 + 0.6 * u)).round().clamp(6.0, 28.0) as u16
}

/// Clear gap kept between two buildings of one compound (metres).
const CELL_GAP_M: f32 = 4.0;
/// Two buildings of one compound closer than this (metres) are neighbours, and
/// neighbours never share a storey count.
const NEIGHBOUR_M: f32 = 18.0;

/// The buildable rectangle of a lot, in the lot's own frame.
#[derive(Clone, Copy)]
struct Site {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum CellKind {
    /// A long, narrow slab of six or seven storeys: the planned rows at the edge.
    Slab,
    /// A longer slab, 11–18 storeys in the core and 11–12 at the edge.
    Tall,
    /// A roughly square point tower, 18–33 storeys.
    Tower,
    /// A low civic building: a clinic or a kindergarten.
    Low,
}

struct CompoundCell {
    ring: Vec<V>,
    floors: u16,
    /// The hash that set `floors`: a neighbour reads it to choose its own step.
    draw: f32,
}

/// The buildings of one compound, as footprints in the lot's own frame with
/// their storeys.
///
/// The site is cut into rows, and each row is walked from one end to the other.
/// Every slot along a row is a cell that draws its own use from a hash: a garden
/// gap, a slab, a taller slab, a point tower or a low civic building.  Each
/// building has its own length, depth, position across its row, tilt (in the
/// core only) and gap to the next, so no two compounds read as one grid.  A
/// building stays inside the lot `lot_uv` (convex) and clears the others by
/// `CELL_GAP_M`.  A building that neighbours one with the same storey count
/// steps off it (see `step_off`).  `centrality` runs from 0 at the planned edge
/// to 1 at the core.
fn chinese_compound_cells(
    site: Site,
    lot_uv: &[V],
    key: i32,
    seed: u32,
    centrality: f32,
    density: f32,
    fits: &dyn Fn(&[V]) -> bool,
) -> Vec<(Vec<V>, u16)> {
    let g = smoothstep(0.15, 0.55, centrality);
    let target_band = if modern_hash(seed, key, 5, 753) < 0.5 {
        28.0
    } else {
        30.0
    };
    let rows = ((site.z1 - site.z0) / target_band).floor().max(1.0) as usize;
    let band = (site.z1 - site.z0) / rows as f32;
    let garden_p = 0.05 + 0.12 * (1.0 - centrality);
    let tower_p = 0.02 + 0.40 * g;
    let tall_p = 0.26 + 0.14 * g;
    let low_p = 0.04;
    // Core slabs are turned by up to 8 degrees, towers by half that; the planned
    // edge is set out square to its streets.
    let turn_max = 8.0_f32.to_radians() * g;

    let mut cells: Vec<CompoundCell> = Vec::new();
    for row in 0..rows {
        let row_c = site.z0 + band * (row as f32 + 0.5);
        let mut u = site.x0 + 6.0 * modern_hash(seed, key, 16 + row as i32, 801);
        let mut slot = 0_i32;
        while u < site.x1 - 14.0 {
            let ck = key.wrapping_mul(257).wrapping_add(slot * 16 + row as i32);
            let cn = |salt: i32| modern_hash(seed, ck, 3, salt);
            slot += 1;
            let gap = 6.0 + 7.0 * cn(837);
            if cn(781) < garden_p {
                u += 16.0 + 14.0 * cn(813);
                continue;
            }
            let r = cn(791);
            let mut kind = if r < tower_p {
                CellKind::Tower
            } else if r < tower_p + tall_p {
                CellKind::Tall
            } else if r < tower_p + tall_p + low_p {
                CellKind::Low
            } else {
                CellKind::Slab
            };
            let drawn = match kind {
                CellKind::Slab => 40.0 + 20.0 * cn(821),
                CellKind::Tall => 44.0 + 22.0 * cn(823),
                CellKind::Tower => 22.0 + 3.0 * cn(825),
                CellKind::Low => 14.0 + 4.0 * cn(827),
            };
            let tower_ratio = 0.94 + 0.06 * cn(833);
            let dep_max = match kind {
                CellKind::Slab => 12.0 + 2.0 * cn(829),
                CellKind::Tall => 12.5 + 1.5 * cn(831),
                CellKind::Tower => drawn * tower_ratio,
                CellKind::Low => 12.0 + 2.0 * cn(835),
            };
            let cv = row_c + ((band - dep_max) * 0.4).max(0.0) * (2.0 * cn(839) - 1.0);
            // Where a rectangle this deep, with its clearance, stays inside the lot.
            let Some((lo, hi)) = strip_span(
                lot_uv,
                cv - dep_max * 0.5 - LOT_MARGIN_M,
                cv + dep_max * 0.5 + LOT_MARGIN_M,
            ) else {
                u += gap;
                continue;
            };
            let start = u.max(lo + LOT_MARGIN_M).max(site.x0);
            let avail = (hi - LOT_MARGIN_M).min(site.x1) - start;
            // A slab cut short by a lot's corner keeps at least 30 m.
            let min_len = match kind {
                CellKind::Slab | CellKind::Tall => 30.0,
                CellKind::Tower => 24.0,
                CellKind::Low => 14.0,
            };
            let mut len = drawn.min(avail);
            if len < min_len {
                // A hole too short for this building: a low civic building fills
                // it, or the slot stays open when even that will not fit.
                if avail < 14.0 {
                    u = start + gap;
                    continue;
                }
                kind = CellKind::Low;
                len = avail.min(18.0);
            }
            let dep = match kind {
                CellKind::Tower => len * tower_ratio,
                CellKind::Low => dep_max.min(14.0),
                CellKind::Slab | CellKind::Tall => dep_max,
            };
            let turn = match kind {
                CellKind::Slab | CellKind::Tall => turn_max * (2.0 * cn(841) - 1.0),
                CellKind::Tower => 0.5 * turn_max * (2.0 * cn(841) - 1.0),
                CellKind::Low => 0.0,
            };
            // The building as drawn, then set out square when its corners clip
            // the lot's edge or a dead-end street.
            let centre_u = start + len * 0.5;
            let placed = [turn, 0.0].into_iter().find_map(|t| {
                let ring = rect_at(centre_u, cv, len, dep, t);
                let clear = fits(&ring)
                    && cells
                        .iter()
                        .all(|c| convex_separation(&ring, &c.ring) >= CELL_GAP_M);
                clear.then_some(ring)
            });
            if let Some(ring) = placed {
                let draw = cn(843);
                let base = match kind {
                    CellKind::Slab => {
                        if draw < 0.4 {
                            6
                        } else {
                            7
                        }
                    }
                    CellKind::Tall => 11 + (draw * (2.0 + 6.0 * g)) as u16,
                    CellKind::Tower => {
                        (18.0 + draw * 16.0 * (0.6 + 0.4 * density)).min(33.0) as u16
                    }
                    CellKind::Low => 3 + (draw * 2.0) as u16,
                };
                let neighbours: Vec<(u16, f32)> = cells
                    .iter()
                    .filter(|c| convex_separation(&ring, &c.ring) < NEIGHBOUR_M)
                    .map(|c| (c.floors, c.draw))
                    .collect();
                let floors = step_off(base, &neighbours);
                cells.push(CompoundCell { ring, floors, draw });
            }
            u = start + len + gap;
        }
    }
    cells.into_iter().map(|c| (c.ring, c.floors)).collect()
}

/// The storey count nearest `base` that no neighbour already has.  The side it
/// steps to is set by the first neighbour that shares `base`: a neighbour whose
/// own draw is in the lower half sends this building up, the others send it down.
fn step_off(base: u16, neighbours: &[(u16, f32)]) -> u16 {
    let Some(&(_, draw)) = neighbours.iter().find(|(floors, _)| *floors == base) else {
        return base;
    };
    let up: i32 = if draw < 0.5 { 1 } else { -1 };
    for step in 1..=12_i32 {
        for dir in [up, -up] {
            let floors = i32::from(base) + dir * step;
            if (3..=33).contains(&floors) && neighbours.iter().all(|(n, _)| i32::from(*n) != floors)
            {
                return floors as u16;
            }
        }
    }
    base
}

/// Lower bound on the clear distance between two convex rings, from the widest
/// gap between their projections on any edge normal.  Negative when the rings
/// overlap along every normal, which means they intersect.
fn convex_separation(a: &[V], b: &[V]) -> f32 {
    let mut best = f32::MIN;
    for ring in [a, b] {
        for i in 0..ring.len() {
            let (p, q) = (ring[i], ring[(i + 1) % ring.len()]);
            let (dx, dz) = (q.0 - p.0, q.1 - p.1);
            let len = dx.hypot(dz);
            if len < 1.0e-6 {
                continue;
            }
            let n = (-dz / len, dx / len);
            let span = |r: &[V]| -> (f32, f32) {
                r.iter().fold((f32::MAX, f32::MIN), |(lo, hi), v| {
                    let t = v.0 * n.0 + v.1 * n.1;
                    (lo.min(t), hi.max(t))
                })
            };
            let (a0, a1) = span(a);
            let (b0, b1) = span(b);
            best = best.max((b0 - a1).max(a0 - b1));
        }
    }
    best
}

/// The u-range along which a strip from `v0` to `v1` lies inside a convex ring
/// (`None` when the strip misses it).  A convex section is widest at its ends,
/// so only the two edges of the strip need checking.
fn strip_span(ring: &[V], v0: f32, v1: f32) -> Option<(f32, f32)> {
    let section = |v: f32| -> Option<(f32, f32)> {
        let (mut lo, mut hi) = (f32::MAX, f32::MIN);
        for i in 0..ring.len() {
            let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
            if (b.1 - a.1).abs() > 1.0e-6 && (a.1 - v) * (b.1 - v) <= 0.0 {
                let t = (v - a.1) / (b.1 - a.1);
                let u = a.0 + t * (b.0 - a.0);
                lo = lo.min(u);
                hi = hi.max(u);
            }
        }
        (lo <= hi).then_some((lo, hi))
    };
    let (a0, a1) = section(v0)?;
    let (b0, b1) = section(v1)?;
    let (lo, hi) = (a0.max(b0), a1.min(b1));
    (hi > lo).then_some((lo, hi))
}

/// Rectangle `w` by `d` centred on (`cx`, `cz`) and turned by `turn` radians.
/// Counter-clockwise, like `rect`.
fn rect_at(cx: f32, cz: f32, w: f32, d: f32, turn: f32) -> Vec<V> {
    let (s, c) = turn.sin_cos();
    [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]
        .iter()
        .map(|&(x, z)| {
            let (x, z) = (x * w, z * d);
            (cx + x * c - z * s, cz + x * s + z * c)
        })
        .collect()
}

/// Axis-aligned rectangle in a lot's own frame (CCW), optionally shrunk.
fn rect(x0: f32, x1: f32, z0: f32, z1: f32, shrink: f32) -> Vec<V> {
    let dx = (x1 - x0) * shrink * 0.5;
    let dz = (z1 - z0) * shrink * 0.5;
    vec![
        (x0 + dx, z0 + dz),
        (x1 - dx, z0 + dz),
        (x1 - dx, z1 - dz),
        (x0 + dx, z1 - dz),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::{CityFrame, generate_modern_chinese_city};
    use super::{NEIGHBOUR_M, chinese_residential_storeys, convex_separation, rect_at};
    use crate::{BuildingFacade, CityStyle, ModernChinaSpec, ModernCity, ParcelUse, Point};
    use std::collections::HashMap;

    /// Centrality, storeys and facade of every building on a residential parcel,
    /// with centrality read at the building's own footprint centre.
    fn residential_buildings(spec: ModernChinaSpec) -> Vec<(f32, u16, BuildingFacade)> {
        let frame = CityFrame::new(spec);
        let city = generate_modern_chinese_city(spec);
        let parcels: HashMap<u32, &crate::Parcel> =
            city.parcels.iter().map(|p| (p.id, p)).collect();
        city.buildings
            .iter()
            .filter_map(|b| {
                let parcel = parcels.get(&b.parcel_id)?;
                if parcel.use_type != ParcelUse::Residential {
                    return None;
                }
                let n = b.footprint.len() as f32;
                let (sx, sy) = b.footprint.iter().fold((0.0_f32, 0.0_f32), |acc, p| {
                    (acc.0 + p.x_km, acc.1 + p.y_km)
                });
                let (x, z) = frame.to_local(Point {
                    x_km: sx / n,
                    y_km: sy / n,
                });
                Some((frame.core_weight(x, z), b.floors, b.facade))
            })
            .collect()
    }

    fn median(mut values: Vec<u16>) -> f32 {
        values.sort_unstable();
        match values.len() {
            0 => 0.0,
            n if n % 2 == 1 => f32::from(values[n / 2]),
            n => (f32::from(values[n / 2 - 1]) + f32::from(values[n / 2])) * 0.5,
        }
    }

    #[test]
    fn chinese_residential_height_follows_the_town_gradient() {
        for seed in [42_u32, 7] {
            let rows = residential_buildings(ModernChinaSpec {
                seed,
                ..ModernChinaSpec::default()
            });
            assert!(
                rows.len() > 200,
                "seed {seed}: {} residential buildings",
                rows.len()
            );
            assert!(
                rows.iter()
                    .all(|r| r.2 == BuildingFacade::PlasterResidential),
                "seed {seed}: a Chinese residential parcel is not plaster"
            );

            let core: Vec<u16> = rows.iter().filter(|r| r.0 >= 0.4).map(|r| r.1).collect();
            let core_n = core.len();
            let core_median = median(core.clone());
            let core_18 = core.iter().filter(|f| **f >= 18).count() as f32 / core_n as f32;
            assert!(core_n >= 40, "seed {seed}: only {core_n} core buildings");
            assert!(
                (18.0..=24.0).contains(&core_median),
                "seed {seed}: core median {core_median} storeys"
            );
            assert!(
                core_18 >= 0.30,
                "seed {seed}: core share at 18+ is {core_18}"
            );

            let outer: Vec<u16> = rows.iter().filter(|r| r.0 < 0.2).map(|r| r.1).collect();
            let outer_n = outer.len();
            let outer_median = median(outer.clone());
            let outer_mid =
                outer.iter().filter(|f| (7..=12).contains(*f)).count() as f32 / outer_n as f32;
            assert!(
                (6.0..=12.0).contains(&outer_median),
                "seed {seed}: outer median {outer_median} storeys"
            );
            assert!(
                outer_mid >= 0.6,
                "seed {seed}: outer share at 7-12 is {outer_mid}"
            );
        }
    }

    #[test]
    fn western_mixed_residential_keeps_brick_and_its_old_heights() {
        let rows = residential_buildings(ModernChinaSpec {
            style: CityStyle::WesternMixed,
            ..ModernChinaSpec::default()
        });
        assert!(!rows.is_empty());
        assert!(rows.iter().all(|r| r.2 == BuildingFacade::BrickResidential));
        assert!(rows.iter().all(|r| (4..=34).contains(&r.1)));
    }

    #[test]
    fn chinese_residential_storeys_is_bounded_and_rises_with_centrality() {
        for density in [0.0_f32, 0.5, 1.0] {
            for u in [0.0_f32, 0.25, 0.5, 0.75, 0.999] {
                let mut last = 0_u16;
                for step in 0..=50 {
                    let floors = chinese_residential_storeys(step as f32 / 50.0, density, u);
                    assert!((6..=28).contains(&floors));
                    assert!(floors >= last, "storeys fell as centrality rose");
                    last = floors;
                }
            }
        }
        // A sparser city is lower at its centre, and its edge is unchanged.
        assert!(
            chinese_residential_storeys(1.0, 0.4, 0.5) < chinese_residential_storeys(1.0, 0.9, 0.5)
        );
        assert_eq!(
            chinese_residential_storeys(0.0, 0.4, 0.5),
            chinese_residential_storeys(0.0, 0.9, 0.5)
        );
    }

    /// A footprint in metres.
    type Metres = Vec<(f32, f32)>;

    /// The residential buildings on each compound parcel of a city, with their
    /// storeys and footprints in metres.
    fn compound_buildings(city: &ModernCity) -> HashMap<u32, Vec<(u16, Metres)>> {
        let parcels: HashMap<u32, &crate::Parcel> =
            city.parcels.iter().map(|p| (p.id, p)).collect();
        let mut out: HashMap<u32, Vec<(u16, Metres)>> = HashMap::new();
        for b in &city.buildings {
            let Some(parcel) = parcels.get(&b.parcel_id) else {
                continue;
            };
            if parcel.use_type != ParcelUse::Residential || !parcel.compound {
                continue;
            }
            let metres: Vec<(f32, f32)> = b
                .footprint
                .iter()
                .map(|p| (p.x_km * 1000.0, p.y_km * 1000.0))
                .collect();
            out.entry(b.parcel_id).or_default().push((b.floors, metres));
        }
        out
    }

    #[test]
    fn chinese_compound_storeys_vary_within_one_compound() {
        for seed in [42_u32, 7] {
            let city = generate_modern_chinese_city(ModernChinaSpec {
                seed,
                ..ModernChinaSpec::default()
            });
            let compounds = compound_buildings(&city);
            let distinct = |b: &Vec<(u16, Metres)>| {
                let mut floors: Vec<u16> = b.iter().map(|x| x.0).collect();
                floors.sort_unstable();
                floors.dedup();
                floors.len()
            };
            let best = compounds.values().map(distinct).max().unwrap_or(0);
            assert!(
                best >= 4,
                "seed {seed}: best compound has {best} storey counts"
            );
            // Within one compound the mix runs from six-storey rows to taller slabs.
            let mixed = compounds
                .values()
                .any(|b| b.iter().any(|x| x.0 <= 7) && b.iter().any(|x| x.0 >= 11));
            assert!(
                mixed,
                "seed {seed}: no compound mixes low rows and taller blocks"
            );
        }
    }

    #[test]
    fn chinese_residential_buildings_do_not_overlap() {
        for seed in [42_u32, 7] {
            let city = generate_modern_chinese_city(ModernChinaSpec {
                seed,
                ..ModernChinaSpec::default()
            });
            let parcels: HashMap<u32, &crate::Parcel> =
                city.parcels.iter().map(|p| (p.id, p)).collect();
            let homes: Vec<(u32, Metres)> = city
                .buildings
                .iter()
                .filter(|b| {
                    parcels
                        .get(&b.parcel_id)
                        .is_some_and(|p| p.use_type == ParcelUse::Residential)
                })
                .map(|b| {
                    let metres = b
                        .footprint
                        .iter()
                        .map(|p| (p.x_km * 1000.0, p.y_km * 1000.0))
                        .collect();
                    (b.parcel_id, metres)
                })
                .collect();
            assert!(homes.len() > 200, "seed {seed}: {} homes", homes.len());
            for (i, (pa, a)) in homes.iter().enumerate() {
                for (pb, b) in &homes[i + 1..] {
                    let gap = convex_separation(a, b);
                    assert!(
                        gap > 0.0,
                        "seed {seed}: buildings on parcels {pa} and {pb} overlap ({gap} m)"
                    );
                }
            }
        }
    }

    #[test]
    fn chinese_compound_neighbours_never_share_a_storey_count() {
        for seed in [42_u32, 7] {
            let city = generate_modern_chinese_city(ModernChinaSpec {
                seed,
                ..ModernChinaSpec::default()
            });
            for buildings in compound_buildings(&city).values() {
                for (i, (fa, a)) in buildings.iter().enumerate() {
                    for (fb, b) in &buildings[i + 1..] {
                        if convex_separation(a, b) < NEIGHBOUR_M {
                            assert_ne!(
                                fa, fb,
                                "seed {seed}: neighbouring compound buildings both have {fa} storeys"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn convex_separation_reads_gaps_and_overlaps() {
        let square = |x: f32, z: f32| rect_at(x, z, 10.0, 10.0, 0.0);
        // Ten metres apart, edge to edge.
        assert!((convex_separation(&square(0.0, 0.0), &square(20.0, 0.0)) - 10.0).abs() < 1e-4);
        // Overlapping, and one inside the other.
        assert!(convex_separation(&square(0.0, 0.0), &square(4.0, 3.0)) < 0.0);
        assert!(convex_separation(&square(0.0, 0.0), &square(0.0, 0.0)) < 0.0);
        // A turned square still keeps its clear distance.
        let turned = rect_at(20.0, 0.0, 10.0, 10.0, std::f32::consts::FRAC_PI_4);
        assert!(convex_separation(&square(0.0, 0.0), &turned) > 4.0);
    }
}
