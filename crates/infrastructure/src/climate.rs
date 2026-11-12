//! A town's climate and landform, read from the terrain simulation it sits on.
//!
//! The scene builder picks species, lawn and wild margins from this, so the same
//! planting code makes a lush humid town, a dry northwestern one and an alpine
//! one. Every number is a terrain field sampled around the town centre, never a
//! constant.

use terrain_core::TerrainData;
use urban::{ModernCity, SiteClimate};
use world_core::WorldGrid;

/// Height below which the terrain model puts open sea (the `boundary_water`
/// threshold in `terrain-core`).
const SEA_HEIGHT_M: f32 = 58.0;
/// The nearest-sea search gives up beyond this, kilometres: inland.
const COAST_SEARCH_KM: f32 = 80.0;
/// Window radius over which the climate fields are averaged, kilometres.
const WINDOW_KM: f32 = 1.5;

pub(crate) fn site_climate(
    terrain: &TerrainData,
    grid: WorldGrid,
    city: &ModernCity,
) -> SiteClimate {
    let n = grid.size;
    let km_per_cell = grid.cell_metres() / 1000.0;
    let cx = (city.frame.origin.x_km / km_per_cell)
        .round()
        .clamp(0.0, (n - 1) as f32) as i64;
    let cy = (city.frame.origin.y_km / km_per_cell)
        .round()
        .clamp(0.0, (n - 1) as f32) as i64;
    let reach = ((WINDOW_KM / km_per_cell).ceil() as i64).max(1);
    let mut sums = [0.0_f32; 4];
    let mut count = 0.0_f32;
    for dy in -reach..=reach {
        for dx in -reach..=reach {
            let (x, y) = (cx + dx, cy + dy);
            if x < 0 || y < 0 || x >= n as i64 || y >= n as i64 {
                continue;
            }
            let i = y as usize * n + x as usize;
            sums[0] += terrain.temperature[i];
            sums[1] += terrain.moisture[i];
            sums[2] += terrain.height[i];
            sums[3] += terrain.snow[i];
            count += 1.0;
        }
    }
    let mean = |k: usize| sums[k] / count.max(1.0);

    // Nearest sea: grow square rings until one holds a sea cell.
    let max_ring = (COAST_SEARCH_KM / km_per_cell).ceil() as i64;
    let mut coast_km = COAST_SEARCH_KM * 6.0;
    'rings: for r in 0..=max_ring {
        let mut best = f32::MAX;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue;
                }
                let (x, y) = (cx + dx, cy + dy);
                if x < 0 || y < 0 || x >= n as i64 || y >= n as i64 {
                    continue;
                }
                if terrain.height[y as usize * n + x as usize] < SEA_HEIGHT_M {
                    best = best.min(((dx * dx + dy * dy) as f32).sqrt() * km_per_cell);
                }
            }
        }
        if best < f32::MAX {
            coast_km = best;
            break 'rings;
        }
    }

    SiteClimate {
        temperature_c: mean(0),
        moisture: mean(1).clamp(0.0, 1.0),
        elevation_m: mean(2),
        snow: mean(3).clamp(0.0, 1.0),
        coast_km,
    }
}
