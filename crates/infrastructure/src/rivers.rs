//! Rivers as lines, read from the terrain's flow.
//!
//! The terrain knows water as grids: how much flows through each cell, and which
//! neighbour it drains to. A road layer wants rivers as lines with a width, to
//! decide where a bridge is needed and where the water is; so the channel cells
//! are followed downstream into chains, smoothed (a grid path turns in
//! forty-five-degree steps, a river does not) and given a width that grows with
//! the flow they carry.

use std::collections::HashMap;

use terrain_core::TerrainData;
use world_core::WorldGrid;
use worldgen_contracts::{Polyline, V2, WaterField, WaterHit, closest_on_segment, v2};
use worldgen_core::Seed;
use worldgen_core::hash::hash_words;
use worldgen_core::noise::fbm;
use worldgen_roads::shape::round_corners;

/// Height below which the ground is sea, metres.
pub const SEA_LEVEL_M: f32 = 58.0;

#[derive(Debug, Clone)]
pub struct River {
    pub id: u64,
    pub line: Polyline,
    /// Width at each vertex, metres.
    pub width: Vec<f64>,
}

impl River {
    pub fn mean_width(&self) -> f64 {
        self.width.iter().sum::<f64>() / self.width.len().max(1) as f64
    }
}

/// All the channels of a terrain, with a spatial index.
pub struct Rivers {
    pub rivers: Vec<River>,
    index: HashMap<(i64, i64), Vec<(usize, usize)>>,
    cell_m: f64,
}

/// Width of a river carrying `share` of the world's largest flow, metres: a few
/// metres for a stream, a hundred and more for the main river.
pub fn width_for_flow(share: f64) -> f64 {
    (8.0 * (share * 500.0).sqrt()).clamp(2.5, 220.0)
}

impl Rivers {
    pub fn from_terrain(terrain: &TerrainData, grid: WorldGrid) -> Rivers {
        let n = grid.size;
        let cell_m = f64::from(grid.cell_metres());
        let max_flow = terrain.flow.iter().copied().fold(1.0_f32, f32::max);
        // Channels worth a bridge: order 2 and up, or a real share of the flow.
        let is_river = |i: usize| terrain.river_order[i] >= 2 || terrain.flow[i] / max_flow > 0.002;
        let receiver = |i: usize| -> Option<usize> {
            let (dx, dy) = (terrain.flow_direction_x[i], terrain.flow_direction_y[i]);
            let (ox, oy) = (
                if dx.abs() > 0.5 {
                    dx.signum() as isize
                } else {
                    0
                },
                if dy.abs() > 0.5 {
                    dy.signum() as isize
                } else {
                    0
                },
            );
            if ox == 0 && oy == 0 {
                return None;
            }
            let (x, y) = ((i % n) as isize + ox, (i / n) as isize + oy);
            ((0..n as isize).contains(&x) && (0..n as isize).contains(&y))
                .then(|| y as usize * n + x as usize)
        };
        let mut upstream = vec![0_u8; n * n];
        for i in 0..n * n {
            if is_river(i)
                && terrain.height[i] >= SEA_LEVEL_M
                && let Some(r) = receiver(i).filter(|r| is_river(*r))
            {
                upstream[r] = upstream[r].saturating_add(1);
            }
        }
        // Follow each source downstream until the sea, the edge, or a cell already followed.
        let mut sources: Vec<usize> = (0..n * n)
            .filter(|&i| is_river(i) && terrain.height[i] >= SEA_LEVEL_M && upstream[i] == 0)
            .collect();
        // The biggest rivers first, so a trunk is traced from its own headwater.
        sources.sort_by(|a, b| terrain.flow[*b].total_cmp(&terrain.flow[*a]));
        let mut seen = vec![false; n * n];
        let mut rivers = Vec::new();
        for source in sources {
            let mut cells = Vec::new();
            let mut at = Some(source);
            while let Some(i) = at {
                cells.push(i);
                if seen[i] || !is_river(i) || terrain.height[i] < SEA_LEVEL_M {
                    break;
                }
                seen[i] = true;
                at = receiver(i);
            }
            if cells.len() < 3 {
                continue;
            }
            let at = |i: usize| v2((i % n) as f64 * cell_m, (i / n) as f64 * cell_m);
            let raw: Vec<V2> = cells.iter().map(|&i| at(i)).collect();
            let width_raw: Vec<f64> = cells
                .iter()
                .map(|&i| width_for_flow(f64::from(terrain.flow[i] / max_flow)))
                .collect();
            // A grid path in 45-degree steps becomes a river: average, then round.
            let averaged: Vec<V2> = (0..raw.len())
                .map(|k| {
                    let (lo, hi) = (k.saturating_sub(1), (k + 1).min(raw.len() - 1));
                    if k == 0 || k == raw.len() - 1 {
                        raw[k]
                    } else {
                        (raw[lo] + raw[k] * 2.0 + raw[hi]) * 0.25
                    }
                })
                .collect();
            let line = round_corners(&Polyline(averaged), 2.2 * cell_m, (cell_m * 0.5).max(10.0));
            // Width along the smoothed line: carry the raw widths over by arc length.
            let total = line.length().max(1e-9);
            let raw_total: f64 = raw
                .windows(2)
                .map(|w| w[0].dist(w[1]))
                .sum::<f64>()
                .max(1e-9);
            let width: Vec<f64> = {
                let mut run = 0.0;
                let mut out = Vec::with_capacity(line.0.len());
                for (k, p) in line.0.iter().enumerate() {
                    if k > 0 {
                        run += line.0[k - 1].dist(*p);
                    }
                    let f = (run / total * (width_raw.len() - 1) as f64)
                        .clamp(0.0, (width_raw.len() - 1) as f64);
                    let (lo, hi) = (
                        f.floor() as usize,
                        (f.ceil() as usize).min(width_raw.len() - 1),
                    );
                    out.push(width_raw[lo] + (width_raw[hi] - width_raw[lo]) * (f - lo as f64));
                }
                let _ = raw_total;
                out
            };
            let id = hash_words(&[cells[0] as u64, cells.len() as u64, 0x817E]);
            let (line, width) = meander(&line, &width, Seed::new(id));
            rivers.push(River { id, line, width });
        }
        let mut index: HashMap<(i64, i64), Vec<(usize, usize)>> = HashMap::new();
        let bucket = 1000.0;
        for (r, river) in rivers.iter().enumerate() {
            for (s, w) in river.line.0.windows(2).enumerate() {
                let steps = (w[0].dist(w[1]) / (bucket * 0.5)).ceil().max(1.0) as usize;
                for k in 0..=steps {
                    let p = w[0].lerp(w[1], k as f64 / steps as f64);
                    let slot = index
                        .entry(((p.x / bucket).floor() as i64, (p.y / bucket).floor() as i64))
                        .or_default();
                    if slot.last() != Some(&(r, s)) {
                        slot.push((r, s));
                    }
                }
            }
        }
        Rivers {
            rivers,
            index,
            cell_m: bucket,
        }
    }

