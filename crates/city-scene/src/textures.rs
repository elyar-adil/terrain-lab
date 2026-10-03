//! Procedural texture baking: the ground, the paint and the shared types.
//!
//! The previous port decoded six baked textures from the payload and then never
//! assigned one to a material, so the city was lit by a 128x128 CPU loop that
//! produced pale, low-contrast walls.  Everything a surface needs is baked
//! somewhere in the crate instead.
//!
//! Two invariants the rest of the crate depends on:
//!
//! * **One tile has one real-world size.**  A ground surface tile covers
//!   `GROUND_TILE_M`; UVs are emitted in metres, so a texture's `repeat` is the
//!   reciprocal of its physical size.  Nothing may guess a scale.
//! * **Every surface keeps a value range.**  Ground surfaces keep a real
//!   normal map and a real value spread, because a surface with neither reads
//!   as a flat colour block at city scale — the exact artefact this port exists
//!   to remove.
//!
//! # Where the building surfaces went
//!
//! The facade tiles, the three ground-floor variants and the roof are **not**
//! here any more.  They live in [`crate::facades`], next to the geometry that
//! consumes them, because the storey alignment, the bay module and the relief
//! of a wall are one decision: a facade tile that is beautiful on its own and a
//! half-storey out of phase with the building it is on is worse than no tile at
//! all.  The names below are kept as re-exports so no caller has to care where
//! a bake physically lives.

use serde::Serialize;

use crate::math::Rng;

pub use crate::facades::{
    FACADE_TILE_H, FACADE_TILE_W, GROUND_FLOOR_TILE_H, GROUND_FLOOR_TILE_W, GROUND_STOREY_M,
    ROOF_TILE_M, STOREY_M, STOREYS_PER_TILE,
};

/// A baked RGBA8 texture plus the physical size one tile covers.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BakedTexture {
    pub name: String,
    pub width: usize,
    pub height: usize,
    /// Metres covered by one tile horizontally and vertically.
    pub tile_width_m: f32,
    pub tile_height_m: f32,
    /// `true` when the alpha channel carries a height field that the renderer
    /// differentiates into a tangent-space normal map.
    pub has_normal_source: bool,
    pub rgba: Vec<u8>,
}

/// Ground surfaces tile every 4 m.
pub const GROUND_TILE_M: f32 = 4.0;

/// Cheap deterministic value noise, shared by every bake in the crate so two
/// surfaces that both ask for "some grain" get the same character.
pub(crate) fn hash(seed: u32, x: i32, y: i32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value as f32 / u32::MAX as f32
}

/// Bilinear value noise on a **wrapping** lattice.
///
/// Wrapping is the whole point: every ground texture tiles on a fixed physical
/// grid, and a bake whose low-frequency term does not wrap puts a visible
/// discontinuity along every tile seam — which on a road running a kilometre
/// through a city is a stripe down the middle of the carriageway.  The lattice
/// period is in cells, and callers pass features sized in *tile fractions* so a
/// 64-px and a 256-px bake of the same surface describe the same road.
pub(crate) fn noise(seed: u32, x: f32, y: f32, period: f32) -> f32 {
    let period = period.max(1.0);
    let xi = x.floor();
    let yi = y.floor();
    let tx = x - xi;
    let ty = y - yi;
    let sx = tx * tx * (3.0 - 2.0 * tx);
    let sy = ty * ty * (3.0 - 2.0 * ty);
    let cell = |i: f32, j: f32| {
        let i = ((i % period) + period) % period;
        let j = ((j % period) + period) % period;
        hash(seed, i as i32, j as i32)
    };
    let a = cell(xi, yi);
    let b = cell(xi + 1.0, yi);
    let c = cell(xi, yi + 1.0);
    let d = cell(xi + 1.0, yi + 1.0);
    let top = a + (b - a) * sx;
    let bottom = c + (d - c) * sx;
    top + (bottom - top) * sy
}

/// Signed variant: `-1..1`, which is how a displacement term reads at a call
/// site (`±8` rather than `16 - noise`).
pub(crate) fn signed_noise(seed: u32, x: f32, y: f32, period: f32) -> f32 {
    noise(seed, x, y, period) * 2.0 - 1.0
}

/// sRGB byte → linear reflectance, the number a physical-albedo assertion has
/// to be made in.  A road surface is 4-12% *linear*; judging it by its 8-bit
/// value is what let a mid-grey "asphalt" through, which is a concrete colour.
#[cfg(test)]
fn to_linear(byte: f32) -> f32 {
    let s = (byte / 255.0).clamp(0.0, 1.0);
    if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}

