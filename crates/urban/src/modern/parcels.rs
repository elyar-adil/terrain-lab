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

        // A Chinese city is zoned by the block: a gated compound takes a whole
        // block, a school or hospital a whole campus, and only the commercial
        // blocks of the core and the arterials are cut plot by plot.
        let zone = if chinese && !park_block {
            block_zone(
                face,
                face_area,
                centrality,
                modern_hash(seed, fi, 6, 707),
                modern_hash(seed, fi, 6, 708),
            )
        } else {
            Zone::Lots
        };
        // A few very large faces stay one lot: a walled compound or campus.
        let superblock = !chinese
            && face_area > 13_000.0
            && centrality < 0.55
            && modern_hash(seed, fi, 4, 777) < 0.45;
        // A park is one park: cut into lots, it read as a grid of lawns.
        let target = if superblock
            || matches!(zone, Zone::Compound | Zone::Campus)
            || (chinese && park_block)
        {
            1.0e9
        } else {
            900.0 + 1500.0 * (1.0 - centrality) * (0.6 + 0.8 * modern_hash(seed, fi, 3, 761))
        };
        if matches!(zone, Zone::Village | Zone::Industrial) {
            for (env_index, env) in envelopes.iter().enumerate() {
                let sink = PlanSink {
                    seed,
                    key: fi * 8 + env_index as i32,
                    block_id: block_base + (env_index as u32).min(block_rings.len() as u32 - 1),
                    to_world: &to_world,
                    parcels: &mut parcels,
                    buildings: &mut buildings,
                    next_parcel: &mut next_parcel,
                    next_building: &mut next_building,
                };
                if zone == Zone::Village {
                    village_block(env, &spurs, sink);
                } else {
                    industrial_block(env, &spurs, sink);
                }
            }
            continue;
        }
        for (env_index, env) in envelopes.iter().enumerate() {
            let mut lots: Vec<Vec<V>> = Vec::new();
            subdivide(env, target, seed, fi * 8 + env_index as i32, 0, &mut lots);
            let block_id = block_base + (env_index as u32).min(block_rings.len() as u32 - 1);
            for (lot_index, lot) in lots.iter().enumerate() {
                // A compound on a collector or an arterial is fronted by a run of
                // two- and three-storey shops (底商), its fence behind them: the
                // strip is cut off the lot as parcels of its own.
                let carved;
                let lot = if zone == Zone::Compound
                    && signed_area(lot) > 6_000.0
                    && modern_hash(seed, fi * 8 + env_index as i32, 7, 709)
                        < 0.45 + 0.4 * centrality
                {
                    carved = street_shops(
                        lot,
                        face,
                        &spurs,
                        PlanSink {
                            seed,
                            key: fi * 64 + (env_index as i32) * 16 + lot_index as i32,
                            block_id: block_base
                                + (env_index as u32).min(block_rings.len() as u32 - 1),
                            to_world: &to_world,
                            parcels: &mut parcels,
                            buildings: &mut buildings,
                            next_parcel: &mut next_parcel,
                            next_building: &mut next_building,
                        },
                    );
                    &carved
                } else {
                    lot
                };
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
                let use_type =
                    if park_block || (plaza && zone == Zone::Lots) || (waterfront && n > 0.82) {
                        ParcelUse::Park
                    } else if zone == Zone::Compound || zone == Zone::Homes {
                        ParcelUse::Residential
                    } else if zone == Zone::Campus {
                        ParcelUse::Civic
                    } else if chinese {
                        // The commercial blocks: offices and malls, shops with flats
                        // over them, the odd civic plot and a few street-wall homes.
                        if n < 0.55 {
                            ParcelUse::Commercial
                        } else if n < 0.82 {
                            ParcelUse::MixedUse
                        } else if n < 0.88 {
                            ParcelUse::Civic
                        } else {
                            ParcelUse::Residential
                        }
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
                    && if zone == Zone::Compound {
                        ob.width() > 45.0 && ob.depth() > 35.0
                    } else {
                        !chinese
                            && ob.width() > 55.0
                            && ob.depth() > 40.0
                            && (superblock || (centrality < 0.5 && noise(779) < 0.6))
                    };
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
                        ParcelUse::Industrial => procedural::FacadeKind::ConcreteGlass,
                        ParcelUse::Residential
                        | ParcelUse::Villa
                        | ParcelUse::Farmstead
                        | ParcelUse::Village
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
                    // A residential slab is thin for its height: a 33-storey slab
                    // 16 m deep is ordinary, about 6 : 1.
                    let slenderness = if use_type == ParcelUse::Residential && chinese {
                        6.2
                    } else {
                        4.4
                    };
                    let height_cap = slenderness * short_side - podium_height - 1.2;
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
                        // A walled compound in south-facing rows: see `chinese_compound_rows`.
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
                        // North in the lot frame, snapped to the lot axis nearest
                        // it: compounds square their rows to their streets.
                        let true_north = (ob.u.1, ob.v.1);
                        let north = [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)]
                            .into_iter()
                            .max_by(|x: &V, y: &V| {
                                (x.0 * true_north.0 + x.1 * true_north.1)
                                    .total_cmp(&(y.0 * true_north.0 + y.1 * true_north.1))
                            })
                            .unwrap_or((0.0, 1.0));
                        for (ring, floors) in chinese_compound_rows(
                            site, &lot_uv, key, seed, centrality, density, north, &fits,
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
                    ParcelUse::Park
                    | ParcelUse::Villa
                    | ParcelUse::Farmstead
                    | ParcelUse::Village
                    | ParcelUse::Industrial => {}
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

/// What a block of a Chinese city is given over to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Zone {
    /// Cut plot by plot: the commercial blocks, and every block of a western city.
    Lots,
    /// One gated residential compound (小区) over the whole block.
    Compound,
    /// A block too small for a compound: street-wall homes, plot by plot.
    Homes,
    /// One campus over the whole block: a school, a hospital, an institute.
    Campus,
    /// 城中村: the whole block a village of self-built houses.
    Village,
    /// Factory and warehouse yards.
    Industrial,
}

/// The zone of a block, from how central it is, whether an arterial passes it
/// and a draw `r` in 0..1.
///
/// Residential is most of a Chinese city's built land (GB 50137 puts it at
/// 25 - 40 % of all urban land, against 5 - 15 % commercial and 5 - 8 % public
/// service), and it comes as compounds, not plots.  Offices and malls gather in
/// the core and along the arterials.
fn block_zone(face: &super::blocks::Face, area: f32, centrality: f32, r: f32, r_land: f32) -> Zone {
    // Industry is 15 - 30 % of a Chinese city's land (GB 50137), almost all of it
    // in parks on the edge; the villages the city swallowed are in the band
    // between the old town and the new districts.
    let industrial = 0.32 * (1.0 - smoothstep(0.08, 0.32, centrality));
    let village =
        0.12 * smoothstep(0.10, 0.25, centrality) * (1.0 - smoothstep(0.45, 0.65, centrality));
    if r_land < industrial && area > 8_000.0 {
        return Zone::Industrial;
    }
    if r_land >= industrial && r_land < industrial + village && area > 5_000.0 {
        return Zone::Village;
    }
    let arterial = (0..face.ring.len()).any(|i| {
        !face.open[i]
            && matches!(
                face.classes[i],
                ModernRoadClass::Arterial | ModernRoadClass::Expressway
            )
    });
    let commercial =
        0.06 + 0.52 * smoothstep(0.35, 0.85, centrality) + if arterial { 0.08 } else { 0.0 };
    if r < commercial {
        Zone::Lots
    } else if r < commercial + 0.07 && area > 6_000.0 {
        Zone::Campus
    } else if area >= 4_000.0 {
        Zone::Compound
    } else {
        Zone::Homes
    }
}

/// Depth of a frontage strip, street to fence: a shop 12 m deep and the lane
/// behind it.
const SHOP_BAND_M: f32 = 14.0;

/// Where the parcels and buildings a block generator makes go.
struct PlanSink<'a> {
    seed: u32,
    key: i32,
    block_id: u32,
    to_world: &'a dyn Fn(&[V]) -> Vec<Point>,
    parcels: &'a mut Vec<Parcel>,
    buildings: &'a mut Vec<ModernBuilding>,
    next_parcel: &'a mut u32,
    next_building: &'a mut u32,
}

/// Cut a frontage strip off every edge of `lot` that faces a collector or an
/// arterial, fill each strip with shops, and return what is left of the lot.
///
/// The shops stand at the building line in runs of 24 - 48 m with a passage
/// between runs, two or three storeys: the 底商 that lines nearly every
/// compound on a main street.  A strip a dead-end street crosses is left out.
fn street_shops(
    lot: &[V],
    face: &super::blocks::Face,
    spurs: &[(V, V, ModernRoadClass)],
    sink: PlanSink,
) -> Vec<V> {
    let seed = sink.seed;
    let mut rest = lot.to_vec();
    for j in 0..lot.len() {
        let (a, b) = (lot[j], lot[(j + 1) % lot.len()]);
        let len = (b.0 - a.0).hypot(b.1 - a.1);
        if len < 30.0 {
            continue;
        }
        let mid = ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);
        // The street this edge looks onto: the face edge nearest its midpoint.
        let Some(i) = (0..face.ring.len()).min_by(|x, y| {
            let d =
                |i: usize| point_seg_dist(mid, face.ring[i], face.ring[(i + 1) % face.ring.len()]);
            d(*x).total_cmp(&d(*y))
        }) else {
            continue;
        };
        let street = point_seg_dist(mid, face.ring[i], face.ring[(i + 1) % face.ring.len()]);
        if face.open[i]
            || !matches!(
                face.classes[i],
                ModernRoadClass::Collector | ModernRoadClass::Arterial
            )
            || street > right_of_way(face.classes[i]) + 3.0
        {
            continue;
        }
        let dir = ((b.0 - a.0) / len, (b.1 - a.1) / len);
        // Inward: the lot is counter-clockwise, so its inside is on the left.
        let n = (-dir.1, dir.0);
        let c = n.0 * a.0 + n.1 * a.1;
        let band = clip_half(&rest, (-n.0, -n.1), -(c + SHOP_BAND_M));
        let kept = clip_half(&rest, n, c + SHOP_BAND_M);
        if band.len() < 4 || kept.len() < 4 || signed_area(&kept) < 2_500.0 {
            continue;
        }
        let crossed = spurs.iter().any(|(p, q, _)| {
            band.iter()
                .zip(band.iter().cycle().skip(1))
                .any(|(x, y)| seg_seg_dist(*x, *y, *p, *q) < 2.0)
        });
        if crossed {
            continue;
        }
        rest = kept;
        let parcel_id = *sink.next_parcel;
        *sink.next_parcel += 1;
        sink.parcels.push(Parcel {
            id: parcel_id,
            block_id: sink.block_id,
            ring: (sink.to_world)(&band),
            use_type: ParcelUse::Commercial,
            compound: false,
            gate_edge: 0,
        });
        // Runs along the edge, kept 5 m off each end so corners stay open.
        let mut t = 5.0;
        let mut run = 0_i32;
        while t < len - 5.0 - 16.0 {
            let h = |salt: i32| modern_hash(seed, sink.key * 31 + j as i32 * 7 + run, 8, salt);
            let length = (24.0 + 24.0 * h(711)).min(len - 5.0 - t);
            if length < 16.0 {
                break;
            }
            let depth = 11.0 + 1.5 * h(713);
            let floors: u16 = if h(715) < 0.65 { 2 } else { 3 };
            let p0 = (a.0 + dir.0 * t + n.0 * 0.5, a.1 + dir.1 * t + n.1 * 0.5);
            let p1 = (p0.0 + dir.0 * length, p0.1 + dir.1 * length);
            let ring = vec![
                p0,
                p1,
                (p1.0 + n.0 * depth, p1.1 + n.1 * depth),
                (p0.0 + n.0 * depth, p0.1 + n.1 * depth),
            ];
            if ring.iter().all(|p| point_in(lot, *p)) {
                let metrics = procedural::facade_metrics(
                    length,
                    depth,
                    floors,
                    procedural::FacadeKind::ConcreteGlass,
                );
                sink.buildings.push(ModernBuilding {
                    id: *sink.next_building,
                    parcel_id,
                    footprint: (sink.to_world)(&ring),
                    // A 4.5 m shop storey and the rest at the office storey.
                    height_metres: PODIUM_FLOOR_M
                        + f32::from(floors - 1) * metrics.storey_height_m
                        + 1.2,
                    floors,
                    use_type: ParcelUse::Commercial,
                    roof: RoofStyle::Flat,
                    facade: BuildingFacade::ConcreteGlass,
                    podium_height_metres: 0.0,
                    window_bays: metrics.bays,
                    balcony_bays: 0,
                    entrance_count: ((metrics.bays / 3).max(1) as u8).clamp(1, 8),
                });
                *sink.next_building += 1;
            }
            t += length + 4.0 + 4.0 * h(717);
            run += 1;
        }
    }
    rest
}

