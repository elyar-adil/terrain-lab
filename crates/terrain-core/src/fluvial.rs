//! Fluvial landscape evolution.
//!
//! A mountain is not a noise function. Its ridges and valleys are what is left
//! when rivers cut into rising rock for a very long time: water collects into
//! channels, channels incise at a rate that grows with the water they carry and
//! the slope beneath them, and the ground between them is left as ridges. The
//! branching, self-similar drainage of a real range comes out of that process;
//! it cannot be painted with noise.
//!
//! This module runs that process. The model is the stream power law with
//! hillslope diffusion,
//!
//! ```text
//! dh/dt = U - K * A^m * S + D * laplacian(h)
//! ```
//!
//! (uplift `U`, erodibility `K`, drainage area `A`, channel slope `S`),
//! solved with the implicit scheme of Braun and Willett (2013), which is
//! unconditionally stable and visits each cell once per step in drainage order,
//! so a step costs `O(N)` plus the depression fill. Because information travels
//! the whole length of a river in one step, a few hundred steps are enough.
//!
//! The one-shot landform model supplies only the *large-scale* shape: its
//! smoothed elevation becomes the uplift field, so a range, a plateau or a
//! coast still lands where it was planned. Everything finer is carved.
//!
//! Resolution is built up in stages: the coarse grid fixes the major valleys and
//! divides, then each finer grid adds the tributaries that the coarser one could
//! not resolve. Rock hardness from the geology model scales erodibility, so soft
//! rock opens broad valleys and hard rock holds ridges.

use rayon::prelude::*;

/// Terrain at or below this elevation is the sea and does not erode.
pub const SEA_LEVEL: f32 = 58.0;

/// Exponent of drainage area in the stream power law.
const AREA_EXPONENT: f32 = 0.45;
/// Erodibility times the time step, in the units the scheme works in. Together
/// with `UPLIFT_RATE` it sets how quickly the landscape approaches balance.
const ERODIBILITY: f32 = 0.05;
/// Uplift per step as a fraction of the smoothed target elevation.
const UPLIFT_RATE: f32 = 2.0e-4;
/// Fraction of the height difference to the neighbour average that hillslope
/// creep removes each step.
const CREEP: f32 = 0.11;

/// Steps spent at each stage of the resolution ladder.
const STEPS: [usize; 3] = [140, 40, 16];

const NONE: u32 = u32::MAX;

struct Level {
    n: usize,
    height: Vec<f32>,
    uplift: Vec<f32>,
    erodibility: Vec<f32>,
}