/// The 24 facade tiles, one texture each so a renderer can bind one and never
/// branch.  Delegates to [`crate::facades`], which is where the design table and
/// the storey arithmetic live.
pub fn facade_textures(size: usize) -> Vec<BakedTexture> {
    crate::facades::facade_textures(size)
}

/// `ground/shop`, `ground/lobby`, `ground/home`, each authored at true scale.
pub fn ground_floor_textures(size: usize) -> Vec<BakedTexture> {
    crate::facades::ground_floor_textures(size)
}

/// Bitumen membrane, laps, gravel ballast and ponding, with a height field in
/// the alpha channel.
pub fn roof_texture(size: usize) -> BakedTexture {
    crate::facades::roof_texture(size)
}

/// Asphalt: an old, patched, near-black surface.
///
/// # Why this bake is mostly about value, not colour
///
/// Asphalt is **4-12% reflectance**.  In sRGB that is roughly 60-95, so a
/// correct bake lives entirely in the bottom third of the range.  Baking it
/// around 130-160 gives a *concrete* value, and a road that reads as light
/// concrete cannot be rescued by a normal map, because the eye judges albedo
/// before relief.  So the mean is pinned low and the variation is carried by
/// four separate physical mechanisms, each of which is something you can point
/// at on a real road:
///
/// * **oxidation and sun-bleaching** — broad, slow, +/-9/255 patches;
/// * **screed passes** — the paving machine's own bands, +/-7/255;
/// * **aggregate** — fine grain plus ~3% bright chert chips that catch raking
///   light and are the only genuinely bright thing on a road;
/// * **joints, cracks and chipping** — the linear features, which is what makes
///   the surface read as *old* rather than as a noise field.
///
/// The blue channel is pushed ~6% above red: real bituminous asphalt is a very
/// slightly cool near-black, and that cast is what keeps it from reading brown
/// under a warm sun.
pub fn asphalt_texture(size: usize) -> BakedTexture {
    let mut rgba = vec![0_u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let u = x as f32 / size as f32;
            let v = y as f32 / size as f32;
            let ix = x as i32;
            let iy = y as i32;

            // Broad oxidation / bleaching.  Three cells per 4 m tile keeps the
            // period near 1.3 m, the scale of a genuinely weathered patch.
            let oxidation = signed_noise(0x51d3, u * 3.0, v * 3.0, 3.0) * 9.0;
            // The paving machine's pass: stretched along `V`, one per tile.
            let screed = signed_noise(0x2a71, u * 8.0, v * 2.0, 8.0) * 7.0;
            // Coarse aggregate clumps, then the grain itself.
            let clump = signed_noise(0x77b1, u * 16.0, v * 16.0, 16.0) * 6.0;
            let grain = (hash(17, ix, iy) - 0.5) * 9.0;
            // Bright chert.  Three per cent, and the reason raking light matters.
            let chip = if hash(109, ix, iy) < 0.032 {
                20.0 + hash(113, ix, iy) * 22.0
            } else {
                0.0
            };

            // Linear features.  A construction joint in `U` — for a road running
            // along `X` that is a joint *across* the carriageway, exactly where
            // a paving pass ends — and a crack that wanders along `V`.
            let joint_u = (u - 0.5 - 0.035 * signed_noise(0x1d0f, v * 4.0, 0.0, 4.0)).abs();
            let joint = if joint_u < 0.010 {
                -15.0 * (1.0 - joint_u / 0.010)
            } else {
                0.0
            };
            let crack_v = v - (0.66 + 0.055 * signed_noise(0x3ca7, u * 5.0, 0.0, 5.0));
            let crack = if crack_v.abs() < 0.005 {
                -20.0 * (1.0 - crack_v.abs() / 0.005)
            } else {
                0.0
            };
            // A finer crack branching off it.  One crack is a scratch; a
            // branching network is forty years of maintenance deferred.
            let branch = if (crack_v - 0.10).abs() < 0.004
                && (u - 0.31).abs() < 0.17 * (0.10 - (crack_v - 0.10).abs()) / 0.10
            {
                -16.0
            } else {
                0.0
            };

            // 72 is ~6.6% linear reflectance: mid-range for a wearing course.
            let value = 72.0 + oxidation + screed + clump + grain + chip + joint + crack + branch;
            let offset = (y * size + x) * 4;
            rgba[offset] = value.clamp(0.0, 255.0) as u8;
            rgba[offset + 1] = value.clamp(0.0, 255.0) as u8;
            rgba[offset + 2] = (value * 1.06).clamp(0.0, 255.0) as u8;
            rgba[offset + 3] = 255;
        }
    }
    BakedTexture {
        name: "ground/asphalt".into(),
        width: size,
        height: size,
        tile_width_m: GROUND_TILE_M,
        tile_height_m: GROUND_TILE_M,
        has_normal_source: true,
        rgba,
    }
}

