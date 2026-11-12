//! Airport siting and layout. See `docs/transport-infrastructure.md` section 3.

use super::World;
use super::geometry::{self as g, P, Pin};
use super::rail::route_between;
use super::wind::WindRose;
use crate::{SettlementClass, SettlementSite};
use serde::{Deserialize, Serialize};
use urban::{AerodromeCode, AirfieldPlan, Point};
use world_core::WorldGrid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Airport {
    pub plan: AirfieldPlan,
    /// The city it serves.
    pub serves_settlement: u32,
    pub distance_to_city_km: f32,
    /// Share of time the crosswind is within the code's limit.
    pub wind_usability: f32,
    /// The same against the stricter 13 kt limit, for this heading and for the
    /// heading at right angles (to show the choice matters).
    pub wind_usability_13kt: f32,
    pub perpendicular_usability_13kt: f32,
    pub relief_m: f32,
    pub longitudinal_slope: f32,
    pub wind: WindRose,
    /// Expressway from the terminal forecourt toward the city, world km.
    pub access_path_km: Vec<[f32; 2]>,
}

/// Axis angles tried, 10 degrees apart over a half turn.
const HEADINGS: usize = 18;

struct Fit {
    score: f32,
    centre: P,
    axis: f32,
    sign: f32,
    elevation: f32,
    relief: f32,
    slope: f32,
    distance_km: f32,
}

