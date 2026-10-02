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
        };
        if parcel.ring.len() < 2 {
            continue;
        }
        for i in 0..tree_count {
            let edge = (i as usize) % parcel.ring.len();
            let a = parcel.ring[edge];
            let b = parcel.ring[(edge + 1) % parcel.ring.len()];
            let t = 0.22 + modern_hash(seed, parcel.id as i32, i as i32, 917) * 0.56;
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
            let variation = 0.86 + modern_hash(seed, parcel.id as i32, i as i32, 923) * 0.30;
            let (height, crown, trunk) = match species {
                TreeSpecies::ChinesePlane => (18.0, 6.0, 0.38),
                TreeSpecies::Ginkgo => (15.0, 4.8, 0.30),
                TreeSpecies::Cedar => (22.0, 4.2, 0.35),
                TreeSpecies::Bamboo => (8.0, 1.5, 0.08),
                TreeSpecies::Willow => (13.0, 5.6, 0.26),
            };
            // Two L-System variants exist per species; the hash keeps a tree
            // stable in its variant while the avenue alternates naturally.
            let variant = (modern_hash(seed, parcel.id as i32, i as i32, 929) * 2.0).floor() as u16;
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
