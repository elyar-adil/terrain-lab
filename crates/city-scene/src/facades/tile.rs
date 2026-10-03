//! The facade tile bake: one texture per design, sampled in construction
//! dimensions.
//!
//! Every sample here is taken in **metres of wall** — piers, bands, sills and
//! reveals are all placed from the [`FacadeDesign`] fields, so the baked tile
//! and the built geometry are two views of the same rule table.  The tile's
//! physical size *is* the design's bay: `tile_width_m` is 开间 and
//! `tile_height_m` is four storeys, so a renderer that repeats the texture by
//! its physical size reproduces the bay rhythm and the floor lines exactly.

use crate::facades::designs::{Cladding, DESIGNS, FacadeDesign, design, tile_height_m};
use crate::textures::{BakedTexture, hash};

/// The 24 facade tiles, one texture each so a renderer can bind one and never
/// branch.  The material key is `facade/NN` and it is the only contract between
/// this module and the renderer.
pub fn facade_textures(size: usize) -> Vec<BakedTexture> {
    (0..DESIGNS.len())
        .map(|index| facade_tile(index, size))
        .collect()
}

/// Encode a **linear** reflectance as an sRGB byte, which is what the renderer
/// decodes back through an sRGB texture.  Getting this wrong — writing a linear
/// value straight into an sRGB texture — darkens every wall by a factor of
/// about 2.2 at the top of the range and is the other half of "why is my city
/// black".
pub(super) fn srgb8(linear: f32) -> u8 {
    let c = linear.clamp(0.0, 1.0);
    let encoded = if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (encoded * 255.0 + 0.5) as u8
}

/// Smooth value noise on a 1 m lattice, so a wall has low-frequency blotching
/// as well as high-frequency grain.  Two surfaces that both ask for "some
/// weather" get the same character because they share the hash.
pub(super) fn value_noise(seed: u32, x: f32, y: f32) -> f32 {
    let (x0, y0) = (x.floor(), y.floor());
    let (fx, fy) = (x - x0, y - y0);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let (ix, iy) = (x0 as i32, y0 as i32);
    let n00 = hash(seed, ix, iy);
    let n10 = hash(seed, ix + 1, iy);
    let n01 = hash(seed, ix, iy + 1);
    let n11 = hash(seed, ix + 1, iy + 1);
    let a = n00 + (n10 - n00) * sx;
    let b = n01 + (n11 - n01) * sx;
    a + (b - a) * sy
}

/// Multiply a reflectance, for every element that is the same material seen
/// under less or more sky: a reveal, a soffit, a shadow line.
pub(super) fn shade(colour: [f32; 3], factor: f32) -> [f32; 3] {
    [colour[0] * factor, colour[1] * factor, colour[2] * factor]
}

/// Move a reflectance towards another, for the elements that are the same
/// material with a different mix: brick varies brick to brick, and a curtain
/// wall's glass varies from a mirror to a hole depending on what is behind it.
fn towards(colour: [f32; 3], target: [f32; 3], amount: f32) -> [f32; 3] {
    let t = amount.clamp(0.0, 1.0);
    [
        colour[0] + (target[0] - colour[0]) * t,
        colour[1] + (target[1] - colour[1]) * t,
        colour[2] + (target[2] - colour[2]) * t,
    ]
}

/// Everything about one window that varies independently of the others.  This
/// struct is the difference between a facade and a texture: real buildings have
/// a different blind, a different curtain and a different amount of rain
/// washing in every opening, and a facade where they all match reads as print.
struct Opening {
    /// Horizontal centre in bay-local metres.
    cx: f32,
    w: f32,
    y0: f32,
    y1: f32,
    blind: bool,
    /// Per-opening tone multiplier on the glass, 0.7 – 1.4.
    tone: f32,
    /// How far the glass has been replaced by a curtain, 0 – 1.
    curtain: f32,
    /// Which way the curtain's colour leans.
    curtain_hue: f32,
    grille: bool,
    /// Rain-washing streaked down the render below this opening, 0 – 1.
    wash: f32,
}

/// Half-width of the pier on bay boundary `k`, in metres.
///
/// Boundaries alternate: a **wide** pier on every other one and a narrow one
/// between.  That is the 2:1 rhythm of a real structural frame, and it is what
/// stops a bay-wide tile from reading as wallpaper.
pub(crate) fn pier_half_at(design: &FacadeDesign, k: i32, bay_w: f32) -> f32 {
    let wide = design.major_pier_w.max(design.pier_w);
    let narrow = design.pier_w.min(wide);
    let width = if k.rem_euclid(2) == 0 { wide } else { narrow };
    (width * 0.5).min(bay_w * 0.32)
}

/// The clear wall between the two piers that bound a bay, and the two pier
/// half-widths themselves.
///
/// A window has to fit in the clear wall.  This is not a detail: an opening that
/// runs under a pier is an opening that has been cut by a column, and it is the
/// single most obvious way a procedural facade gives itself away at fifty metres.
pub(crate) fn bay_clearance(
    design: &FacadeDesign,
    bays: i32,
    bay: i32,
    bay_w: f32,
) -> (f32, f32, f32) {
    let left = pier_half_at(design, bay, bay_w);
    // The boundary at the far side of the last bay is the *next tile's* first
    // boundary, so the pattern stays exactly periodic over the tile repeat.
    let right_k = if (bay + 1) % bays == 0 { 0 } else { bay + 1 };
    let right = pier_half_at(design, right_k, bay_w);
    let clear = (bay_w - left - right - 0.12).max(0.34);
    (left, right, clear)
}

