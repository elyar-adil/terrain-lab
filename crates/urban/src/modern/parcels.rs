//! Blocks, lots and building massing derived from the *actual* road graph.
//!
//! Blocks are the bounded faces of the planar street graph.  Each block is set
//! back from every bounding street by that street's real right-of-way (half
//! carriageway + sidewalk + a building line), split into lots by recursive
//! oriented-box bisection, and massed inside the lot.  Because the envelope is
//! computed from the streets themselves, a building can never sit on a road,
//! whatever shape the network has.

use super::CityFrame;
use super::blocks::{Extraction, extract_faces};
use super::geom::{
    Obb, V, centroid, clip_half, obb, point_in, point_seg_dist, ring_polyline_dist, seg_seg_dist,
    signed_area,
};
use super::graph::modern_hash;
use crate::model::cross_section;
use crate::{
    BuildingFacade, ModernBuilding, ModernRoadClass, Parcel, ParcelUse, Point, RoofStyle, SdNode,
    SdRoad, UrbanBlock,
};

pub(super) struct ParcelOutput {
    pub blocks: Vec<UrbanBlock>,
    pub parcels: Vec<Parcel>,
    pub buildings: Vec<ModernBuilding>,
}

/// Design floor-to-floor heights, metres.  Residential runs 2.95-3.0 m, offices
/// 3.9-4.2 m and retail podiums 4.5-5.1 m in Chinese practice.
const RESIDENTIAL_FLOOR_M: f32 = 3.0;
const PODIUM_FLOOR_M: f32 = 4.5;
/// Gap left between two lots that share a bisection line.
const LOT_GAP_M: f32 = 1.5;
/// Extra clearance between a footprint and its lot line.
const LOT_MARGIN_M: f32 = 2.0;
/// Building line: distance from the back of the sidewalk to the wall.
const BUILDING_LINE_M: f32 = 3.0;
/// Clearance kept between any building lot and the river bank.
const RIVER_BANK_M: f32 = 12.0;

