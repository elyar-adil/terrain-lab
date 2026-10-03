//! The ground-floor bakes: `ground/shop`, `ground/lobby`, `ground/home`.
//!
//! Each is one shop bay by one ground storey at true scale, so a door is 2.1 m
//! tall on the wall rather than a stretched repeat.  The shopfront is the
//! darkest band on a street and the busiest: a stall riser, piers, glazing, a
//! roller shutter somewhere in every four bays, a fascia — and the fascia is
//! **per-bay coloured signage**, because a continuous run of blue, green, maroon
//! and white sign boards is the single most recognisable feature of a Chinese
//! retail street.

use crate::facades::{
    GROUND_FLOOR_BAY_W, GROUND_FLOOR_BAYS, GROUND_FLOOR_TILE_H, GROUND_FLOOR_TILE_W, GROUND_STOREY_M,
};
use crate::facades::tile::{shade, srgb8, value_noise};
use crate::textures::{BakedTexture, hash};

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
    [0.062, 0.094, 0.160], // deep blue, the default sign colour of the nation
    [0.048, 0.105, 0.070], // pharmacy green
    [0.135, 0.048, 0.042], // restaurant maroon
    [0.320, 0.312, 0.286], // ivory plate
    [0.078, 0.082, 0.088], // charcoal, the upscale one
    [0.185, 0.132, 0.048], // bakery gold on brown
];

/// A per-bay decision in `0..1`.  The shared `hash` is a half-finalised mix whose
/// outputs are correlated for consecutive small integers, which is exactly what a
/// bay index is, so bays get the full 32-bit avalanche.
fn bay_hash(seed: u32, bay: i32) -> f32 {
    let mut v = seed ^ (bay as u32).wrapping_mul(0x9e37_79b9);
    v ^= v >> 16;
    v = v.wrapping_mul(0x85eb_ca6b);
    v ^= v >> 13;
    v = v.wrapping_mul(0xc2b2_ae35);
    v ^= v >> 16;
    v as f32 / u32::MAX as f32
}

/// The sign board of one bay.  Neighbouring shops never share a board: each bay
/// steps a random 1..N-1 places on from the one before it.
fn sign_board(key: u32, bay: i32) -> usize {
    let n = SIGN_BOARDS.len();
    let mut index = (bay_hash(key.wrapping_add(701), 0) * n as f32) as usize % n;
    for b in 1..=bay.max(0) {
        let step = 1 + (bay_hash(key.wrapping_add(701), b) * (n - 1) as f32) as usize % (n - 1);
        index = (index + step) % n;
    }
    index
}