fn opening_for(
    design: &FacadeDesign,
    key: u32,
    bay: i32,
    storey: i32,
    left: f32,
    right: f32,
    bay_w: f32,
    clear: f32,
) -> Opening {
    let a = hash(key.wrapping_add(11), bay, storey);
    let b = hash(key.wrapping_add(29), bay, storey);
    let c = hash(key.wrapping_add(47), bay, storey);
    let d = hash(key.wrapping_add(71), bay, storey);
    let e = hash(key.wrapping_add(97), bay, storey);
    let w = design.open_w.min(clear);
    // Exactly centred between the two piers that bound the bay: a window that sits
    // off-centre in its cell is the first thing the eye catches on a facade.
    let _ = (a, clear);
    let cx = left + (bay_w - left - right) * 0.5;
    // ±50 mm around the code sill: construction tolerance, not design intent.
    let y0 = design.sill_m + (b - 0.5) * 0.10;
    Opening {
        cx,
        w,
        y0,
        y1: y0 + design.open_h,
        blind: c < design.blind,
        tone: 0.70 + d * 0.70,
        curtain: if e < 0.34 {
            0.35 + hash(key, storey, bay) * 0.55
        } else {
            0.0
        },
        curtain_hue: hash(key.wrapping_add(131), bay, storey),
        grille: hash(key.wrapping_add(151), bay, storey) < design.grille,
        wash: if hash(key.wrapping_add(173), bay, storey) < 0.45 {
            0.35 + hash(key.wrapping_add(191), bay, storey) * 0.5
        } else {
            0.0
        },
    }
}

/// One facade tile.
fn facade_tile(index: usize, size: usize) -> BakedTexture {
    let design = design(index);
    let key = (index as u32).wrapping_mul(2_654_435_761);
    let n = size.max(16);
    let mut rgba = vec![0_u8; n * n * 4];
    let tile_h = tile_height_m(design);
    let bays = design.bays.max(1) as f32;
    let bay_w = design.bay_m / bays;
    // Pixels per metre, so a joint or a bar is a fixed physical width instead of
    // a fixed number of texels — a fixed pixel width swallows a 45 mm tile whole
    // at low bake sizes and vanishes at high ones.
    let px_per_m = n as f32 / design.bay_m;
    let hairline = 1.0 / px_per_m;

    for y in 0..n {
        for x in 0..n {
            // `um` runs along the wall in metres, `m` up it in metres.
            let um = (x as f32 + 0.5) / px_per_m;
            let m = (1.0 - (y as f32 + 0.5) / n as f32) * tile_h;
            let storey = ((m / design.storey_m).floor() as i32)
                .clamp(0, crate::facades::STOREYS_PER_TILE as i32 - 1);
            let ml = m - storey as f32 * design.storey_m;
            let bays_i = design.bays.max(1) as i32;
            let bay = ((um / bay_w).floor() as i32).clamp(0, bays_i - 1);
            let bx = um - bay as f32 * bay_w;
            let rgb = if design.cladding == Cladding::CurtainWall {
                curtain_sample(
                    design,
                    key,
                    um,
                    ml,
                    bay,
                    bx,
                    bay_w,
                    storey,
                    (x as i32, y as i32),
                )
            } else {
                let (left, right, clear) = bay_clearance(design, bays_i, bay, bay_w);
                masonry_sample(
                    design,
                    key,
                    um,
                    m,
                    ml,
                    bay,
                    bx,
                    bay_w,
                    left,
                    right,
                    clear,
                    storey,
                    (x as i32, y as i32),
                    px_per_m,
                    hairline,
                )
            };
            let offset = (y * n + x) * 4;
            rgba[offset] = srgb8(rgb[0]);
            rgba[offset + 1] = srgb8(rgb[1]);
            rgba[offset + 2] = srgb8(rgb[2]);
            // Opaque, always.  The alpha channel of a facade is not a height
            // field and must not become one: the renderer ignores it today, and
            // a wall with a hole in its alpha is a wall you can see through.
            rgba[offset + 3] = 255;
        }
    }
    BakedTexture {
        name: format!("facade/{index:02}"),
        width: n,
        height: n,
        // The tile *is* the bay: one 开间 wide, four 层高 tall.
        tile_width_m: design.bay_m,
        tile_height_m: tile_h,
        has_normal_source: false,
        rgba,
    }
}