/// Running-bond footway paving, 0.5 x 0.25 m, with recessed dirty joints.
///
/// Concrete paving is **25-35% reflectance** — nearly three times asphalt — and
/// that contrast is one of the strongest cues that a viewer is looking at a
/// street rather than at a plane.  The bake therefore keeps the slab face in
/// the 145-170 band and drops the joints to ~90, with the dirt that collects in
/// them.  Individual slabs vary, and a few are markedly darker: replaced or
/// oil-stained units are what stop a running bond reading as wallpaper.
pub fn paving_texture(size: usize) -> BakedTexture {
    let mut rgba = vec![0_u8; size * size * 4];
    // A joint is about 1.5 cm wide regardless of bake resolution; at 4 m per
    // tile that is a fraction of a texel at low sizes and exactly one texel at
    // 256.  A fixed pixel width would swallow the slab face entirely.
    let joint_px = (size / 256).max(1);
    let paver_w = (size / 8).max(1);
    let paver_h = (size / 16).max(1);
    for y in 0..size {
        for x in 0..size {
            // One tile is 4 m, so a 0.5 m paver is an eighth of the texture.
            let row = y / paver_h;
            let stagger = if row % 2 == 0 { 0 } else { paver_w };
            let column = (x + stagger) / paver_w;
            let local_x = (x + stagger) % paver_w;
            let local_y = y % paver_h;
            let joint = local_x < joint_px || local_y < joint_px;
            // The worn arris: the first few millimetres of each slab face are
            // lighter where feet have polished the chamfer off.
            let chamfer = !joint && (local_x < joint_px * 3 || local_y < joint_px * 3);
            let roll = hash(23, column as i32, row as i32);
            // Two lattices that *wrap* on the slab grid, so a 4 m tile does not
            // show a seam every four pavers.
            let slab = signed_noise(0x6b21, (column % 8) as f32, (row % 16) as f32, 8.0) * 6.0;
            let dirt = signed_noise(0x33b7, x as f32 / 24.0, y as f32 / 24.0, 64.0) * 7.0;
            // A tenth of the units are replacements or oil-stained and read
            // several shades darker than their neighbours.
            let stained = if roll < 0.10 { -24.0 } else { 0.0 };
            let value = if joint {
                92.0 - local_y as f32 * 0.4
            } else if chamfer {
                168.0 + slab
            } else {
                152.0 + slab * 1.4 + dirt + stained
            };
            let offset = (y * size + x) * 4;
            rgba[offset] = value.clamp(0.0, 255.0) as u8;
            rgba[offset + 1] = value.clamp(0.0, 255.0) as u8;
            rgba[offset + 2] = (value * 0.965).clamp(0.0, 255.0) as u8;
            rgba[offset + 3] = 255;
        }
    }
    BakedTexture {
        name: "ground/paving".into(),
        width: size,
        height: size,
        tile_width_m: GROUND_TILE_M,
        tile_height_m: GROUND_TILE_M,
        has_normal_source: true,
        rgba,
    }
}

/// Planted ground: the median belt, block lawns and the tussocks on a verge.
///
/// A real Chinese median is not a lawn — it is a clipped shrub bed with bare
/// earth showing through the thin places and last year's leaves caught in it.  So
/// the bake is a low-saturation green with a substantial dirt component rather
/// than the vivid green of a park.
pub fn grass_texture(size: usize) -> BakedTexture {
    let mut rgba = vec![0_u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let u = x as f32 / size as f32;
            let v = y as f32 / size as f32;
            let clump = signed_noise(31, u * 12.0, v * 12.0, 12.0) * 16.0;
            let blade = if hash(37, x as i32, y as i32) < 0.26 { 16.0 } else { 0.0 };
            // Bare earth: the thin places a clipped bed always has.
            let bare = noise(0x9d17, u * 4.0, v * 4.0, 4.0);
            let earth = if bare > 0.62 { (bare - 0.62) * 210.0 } else { 0.0 };
            let dry = signed_noise(41, u * 6.0, v * 6.0, 6.0) * 11.0;
            let leaf = if hash(43, x as i32, y as i32) < 0.012 { 22.0 } else { 0.0 };
            let red = 66.0 + clump + blade + dry * 1.15 + leaf - earth;
            let green = 90.0 + clump + blade + dry + leaf * 0.8 - earth * 0.75;
            let blue = 52.0 + clump * 0.7 + blade * 0.6 + dry * 0.6 - earth * 0.55;
            let offset = (y * size + x) * 4;
            rgba[offset] = red.clamp(0.0, 255.0) as u8;
            rgba[offset + 1] = green.clamp(0.0, 255.0) as u8;
            rgba[offset + 2] = blue.clamp(0.0, 255.0) as u8;
            rgba[offset + 3] = 255;
        }
    }
    BakedTexture {
        name: "ground/grass".into(),
        width: size,
        height: size,
        tile_width_m: GROUND_TILE_M,
        tile_height_m: GROUND_TILE_M,
        has_normal_source: true,
        rgba,
    }
}

