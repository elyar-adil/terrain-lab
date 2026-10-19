//! The Chinese street shop storey (底商): a run of 门面, each a glazed
//! storefront under a signboard (门头) that spans the unit.
//!
//! What makes a Chinese shopping street recognisable in a photograph is the
//! signboards: a continuous band, one colour per shop, each carrying two to
//! five large characters, over storefronts that are almost all glass with a
//! lit shop behind.  The previous ground-floor bake painted the whole shopfront
//! in the sign colour, which reads as a row of shipping containers.
//!
//! The tile's alpha channel is a surface class (fixed colour, or glass with a
//! shop id), so the renderer draws the storefront glass with a reflection and
//! a lit interior.  It is 8 bays of 4.2 m and one 4.5 m
//! storey, and a shop is one or two bays.  The geometry layer builds each
//! signboard as a box standing 250 mm proud of the wall whose face samples
//! [`SIGN_Y0`]..[`SIGN_Y1`] of this tile, so the lettering is on a real board.

use crate::textures::BakedTexture;
use worldgen_core::hash::avalanche01;

use super::tile::{srgb8, value_noise};
use super::{GROUND_FLOOR_BAY_W, GROUND_FLOOR_BAYS, GROUND_STOREY_M};

/// Alpha of baked colour drawn as is.
const ALPHA_FIXED: u8 = 192;
/// Glazing carries a shop id in `0 ..= ALPHA_GLASS_MAX`.
const ALPHA_GLASS_MAX: u8 = 120;

/// The material key.
pub const SHOPFRONT: &str = "cnshop/street";
/// The signboard band, metres above the shop floor.
pub const SIGN_Y0: f32 = 3.38;
pub const SIGN_Y1: f32 = 4.42;
/// The storefront head.
const HEAD_Y: f32 = 3.28;
const RISER_Y: f32 = 0.34;
const PIER_W: f32 = 0.20;

const PIER: [f32; 3] = [0.52, 0.50, 0.47];
const RISER: [f32; 3] = [0.10, 0.10, 0.10];
const FRAME_DARK: [f32; 3] = [0.07, 0.075, 0.08];
const FRAME_SILVER: [f32; 3] = [0.50, 0.51, 0.52];
const SHUTTER: [f32; 3] = [0.46, 0.47, 0.47];
const STEEL: [f32; 3] = [0.58, 0.58, 0.57];

/// Signboard grounds and their lettering: red with white or gold, blue with
/// white, green with white, white with red, yellow with red, black with gold,
/// orange with white.  Reflectances, linear; signs are saturated.
const BOARDS: [([f32; 3], [f32; 3]); 8] = [
    ([0.42, 0.035, 0.03], [0.80, 0.79, 0.74]),
    ([0.42, 0.035, 0.03], [0.74, 0.55, 0.10]),
    ([0.03, 0.12, 0.40], [0.80, 0.80, 0.78]),
    ([0.03, 0.26, 0.11], [0.80, 0.80, 0.78]),
    ([0.72, 0.71, 0.68], [0.40, 0.03, 0.03]),
    ([0.70, 0.50, 0.04], [0.35, 0.03, 0.02]),
    ([0.03, 0.03, 0.035], [0.68, 0.52, 0.16]),
    ([0.68, 0.24, 0.03], [0.80, 0.79, 0.74]),
];

/// What the signboards say: shop names of two to four characters.  Every
/// character here must be in the atlas, which `scripts/sign-glyphs.py` builds
/// from this list.
const SIGN_PHRASES: &[&str] = &[
    "超市",
    "便利店",
    "大药房",
    "药房",
    "面馆",
    "饺子馆",
    "包子铺",
    "小吃",
    "烧烤",
    "火锅",
    "奶茶",
    "咖啡",
    "茶饮",
    "理发",
    "美发店",
    "美容院",
    "洗浴",
    "足疗",
    "烟酒",
    "水果",
    "生鲜",
    "五金",
    "建材",
    "修车",
    "汽修",
    "快递",
    "电信",
    "银行",
    "诊所",
    "口腔",
    "眼镜",
    "服装",
    "鞋业",
    "手机",
    "维修",
    "文具",
    "书店",
    "打印",
    "花店",
    "宠物",
    "早餐",
    "米线",
    "家电",
    "饰品",
    "金店",
    "棋牌",
    "网吧",
    "台球",
    "健身",
    "教育",
    "培训",
    "幼儿园",
    "副食",
    "百货",
    "菜馆",
    "粮油",
    "干洗",
    "照相",
    "彩票",
    "旧货",
    "装饰",
    "家具",
    "童装",
    "寿司",
    "面包",
    "蛋糕",
];