/// A masonry wall, sampled at one point.
#[allow(clippy::too_many_arguments)]
fn masonry_sample(
    design: &FacadeDesign,
    key: u32,
    um: f32,
    m: f32,
    ml: f32,
    bay: i32,
    bx: f32,
    bay_w: f32,
    left: f32,
    right: f32,
    clear: f32,
    storey: i32,
    px: (i32, i32),
    px_per_m: f32,
    hairline: f32,
) -> [f32; 3] {
    let opening = opening_for(design, key, bay, storey, left, right, bay_w, clear);
    let frame_w = 0.055_f32.min((clear - opening.w) * 0.35).max(0.02);
    let x0 = opening.cx - opening.w * 0.5;
    let x1 = opening.cx + opening.w * 0.5;

    // --- 1. the opening, from the inside out -------------------------------
    let in_frame = bx > x0 - frame_w
        && bx < x1 + frame_w
        && ml > opening.y0 - frame_w
        && ml < opening.y1 + frame_w;
    if in_frame {
        let in_glass = bx > x0 && bx < x1 && ml > opening.y0 && ml < opening.y1;
        if in_glass {
            let u = (bx - x0) / opening.w.max(1.0e-3);
            let v = (ml - opening.y0) / (opening.y1 - opening.y0).max(1.0e-3);
            // The reveal.  A 60 mm plaster return is a real surface: its head is
            // in permanent shadow, one jamb is darker than the other, and its
            // inner sill catches the sky.  These are the wall's own reflectance
            // under a different amount of sky, not painted occlusion.
            let head = ml > opening.y1 - 0.055;
            let sill_return = ml < opening.y0 + 0.045;
            let jamb_left = bx < x0 + 0.045;
            let jamb_right = bx > x1 - 0.045;
            if head || jamb_left || jamb_right || sill_return {
                let factor = if head {
                    0.40
                } else if jamb_left {
                    0.60
                } else if jamb_right {
                    0.80
                } else {
                    1.12
                };
                return shade(design.wall, factor);
            }
            // The glass.  Darker than the wall by an order of magnitude, with
            // its own per-opening tone: a room behind a window is a different
            // brightness in every one of them.
            let mut glass = shade(design.glass, opening.tone);
            if opening.curtain > 0.0 {
                // A hung curtain is a diffuse, mid-value, slightly warm
                // surface — the one thing in a window that is genuinely light.
                let hue = opening.curtain_hue;
                let linen = if hue < 0.4 {
                    [0.30, 0.29, 0.27]
                } else if hue < 0.75 {
                    [0.26, 0.26, 0.25]
                } else {
                    [0.22, 0.24, 0.26]
                };
                glass = towards(glass, linen, opening.curtain);
            }
            if opening.blind {
                // A blind down: an off-white roller covering the head of the
                // opening, with a fold and a bottom rail.
                let blind_bottom = 0.34 + opening.tone * 0.16;
                if v < blind_bottom {
                    let fold = ((v * 11.0).fract() - 0.5).abs() * 0.10;
                    let rail = if v < 0.045 { 0.55 } else { 0.0 };
                    let linen = 0.52 + 0.16 * opening.curtain_hue - fold + rail;
                    return [linen * 1.03, linen, linen * 0.94];
                }
            }
            // A faint interior falloff, the horizontal banding a reflection off a
            // neighbouring building leaves in a pane, and a cross-pane gradient:
            // a sash window is two lights, and the light nearer the room's
            // ceiling shows more of the ceiling.
            let interior = 0.86 + 0.22 * (1.0 - v);
            let band = if ((v * 6.0).fract() - 0.5).abs() < 0.10 {
                1.10
            } else {
                1.0
            };
            let cross = 0.94 + 0.12 * (1.0 - u);
            if opening.grille {
                // 防盗窗: a pale steel grille over the opening.  Two louvre
                // families, the way they are actually welded.
                let vertical = (bx / 0.082).fract() < 0.34;
                let horizontal = (ml / 0.155).fract() < 0.28;
                if vertical || horizontal {
                    // Galvanised steel in sun is 0.35-0.50; painted steel about
                    // 0.55.  A grille much above that reads as a white panel
                    // rather than as a grid, which is the failure mode.
                    return [
                        design.frame[0] * 0.86,
                        design.frame[1] * 0.86,
                        design.frame[2] * 0.86,
                    ];
                }
            }
            let k = interior * band * cross;
            return [glass[0] * k, glass[1] * k, glass[2] * k];
        }
        // The frame itself: white-painted aluminium, weathered.
        let wear = 0.90 + 0.20 * value_noise(key.wrapping_add(211), um * 3.0, m * 3.0);
        return shade(design.frame, wear);
    }

    // --- 2. the sill apron, and the dark drip under it ----------------------
    if design.sill_m > 0.0
        && bx > x0 - 0.11
        && bx < x1 + 0.11
        && ml > opening.y0 - 0.15
        && ml < opening.y0
    {
        // The apron is a real projecting stone or rendered sill, so it is
        // *brighter* than the wall above it, and the 20 mm of shadow under its
        // drip is much darker.  That light-dark pair is the single detail that
        // makes a punched window read as an opening rather than a hole.
        if ml < opening.y0 - 0.055 {
            return shade(design.wall, 0.34);
        }
        return shade(design.pier, 0.98);
    }

    // --- 3. the painted surround above the opening -------------------------
    if design.lintel_h > 0.0
        && bx > x0 - 0.14
        && bx < x1 + 0.14
        && ml > opening.y1 + 0.10
        && ml < opening.y1 + 0.10 + design.lintel_h
    {
        return shade(design.lintel, 1.0);
    }

    // --- 4. the piers -------------------------------------------------------
    if bx < left || bx > bay_w - right {
        let half = if bx < left { left } else { right };
        let edge = (1.0 - (bx.min(bay_w - bx) / half)).clamp(0.0, 1.0);
        // A pier is a projecting strip, so its flanks shade: dark at the edge,
        // full value in the middle.  A wide pier is a structural bay wall and
        // is very slightly lighter than the narrow one, which is what makes the
        // 2:1 rhythm read.
        let wide = (left > design.pier_w * 0.5) || (right > design.pier_w * 0.5);
        return shade(
            design.pier,
            0.78 + 0.22 * edge + if wide { 0.04 } else { 0.0 },
        );
    }

    // --- 5. the inter-storey band -------------------------------------------
    // Three parts, in the order they stack on a real elevation: the shadow the
    // slab above throws on its own edge, the band face, and the pale drip lip
    // that throws the water clear.  Storey lines are the reason a facade is
    // legible from a kilometre, so this is the part that is never simplified.
    if ml < design.band_h + 0.12 {
        if ml < 0.055 {
            return shade(design.wall, 0.30);
        }
        if ml < 0.055 + design.band_h {
            let t = (ml - 0.055) / design.band_h.max(1.0e-3);
            return shade(design.band, 0.88 + 0.14 * t);
        }
        return shade(design.wall, 1.12);
    }
    if m < 0.14 {
        // The floor line itself, where the skirting runs.
        return shade(design.wall, 0.62);
    }

    // --- 6. rain washing down the render below an opening -------------------
    if opening.wash > 0.0
        && bx > opening.cx - opening.w * 0.55
        && bx < opening.cx + opening.w * 0.55
        && ml > opening.y0 - 0.15 - 0.85 * opening.wash
        && ml < opening.y0 - 0.15
    {
        let t =
            (ml - (opening.y0 - 0.15 - 0.85 * opening.wash)) / (0.85 * opening.wash).max(1.0e-3);
        let streak = 0.72 + 0.28 * value_noise(key.wrapping_add(233), um * 5.0, m * 1.2);
        return shade(design.wall, 1.0 - 0.20 * opening.wash * (1.0 - t) * streak);
    }

    // --- 7. the wall field --------------------------------------------------
    wall_field(design, key, um, m, ml, px, px_per_m, hairline)
}

