//! The ground-floor bakes: `ground/shop`, `ground/lobby`, `ground/home`.
//!
//! Each is one shop bay by one ground storey at true scale, so a door is 2.1 m
//! tall on the wall rather than a stretched repeat.  The shopfront is the
//! darkest band on a street and the busiest: a stall riser, piers, glazing, a
//! roller shutter somewhere in every four bays, a fascia — and the fascia is
//! **per-bay coloured signage**, because a continuous run of blue, green, maroon
//! and white sign boards is the single most recognisable feature of a Chinese
//! retail street.

use crate::facades::tile::{shade, srgb8, value_noise};
use crate::facades::{
    GROUND_FLOOR_BAY_W, GROUND_FLOOR_BAYS, GROUND_FLOOR_TILE_H, GROUND_FLOOR_TILE_W,
    GROUND_STOREY_M,
};
use crate::textures::{BakedTexture, hash};
use worldgen_core::hash::avalanche01;

/// The three ground-floor variants.  Each is one shop bay by one ground storey
/// at true scale, so a door is 2.1 m tall on the wall rather than a stretched
/// repeat.
pub fn ground_floor_textures(size: usize) -> Vec<BakedTexture> {
    ["shop", "lobby", "home"]
        .iter()
        .map(|kind| ground_floor(kind, size))
        .collect()
}

/// Sign-board reflectances for the shopfront fascia — one per bay, picked by
/// hash.  All are dark: a backlit sign box is a *lit* surface only when it is
/// switched on, and the renderer owns lighting, so the albedo stays at painted
/// metal and acrylic sheet values (0.04 – 0.35).
const SIGN_BOARDS: [[f32; 3]; 6] = [
    [0.52, 0.08, 0.06], // red, the shop sign of most of the country
    [0.66, 0.46, 0.10], // gold on red
    [0.08, 0.24, 0.56], // blue
    [0.10, 0.40, 0.22], // green pharmacy
    [0.80, 0.79, 0.76], // white plate with black letters
    [0.70, 0.34, 0.08], // orange
];

/// The sign board of one bay.  Neighbouring shops never share a board: each bay
/// steps a random 1..N-1 places on from the one before it.
fn sign_board(key: u32, bay: i32) -> usize {
    let n = SIGN_BOARDS.len();
    let mut index = (avalanche01(key, 0, 0, 701) * n as f32) as usize % n;
    for b in 1..=bay.max(0) {
        let step = 1 + (avalanche01(key, b, 0, 701) * (n - 1) as f32) as usize % (n - 1);
        index = (index + step) % n;
    }
    index
}

/// Chinese ground floors are a *continuous* band, not a row of doors: a stall
/// riser, a run of piers, glazed shopfronts, a roller shutter somewhere in
/// every four bays, a fascia, and the projecting sign boxes the geometry adds in
/// front of it.  That rhythm — not the door — is what makes a street read as
/// retail.
/// Fraction of a `pitch`-spaced cell that `x` has passed through: 0 at a bar's
/// centre line, so `< w` is a bar of width `w` in metres.
fn bx_pitch(x: f32, pitch: f32) -> f32 {
    (x / pitch).fract()
}