/// Distance from a street centreline to the nearest permitted wall.
fn right_of_way(class: ModernRoadClass) -> f32 {
    let s = cross_section(class);
    s.width_metres * 0.5 + s.sidewalk_metres + BUILDING_LINE_M
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

    let pts: Vec<V> = nodes.iter().map(|n| frame.to_local(n.point)).collect();
    let edges: Vec<(usize, usize, ModernRoadClass)> = sd_roads
        .iter()
        .filter(|r| !r.bridge)
        .map(|r| (r.from as usize, r.to as usize, r.class))
        .collect();
    let Extraction { faces, spurs } = extract_faces(&pts, &edges);

    let river_local: Vec<V> = (0..=48)
        .map(|i| {
            let z = -radius_m * 1.2 + 2.4 * radius_m * i as f32 / 48.0;
            (frame.river_x(z), z)
        })
        .collect();

    let mut blocks: Vec<UrbanBlock> = Vec::new();
    let mut parcels: Vec<Parcel> = Vec::new();
    let mut buildings: Vec<ModernBuilding> = Vec::new();
    let mut next_parcel = 0_u32;
    let mut next_building = 0_u32;
    let to_world =
        |ring: &[V]| -> Vec<Point> { ring.iter().map(|p| frame.to_world(p.0, p.1)).collect() };

    for (face_index, face) in faces.iter().enumerate() {
        let centre = centroid(&face.ring);
        if centre.0.hypot(centre.1) > radius_m * 1.04 {
            continue;
        }
        if signed_area(&face.ring) < 900.0 {
            continue;
        }
        let fi = face_index as i32;
        let centrality = (1.0 - centre.0.hypot(centre.1) / radius_m).clamp(0.0, 1.0);

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
            envelope = clip_half(
                &envelope,
                n,
                n.0 * a.0 + n.1 * a.1 + right_of_way(face.classes[i]),
            );
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
            let inset = (right_of_way(face.classes[i]) - BUILDING_LINE_M - 1.2).max(1.0);
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
                let rx = frame.river_x(c.1);
                let slope = (frame.river_x(c.1 + 5.0) - frame.river_x(c.1 - 5.0)) / 10.0;
                let norm = (1.0 + slope * slope).sqrt();
                let n = (1.0 / norm, -slope / norm);
                let d = n.0 * rx + n.1 * c.1;
                [clip_half(ring, n, d + bank), clip_half(ring, (-n.0, -n.1), -d + bank)]
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
            blocks.push(UrbanBlock { boundary: to_world(ring), courtyard: None });
        }
        if block_rings.is_empty() {
            continue;
        }
        // Parcels reference the block that contains their envelope.
        let block_base = (blocks.len() - block_rings.len()) as u32;

        let block_noise = modern_hash(seed, fi, 0, 701);
        let face_area = signed_area(&face.ring);
        let park_block = block_noise < 0.025 + 0.045 * (1.0 - density)
            || (face_area > 14_000.0 && block_noise < 0.10);

        // Widest street around this block: where compound gates face.
        let widest = (0..face.ring.len())
            .max_by_key(|i| face.classes[*i] as i32)
            .unwrap_or(0);
        let (wa, wb) = (face.ring[widest], face.ring[(widest + 1) % face.ring.len()]);

        let target = 2600.0
            + 2400.0 * (1.0 - centrality) * (0.6 + 0.8 * modern_hash(seed, fi, 3, 761));
        for (env_index, env) in envelopes.iter().enumerate() {
            let mut lots: Vec<Vec<V>> = Vec::new();
            subdivide(env, target, seed, fi * 8 + env_index as i32, 0, &mut lots);
            let block_id = block_base + (env_index as u32).min(block_rings.len() as u32 - 1);
            for (lot_index, lot) in lots.iter().enumerate() {
                let ob = obb(lot);
                let lot_area = signed_area(lot);
                if lot.len() < 4 || lot_area < 400.0 || ob.width().min(ob.depth()) < 12.0 {
                    continue;
                }
                let key = fi * 64 + (env_index as i32) * 16 + lot_index as i32;
                let n = modern_hash(seed, key, 1, 719);
                let noise = |salt: i32| modern_hash(seed, key, 2, salt);
                let waterfront = ring_polyline_dist(lot, &river_local) < river_half + RIVER_BANK_M;
                let use_type = if park_block || waterfront {
                    ParcelUse::Park
                } else if n < 0.17 + 0.18 * centrality {
                    ParcelUse::Commercial
                } else if n < 0.44 {
                    ParcelUse::MixedUse
                } else if n < 0.52 {
                    ParcelUse::Civic
                } else {
                    ParcelUse::Residential
                };
                let compound = matches!(use_type, ParcelUse::Residential)
                    && ob.width() > 55.0
                    && ob.depth() > 40.0;
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
                if width < 22.0 || depth < 22.0 {
                    continue;
                }
                let parcel_area = lot_area.min(width * depth * 1.15);

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
                    let placed = [1.0_f32, 0.92, 0.84, 0.76, 0.68].iter().find_map(|s| {
                        let ring: Vec<V> = footprint
                            .iter()
                            .map(|p| {
                                ob.to_world(fc.0 + (p.0 - fc.0) * s, fc.1 + (p.1 - fc.1) * s)
                            })
                            .collect();
                        let clear_of_spurs = spurs.iter().all(|(a, b, class)| {
                            let row = right_of_way(*class);
                            ring.iter().all(|p| point_seg_dist(*p, *a, *b) >= row)
                                && ring
                                    .iter()
                                    .zip(ring.iter().cycle().skip(1))
                                    .all(|(p, q)| seg_seg_dist(*p, *q, *a, *b) >= row)
                        });
                        (clear_of_spurs && ring.iter().all(|p| point_in(lot, *p))).then_some(ring)
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
                        ParcelUse::Residential => procedural::FacadeKind::Residential,
                        ParcelUse::Park => procedural::FacadeKind::Residential,
                    };
                    let metrics =
                        procedural::facade_metrics(max_side, max_side * 0.8, floors, facade_kind);
                    let podium_height = podium_floors as f32 * PODIUM_FLOOR_M;
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
                        let podium_floors = if noise(731) < 0.5 { 3 } else { 4 };
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
                        let tower_far = 2.2 + 2.2 * centrality * density;
                        let tower_floors = ((tower_far * parcel_area * 0.55 / tower_area).round()
                            as u16)
                            .clamp(10, 38);
                        let tower_w = 24.0 * (0.85 + noise(737) * 0.3);
                        let tower_d = 20.0 * (0.85 + noise(739) * 0.3);
                        let towers = if parcel_area > 6_800.0 && noise(741) < 0.45 { 2 } else { 1 };
                        for t in 0..towers {
                            let (tx, tz) = if towers == 2 {
                                (
                                    if t == 0 { bx0 + width * 0.26 } else { bx0 + width * 0.74 },
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
                        let far = 2.2 + 1.4 * centrality * density;
                        let tower_floors = ((far * parcel_area * 0.55 / tower_area).round() as u16)
                            .clamp(9, 33);
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
                    ParcelUse::Residential => {
                        let far = 2.4 + 1.0 * centrality * density;
                        let slab = noise(753) < 0.55 && depth >= 46.0;
                        if slab {
                            let slab_w = (width * 0.72).clamp(46.0, 78.0).min(width);
                            let slab_d = 12.5 + noise(757) * 2.5;
                            let footprint_area = slab_w * slab_d;
                            let far_floors = |rows: f32| {
                                ((far * parcel_area / (footprint_area * rows)).round() as u16)
                                    .clamp(6, 18)
                            };
                            let mut rows = 1_u16;
                            let mut floors = far_floors(1.0);
                            let pitch_needed =
                                floors as f32 * RESIDENTIAL_FLOOR_M * 1.2 + slab_d + 6.0;
                            if depth >= pitch_needed + slab_d {
                                rows = 2;
                                floors = far_floors(2.0);
                            }
                            for row in 0..rows {
                                let row_z = if rows == 1 {
                                    bz0 + depth * 0.5
                                } else {
                                    bz0 + depth * (if row == 0 { 0.26 } else { 0.74 })
                                };
                                emit(
                                    rect(
                                        bx0 + width * 0.5 - slab_w * 0.5,
                                        bx0 + width * 0.5 + slab_w * 0.5,
                                        row_z - slab_d * 0.5,
                                        row_z + slab_d * 0.5,
                                        0.0,
                                    ),
                                    floors,
                                    ParcelUse::Residential,
                                    0,
                                    BuildingFacade::BrickResidential,
                                    RoofStyle::Flat,
                                    true,
                                );
                            }
                        } else {
                            let tower_w = 23.0 * (0.88 + noise(759) * 0.24);
                            let tower_d = 19.0 * (0.88 + noise(761) * 0.24);
                            let cols = if width >= 78.0 { 2 } else { 1 };
                            let footprint_area = tower_w * tower_d;
                            let far_floors = |masses: f32| {
                                ((far * parcel_area / (footprint_area * masses)).round() as u16)
                                    .clamp(8, 33)
                            };
                            let mut rows = 1_u16;
                            let mut floors = far_floors(cols as f32);
                            let pitch_needed =
                                floors as f32 * RESIDENTIAL_FLOOR_M * 1.1 + tower_d + 6.0;
                            if depth >= pitch_needed + tower_d {
                                rows = 2;
                                floors = far_floors(cols as f32 * 2.0);
                            }
                            for row in 0..rows {
                                let row_z = if rows == 1 {
                                    bz0 + depth * 0.5
                                } else {
                                    bz0 + depth * (if row == 0 { 0.24 } else { 0.76 })
                                };
                                for col in 0..cols {
                                    let col_x = if cols == 1 {
                                        bx0 + width * 0.5
                                    } else {
                                        bx0 + width * (if col == 0 { 0.27 } else { 0.73 })
                                    };
                                    emit(
                                        rect(
                                            col_x - tower_w * 0.5,
                                            col_x + tower_w * 0.5,
                                            row_z - tower_d * 0.5,
                                            row_z + tower_d * 0.5,
                                            0.0,
                                        ),
                                        floors,
                                        ParcelUse::Residential,
                                        0,
                                        BuildingFacade::BrickResidential,
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
                    }
                    ParcelUse::Park => {}
                }
            }
        }
    }
    ParcelOutput { blocks, parcels, buildings }
}

/// Recursive oriented-box bisection into lots no larger than `target` m².
fn subdivide(poly: &[V], target: f32, seed: u32, key: i32, depth: u32, out: &mut Vec<Vec<V>>) {
    let area = signed_area(poly);
    let ob: Obb = obb(poly);
    let (w, d) = (ob.width(), ob.depth());
    if area <= target || depth >= 6 || w.max(d) < 48.0 {
        out.push(poly.to_vec());
        return;
    }
    let t = 0.42 + 0.16 * modern_hash(seed, key, depth as i32, 769 + depth as i32);
    // Cut across the longer side.
    let (axis, lo, hi) =
        if w >= d { (ob.u, ob.min_u, ob.max_u) } else { (ob.v, ob.min_v, ob.max_v) };
    let s = lo + (hi - lo) * t;
    let a = clip_half(poly, (-axis.0, -axis.1), -(s - LOT_GAP_M));
    let b = clip_half(poly, axis, s + LOT_GAP_M);
    if a.len() < 3 || b.len() < 3 || signed_area(&a) < 500.0 || signed_area(&b) < 500.0 {
        out.push(poly.to_vec());
        return;
    }
    subdivide(&a, target, seed, key * 2, depth + 1, out);
    subdivide(&b, target, seed, key * 2 + 1, depth + 1, out);
}

/// Axis-aligned rectangle in a lot's own frame (CCW), optionally shrunk.
fn rect(x0: f32, x1: f32, z0: f32, z1: f32, shrink: f32) -> Vec<V> {
    let dx = (x1 - x0) * shrink * 0.5;
    let dz = (z1 - z0) * shrink * 0.5;
    vec![(x0 + dx, z0 + dz), (x1 - dx, z0 + dz), (x1 - dx, z1 - dz), (x0 + dx, z1 - dz)]
}