/// The wall itself: cladding module, grain and patch repairs.  This is the
/// largest area of every masonry tile, so it is also the part that decides
/// whether the tile reads as a material or as a colour.
#[allow(clippy::too_many_arguments)]
fn wall_field(
    design: &FacadeDesign,
    key: u32,
    um: f32,
    m: f32,
    ml: f32,
    px: (i32, i32),
    px_per_m: f32,
    hairline: f32,
) -> [f32; 3] {
    // High-frequency grain, then a slow blotch, then patch repairs on a 1.2 m
    // grid — a rendered wall is never one colour over four square metres, and
    // the patches are what a repair leaves behind.
    let grain = (hash(key, px.0, px.1) - 0.5) * design.grain;
    let blotch = (value_noise(key.wrapping_add(7), um * 0.7, m * 0.55) - 0.5) * design.grain * 1.5;
    let patch_cell = (um / 1.2).floor() as i32;
    let patch_row = (m / 1.05).floor() as i32;
    let patch = (hash(key.wrapping_add(307), patch_cell, patch_row) - 0.5) * design.grain * 2.2;
    let mut colour = [
        design.wall[0] + grain + blotch + patch,
        design.wall[1] + grain + blotch + patch,
        design.wall[2] + grain + blotch + patch,
    ];

    match design.cladding {
        Cladding::Brick => {
            // 240 x 60 clay brick in running bond.  The mortar is the only light
            // thing on a brick wall, and it is what makes brick read as brick
            // from fifty metres.
            let course = (m / 0.072).floor() as i32;
            let stagger = if course.rem_euclid(2) == 0 {
                0.0
            } else {
                0.120
            };
            let bx = um + stagger;
            let along = bx / 0.240;
            let face = along - along.floor();
            let up = (m / 0.072) - (m / 0.072).floor();
            // 10 mm of joint on a 65 mm course is 15% of the face, not 25%:
            // a brick wall whose mortar is a quarter of its area reads as a
            // grid of pale lines, not as brick.
            let joint = face < 0.045 || face > 0.975 || up < 0.13 || up > 0.93;
            if joint {
                let mortar =
                    0.330 + 0.045 * hash(key.wrapping_add(401), along.floor() as i32, course);
                return [mortar, mortar * 0.985, mortar * 0.945];
            }
            // Every brick was fired separately, so the wall is a mosaic.
            let fire = (hash(key.wrapping_add(419), along.floor() as i32, course) - 0.5) * 0.055;
            let face_shadow = 0.92 + 0.16 * (1.0 - ((face - 0.055) / 0.91).clamp(0.0, 1.0));
            colour = [
                colour[0] + fire,
                colour[1] + fire * 0.8,
                colour[2] + fire * 0.6,
            ];
            [
                colour[0] * face_shadow,
                colour[1] * face_shadow,
                colour[2] * face_shadow,
            ]
        }
        Cladding::Mosaic => {
            // Small square tile with a grout joint.  Below about three pixels per
            // module the joint is dropped rather than aliased into a checkerboard.
            let module = design.module_m.max(0.02);
            if module * px_per_m >= 3.0 {
                let joint = (module * 0.10).max(hairline * 1.0);
                let tx = um / module;
                let ty = m / module;
                let fx = tx - tx.floor();
                let fy = ty - ty.floor();
                if fx < joint / module || fy < joint / module {
                    let grout = 0.40
                        + 0.06 * hash(key.wrapping_add(433), tx.floor() as i32, ty.floor() as i32);
                    return [grout, grout * 0.99, grout * 0.96];
                }
                // Fired tile, so each one is its own colour.
                let fire = (hash(key.wrapping_add(449), tx.floor() as i32, ty.floor() as i32)
                    - 0.5)
                    * 0.040;
                colour = [colour[0] + fire, colour[1] + fire, colour[2] + fire * 0.92];
            }
            if design.motif {
                // The accent tile: a 280 mm square of blue-green printed tile,
                // one per bay per storey, set in the pier beside the window.  It
                // is the detail that dates a Chinese apartment block to a decade.
                let module_centre = (um / 0.56).floor() as f32 * 0.56 + 0.28;
                let dx = (um - module_centre).abs();
                let dy = (ml - 1.55).abs();
                if dx < 0.14 && dy < 0.14 {
                    let checker =
                        ((um / 0.047).floor() as i32 + (m / 0.047).floor() as i32) % 2 == 0;
                    let motif = if checker {
                        [0.075, 0.105, 0.115]
                    } else {
                        [0.150, 0.190, 0.195]
                    };
                    return motif;
                }
            }
            colour
        }
        Cladding::Panel => {
            // Cast panel with a recessed joint, and the formwork marks a fair
            // faced panel always has.
            let joint_v = (um / 1.10) - (um / 1.10).floor();
            let joint_h = (m / 1.25) - (m / 1.25).floor();
            if joint_v < 0.02 || joint_h < 0.02 {
                return shade(design.wall, 0.62);
            }
            // Tie holes on a 0.55 m grid, one per panel.
            let tx = (um / 0.55) - (um / 0.55).floor();
            let ty = (m / 0.62) - (m / 0.62).floor();
            if (tx - 0.5).abs() < 0.045 && (ty - 0.5).abs() < 0.045 {
                return shade(design.wall, 0.78);
            }
            colour
        }
        _ => {
            // Smooth render: a faint trowel banding, and the drip edge of a
            // coat of paint every so often.
            let trowel = (value_noise(key.wrapping_add(463), um * 3.2, m * 0.6) - 0.5) * 0.016;
            [colour[0] + trowel, colour[1] + trowel, colour[2] + trowel]
        }
    }
}