/// Evolve `height` (square, `n` by `n`, metres) toward a fluvial landscape.
///
/// `resistance` is the geology model's erosion resistance (about 0.3 to 2.4) at
/// the same size. `progress` receives 0..1 as the work advances.
pub fn evolve(
    height: &mut [f32],
    resistance: &[f32],
    n: usize,
    cell_metres: f32,
    seed: u32,
    progress: &mut dyn FnMut(f32),
) {
    if n < 32 || height.len() != n * n || resistance.len() != n * n {
        return;
    }
    let original = height.to_vec();
    let target_max = height.iter().copied().fold(0.0_f32, f32::max);
    if target_max <= SEA_LEVEL + 1.0 {
        return;
    }

    // The ladder: 512 (or the grid itself if smaller), doubling up to `n`.
    let mut sizes = vec![n.min(512)];
    while *sizes.last().unwrap() * 2 <= n {
        sizes.push(sizes.last().unwrap() * 2);
    }
    if *sizes.last().unwrap() != n {
        sizes.push(n);
    }

    let first = sizes[0];
    let coarse_target = resample(height, n, first);
    let smooth = smooth_field(&coarse_target, first, (first / 36).max(2));
    let coarse_resistance = resample(resistance, n, first);

    let mut level = Level {
        n: first,
        height: seeded_start(&smooth, first, seed),
        uplift: smooth.iter().map(|h| (h - SEA_LEVEL).max(0.0) * UPLIFT_RATE).collect(),
        erodibility: coarse_resistance
            .iter()
            .zip(heterogeneity(first, seed))
            .map(|(r, h)| ERODIBILITY * h / r.max(0.25))
            .collect(),
    };

    let total_steps: usize = sizes.iter().enumerate().map(|(i, _)| STEPS[i.min(2)]).sum();
    let mut done = 0;
    for (stage, &size) in sizes.iter().enumerate() {
        if stage > 0 {
            // Carry the carved surface up a size; the new grid starts from it and
            // is free to cut finer channels into it.
            let factor = size as f32 / level.n as f32;
            level = Level {
                n: size,
                height: resample(&level.height, level.n, size),
                uplift: resample(&level.uplift, level.n, size),
                erodibility: resample(resistance, n, size)
                    .iter()
                    .zip(heterogeneity(size, seed))
                    .map(|(r, h)| ERODIBILITY * h / r.max(0.25))
                    .collect(),
            };
            let _ = factor;
        }
        let cell = cell_metres * n as f32 / level.n as f32;
        for _ in 0..STEPS[stage.min(2)] {
            step(&mut level, cell);
            done += 1;
            progress(done as f32 / total_steps as f32);
        }
        // Erosion in this model lowers the whole range slightly each stage; put
        // the planned relief back so the world keeps the height it was designed
        // to have, and keep the uplift in proportion so later stages agree.
        let max_now = level.height.iter().copied().fold(0.0_f32, f32::max);
        if max_now > SEA_LEVEL + 1.0 {
            let k = (target_max - SEA_LEVEL) / (max_now - SEA_LEVEL);
            for h in &mut level.height {
                if *h > SEA_LEVEL {
                    *h = SEA_LEVEL + (*h - SEA_LEVEL) * k;
                }
            }
            for u in &mut level.uplift {
                *u *= k;
            }
        }
    }

    // Sea and map edge are base level and keep their planned values; land the
    // smoothing pulled to the waterline stays above it.
    for (i, out) in height.iter_mut().enumerate() {
        let (x, y) = (i % n, i / n);
        *out = if original[i] <= SEA_LEVEL || x == 0 || y == 0 || x + 1 == n || y + 1 == n {
            original[i]
        } else {
            level.height[i].max(SEA_LEVEL + 0.5)
        };
    }
}

/// The starting surface: the smoothed plan plus a little seeded roughness, which
/// is what the first channels grow from.
fn seeded_start(smooth: &[f32], n: usize, seed: u32) -> Vec<f32> {
    smooth
        .par_iter()
        .enumerate()
        .map(|(i, &h)| {
            if h <= SEA_LEVEL {
                return h;
            }
            let x = (i % n) as u32;
            let y = (i / n) as u32;
            let mut v = seed ^ x.wrapping_mul(0x9e37_79b9) ^ y.wrapping_mul(0x85eb_ca6b);
            v ^= v >> 15;
            v = v.wrapping_mul(0x2c1b_3c6d);
            v ^= v >> 12;
            let r = v as f32 / u32::MAX as f32 - 0.5;
            h + r * (4.0 + h * 0.004)
        })
        .collect()
}

/// Smooth multi-scale multiplier near 1 (about 0.4 to 2.5) for erodibility. Real
/// rock is layered and patchy; a uniform field makes every valley the same width
/// and spacing, which is the signature of a simulation rather than a mountain.
fn heterogeneity(n: usize, seed: u32) -> Vec<f32> {
    let value = |gx: i32, gy: i32, salt: u32| -> f32 {
        let mut v = seed ^ (gx as u32).wrapping_mul(0x9e37_79b9) ^ (gy as u32).wrapping_mul(0x85eb_ca6b) ^ salt.wrapping_mul(0xc2b2_ae35);
        v ^= v >> 16;
        v = v.wrapping_mul(0x7feb_352d);
        v ^= v >> 15;
        v as f32 / u32::MAX as f32
    };
    let octave = |x: f32, y: f32, cells: f32, salt: u32| -> f32 {
        let (fx, fy) = (x / n as f32 * cells, y / n as f32 * cells);
        let (ix, iy) = (fx.floor() as i32, fy.floor() as i32);
        let (tx, ty) = (fx - ix as f32, fy - iy as f32);
        let (sx, sy) = (tx * tx * (3.0 - 2.0 * tx), ty * ty * (3.0 - 2.0 * ty));
        let a = value(ix, iy, salt) * (1.0 - sx) + value(ix + 1, iy, salt) * sx;
        let b = value(ix, iy + 1, salt) * (1.0 - sx) + value(ix + 1, iy + 1, salt) * sx;
        a * (1.0 - sy) + b * sy - 0.5
    };
    (0..n * n)
        .into_par_iter()
        .map(|i| {
            let (x, y) = ((i % n) as f32, (i / n) as f32);
            let v = octave(x, y, 5.0, 1) * 1.0 + octave(x, y, 17.0, 2) * 0.7 + octave(x, y, 53.0, 3) * 0.45;
            (v * 1.3).exp()
        })
        .collect()
}