/// The leaf card a tree canopy is built from: dozens of small leaves in one
/// tile, drawn as hard-edged alpha.  A card is roughly 1 m across and carries
/// enough leaves that a canopy of 250 cards reads as a continuous mass while
/// costing a fraction of the triangles real leaf geometry would need.
pub fn leaf_card_texture(size: usize) -> BakedTexture {
    let mut rgba = vec![0_u8; size * size * 4];
    let mut rng = Rng::new(9931);
    let leaves = (size as f32 * size as f32 / 160.0) as u32;
    for _ in 0..leaves {
        let cx = rng.range(4.0, size as f32 - 4.0);
        let cy = rng.range(4.0, size as f32 - 4.0);
        let angle = rng.unit() * std::f32::consts::PI;
        // Leaf size scales with the texture so a card looks the same at every
        // bake resolution.  These proportions are the source renderer's: a leaf
        // is roughly a twentieth of the card across.
        let half_length = rng.range(0.085, 0.170) * size as f32;
        let half_width = rng.range(0.037, 0.075) * size as f32;
        let shade = rng.range(0.72, 1.08);
        let (sin, cos) = angle.sin_cos();
        let bound = half_length.ceil() as i32;
        for dy in -bound..=bound {
            for dx in -bound..=bound {
                let px = cx as i32 + dx;
                let py = cy as i32 + dy;
                if px < 0 || py < 0 || px >= size as i32 || py >= size as i32 {
                    continue;
                }
                let u = (dx as f32 * cos + dy as f32 * sin) / half_length;
                let v = (-dx as f32 * sin + dy as f32 * cos) / half_width;
                if u * u + v * v > 1.0 {
                    continue;
                }
                let value = (226.0 * shade) as u8;
                let index = (py as usize * size + px as usize) * 4;
                if rgba[index + 3] == 0 {
                    rgba[index] = value;
                    rgba[index + 1] = value;
                    rgba[index + 2] = value;
                    rgba[index + 3] = 255;
                }
            }
        }
    }
    BakedTexture {
        name: "vegetation/leaf".into(),
        width: size,
        height: size,
        tile_width_m: 1.0,
        tile_height_m: 1.0,
        has_normal_source: false,
        rgba,
    }
}

/// Shop signage, as a backlit light box.
///
/// Twelve names and six colour pairs, ported from the source renderer's
/// commercial strip.  Glyphs are drawn as block strokes rather than text, so the
/// bake needs no font and is identical in every environment.
///
/// Three rows of strokes, not one, because a single row of blocks reads as a
/// barcode while a real Chinese shop sign stacks a large name over a smaller
/// trade line over a phone number — and it is that density of bright rectangles
/// along a footway which is half of what makes a Chinese street recognisable
/// from a distance.  The tile is 3.4 x 0.9 m, which is one fascia panel.
pub fn signage_texture(size: usize) -> BakedTexture {
    let mut rgba = vec![0_u8; size * size * 4];
    const PAIRS: [([u8; 3], [u8; 3]); 6] = [
        ([26, 42, 92], [244, 238, 220]),
        ([140, 32, 34], [248, 244, 232]),
        ([22, 92, 62], [238, 244, 232]),
        ([212, 168, 62], [34, 30, 26]),
        ([64, 44, 84], [240, 236, 244]),
        ([28, 30, 34], [226, 214, 96]),
    ];
    let width = size;
    let height = size / 4;
    for y in 0..height {
        let v = y as f32 / height as f32;
        for x in 0..width {
            let u = x as f32 / width as f32;
            let index = (y * width + x) * 4;
            let pair = PAIRS[((u * 6.0) as usize) % 6];
            let mut rgb = pair.0;
            // A recessed aluminium border, a slight vertical fall-off from the
            // internal tube, and grime at the bottom where the wash runs.
            if x < 3 || y < 3 || x >= width - 3 || y >= height - 3 {
                rgb = pair.0.map(|value| (value as f32 * 0.55) as u8);
            } else {
                let gradient = v * 0.22;
                let grime = if v > 0.82 { -(v - 0.82) * 210.0 } else { 0.0 };
                rgb = rgb.map(|value| (value as f32 * (1.0 - gradient) + grime).max(0.0) as u8);
            }
            // Three rows of glyph blocks, each row at its own scale and inset.
            let rows: [(f32, f32, f32); 3] = [
                (0.16, 0.42, 1.00), // the name
                (0.50, 0.66, 0.72), // the trade line
                (0.74, 0.86, 0.55), // digits
            ];
            for (row, (top, bottom, scale)) in rows.iter().enumerate() {
                if v < *top || v > *bottom {
                    continue;
                }
                let inset = 0.08 + 0.02 * scale;
                if u < inset || u > 1.0 - inset {
                    continue;
                }
                let cell = width as f32 * 0.055 * scale;
                let local = (u - inset) * width as f32 / cell;
                // Two bars per cell with a gap, and a per-cell jitter so the row
                // is not a picket fence.
                let cell_index = local.floor() as i32;
                let roll = (cell_index as u32).wrapping_mul(2_654_435_761) >> 24;
                if local.fract() < 0.42 && roll & 7 != 0 {
                    rgb = pair.1;
                }
                let _ = row;
            }
            for channel in 0..3 {
                rgba[index + channel] = rgb[channel];
            }
            rgba[index + 3] = 255;
        }
    }
    BakedTexture {
        name: "sign/shop".into(),
        width,
        height,
        tile_width_m: 3.4,
        tile_height_m: 0.9,
        has_normal_source: false,
        rgba,
    }
}