pub fn plan_airport(
    world: &World,
    grid: WorldGrid,
    settlements: &[SettlementSite],
    urban_land: &[f32],
    rose: WindRose,
    seed: u32,
) -> Option<Airport> {
    let city = settlements
        .iter()
        .find(|s| s.class == SettlementClass::RegionalCentre)?;
    let to_m = |s: &SettlementSite| -> P {
        [
            s.location.x as f32 / (grid.size - 1) as f32 * world.size_m,
            s.location.y as f32 / (grid.size - 1) as f32 * world.size_m,
        ]
    };
    let city_m = to_m(city);
    let code = AerodromeCode::C4C;
    let limit = code.crosswind_limit_ms();
    let usability: Vec<f32> = (0..HEADINGS)
        .map(|k| rose.usability((k as f32 * 10.0).to_radians(), limit))
        .collect();
    // Headings are ranked on the stricter 13 kt limit (the value for reference
    // field lengths under 1 500 m and for poor braking): at the 20 kt limit
    // nearly every heading passes in a light-wind climate and the choice would
    // be arbitrary.
    let strict_limit = 13.0 * 0.514_444;
    let strict: Vec<f32> = (0..HEADINGS)
        .map(|k| rose.usability((k as f32 * 10.0).to_radians(), strict_limit))
        .collect();
    let best_u = strict.iter().copied().fold(0.0_f32, f32::max);
    let headings: Vec<usize> = (0..HEADINGS)
        .filter(|k| strict[*k] >= best_u - 0.004)
        .collect();
    let tr = world.terrain;
    let last = (world.n - 1) as f32;
    let idx = |p: P| -> usize {
        let gx = (p[0] / world.size_m * last).round().clamp(0.0, last) as usize;
        let gy = (p[1] / world.size_m * last).round().clamp(0.0, last) as usize;
        gy * world.n + gx
    };
    let step = 1500.0;
    let margin = 5000.0;
    let mut best: Option<Fit> = None;
    let l = code.runway_length_m();
    let mut cy = margin;
    while cy < world.size_m - margin {
        let mut cx = margin;
        while cx < world.size_m - margin {
            let c = [cx, cy];
            cx += step;
            let d_km = g::dist(c, city_m) / 1000.0;
            if !(10.0..=40.0).contains(&d_km) {
                continue;
            }
            if settlements.iter().any(|s| g::dist(c, to_m(s)) < 4000.0) {
                continue;
            }
            let toward = g::unit(g::sub(city_m, c));
            for &k in &headings {
                let axis = (k as f32 * 10.0).to_radians();
                let a = [axis.cos(), axis.sin()];
                let n0 = [-a[1], a[0]];
                let sign = if n0[0] * toward[0] + n0[1] * toward[1] >= 0.0 {
                    1.0
                } else {
                    -1.0
                };
                let n = [n0[0] * sign, n0[1] * sign];
                // Sample the platform: along +-(L/2+300), across -250..+700.
                let (mut sum, mut cnt) = (0.0_f32, 0.0_f32);
                let (mut lo, mut hi) = (f32::INFINITY, f32::NEG_INFINITY);
                let mut wet = 0.0_f32;
                let mut bad = false;
                let mut strip: Vec<(f32, f32)> = Vec::new();
                let mut x = -(l * 0.5 + 300.0);
                while x <= l * 0.5 + 300.0 && !bad {
                    let mut z = -250.0;
                    let (mut srow, mut crow) = (0.0_f32, 0.0_f32);
                    while z <= 700.0 {
                        let p = [c[0] + a[0] * x + n[0] * z, c[1] + a[1] * x + n[1] * z];
                        let i = idx(p);
                        if world.is_sea(p[0], p[1])
                            || tr.lake[i] > 0.05
                            || tr.river_order[i] >= 2
                            || tr.water[i] > 0.3
                            || urban_land[i] > 0.05
                        {
                            bad = true;
                            break;
                        }
                        let h = world.height(p[0], p[1]);
                        sum += h;
                        cnt += 1.0;
                        srow += h;
                        crow += 1.0;
                        lo = lo.min(h);
                        hi = hi.max(h);
                        wet += tr.wetland[i].max(tr.floodplain[i] * 0.8);
                        z += 100.0;
                    }
                    strip.push((x, srow / crow.max(1.0)));
                    x += 150.0;
                }
                if bad || cnt < 10.0 {
                    continue;
                }
                let relief = hi - lo;
                let wet_share = wet / cnt;
                if relief > 16.0 || wet_share > 0.12 {
                    continue;
                }
                // Longitudinal slope: least-squares line through the row means.
                let m = strip.len() as f32;
                let (mx, my) = (
                    strip.iter().map(|s| s.0).sum::<f32>() / m,
                    strip.iter().map(|s| s.1).sum::<f32>() / m,
                );
                let sxx: f32 = strip.iter().map(|s| (s.0 - mx).powi(2)).sum();
                let sxy: f32 = strip.iter().map(|s| (s.0 - mx) * (s.1 - my)).sum();
                let slope = (sxy / sxx.max(1.0)).abs();
                if slope > 0.012 {
                    continue;
                }
                let elevation = sum / cnt;
                // Approach surfaces must clear the ground (Annex 14: 2 %).
                let clear = [-1.0_f32, 1.0].iter().all(|end| {
                    (2..=10).all(|j| {
                        let ox = j as f32 * 300.0;
                        let along = end * (l * 0.5 + 60.0 + ox);
                        let p = [c[0] + a[0] * along, c[1] + a[1] * along];
                        world.height(p[0], p[1]) <= elevation + 0.02 * ox - 3.0
                    })
                });
                if !clear {
                    continue;
                }
                // Wetland and water nearby attract birds: keep clear of them.
                let ring_wet = (0..16)
                    .map(|j| {
                        let ang = j as f32 / 16.0 * std::f32::consts::TAU;
                        let p = [c[0] + ang.cos() * 2500.0, c[1] + ang.sin() * 2500.0];
                        let i = idx(p);
                        tr.wetland[i]
                            .max(tr.lake[i])
                            .max(f32::from(tr.water[i] > 0.4))
                    })
                    .sum::<f32>()
                    / 16.0;
                let dist_term = (-((d_km - 22.0) / 12.0).powi(2)).exp();
                let score = (-relief / 6.0).exp()
                    * (0.5 + 0.5 * strict[k])
                    * dist_term
                    * (1.0 - 0.6 * wet_share * 8.0).max(0.2)
                    * (1.0 - 0.5 * ring_wet);
                if best.as_ref().is_none_or(|b| score > b.score + 1.0e-6) {
                    best = Some(Fit {
                        score,
                        centre: c,
                        axis,
                        sign,
                        elevation,
                        relief,
                        slope,
                        distance_km: d_km,
                    });
                }
            }
        }
        cy += step;
    }
    let fit = best?;
    let plan = AirfieldPlan::layout(
        code,
        seed,
        Point {
            x_km: fit.centre[0] / 1000.0,
            y_km: fit.centre[1] / 1000.0,
        },
        fit.axis,
        fit.sign,
        fit.elevation,
    );
    // Access expressway: forecourt to the city by the cheapest ground.
    let forecourt = plan.to_world(
        plan.car_park.x0 + (plan.car_park.x1 - plan.car_park.x0) * 0.5,
        plan.fence.z1 + 40.0,
    );
    let from = [forecourt.x_km * 1000.0, forecourt.y_km * 1000.0];
    let access = route_between(world, urban_land, from, city_m)
        .map(|path| {
            let simple = g::simplify(&path, 300.0);
            let verts: Vec<(P, Pin)> = simple
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    (
                        *p,
                        if i == 0 || i + 1 == simple.len() {
                            Pin::Fixed
                        } else {
                            Pin::Free
                        },
                    )
                })
                .collect();
            let flags = vec![false; verts.len()];
            let curve = g::fillet(&verts, &flags, 450.0, 900.0).unwrap_or(simple);
            g::resample(&curve, 50.0)
                .0
                .iter()
                .map(|p| [p[0] / 1000.0, p[1] / 1000.0])
                .collect()
        })
        .unwrap_or_default();
    Some(Airport {
        serves_settlement: city.id,
        distance_to_city_km: fit.distance_km,
        wind_usability: usability[(fit.axis.to_degrees() / 10.0).round() as usize % HEADINGS],
        wind_usability_13kt: strict[(fit.axis.to_degrees() / 10.0).round() as usize % HEADINGS],
        perpendicular_usability_13kt: strict
            [((fit.axis.to_degrees() / 10.0).round() as usize + 9) % HEADINGS],
        relief_m: fit.relief,
        longitudinal_slope: fit.slope,
        wind: rose,
        access_path_km: access,
        plan,
    })
}