/// The signboard atlas: 24 by 24 cells per character, four bits a cell.
const GLYPH_CELLS: usize = 24;
const GLYPH_BYTES: usize = GLYPH_CELLS * GLYPH_CELLS / 2;
const ATLAS: &[u8] = include_bytes!("sign_glyphs.bin");

/// The packed cells of a character, if the atlas has it.
fn glyph_of(c: char) -> Option<&'static [u8]> {
    ATLAS
        .chunks_exact(4 + GLYPH_BYTES)
        .find(|entry| u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]) == c as u32)
        .map(|entry| &entry[4..])
}

fn cell(glyph: &[u8], x: usize, y: usize) -> f32 {
    let i = y * GLYPH_CELLS + x;
    let byte = glyph[i / 2];
    let level = if i.is_multiple_of(2) {
        byte >> 4
    } else {
        byte & 15
    };
    f32::from(level) / 15.0
}

/// Ink at `(gx, gy)` in 0..1 across a glyph, row 0 at the top.  Bilinear, so an
/// edge is a ramp a texel wide and the strokes do not crawl.
fn ink(glyph: &[u8], gx: f32, gy: f32) -> f32 {
    let last = (GLYPH_CELLS - 1) as f32;
    let x = (gx * GLYPH_CELLS as f32 - 0.5).clamp(0.0, last);
    let y = (gy * GLYPH_CELLS as f32 - 0.5).clamp(0.0, last);
    let (x0, y0) = (x as usize, y as usize);
    let (x1, y1) = ((x0 + 1).min(GLYPH_CELLS - 1), (y0 + 1).min(GLYPH_CELLS - 1));
    let (fx, fy) = (x - x0 as f32, y - y0 as f32);
    let top = cell(glyph, x0, y0) * (1.0 - fx) + cell(glyph, x1, y0) * fx;
    let bottom = cell(glyph, x0, y1) * (1.0 - fx) + cell(glyph, x1, y1) * fx;
    top * (1.0 - fy) + bottom * fy
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// The tile, `size` texels per bay horizontally.
pub fn cn_shopfront_texture(size: usize) -> BakedTexture {
    let per_bay = size.max(32);
    let w = per_bay * GROUND_FLOOR_BAYS;
    let h = (per_bay as f32 * GROUND_STOREY_M / GROUND_FLOOR_BAY_W).round() as usize;
    let tile_w = GROUND_FLOOR_BAY_W * GROUND_FLOOR_BAYS as f32;
    let mut rgba = vec![0_u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let um = (x as f32 + 0.5) / w as f32 * tile_w;
            let m = (1.0 - (y as f32 + 0.5) / h as f32) * GROUND_STOREY_M;
            let (rgb, alpha) = sample(um, m);
            let offset = (y * w + x) * 4;
            rgba[offset] = srgb8(rgb[0]);
            rgba[offset + 1] = srgb8(rgb[1]);
            rgba[offset + 2] = srgb8(rgb[2]);
            rgba[offset + 3] = alpha;
        }
    }
    BakedTexture {
        name: SHOPFRONT.to_owned(),
        width: w,
        height: h,
        tile_width_m: tile_w,
        tile_height_m: GROUND_STOREY_M,
        has_normal_source: false,
        rgba,
    }
}

const KEY: u32 = 0x2c1b_3c6d;

/// A shop: which bays it spans.  A shop is one bay, or two with the next one.
fn shop_of(bay: i32) -> (i32, i32) {
    // Walk from bay 0 so a pair never overlaps another pair.
    let mut start = 0;
    while start < GROUND_FLOOR_BAYS as i32 {
        let wide = avalanche01(KEY, start, 0, 11) < 0.35 && start + 1 < GROUND_FLOOR_BAYS as i32;
        let end = if wide { start + 2 } else { start + 1 };
        if bay < end {
            return (start, end);
        }
        start = end;
    }
    (bay, bay + 1)
}