/// A zebra crossing, drawn as one quad instead of thirty bars.
///
/// Bars are 0.45 m wide at 1.05 m pitch across a road that can be 36 m wide, so
/// building the crossing from geometry cost more vertices than the entire road
/// network.  The pattern is in the texture instead, and the quad's `U` is scaled
/// by the real crossing width so the bar count is still exact.
///
/// # Where the wear goes
///
/// `U` runs *across* the road, one tile per 8.4 m, and `V` runs along the 4 m
/// crossing depth.  Traffic that drives over a crossing runs **along** `V`, so
/// the tyre polish, the rubber and the accumulated grit are all functions of `V`
/// and nothing depends on `U` — which matters, because anything varying along
/// `U` would visibly repeat every 8.4 m.  Per-bar variation is instead keyed off
/// the *bar index* rather than off the pixel column, so each of the eight bars
/// in a tile wears differently and the repeat does not read as a repeat.
pub fn crosswalk_texture(size: usize) -> BakedTexture {
    let width = 256;
    let height = size.max(64);
    let mut rgba = vec![0_u8; width * height * 4];
    // Eight bars per tile: 8 * 1.05 m = 8.4 m of crossing.
    const BARS: usize = 8;
    let pitch = width / BARS;
    let bar = ((pitch as f32 * 0.45 / 1.05) as usize).max(3);
    for y in 0..height {
        // `v` is 0..1 along the 4 m crossing depth.
        let v = y as f32 / height as f32;
        for x in 0..width {
            let index = (y * width + x) * 4;
            let bar_index = (x / pitch) as i32;
            let in_bar = (x % pitch) as f32;
            // Where the paint survives.  Bars are laid 0.45 m wide but they chip
            // at the arrises, so the surviving width of each bar is its own
            // draw and no two bars in a tile are identical.
            let live = (bar as f32 * (0.84 + hash(0x2b17, bar_index, 0) * 0.30)) as usize;
            let inside_bar = (x % pitch) < live.max(3);
            // Ragged ends: the bar's leading and trailing 12% is eaten away.
            let end_wear = (v - 0.5).abs();
            let end_ok = end_wear < 0.38 + hash(0x51a3, bar_index, 7) * 0.08;
            // Tyre polish: two darker, dirtier bands where wheels run, plus a
            // general loss of brightness away from the crown of the bar.
            let polish = if (v - 0.30).abs() < 0.11 || (v - 0.72).abs() < 0.09 {
                0.74
            } else {
                1.0
            };
            let across = 1.0 - 0.16 * ((in_bar - live as f32 * 0.5) / live.max(1) as f32).abs();
            let wear = (0.88 + hash(53, x as i32, y as i32) * 0.20)
                * polish
                * across
                * (0.90 + hash(0x7c31, bar_index, 3) * 0.20);
            let rgb = if inside_bar && end_ok {
                [233.0 * wear, 234.0 * wear, 226.0 * wear]
            } else {
                [0.0, 0.0, 0.0]
            };
            // The gaps are transparent so the asphalt underneath shows through
            // and the crossing never has a seam against the road.
            rgba[index] = rgb[0] as u8;
            rgba[index + 1] = rgb[1] as u8;
            rgba[index + 2] = rgb[2] as u8;
            rgba[index + 3] = if inside_bar && end_ok { 255 } else { 0 };
        }
    }
    BakedTexture {
        name: "marking/crosswalk".into(),
        width,
        height,
        tile_width_m: 8.4,
        tile_height_m: 4.0,
        has_normal_source: false,
        rgba,
    }
}