impl Airport {
    /// Level terrain mesh heights (metres) under the platform and blend back
    /// to relief over 300 m beyond it.
    pub fn level_heights(&self, heights: &mut [f32], mesh_size: usize, world_size_km: f32) {
        if mesh_size < 2 || heights.len() < mesh_size * mesh_size {
            return;
        }
        let plan = &self.plan;
        let cell_km = world_size_km / (mesh_size - 1) as f32;
        let r = plan
            .platform
            .x1
            .hypot(plan.platform.z1.abs().max(plan.platform.z0.abs()))
            + 400.0;
        let c = plan.centre;
        let span = (r / 1000.0 / cell_km).ceil() as isize + 1;
        let (cx, cy) = (
            (c.x_km / cell_km).round() as isize,
            (c.y_km / cell_km).round() as isize,
        );
        for y in (cy - span).max(0)..=(cy + span).min(mesh_size as isize - 1) {
            for x in (cx - span).max(0)..=(cx + span).min(mesh_size as isize - 1) {
                let p = Point {
                    x_km: x as f32 * cell_km,
                    y_km: y as f32 * cell_km,
                };
                let [lx, lz] = plan.to_local(p);
                let dx = (plan.platform.x0 - lx).max(lx - plan.platform.x1).max(0.0);
                let dz = (plan.platform.z0 - lz).max(lz - plan.platform.z1).max(0.0);
                let d = dx.hypot(dz);
                if d >= 300.0 {
                    continue;
                }
                let t = 1.0 - d / 300.0;
                let w = t * t * (3.0 - 2.0 * t);
                let h = &mut heights[y as usize * mesh_size + x as usize];
                *h = *h * (1.0 - w) + plan.elevation_m * w;
            }
        }
    }
}
