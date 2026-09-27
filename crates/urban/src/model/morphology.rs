//! Ported from the source city kernel: statistics used to calibrate and
//! evaluate generated road graphs, retargeted from `CityDocument` to the
//! crate's `ModernCity`.  The default prior encodes a real mainland-Chinese
//! street network: kilometre-scale arterial spacing, a strong two-orientation
//! grid, low circuity and roughly one big road in five.

use serde::{Deserialize, Serialize};

use crate::model::{HdRoad, ModernCity, ModernRoadClass, SdNode, SdRoad};

/// Minimal graph view the morphology scorer needs.  Implemented by the owned
/// `ModernCity` and by borrow probes used while a city is still being built.
pub trait CityGraph {
    fn nodes(&self) -> &[SdNode];
    fn sd_roads(&self) -> &[SdRoad];
    fn hd_roads(&self) -> &[HdRoad];
}

impl CityGraph for ModernCity {
    fn nodes(&self) -> &[SdNode] {
        &self.nodes
    }
    fn sd_roads(&self) -> &[SdRoad] {
        &self.sd_roads
    }
    fn hd_roads(&self) -> &[HdRoad] {
        &self.hd_roads
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorphologyStats {
    pub mean_segment_m: f64,
    pub segment_cv: f64,
    pub mean_degree: f64,
    pub arterial_ratio: f64,
    pub expressway_ratio: f64,
    pub orientation_entropy: f64,
    pub circuity: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorphologyPrior {
    pub mean_segment_m: f64,
    pub segment_cv: f64,
    pub mean_degree: f64,
    pub arterial_ratio: f64,
    pub expressway_ratio: f64,
    pub orientation_entropy: f64,
    pub circuity: f64,
}

impl Default for MorphologyPrior {
    fn default() -> Self {
        Self {
            // 支路 spacing in the generator's base grid; the mean segment sits
            // just above one cell because river gaps remove short pieces.
            mean_segment_m: 132.0,
            segment_cv: 0.16,
            // Grid interiors are degree 4; boundary nodes pull the mean down.
            mean_degree: 3.55,
            arterial_ratio: 0.22,
            expressway_ratio: 0.06,
            // A real Chinese plan concentrates bearings into two bins, so the
            // normalized orientation entropy stays far below organic fabrics.
            orientation_entropy: 0.32,
            circuity: 1.04,
        }
    }
}

impl MorphologyPrior {
    pub fn score(&self, city: &impl CityGraph) -> f64 {
        let s = measure(city);
        let z = |a: f64, b: f64, scale: f64| ((a - b) / scale).powi(2);
        z(
            s.mean_segment_m,
            self.mean_segment_m,
            self.mean_segment_m.max(1.0) * 0.45,
        ) + z(s.segment_cv, self.segment_cv, 0.12)
            + z(s.mean_degree, self.mean_degree, 0.55)
            + z(s.arterial_ratio, self.arterial_ratio, 0.10)
            + z(s.expressway_ratio, self.expressway_ratio, 0.045)
            + z(s.orientation_entropy, self.orientation_entropy, 0.20)
            + z(s.circuity, self.circuity, 0.10)
    }
}

pub fn measure(city: &impl CityGraph) -> MorphologyStats {
    let nodes: std::collections::HashMap<u32, crate::Point> = city
        .nodes()
        .iter()
        .map(|node| (node.id, node.point))
        .collect();
    // Circuity follows the source definition: polyline length of the HD
    // centreline over the straight distance between its endpoints.  The grid
    // keeps it at 1.0; curved organic variants drift upward.
    let centreline_by_road: std::collections::HashMap<u32, &[crate::Point]> = city
        .hd_roads()
        .iter()
        .map(|road| (road.id, road.centreline.as_slice()))
        .collect();
    let mut lengths = Vec::new();
    let mut bearings = Vec::new();
    let mut degree = std::collections::HashMap::<u32, usize>::new();
    let mut circuity_sum = 0.0;
    let mut circuity_count = 0.0;
    for road in city.sd_roads() {
        let (Some(a), Some(b)) = (
            nodes.get(&road.from),
            nodes.get(&road.to),
        ) else {
            continue;
        };
        let dx = (b.x_km - a.x_km) as f64 * 1000.0;
        let dy = (b.y_km - a.y_km) as f64 * 1000.0;
        let direct = dx.hypot(dy);
        if direct <= 0.0 {
            continue;
        }
        let polyline: f64 = centreline_by_road
            .get(&road.id)
            .map(|points| {
                points
                    .windows(2)
                    .map(|pair| {
                        ((pair[1].x_km - pair[0].x_km) as f64 * 1000.0).hypot(
                            (pair[1].y_km - pair[0].y_km) as f64 * 1000.0,
                        )
                    })
                    .sum()
            })
            .filter(|length| *length > 0.0)
            .unwrap_or(direct);
        circuity_sum += polyline / direct;
        circuity_count += 1.0;
        lengths.push(direct);
        bearings.push(dy.atan2(dx).rem_euclid(std::f64::consts::PI));
        *degree.entry(road.from).or_default() += 1;
        *degree.entry(road.to).or_default() += 1;
    }
    let mean = lengths.iter().sum::<f64>() / lengths.len().max(1) as f64;
    let variance = lengths
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>()
        / lengths.len().max(1) as f64;
    let bins = 12;
    let mut histogram = vec![0.0_f64; bins];
    for bearing in bearings {
        histogram[((bearing / std::f64::consts::PI) * bins as f64) as usize % bins] += 1.0;
    }
    let total = histogram.iter().sum::<f64>().max(1.0);
    let entropy = -histogram
        .iter()
        .filter(|x| **x > 0.0)
        .map(|x| {
            let p = x / total;
            p * p.ln()
        })
        .sum::<f64>()
        / (bins as f64).ln();
    MorphologyStats {
        mean_segment_m: mean,
        segment_cv: variance.sqrt() / mean.max(1.0),
        mean_degree: degree.values().sum::<usize>() as f64 / degree.len().max(1) as f64,
        arterial_ratio: share(city.sd_roads(), |class| {
            matches!(class, ModernRoadClass::Arterial)
        }),
        expressway_ratio: share(city.sd_roads(), |class| {
            matches!(class, ModernRoadClass::Expressway)
        }),
        orientation_entropy: entropy,
        circuity: if circuity_count > 0.0 {
            circuity_sum / circuity_count
        } else {
            1.0
        },
    }
}

fn share(
    roads: &[crate::SdRoad],
    predicate: impl Fn(ModernRoadClass) -> bool,
) -> f64 {
    let big = roads
        .iter()
        .filter(|road| predicate(road.class))
        .count() as f64;
    big / roads.len().max(1) as f64
}