/// A unitised curtain wall, sampled at one point.
#[allow(clippy::too_many_arguments)]
fn curtain_sample(
    design: &FacadeDesign,
    key: u32,
    um: f32,
    ml: f32,
    bay: i32,
    bx: f32,
    bay_w: f32,
    storey: i32,
    px: (i32, i32),
) -> [f32; 3] {
    // A unitised curtain wall is a grid, and the grid is what the eye reads.
    // Three widths matter: the fin over the main mullion, the main mullion
    // itself, and the sub-mullion that splits a wide bay into a real module.
    let within = um.rem_euclid(bay_w);
    let to_mullion = within.min(bay_w - within);
    if design.pier_w > 0.0 && to_mullion < design.pier_w * 0.5 {
        // The pale stone fin.  A projecting fin has a shaded flank and a lit
        // face, and its two edges are the strongest vertical lines a glass
        // tower has.
        let across = to_mullion / (design.pier_w * 0.5);
        let body = 0.86 + 0.20 * across;
        let flute = ((bx / 0.30) - (bx / 0.30).floor() - 0.5).abs() * 0.10;
        return [
            design.pier[0] * body - flute,
            design.pier[1] * body - flute,
            design.pier[2] * body - flute,
        ];
    }
    let mullion_w = 0.075_f32.min(bay_w * 0.16);
    if to_mullion < mullion_w * 0.5 {
        return shade(design.frame, 0.90 + 0.20 * (to_mullion / (mullion_w * 0.5)));
    }
    // A sub-mullion at the centre of a wide module, which is what turns a 1.5 m
    // bay into the 0.75 m module a fabricator actually builds.
    let sub = bay_w * 0.5;
    let to_sub = (bx - sub).abs();
    if bay_w > 1.0 && to_sub < mullion_w * 0.35 {
        return shade(design.frame, 0.86);
    }

    // Storey built up from the floor: shadow, spandrel, coping, glass.
    if ml < design.band_h {
        return shade(design.band, 0.9);
    }
    let spandrel_top = design.band_h + design.spandrel_h;
    if ml < spandrel_top {
        if ml > spandrel_top - design.cap_h {
            // The coping at the head of the spandrel, and the shadow it throws
            // on the panel immediately under it.
            let t = (ml - (spandrel_top - design.cap_h)) / design.cap_h.max(1.0e-3);
            return shade(design.cap, 0.70 + 0.34 * t);
        }
        let seam = (ml / 0.55) - (ml / 0.55).floor();
        if seam < 0.03 {
            return shade(design.spandrel, 0.72);
        }
        // An opaque spandrel is a painted or ceramic-frit panel, so it takes
        // the same trowel-scale variation as any other opaque surface.
        let panel = 1.0 + (value_noise(key.wrapping_add(521), um * 1.4, ml * 1.4) - 0.5) * 0.10;
        return shade(design.spandrel, panel);
    }
    let glass_top = design.storey_m - mullion_w;
    if ml > glass_top {
        return shade(design.frame, 0.94);
    }

    // Vision glass.  Per module it takes a different tone and often a different
    // blind, and that scatter of tones is the entire texture of a real glass
    // tower: 300 identical panes read as a photograph of a wall, and 300
    // different ones read as a building.
    let glass_h = glass_top - spandrel_top;
    let v = ((ml - spandrel_top) / glass_h.max(1.0e-3)).clamp(0.0, 1.0);
    let a = hash(key.wrapping_add(541), bay, storey);
    let b = hash(key.wrapping_add(563), bay, storey);
    let c = hash(key.wrapping_add(587), bay, storey);
    let glass = shade(design.glass, 0.70 + a * 0.80);
    // An openable panel is a different glass from a fixed one, and a quarter of
    // them are openable.
    if b < design.blind {
        let blind_top = 0.30 + c * 0.45;
        if v > blind_top {
            let linen = 0.50 + 0.18 * c;
            let fold = ((v * 9.0).fract() - 0.5).abs() * 0.12;
            return [linen + fold, linen + fold, linen * 0.97 + fold];
        }
        // The gap under a raised blind, where the room shows.
        if v < blind_top - 0.10 {
            return shade(design.glass, 0.55);
        }
    }
    // A horizontal transom splitting a tall vision panel, and the shadow it
    // casts on the glass below it.
    let transom = spandrel_top + glass_h * 0.52;
    if (ml - transom).abs() < mullion_w * 0.4 {
        return shade(design.frame, 0.88);
    }
    if ml < transom + mullion_w * 0.4 {
        return shade(glass, 0.90);
    }
    // A faint vertical gradient: the room is darker at the ceiling, and a
    // neighbouring slab reflects in the upper half of the pane.
    let depth = 0.88 + 0.20 * (1.0 - v);
    let sheen = if ((um * 2.2) - (um * 2.2).floor()) < 0.14 {
        1.06
    } else {
        1.0
    };
    let _ = px;
    [
        glass[0] * depth * sheen,
        glass[1] * depth * sheen,
        glass[2] * depth * sheen,
    ]
}