    /// The segments of rivers within `reach` of `p`: (river, segment index).
    fn near(&self, p: V2, reach: f64) -> Vec<(usize, usize)> {
        let (x0, x1) = (
            ((p.x - reach) / self.cell_m).floor() as i64,
            ((p.x + reach) / self.cell_m).floor() as i64,
        );
        let (y0, y1) = (
            ((p.y - reach) / self.cell_m).floor() as i64,
            ((p.y + reach) / self.cell_m).floor() as i64,
        );
        let mut out = Vec::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                if let Some(slot) = self.index.get(&(x, y)) {
                    out.extend_from_slice(slot);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// The nearest river to `p` within `reach`: its distance from the bank and its width there.
    pub fn nearest(&self, p: V2, reach: f64) -> Option<WaterHit> {
        let mut best: Option<(f64, f64, u64)> = None;
        for (r, s) in self.near(p, reach + 250.0) {
            let river = &self.rivers[r];
            let (a, b) = (river.line.0[s], river.line.0[s + 1]);
            let (_, t, d) = closest_on_segment(a, b, p);
            let width = river.width[s] + (river.width[s + 1] - river.width[s]) * t;
            let edge = d - width * 0.5;
            if best.is_none_or(|(e, _, _)| edge < e) {
                best = Some((edge, width, river.id));
            }
        }
        best.filter(|(edge, _, _)| *edge <= reach)
            .map(|(distance_m, width_m, id)| WaterHit {
                distance_m,
                width_m,
                id,
            })
    }

    /// The pieces of rivers inside the circle `centre`, `radius`, with their mean width.
    pub fn through(&self, centre: V2, radius: f64) -> Vec<(Polyline, f64)> {
        let mut out = Vec::new();
        for river in &self.rivers {
            let mut run: Vec<V2> = Vec::new();
            let mut widths: Vec<f64> = Vec::new();
            let mut flush = |run: &mut Vec<V2>, widths: &mut Vec<f64>| {
                if run.len() >= 2 {
                    let mean = widths.iter().sum::<f64>() / widths.len() as f64;
                    out.push((Polyline(std::mem::take(run)), mean));
                }
                run.clear();
                widths.clear();
            };
            for (k, p) in river.line.0.iter().enumerate() {
                if p.dist(centre) <= radius {
                    run.push(*p);
                    widths.push(river.width[k]);
                } else {
                    flush(&mut run, &mut widths);
                }
            }
            flush(&mut run, &mut widths);
        }
        out
    }
}

/// A river runs through its valley in bends much finer than a grid of hundreds of
/// metres can show. Add them: a lateral wander whose wavelength and amplitude
/// scale with the width (meander wavelength is about eleven channel widths), tapered
/// to nothing at both ends so a tributary still reaches the river it joins.
fn meander(line: &Polyline, width: &[f64], seed: Seed) -> (Polyline, Vec<f64>) {
    let total = line.length();
    if total < 100.0 || line.0.len() < 2 {
        return (line.clone(), width.to_vec());
    }
    let mean = width.iter().sum::<f64>() / width.len().max(1) as f64;
    let step = (mean * 0.9).clamp(18.0, 60.0);
    let n = (total / step).ceil() as usize;
    let mut points = Vec::with_capacity(n + 1);
    let mut widths = Vec::with_capacity(n + 1);
    for k in 0..=n {
        let s = total * k as f64 / n as f64;
        let Some((p, tangent)) = line.at(s) else {
            continue;
        };
        // Width here, by arc fraction through the original vertices.
        let f = (s / total * (width.len() - 1) as f64).clamp(0.0, (width.len() - 1) as f64);
        let (lo, hi) = (f.floor() as usize, (f.ceil() as usize).min(width.len() - 1));
        let w = width[lo] + (width[hi] - width[lo]) * (f - lo as f64);
        let (wavelength, amplitude) = ((11.0 * w).clamp(90.0, 1200.0), (2.0 * w).clamp(5.0, 70.0));
        let taper = ((s / 220.0).min(1.0) * ((total - s) / 220.0).min(1.0)).clamp(0.0, 1.0);
        let wander = (fbm(seed, s / wavelength, 0.37, 2, 0.5) - 0.5) * 2.0;
        points.push(p + tangent.perp() * (wander * amplitude * taper));
        widths.push(w);
    }
    (Polyline(points), widths)
}

/// Rivers, and the still water of lakes and the sea, as one `WaterField`.
pub struct WorldWater {
    pub rivers: std::sync::Arc<Rivers>,
    grid: WorldGrid,
    still: Vec<f32>,
}

impl WorldWater {
    pub fn new(
        terrain: &TerrainData,
        grid: WorldGrid,
        rivers: std::sync::Arc<Rivers>,
    ) -> WorldWater {
        // 1 where the ground is under still water: the sea, or a lake.
        let still = (0..grid.len())
            .map(|i| {
                if terrain.height[i] < SEA_LEVEL_M || terrain.lake[i] > 0.45 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        WorldWater {
            rivers,
            grid,
            still,
        }
    }

    fn still_at(&self, p: V2) -> f64 {
        let n = self.grid.size;
        let cell = f64::from(self.grid.cell_metres());
        let (fx, fy) = (
            (p.x / cell).clamp(0.0, (n - 1) as f64),
            (p.y / cell).clamp(0.0, (n - 1) as f64),
        );
        let (x0, y0) = (fx.floor() as usize, fy.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(n - 1), (y0 + 1).min(n - 1));
        let (tx, ty) = (fx - x0 as f64, fy - y0 as f64);
        let v = |x: usize, y: usize| f64::from(self.still[y * n + x]);
        let top = v(x0, y0) + (v(x1, y0) - v(x0, y0)) * tx;
        let bottom = v(x0, y1) + (v(x1, y1) - v(x0, y1)) * tx;
        top + (bottom - top) * ty
    }
}

impl WaterField for WorldWater {
    fn nearest(&self, p: V2, within_m: f64) -> Option<WaterHit> {
        if self.still_at(p) > 0.5 {
            return Some(WaterHit {
                distance_m: 0.0,
                width_m: 1000.0,
                id: 0x5EA,
            });
        }
        self.rivers.nearest(p, within_m)
    }
}
