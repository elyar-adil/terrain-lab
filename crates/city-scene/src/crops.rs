//! Farmland: what a field looks like from a few metres up, and from a hill.
//!
//! A field is a polygon laid on the ground with a crop texture whose rows run
//! the way the farmer ploughed. The texture is baked in metres, so a row of wheat is
//! the width a row of wheat is and the field reads the same at every size. Each
//! crop has three looks (a stage of growth, a variety), picked by the plot, so
//! neighbouring strips of the same crop do not match.

use urban::CropKind;

use crate::textures::{BakedTexture, hash};

/// Metres one tile covers. Rows run along the tile's horizontal axis.
pub const CROP_TILE_M: f32 = 4.0;

/// Every crop a field can be sown with.
pub const ALL_CROPS: [CropKind; 7] = [
    CropKind::Wheat,
    CropKind::Rice,
    CropKind::Rapeseed,
    CropKind::Corn,
    CropKind::Vegetables,
    CropKind::Fallow,
    CropKind::Orchard,
];

pub fn crop_key(crop: CropKind, variant: u8) -> String {
    format!("field/{}.{}", crop_name(crop), variant.min(2))
}

fn crop_name(crop: CropKind) -> &'static str {
    match crop {
        CropKind::Wheat => "wheat",
        CropKind::Rice => "rice",
        CropKind::Rapeseed => "rapeseed",
        CropKind::Corn => "corn",
        CropKind::Vegetables => "vegetables",
        CropKind::Fallow => "fallow",
        CropKind::Orchard => "orchard",
    }
}

/// A value noise that tiles over `period` cells, so the texture repeats without a seam.
fn tiled_noise(seed: u32, x: f32, y: f32, period: i32) -> f32 {
    let (x0, y0) = (x.floor(), y.floor());
    let (fx, fy) = (x - x0, y - y0);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let w = |i: i32| i.rem_euclid(period);
    let (ix, iy) = (x0 as i32, y0 as i32);
    let n00 = hash(seed, w(ix), w(iy));
    let n10 = hash(seed, w(ix + 1), w(iy));
    let n01 = hash(seed, w(ix), w(iy + 1));
    let n11 = hash(seed, w(ix + 1), w(iy + 1));
    let a = n00 + (n10 - n00) * sx;
    let b = n01 + (n11 - n01) * sx;
    a + (b - a) * sy
}