// ---------------------------------------------------------------------------
// roofs
// ---------------------------------------------------------------------------

/// A flat roof: bitumen membrane in 1 m sheets with a lapped seam, gravel
/// ballast washed to the low spots, and the staining that a roof collects
/// around its plant.
///
/// The alpha channel carries a **height field**, which is what
/// `has_normal_source` promises: the sheet laps, the gravel and the ponding are
/// real relief, and a renderer that differentiates this gets a normal map for
/// free.  Facades and ground floors do not claim it, because their alpha is
/// opacity and must stay so.
pub fn roof_texture(size: usize) -> BakedTexture {
    let n = size.max(16);
    let mut rgba = vec![0_u8; n * n * 4];
    let px_per_m = n as f32 / crate::facades::ROOF_TILE_M;
    for y in 0..n {
        for x in 0..n {
            let u = (x as f32 + 0.5) / px_per_m;
            let v = (y as f32 + 0.5) / px_per_m;
            // Bitumen membrane: 0.15-0.18, slightly cool.  Not a grey lid — a
            // roof is seen from above at every distance, so its value is the
            // city's mid-grey read from a plane and it must not drift light.
            let mut value = 0.165 + (hash(3, x as i32, y as i32) - 0.5) * 0.026;
            let mut relief = 0.0_f32;

            // Rolled sheets, 1.0 m wide, laid in 3 m rolls and welded with a
            // bead.  The lap is the only *linear* feature on a roof, and it is
            // what makes a roof read as a built surface at fifty metres.  The
            // bead width is a physical 40 mm, so it is one texel at 256 and a
            // quarter of one at 64 — hence the resolution-aware threshold.
            let bead = 0.040_f32.min(1.6 / px_per_m);
            let along = (u / 1.0) - (u / 1.0).floor();
            if along < bead {
                value *= 1.30;
                relief += 0.60;
            } else if along < bead * 2.4 {
                value *= 0.88;
            }

            // Loose gravel ballast.  **Fine**: gravel is 20 mm across, so at any
            // sane bake resolution it is grain, not blobs.  An earlier version of
            // this bake put two-metre light and dark patches on the roof and it
            // read as camouflage from the air.
            let drift = (value_noise(29, u * 1.1, v * 1.1) - 0.5) * 0.030;
            let grain = (hash(41, x as i32, y as i32) - 0.5) * 0.042;
            let pebble = if hash(43, x as i32 / 2, y as i32 / 2) < 0.10 {
                0.030
            } else {
                0.0
            };
            value += drift + grain + pebble;
            relief += (grain.abs() + pebble) * 6.0;

            // Ponding: where water stands, the ballast washes off and the
            // bitumen darkens and stays dark.  A real, low-frequency, gentle
            // effect — a few per cent, not a camouflage pattern.
            let pond = (value_noise(53, u * 0.55, v * 0.55) - 0.52).max(0.0);
            value *= 1.0 - pond * 0.34;
            // Sun-bleaching on the exposed runs, and dirt in the shadowed ones.
            value += (value_noise(67, u * 0.30, v * 0.30) - 0.5) * 0.024;

            let offset = (y * n + x) * 4;
            // Bitumen is blue-black, gravel is warm grey, and the mix is
            // somewhere between: a roof is not a neutral surface.
            rgba[offset] = srgb8(value * 1.04);
            rgba[offset + 1] = srgb8(value);
            rgba[offset + 2] = srgb8(value * 0.92);
            rgba[offset + 3] = srgb8(relief.clamp(0.0, 1.0));
        }
    }
    BakedTexture {
        name: "roof".into(),
        width: n,
        height: n,
        tile_width_m: crate::facades::ROOF_TILE_M,
        tile_height_m: crate::facades::ROOF_TILE_M,
        has_normal_source: true,
        rgba,
    }
}

/// What a pitched roof is covered with.
#[derive(Clone, Copy)]
pub enum RoofCovering {
    /// 小青瓦: grey clay pan-tiles, the roof of the Chinese village and suburb.
    GreyClay,
    /// 红瓦: terracotta, the villa roof of the 2000s.
    Terracotta,
    /// 彩钢瓦: blue painted steel sheet, on the farm shed and the factory.
    BlueSteel,
}

impl RoofCovering {
    pub const fn key(self) -> &'static str {
        match self {
            RoofCovering::GreyClay => "roof.tile",
            RoofCovering::Terracotta => "roof.terracotta",
            RoofCovering::BlueSteel => "roof.steel",
        }
    }
    pub const ALL: [RoofCovering; 3] = [
        RoofCovering::GreyClay,
        RoofCovering::Terracotta,
        RoofCovering::BlueSteel,
    ];
}