/// Chinese ground floors are a *continuous* band, not a row of doors: a stall
/// riser, a run of piers, glazed shopfronts, a roller shutter somewhere in
/// every four bays, a fascia, and the projecting sign boxes the geometry adds in
/// front of it.  That rhythm — not the door — is what makes a street read as
/// retail.
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
    const RISER: [f32; 3] = [0.098, 0.100, 0.104];
    const PIER: [f32; 3] = [0.330, 0.318, 0.296];
    const PIER_LIGHT: [f32; 3] = [0.405, 0.398, 0.380];
    const FRAME: [f32; 3] = [0.062, 0.065, 0.068];
    const GLASS: [f32; 3] = [0.052, 0.058, 0.066];
    const FASCIA: [f32; 3] = [0.265, 0.258, 0.244];
    const FASCIA_TOP: [f32; 3] = [0.360, 0.353, 0.336];
    const RENDER: [f32; 3] = [0.470, 0.442, 0.390];
    const DOOR: [f32; 3] = [0.115, 0.090, 0.070];

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
            let plain_pick = bay_hash(key.wrapping_add(733), bay);
            let fascia = if plain_pick < 0.30 {
                // Some units never re-clad: bare render fascia.
                shade(FASCIA, 0.85 + 0.3 * bay_hash(key.wrapping_add(751), bay))
            } else {
                shade(
                    SIGN_BOARDS[board],
                    0.92 + 0.16 * bay_hash(key.wrapping_add(769), bay),
                )
            };

            let mut colour = match kind {
                "shop" => {
                    if h < 0.42 {
                        // Stall riser: a dark stone kick plate, grooved, scuffed
                        // by a thousand trolleys.
                        let groove = (u / 0.42) - (u / 0.42).floor() < 0.06;
                        shade(RISER, if groove { 0.72 } else { 1.0 })
                    } else if h > 3.55 {
                        // Fascia: the sign band, with a soffit shadow under the
                        // canopy lip and a pale top rail.  The board itself is
                        // one coloured sign per bay — see `fascia` above — and
                        // the geometry adds the projecting sign boxes in front.
                        if h > 4.32 {
                            FASCIA_TOP
                        } else if h > 4.20 {
                            [0.055, 0.058, 0.060]
                        } else if h < 3.68 {
                            shade(fascia, 0.60)
                        } else {
                            fascia
                        }
                    } else if u < pier_a || u >= flank {
                        // The structural pier, and the flank wall beyond it.
                        // Fair-faced stone, and the brightest thing at street
                        // level apart from the signs.
                        let flute = ((u / 0.31) - (u / 0.31).floor() - 0.5).abs();
                        shade(
                            if block.rem_euclid(2) == 0 { PIER_LIGHT } else { PIER },
                            0.84 + flute * 0.26,
                        )
                    } else if (mullion - 0.30..mullion).contains(&u) {
                        FRAME
                    } else if (mullion..door).contains(&u) {
                        // The entrance: a pair of glass doors with a dark frame
                        // and a bronze pull rail, and a mat behind the glass.
                        if h > 3.02 {
                            FRAME
                        } else if (u - mullion - 0.22).abs() < 0.030
                            && h > 1.00
                            && h < 1.95
                        {
                            [0.320, 0.272, 0.170]
                        } else if u < mullion + 0.06 || u > door - 0.06 {
                            FRAME
                        } else {
                            let depth = 0.42 + 0.70 * (1.0 - (h / 3.0).clamp(0.0, 1.0));
                            shade(GLASS, depth * 1.5)
                        }
                    } else if (glazed..glazed + 0.10).contains(&u) {
                        FRAME
                    } else if (2.95..3.07).contains(&h) {
                        // The head rail over the whole shopfront.
                        shade(FRAME, 1.3)
                    } else if h < 2.30 && decide(29) < 0.55 {
                        // A roller shutter, down over the lower two thirds of the
                        // shopfront.  Its ribs are the only repeating
                        // high-frequency line on a Chinese shopfront and they
                        // catch raking light; a 60 mm slat at 30 px/m is two
                        // pixels, so the rib is resolution-aware.
                        if ((h / 0.09) - (h / 0.09).floor()) < 0.45 {
                            shade([0.250, 0.254, 0.250], 0.80)
                        } else {
                            shade([0.250, 0.254, 0.250], 1.10)
                        }
                    } else if h < 2.30 {
                        // The serving hatch (取货口), lit from inside.  The one warm
                        // thing at street level, and the reason a Chinese
                        // shopfront reads as a shop rather than as a mirror.  It
                        // has a counter, a roller box and a stack of goods behind
                        // it, because a bare bright rectangle is just a light.
                        if h < 0.95 {
                            // The counter, and its front panel.
                            if h < 0.86 {
                                shade([0.180, 0.172, 0.158], 0.90)
                            } else {
                                [0.045, 0.046, 0.048]
                            }
                        } else if h < 1.10 {
                            // The roller box the hatch shutter rolls into.
                            shade([0.230, 0.226, 0.216], 0.86)
                        } else {
                            let lit = 0.45 + 0.75 * decide(31);
                            // Warm, and falling off towards the back of the shop.
                            let fall = 1.0 - 0.35 * ((h - 1.10) / 1.20).clamp(0.0, 1.0);
                            let goods = if ((u * 3.4) - (u * 3.4).floor()) < 0.5
                                && ((h * 5.0) - (h * 5.0).floor()) < 0.55
                            {
                                0.55
                            } else {
                                1.0
                            };
                            [0.235 * lit * fall * goods, 0.205 * lit * fall * goods, 0.160 * lit * fall * goods]
                        }
                    } else {
                        // Shopfront glass above the hatch: dark, with the
                        // interior's own reflections as broad soft bands.
                        let band = 0.82
                            + 0.30 * value_noise(key.wrapping_add(601), u * 1.6, h * 0.9);
                        shade(GLASS, band)
                    }
                }
                "lobby" => {
                    if h < 0.50 {
                        // Dark granite base, and every lobby in a Chinese city
                        // has one because the first forty centimetres get hit.
                        let groove = (u / 0.90) - (u / 0.90).floor() < 0.05;
                        shade([0.082, 0.086, 0.092], if groove { 0.74 } else { 1.0 })
                    } else if h > 3.55 {
                        if h > 4.24 {
                            [0.440, 0.432, 0.412]
                        } else if h > 4.14 {
                            [0.050, 0.053, 0.056]
                        } else if h < 3.68 {
                            shade([0.320, 0.312, 0.296], 0.58)
                        } else {
                            [0.320, 0.312, 0.296]
                        }
                    } else if u < 0.70 || u > 3.50 {
                        // A stone portal: two piers and a deep head, which is
                        // what throws the shadow across the doors.
                        let flute = ((u / 0.35) - (u / 0.35).floor() - 0.5).abs();
                        shade(PIER_LIGHT, 0.78 + flute * 0.30)
                    } else if h > 3.02 {
                        shade(PIER_LIGHT, 0.92)
                    } else {
                        // Full-height entrance glazing, with a pair of doors in
                        // the middle and a bronze pull rail on each.
                        let door = (1.42..2.78).contains(&u);
                        let stile = (u / 0.735) - (u / 0.735).floor() < 0.028;
                        let rail = door
                            && ((u - 1.55).abs() < 0.032 || (u - 2.65).abs() < 0.032)
                            && h > 1.05
                            && h < 1.95;
                        if rail {
                            [0.330, 0.282, 0.176]
                        } else if stile || h > 2.94 {
                            FRAME
                        } else {
                            let depth = 0.45 + 0.80 * (1.0 - (h / 3.0).clamp(0.0, 1.0));
                            if door {
                                shade(GLASS, depth * 1.35)
                            } else {
                                // A lit lobby behind the sidelights, which is
                                // what a tower entrance looks like at dusk.
                                let lit = 0.35 + 0.55 * decide(31);
                                [0.085 * lit, 0.092 * lit, 0.100 * lit]
                            }
                        }
                    }
                }
                _ => {
                    // A residential entrance.  Render, a panelled door, and a
                    // barred window either side — the 防盗窗 is not optional on
                    // the ground floor of a Chinese apartment block.
                    if h < 0.45 {
                        let groove = (u / 0.50) - (u / 0.50).floor() < 0.06;
                        shade([0.120, 0.116, 0.110], if groove { 0.72 } else { 1.0 })
                    } else if h > 3.50 {
                        if h > 4.22 {
                            [0.500, 0.486, 0.452]
                        } else if h > 4.12 {
                            [0.058, 0.060, 0.062]
                        } else if h < 3.64 {
                            shade(RENDER, 0.58)
                        } else {
                            shade(RENDER, 1.10)
                        }
                    } else if (1.78..2.62).contains(&u) {
                        // The door: an 0.84 m leaf, panelled, with a frame and a
                        // step.  Dark, because a steel security door is.
                        if h > 3.05 {
                            shade(RENDER, 1.08)
                        } else if u < 1.86 || u > 2.54 || h < 0.62 {
                            [0.068, 0.070, 0.072]
                        } else if (u - 2.40).abs() < 0.030 && h > 1.05 && h < 1.95 {
                            [0.320, 0.280, 0.180]
                        } else {
                            let panel = (h - 0.72) / 0.78;
                            let recess = if (panel - 0.5).abs() < 0.34 { 0.72 } else { 1.0 };
                            shade(DOOR, recess)
                        }
                    } else if (0.42..1.46).contains(&u) || (2.94..3.98).contains(&u) {
                        // The barred window, with a projecting cill.  The bars
                        // are a 115 mm grid, which is what a welded security
                        // grille actually is, and they are the brightest thing on
                        // the wall because they are galvanised steel in sun.
                        if h < 0.92 {
                            [0.330, 0.320, 0.302]
                        } else if h < 1.02 {
                            shade(RENDER, 0.32)
                        } else {
                            let vertical = (u / 0.115) - (u / 0.115).floor() < 0.26;
                            let horizontal = (h / 0.26) - (h / 0.26).floor() < 0.20;
                            if vertical || horizontal {
                                [0.520, 0.512, 0.494]
                            } else {
                                let depth =
                                    0.40 + 0.75 * (1.0 - ((h - 1.02) / 2.1).clamp(0.0, 1.0));
                                shade(GLASS, depth * 1.4)
                            }
                        }
                    } else {
                        // Render, with a downpipe in the corner and the stain
                        // that runs from it.
                        if (u < 0.30 || u > 3.90) && h < 3.30 {
                            [0.205, 0.200, 0.192]
                        } else if u > 3.90 {
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
            // A shopfront is the darkest thing on a street, so its median has to
            // be dark.  The old bake put a 176/255 stone pier over half of it.
            let median = luma(median_albedo(texture));
            assert!(
                (0.04..=0.26).contains(&median),
                "{}: a median reflectance of {median:.3} is not a shopfront",
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
        // Sample the middle of the fascia band (h ≈ 3.9 m) per bay.
        let row = (((GROUND_FLOOR_TILE_H - 3.9) / GROUND_FLOOR_TILE_H) * n as f32) as usize;
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
                luma(*colour) < 0.40,
                "bay {bay}: a fascia of {:.3} luma is a light box pretending to be paint",
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
