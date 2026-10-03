//! City sites: where the metre-scale city owns the ground.
//!
//! The city is built on a flat frame, so the terrain under a settlement must
//! agree with it (roads must not float on slopes and hills must not poke through
//! blocks), and the land must blend back to its natural relief beyond the city
//! so there is no visible plateau. Vegetation, fields and forest must also not
//! be scattered under the streets. Both used to be done by the renderer after
//! the payload arrived; they are terrain generation, so they live here and the
//! renderer only consumes the result.

use serde::{Deserialize, Serialize};

/// Width of the smooth blend from the flat city pad back to natural relief.
pub const BLEND_M: f32 = 420.0;
/// A city site never levels less than this radius, however small the plan.
pub const MIN_RADIUS_M: f32 = 450.0;
/// Vegetation stays out this far beyond a site's levelled radius.
pub const VEGETATION_MARGIN_M: f32 = 60.0;
/// The level the pad is set to sits this far under the mean of the core.
const PAD_DROP_M: f32 = 0.12;

/// A settlement's footprint on the terrain, in kilometres from the world corner.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitySite {
    pub x_km: f32,
    pub y_km: f32,
    /// Radius of the flat pad; the blend ring lies outside it.
    pub radius_m: f32,
}

/// Site for a city from the extent of its road-graph nodes
/// (`min_x, max_x, min_y, max_y`, all in km).
pub fn site_from_extent(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> CitySite {
    CitySite {
        x_km: (min_x + max_x) * 0.5,
        y_km: (min_y + max_y) * 0.5,
        radius_m: MIN_RADIUS_M.max((max_x - min_x).hypot(max_y - min_y) * 500.0 + 220.0),
    }
}

/// Site for a city from its node points, or `None` for an empty graph.
pub fn site_from_points(points: impl IntoIterator<Item = (f32, f32)>) -> Option<CitySite> {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::INFINITY,
        f32::NEG_INFINITY,
    );
    let mut any = false;
    for (x, y) in points {
        any = true;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    any.then(|| site_from_extent(min_x, max_x, min_y, max_y))
}

fn cell_metres(world_size_km: f32, mesh_size: usize) -> f32 {
    world_size_km * 1000.0 / (mesh_size.max(2) - 1) as f32
}

/// Level the mesh heights (metres) under every site and blend back to relief.
///
/// The pad level is the mean of the site's inner half radius, minus a hair so
/// road surfaces sit just above the ground. Blend uses a smoothstep over
/// [`BLEND_M`] beyond the pad radius.
pub fn flatten_heights(
    heights: &mut [f32],
    mesh_size: usize,
    world_size_km: f32,
    sites: &[CitySite],
) {
    if mesh_size < 2 || heights.len() < mesh_size * mesh_size {
        return;
    }
    let cell_m = cell_metres(world_size_km, mesh_size);
    let last = (mesh_size - 1) as f32;
    for site in sites {
        let cx = site.x_km / world_size_km * last;
        let cy = site.y_km / world_size_km * last;
        let reach = ((site.radius_m + BLEND_M) / cell_m).ceil() as i64 + 1;
        let core = (((site.radius_m * 0.5) / cell_m).round() as i64).max(1);
        let (rx, ry) = (cx.round() as i64, cy.round() as i64);
        let (mut sum, mut count) = (0.0_f64, 0_u32);
        for dy in -core..=core {
            for dx in -core..=core {
                let (ix, iy) = (rx + dx, ry + dy);
                if ix < 0 || iy < 0 || ix >= mesh_size as i64 || iy >= mesh_size as i64 {
                    continue;
                }
                sum += heights[iy as usize * mesh_size + ix as usize] as f64;
                count += 1;
            }
        }
        if count == 0 {
            continue;
        }
        let level = (sum / count as f64) as f32 - PAD_DROP_M;
        for dy in -reach..=reach {
            for dx in -reach..=reach {
                let (ix, iy) = (rx + dx, ry + dy);
                if ix < 0 || iy < 0 || ix >= mesh_size as i64 || iy >= mesh_size as i64 {
                    continue;
                }
                let distance = ((ix as f32 - cx).hypot(iy as f32 - cy)) * cell_m;
                if distance >= site.radius_m + BLEND_M {
                    continue;
                }
                let t = ((distance - site.radius_m) / BLEND_M).clamp(0.0, 1.0);
                let weight = 1.0 - t * t * (3.0 - 2.0 * t);
                let at = iy as usize * mesh_size + ix as usize;
                heights[at] = heights[at] * (1.0 - weight) + level * weight;
            }
        }
    }
}

/// Mark the vegetation-exclusion mask (255 = no plants) under every site, out
/// to the pad radius plus [`VEGETATION_MARGIN_M`].
pub fn exclude_vegetation(
    mask: &mut [u8],
    mesh_size: usize,
    world_size_km: f32,
    sites: &[CitySite],
) {
    if mesh_size < 2 || mask.len() < mesh_size * mesh_size {
        return;
    }
    let cell_m = cell_metres(world_size_km, mesh_size);
    let last = (mesh_size - 1) as f32;
    for site in sites {
        let cx = site.x_km / world_size_km * last;
        let cy = site.y_km / world_size_km * last;
        let clear_m = site.radius_m + VEGETATION_MARGIN_M;
        let reach = (clear_m / cell_m).ceil() as i64 + 1;
        let (rx, ry) = (cx.round() as i64, cy.round() as i64);
        for dy in -reach..=reach {
            for dx in -reach..=reach {
                let (ix, iy) = (rx + dx, ry + dy);
                if ix < 0 || iy < 0 || ix >= mesh_size as i64 || iy >= mesh_size as i64 {
                    continue;
                }
                if ((ix as f32 - cx).hypot(iy as f32 - cy)) * cell_m <= clear_m {
                    mask[iy as usize * mesh_size + ix as usize] = 255;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_site_is_at_least_the_minimum_radius_and_centred_on_its_nodes() {
        let site = site_from_points([(1.0, 2.0), (1.1, 2.1)]).unwrap();
        assert!((site.x_km - 1.05).abs() < 1e-5 && (site.y_km - 2.05).abs() < 1e-5);
        assert_eq!(site.radius_m, MIN_RADIUS_M);
        assert!(site_from_points(std::iter::empty()).is_none());
    }

    #[test]
    fn flattening_levels_the_core_and_leaves_far_ground_alone() {
        let size = 257;
        let world = 20.0; // 78 m cells
        let mut heights: Vec<f32> = (0..size * size)
            .map(|i| 100.0 + 0.5 * (i % size) as f32 + 0.3 * (i / size) as f32)
            .collect();
        let original = heights.clone();
        let site = CitySite {
            x_km: 10.0,
            y_km: 10.0,
            radius_m: 600.0,
        };
        flatten_heights(&mut heights, size, world, &[site]);
        let centre = 128 * size + 128;
        let near = 128 * size + 130;
        assert!(
            (heights[centre] - heights[near]).abs() < 0.2,
            "pad is not flat"
        );
        assert_eq!(heights[0], original[0]);
        assert_eq!(heights[size * size - 1], original[size * size - 1]);
        let mut mask = vec![0_u8; size * size];
        exclude_vegetation(&mut mask, size, world, &[site]);
        assert_eq!(mask[centre], 255);
        assert_eq!(mask[0], 0);
    }
}
