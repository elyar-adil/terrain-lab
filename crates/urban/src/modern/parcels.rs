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
    BuildingFacade, ModernBuilding, ModernRoadClass, Parcel, ParcelUse, Point, RoofStyle, SdNode,
    SdRoad, UrbanBlock,
};

pub(super) struct ParcelOutput {
    pub blocks: Vec<UrbanBlock>,
    pub parcels: Vec<Parcel>,
    pub buildings: Vec<ModernBuilding>,
    pub fields: Vec<crate::Field>,
}

/// Design floor-to-floor heights, metres.  Residential runs 2.95-3.0 m, offices
/// 3.9-4.2 m and retail podiums 4.5-5.1 m in Chinese practice.
const RESIDENTIAL_FLOOR_M: f32 = 3.0;
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
    if face.open[i] { LOT_GAP_M * 0.5 } else { right_of_way(face.classes[i]) }
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
            face.ring.iter().fold(built, |m, p| m.max(frame.urbanness(p.0, p.1)))
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
            * if frame.organic_footprint { 0.35 + 0.65 * built } else { 1.0 };

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
                n.0 * a.0 + n.1 * a.1 + edge_row(face, i),
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
            let inset = if face.open[i] { 0.0 } else { (right_of_way(face.classes[i]) - BUILDING_LINE_M - 1.2).max(1.0) };
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
                    river: &river_local,
                    river_half,
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
        let superblock = face_area > 13_000.0
            && centrality < 0.55
            && modern_hash(seed, fi, 4, 777) < 0.45;
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
                let waterfront = ring_polyline_dist(lot, &river_local)
                    < river_half + QUAY_OFF_M + 45.0;
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
                    let placed = [1.0_f32, 0.92, 0.84, 0.76, 0.68, 0.58, 0.5].iter().find_map(|s| {
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
                            let clear_of_spurs = spurs.iter().all(|(a, b, class)| {
                                let row = right_of_way(*class);
                                ring.iter().all(|p| point_seg_dist(*p, *a, *b) >= row)
                                    && ring
                                        .iter()
                                        .zip(ring.iter().cycle().skip(1))
                                        .all(|(p, q)| seg_seg_dist(*p, *q, *a, *b) >= row)
                            });
                            (clear_of_spurs && ring.iter().all(|p| point_in(lot, *p))).then_some(ring)
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
                        let podium_floors =
                            if centrality < 0.3 || noise(731) < 0.5 { 3 } else { 4 };
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
                        let tower_floors = ((6.0 + 30.0 * centrality.powf(1.15) * (0.7 + 0.3 * density))
                            * (0.72 + 0.56 * noise(745)))
                        .round()
                        .clamp(5.0, 38.0) as u16;
                        let size = 0.85 + 0.3 * centrality;
                        let tower_w = 30.0 * size * (0.85 + noise(737) * 0.3);
                        let tower_d = 26.0 * size * (0.85 + noise(739) * 0.3);
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
                        let _ = tower_area;
                        let tower_floors = ((5.0 + 24.0 * centrality.powf(1.15) * (0.7 + 0.3 * density))
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
                        let floors = (4.0 + noise(763) * 5.0 + 11.0 * centrality.powf(1.2) * density).round() as u16;
                        emit(
                            rect(bx0, bx1, bz0, bz1, 0.03 + 0.2 * (1.0 - centrality)),
                            floors.clamp(4, 18),
                            ParcelUse::Residential,
                            0,
                            BuildingFacade::BrickResidential,
                            RoofStyle::Flat,
                            true,
                        );
                    }
                    ParcelUse::Residential => {
                        // A walled compound: the lot is tiled into cells, each
                        // holding a slab or tower, a minority left as garden.
                        let slab_style = noise(753) < 0.5;
                        let (cw, cd) = if slab_style { (46.0, 28.0) } else { (34.0, 30.0) };
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
                                let floors = (6.0
                                    + (4.0 + 22.0 * centrality) * (0.5 + 0.7 * cn(783)))
                                .round()
                                .clamp(6.0, 34.0) as u16;
                                let (w, d) = if slab_style {
                                    ((cell_w * 0.86).min(72.0), 12.5 + cn(785) * 2.5)
                                } else {
                                    (
                                        (23.0 * (0.88 + cn(787) * 0.24)).min(cell_w * 0.82),
                                        (19.0 * (0.88 + cn(789) * 0.24)).min(cell_d * 0.82),
                                    )
                                };
                                emit(
                                    rect(cx - w * 0.5, cx + w * 0.5, cz - d * 0.5, cz + d * 0.5, 0.0),
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
                    ParcelUse::Park | ParcelUse::Villa | ParcelUse::Farmstead => {}
                }
            }
        }
    }
    ParcelOutput { blocks, parcels, buildings, fields }
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
    let (axis, lo, hi) =
        if w >= d { (ob.u, ob.min_u, ob.max_u) } else { (ob.v, ob.min_v, ob.max_v) };
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

/// Axis-aligned rectangle in a lot's own frame (CCW), optionally shrunk.
fn rect(x0: f32, x1: f32, z0: f32, z1: f32, shrink: f32) -> Vec<V> {
    let dx = (x1 - x0) * shrink * 0.5;
    let dz = (z1 - z0) * shrink * 0.5;
    vec![(x0 + dx, z0 + dz), (x1 - dx, z0 + dz), (x1 - dx, z1 - dz), (x0 + dx, z1 - dz)]
}
