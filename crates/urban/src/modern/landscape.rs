use super::graph::modern_hash;
use crate::math::{centroid, scale_polygon};
use crate::{Compound, Parcel, ParcelUse, Point, TreeInstance, TreeSpecies};

pub(super) fn derive_compounds_and_trees(
    parcels: &[Parcel],
    seed: u32,
) -> (Vec<Compound>, Vec<TreeInstance>) {
    let mut compounds = Vec::new();
    let mut trees = Vec::new();
    let mut tree_id = 0_u32;
    for parcel in parcels {
        let centre = centroid(&parcel.ring);
        if parcel.compound {
            let courtyard = scale_polygon(&parcel.ring, centre, 0.50);
            // The gate fronts the widest adjacent street (see Parcel::gate_edge)
            // and spans six metres across the sidewalk.
            let edge = parcel.gate_edge as usize % parcel.ring.len().max(1);
            let gate_points = if parcel.ring.len() >= 2 {
                let a = parcel.ring[edge];
                let b = parcel.ring[(edge + 1) % parcel.ring.len()];
                let mid = Point {
                    x_km: (a.x_km + b.x_km) * 0.5,
                    y_km: (a.y_km + b.y_km) * 0.5,
                };
                let dx = b.x_km - a.x_km;
                let dy = b.y_km - a.y_km;
                let length = (dx * dx + dy * dy).sqrt();
                let (nx, ny) = if length > 1e-9 {
                    (-dy / length * 0.003, dx / length * 0.003)
                } else {
                    (0.0, 0.0)
                };
                vec![
                    Point {
                        x_km: mid.x_km + nx,
                        y_km: mid.y_km + ny,
                    },
                    Point {
                        x_km: mid.x_km - nx,
                        y_km: mid.y_km - ny,
                    },
                ]
            } else {
                vec![centre]
            };
            // An internal loop drive circles the courtyard, the classic 小区
            // fire-lane layout the renderer draws as a light ribbon.
            let loop_road = scale_polygon(&parcel.ring, centre, 0.66);
            compounds.push(Compound {
                id: compounds.len() as u32,
                parcel_id: parcel.id,
                boundary: parcel.ring.clone(),
                courtyard: Some(courtyard),
                gate_points,
                paths: vec![loop_road],
                road_width_metres: 5.0,
                fence_height_metres: 1.8,
                planted_ratio: 0.24 + modern_hash(seed, parcel.id as i32, 0, 909) * 0.28,
            });
        }
        let tree_count = match parcel.use_type {
            ParcelUse::Park => 8,
            ParcelUse::Residential if parcel.compound => 6,
            ParcelUse::Residential => 3,
            ParcelUse::MixedUse | ParcelUse::Commercial => 2,
            ParcelUse::Civic => 4,
            ParcelUse::Villa => 4,
            ParcelUse::Farmstead => 3,
            // Not a tree between the houses of a village; a few along a factory fence.
            ParcelUse::Village => 0,
            ParcelUse::Industrial => 3,
        };
        if parcel.ring.len() < 2 {
            continue;
        }
        for i in 0..tree_count {
            let edge = (i as usize) % parcel.ring.len();
            let a = parcel.ring[edge];
            let b = parcel.ring[(edge + 1) % parcel.ring.len()];
            let t = 0.22 + modern_hash(seed, parcel.id as i32, i, 917) * 0.56;
            let point = Point {
                x_km: a.x_km + (b.x_km - a.x_km) * t,
                y_km: a.y_km + (b.y_km - a.y_km) * t,
            };
            let species = match parcel.use_type {
                ParcelUse::Park => {
                    if i % 3 == 0 {
                        TreeSpecies::Willow
                    } else {
                        TreeSpecies::Ginkgo
                    }
                }
                ParcelUse::Residential => {
                    if i % 2 == 0 {
                        TreeSpecies::ChinesePlane
                    } else {
                        TreeSpecies::Ginkgo
                    }
                }
                _ => TreeSpecies::ChinesePlane,
            };
            let variation = 0.86 + modern_hash(seed, parcel.id as i32, i, 923) * 0.30;
            let (height, crown, trunk) = match species {
                TreeSpecies::ChinesePlane => (18.0, 6.0, 0.38),
                TreeSpecies::Ginkgo => (15.0, 4.8, 0.30),
                TreeSpecies::Cedar => (22.0, 4.2, 0.35),
                TreeSpecies::Bamboo => (8.0, 1.5, 0.08),
                TreeSpecies::Willow => (13.0, 5.6, 0.26),
            };
            // Two L-System variants exist per species; the hash keeps a tree
            // stable in its variant while the avenue alternates naturally.
            let variant = (modern_hash(seed, parcel.id as i32, i, 929) * 2.0).floor() as u16;
            trees.push(TreeInstance {
                id: tree_id,
                point,
                species,
                variant: variant.min(1),
                height_metres: height * variation,
                crown_radius_metres: crown * variation,
                trunk_radius_metres: trunk * variation,
            });
            tree_id += 1;
        }
    }
    (compounds, trees)
}

