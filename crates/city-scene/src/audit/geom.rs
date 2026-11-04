//! Small geometry helpers for the plausibility audit: a bucket grid, ring
//! overlap, and a road index that answers "which carriageway is this point on,
//! and how far across it".

use std::collections::HashMap;

use crate::math::{Vec2, point_in_ring, point_segment_distance, signed_area};
use crate::network::{Junction, Network, Road};

/// Items bucketed by the cells their bounding box touches.
pub struct Grid {
    cell: f32,
    cells: HashMap<(i32, i32), Vec<u32>>,
}

impl Grid {
    pub fn new(cell: f32) -> Self {
        Self {
            cell,
            cells: HashMap::new(),
        }
    }

    fn key(&self, v: f32) -> i32 {
        (v / self.cell).floor() as i32
    }

    pub fn insert(&mut self, id: u32, min: Vec2, max: Vec2) {
        for i in self.key(min.x)..=self.key(max.x) {
            for j in self.key(min.y)..=self.key(max.y) {
                self.cells.entry((i, j)).or_default().push(id);
            }
        }
    }

    /// Ids whose cells touch the box, deduplicated and sorted.
    pub fn query(&self, min: Vec2, max: Vec2) -> Vec<u32> {
        let mut out = Vec::new();
        for i in self.key(min.x)..=self.key(max.x) {
            for j in self.key(min.y)..=self.key(max.y) {
                if let Some(ids) = self.cells.get(&(i, j)) {
                    out.extend_from_slice(ids);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}

pub fn bounds(ring: &[Vec2]) -> (Vec2, Vec2) {
    let mut lo = Vec2::new(f32::MAX, f32::MAX);
    let mut hi = Vec2::new(f32::MIN, f32::MIN);
    for p in ring {
        lo.x = lo.x.min(p.x);
        lo.y = lo.y.min(p.y);
        hi.x = hi.x.max(p.x);
        hi.y = hi.y.max(p.y);
    }
    (lo, hi)
}

pub fn centroid(ring: &[Vec2]) -> Vec2 {
    let n = ring.len().max(1) as f32;
    let s = ring.iter().fold(Vec2::new(0.0, 0.0), |a, p| a + *p);
    s * (1.0 / n)
}

/// Area shared by two rings, in square metres.
pub fn overlap_area(a: &[Vec2], b: &[Vec2]) -> f32 {
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::single::SingleFloatOverlay;
    if a.len() < 3 || b.len() < 3 {
        return 0.0;
    }
    let subject: Vec<[f32; 2]> = a.iter().map(|p| [p.x, p.y]).collect();
    let clip: Vec<[f32; 2]> = b.iter().map(|p| [p.x, p.y]).collect();
    let shapes = vec![subject].overlay(&vec![clip], OverlayRule::Intersect, FillRule::NonZero);
    let mut total = 0.0;
    for shape in shapes {
        for (i, contour) in shape.iter().enumerate() {
            let ring: Vec<Vec2> = contour.iter().map(|p| Vec2::new(p[0], p[1])).collect();
            let area = signed_area(&ring).abs();
            if i == 0 {
                total += area;
            } else {
                total -= area;
            }
        }
    }
    total.max(0.0)
}

/// Distance from a point to the closest edge of a ring.
pub fn distance_to_ring(point: Vec2, ring: &[Vec2]) -> f32 {
    let mut best = f32::MAX;
    for i in 0..ring.len() {
        best = best.min(point_segment_distance(
            point,
            ring[i],
            ring[(i + 1) % ring.len()],
        ));
    }
    best
}

/// Smallest gap between two rings that do not overlap, sampled along edges.
pub fn ring_gap(a: &[Vec2], b: &[Vec2]) -> f32 {
    let mut best = f32::MAX;
    for ring in [(a, b), (b, a)] {
        for p in ring.0 {
            best = best.min(distance_to_ring(*p, ring.1));
        }
    }
    best
}

/// Points along a ring's edges, `step` metres apart, corners included.
pub fn ring_samples(ring: &[Vec2], step: f32) -> Vec<Vec2> {
    let mut out = Vec::new();
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        let n = ((a.distance(b) / step).ceil() as usize).max(1);
        for k in 0..n {
            out.push(a.lerp(b, k as f32 / n as f32));
        }
    }
    out
}

/// Where a point stands relative to one road.
#[derive(Debug, Clone, Copy)]
pub struct OnRoad {
    pub road: usize,
    /// Plan distance to the carriageway centreline.
    pub distance: f32,
    /// Signed offset across the road, `+` on the left of travel.
    pub lateral: f32,
    /// Deck height at the nearest point.
    pub y: f32,
    pub station: f32,
}

/// Every road's carriageway segments in a grid.
pub struct RoadIndex<'a> {
    pub network: &'a Network,
    grid: Grid,
    segments: Vec<(u32, u32)>,
}

impl<'a> RoadIndex<'a> {
    pub fn new(network: &'a Network) -> Self {
        let mut grid = Grid::new(24.0);
        let mut segments = Vec::new();
        for (ri, road) in network.roads.iter().enumerate() {
            let pts = road.carriageway.points();
            for s in 0..pts.len().saturating_sub(1) {
                let a = Vec2::new(pts[s].x, pts[s].z);
                let b = Vec2::new(pts[s + 1].x, pts[s + 1].z);
                let reach = road.half_width() + road.section.sidewalk_metres + 6.0;
                let id = segments.len() as u32;
                segments.push((ri as u32, s as u32));
                grid.insert(
                    id,
                    Vec2::new(a.x.min(b.x) - reach, a.y.min(b.y) - reach),
                    Vec2::new(a.x.max(b.x) + reach, a.y.max(b.y) + reach),
                );
            }
        }
        Self {
            network,
            grid,
            segments,
        }
    }

    /// Every road within `reach` metres of `point`, nearest segment of each.
    pub fn near(&self, point: Vec2, reach: f32) -> Vec<OnRoad> {
        let mut best: HashMap<u32, OnRoad> = HashMap::new();
        let lo = Vec2::new(point.x - reach, point.y - reach);
        let hi = Vec2::new(point.x + reach, point.y + reach);
        for id in self.grid.query(lo, hi) {
            let (ri, si) = self.segments[id as usize];
            let road = &self.network.roads[ri as usize];
            let pts = road.carriageway.points();
            let (p, q) = (pts[si as usize], pts[si as usize + 1]);
            let a = Vec2::new(p.x, p.z);
            let b = Vec2::new(q.x, q.z);
            let ab = b - a;
            let len2 = ab.length_squared();
            let t = if len2 < 1.0e-6 {
                0.0
            } else {
                ((point - a).dot(ab) / len2).clamp(0.0, 1.0)
            };
            let foot = a + ab * t;
            let distance = point.distance(foot);
            if distance > reach {
                continue;
            }
            let side = if ab.cross(point - a) >= 0.0 { 1.0 } else { -1.0 };
            let seg_len = len2.sqrt();
            let entry = OnRoad {
                road: ri as usize,
                distance,
                lateral: side * distance,
                y: p.y + (q.y - p.y) * t,
                station: road.carriageway.stations()[si as usize] + seg_len * t,
            };
            match best.get(&ri) {
                Some(old) if old.distance <= distance => {}
                _ => {
                    best.insert(ri, entry);
                }
            }
        }
        let mut out: Vec<OnRoad> = best.into_values().collect();
        out.sort_by(|a, b| a.distance.total_cmp(&b.distance));
        out
    }
}

/// Junction boxes in a grid.
pub struct JunctionIndex<'a> {
    pub junctions: &'a [Junction],
    grid: Grid,
}

impl<'a> JunctionIndex<'a> {
    pub fn new(junctions: &'a [Junction]) -> Self {
        let mut grid = Grid::new(32.0);
        for (i, j) in junctions.iter().enumerate() {
            if j.ring.len() >= 3 {
                let (lo, hi) = bounds(&j.ring);
                grid.insert(i as u32, lo, hi);
            }
        }
        Self { junctions, grid }
    }

    pub fn containing(&self, p: Vec2) -> Option<&'a Junction> {
        self.grid
            .query(p, p)
            .into_iter()
            .map(|i| &self.junctions[i as usize])
            .find(|j| point_in_ring(p, &j.ring))
    }
}

/// Is the road at street level at this deck height?
pub fn at_grade(y: f32) -> bool {
    y < 1.0
}

/// Lateral reach of the motor lanes, excluding the median: `(inner, outer)`.
pub fn lane_band(road: &Road) -> (f32, f32) {
    let s = &road.section;
    (s.median_metres * 0.5 + 0.05, s.half_carriageway())
}
