//! The climate and landform a town sits in, as far as its vegetation cares.
//!
//! A town's trees, hedges, lawns and wild margins are not a fixed list: they
//! follow the terrain simulation the town was placed on. This is the small
//! descriptor the planner hands to the scene builder, sampled from the terrain
//! fields around the town centre (see `infrastructure::climate`).

use serde::{Deserialize, Serialize};

/// Climate and landform at the town centre.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteClimate {
    /// Mean annual temperature, degrees Celsius (terrain `temperature`).
    pub temperature_c: f32,
    /// Climate wetness 0 (arid) to 1 (humid) (terrain `moisture`).
    pub moisture: f32,
    /// Ground height above the sea datum, metres (terrain `height`).
    pub elevation_m: f32,
    /// Snow cover 0..1 (terrain `snow`), the persistent-snow signal.
    pub snow: f32,
    /// Distance to the nearest open sea or large lake, kilometres. A large value
    /// means inland.
    pub coast_km: f32,
}

impl Default for SiteClimate {
    /// A warm-temperate lowland a long way from the sea: the setting the
    /// planting lists were first written for, so a plan made without terrain
    /// plants what it always planted.
    fn default() -> Self {
        Self {
            temperature_c: 15.5,
            moisture: 0.5,
            elevation_m: 60.0,
            snow: 0.0,
            coast_km: 500.0,
        }
    }
}

impl SiteClimate {
    /// 0 (humid) to 1 (arid).
    pub fn aridity(&self) -> f32 {
        (1.0 - self.moisture).clamp(0.0, 1.0)
    }
}