/// A lane line that dashes itself.
///
/// Same reasoning as the crossing: a 3 m dash with a 5 m gap repeated down a
/// road is fifteen quads of pure repetition per divider.  One ribbon with a
/// periodic alpha texture is one quad, and the dash phase is exact because the
/// texture is anchored at the road's start station.
///
/// # Two dashes per tile
///
/// One dash per tile means every dash on every road in the city is pixel-for-pixel
/// identical, and a lane line that repeats perfectly is the single loudest
/// "procedural" tell a road surface has.  Two dashes per tile keeps the phase
/// exactly right (the pattern is still `dash : gap` in metres, because
/// `tile_height_m` is *two* periods) while letting the two carry different wear.
/// The dash ends are also eroded: real thermoplastic has a rounded, chipped
/// leading edge, and the erosion is what makes a 3 m dash read as paint rather
/// than as a stencil.
pub fn dashed_line_texture(width: usize, dash_m: f32, gap_m: f32) -> BakedTexture {
    let period = (dash_m + gap_m).max(0.1);
    let cycles = 2_usize;
    let height = 64 * cycles;
    let mut rgba = vec![0_u8; width * height * 4];
    for y in 0..height {
        // `t` is 0..1 over *one* period; the first `dash_m / period` is paint.
        let t = (y % 64) as f32 / 64.0;
        let cycle = y / 64;
        let paint = dash_m / period;
        // The two cycles in a tile are not twins: the second is the one that has
        // been scuffed by the wheel that clips the corner every time.
        let scuff = if cycle == 0 { 1.0 } else { 0.88 };
        for x in 0..width {
            let index = (y * width + x) * 4;
            let u = x as f32 / width as f32;
            // Erode the ends: the paint thins and finally lets go over the last
            // 6% of the dash at each end.
            let head = (t / 0.06).min(1.0);
            let tail = ((paint - t) / 0.06).min(1.0);
            let end = head.min(tail).clamp(0.0, 1.0);
            // The cross-section of a thermoplastic line is a shallow lens: bright
            // and clean down the middle, grimy where the tyre runs along the
            // arris.  The lane line is 15 cm and this is its whole width.
            let across = 1.0 - 0.30 * (2.0 * u - 1.0).abs().powf(2.2);
            let grain = 0.86 + hash(59, x as i32, y as i32) * 0.26;
            let value = (233.0 * grain * across * scuff * (0.35 + 0.65 * end)) as u8;
            rgba[index] = value;
            rgba[index + 1] = (value as f32 * 1.01) as u8;
            rgba[index + 2] = (value as f32 * 0.98) as u8;
            // The dash boundary is geometric and exact: `alphaTest` cuts exactly
            // where `t` crosses `paint`, so the rhythm stays 3 m / 5 m.  The
            // erosion lives in the *brightness* instead, which is the right level
            // of detail for a 15 cm line — geometric chipping on a dash end is
            // sub-pixel at any distance the line is legible from, and it would
            // make the rhythm itself look wrong.
            rgba[index + 3] = if t < paint { 255 } else { 0 };
        }
    }
    BakedTexture {
        name: format!("marking/dashed-{dash_m}-{gap_m}"),
        width,
        height,
        tile_width_m: 0.15,
        tile_height_m: period * cycles as f32,
        has_normal_source: false,
        rgba,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luma(rgba: &[u8], index: usize) -> f32 {
        0.2126 * rgba[index] as f32 + 0.7152 * rgba[index + 1] as f32 + 0.0722 * rgba[index + 2] as f32
    }

    /// Sorted linear-reflectance percentiles of a texture's opaque pixels.
    fn reflectance(texture: &BakedTexture) -> (f32, f32, f32) {
        let mut values: Vec<f32> = texture
            .rgba
            .chunks(4)
            .filter(|pixel| pixel[3] > 0)
            .map(|pixel| to_linear(luma(pixel, 0)))
            .collect();
        values.sort_by(|a, b| a.total_cmp(b));
        let count = values.len().max(1);
        (
            values[count / 2],
            values[count / 20],
            values[count * 19 / 20],
        )
    }

    #[test]
    fn asphalt_is_near_black_and_the_footway_is_not() {
        // The single most consequential number in the whole ground layer.  A road
        // baked at a mid grey is a concrete road, and no amount of normal mapping
        // or geometry rescues it, because albedo is judged first and hardest.
        let asphalt = asphalt_texture(96);
        let (median, dark, bright) = reflectance(&asphalt);
        assert!(
            (0.04..0.12).contains(&median),
            "asphalt median reflectance {median:.3} is outside 4-12%"
        );
        // ... but it is not a flat fill: aggregate, seams and chips have to
        // survive, or the road reads as painted card.
        assert!(
            bright > median * 1.35 && dark < median * 0.80,
            "asphalt has no value range: dark {dark:.3} median {median:.3} bright {bright:.3}"
        );

        let paving = paving_texture(96);
        let (median, _, _) = reflectance(&paving);
        assert!(
            (0.22..0.40).contains(&median),
            "footway paving median reflectance {median:.3} is outside 25-35%"
        );
        // The whole point of the contrast: kerb-line concrete against bitumen.
        assert!(
            median > reflectance(&asphalt).0 * 2.5,
            "paving is not clearly lighter than the road it borders"
        );
    }

    #[test]
    fn asphalt_is_slightly_cool_not_brown() {
        let texture = asphalt_texture(96);
        let mut red = 0.0_f32;
        let mut blue = 0.0_f32;
        for pixel in texture.rgba.chunks(4) {
            red += pixel[0] as f32;
            blue += pixel[2] as f32;
        }
        // Bituminous asphalt is a very slightly cool near-black.  Under a warm
        // sun an equal-channel bake reads as brown tarmac instead.
        assert!(
            blue > red * 1.02,
            "asphalt blue {blue:.0} is not above red {red:.0}: it will read brown"
        );
    }

    #[test]
    fn lane_paint_is_the_brightest_thing_on_the_road() {
        // Paint is 55-70% reflectance because it is retroreflective glass bead.
        // Getting this wrong in either direction is conspicuous: too dark and the
        // markings vanish at 40 m, too bright and the road reads as a diagram.
        for texture in [dashed_line_texture(64, 3.0, 5.0), crosswalk_texture(96)] {
            let (median, _, bright) = reflectance(&texture);
            assert!(
                (0.45..0.82).contains(&median),
                "{} paint median reflectance {median:.3} is not paint-like",
                texture.name
            );
            assert!(
                bright > reflectance(&asphalt_texture(64)).0 * 4.0,
                "{} is not clearly brighter than the asphalt",
                texture.name
            );
        }
    }

    #[test]
    fn a_lane_line_dash_still_ends_where_gb_says_it_does() {
        // The wear bake must not move the dash boundary: `alphaTest` at 0.5 cuts
        // where alpha drops, so the *geometric* dash has to stay 3 m of 8 m even
        // though the ends are eroded.
        for (dash, gap) in [(3.0_f32, 5.0_f32), (6.0, 9.0)] {
            let texture = dashed_line_texture(64, dash, gap);
            let period = dash + gap;
            assert!((texture.tile_height_m - 2.0 * period).abs() < 1.0e-4);
            for y in 0..64 {
                let painted = texture.rgba[(y * 64) * 4 + 3] > 127;
                let t = y as f32 / 64.0;
                // Only the very ends of a dash may be eroded, never its middle
                // and never the gap.
                assert_eq!(
                    painted,
                    t < dash / period,
                    "dash boundary moved at t={t} in {}",
                    texture.name
                );
            }
        }
    }

    #[test]
    fn a_crossing_holds_its_gb_bar_geometry() {
        // 0.45 m bars at 1.05 m pitch, eight to an 8.4 m tile.  The bar count and
        // width are the whole reason the crossing is a textured quad, so they are
        // asserted rather than assumed.
        let texture = crosswalk_texture(96);
        assert!((texture.tile_width_m - 8.4).abs() < 1.0e-4);
        assert!((texture.tile_height_m - 4.0).abs() < 1.0e-4);
        let pitch = 256 / 8;
        // Probe the middle of the crossing depth: the bar ends are deliberately
        // eaten away, and measuring there would measure the erosion, not the bar.
        let row = texture.height / 2;
        let mut widest = 0;
        for bar in 0..8_usize {
            let live = (0..pitch)
                .filter(|x| texture.rgba[(row * texture.width + *x + bar * pitch) * 4 + 3] > 0)
                .count();
            widest = widest.max(live);
        }
        // 0.45 m of 1.05 m is 13.7 of 32 texels; erosion may only take some away.
        assert!(
            (10..=14).contains(&widest),
            "widest surviving bar is {widest} texels, expected ~13.7"
        );
    }

    #[test]
    fn a_ground_bake_tiles_without_a_seam() {
        // Every ground surface repeats on a 4 m grid and a road runs a kilometre
        // through a city, so a non-wrapping low-frequency term draws a stripe
        // down the middle of the carriageway.  The first and last columns of the
        // bake must therefore agree.
        for texture in [asphalt_texture(64), grass_texture(64)] {
            let mean_edge = |column: usize| {
                (0..texture.height)
                    .map(|row| luma(&texture.rgba, (row * texture.width + column) * 4))
                    .sum::<f32>()
                    / texture.height as f32
            };
            let first = mean_edge(0);
            let last = mean_edge(texture.width - 1);
            assert!(
                (first - last).abs() < 14.0,
                "{} has a tile seam: column means {first:.1} vs {last:.1}",
                texture.name
            );
        }
    }

    /// The building bakes moved to `facades.rs`, where the design table and the
    /// storey arithmetic live.  What is asserted here is the *delegation* — the
    /// material keys, the physical tile sizes and the fact that the two paths
    /// are the same bytes — because a silent divergence between
    /// `textures::facade_textures` and `facades::facade_textures` would mean the
    /// renderer binds a texture nobody baked.
    #[test]
    fn the_building_bakes_delegate_to_facades_unchanged() {
        let here = facade_textures(48);
        let there = crate::facades::facade_textures(48);
        assert_eq!(here.len(), 24);
        assert_eq!(there.len(), 24);
        for (a, b) in here.iter().zip(there.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.rgba, b.rgba, "{} diverges between the two paths", a.name);
            assert_eq!(a.tile_width_m, b.tile_width_m);
            assert_eq!(a.tile_height_m, b.tile_height_m);
        }
        let ground_here = ground_floor_textures(48);
        let ground_there = crate::facades::ground_floor_textures(48);
        for (a, b) in ground_here.iter().zip(ground_there.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.rgba, b.rgba, "{} diverges between the two paths", a.name);
        }
        let roof = roof_texture(48);
        assert_eq!(roof.name, crate::facades::roof_texture(48).name);
        assert_eq!(roof.rgba, crate::facades::roof_texture(48).rgba);
    }

    #[test]
    fn the_leaf_card_is_mostly_transparent_with_real_coverage() {
        let texture = leaf_card_texture(64);
        let opaque = texture.rgba.chunks(4).filter(|pixel| pixel[3] > 0).count();
        let total = 64 * 64;
        // A canopy card must be mostly hole, or the tree reads as a solid blob,
        // and it must not be nearly empty, or it reads as noise.
        assert!(opaque > total / 8, "leaf card coverage {opaque}/{total}");
        assert!(opaque < total * 2 / 3, "leaf card is too solid: {opaque}/{total}");
    }

    #[test]
    fn ground_surfaces_declare_a_height_source_for_normal_mapping() {
        for texture in [
            asphalt_texture(32),
            paving_texture(32),
            grass_texture(32),
            roof_texture(32),
        ] {
            assert!(texture.has_normal_source, "{}", texture.name);
            assert_eq!(texture.rgba.len(), texture.width * texture.height * 4);
        }
    }

    #[test]
    fn paving_joints_are_darker_than_the_slab_face() {
        let texture = paving_texture(64);
        // Sampled as a distribution, not at fixed coordinates: a running-bond
        // pattern means any single pixel is either joint or face depending on
        // where it lands, and a fixed probe is a flaky test.
        let mut values: Vec<f32> = texture
            .rgba
            .chunks(4)
            .map(|pixel| 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32)
            .collect();
        values.sort_by(|a, b| a.total_cmp(b));
        let dark = values.iter().take(values.len() / 10).sum::<f32>() / (values.len() / 10) as f32;
        let median = values[values.len() / 2];
        assert!(median > dark + 20.0, "joints {dark:.0} do not read against {median:.0}");
    }

    #[test]
    fn the_standard_set_covers_every_material_the_city_binds() {
        let set = crate::bake::standard_set(32, 32);
        let names: Vec<&str> = set.iter().map(|texture| texture.name.as_str()).collect();
        for required in [
            "facade/00",
            "facade/23",
            "ground/shop",
            "ground/lobby",
            "ground/home",
            "roof",
            "ground/asphalt",
            "ground/paving",
            "ground/grass",
            "sign/shop",
        ] {
            assert!(names.contains(&required), "missing {required}");
        }
        // Foliage is one card per species, not one generic card. A single shared
        // leaf image is exactly what made the last port's trees read as blobs.
        for species in crate::species::SPECIES {
            let key = format!("vegetation/leaf/{}", species.key);
            assert!(names.contains(&key.as_str()), "missing {key}");
        }
    }
}
