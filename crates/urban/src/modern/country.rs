//! What the land says about the country around a town.
//!
//! Farmland is not laid on a fixed pattern. Where a terrain simulation is
//! available it is sampled at each plot: steep or soil-less ground is left to
//! scrub, moderate slopes are terraced along the contours, dry or bare ground
//! carries only sparse fallow, and the fertile plain gets crops chosen by
//! moisture and temperature, rows following the road, and shelterbelts of trees
//! along the field edges and the road. Without a sampler none of this applies
//! and the older, uniform behaviour is kept.

use super::CityFrame;
use super::graph::modern_hash;
use super::suburb::{Zone, zone};
use crate::{CropKind, Point, TreeInstance, TreeSpecies};

/// The terrain at one place, as the planner needs it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CountrySample {
    /// Rise over run of the ground.
    pub slope: f32,
    /// Direction of steepest descent in the world frame, radians.
    pub downhill: f32,
    /// Climate wetness 0 (arid) to 1 (humid).
    pub moisture: f32,
    pub temperature_c: f32,
    /// Soil depth 0..1.
    pub soil: f32,
    /// Share of bare ground 0..1.
    pub bare: f32,
    /// Flood-plain share 0..1.
    pub floodplain: f32,
}

/// A terrain sampler over world kilometres.
pub type CountryFn = std::sync::Arc<dyn Fn(Point) -> CountrySample + Send + Sync>;

/// Slope above which nothing is farmed.
pub const STEEP: f32 = 0.34;
/// Slope above which farmland is terraced along the contours.
pub const TERRACE: f32 = 0.09;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Land {
    /// Too steep or too thin of soil to farm.
    Barren,
    /// Dry or bare: sparse fallow only.
    Arid,
    /// A slope worth terracing.
    Terrace,
    /// Flat, fertile ground.
    Plain,
}

pub(super) fn classify(s: &CountrySample) -> Land {
    if s.slope > STEEP || s.soil < 0.08 {
        Land::Barren
    } else if s.moisture < 0.25 || s.bare > 0.6 {
        Land::Arid
    } else if s.slope > TERRACE {
        Land::Terrace
    } else {
        Land::Plain
    }
}

/// Whether shelterbelts are planted here: the fertile, watered plain.
pub(super) fn belts_here(s: &CountrySample) -> bool {
    classify(s) == Land::Plain && s.moisture > 0.3 && s.soil > 0.25
}

/// What a plot is, as the land decides it.
pub(super) struct Plot {
    pub crop: CropKind,
    /// Row direction in the city's local frame.
    pub row_angle: f32,
    pub terraced: bool,
}

/// Decide crop and row direction of a plot at local point `c`, or `None` if the
/// land will not take a field. `choices` and `pick` are the zone's old table and
/// the plot's preference (0..1), used when there is no terrain; `road_angle` is
/// the direction rows follow on flat ground.
pub(super) fn plan_plot(
    frame: &CityFrame,
    c: (f32, f32),
    choices: &[CropKind],
    pick: f32,
    road_angle: f32,
) -> Option<Plot> {
    let old = choices[((pick * choices.len() as f32) as usize).min(choices.len() - 1)];
    let Some(s) = frame.country_at(c.0, c.1) else {
        return Some(Plot {
            crop: old,
            row_angle: road_angle,
            terraced: false,
        });
    };
    let land = classify(&s);
    // Rows follow the contour (across the fall line) on a slope.
    let contour = s.downhill + std::f32::consts::FRAC_PI_2 - frame.spec.rotation_radians;
    let (crop, row_angle, terraced) = match land {
        Land::Barren => return None,
        // Thin fallow patches, and only now and then.
        Land::Arid => {
            if pick > 0.35 {
                return None;
            }
            (CropKind::Fallow, road_angle, false)
        }
        Land::Terrace => {
            let wet_warm = s.moisture > 0.5 && s.temperature_c > 14.0;
            let crop = if wet_warm && pick < 0.6 {
                CropKind::Rice
            } else if pick < 0.5 {
                CropKind::Orchard
            } else if s.temperature_c > 11.0 {
                CropKind::Corn
            } else {
                CropKind::Wheat
            };
            (crop, contour, true)
        }
        Land::Plain => {
            let near_town = zone(frame.urbanness(c.0, c.1)) != Zone::Farm;
            let wet = s.moisture > 0.55 || s.floodplain > 0.3;
            let crop = if near_town {
                old
            } else if wet && s.temperature_c > 12.0 {
                if pick < 0.7 {
                    CropKind::Rice
                } else {
                    CropKind::Vegetables
                }
            } else if s.temperature_c < 8.0 {
                if pick < 0.6 {
                    CropKind::Wheat
                } else {
                    CropKind::Rapeseed
                }
            } else if s.moisture < 0.4 {
                if pick < 0.6 {
                    CropKind::Wheat
                } else {
                    CropKind::Corn
                }
            } else if pick < 0.4 {
                CropKind::Corn
            } else if pick < 0.7 {
                CropKind::Wheat
            } else if pick < 0.9 {
                CropKind::Rapeseed
            } else {
                CropKind::Vegetables
            };
            (crop, road_angle, false)
        }
    };
    Some(Plot {
        crop,
        row_angle,
        terraced,
    })
}

/// A shelterbelt tree: poplar-like rows of plane trees, willows by the water,
/// cedars in the cold.
pub(super) fn belt_tree(
    frame: &CityFrame,
    p: (f32, f32),
    s: &CountrySample,
    seed: u32,
    key: i32,
) -> TreeInstance {
    let species = if s.floodplain > 0.3 {
        TreeSpecies::Willow
    } else if s.temperature_c < 7.0 {
        TreeSpecies::Cedar
    } else {
        TreeSpecies::ChinesePlane
    };
    let variation = 0.85 + modern_hash(seed, key, 3, 1403) * 0.3;
    let (h, crown, trunk) = match species {
        TreeSpecies::Willow => (13.0, 5.6, 0.26),
        TreeSpecies::Cedar => (22.0, 4.2, 0.35),
        _ => (18.0, 6.0, 0.38),
    };
    TreeInstance {
        id: 0,
        point: frame.to_world(p.0, p.1),
        species,
        variant: (modern_hash(seed, key, 5, 1405) * 2.0).floor().min(1.0) as u16,
        height_metres: h * variation,
        crown_radius_metres: crown * variation,
        trunk_radius_metres: trunk * variation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CountrySample {
        CountrySample {
            slope: 0.01,
            downhill: 0.0,
            moisture: 0.7,
            temperature_c: 16.0,
            soil: 0.7,
            bare: 0.0,
            floodplain: 0.0,
        }
    }

    #[test]
    fn the_land_decides_what_is_farmed() {
        assert_eq!(classify(&sample()), Land::Plain);
        assert_eq!(
            classify(&CountrySample {
                slope: 0.2,
                ..sample()
            }),
            Land::Terrace
        );
        assert_eq!(
            classify(&CountrySample {
                slope: 0.5,
                ..sample()
            }),
            Land::Barren
        );
        assert_eq!(
            classify(&CountrySample {
                moisture: 0.1,
                ..sample()
            }),
            Land::Arid
        );
        assert_eq!(
            classify(&CountrySample {
                soil: 0.02,
                ..sample()
            }),
            Land::Barren
        );
        assert!(belts_here(&sample()));
        assert!(!belts_here(&CountrySample {
            slope: 0.2,
            ..sample()
        }));
    }
}
