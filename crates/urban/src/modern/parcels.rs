use super::CityFrame;
use super::graph::modern_hash;
use crate::{
    BuildingFacade, ModernBuilding, ModernRoadClass, Parcel, ParcelUse, RoofStyle, UrbanBlock,
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

/// How far a parcel edge sits back from the street centreline: half the
/// carriageway ribbon plus a sidewalk/green margin, so the massing leaves the
/// real right-of-way empty.
fn setback_for(class: ModernRoadClass) -> f32 {
    class.width_metres() * 0.5 + 2.0
}

pub(super) fn build_parcels(
    frame: &CityFrame,
    x_lines: &[(f32, ModernRoadClass)],
    z_lines: &[(f32, ModernRoadClass)],
) -> ParcelOutput {
    let radius_m = frame.radius_m;
    let block_m = frame.block_m;
    let density = frame.density;
    let river_half = frame.river_half;
    let seed = frame.spec.seed;
    let mut blocks = Vec::new();
    let mut parcels = Vec::new();
    let mut buildings = Vec::new();
    let mut next_parcel = 0_u32;
    let mut next_building = 0_u32;
    for ix in 0..x_lines.len().saturating_sub(1) {
        for iz in 0..z_lines.len().saturating_sub(1) {
            let x0 = x_lines[ix].0;
            let x1 = x_lines[ix + 1].0;
            let z0 = z_lines[iz].0;
            let z1 = z_lines[iz + 1].0;
            let cx = (x0 + x1) * 0.5;
            let cz = (z0 + z1) * 0.5;
            if cx.hypot(cz) > radius_m * 1.04 || (cx - frame.river_x(cz)).abs() < river_half + 10.0
            {
                continue;
            }
            let block_id = blocks.len() as u32;
            let ring = vec![
                frame.to_world(x0, z0),
                frame.to_world(x1, z0),
                frame.to_world(x1, z1),
                frame.to_world(x0, z1),
            ];
            let local_area = (x1 - x0).abs() * (z1 - z0).abs();
            let block_noise = modern_hash(seed, ix as i32, iz as i32, 701);
            let centrality = (1.0 - cx.hypot(cz) / radius_m).clamp(0.0, 1.0);
            let park = block_noise < 0.025 + 0.045 * (1.0 - density)
                || (local_area > block_m * block_m * 1.7 && block_noise < 0.08);
            blocks.push(UrbanBlock {
                boundary: ring.clone(),
                courtyard: None,
            });
            // Adjacent street classes gate the setback on each side and which
            // edge the compound gate prefers (the widest road wins).
            let side_classes = [
                z_lines[iz].1,     // edge 0: z0 side
                x_lines[ix + 1].1, // edge 1: x1 side
                z_lines[iz + 1].1, // edge 2: z1 side
                x_lines[ix].1,     // edge 3: x0 side
            ];
            let gate_edge = (0..4)
                .max_by_key(|edge| side_classes[*edge] as i32)
                .unwrap_or(0) as u8;
            let split_x = (x1 - x0).abs() >= (z1 - z0).abs();
            let split = if split_x {
                x0 + (x1 - x0) * (0.45 + 0.10 * modern_hash(seed, ix as i32, iz as i32, 707))
            } else {
                z0 + (z1 - z0) * (0.45 + 0.10 * modern_hash(seed, ix as i32, iz as i32, 709))
            };
            let parcel_ranges: [(f32, f32, f32, f32); 2] = if split_x {
                [(x0, split, z0, z1), (split, x1, z0, z1)]
            } else {
                [(x0, x1, z0, split), (x0, x1, split, z1)]
            };
            for (parcel_index, (px0, px1, pz0, pz1)) in parcel_ranges.into_iter().enumerate() {
                let parcel_id = next_parcel;
                next_parcel += 1;
                let pr = vec![
                    frame.to_world(px0, pz0),
                    frame.to_world(px1, pz0),
                    frame.to_world(px1, pz1),
                    frame.to_world(px0, pz1),
                ];
                let n = modern_hash(seed, ix as i32 * 17 + parcel_index as i32, iz as i32, 719);
                let use_type = if park {
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
                    && (px1 - px0).abs() > 55.0
                    && (pz1 - pz0).abs() > 40.0;
                parcels.push(Parcel {
                    id: parcel_id,
                    block_id,
                    ring: pr.clone(),
                    use_type,
                    compound,
                    gate_edge,
                });
                if matches!(use_type, ParcelUse::Park) {
                    continue;
                }
                // Buildable envelope after the real setbacks of the bounding
                // streets.  Chinese residential compounds wall to the red line;
                // commercial fronts its podium straight onto the sidewalk.
                let bx0 = px0 + setback_for(side_classes[3]);
                let bx1 = px1 - setback_for(side_classes[1]);
                let bz0 = pz0 + setback_for(side_classes[0]);
                let bz1 = pz1 - setback_for(side_classes[2]);
                let width = (bx1 - bx0).abs();
                let depth = (bz1 - bz0).abs();
                if width < 22.0 || depth < 22.0 {
                    // Too tight between carriageways: surface parking or a
                    // pocket garden instead of forced massing.
                    continue;
                }
                let parcel_area = width * depth;
                let noise = |salt: i32| {
                    modern_hash(
                        seed,
                        ix as i32 * 17 + parcel_index as i32,
                        iz as i32,
                        salt,
                    )
                };
                let mut emit = |footprint: Vec<(f32, f32)>,
                                floors: u16,
                                use_type: ParcelUse,
                                podium_floors: u16,
                                facade: BuildingFacade,
                                roof: RoofStyle,
                                balconies: bool| {
                    let max_side = footprint
                        .iter()
                        .zip(footprint.iter().cycle().skip(1))
                        .take(footprint.len())
                        .map(|(a, b)| (a.0 - b.0).hypot(a.1 - b.1))
                        .fold(0.0_f32, f32::max);
                    // Bay rhythm, storey heights and opening ratios come from
                    // the shared building system so every facade follows the
                    // same architectural logic.
                    let facade_kind = match use_type {
                        ParcelUse::Commercial => procedural::FacadeKind::CurtainWall,
                        ParcelUse::MixedUse => procedural::FacadeKind::ConcreteGlass,
                        ParcelUse::Civic => procedural::FacadeKind::StoneCivic,
                        ParcelUse::Residential => procedural::FacadeKind::Residential,
                        ParcelUse::Park => procedural::FacadeKind::Residential,
                    };
                    let metrics = procedural::facade_metrics(
                        max_side,
                        max_side * 0.8,
                        floors,
                        facade_kind,
                    );
                    let podium_height = podium_floors as f32 * PODIUM_FLOOR_M;
                    let floor_height = metrics.storey_height_m;
                    buildings.push(ModernBuilding {
                        id: next_building,
                        parcel_id,
                        footprint: footprint
                            .iter()
                            .map(|(x, z)| frame.to_world(*x, *z))
                            .collect(),
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
                        // 零售裙房 fills the frontage with a 3-4 storey podium;
                        // one or two office towers rise from it, sized by the
                        // plot's FAR so the density matches a real 综合体.
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
                        let tower_floors = ((tower_far * parcel_area * 0.55 / tower_area)
                            .round() as u16)
                            .clamp(10, 38);
                        let tower_w = 24.0 * (0.85 + noise(737) * 0.3);
                        let tower_d = 20.0 * (0.85 + noise(739) * 0.3);
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
                                (
                                    bx0 + width * 0.5,
                                    bz0 + depth * (0.32 + 0.36 * noise(743)),
                                )
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
                        // 街墙 podium along the widest street + residences
                        // behind: the classic tower-on-podium street block.
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
                        // The tower carries about 55% of the plot FAR; the
                        // street podium already holds the rest.
                        let tower_floors = ((far * parcel_area * 0.55 / tower_area).round()
                            as u16)
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
                        // Schools, clinics and community halls: low slabs in
                        // green, floor counts pinned by code, not by market.
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
                        // 高层点式塔楼 vs 板楼, chosen per parcel.  Floors are
                        // solved from the plot's FAR (容积率 2.4-3.4) and then
                        // capped by the 日照 sunlight pitch — one row of tall
                        // towers and two rows of mid-rise carry the same FAR,
                        // exactly the trade-off a real plan makes.
                        let far = 2.4 + 1.0 * centrality * density;
                        let slab = noise(753) < 0.42 && depth >= 46.0;
                        if slab {
                            let slab_w = (width * 0.72).clamp(46.0, 78.0);
                            let slab_d = 12.5 + noise(757) * 2.5;
                            let footprint_area = slab_w * slab_d;
                            let far_floors = |rows: f32| {
                                ((far * parcel_area / (footprint_area * rows)).round() as u16)
                                    .clamp(6, 18)
                            };
                            let mut rows = 1_u16;
                            let mut floors = far_floors(1.0);
                            let pitch_needed = floors as f32 * RESIDENTIAL_FLOOR_M * 1.2
                                + slab_d
                                + 6.0;
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
                            // 点式塔楼: compact one-staircase points in a
                            // single south-facing row (or two when the plot is
                            // deep enough for the sunlight pitch).
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
                            let pitch_needed = floors as f32 * RESIDENTIAL_FLOOR_M * 1.1
                                + tower_d
                                + 6.0;
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
    ParcelOutput {
        blocks,
        parcels,
        buildings,
    }
}

fn rect(x0: f32, x1: f32, z0: f32, z1: f32, shrink: f32) -> Vec<(f32, f32)> {
    let dx = (x1 - x0) * shrink * 0.5;
    let dz = (z1 - z0) * shrink * 0.5;
    vec![
        (x0 + dx, z0 + dz),
        (x1 - dx, z0 + dz),
        (x1 - dx, z1 - dz),
        (x0 + dx, z1 - dz),
    ]
}