/// A building the block generators place directly, without the lot fitting.
fn push_building(
    sink: &mut PlanSink,
    parcel_id: u32,
    ring: &[V],
    floors: u16,
    height_metres: f32,
    use_type: ParcelUse,
    facade: BuildingFacade,
) {
    let side = |i: usize| {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        (b.0 - a.0).hypot(b.1 - a.1)
    };
    let (w, d) = (side(0), side(1));
    let kind = match use_type {
        ParcelUse::Village | ParcelUse::Residential => procedural::FacadeKind::Residential,
        _ => procedural::FacadeKind::ConcreteGlass,
    };
    let metrics = procedural::facade_metrics(w.max(d), w.min(d), floors, kind);
    sink.buildings.push(ModernBuilding {
        id: *sink.next_building,
        parcel_id,
        footprint: (sink.to_world)(ring),
        height_metres,
        floors,
        use_type,
        roof: RoofStyle::Flat,
        facade,
        podium_height_metres: 0.0,
        window_bays: metrics.bays,
        balcony_bays: 0,
        entrance_count: 1,
    });
    *sink.next_building += 1;
}

/// A new parcel over `ring`, returning its id.
fn push_parcel(sink: &mut PlanSink, ring: &[V], use_type: ParcelUse) -> u32 {
    let id = *sink.next_parcel;
    *sink.next_parcel += 1;
    sink.parcels.push(Parcel {
        id,
        block_id: sink.block_id,
        ring: (sink.to_world)(ring),
        use_type,
        compound: false,
        gate_edge: 0,
    });
    id
}