/// One implicit stream-power step plus hillslope creep.
fn step(level: &mut Level, cell: f32) {
    let n = level.n;
    let len = n * n;
    let mut filled = vec![0.0_f32; len];
    super::priority_flood(&level.height, n, &mut filled);

    // Steepest-descent receiver of every cell on the filled surface.
    let receiver: Vec<u32> = (0..len)
        .into_par_iter()
        .map(|i| {
            let x = i % n;
            let y = i / n;
            if x == 0 || y == 0 || x + 1 == n || y + 1 == n || level.height[i] <= SEA_LEVEL {
                return NONE;
            }
            let mut best = NONE;
            let mut best_slope = 0.0_f32;
            for oy in -1_isize..=1 {
                for ox in -1_isize..=1 {
                    if ox == 0 && oy == 0 {
                        continue;
                    }
                    let j = (y as isize + oy) as usize * n + (x as isize + ox) as usize;
                    let dist = if ox != 0 && oy != 0 { std::f32::consts::SQRT_2 } else { 1.0 };
                    let slope = (filled[i] - filled[j]) / dist;
                    if slope > best_slope {
                        best_slope = slope;
                        best = j as u32;
                    }
                }
            }
            best
        })
        .collect();

    // Upstream-to-downstream order: highest filled surface first.
    let mut order: Vec<u32> = (0..len as u32).collect();
    order.par_sort_unstable_by(|&a, &b| filled[b as usize].total_cmp(&filled[a as usize]));

    // Drainage area (m^2).
    let mut area = vec![cell * cell; len];
    for &i in &order {
        let r = receiver[i as usize];
        if r != NONE {
            let a = area[i as usize];
            area[r as usize] += a;
        }
    }

    // Implicit update, downstream first so each receiver is already new.
    let mut next = level.height.clone();
    for &i in order.iter().rev() {
        let i = i as usize;
        let r = receiver[i];
        if r == NONE {
            // Base level (edge or sea): fixed, but a lake-filled sink keeps its fill.
            next[i] = if level.height[i] <= SEA_LEVEL { level.height[i] } else { filled[i] };
            continue;
        }
        let r = r as usize;
        let dx = if (i % n) != (r % n) && (i / n) != (r / n) { cell * std::f32::consts::SQRT_2 } else { cell };
        let f = level.erodibility[i] * area[i].powf(AREA_EXPONENT) / dx;
        next[i] = (filled[i] + level.uplift[i] + f * next[r]) / (1.0 + f);
    }

    // Hillslope creep: soften what the channels leave without erasing them.
    let source = next.clone();
    next.par_iter_mut().enumerate().for_each(|(i, h)| {
        let x = i % n;
        let y = i / n;
        if x == 0 || y == 0 || x + 1 == n || y + 1 == n || source[i] <= SEA_LEVEL {
            return;
        }
        let avg = (source[i - 1] + source[i + 1] + source[i - n] + source[i + n]) * 0.25;
        *h = source[i] + (avg - source[i]) * CREEP;
    });
    level.height = next;
}

/// Box blur, three passes (close to Gaussian), edges clamped.
fn smooth_field(src: &[f32], n: usize, radius: usize) -> Vec<f32> {
    let mut a = src.to_vec();
    let mut b = vec![0.0_f32; a.len()];
    for _ in 0..3 {
        blur_axis(&a, &mut b, n, radius, true);
        blur_axis(&b, &mut a, n, radius, false);
    }
    a
}

fn blur_axis(src: &[f32], dst: &mut [f32], n: usize, radius: usize, horizontal: bool) {
    let r = radius as isize;
    dst.par_chunks_mut(n).enumerate().for_each(|(row, out)| {
        for col in 0..n {
            let mut sum = 0.0;
            for k in -r..=r {
                let (x, y) = if horizontal {
                    ((col as isize + k).clamp(0, n as isize - 1) as usize, row)
                } else {
                    (col, (row as isize + k).clamp(0, n as isize - 1) as usize)
                };
                sum += src[y * n + x];
            }
            out[col] = sum / (2 * radius + 1) as f32;
        }
    });
}