/// Trees on the town's open ground, the blocks no lot was cut from: a verge, land
/// not yet built on, the green belt along an expressway. Such ground in a Chinese
/// town is seldom bare lawn: a line of trees stands back from each street that
/// bounds it, and the rest is scrub with clumps of self-sown or nursery trees.
/// Nothing is planted in a field.
pub(super) fn plant_open_ground(
    blocks: &[crate::UrbanBlock],
    parcels: &[Parcel],
    fields: &[crate::Field],
    seed: u32,
    first_id: u32,
) -> Vec<TreeInstance> {
    // Back from the centreline of the street, clear of its carriageway and pavement.
    const SETBACK_KM: f32 = 0.016;
    const ROW_SPACING_KM: f32 = 0.011;
    // About one clump to every 0.4 ha, of three to seven trees.
    const CLUMP_AREA_KM2: f32 = 0.004;
    const CLUMP_RADIUS_KM: f32 = 0.011;
    const MAX_CLUMPS: usize = 60;
    let built: std::collections::HashSet<u32> = parcels.iter().map(|p| p.block_id).collect();
    let mut trees = Vec::new();
    let mut id = first_id;
    for (index, block) in blocks.iter().enumerate() {
        let ring = &block.boundary;
        if built.contains(&(index as u32)) || ring.len() < 3 {
            continue;
        }
        let key = index as i32;
        let fits = |p: Point| {
            inside(ring, p)
                && distance_to_ring(ring, p) >= SETBACK_KM * 0.9
                && !fields.iter().any(|f| inside(&f.ring, p))
        };
        let mut plant = |trees: &mut Vec<TreeInstance>, point: Point, salt: i32| {
            let h = |s: i32| modern_hash(seed, key, salt, s);
            let (species, height, crown, trunk) = match (h(941) * 4.0) as u32 {
                0 => (TreeSpecies::Willow, 12.0, 5.2, 0.26),
                1 => (TreeSpecies::Ginkgo, 13.0, 4.2, 0.28),
                _ => (TreeSpecies::ChinesePlane, 15.0, 5.4, 0.34),
            };
            let variation = 0.7 + h(943) * 0.5;
            trees.push(TreeInstance {
                id,
                point,
                species,
                variant: (h(947) * 2.0) as u16 % 2,
                height_metres: height * variation,
                crown_radius_metres: crown * variation,
                trunk_radius_metres: trunk * variation,
            });
            id += 1;
        };
        // The row along each street, set back and broken where a gap was left.
        let inward = if signed_area(ring) > 0.0 { 1.0 } else { -1.0 };
        let mut salt = 0;
        for e in 0..ring.len() {
            let a = ring[e];
            let b = ring[(e + 1) % ring.len()];
            let (dx, dy) = (b.x_km - a.x_km, b.y_km - a.y_km);
            let len = (dx * dx + dy * dy).sqrt();
            if len < SETBACK_KM * 3.0 {
                continue;
            }
            let (nx, ny) = (-dy / len * inward, dx / len * inward);
            let count = (len / ROW_SPACING_KM) as i32;
            for i in 1..count {
                salt += 1;
                if modern_hash(seed, key, salt, 951) < 0.18 {
                    continue;
                }
                let t = i as f32 / count as f32;
                let p = Point {
                    x_km: a.x_km + dx * t + nx * SETBACK_KM,
                    y_km: a.y_km + dy * t + ny * SETBACK_KM,
                };
                if fits(p) {
                    plant(&mut trees, p, salt);
                }
            }
        }
        // Clumps over the rest, scattered by rejection inside the block.
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for p in ring {
            x0 = x0.min(p.x_km);
            y0 = y0.min(p.y_km);
            x1 = x1.max(p.x_km);
            y1 = y1.max(p.y_km);
        }
        let clumps = ((signed_area(ring).abs() / CLUMP_AREA_KM2) as usize).min(MAX_CLUMPS);
        for c in 0..clumps as i32 {
            let h = |s: i32| modern_hash(seed, key, c, s);
            let centre = Point {
                x_km: x0 + (x1 - x0) * h(953),
                y_km: y0 + (y1 - y0) * h(957),
            };
            if !fits(centre) {
                continue;
            }
            let size = 3 + (h(959) * 5.0) as i32;
            for k in 0..size {
                let angle = h(961 + k) * std::f32::consts::TAU;
                let r = CLUMP_RADIUS_KM * h(971 + k).sqrt();
                let p = Point {
                    x_km: centre.x_km + angle.cos() * r,
                    y_km: centre.y_km + angle.sin() * r,
                };
                if fits(p) {
                    salt += 1;
                    plant(&mut trees, p, salt);
                }
            }
        }
    }
    trees
}

fn signed_area(ring: &[Point]) -> f32 {
    let mut sum = 0.0;
    for i in 0..ring.len() {
        let a = ring[i];
        let b = ring[(i + 1) % ring.len()];
        sum += a.x_km * b.y_km - b.x_km * a.y_km;
    }
    sum * 0.5
}

fn inside(ring: &[Point], p: Point) -> bool {
    let mut odd = false;
    let mut j = ring.len() - 1;
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[j]);
        if (a.y_km > p.y_km) != (b.y_km > p.y_km)
            && p.x_km < (b.x_km - a.x_km) * (p.y_km - a.y_km) / (b.y_km - a.y_km) + a.x_km
        {
            odd = !odd;
        }
        j = i;
    }
    odd
}

fn distance_to_ring(ring: &[Point], p: Point) -> f32 {
    let mut best = f32::MAX;
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        let (dx, dy) = (b.x_km - a.x_km, b.y_km - a.y_km);
        let l2 = dx * dx + dy * dy;
        let t = if l2 > 0.0 {
            (((p.x_km - a.x_km) * dx + (p.y_km - a.y_km) * dy) / l2).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let (ex, ey) = (a.x_km + dx * t - p.x_km, a.y_km + dy * t - p.y_km);
        best = best.min((ex * ex + ey * ey).sqrt());
    }
    best
}