/// Whether a dead-end street runs within `clear` metres of `ring`.
fn spur_hits(ring: &[V], spurs: &[(V, V, ModernRoadClass)], clear: f32) -> bool {
    spurs.iter().any(|(p, q, _)| {
        ring.iter()
            .zip(ring.iter().cycle().skip(1))
            .any(|(x, y)| seg_seg_dist(*x, *y, *p, *q) < clear)
    })
}

/// A block of 城中村: the village's old house plots, each rebuilt by its family
/// as tall as it could get away with.
///
/// Read off photographs of Shenzhen and Guangzhou villages and the published
/// figures on them (`docs/city-visual-target.md`, 城中村):
///
/// - a house is 6 - 10 m across its lane front (two to four windows a floor)
///   and 10 - 14 m deep, a footprint of 80 - 120 m²; about one plot in twelve is
///   two or three merged and built higher;
/// - houses stand back to back in pairs, a 2.5 m lane between pairs and a 4 m
///   lane every few plots across them (Shuiwei: 2.5 m east-west, 4 m
///   north-south), with a slit of 0.6 - 1.5 m between neighbours on a lane;
/// - the rows do not line up: each starts at its own offset;
/// - neighbours are of unrelated heights: mostly six to eight storeys, the odd
///   two- or three-storey house of the 1980s left standing, merged plots to
///   fourteen;
/// - the houses on the village's edge and on its wide lanes have shops on the
///   ground floor (marked by a second entrance, the shop's).
fn village_block(env: &[V], spurs: &[(V, V, ModernRoadClass)], mut sink: PlanSink) {
    let (seed, key) = (sink.seed, sink.key);
    let ob = obb(env);
    let parcel_id = push_parcel(&mut sink, env, ParcelUse::Village);
    let h = |salt: i32| modern_hash(seed, key, 11, salt);
    let plot_depth = 10.0 + 4.0 * h(803);
    let back_gap = 0.6 + 0.6 * h(805);
    let pair_lane = 2.5;
    let cross_lane = 4.0;
    let mut v = ob.min_v + 1.0;
    let mut pair = 0_i32;
    let pairs_total =
        ((ob.max_v - ob.min_v) / (2.0 * plot_depth + back_gap + pair_lane)).floor() as i32;
    while v + plot_depth * 2.0 + back_gap < ob.max_v - 1.0 {
        for half in 0..2_i32 {
            let v0 = v + f32::from(half as u8) * (plot_depth + back_gap);
            let edge_row = (pair == 0 && half == 0) || (pair == pairs_total - 1 && half == 1);
            // Each row starts at its own offset, so plots do not line up.
            let row_h = |salt: i32| modern_hash(seed, key * 977 + pair * 131 + half * 61, 16, salt);
            let mut u = ob.min_u + 1.0 + 6.0 * row_h(841);
            let cross_every = 4 + (row_h(843) * 4.0) as i32;
            let mut plot = 0_i32;
            while u + 6.0 < ob.max_u - 1.0 {
                let cell = key * 977 + pair * 131 + half * 61 + plot * 7;
                let ph = |salt: i32| modern_hash(seed, cell, 12, salt);
                let merged = ph(823) < 0.08;
                let w = if merged {
                    15.0 + 10.0 * ph(811)
                } else {
                    6.0 + 4.0 * ph(811)
                };
                let w = w.min(ob.max_u - 1.0 - u);
                let d = plot_depth * (0.88 + 0.12 * ph(813));
                // Each family built out to its own line, a little behind its
                // neighbours' on the lane.
                let back = 0.6 * ph(821);
                let (d0, d1) = if half == 0 {
                    (v0 + plot_depth - d, v0 + plot_depth - back)
                } else {
                    (v0 + back, v0 + d)
                };
                let ring: Vec<V> = [(u, d0), (u + w, d0), (u + w, d1), (u, d1)]
                    .iter()
                    .map(|p| ob.to_world(p.0, p.1))
                    .collect();
                let empty = ph(815) < 0.04;
                if w >= 5.5
                    && !empty
                    && ring.iter().all(|p| point_in(env, *p))
                    && !spur_hits(&ring, spurs, 3.0)
                {
                    let r = ph(817);
                    let floors: u16 = if merged {
                        9 + (r * 6.0) as u16
                    } else if r < 0.08 {
                        2 + u16::from(r < 0.04)
                    } else if r < 0.25 {
                        4 + u16::from(r < 0.16)
                    } else if r < 0.75 {
                        6 + ((r - 0.25) / 0.5 * 3.0) as u16
                    } else if r < 0.92 {
                        9 + u16::from(r < 0.84)
                    } else {
                        11 + ((r - 0.92) / 0.08 * 2.0) as u16
                    };
                    // About 3 m a storey and a parapet.
                    let height = f32::from(floors) * 3.0 + 0.4 + 0.6 * ph(819);
                    let shop = edge_row || ph(825) < 0.3;
                    push_building(
                        &mut sink,
                        parcel_id,
                        &ring,
                        floors,
                        height,
                        ParcelUse::Village,
                        BuildingFacade::PlasterResidential,
                    );
                    if shop {
                        if let Some(b) = sink.buildings.last_mut() {
                            b.entrance_count = 2;
                        }
                    }
                }
                plot += 1;
                let slit = 0.6 + 0.9 * ph(827);
                u += w + if plot % cross_every == 0 {
                    cross_lane
                } else {
                    slit
                };
            }
        }
        v += plot_depth * 2.0 + back_gap + pair_lane;
        pair += 1;
    }
}

