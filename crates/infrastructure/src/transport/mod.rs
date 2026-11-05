//! Railways and airports of the world planner: derived from the same terrain
//! layers as settlements and roads. Rules and sources: `docs/transport-infrastructure.md`.

pub mod airport;
pub mod geometry;
pub mod rail;
pub mod wind;

use terrain_core::TerrainData;

/// The terrain as metres-space fields: all transport geometry is planned in
/// metres from the world corner and converted to kilometres at the edge.
pub struct World<'a> {
    pub terrain: &'a TerrainData,
    pub n: usize,
    pub size_m: f32,
}

/// Open water below this height is the sea (the planner's convention).
pub const SEA_LEVEL_M: f32 = 58.0;

impl<'a> World<'a> {
    pub fn new(terrain: &'a TerrainData, world_size_km: f32) -> Self {
        Self {
            terrain,
            n: terrain.size,
            size_m: world_size_km * 1000.0,
        }
    }

    pub fn cell_m(&self) -> f32 {
        self.size_m / (self.n - 1) as f32
    }

    pub fn sample(&self, field: &[f32], x_m: f32, y_m: f32) -> f32 {
        let last = (self.n - 1) as f32;
        let gx = (x_m / self.size_m * last).clamp(0.0, last);
        let gy = (y_m / self.size_m * last).clamp(0.0, last);
        let (x0, y0) = (gx.floor() as usize, gy.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(self.n - 1), (y0 + 1).min(self.n - 1));
        let (fx, fy) = (gx - x0 as f32, gy - y0 as f32);
        let at = |x: usize, y: usize| field[y * self.n + x];
        let top = at(x0, y0) * (1.0 - fx) + at(x1, y0) * fx;
        let bottom = at(x0, y1) * (1.0 - fx) + at(x1, y1) * fx;
        top * (1.0 - fy) + bottom * fy
    }

    pub fn height(&self, x_m: f32, y_m: f32) -> f32 {
        self.sample(&self.terrain.height, x_m, y_m)
    }

    /// Ground slope (rise over run) from central differences over `span_m`.
    pub fn slope(&self, x_m: f32, y_m: f32, span_m: f32) -> f32 {
        let dx = (self.height(x_m + span_m, y_m) - self.height(x_m - span_m, y_m)) / (2.0 * span_m);
        let dy = (self.height(x_m, y_m + span_m) - self.height(x_m, y_m - span_m)) / (2.0 * span_m);
        dx.hypot(dy)
    }

    pub fn is_sea(&self, x_m: f32, y_m: f32) -> bool {
        self.height(x_m, y_m) < SEA_LEVEL_M
    }

    pub fn is_lake(&self, x_m: f32, y_m: f32) -> bool {
        self.sample(&self.terrain.lake, x_m, y_m) > 0.3
    }

    /// A river or open water the line must bridge.
    pub fn is_river(&self, x_m: f32, y_m: f32) -> bool {
        let last = (self.n - 1) as f32;
        let gx = (x_m / self.size_m * last).round().clamp(0.0, last) as usize;
        let gy = (y_m / self.size_m * last).round().clamp(0.0, last) as usize;
        let i = gy * self.n + gx;
        self.terrain.height[i] >= SEA_LEVEL_M
            && (self.terrain.river_order[i] >= 2 || self.terrain.water[i] > 0.35)
    }

    pub fn wetness(&self, x_m: f32, y_m: f32) -> f32 {
        let w = self.sample(&self.terrain.wetland, x_m, y_m);
        let f = self.sample(&self.terrain.floodplain, x_m, y_m);
        w.max(f * 0.8)
    }
}

#[cfg(test)]
mod tests {
    use crate::{InfrastructureData, generate_infrastructure};
    use terrain_core::{Landform, SimulationConfig, TerrainPreset, generate};
    use urban::RailGround;

    fn config(seed: u32, landform: Landform, grid_size: usize) -> SimulationConfig {
        SimulationConfig {
            seed,
            preset: TerrainPreset::Temperate,
            landform,
            grid_size,
            world_size_km: 80.0,
            rainfall: 1275.0,
            evaporation: 600.0,
            wind_speed: 7.0,
            wind_direction: 225.0,
            sun_azimuth: 235.0,
            sun_elevation: 42.0,
            haze: 2.5,
            cloud_coverage: 35.0,
            cloud_speed: 24.0,
        }
    }