/// Bilinear resample of a square field.
fn resample(src: &[f32], from: usize, to: usize) -> Vec<f32> {
    if from == to {
        return src.to_vec();
    }
    let scale = (from - 1) as f32 / (to - 1) as f32;
    (0..to * to)
        .into_par_iter()
        .map(|i| {
            let sx = (i % to) as f32 * scale;
            let sy = (i / to) as f32 * scale;
            let x0 = sx.floor() as usize;
            let y0 = sy.floor() as usize;
            let x1 = (x0 + 1).min(from - 1);
            let y1 = (y0 + 1).min(from - 1);
            let fx = sx - x0 as f32;
            let fy = sy - y0 as f32;
            let top = src[y0 * from + x0] * (1.0 - fx) + src[y0 * from + x1] * fx;
            let bottom = src[y1 * from + x0] * (1.0 - fx) + src[y1 * from + x1] * fx;
            top * (1.0 - fy) + bottom * fy
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A smooth ridge with a plain either side, 128 cells, 150 m cells.
    fn ridge(n: usize) -> (Vec<f32>, Vec<f32>) {
        let height: Vec<f32> = (0..n * n)
            .map(|i| {
                let y = (i / n) as f32 / (n - 1) as f32;
                let belt = (-((y - 0.5) / 0.2).powi(2)).exp();
                20.0 + belt * 2400.0
            })
            .collect();
        (height, vec![1.0; n * n])
    }

    #[test]
    fn evolution_is_deterministic() {
        let (mut a, r) = ridge(128);
        let mut b = a.clone();
        evolve(&mut a, &r, 128, 150.0, 7, &mut |_| {});
        evolve(&mut b, &r, 128, 150.0, 7, &mut |_| {});
        assert_eq!(a, b);
        let mut c = ridge(128).0;
        evolve(&mut c, &r, 128, 150.0, 8, &mut |_| {});
        assert_ne!(a, c, "the seed must change the drainage");
    }

    #[test]
    fn the_planned_relief_survives_and_the_sea_does_not_move() {
        let (mut h, r) = ridge(128);
        let before = h.clone();
        evolve(&mut h, &r, 128, 150.0, 3, &mut |_| {});
        let max_before = before.iter().copied().fold(0.0_f32, f32::max);
        let max_after = h.iter().copied().fold(0.0_f32, f32::max);
        assert!((max_after / max_before - 1.0).abs() < 0.02, "{max_before} -> {max_after}");
        // Terrain at or below the sea, and the map edge, are base level.
        for (i, (&b, &a)) in before.iter().zip(&h).enumerate() {
            let (x, y) = (i % 128, i / 128);
            if b <= SEA_LEVEL || x == 0 || y == 0 || x == 127 || y == 127 {
                assert!((a - b).abs() < 1.0e-3, "cell {i} moved from {b} to {a}");
            }
        }
    }

    #[test]
    fn every_cell_drains_downhill_and_the_ridge_is_dissected() {
        let n = 128;
        let (mut h, r) = ridge(n);
        let smooth = h.clone();
        evolve(&mut h, &r, n, 150.0, 3, &mut |_| {});
        // No closed basins: the evolved surface has nowhere to hold water.
        let mut filled = vec![0.0; n * n];
        crate::priority_flood(&h, n, &mut filled);
        let pits = h.iter().zip(&filled).filter(|(a, b)| **b - **a > 1.0).count();
        assert!(pits < n * n / 200, "{pits} cells sit in depressions");
        // Rivers cut the ridge into spurs and gullies: far more small-scale relief
        // than the smooth plan it started from.
        let roughness = |f: &[f32]| -> f32 {
            let mut sum = 0.0;
            for y in 1..n - 1 {
                for x in 1..n - 1 {
                    let i = y * n + x;
                    sum += (f[i - 1] + f[i + 1] + f[i - n] + f[i + n] - 4.0 * f[i]).abs();
                }
            }
            sum
        };
        assert!(roughness(&h) > roughness(&smooth) * 5.0);
    }
}