/// A pitched roof's covering, baked in metres of slope. `u` runs across the
/// slope (along the eave), `v` down it from the ridge, so tile columns and the
/// ribs of a steel sheet run down the fall of the roof, and courses overlap
/// along it. Linear reflectance; the alpha channel is the relief a renderer can
/// differentiate into a normal map.
pub fn pitched_roof_texture(covering: RoofCovering, size: usize) -> BakedTexture {
    let n = size.max(16);
    let tile_m = crate::facades::ROOF_TILE_M;
    let px_per_m = n as f32 / tile_m;
    let mut rgba = vec![0_u8; n * n * 4];
    // The pitch of a tile column, and how far one course is laid over the next.
    let (pitch, course) = match covering {
        RoofCovering::GreyClay => (0.16_f32, 0.21_f32),
        RoofCovering::Terracotta => (0.20, 0.27),
        RoofCovering::BlueSteel => (0.30, 1.0),
    };
    let base: [f32; 3] = match covering {
        RoofCovering::GreyClay => [0.118, 0.122, 0.132],
        RoofCovering::Terracotta => [0.300, 0.115, 0.070],
        RoofCovering::BlueSteel => [0.060, 0.110, 0.210],
    };
    for y in 0..n {
        for x in 0..n {
            let u = (x as f32 + 0.5) / px_per_m;
            let v = (y as f32 + 0.5) / px_per_m;
            let col = (u / pitch).floor();
            let along = u / pitch - col;
            let row = (v / course).floor();
            let down = v / course - row;
            // Each tile is its own: a shade of the clay, a bit of lichen or soot.
            let tile = hash(11, col as i32, row as i32);
            let tone = 0.80 + 0.40 * tile + (value_noise(23, u * 2.3, v * 2.3) - 0.5) * 0.16;
            let grain = (hash(41, x as i32, y as i32) - 0.5) * 0.07;
            let mut relief;
            let mut value;
            match covering {
                RoofCovering::BlueSteel => {
                    // Trapezoid ribs and the flat between them; a lap every
                    // sheet length, not a course.
                    let rib = (1.0 - ((along - 0.5).abs() * 4.2)).clamp(0.0, 1.0);
                    relief = rib;
                    value = tone.max(0.9) * (0.86 + 0.20 * rib);
                    let lap = ((v / 2.0) - (v / 2.0).floor()) < 0.05;
                    if lap {
                        value *= 0.78;
                        relief += 0.3;
                    }
                    // Rust at the fixing lines, streaking down the fall.
                    let streak = (value_noise(61, u * 9.0, v * 0.7) - 0.62).max(0.0) * 1.6;
                    value *= 1.0 - streak * 0.45;
                }
                _ => {
                    // Convex cover tile on the column line, the concave pan between.
                    let curve = 0.5 + 0.5 * (std::f32::consts::TAU * along).cos();
                    relief = curve * 0.7;
                    value = tone * (0.80 + 0.30 * curve);
                    // The lip of each course: a bright edge, then the shadow it throws.
                    if down > 0.86 {
                        let t = (down - 0.86) / 0.14;
                        value *= 1.0 - 0.45 * t;
                        relief += 0.2 * (1.0 - t);
                    }
                    if down < 0.07 {
                        value *= 1.16;
                    }
                    // A cracked or slipped tile, and moss at the shaded foot.
                    if hash(17, col as i32, row as i32) > 0.985 {
                        value *= 0.55;
                    }
                    let moss = (value_noise(71, u * 3.1, v * 3.1) - 0.66).max(0.0) * 1.4;
                    value *= 1.0 - moss * 0.3;
                }
            }
            value *= 1.0 + grain;
            let colour = [
                base[0]
                    * value
                    * if matches!(covering, RoofCovering::GreyClay) {
                        1.0 + moss_tint(u, v)
                    } else {
                        1.0
                    },
                base[1] * value,
                base[2] * value,
            ];
            let offset = (y * n + x) * 4;
            rgba[offset] = srgb8(colour[0]);
            rgba[offset + 1] = srgb8(colour[1]);
            rgba[offset + 2] = srgb8(colour[2]);
            rgba[offset + 3] = srgb8(relief.clamp(0.0, 1.0));
        }
    }
    BakedTexture {
        name: covering.key().into(),
        width: n,
        height: n,
        tile_width_m: tile_m,
        tile_height_m: tile_m,
        has_normal_source: true,
        rgba,
    }
}