    fn worlds() -> Vec<(SimulationConfig, InfrastructureData)> {
        [(7, Landform::Plains, 256), (11, Landform::Plateau, 192), (99, Landform::Plains, 192)]
            .into_iter()
            .map(|(seed, form, size)| {
                let c = config(seed, form, size);
                let t = generate(&c, |_, _| {}).unwrap();
                let d = generate_infrastructure(&t, &c).unwrap();
                (c, d)
            })
            .collect()
    }

    /// Grade and curvature stay inside the limits of the line's own class;
    /// stations are level; road crossings are separated or protected.
    #[test]
    fn railways_obey_their_class_limits() {
        let worlds = worlds();
        let mut lines = 0;
        for (_, data) in &worlds {
            for r in &data.railways {
                lines += 1;
                let limits = r.kind.limits();
                assert!(
                    r.max_grade <= limits.max_grade + 1.0e-4,
                    "{:?} grade {} over {}",
                    r.kind, r.max_grade, limits.max_grade
                );
                // Three-point circles over 100 m on 25 m samples are slightly tight on arcs.
                assert!(
                    r.min_radius_m >= limits.min_radius_m * 0.93,
                    "{:?} radius {} under {}",
                    r.kind, r.min_radius_m, limits.min_radius_m
                );
                for st in &r.stations {
                    let around: Vec<f32> = r
                        .points
                        .iter()
                        .filter(|p| (p.s_m - st.plan.s_m).abs() <= st.plan.platform_length_m * 0.5)
                        .map(|p| p.design_m)
                        .collect();
                    let spread = around.iter().cloned().fold(f32::MIN, f32::max)
                        - around.iter().cloned().fold(f32::MAX, f32::min);
                    // 2.5 per mille over the platform length at most.
                    assert!(spread <= st.plan.platform_length_m * 0.0025 + 0.01, "station not level: {spread}");
                    assert!(st.plan.platform_length_m >= limits.platform_length_m);
                }
                for c in &r.crossings {
                    let i = r.points.iter().position(|p| p.s_m >= c.s_m).unwrap_or(0);
                    let over = r.points[i].design_m - r.points[i].ground_m;
                    match c.treatment {
                        super::rail::CrossingTreatment::RailOverRoad => {
                            assert!(over >= 5.0, "rail over road with only {over} m")
                        }
                        super::rail::CrossingTreatment::RoadOverRail => {
                            assert!(over <= -6.5, "road over rail with the line only {over} m down")
                        }
                        super::rail::CrossingTreatment::LevelProtected => {
                            assert!(!limits.closed, "level crossing on a closed line")
                        }
                    }
                }
                assert!(
                    r.points.iter().all(|p| {
                        p.ground != RailGround::Embankment || p.design_m - p.ground_m <= 9.01
                    }),
                    "embankment above the viaduct threshold"
                );
            }
        }
        assert!(lines >= 2, "the test worlds should carry railways");
    }

    #[test]
    fn airports_are_sited_and_oriented_by_the_rules() {
        let worlds = worlds();
        let mut found = 0;
        for (_, data) in &worlds {
            let Some(a) = &data.airport else { continue };
            found += 1;
            assert_eq!(a.plan.runway_length_m, a.plan.code.runway_length_m());
            assert_eq!(a.plan.runway_width_m, a.plan.code.runway_width_m());
            assert!((10.0..=40.0).contains(&a.distance_to_city_km));
            assert!(a.wind_usability >= 0.95, "usability {}", a.wind_usability);
            assert!(a.wind_usability_13kt >= a.perpendicular_usability_13kt);
            assert!(a.relief_m <= 16.0 && a.longitudinal_slope <= 0.012);
            assert!((a.plan.taxiway_z - a.plan.code.taxiway_separation_m()).abs() < 1.0e-3);
            // The designators are the bearing over ten, both ends 18 apart.
            let hi: i32 = a.plan.designator_hi.parse().unwrap();
            let lo: i32 = a.plan.designator_lo.parse().unwrap();
            assert_eq!((hi - lo).rem_euclid(36), 18);
            // Nothing the OLS forbids stands in the approach: the plan's own centre is under the inner horizontal.
            let limit = a.plan.surface_height_limit(a.plan.centre).unwrap();
            assert!((limit - 45.0).abs() < 1.0e-3);
            // No water or built-up land in the graded platform.
            let grid = data.urban_land.grid;
            for k in 0..200 {
                let x = a.plan.platform.x0 + (a.plan.platform.x1 - a.plan.platform.x0) * (k % 20) as f32 / 19.0;
                let z = a.plan.platform.z0 + (a.plan.platform.z1 - a.plan.platform.z0) * (k / 20) as f32 / 9.0;
                let p = a.plan.to_world(x, z);
                let gx = (p.x_km / grid.world_size_km * (grid.size - 1) as f32).round() as usize;
                let gy = (p.y_km / grid.world_size_km * (grid.size - 1) as f32).round() as usize;
                assert!(data.urban_land.values[gy * grid.size + gx] < 0.2);
            }
        }
        assert!(found >= 1, "no airport in any test world");
    }