/// A block of factories: one yard, or several on a large block.
///
/// Each yard has a drive round it inside its fence (8 m: a truck lane and the
/// parking along it), an office block of four or five storeys at the street
/// end, and portal-frame sheds in rows behind it: spans of 24 - 48 m, 60 - 120 m
/// long, eaves at 8 - 12 m, with 12 - 15 m between them for trucks and for the
/// fire spacing GB 50016 asks between such sheds.  A share of the yards are a
/// standard-factory estate (标准厂房) of four- and five-storey blocks instead.
fn industrial_block(env: &[V], spurs: &[(V, V, ModernRoadClass)], mut sink: PlanSink) {
    let seed = sink.seed;
    let mut yards: Vec<Vec<V>> = Vec::new();
    subdivide(env, 22_000.0, sink.seed, sink.key * 3 + 1, 0, &mut yards);
    for (index, yard) in yards.iter().enumerate() {
        if yard.len() < 4 || signed_area(yard) < 3_000.0 {
            continue;
        }
        let ob = obb(yard);
        if ob.width().min(ob.depth()) < 45.0 {
            continue;
        }
        let parcel_id = push_parcel(&mut sink, yard, ParcelUse::Industrial);
        let key = sink.key * 16 + index as i32;
        let h = |salt: i32| modern_hash(seed, key, 13, salt);
        let (u0, u1) = (ob.min_u + 8.0, ob.max_u - 8.0);
        let (v0, v1) = (ob.min_v + 8.0, ob.max_v - 8.0);
        let place = |sink: &mut PlanSink,
                     uv: [(f32, f32); 4],
                     floors: u16,
                     height: f32,
                     use_type: ParcelUse,
                     facade: BuildingFacade| {
            let ring: Vec<V> = uv.iter().map(|p| ob.to_world(p.0, p.1)).collect();
            if ring.iter().all(|p| point_in(yard, *p)) && !spur_hits(&ring, spurs, 4.0) {
                push_building(sink, parcel_id, &ring, floors, height, use_type, facade);
                true
            } else {
                false
            }
        };
        // The office block, across the street end of the yard.
        let office_len = (30.0 + 15.0 * h(821)).min(u1 - u0);
        let office_dep = 14.0 + 2.0 * h(823);
        let office_floors: u16 = 4 + u16::from(h(825) < 0.4);
        place(
            &mut sink,
            [
                (u0, v0),
                (u0 + office_len, v0),
                (u0 + office_len, v0 + office_dep),
                (u0, v0 + office_dep),
            ],
            office_floors,
            f32::from(office_floors) * 3.8 + 1.2,
            ParcelUse::Commercial,
            BuildingFacade::ConcreteGlass,
        );
        let estate = h(827) < 0.25;
        let mut v = v0 + office_dep + 15.0;
        let mut row = 0_i32;
        while v < v1 - 20.0 {
            let rh = |salt: i32| modern_hash(seed, key * 61 + row, 14, salt);
            let span = if estate {
                18.0 + 4.0 * rh(831)
            } else {
                24.0 + 24.0 * rh(831)
            };
            let span = span.min(v1 - v);
            if span < 18.0 {
                break;
            }
            let mut u = u0;
            let mut slot = 0_i32;
            while u < u1 - 30.0 {
                let sh = |salt: i32| modern_hash(seed, key * 977 + row * 31 + slot, 15, salt);
                let length = if estate {
                    48.0 + 24.0 * sh(833)
                } else {
                    60.0 + 60.0 * sh(833)
                };
                let length = length.min(u1 - u);
                if length < 30.0 {
                    break;
                }
                let (floors, height) = if estate {
                    let f: u16 = 4 + u16::from(sh(835) < 0.5);
                    (f, f32::from(f) * 4.5 + 1.0)
                } else {
                    (1, 8.0 + 4.0 * sh(835))
                };
                place(
                    &mut sink,
                    [
                        (u, v),
                        (u + length, v),
                        (u + length, v + span),
                        (u, v + span),
                    ],
                    floors,
                    height,
                    ParcelUse::Industrial,
                    BuildingFacade::MetalIndustrial,
                );
                u += length + 10.0 + 5.0 * sh(837);
                slot += 1;
            }
            v += span + 12.0 + 3.0 * rh(839);
            row += 1;
        }
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
const CELL_GAP_M: f32 = 6.0;
/// How far a compound's buildings stand inside its fence: the footway and the
/// 4 m fire lane that runs round inside it (GB 50016).
const COMPOUND_EDGE_M: f32 = 7.0;
/// Storey height used for sunlight spacing.
const HOME_STOREY_M: f32 = 2.95;

/// The buildable rectangle of a lot, in the lot's own frame.
#[derive(Clone, Copy)]
struct Site {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
}

/// When a compound was built, which fixes nearly everything about its blocks.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Era {
    /// 1980s - 1990s: six- and seven-storey walk-up slabs (多层), no lifts.
    Walkup,
    /// 2000s: 11 - 18-storey slabs (小高层), one lift core per unit.
    Slab,
    /// 2010s on: 24 - 33-storey slabs and point towers (高层).
    HighRise,
}

/// The dimensions of one era's blocks, from the code that governed them.
struct EraForm {
    floors: (u16, u16),
    length: (f32, f32),
    depth: (f32, f32),
    /// 日照间距系数: row spacing as a share of the height of the row to the south.
    /// About 1.2 - 1.5 for multi-storey rows in central and northern China; a
    /// high-rise compound is spaced by sunlight-hours analysis instead, which
    /// with staggered blocks lands near 0.5 - 0.7.
    spacing: f32,
    /// Gap between the ends of two blocks in a row: 6 m between multi-storey
    /// blocks, 9 m beside a 小高层, 13 m between high-rises (GB 50016).
    end_gap: (f32, f32),
    /// Share of the blocks that are square point towers.
    towers: f32,
}

fn era_form(era: Era) -> EraForm {
    match era {
        Era::Walkup => EraForm {
            floors: (6, 7),
            length: (42.0, 72.0),
            depth: (10.5, 12.5),
            spacing: 1.3,
            end_gap: (6.0, 12.0),
            towers: 0.0,
        },
        Era::Slab => EraForm {
            floors: (11, 18),
            length: (36.0, 60.0),
            depth: (13.0, 15.5),
            spacing: 1.0,
            end_gap: (9.0, 15.0),
            towers: 0.15,
        },
        Era::HighRise => EraForm {
            floors: (24, 33),
            length: (32.0, 48.0),
            depth: (15.0, 18.0),
            spacing: 0.6,
            end_gap: (13.0, 22.0),
            towers: 0.5,
        },
    }
}

/// The era of a compound.  Walk-ups are most of the older, inner stock and
/// high-rises most of the newer; a large city has all three in every ring.
fn compound_era(centrality: f32, density: f32, r: f32) -> Era {
    let g = smoothstep(0.15, 0.55, centrality);
    let high = (0.08 + 0.47 * g) * (0.6 + 0.4 * density);
    let walkup = 0.45 - 0.26 * g;
    if r < high {
        Era::HighRise
    } else if r < high + walkup {
        Era::Walkup
    } else {
        Era::Slab
    }
}

/// The buildings of one compound, as footprints in the lot's own frame with
/// their storeys.
///
/// Every Chinese compound is laid out the same way, which is what makes one
/// recognisable from the air: blocks in parallel rows facing south, each row
/// spaced from the row to its south by the sunlight rule, so the spacing grows
/// with the height.  Rows rise toward the north (南低北高) so the low rows do not
/// shade the tall ones.  Within a compound the blocks are of one era and nearly
/// one height; it is from compound to compound that a city varies.  A garden
/// opens in the middle of the site, and a clubhouse or kindergarten may stand
/// in it.
///
/// `north` is the direction of north in the lot frame; the rows run across it.
#[allow(clippy::too_many_arguments)]
fn chinese_compound_rows(
    site: Site,
    lot_uv: &[V],
    key: i32,
    seed: u32,
    centrality: f32,
    density: f32,
    north: V,
    fits: &dyn Fn(&[V]) -> bool,
) -> Vec<(Vec<V>, u16)> {
    let h = |salt: i32| modern_hash(seed, key, 5, salt);
    let era = compound_era(centrality, density, h(751));
    let form = era_form(era);
    let span = |range: (f32, f32), t: f32| range.0 + (range.1 - range.0) * t;
    // Work in a frame whose `b` axis points north and whose `a` axis runs along
    // the rows: rows are walked from the south, blocks along each row.
    let east = (north.1, -north.0);
    let to_ab = |p: V| (p.0 * east.0 + p.1 * east.1, p.0 * north.0 + p.1 * north.1);
    let from_ab = |p: V| (p.0 * east.0 + p.1 * north.0, p.0 * east.1 + p.1 * north.1);
    let ring_ab: Vec<V> = lot_uv.iter().map(|p| to_ab(*p)).collect();
    let corners = [
        (site.x0, site.z0),
        (site.x1, site.z0),
        (site.x1, site.z1),
        (site.x0, site.z1),
    ]
    .map(to_ab);
    let (a_lo, a_hi) = corners.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| {
        (lo.min(p.0), hi.max(p.0))
    });
    let (b_lo, b_hi) = corners.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| {
        (lo.min(p.1), hi.max(p.1))
    });
    let base_floors = form.floors.0 + ((form.floors.1 - form.floors.0 + 1) as f32 * h(753)) as u16;
    let base_floors = base_floors.min(form.floors.1);
    let depth = span(form.depth, h(755));
    let turn = (2.0 * h(757) - 1.0) * 6.0_f32.to_radians();

    // One block type for the whole compound, repeated: slabs, or point towers
    // set out in a staggered pattern so each sees past the one to its south.
    let towers = h(759) < form.towers;
    let length = if towers {
        depth.max(20.0) + 4.0 * h(775)
    } else {
        span(form.length, h(777))
    };
    let dep = if towers { length * 0.94 } else { depth };
    let gap = span(form.end_gap, h(785));

    // The rows, south to north, each spaced from the one before by its shadow.
    let avail = (b_hi - b_lo) - 2.0 * COMPOUND_EDGE_M;
    let mut rows: Vec<(f32, u16)> = Vec::new();
    let mut b = 0.0_f32;
    for row in 0..32_i32 {
        let rh = modern_hash(seed, key.wrapping_mul(131).wrapping_add(row), 9, 761);
        // South low, north high: a later row may step up a storey or three.
        let step = if era == Era::Walkup {
            u16::from(rh < 0.3)
        } else {
            (row as f32 * (0.5 + 1.0 * h(763))) as u16
        };
        let floors = (base_floors + step).min(form.floors.1);
        if b + dep > avail {
            break;
        }
        rows.push((b, floors));
        b += dep + (form.spacing * f32::from(floors) * HOME_STOREY_M).max(13.0);
    }
    if rows.is_empty() && avail >= 10.0 {
        rows.push((0.0, base_floors));
    }
    // Centre the stack between the south and north fences.
    let used = rows.last().map(|r| r.0 + dep.min(avail)).unwrap_or(0.0);
    let offset = b_lo + COMPOUND_EDGE_M + (avail - used).max(0.0) * 0.5;
    let middle = rows.len() / 2;

    let mut out: Vec<(Vec<V>, u16)> = Vec::new();
    let mut placed_ab: Vec<Vec<V>> = Vec::new();
    for (index, (rb, row_floors)) in rows.iter().enumerate() {
        let depth_here = dep.min(avail);
        let b0 = offset + rb;
        let Some((lo, hi)) =
            strip_span(&ring_ab, b0 - LOT_MARGIN_M, b0 + depth_here + LOT_MARGIN_M)
        else {
            continue;
        };
        let lo = lo.max(a_lo) + COMPOUND_EDGE_M;
        let hi = hi.min(a_hi) - COMPOUND_EDGE_M;
        let width = hi - lo;
        let min_len = if towers { 18.0 } else { 26.0 };
        let mut count = ((width + gap) / (length + gap)).floor() as usize;
        let mut len = length;
        if count == 0 {
            if width < min_len {
                continue;
            }
            count = 1;
            len = width;
        }
        // Staggered towers: every other row shifts half a bay, when there is room.
        let stagger = towers && index % 2 == 1 && count >= 2;
        if stagger {
            count -= 1;
        }
        let total = count as f32 * len + (count as f32 - 1.0) * gap;
        let start = lo + (width - total) * 0.5;
        for slot in 0..count {
            let cell = key
                .wrapping_mul(977)
                .wrapping_add(index as i32 * 64 + slot as i32);
            let ch = |salt: i32| modern_hash(seed, cell, 10, salt);
            let centre_a = start + slot as f32 * (len + gap) + len * 0.5;
            let centre = (centre_a, b0 + depth_here * 0.5);
            // The garden at the heart of the compound: the middle of the middle
            // row is left open, sometimes for a clubhouse or kindergarten.
            if index == middle && rows.len() >= 3 && count >= 3 && slot == count / 2 {
                if ch(769) < 0.5 {
                    let ring =
                        rect_at(centre.0, centre.1, (len * 0.5).clamp(14.0, 24.0), 14.0, 0.0);
                    if try_place(&ring, &placed_ab, &from_ab, fits) {
                        out.push((ring.iter().map(|p| from_ab(*p)).collect(), 3));
                        placed_ab.push(ring);
                    }
                }
                continue;
            }
            let floors = if era == Era::Walkup {
                *row_floors
            } else {
                // Blocks of one row differ by a storey or two, no more.
                (*row_floors + (ch(783) * 3.0) as u16).clamp(form.floors.0, form.floors.1)
            };
            let d = if towers { len.min(dep) } else { depth_here };
            let placed = [(turn, 1.0), (0.0, 1.0), (0.0, 0.85)]
                .into_iter()
                .find_map(|(t, k)| {
                    let ring = rect_at(centre.0, centre.1, len * k, d, t);
                    try_place(&ring, &placed_ab, &from_ab, fits).then_some(ring)
                });
            if let Some(ring) = placed {
                out.push((ring.iter().map(|p| from_ab(*p)).collect(), floors));
                placed_ab.push(ring);
            }
        }
    }
    // A frame turned through `east`/`north` can mirror the lot frame; keep every
    // footprint counter-clockwise, as the rest of the pipeline expects.
    for (ring, _) in &mut out {
        if signed_area(ring) < 0.0 {
            ring.reverse();
        }
    }
    out
}

