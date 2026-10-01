//! Roads that are given, not generated.
//!
//! A world planner may already know where its long roads go: the ones that join
//! one town to the next, routed over the terrain. A roads layer lays those down as
//! they are and generates the rest of the network around them, so the long road
//! through a town is the same road as the street on its far side.

use std::collections::HashMap;
use std::sync::Arc;

use crate::geom::{Polyline, V2};
use crate::road::RoadClass;

#[derive(Debug, Clone, PartialEq)]
pub struct PinnedRoad {
    /// Stable identity; every piece the road is cut into derives its id from this.
    pub id: u64,
    pub class: RoadClass,
    /// The centreline, in metres, from one end to the other.
    pub path: Polyline,
}

/// A source of given roads, queried by window.
pub trait PinnedRoads: Send + Sync {
    /// The roads whose bounding box meets the rectangle `min`..`max`.
    fn within(&self, min: V2, max: V2) -> Vec<Arc<PinnedRoad>>;
}

/// No given roads.
#[derive(Debug, Clone, Copy)]
pub struct NoPinnedRoads;

impl PinnedRoads for NoPinnedRoads {
    fn within(&self, _: V2, _: V2) -> Vec<Arc<PinnedRoad>> {
        Vec::new()
    }
}

/// A fixed set of given roads, indexed on a coarse grid.
pub struct PinnedSet {
    roads: Vec<Arc<PinnedRoad>>,
    grid: HashMap<(i64, i64), Vec<usize>>,
    cell_m: f64,
}

impl PinnedSet {
    pub fn new(roads: Vec<PinnedRoad>) -> PinnedSet {
        let cell_m = 1000.0;
        let roads: Vec<Arc<PinnedRoad>> = roads.into_iter().map(Arc::new).collect();
        let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for (k, road) in roads.iter().enumerate() {
            // Index every cell the road's segments touch, not its whole bounding
            // box: a long diagonal road would otherwise sit in thousands of cells.
            for w in road.path.0.windows(2) {
                let steps = (w[0].dist(w[1]) / (cell_m * 0.5)).ceil().max(1.0) as usize;
                for s in 0..=steps {
                    let p = w[0].lerp(w[1], s as f64 / steps as f64);
                    let key = ((p.x / cell_m).floor() as i64, (p.y / cell_m).floor() as i64);
                    let slot = grid.entry(key).or_default();
                    if slot.last() != Some(&k) {
                        slot.push(k);
                    }
                }
            }
        }
        for slot in grid.values_mut() {
            slot.sort_unstable();
            slot.dedup();
        }
        PinnedSet { roads, grid, cell_m }
    }

    pub fn len(&self) -> usize {
        self.roads.len()
    }

    pub fn is_empty(&self) -> bool {
        self.roads.is_empty()
    }
}

impl PinnedRoads for PinnedSet {
    fn within(&self, min: V2, max: V2) -> Vec<Arc<PinnedRoad>> {
        let (x0, x1) = ((min.x / self.cell_m).floor() as i64, (max.x / self.cell_m).floor() as i64);
        let (y0, y1) = ((min.y / self.cell_m).floor() as i64, (max.y / self.cell_m).floor() as i64);
        let mut found: Vec<usize> = Vec::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                if let Some(slot) = self.grid.get(&(x, y)) {
                    found.extend(slot);
                }
            }
        }
        found.sort_unstable();
        found.dedup();
        found
            .into_iter()
            .map(|k| self.roads[k].clone())
            .filter(|r| r.path.bounds().is_some_and(|(lo, hi)| lo.x <= max.x && hi.x >= min.x && lo.y <= max.y && hi.y >= min.y))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::v2;

    fn road(id: u64, pts: &[(f64, f64)]) -> PinnedRoad {
        PinnedRoad { id, class: RoadClass::Arterial, path: Polyline(pts.iter().map(|p| v2(p.0, p.1)).collect()) }
    }

    #[test]
    fn a_window_finds_the_roads_that_touch_it_and_no_others() {
        let set = PinnedSet::new(vec![
            road(1, &[(0.0, 0.0), (10_000.0, 10_000.0)]),
            road(2, &[(0.0, 9_000.0), (1_000.0, 9_000.0)]),
            road(3, &[(50_000.0, 50_000.0), (51_000.0, 50_000.0)]),
        ]);
        let ids = |min: V2, max: V2| -> Vec<u64> { set.within(min, max).iter().map(|r| r.id).collect() };
        assert_eq!(ids(v2(4_500.0, 4_500.0), v2(5_500.0, 5_500.0)), vec![1]);
        assert_eq!(ids(v2(0.0, 8_500.0), v2(500.0, 9_500.0)), vec![2]);
        assert!(ids(v2(2_000.0, 8_000.0), v2(2_500.0, 8_500.0)).is_empty());
        assert_eq!(ids(v2(-1e6, -1e6), v2(1e6, 1e6)), vec![1, 2, 3]);
        assert_eq!(set.len(), 3);
        assert!(NoPinnedRoads.within(v2(0.0, 0.0), v2(1.0, 1.0)).is_empty());
    }
}