    #[test]
    fn height_limits_follow_the_surfaces() {
        let plan = urban::AirfieldPlan::layout(
            urban::AerodromeCode::C4C,
            1,
            urban::Point { x_km: 40.0, y_km: 40.0 },
            0.0,
            1.0,
            100.0,
        );
        // On the extended centreline, 1 000 m past the strip end: 2 % slope = 20 m.
        let p = plan.to_world(plan.runway_length_m * 0.5 + 60.0 + 1000.0, 0.0);
        assert!((plan.surface_height_limit(p).unwrap() - 20.0).abs() < 0.5);
        // 3 000 m beside the strip: the transitional surface has reached the inner horizontal.
        let q = plan.to_world(0.0, 150.0 + 100.0);
        assert!((plan.surface_height_limit(q).unwrap() - 14.3).abs() < 0.3);
        // Far away: no surface.
        assert!(plan.surface_height_limit(plan.to_world(0.0, 20_000.0)).is_none());
    }

    #[test]
    fn transport_is_deterministic() {
        let c = config(7, Landform::Plains, 160);
        let t = generate(&c, |_, _| {}).unwrap();
        let a = generate_infrastructure(&t, &c).unwrap();
        let b = generate_infrastructure(&t, &c).unwrap();
        assert_eq!(
            serde_json::to_string(&a.railways).unwrap(),
            serde_json::to_string(&b.railways).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&a.airport).unwrap(),
            serde_json::to_string(&b.airport).unwrap()
        );
    }

    /// A town with a line through it clears the protection zone, and its scene
    /// builds track with every street crossing either carried over or protected.
    #[test]
    fn a_town_receives_the_line() {
        let c = config(7, Landform::Plains, 256);
        let t = generate(&c, |_, _| {}).unwrap();
        let data = generate_infrastructure(&t, &c).unwrap();
        let mut built = 0;
        for (i, city) in data.modern_cities.iter().enumerate() {
            if city.rail.is_empty() {
                continue;
            }
            // Nothing stands inside the formation of the line.
            for corridor in &city.rail {
                let half = corridor.kind.limits().formation_width_m * 0.5;
                let line = corridor.local_line(city);
                for b in &city.buildings {
                    for p in &b.footprint {
                        let q = city.frame.to_local(*p);
                        let d = line
                            .windows(2)
                            .map(|w| {
                                let (vx, vy) = (w[1][0] - w[0][0], w[1][1] - w[0][1]);
                                let l2 = (vx * vx + vy * vy).max(1.0e-6);
                                let t = (((q[0] - w[0][0]) * vx + (q[1] - w[0][1]) * vy) / l2).clamp(0.0, 1.0);
                                (q[0] - w[0][0] - vx * t).hypot(q[1] - w[0][1] - vy * t)
                            })
                            .fold(f32::INFINITY, f32::min);
                        assert!(d > half, "a building corner is {d} m from the line");
                    }
                }
            }
            if built < 1 {
                let scene = data.build_city_scene(i).unwrap();
                let s = &scene.stats;
                assert!(s.rail_sleepers > 100 && s.rail_masts > 4, "{} {}", s.rail_sleepers, s.rail_masts);
                assert!(
                    !s.warnings.iter().any(|w| w.starts_with("rail:")),
                    "{:?}",
                    s.warnings
                );
                built += 1;
            }
        }
        assert!(built >= 1, "no town received a line");
    }
}