/// Whether `ring` (in the row frame) fits the lot and clears every block placed.
fn try_place(
    ring: &[V],
    placed: &[Vec<V>],
    from_ab: &dyn Fn(V) -> V,
    fits: &dyn Fn(&[V]) -> bool,
) -> bool {
    let lot_frame: Vec<V> = ring.iter().map(|p| from_ab(*p)).collect();
    let mut oriented = lot_frame.clone();
    if signed_area(&oriented) < 0.0 {
        oriented.reverse();
    }
    fits(&oriented)
        && placed
            .iter()
            .all(|other| convex_separation(ring, other) >= CELL_GAP_M)
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
    use super::{
        HOME_STOREY_M, Site, chinese_compound_rows, chinese_residential_storeys, convex_separation,
        rect_at,
    };
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
                rows.len() > 100,
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
            // Enough to take a median of; the core is mostly offices and malls.
            assert!(core_n >= 25, "seed {seed}: only {core_n} core buildings");
            assert!(
                (15.0..=28.0).contains(&core_median),
                "seed {seed}: core median {core_median} storeys"
            );
            assert!(
                core_18 >= 0.30,
                "seed {seed}: core share at 18+ is {core_18}"
            );

            let outer: Vec<u16> = rows.iter().filter(|r| r.0 < 0.2).map(|r| r.1).collect();
            let outer_n = outer.len();
            let outer_median = median(outer.clone());
            // The edge is older and lower: walk-up rows are a real share of it.
            let outer_walkup =
                outer.iter().filter(|f| (6..=7).contains(*f)).count() as f32 / outer_n as f32;
            // A whole compound is of one era, so a median moves by several storeys
            // with each compound: the gradient is the assertion, not exact values.
            assert!(
                (6.0..=14.0).contains(&outer_median) && core_median >= outer_median + 4.0,
                "seed {seed}: core median {core_median}, outer median {outer_median} storeys"
            );
            assert!(
                outer_walkup >= 0.25,
                "seed {seed}: outer share of walk-ups is {outer_walkup}"
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
    fn a_city_has_compounds_of_every_era() {
        for seed in [42_u32, 7] {
            let city = generate_modern_chinese_city(ModernChinaSpec {
                seed,
                ..ModernChinaSpec::default()
            });
            let compounds = compound_buildings(&city);
            assert!(
                compounds.len() >= 20,
                "seed {seed}: {} compounds",
                compounds.len()
            );
            let tallest = |b: &Vec<(u16, Metres)>| b.iter().map(|x| x.0).max().unwrap_or(0);
            let walkups = compounds.values().filter(|b| tallest(b) <= 7).count();
            let slabs = compounds
                .values()
                .filter(|b| (11..=20).contains(&tallest(b)))
                .count();
            let high = compounds.values().filter(|b| tallest(b) >= 24).count();
            assert!(
                walkups > 0 && slabs > 0 && high > 0,
                "seed {seed}: walk-up {walkups}, slab {slabs}, high-rise {high} compounds"
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
            assert!(homes.len() > 100, "seed {seed}: {} homes", homes.len());
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
    fn villages_and_industrial_parks_have_their_own_buildings() {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 3.0,
            block_size_metres: 180.0,
            ..ModernChinaSpec::default()
        });
        let uses: HashMap<u32, ParcelUse> =
            city.parcels.iter().map(|p| (p.id, p.use_type)).collect();
        let on = |use_type: ParcelUse| -> Vec<&crate::ModernBuilding> {
            city.buildings
                .iter()
                .filter(|b| uses.get(&b.parcel_id) == Some(&use_type))
                .collect()
        };
        let houses = on(ParcelUse::Village);
        assert!(houses.len() > 100, "{} village houses", houses.len());
        // Mostly six to eight storeys; a few old two-storey houses, merged plots to fourteen.
        assert!(houses.iter().all(|b| (2..=14).contains(&b.floors)));
        let mid = houses
            .iter()
            .filter(|b| (6..=8).contains(&b.floors))
            .count() as f32;
        assert!(
            mid / houses.len() as f32 > 0.4,
            "six to eight storeys is not the mode"
        );
        let sheds: Vec<_> = on(ParcelUse::Industrial)
            .into_iter()
            .filter(|b| b.use_type == ParcelUse::Industrial && b.floors == 1)
            .collect();
        assert!(sheds.len() > 20, "{} sheds", sheds.len());
        assert!(
            sheds
                .iter()
                .all(|b| (8.0..=12.5).contains(&b.height_metres))
        );
        // Neither the houses of a village nor the sheds of a yard touch.
        let metres = |b: &crate::ModernBuilding| -> Metres {
            b.footprint
                .iter()
                .map(|p| (p.x_km * 1000.0, p.y_km * 1000.0))
                .collect()
        };
        for group in [&houses, &sheds] {
            let rings: Vec<Metres> = group.iter().map(|b| metres(b)).collect();
            for (i, a) in rings.iter().enumerate() {
                for b in &rings[i + 1..] {
                    assert!(convex_separation(a, b) > 0.5, "two buildings touch");
                }
            }
        }
    }

    #[test]
    fn compound_rows_stand_clear_of_the_shadow_of_the_row_south() {
        let site = Site {
            x0: 0.0,
            x1: 220.0,
            z0: 0.0,
            z1: 240.0,
        };
        let lot = rect_at(110.0, 120.0, 220.0, 240.0, 0.0);
        for key in 0..40 {
            for centrality in [0.1_f32, 0.5, 0.9] {
                let blocks = chinese_compound_rows(
                    site,
                    &lot,
                    key,
                    42,
                    centrality,
                    0.8,
                    (0.0, 1.0),
                    &|_| true,
                );
                assert!(!blocks.is_empty(), "key {key}: an empty compound");
                let extent = |r: &[(f32, f32)], i: usize| {
                    r.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| {
                        let v = if i == 0 { p.0 } else { p.1 };
                        (lo.min(v), hi.max(v))
                    })
                };
                for (south, hs) in &blocks {
                    for (north, _) in &blocks {
                        let (sx, nx) = (extent(south, 0), extent(north, 0));
                        let (sz, nz) = (extent(south, 1), extent(north, 1));
                        let overlap_x = sx.1.min(nx.1) - sx.0.max(nx.0);
                        let gap = nz.0 - sz.1;
                        // Only a block squarely north of another, in a later row.
                        if overlap_x < 10.0 || gap <= 0.0 || nz.0 < sz.1 {
                            continue;
                        }
                        // Slack for the few degrees a row may be turned.
                        let need = (0.6 * f32::from(*hs) * HOME_STOREY_M).max(13.0) - 10.0;
                        assert!(
                            gap >= need,
                            "key {key}: {gap} m north of a {hs}-storey block, needs {need}"
                        );
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