/// Every shop of the tile as `(x0, x1)` metres from the tile's left edge.
pub fn shop_spans() -> Vec<(f32, f32)> {
    let mut spans = Vec::new();
    let mut bay = 0;
    while bay < GROUND_FLOOR_BAYS as i32 {
        let (s0, s1) = shop_of(bay);
        spans.push((
            s0 as f32 * GROUND_FLOOR_BAY_W,
            s1 as f32 * GROUND_FLOOR_BAY_W,
        ));
        bay = s1;
    }
    spans
}

fn sample(um: f32, m: f32) -> ([f32; 3], u8) {
    let bay = (um / GROUND_FLOOR_BAY_W).floor() as i32;
    let (s0, s1) = shop_of(bay);
    let x0 = s0 as f32 * GROUND_FLOOR_BAY_W;
    let x1 = s1 as f32 * GROUND_FLOOR_BAY_W;
    let local = um - x0;
    let width = x1 - x0;
    let shop_id = (avalanche01(KEY, s0, 0, 31) * ALPHA_GLASS_MAX as f32) as u8;
    let grime = 1.0 - 0.05 * value_noise(KEY, um / 0.6, m / 0.6);

    // Piers between shops, and a slimmer mullion pier inside a two-bay shop.
    let at_shop_edge = local < PIER_W * 0.5 || local > width - PIER_W * 0.5;
    let at_mid_pier = s1 - s0 == 2 && (local - GROUND_FLOOR_BAY_W).abs() < 0.08;
    if m < SIGN_Y0 && (at_shop_edge || at_mid_pier) {
        let joint = (m / 0.6).fract() < 0.03;
        let tone = if joint { 0.80 } else { grime };
        return (shade(PIER, tone), ALPHA_FIXED);
    }

    // The signboard.
    if m >= SIGN_Y0 {
        if m > SIGN_Y1 || local < 0.04 || local > width - 0.04 {
            return (shade(PIER, grime), ALPHA_FIXED);
        }
        let board =
            BOARDS[(avalanche01(KEY, s0, 0, 41) * BOARDS.len() as f32) as usize % BOARDS.len()];
        // A stainless edge.
        if !(SIGN_Y0 + 0.04..=SIGN_Y1 - 0.04).contains(&m) || local < 0.08 || local > width - 0.08 {
            return (STEEL, ALPHA_FIXED);
        }
        // The shop's name in one line, centred, 0.72 m high.
        let glyph_h = 0.72;
        let pitch = 0.86;
        let name = SIGN_PHRASES[(avalanche01(KEY, s0, 0, 43) * SIGN_PHRASES.len() as f32) as usize
            % SIGN_PHRASES.len()];
        let count = name.chars().count() as f32;
        let start = (width - count * pitch) * 0.5 + (pitch - glyph_h) * 0.5;
        let gy = (m - (SIGN_Y0 + SIGN_Y1 - glyph_h) * 0.5) / glyph_h;
        let slot = ((local - start) / pitch).floor();
        let ground = shade(board.0, 0.96 + 0.04 * grime);
        if slot >= 0.0 && slot < count {
            let gx = (local - start - slot * pitch) / glyph_h;
            let gy = 1.0 - gy;
            let inside = (0.0..1.0).contains(&gx) && (0.0..1.0).contains(&gy);
            if let Some(glyph) = name
                .chars()
                .nth(slot as usize)
                .and_then(glyph_of)
                .filter(|_| inside)
            {
                return (mix(ground, board.1, ink(glyph, gx, gy)), ALPHA_FIXED);
            }
        }
        return (ground, ALPHA_FIXED);
    }

    // The storefront.
    if m < RISER_Y {
        return (shade(RISER, grime), ALPHA_FIXED);
    }
    let silver = avalanche01(KEY, s0, 0, 51) < 0.5;
    let frame = if silver { FRAME_SILVER } else { FRAME_DARK };
    if m > HEAD_Y {
        return (frame, ALPHA_FIXED);
    }
    // A shut shop: the roller shutter down.
    if avalanche01(KEY, s0, 0, 53) < 0.16 {
        let slat = (m / 0.075).fract();
        let tone = if slat < 0.18 { 0.70 } else { 1.0 };
        return (shade(SHUTTER, tone * grime), ALPHA_FIXED);
    }
    let gx = local - PIER_W * 0.5;
    let gw = width - PIER_W;
    let f = 0.05;
    if gx < f || gx > gw - f || m < RISER_Y + f || m > HEAD_Y - f {
        return (frame, ALPHA_FIXED);
    }
    // Mullions: a door in the middle of the shop, fixed panes either side.
    let door_w = 1.8_f32.min(gw * 0.5);
    let d0 = (gw - door_w) * 0.5;
    let d1 = d0 + door_w;
    let is_door = gx > d0 && gx < d1;
    let mullion = (gx - d0).abs() < 0.04
        || (gx - d1).abs() < 0.04
        || (is_door && (gx - (d0 + d1) * 0.5).abs() < 0.03)
        || (!is_door && {
            let side_w = if gx < d0 { d0 } else { gw - d1 };
            let panes = (side_w / 1.3).round().max(1.0);
            let pane_w = side_w / panes;
            let p = if gx < d0 { gx } else { gx - d1 };
            let fp = (p / pane_w).fract() * pane_w;
            p > 0.05 && fp < 0.035
        });
    // The door's transom.
    if mullion || (is_door && (m - 2.45).abs() < 0.04) {
        return (frame, ALPHA_FIXED);
    }
    // Door handles: two steel bars either side of the meeting stile.
    if is_door && (m - 1.1).abs() < 0.30 && ((gx - (d0 + d1) * 0.5).abs() - 0.10).abs() < 0.018 {
        return (STEEL, ALPHA_FIXED);
    }
    // Glass: u across the whole shop, v up the storefront, width over 5 m.
    let u = (gx / gw).clamp(0.0, 1.0);
    let v = ((m - RISER_Y) / (HEAD_Y - RISER_Y)).clamp(0.0, 1.0);
    ([u, v, (gw / 5.0).clamp(0.0, 0.96)], shop_id)
}