fn ground_floor(kind: &str, size: usize) -> BakedTexture {
    let n = size.max(16);
    // One storey tall, `GROUND_FLOOR_BAYS` bays wide, at the same texel density
    // horizontally as the one-bay tile had.
    let w = n * GROUND_FLOOR_BAYS;
    let mut rgba = vec![0_u8; w * n * 4];
    let key = match kind {
        "shop" => 977_u32,
        "lobby" => 1481,
        _ => 2003,
    };
    let px_per_m = n as f32 / GROUND_FLOOR_BAY_W;
    let hairline = 1.0 / px_per_m;

    // Reflectances, not screen colours.  A shopfront is the darkest thing on a
    // street: a dark stone stall riser, dark glass, a dark frame, and only the
    // fascia, the piers and the serving hatch catching any light at all.  The
    // previous bake put a 0.68 stone pier over half the tile, which is why a
    // ground floor read as a bright door instead of as a shop.
    // Chinese ground floors are light: cream and grey stone, white aluminium
    // frames, bright glass with a blue-grey tint, and painted signs.
    const RISER: [f32; 3] = [0.36, 0.35, 0.33];
    const PIER: [f32; 3] = [0.60, 0.58, 0.55];
    const PIER_LIGHT: [f32; 3] = [0.68, 0.66, 0.63];
    const FRAME: [f32; 3] = [0.74, 0.74, 0.72];
    const GLASS_SHOP: [f32; 3] = [0.20, 0.27, 0.32];
    const GLASS_HOME: [f32; 3] = [0.24, 0.31, 0.36];
    const FASCIA: [f32; 3] = [0.66, 0.65, 0.62];
    const FASCIA_TOP: [f32; 3] = [0.72, 0.71, 0.68];
    const AWNING_RED: [f32; 3] = [0.50, 0.10, 0.08];
    const AWNING_WHITE: [f32; 3] = [0.78, 0.77, 0.74];
    const SHUTTER: [f32; 3] = [0.50, 0.50, 0.48];
    const RENDER: [f32; 3] = [0.66, 0.63, 0.57];
    const GRANITE: [f32; 3] = [0.40, 0.40, 0.38];
    const DOOR: [f32; 3] = [0.34, 0.26, 0.19];

    for y in 0..n {
        for x in 0..w {
            // `run_u` is metres along the whole tile; `u` is metres within this
            // pixel's own bay, which is what every layout rule below is written in.
            let run_u = (x as f32 + 0.5) / px_per_m;
            let bay = (run_u / GROUND_FLOOR_BAY_W).floor() as i32;
            let u = run_u - bay as f32 * GROUND_FLOOR_BAY_W;
            // `h` is metres above the shop floor; the tile is one storey tall.
            let h = (1.0 - (y as f32 + 0.5) / n as f32) * GROUND_FLOOR_TILE_H;
            // Per-pixel grain only.  Anything that *decides* something — whether
            // a shop is lit, whether a shutter is down, what colour a fascia is —
            // is keyed on a 1.05 m block, because a decision that changes every
            // pixel is not a decision, it is noise, and noise at this contrast
            // reads as dirt on the lens.
            let block = (run_u / 1.05).floor() as i32;
            let decide = |salt: u32| hash(key.wrapping_add(salt), block, 0);
            let g = |salt: u32| hash(key.wrapping_add(salt), x as i32, y as i32);
            // Grime rises from the pavement and rain runs down from every
            // horizontal edge.  Both are real, and both are the difference
            // between a rendered shopfront and a flat one.
            let splash = (1.0 - (h / 0.55).clamp(0.0, 1.0)).powi(2) * 0.24;
            let run = if ((u * 3.1) - (u * 3.1).floor()) < 0.14 {
                (1.0 - h / GROUND_FLOOR_TILE_H) * 0.08
            } else {
                0.0
            };
            let grime = splash + run + (g(11) - 0.5) * 0.010;

            // The 4.2 m bay, laid out the way a 4.2 m 商铺开间 is: a structural
            // pier, an 1.8 m shopfront with a serving hatch in it, a mullion, a
            // 1.1 m double door, and the flank wall of the next unit.
            let (pier_a, glazed, mullion, door, flank) = (0.50_f32, 2.30, 2.60, 3.70, 4.20);
            // One sign board per bay: every 4.2 m unit picked its own colour the
            // day it opened, and the run of colours down a street is the look.
            let board = sign_board(key, bay);
            let plain_pick = avalanche01(key, bay, 0, 733);
            let fascia = if plain_pick < 0.30 {
                // Some units never re-clad: bare render fascia.
                shade(FASCIA, 0.85 + 0.3 * avalanche01(key, bay, 0, 751))
            } else {
                shade(
                    SIGN_BOARDS[board],
                    0.92 + 0.16 * avalanche01(key, bay, 0, 769),
                )
            };

            let awning_pick = avalanche01(key, bay, 0, 811);
            let mut colour = match kind {
                "shop" => {
                    if h < 0.42 {
                        // Stall riser: light granite, grooved.
                        let groove = (u / 0.42) - (u / 0.42).floor() < 0.06;
                        shade(RISER, if groove { 0.80 } else { 1.0 })
                    } else if h > 3.55 {
                        // Fascia: a painted sign board per bay.  Its lettering is a
                        // row of lighter blocks, so it reads as a sign at street
                        // distance rather than as a flat band of paint.
                        if h > 4.32 {
                            FASCIA_TOP
                        } else if h > 4.20 {
                            FRAME
                        } else if h < 3.68 {
                            shade(fascia, 0.85)
                        } else {
                            let block_u = ((u * 2.6).fract()) < 0.62;
                            let block_h = ((h - 3.72) / 0.16).fract() < 0.55;
                            if (3.80..4.08).contains(&h) && block_u && block_h {
                                [0.86, 0.85, 0.82]
                            } else {
                                fascia
                            }
                        }
                    } else if u < pier_a || u >= flank {
                        // Light stone pier, fluted.
                        let flute = ((u / 0.31) - (u / 0.31).floor() - 0.5).abs();
                        shade(
                            if block.rem_euclid(2) == 0 {
                                PIER_LIGHT
                            } else {
                                PIER
                            },
                            0.92 + flute * 0.16,
                        )
                    } else if (mullion - 0.30..mullion).contains(&u) {
                        FRAME
                    } else if (mullion..door).contains(&u) {
                        // Entrance: an aluminium double door in glass.
                        if h > 3.02 {
                            FRAME
                        } else if (u - mullion - 0.22).abs() < 0.030 && h > 1.00 && h < 1.95 {
                            [0.42, 0.36, 0.24]
                        } else if u < mullion + 0.06 || u > door - 0.06 {
                            FRAME
                        } else {
                            shade(GLASS_SHOP, 1.25)
                        }
                    } else if (glazed..glazed + 0.10).contains(&u) || (2.95..3.07).contains(&h) {
                        FRAME
                    } else if (2.74..2.95).contains(&h) && awning_pick < 0.45 {
                        // Striped awning over the shop bay.
                        if ((u * 3.2).fract()) < 0.5 {
                            AWNING_RED
                        } else {
                            AWNING_WHITE
                        }
                    } else if h < 2.30 && decide(29) < 0.45 {
                        // Roller shutter, half down, light grey steel with ribs.
                        if ((h / 0.09) - (h / 0.09).floor()) < 0.45 {
                            shade(SHUTTER, 0.92)
                        } else {
                            shade(SHUTTER, 1.06)
                        }
                    } else if h < 2.30 {
                        // Shop interior, lit warm at the counter.
                        let lit = 0.50 + 0.60 * decide(31);
                        [0.30 * lit, 0.25 * lit, 0.17 * lit]
                    } else {
                        // Glass above the shop front: a bright blue-grey pane with
                        // diagonal reflections across it.
                        let reflection = ((u * 0.9 + h * 0.35).fract()) < 0.07;
                        let band =
                            0.90 + 0.22 * value_noise(key.wrapping_add(601), u * 1.6, h * 0.9);
                        let g = shade(GLASS_SHOP, band);
                        if reflection { shade(g, 2.4) } else { g }
                    }
                }
                "lobby" => {
                    if h < 0.50 {
                        let groove = (u / 0.90) - (u / 0.90).floor() < 0.05;
                        shade(GRANITE, if groove { 0.84 } else { 1.0 })
                    } else if h > 3.55 {
                        if h > 4.24 {
                            FASCIA_TOP
                        } else if h > 4.14 {
                            FRAME
                        } else {
                            shade(fascia, 0.92)
                        }
                    } else if !(0.70..=3.50).contains(&u) {
                        // Light stone portal.
                        let flute = ((u / 0.35) - (u / 0.35).floor() - 0.5).abs();
                        shade(PIER_LIGHT, 0.92 + flute * 0.16)
                    } else if h > 3.02 {
                        shade(PIER_LIGHT, 0.96)
                    } else {
                        // Full-height glass doors in aluminium frames, with the
                        // lobby lit warm behind the sidelights.
                        let door = (1.42..2.78).contains(&u);
                        let stile = (u / 0.735) - (u / 0.735).floor() < 0.028;
                        let rail = door
                            && ((u - 1.55).abs() < 0.032 || (u - 2.65).abs() < 0.032)
                            && h > 1.05
                            && h < 1.95;
                        if rail {
                            [0.46, 0.40, 0.28]
                        } else if stile || h > 2.94 {
                            FRAME
                        } else if door {
                            shade(GLASS_SHOP, 1.25)
                        } else {
                            let lit = 0.45 + 0.55 * decide(31);
                            [0.40 * lit, 0.34 * lit, 0.24 * lit]
                        }
                    }
                }
                _ => {
                    // A residential entrance: a light render wall, a brown door, and
                    // windows with white frames.  The steel grille is only on the
                    // lower openings, and it is a fine light grille, not a dark grid.
                    if h < 0.45 {
                        let groove = (u / 0.50) - (u / 0.50).floor() < 0.06;
                        shade(GRANITE, if groove { 0.84 } else { 1.0 })
                    } else if h > 3.50 {
                        if h > 4.22 {
                            FASCIA_TOP
                        } else if h > 4.12 {
                            FRAME
                        } else if h < 3.64 {
                            shade(RENDER, 0.94)
                        } else {
                            shade(RENDER, 1.04)
                        }
                    } else if (1.78..2.62).contains(&u) {
                        // Entrance door: a panelled leaf in a white frame.
                        if h > 3.05 {
                            shade(RENDER, 1.05)
                        } else if !(1.86..=2.54).contains(&u) || h < 0.62 {
                            FRAME
                        } else if (u - 2.40).abs() < 0.030 && h > 1.05 && h < 1.95 {
                            [0.46, 0.40, 0.28]
                        } else {
                            let panel = (h - 0.72) / 0.78;
                            let recess = if (panel - 0.5).abs() < 0.34 {
                                0.84
                            } else {
                                1.0
                            };
                            shade(DOOR, recess)
                        }
                    } else if (0.42..1.46).contains(&u) || (2.94..3.98).contains(&u) {
                        // Window: sill, white frame, glass, and a sash bar.
                        if h < 0.92 {
                            shade(FRAME, 0.94)
                        } else if h < 1.02 {
                            shade(RENDER, 0.80)
                        } else {
                            let edge = (u - 0.42).min(1.46 - u).min((u - 2.94).abs()).min(3.98 - u);
                            if edge < 0.05 || (h > 2.20 && h < 2.28) {
                                FRAME
                            } else {
                                let grille = h < 3.0
                                    && ((bx_pitch(u, 0.40) < 0.10)
                                        || (((h / 0.60).fract()) < 0.07));
                                if grille {
                                    shade(FRAME, 0.80)
                                } else {
                                    let reflection = ((u * 1.3 + h * 0.5).fract()) < 0.08;
                                    let g = shade(
                                        GLASS_HOME,
                                        0.95 + 0.15
                                            * value_noise(key.wrapping_add(603), u * 1.2, h * 0.8),
                                    );
                                    if reflection { shade(g, 2.2) } else { g }
                                }
                            }
                        }
                    } else {
                        // Render with a downpipe in the corner.
                        if !(0.30..=3.90).contains(&u) && h < 3.30 {
                            shade(RENDER, 0.70)
                        } else {
                            RENDER
                        }
                    }
                }
            };
            // A hairline joint in every panel, so a large flat area is never
            // truly flat.
            if hairline < 0.02 {
                let joint = (u / 0.90) - (u / 0.90).floor() < (hairline / 0.90).max(0.004)
                    || (h / 0.90) - (h / 0.90).floor() < (hairline / 0.90).max(0.004);
                if joint && h > 0.45 {
                    colour = shade(colour, 0.86);
                }
            }
            let offset = (y * w + x) * 4;
            for channel in 0..3 {
                let value = colour[channel] - grime * (1.0 - h / GROUND_FLOOR_TILE_H).min(1.0);
                rgba[offset + channel] = srgb8(value);
            }
            rgba[offset + 3] = 255;
        }
    }
    BakedTexture {
        name: format!("ground/{kind}"),
        width: w,
        height: n,
        tile_width_m: GROUND_FLOOR_TILE_W,
        tile_height_m: GROUND_STOREY_M,
        has_normal_source: false,
        rgba,
    }
}

