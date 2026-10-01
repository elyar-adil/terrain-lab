//! Continuous fields a layer may ask about: ground height and water.
//!
//! These are interfaces, not implementations. The roads layer asks "how steep is
//! it here?" and "is there a river between these two points?" and does not care
//! whether the answer comes from this project's eroded terrain, a hand-painted map
//! or a real elevation model. Anything that can answer can feed it; the two toy
//! implementations below are what make a layer usable on its own.

use crate::geom::{V2, closest_on_segment, v2};

/// Ground height above datum, in metres.
pub trait HeightField: Send + Sync {
    fn height_m(&self, p: V2) -> f64;

    /// Rise over run, by central differences over `step_m`.
    fn slope(&self, p: V2, step_m: f64) -> f64 {
        let h = step_m;
        let dx = self.height_m(p + v2(h, 0.0)) - self.height_m(p - v2(h, 0.0));
        let dy = self.height_m(p + v2(0.0, h)) - self.height_m(p - v2(0.0, h));
        (dx * dx + dy * dy).sqrt() / (2.0 * h)
    }
}

/// A river, lake or sea nearby.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterHit {
    /// Distance from the query point to the nearest water edge; zero or negative inside.
    pub distance_m: f64,
    /// Width of the water across the nearest point, metres.
    pub width_m: f64,
    /// Stable identity of the water body, for naming and for bridges that must agree.
    pub id: u64,
}

pub trait WaterField: Send + Sync {
    /// The nearest water within `within_m` of `p`, if any.
    fn nearest(&self, p: V2, within_m: f64) -> Option<WaterHit>;

    fn is_water(&self, p: V2) -> bool {
        self.nearest(p, 0.0).is_some_and(|h| h.distance_m <= 0.0)
    }

    /// Does the straight line `a`→`b` cross water? Returns the span of water crossed.
    fn crossing(&self, a: V2, b: V2, probe_step_m: f64) -> Option<f64> {
        let l = a.dist(b);
        let n = (l / probe_step_m).ceil().max(1.0) as usize;
        let wet = (0..=n).filter(|i| self.is_water(a.lerp(b, *i as f64 / n as f64))).count();
        (wet > 0).then(|| wet as f64 * l / n as f64)
    }
}

/// How built-up a place is, from 0 (open country) to 1 (the dense core of a city).
///
/// One number is deliberately all the roads layer needs to decide how fine its
/// street grid is and whether a road is a country road or a city street. A
/// settlement layer, a land-use map or a hand-painted mask can all supply it.
pub trait UrbanField: Send + Sync {
    fn urbanness(&self, p: V2) -> f64;
}

/// The same everywhere.
#[derive(Debug, Clone, Copy)]
pub struct ConstantUrban(pub f64);

impl UrbanField for ConstantUrban {
    fn urbanness(&self, _: V2) -> f64 {
        self.0
    }
}

/// Flat ground at a fixed height. The terrain a standalone consumer uses.
#[derive(Debug, Clone, Copy)]
pub struct FlatGround(pub f64);

impl HeightField for FlatGround {
    fn height_m(&self, _: V2) -> f64 {
        self.0
    }
}

/// No water anywhere.
#[derive(Debug, Clone, Copy)]
pub struct DryLand;

impl WaterField for DryLand {
    fn nearest(&self, _: V2, _: f64) -> Option<WaterHit> {
        None
    }
}

/// A river described by a centreline and a width: handy in tests, and the shape
/// an adapter over any vector hydrography takes.
#[derive(Debug, Clone)]
pub struct PolylineRiver {
    pub id: u64,
    pub line: Vec<V2>,
    pub width_m: f64,
}

impl WaterField for PolylineRiver {
    fn nearest(&self, p: V2, within_m: f64) -> Option<WaterHit> {
        let centre = self
            .line
            .windows(2)
            .map(|s| closest_on_segment(s[0], s[1], p).2)
            .fold(f64::INFINITY, f64::min);
        let edge = centre - self.width_m * 0.5;
        (edge <= within_m).then_some(WaterHit { distance_m: edge, width_m: self.width_m, id: self.id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ramp;
    impl HeightField for Ramp {
        fn height_m(&self, p: V2) -> f64 {
            0.2 * p.x
        }
    }

    #[test]
    fn slope_is_rise_over_run() {
        assert!((Ramp.slope(v2(10.0, 5.0), 3.0) - 0.2).abs() < 1e-12);
        assert_eq!(FlatGround(12.0).slope(v2(0.0, 0.0), 3.0), 0.0);
        assert_eq!(FlatGround(12.0).height_m(v2(9.0, 9.0)), 12.0);
    }

    #[test]
    fn a_river_is_water_inside_its_banks_and_a_line_across_it_crosses_it() {
        let river = PolylineRiver { id: 7, line: vec![v2(0.0, -100.0), v2(0.0, 100.0)], width_m: 20.0 };
        assert!(river.is_water(v2(5.0, 0.0)));
        assert!(!river.is_water(v2(15.0, 0.0)));
        let hit = river.nearest(v2(30.0, 0.0), 50.0).unwrap();
        assert!((hit.distance_m - 20.0).abs() < 1e-9 && hit.id == 7);
        assert!(river.nearest(v2(30.0, 0.0), 10.0).is_none());
        let span = river.crossing(v2(-50.0, 0.0), v2(50.0, 0.0), 1.0).unwrap();
        assert!((span - 20.0).abs() < 2.0, "{span}");
        assert!(river.crossing(v2(30.0, 0.0), v2(80.0, 0.0), 1.0).is_none());
        assert!(DryLand.crossing(v2(0.0, 0.0), v2(100.0, 0.0), 1.0).is_none());
    }
}