fn shade(colour: [f32; 3], factor: f32) -> [f32; 3] {
    [colour[0] * factor, colour[1] * factor, colour[2] * factor]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_storefront_is_mostly_glass_and_the_signs_carry_lettering() {
        let t = cn_shopfront_texture(64);
        let (w, h) = (t.width, t.height);
        let mut glass = 0;
        let mut storefront = 0;
        for y in 0..h {
            let m = (1.0 - (y as f32 + 0.5) / h as f32) * GROUND_STOREY_M;
            if !(RISER_Y..HEAD_Y).contains(&m) {
                continue;
            }
            for x in 0..w {
                storefront += 1;
                if t.rgba[(y * w + x) * 4 + 3] <= ALPHA_GLASS_MAX {
                    glass += 1;
                }
            }
        }
        let share = glass as f32 / storefront as f32;
        assert!(
            (0.55..0.92).contains(&share),
            "storefront glass share {share}"
        );
        // Every sign has ink on it: some texels in the sign band differ from
        // the board colour at the board's own centre line.
        let row = ((1.0 - (SIGN_Y0 + SIGN_Y1) * 0.5 / GROUND_STOREY_M) * h as f32) as usize;
        let mut changes = 0;
        for x in 1..w {
            let a = &t.rgba[(row * w + x - 1) * 4..(row * w + x - 1) * 4 + 3];
            let b = &t.rgba[(row * w + x) * 4..(row * w + x) * 4 + 3];
            if a != b {
                changes += 1;
            }
        }
        assert!(
            changes > 40,
            "the signboards carry no lettering ({changes} edges)"
        );
    }

    #[test]
    fn every_sign_name_is_drawn_from_the_atlas() {
        for name in SIGN_PHRASES {
            assert!(name.chars().count() <= 4, "{name} does not fit a bay");
            for c in name.chars() {
                assert!(glyph_of(c).is_some(), "{c} is missing from the sign atlas");
            }
        }
    }

    #[test]
    fn shops_tile_the_bays_without_overlap() {
        let mut bay = 0;
        while bay < GROUND_FLOOR_BAYS as i32 {
            let (s0, s1) = shop_of(bay);
            assert_eq!(s0, bay);
            assert!(s1 > s0 && s1 <= GROUND_FLOOR_BAYS as i32);
            for inner in s0..s1 {
                assert_eq!(shop_of(inner), (s0, s1));
            }
            bay = s1;
        }
    }
}