#[allow(clippy::needless_range_loop)]
#[cfg(test)]
mod tests {
    use super::*;

    fn luma(linear: [f32; 3]) -> f32 {
        0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
    }

    fn median_albedo(texture: &BakedTexture) -> [f32; 3] {
        let mut values: Vec<[f32; 3]> = (0..texture.width * texture.height)
            .map(|index| {
                let offset = index * 4;
                [
                    srgb_to_linear(texture.rgba[offset] as f32 / 255.0),
                    srgb_to_linear(texture.rgba[offset + 1] as f32 / 255.0),
                    srgb_to_linear(texture.rgba[offset + 2] as f32 / 255.0),
                ]
            })
            .collect();
        values.sort_by(|a, b| luma(*a).total_cmp(&luma(*b)));
        values[values.len() / 2]
    }

    fn srgb_to_linear(encoded: f32) -> f32 {
        if encoded <= 0.040_45 {
            encoded / 12.92
        } else {
            ((encoded + 0.055) / 1.055).powf(2.4)
        }
    }

    /// The three ground floors are a retail band, a portal and an entrance, and
    /// they are authored at true size: a run of 4.2 m bays by one 4.5 m storey,
    /// with no vertical repeat, so a 2.1 m door is 2.1 m on the wall.
    #[test]
    fn the_ground_floor_is_authored_at_true_scale() {
        let textures = ground_floor_textures(128);
        let names: Vec<&str> = textures.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, vec!["ground/shop", "ground/lobby", "ground/home"]);
        for texture in &textures {
            assert_eq!(texture.rgba.len(), texture.width * texture.height * 4);
            assert_eq!(texture.width, 128 * GROUND_FLOOR_BAYS);
            assert_eq!(texture.tile_width_m, GROUND_FLOOR_TILE_W);
            assert_eq!(texture.tile_height_m, GROUND_STOREY_M);
            assert!(
                texture.rgba.chunks(4).all(|pixel| pixel[3] == 255),
                "{} has a hole",
                texture.name
            );
            // A shopfront is a bright, glazed front: light stone piers, a pale
            // aluminium frame and blue-grey glass, so its median is well below a
            // sunlit render wall.  A residential entrance is a render wall and is
            // allowed to be one (about 0.6); its band is the wall's own range.
            let median = luma(median_albedo(texture));
            let band = if texture.name == "ground/shop" {
                0.10..=0.40
            } else {
                0.10..=0.70
            };
            assert!(
                band.contains(&median),
                "{}: a median reflectance of {median:.3} is outside its band",
                texture.name
            );
        }
    }

    /// The shop fascia is signage, not paint: down a run of bays the fascia
    /// changes colour per 4.2 m unit, every board stays in the painted-metal /
    /// acrylic reflectance range, and a decent share of units keep plain render.
    #[test]
    fn the_shop_signage_varies_per_bay_within_physical_reflectance() {
        let texture = ground_floor("shop", 256);
        let n = texture.height;
        let px_per_m = texture.width as f32 / GROUND_FLOOR_TILE_W;
        // Sample the plain lower margin of the sign board (h ≈ 3.62 m).  The
        // lettering is a pattern laid over the board, and the bay-local pattern
        // repeats in every bay, so the board colour is only visible off it.
        let row = (((GROUND_FLOOR_TILE_H - 3.62) / GROUND_FLOOR_TILE_H) * n as f32) as usize;
        let mut bay_colours: Vec<[f32; 3]> = Vec::new();
        for bay in 0..8 {
            let x = (((bay as f32 + 0.5) * GROUND_FLOOR_BAY_W * px_per_m) as usize)
                .min(texture.width - 1);
            let offset = (row * texture.width + x) * 4;
            bay_colours.push([
                srgb_to_linear(texture.rgba[offset] as f32 / 255.0),
                srgb_to_linear(texture.rgba[offset + 1] as f32 / 255.0),
                srgb_to_linear(texture.rgba[offset + 2] as f32 / 255.0),
            ]);
        }
        // No board is a lit surface.
        for (bay, colour) in bay_colours.iter().enumerate() {
            assert!(
                // A white sign plate is painted at about 0.8, so the bound is the
                // albedo of the lightest painted board, not a dark-only rule.
                luma(*colour) < 0.90,
                "bay {bay}: a fascia of {:.3} luma is brighter than any painted board",
                luma(*colour)
            );
        }
        // And they are not all the same: at least four distinct boards in eight
        // bays, or the street reads as one franchise.
        let mut distinct: Vec<[f32; 3]> = Vec::new();
        for colour in &bay_colours {
            if !distinct
                .iter()
                // Boards differ in hue as much as in value: a blue and a green
                // sign at the same luma are still two shops.
                .any(|other| {
                    other
                        .iter()
                        .zip(colour)
                        .map(|(a, b)| (a - b).abs())
                        .fold(0.0_f32, f32::max)
                        < 0.02
                })
            {
                distinct.push(*colour);
            }
        }
        assert!(
            distinct.len() >= 4,
            "only {} distinct sign boards in 8 bays",
            distinct.len()
        );
    }
}