/// Weathering on grey tile: a faint warm cast where the sun has dried it.
fn moss_tint(u: f32, v: f32) -> f32 {
    (value_noise(83, u * 0.5, v * 0.5) - 0.5) * 0.10
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luma(linear: [f32; 3]) -> f32 {
        0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
    }

    /// The **median linear albedo** of a baked tile, decoded back out of sRGB.
    fn median_luma(texture: &BakedTexture) -> f32 {
        let n = texture.width;
        let mut values: Vec<f32> = (0..n * n)
            .map(|index| {
                let offset = index * 4;
                luma([
                    srgb_to_linear(texture.rgba[offset] as f32 / 255.0),
                    srgb_to_linear(texture.rgba[offset + 1] as f32 / 255.0),
                    srgb_to_linear(texture.rgba[offset + 2] as f32 / 255.0),
                ])
            })
            .collect();
        values.sort_by(|a, b| a.total_cmp(b));
        values[values.len() / 2]
    }

    fn srgb_to_linear(encoded: f32) -> f32 {
        if encoded <= 0.040_45 {
            encoded / 12.92
        } else {
            ((encoded + 0.055) / 1.055).powf(2.4)
        }
    }

    /// The value rhythm, asserted on the baked pixels a renderer samples.
    ///
    /// A facade whose darkest tenth is close to its median dissolves into a
    /// silhouette at a kilometre: the inter-storey band and the glazing have to
    /// be a real value step, not a tint.
    #[test]
    fn every_tile_keeps_a_value_rhythm_a_kilometre_can_read() {
        for texture in facade_textures(96) {
            let n = texture.width;
            let mut values: Vec<f32> = (0..n * n)
                .map(|index| {
                    let offset = index * 4;
                    luma([
                        srgb_to_linear(texture.rgba[offset] as f32 / 255.0),
                        srgb_to_linear(texture.rgba[offset + 1] as f32 / 255.0),
                        srgb_to_linear(texture.rgba[offset + 2] as f32 / 255.0),
                    ])
                })
                .collect();
            values.sort_by(|a, b| a.total_cmp(b));
            let tenth = values.len() / 10;
            let dark = values[..tenth].iter().sum::<f32>() / tenth as f32;
            let median = values[values.len() / 2];
            let bright = values[values.len() - tenth..].iter().sum::<f32>() / tenth as f32;
            assert!(
                dark < median * 0.70,
                "{}: darkest tenth {dark:.3} is not darker than its median {median:.3}",
                texture.name
            );
            // The bright direction is deliberately a much weaker requirement.
            assert!(
                bright > median * 1.12,
                "{}: brightest tenth {bright:.3} is not lighter than its median {median:.3}",
                texture.name
            );
        }
    }

    /// Distinctness.  Twenty-four tints of one wall is precisely what the
    /// previous port shipped, and it is invisible in any single screenshot and
    /// fatal in a whole district.
    #[test]
    fn the_twenty_four_tiles_are_measurably_different_from_each_other() {
        let textures = facade_textures(64);
        let mut values: Vec<f32> = textures.iter().map(median_luma).collect();
        values.sort_by(|a, b| a.total_cmp(b));
        let (lo, hi) = (values[0], *values.last().unwrap());
        // Points on (chromaticity, normalised value), the scale-invariant pair.
        let points: Vec<[f32; 4]> = textures
            .iter()
            .map(|texture| {
                let n = texture.width;
                let colour = {
                    let mut all: Vec<[f32; 3]> = (0..n * n)
                        .map(|index| {
                            let offset = index * 4;
                            [
                                srgb_to_linear(texture.rgba[offset] as f32 / 255.0),
                                srgb_to_linear(texture.rgba[offset + 1] as f32 / 255.0),
                                srgb_to_linear(texture.rgba[offset + 2] as f32 / 255.0),
                            ]
                        })
                        .collect();
                    all.sort_by(|a, b| luma(*a).total_cmp(&luma(*b)));
                    all[all.len() / 2]
                };
                let total = (colour[0] + colour[1] + colour[2]).max(1.0e-4);
                [
                    colour[0] / total,
                    colour[1] / total,
                    colour[2] / total,
                    ((luma(colour) - lo) / (hi - lo).max(1.0e-4)).clamp(0.0, 1.0),
                ]
            })
            .collect();
        let mut worst = (f32::INFINITY, 0, 1);
        for a in 0..points.len() {
            for b in a + 1..points.len() {
                let d = (0..4)
                    .map(|axis| (points[a][axis] - points[b][axis]).powi(2))
                    .sum::<f32>()
                    .sqrt();
                if d < worst.0 {
                    worst = (d, a, b);
                }
            }
        }
        assert!(
            worst.0 > 0.030,
            "{} and {} are the same building ({}); the palette reads as one facade",
            DESIGNS[worst.1].name,
            DESIGNS[worst.2].name,
            worst.0
        );
        // And the palette has to use its whole range, not cluster.
        assert!(
            hi - lo > 0.35,
            "the palette spans only {:.3} of value",
            hi - lo
        );
    }

    #[test]
    fn every_facade_tile_is_opaque_and_the_right_size() {
        let textures = facade_textures(64);
        assert_eq!(textures.len(), 24);
        for (index, texture) in textures.iter().enumerate() {
            assert_eq!(texture.name, format!("facade/{index:02}"));
            assert_eq!(texture.width, 64);
            assert_eq!(texture.height, 64);
            assert_eq!(texture.rgba.len(), 64 * 64 * 4);
            assert!(
                texture.rgba.chunks(4).all(|pixel| pixel[3] == 255),
                "{} has a hole: a wall must be opaque",
                texture.name
            );
            // The tile's physical size *is* the design's dimensions: one 开间
            // wide, four 层高 tall.
            let design = design(index);
            assert!((texture.tile_width_m - design.bay_m).abs() < 1.0e-6);
            assert!((texture.tile_height_m - tile_height_m(design)).abs() < 1.0e-6);
        }
    }

    /// Every bake in this module is deterministic, and a facade that changes
    /// when you ask twice is a facade nobody can art-direct.
    #[test]
    fn the_bakes_are_deterministic() {
        let first = facade_textures(48);
        let second = facade_textures(48);
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.rgba, b.rgba, "{} is not deterministic", a.name);
        }
    }

    /// The roof is a surface, not a lid: bitumen, laps, gravel and ponding, with
    /// a real height field in the alpha channel as `has_normal_source` promises.
    #[test]
    fn the_roof_is_a_surface_with_relief() {
        let texture = roof_texture(64);
        assert_eq!(texture.name, "roof");
        assert!(texture.has_normal_source);
        assert_eq!(texture.rgba.len(), 64 * 64 * 4);
        let heights: Vec<u8> = texture.rgba.chunks(4).map(|pixel| pixel[3]).collect();
        let lo = *heights.iter().min().unwrap();
        let hi = *heights.iter().max().unwrap();
        assert!(
            hi > lo + 12,
            "the roof has no relief to differentiate ({}..{lo})",
            hi
        );
        let values: Vec<f32> = texture
            .rgba
            .chunks(4)
            .map(|pixel| {
                luma([
                    srgb_to_linear(pixel[0] as f32 / 255.0),
                    srgb_to_linear(pixel[1] as f32 / 255.0),
                    srgb_to_linear(pixel[2] as f32 / 255.0),
                ])
            })
            .collect();
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let median = sorted[sorted.len() / 2];
        // Bitumen and gravel ballast, not a pale grey lid.
        assert!(
            (0.12..=0.30).contains(&median),
            "the roof's median reflectance is {median:.3}"
        );
    }
}