fn smooth01(x: f32) -> f32 {
    let t = x.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn srgb8(linear: f32) -> u8 {
    let c = linear.clamp(0.0, 1.0);
    let encoded = if c <= 0.003_130_8 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
    (encoded * 255.0 + 0.5) as u8
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

fn scale(a: [f32; 3], k: f32) -> [f32; 3] {
    [a[0] * k, a[1] * k, a[2] * k]
}

const SOIL: [f32; 3] = [0.105, 0.072, 0.050];
const SOIL_WET: [f32; 3] = [0.060, 0.042, 0.032];

/// One texel of a crop: reflectance and relief. `u` runs along the rows, `v` across
/// them, both in metres of ground and tiling over `CROP_TILE_M`.
fn crop_texel(crop: CropKind, variant: u8, u: f32, v: f32, x: i32, y: i32, n: i32) -> ([f32; 3], f32) {
    let grain = |salt: u32| hash(salt, x, y);
    // Slow variation across the plot: where it was watered, where it lodged.
    let patch = tiled_noise(11 + variant as u32, u * 0.8, v * 0.8, 3);
    let fine = tiled_noise(13, u * 6.0, v * 6.0, 24);
    let _ = n;
    match crop {
        CropKind::Wheat => {
            // Drilled rows 0.20 m apart; the crop fills most of the row.
            let spacing = 0.20;
            let phase = (v / spacing).fract();
            let row = (-(((phase - 0.5).abs() - 0.34) * 14.0)).clamp(0.0, 1.0);
            let green = [[0.050, 0.130, 0.035], [0.090, 0.170, 0.045], [0.300, 0.235, 0.085]][variant.min(2) as usize];
            let leaf = 0.80 + 0.40 * grain(21) + 0.18 * (fine - 0.5);
            let colour = mix(SOIL, scale(green, leaf * (0.88 + 0.24 * patch)), 0.12 + 0.88 * row);
            (colour, row * 0.7 + grain(23) * 0.2)
        }
        CropKind::Rice => {
            let hill = 0.25;
            let (pu, pv) = ((u / hill).fract() - 0.5, (v / (4.0 / 14.0)).fract() - 0.5);
            let plant = (-((pu * pu * 4.0 + pv * pv * 4.0).sqrt() - 0.42) * 7.0).clamp(0.0, 1.0);
            match variant {
                0 => {
                    // Flooded after transplanting: sky in the water, seedlings in rows.
                    let water = mix([0.075, 0.095, 0.105], [0.14, 0.17, 0.19], patch);
                    (mix(water, [0.09, 0.19, 0.05], plant * 0.85), 0.2 + plant * 0.5)
                }
                1 => {
                    let green = [0.065, 0.185, 0.035];
                    let c = mix(scale(SOIL_WET, 1.0), scale(green, 0.8 + 0.4 * grain(25) + 0.2 * (fine - 0.5)), 0.55 + 0.45 * plant);
                    (c, plant * 0.6)
                }
                _ => {
                    let gold = [0.30, 0.235, 0.065];
                    let c = mix(SOIL_WET, scale(gold, 0.78 + 0.4 * grain(27) + 0.2 * patch), 0.6 + 0.4 * plant);
                    (c, plant * 0.6)
                }
            }
        }
        CropKind::Rapeseed => {
            // Broadcast, not drilled: flowers in a continuous sheet, rows only faintly.
            let rows = 0.5 + 0.5 * (std::f32::consts::TAU * v / 0.5).cos();
            match variant {
                0 => {
                    let flower = 0.55 + 0.45 * grain(31);
                    let c = mix([0.07, 0.17, 0.035], [0.62, 0.50, 0.025], (0.55 + 0.35 * patch + 0.25 * (fine - 0.5)) * flower);
                    (scale(c, 0.9 + 0.1 * rows), 0.5)
                }
                1 => (mix([0.10, 0.18, 0.04], [0.45, 0.38, 0.03], 0.35 + 0.4 * patch + 0.3 * grain(33)), 0.4),
                _ => (scale([0.075, 0.150, 0.045], 0.8 + 0.4 * grain(35) + 0.3 * (fine - 0.5)), 0.4),
            }
        }
        CropKind::Corn => {
            // Rows 0.6 m apart, plants every 0.25 m, soil between.
            let spacing = 4.0 / 7.0;
            let phase = (v / spacing).fract();
            let band = (-(((phase - 0.5).abs() - 0.26) * 10.0)).clamp(0.0, 1.0);
            let plant = ((u / 0.25).fract() - 0.5).abs();
            let blades = band * (0.55 + 0.45 * (1.0 - plant * 1.5).clamp(0.0, 1.0));
            let green = if variant == 2 { [0.20, 0.17, 0.065] } else if variant == 1 { [0.075, 0.165, 0.040] } else { [0.055, 0.140, 0.035] };
            let colour = mix(SOIL, scale(green, 0.78 + 0.4 * grain(37) + 0.2 * patch), (0.10 + 0.90 * blades).clamp(0.0, 1.0));
            (colour, blades * 0.8)
        }
        CropKind::Vegetables => {
            // Raised beds with a furrow between (three beds to the 4 m tile), plants in rows
            // 1/3 m apart and 1/4 m along: every period divides the tile, so it repeats
            // without a seam. Each plant is a round tuft of its own size and tone.
            let bed = 4.0 / 3.0;
            let phase = (v / bed).fract();
            let on_bed = smooth01((0.5 - (phase - 0.5).abs()) / 0.16);
            let furrow = 1.0 - on_bed;
            let (pu, pv) = (0.25_f32, 4.0 / 12.0);
            let (iu, iv) = ((u / pu).floor(), (v / pv).floor());
            let (lu, lv) = ((u / pu).fract() - 0.5, (v / pv).fract() - 0.5);
            let (cu, cv) = (iu as i32 % 16, iv as i32 % 12);
            let size = 0.42 + 0.30 * hash(41, cu, cv);
            let tone = 0.75 + 0.5 * hash(43, cu, cv);
            let d = ((lu * pu).powi(2) + (lv * pv).powi(2)).sqrt() / (0.5 * pu.min(pv)) / size.max(0.2);
            // Lobed edge so a tuft is not a disc.
            let ang = (lv * pv).atan2(lu * pu);
            let lobes = 1.0 + 0.22 * (ang * 5.0 + hash(47, cu, cv) * 6.28).sin();
            let blob = smooth01((1.0 - d / lobes) / 0.35);
            let bed_look = match variant {
                0 => {
                    let leafy = scale([0.085, 0.215, 0.050], tone * (0.85 + 0.3 * (fine - 0.5)));
                    (mix(scale(SOIL, 0.9), leafy, on_bed * blob), on_bed * (0.25 + 0.6 * blob))
                }
                1 => {
                    // Black mulch film with the crop pushing through at intervals.
                    let film = scale([0.020, 0.022, 0.026], 1.0 + 0.6 * (fine - 0.5));
                    let sprout = smooth01((1.0 - d / lobes * 1.9) / 0.4);
                    let c = mix(film, scale([0.08, 0.19, 0.05], tone), sprout);
                    (mix(scale(SOIL, 0.9), c, on_bed), on_bed * (0.2 + 0.5 * sprout))
                }
                _ => {
                    let brassica = scale([0.075, 0.170, 0.100], tone * (0.85 + 0.3 * (fine - 0.5)));
                    (mix(scale(SOIL, 0.9), brassica, on_bed * blob), on_bed * (0.25 + 0.7 * blob))
                }
            };
            bed_look.pipe_furrow(furrow * 0.7)
        }
        CropKind::Fallow => {
            match variant {
                0 => {
                    // Ploughed: furrows 0.35 m, lit on one side.
                    let ridge = 0.5 + 0.5 * (std::f32::consts::TAU * v / (4.0 / 12.0)).sin();
                    (scale(mix(SOIL, [0.17, 0.115, 0.075], 0.4 * patch), 0.65 + 0.55 * ridge * (0.7 + 0.5 * grain(43))), ridge)
                }
                1 => {
                    // Gone to weed.
                    let weed = (0.45 + 0.55 * (patch + 0.5 * (fine - 0.5)).clamp(0.0, 1.0)).clamp(0.0, 1.0);
                    (mix(SOIL, scale([0.10, 0.15, 0.045], 0.8 + 0.5 * grain(45)), weed), 0.3 * weed)
                }
                _ => {
                    // Stubble and straw.
                    let straw = [0.27, 0.215, 0.105];
                    let lines = (std::f32::consts::TAU * v / 0.2).sin() * 0.5 + 0.5;
                    (mix(scale(SOIL, 1.2), scale(straw, 0.7 + 0.4 * grain(47) + 0.25 * patch), 0.55 + 0.35 * lines), 0.4 * lines)
                }
            }
        }
        CropKind::Orchard => {
            // The floor under the trees: grass, with a mown strip down the middle of each alley.
            let alley = (v / 4.0).fract();
            let strip = ((alley - 0.5).abs() < 0.22) as i32 as f32;
            let grass = scale([0.075, 0.125, 0.040], 0.75 + 0.5 * grain(49) + 0.3 * (fine - 0.5));
            let bare = mix(SOIL, [0.13, 0.10, 0.07], patch);
            let c = if variant == 1 { mix(bare, grass, strip * 0.6 + 0.2) } else { mix(grass, scale(grass, 1.15), strip) };
            (c, 0.2)
        }
    }
}

/// Darken the furrow between vegetable beds: the bed shapes already carry it, the
/// bare soil in the furrow stays soil.
trait PipeFurrow {
    fn pipe_furrow(self, furrow: f32) -> ([f32; 3], f32);
}

impl PipeFurrow for ([f32; 3], f32) {
    fn pipe_furrow(self, furrow: f32) -> ([f32; 3], f32) {
        (mix(self.0, scale(SOIL_WET, 1.1), furrow * 0.7), self.1 * (1.0 - furrow * 0.8))
    }
}

pub fn crop_texture(crop: CropKind, variant: u8, size: usize) -> BakedTexture {
    let n = size.max(32);
    let mut rgba = vec![0_u8; n * n * 4];
    for y in 0..n {
        for x in 0..n {
            let u = (x as f32 + 0.5) / n as f32 * CROP_TILE_M;
            let v = (y as f32 + 0.5) / n as f32 * CROP_TILE_M;
            let (c, relief) = crop_texel(crop, variant, u, v, x as i32, y as i32, n as i32);
            let offset = (y * n + x) * 4;
            rgba[offset] = srgb8(c[0]);
            rgba[offset + 1] = srgb8(c[1]);
            rgba[offset + 2] = srgb8(c[2]);
            rgba[offset + 3] = srgb8(relief.clamp(0.0, 1.0));
        }
    }
    BakedTexture {
        name: crop_key(crop, variant),
        width: n,
        height: n,
        tile_width_m: CROP_TILE_M,
        tile_height_m: CROP_TILE_M,
        has_normal_source: true,
        rgba,
    }
}

/// Every crop in every look.
pub fn crop_textures(size: usize) -> Vec<BakedTexture> {
    let mut set = Vec::new();
    for crop in ALL_CROPS {
        for variant in 0..3_u8 {
            set.push(crop_texture(crop, variant, size));
        }
    }
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_crop_look_bakes_a_texture_of_its_own() {
        let set = crop_textures(64);
        assert_eq!(set.len(), 21);
        let mut names: Vec<_> = set.iter().map(|t| t.name.clone()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 21);
        // No two looks of one crop are the same picture.
        for crop in ALL_CROPS {
            let looks: Vec<_> = (0..3).map(|v| crop_texture(crop, v, 64).rgba).collect();
            assert_ne!(looks[0], looks[1], "{crop:?}");
            assert_ne!(looks[1], looks[2], "{crop:?}");
        }
    }

    #[test]
    fn crops_are_not_the_colour_of_a_lawn_or_of_a_road() {
        // A field is a mid-value surface: not black, not white.
        for texture in crop_textures(64) {
            let mean: f32 = texture.rgba.chunks_exact(4).map(|p| (p[0] as f32 + p[1] as f32 + p[2] as f32) / 3.0).sum::<f32>()
                / (texture.width * texture.height) as f32;
            assert!(mean > 30.0 && mean < 190.0, "{} has mean {mean}", texture.name);
        }
    }
}
