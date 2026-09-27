//! Foliage cards, one per species.
//!
//! # Why the leaf card is the whole ball game
//!
//! A tree at street distance is not a mesh of leaves. It is a few hundred quads
//! with a photograph of leaves painted on them, and whether it reads as a tree
//! depends entirely on the alpha channel of that image. The previous port used
//! one generic leaf card for all foliage, tinted per instance — so a `水杉` and
//! a `紫花风铃木` were the same silhouette in different greens, and a canopy read
//! as a cloud of blobs.
//!
//! A card is authored per species, from that species' `LeafForm`, `foliage` and
//! `bloom`, so:
//!
//! * a `水杉`'s card is a fine needle spray and its canopy is a narrow spire;
//! * a `麻楝`'s card is a pinnate compound leaf and its canopy is open and
//!   see-through;
//! * a `锦叶樱仁`'s card is wine-purple because the *foliage* is the feature.
//!
//! The card is drawn in **alpha**, with hard cutout rather than blending: real
//! foliage has no soft edge, and a blended card over a dark background reads as
//! a smudge where a cutout reads as a leaf.
//!
//! # The card's colour is reflectance, and it is written as reflectance
//!
//! A leaf reflects 5-20% and a petal 40-70%. The previous card was drawn as a
//! near-white blob (226) and left for the per-vertex tint to *darken* into a
//! green, which is backwards: the albedo ended up where highlighter paint is.
//! So every colour written here is the species' own reflectance, sRGB-encoded
//! once on the way out, and the card is left to be exactly as dark as the
//! material it depicts. The per-vertex tint that reaches the renderer can then
//! only subtract from it — which is the only direction that has to be safe.
//!
//! # One card is one crop, not one stamp
//!
//! A canopy of 900 quads all sampling the same texels reads as wallpaper. Two
//! things prevent that, and both are geometry decisions made in `trees.rs`:
//! each card takes a different random window of the tile ([`CARD_WINDOW`]), and
//! each card is scaled, spun and tinted independently. Foliage is
//! statistically homogeneous, so any window of a leaf scatter is a valid leaf
//! scatter and the wrap is invisible.

use crate::math::Rng;
use crate::species::{LeafForm, SPECIES, Species};
use crate::textures::BakedTexture;

const TAU: f32 = std::f32::consts::PI * 2.0;

/// The window of the card texture one leaf card samples, as a fraction of the
/// tile.  Less than one so a canopy never stamps the same crop twice: every
/// card takes a different random 80% window, and because a leaf scatter has no
/// structure at any scale, any window of it looks like foliage.
pub const CARD_WINDOW: f32 = 0.80;

/// Physical size of one leaf card for a species, in metres.
///
/// The card is a *spray* of foliage, not a single leaf, so the useful number is
/// the size of the twig section it depicts: a few hundred millimetres for a
/// conifer spray, over a metre for the big-leaved tropical species. It is
/// derived from `leaf_scale` (a fraction of crown radius) against the palette's
/// reference scale, so the two never disagree, and it is also the card texture's
/// `tile_width_m` — one tile is exactly one card, so a card never samples a
/// region of the image that was not drawn.
pub fn card_tile_m(species: &Species) -> f32 {
    (0.90 * species.leaf_scale / 0.30).clamp(0.35, 1.60)
}

/// One leaf card per species.
///
/// Names are `vegetation/leaf/<species-key>` and must match the material key the
/// tree geometry is built under, which ends in `#leaf`.
pub fn leaf_card_textures(size: usize) -> Vec<BakedTexture> {
    SPECIES
        .iter()
        .enumerate()
        .map(|(index, species)| leaf_card(species, index, size.max(48)))
        .collect()
}

/// The generic grass tuft card, `vegetation/tuft`.
///
/// A tuft is a card like any other and it used to ship as an entirely
/// transparent image, which meant the grass tuft prototype drew two solid
/// crossed quads — a green card standing in the park, which is exactly the
/// cartoon tell the rest of this file exists to remove.  One tile is one 1 m
/// card and it carries a real tuft of blades.
pub fn tuft_texture(size: usize) -> BakedTexture {
    let size = size.max(48);
    let mut card = Card::new(size, 0x7af3_11c7);
    // Grass reflectance: low, and far greener in the middle channel than a
    // forest leaf because blades are thin and the mesophyll is deep.
    let blade: [f32; 3] = [0.070, 0.118, 0.052];
    let dry: [f32; 3] = [0.135, 0.128, 0.070];
    let count = 26 + (size as f32 * 0.16) as usize;
    for _ in 0..count {
        let x = card.centre() + card.rng.range(-0.46, 0.46) * size as f32;
        let length = card.rng.range(0.34, 0.80) * size as f32;
        // Blades fan out of one point at the card's foot and arch over, so the
        // tuft has a base and a silhouette rather than being a spray.
        let lean = card.rng.range(-0.62, 0.62);
        let steps = (length * 0.7) as i32;
        for step in 0..=steps {
            let t = step as f32 / steps.max(1) as f32;
            // Quadratic arch, so the tip actually turns over.
            let along = t * length * 0.82;
            let across = lean * length * t * t;
            let px = (x + across).round() as i32;
            let py = (card.foot() - along).round() as i32;
            // Value falls toward the base: a blade's lower half is shaded by
            // every blade above it.
            let value = 0.62 + 0.44 * t;
            card.plot(px, py, [blade[0] * value, blade[1] * value, blade[2] * value]);
            card.plot(px + 1, py, [blade[0] * value, blade[1] * value, blade[2] * value]);
        }
    }
    // A few dead blades in among the green, which is what a city park's grass
    // actually looks like in August.
    for _ in 0..(count / 6) {
        let x = card.centre() + card.rng.range(-0.40, 0.40) * size as f32;
        let length = card.rng.range(0.26, 0.52) * size as f32;
        let lean = card.rng.range(-0.7, 0.7);
        let steps = (length * 0.7) as i32;
        for step in 0..=steps {
            let t = step as f32 / steps.max(1) as f32;
            let px = (x + lean * length * t * t).round() as i32;
            let py = (card.foot() - t * length * 0.8).round() as i32;
            let value = 0.7 + 0.4 * t;
            card.plot(px, py, [dry[0] * value, dry[1] * value, dry[2] * value]);
        }
    }
    card.into_texture("vegetation/tuft", 1.0)
}

/// Opaque coverage a card aims for, as a fraction of the tile.
///
/// This is the number that decides whether a canopy is a painted ball or a
/// tree.  A `麻楝` at 0.58 density and a `柚子` at 0.88 are both closed-crown
/// species but they do not read the same, and the difference is exactly here:
/// 0.45 against 0.53.  A conifer is pushed lower still, because a needle spray
/// is mostly air by construction and a card that filled its own tile would read
/// as moss.
pub fn target_coverage(species: &Species) -> f32 {
    if species.leaf == LeafForm::Needle {
        0.14 + 0.13 * species.density
    } else {
        0.30 + 0.26 * species.density
    }
}

/// Leaf length as a fraction of the tile, and the fraction of its own bounding
/// box that the leaf actually covers.  Together they give the scatter an area,
/// which is what the coverage correction below needs to converge.
fn form_metrics(form: LeafForm) -> (f32, f32) {
    match form {
        LeafForm::Ovate => (0.155, 0.62),
        LeafForm::Elliptic => (0.145, 0.72),
        LeafForm::Cordate => (0.165, 0.58),
        LeafForm::Palmate => (0.235, 0.30),
        LeafForm::Pinnate => (0.300, 0.46),
        LeafForm::Needle => (0.340, 0.14),
    }
}

/// A petal's length as a fraction of the tile, and the fraction of that box the
/// whole flower — five petals and a throat — actually covers.  These exist so a
/// flowering species' blossom gets a *share of the card's area* rather than a
/// share of a leaf-sized count: a crape myrtle's petals are a tenth the area of
/// its leaves, so asking for "the same number of petals as leaves" would put
/// almost nothing on the card.
const PETAL_UNIT: f32 = 0.115;
const PETAL_FILL: f32 = 0.36;

fn leaf_card(species: &Species, index: usize, size: usize) -> BakedTexture {
    let tile = card_tile_m(species);
    let target = target_coverage(species).clamp(0.08, 0.72);
    let (unit, fill) = form_metrics(species.leaf);
    let leaf_area = (unit * size as f32).powi(2) * fill;
    let flower_area = (PETAL_UNIT * size as f32).powi(2) * PETAL_FILL;
    let density = match species.bloom.colour {
        Some(_) => species.bloom.density.clamp(0.0, 1.0),
        None => 0.0,
    };
    // The card's area budget, split by bloom density: leaves get
    // `1 - density` of it and flowers get `density`.  Both counts are then
    // solved from that budget rather than from a single "number of shapes", and
    // the coverage correction below scales them together so the split survives.
    let pixels = size as f32 * size as f32;
    let leaf_budget = target * (1.0 - density) * pixels / leaf_area.max(1.0);
    let petal_budget = target * density * pixels / flower_area.max(1.0);
    let mut leaves = leaf_budget;
    let mut petals = petal_budget;
    let seed = 0x1eaf_0000u32 ^ (index as u32).wrapping_mul(0x9e37_79b9);
    let mut card = Card::new(size, seed);
    for attempt in 0..4 {
        paint(&mut card, species, size, leaves, petals);
        let measured = card.coverage();
        if (measured - target).abs() <= target * 0.06 {
            break;
        }
        let scale = (1.0 - target).max(1.0e-3).ln() / (1.0 - measured).max(0.02).ln();
        let scale = scale.clamp(0.3, 3.0);
        let next_leaves = (leaves * scale).max(0.0);
        let next_petals = (petals * scale).max(0.0);
        if (next_leaves - leaves).abs() < 0.5 && (next_petals - petals).abs() < 0.5 {
            break;
        }
        leaves = next_leaves;
        petals = next_petals;
        // Deliberately *not* cleared on the last attempt: a blank card is worse
        // than a card three percent off its coverage target.
        if attempt < 3 {
            card = Card::new(size, seed);
        }
    }
    card.into_texture(&format!("vegetation/leaf/{}", species.key), tile)
}

/// Draw one card: the leaves, then the blossom.
///
/// The split is `species.bloom.density`, straight from the table, and it is the
/// whole difference between a `紫花风铃木` and a `乌桕`.  A flowering canopy is
/// not a green canopy with white confetti on it: at the table's densities the
/// flower is most of the visible surface and the tree simply *is* its colour.
/// The leaf area is what shows through the gaps, which is how it works on a real
/// tree in flower — the green is behind, not mixed in.
fn paint(card: &mut Card, species: &Species, size: usize, leaves: f32, petals: f32) {
    let (unit, _) = form_metrics(species.leaf);
    let leaves = leaves.round().max(0.0) as usize;
    for index in 0..leaves {
        // Cards are crops of a canopy, so foliage runs off the edge and gets
        // clipped: that is what makes a canopy of cards read as continuous
        // rather than as a field of separate stickers.
        let x = card.rng.range(-0.06, 1.06) * size as f32;
        let y = card.rng.range(-0.06, 1.06) * size as f32;
        let angle = card.rng.range(0.0, TAU);
        let length = size as f32 * unit * card.rng.range(0.78, 1.26);
        // Back of the crown first: leaves drawn later are on the outside, where
        // they see more sky, so they are lighter.  The gradient is the real one.
        let value = 0.78 + 0.24 * (index as f32 / leaves.max(1) as f32);
        let tint = scaled(leaf_colour(species, card.rng.unit()), value);
        match species.leaf {
            LeafForm::Palmate => draw_palmate(card, x, y, length, angle, tint),
            LeafForm::Pinnate => draw_pinnate(card, x, y, length, angle, tint),
            LeafForm::Needle => draw_needle_spray(card, x, y, length, angle, tint, size),
            _ => draw_simple_leaf(card, species.leaf, x, y, length, angle, tint),
        }
    }
    let Some(colour) = species.bloom.colour else {
        return;
    };
    if colour[0] + colour[1] + colour[2] <= 0.0 {
        return;
    }
    let petals = petals.round().max(0.0) as usize;
    for index in 0..petals {
        let x = card.rng.range(-0.06, 1.06) * size as f32;
        let y = card.rng.range(-0.06, 1.06) * size as f32;
        // Petals sit at the canopy's outside, where there is no shading gradient
        // to speak of — they are the lightest thing on the tree.
        let value = 0.90 + 0.10 * (index as f32 / petals.max(1) as f32);
        let jitter = card.rng.range(0.88, 1.02);
        let length = size as f32 * PETAL_UNIT * card.rng.range(0.80, 1.25);
        let angle = card.rng.range(0.0, TAU);
        draw_flower(card, x, y, length, angle, scaled(colour, value * jitter));
    }
}

fn scaled(colour: [f32; 3], value: f32) -> [f32; 3] {
    [colour[0] * value, colour[1] * value, colour[2] * value]
}

/// A leaf's own colour, given one roll.
///
/// Most of a leaf is its own reflectance; a minority in a canopy is a shade
/// older, more yellow and more transparent, and that spread is most of why
/// foliage is not one flat green.  The variation is kept *inside* the species'
/// colour — a yellowing `柚子` leaf is still a dark green — so nothing here can
/// brighten a canopy past what the material really is.
fn leaf_colour(species: &Species, roll: f32) -> [f32; 3] {
    let base = species.foliage;
    let value = 0.84 + 0.16 * roll;
    if roll > 0.86 {
        // Old leaf: chlorophyll partly gone.  Toward yellow, not toward white.
        [base[0] * value * 1.14, base[1] * value * 1.02, base[2] * value * 0.68]
    } else {
        scaled(base, value)
    }
}

/// `0 -> 1` ramp between two points, because `math::smoothstep` is
/// single-argument.
fn ramp(t: f32, lo: f32, hi: f32) -> f32 {
    let t = ((t - lo) / (hi - lo).max(1.0e-5)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Half-width of a simple leaf at `t` along its length: 0 at the petiole, 0 at
/// the tip.  These three curves are the identification.  A `柚子`'s ovate leaf
/// and a `海棠`'s elliptic one are the same size in a different silhouette, and
/// at street distance that difference is the whole species.
fn profile(form: LeafForm, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    match form {
        // Ovate: widest above the middle, tapering to a drip tip, rounded at the
        // foot.
        LeafForm::Ovate => 0.30 * (std::f32::consts::PI * t.powf(0.85)).sin().max(0.0).powf(0.75),
        // Elliptic: symmetric, widest at the middle, and finely serrate — the
        // teeth are a twentieth of the half-width, which is all it takes for the
        // margin to stop reading as a stencil.
        LeafForm::Elliptic => {
            0.26 * (std::f32::consts::PI * t).sin().max(0.0).powf(0.55)
                * (1.0 + 0.05 * (t * 24.0 * TAU).sin())
        }
        // Cordate: an ovate blade with two basal lobes and the sinus between
        // them.  The notch is the identification and it is the reason this
        // profile is not `Ovate` with a different constant in it — a blade with
        // a wide foot and no dip between the lobes is an ovate blade.
        LeafForm::Cordate => {
            let blade = 0.30 * (std::f32::consts::PI * t.powf(0.80)).sin().max(0.0).powf(0.62);
            // A Gaussian cut into the blade, centred just above the lobes.
            let dip = (t - 0.19) / 0.075;
            let sinus = 1.0 - 0.55 * (-dip * dip).exp();
            // The lobes themselves, which protrude below the sinus.
            let lobes = 0.16 * (std::f32::consts::PI * t / 0.20)
                .sin()
                .max(0.0)
                .powf(1.2)
                * (1.0 - ramp(t, 0.14, 0.36));
            blade * sinus + lobes
        }
        _ => 0.0,
    }
}

/// Position of a point in a branch's own frame: `along` up the shoot, `across`
/// to its left.  `angle` is measured from image up, so a leaf at angle 0 points
/// at the top of the card.
fn to_px(centre: [f32; 2], angle: f32, along: f32, across: f32) -> (i32, i32) {
    let (sin, cos) = angle.sin_cos();
    (
        (centre[0] + sin * along + cos * across).round() as i32,
        (centre[1] - cos * along + sin * across).round() as i32,
    )
}

/// A single leaf blade: `Ovate`, `Elliptic` or `Cordate`.
///
/// Scanned along the midrib rather than over the bounding box, so the cost is
/// proportional to the leaf and not to the square of it — which is what makes a
/// 256-pixel bake of sixteen species cheap enough to run inside a test.
fn draw_simple_leaf(
    card: &mut Card,
    form: LeafForm,
    x: f32,
    y: f32,
    length: f32,
    angle: f32,
    colour: [f32; 3],
) {
    let centre = [x, y];
    let steps = ((length * 1.08) as i32).max(2);
    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        let half = profile(form, t) * length;
        if half < 0.35 {
            continue;
        }
        let along = t * length;
        let across_max = half.ceil() as i32 + 1;
        for across in -across_max..=across_max {
            let u = across as f32 / half;
            if u.abs() > 1.0 {
                continue;
            }
            // Midrib raised and paler, fading out toward the tip; margin thinner,
            // more translucent and yellower.  Both are real and both are small,
            // and they straddle zero, so the leaf's mean is its reflectance
            // rather than a brightened version of it.
            let rib = (1.0 - (u.abs() / 0.20).min(1.0)).max(0.0);
            let edge = ramp(u.abs(), 0.74, 1.0);
            let v = (1.0 + 0.14 * rib * (1.0 - t)) * (1.0 + 0.05 * edge - 0.11 * edge);
            let (px, py) = to_px(centre, angle, along, across as f32);
            card.plot(
                px,
                py,
                [
                    colour[0] * v * (1.0 + 0.10 * edge),
                    colour[1] * v,
                    colour[2] * v * (1.0 - 0.26 * edge),
                ],
            );
        }
    }
}

/// A palmate leaf: five lobes off one petiole, with real sinuses between them.
///
/// Drawn as the union of five wedges whose half-width grows *linearly* with
/// distance from the petiole and is pinched near it.  A constant half-width
/// cannot produce a sinus at all — the notch is what falls out of the pinch, so
/// the shape is the shape for a reason and not a decoration.
fn draw_palmate(card: &mut Card, x: f32, y: f32, length: f32, angle: f32, colour: [f32; 3]) {
    const LOBES: [(f32, f32, f32); 5] = [
        // (angle from the midrib, length factor, width factor)
        (-1.15, 0.56, 0.200),
        (-0.50, 0.86, 0.225),
        (0.00, 1.00, 0.240),
        (0.50, 0.86, 0.225),
        (1.15, 0.56, 0.200),
    ];
    let centre = [x, y];
    let reach = length * 1.04;
    let steps = (reach * 1.05) as i32;
    let across_max = (length * 0.62) as i32;
    for step in 0..=steps {
        let ly = step as f32;
        for across in -across_max..=across_max {
            let lx = across as f32;
            let r = lx.hypot(ly);
            if r > reach {
                continue;
            }
            let mut inside = r < length * 0.035;
            let a = lx.atan2(ly.max(0.5));
            for (lobe_angle, lobe_length, lobe_width) in LOBES {
                let s = r / (length * lobe_length);
                if s > 1.0 {
                    continue;
                }
                let spread = (s / 0.30).min(1.0) * (1.0 - ramp(s, 0.70, 1.0));
                let half = length * lobe_width * s * spread;
                if (a - lobe_angle).abs() * r < half {
                    inside = true;
                    break;
                }
            }
            if !inside {
                continue;
            }
            let v = 0.94 + 0.12 * (1.0 - r / reach);
            let (px, py) = to_px(centre, angle, ly, lx);
            card.plot(px, py, scaled(colour, v));
        }
    }
}

/// A pinnate compound leaf: a rachis with paired leaflets and a terminal one.
///
/// The whole compound leaf is on one card, which is the point.  A chinaberry
/// photographed at ten metres shows *leaflets*, not blades, and a card of ovate
/// blades is a different species no matter what colour it is tinted.
fn draw_pinnate(card: &mut Card, x: f32, y: f32, length: f32, angle: f32, colour: [f32; 3]) {
    let centre = [x, y];
    draw_line(
        card,
        centre,
        angle,
        0.0,
        length,
        (length * 0.020).max(0.9),
        scaled(colour, 0.78),
    );
    const PAIRS: usize = 4;
    for pair in 0..PAIRS {
        let along = (0.15 + 0.19 * pair as f32) * length;
        let leaflet = length * (0.30 - 0.030 * pair as f32);
        let (px, py) = to_px(centre, angle, along, 0.0);
        for side in [-1.0_f32, 1.0_f32] {
            draw_simple_leaf(
                card,
                LeafForm::Ovate,
                px as f32,
                py as f32,
                leaflet,
                angle + side * 0.78,
                colour,
            );
        }
    }
    let (px, py) = to_px(centre, angle, 0.94 * length, 0.0);
    draw_simple_leaf(
        card,
        LeafForm::Ovate,
        px as f32,
        py as f32,
        length * 0.20,
        angle,
        colour,
    );
}

/// A conifer spray: a few slender shoots, each carrying dozens of fine needles.
///
/// Coverage is low by construction, which is the honest number: a metasequoia
/// or a Chinese fir is mostly air, and a card that filled its own tile would read
/// as moss.  The geometry side compensates by using more, smaller cards, so the
/// canopy is still dense while the texture stays fine.
fn draw_needle_spray(
    card: &mut Card,
    x: f32,
    y: f32,
    length: f32,
    angle: f32,
    colour: [f32; 3],
    size: usize,
) {
    let centre = [x, y];
    // A needle is a texel at 128 and two at 256.  Anything wider and a conifer
    // card turns into a bundle of rods.
    let needle_width = (size as f32 / 170.0).max(0.85);
    let shoots = 3 + card.rng.int(3);
    for shoot in 0..shoots {
        let a = angle
            + (shoot as f32 - shoots as f32 * 0.5) * 0.30
            + card.rng.range(-0.30, 0.30);
        let shoot_length = length * card.rng.range(0.52, 1.0);
        draw_line(
            card,
            centre,
            a,
            0.0,
            shoot_length,
            (shoot_length * 0.014).max(0.85),
            scaled(colour, 0.74),
        );
        let needles = 12 + (shoot_length * 0.34) as usize;
        for needle in 0..needles {
            let t = 0.06 + 0.92 * (needle as f32 / needles as f32);
            let (px, py) = to_px(centre, a, shoot_length * t, 0.0);
            let side = if needle % 2 == 0 { 1.0 } else { -1.0 };
            let na = a + side * card.rng.range(0.55, 1.25);
            let needle_length = shoot_length * card.rng.range(0.12, 0.32);
            draw_line(
                card,
                [px as f32, py as f32],
                na,
                0.0,
                needle_length,
                needle_width,
                colour,
            );
        }
    }
}

/// One flower: a short spur of petals with a darker throat.
///
/// Petals are the reflective part of a tree, so nothing here darkens them past
/// the table's value and the throat is the only place a petal gets darker.
fn draw_flower(card: &mut Card, x: f32, y: f32, length: f32, angle: f32, colour: [f32; 3]) {
    const PETALS: usize = 5;
    let base = angle + card.rng.range(0.0, TAU);
    for petal in 0..PETALS {
        let a = base + petal as f32 / PETALS as f32 * TAU;
        let l = length * card.rng.range(0.80, 1.06);
        draw_petal(card, x, y, l, a, colour);
    }
    // The throat, where the petals meet: a real flower's centre is its darkest
    // and warmest point, and it is what makes five petals read as a flower
    // rather than as five pale blobs.
    let throat = [colour[0] * 0.58, colour[1] * 0.50, colour[2] * 0.42];
    let radius = (length * 0.10).max(1.0);
    for dy in -radius as i32..=radius as i32 {
        for dx in -radius as i32..=radius as i32 {
            if (dx * dx + dy * dy) as f32 <= radius * radius {
                card.plot(x as i32 + dx, y as i32 + dy, throat);
            }
        }
    }
}

/// A petal: a rounded, slightly cupped blade with a claw at its base.
fn draw_petal(card: &mut Card, x: f32, y: f32, length: f32, angle: f32, colour: [f32; 3]) {
    let centre = [x, y];
    let steps = length as i32;
    for step in 0..=steps {
        let t = step as f32 / steps.max(1) as f32;
        // An ellipse, narrowed to a claw at the base, with a slightly crinkled
        // margin: a crape myrtle's petal is corrugated, and at 256 pixels the
        // corrugation is the only thing that separates it from a smooth blob.
        let mut half = 0.30 * (1.0 - (2.0 * t - 1.0).powi(2)).max(0.0).powf(0.45);
        half *= 0.30 + 0.70 * ramp(t, 0.0, 0.16);
        half *= 1.0 + 0.09 * (t * 9.0 * TAU).sin();
        if half < 0.3 {
            continue;
        }
        let across_max = half * length;
        for across in -(across_max.ceil() as i32)..=(across_max.ceil() as i32) {
            let u = across as f32 / across_max.max(0.001);
            if u.abs() > 1.0 {
                continue;
            }
            let (px, py) = to_px(centre, angle, t * length, across as f32);
            // Petals are translucent, so the thin edge transmits and reads very
            // slightly lighter than the middle.
            let v = 0.95 + 0.05 * ramp(u.abs(), 0.55, 1.0) + 0.05 * t;
            card.plot(px, py, scaled(colour, v));
        }
    }
}

/// A thick line: the rachis, a conifer shoot, a needle, a grass blade.
fn draw_line(
    card: &mut Card,
    centre: [f32; 2],
    angle: f32,
    from: f32,
    length: f32,
    width: f32,
    colour: [f32; 3],
) {
    let steps = length as i32;
    // Sub-pixel widths must collapse to one texel: rounding a 0.9-wide needle up
    // to three texels is the difference between a conifer spray and a bundle of
    // rods, and it is exactly the kind of thing that only shows up in the
    // coverage.
    let half_px = (width * 0.5 - 0.25).round().max(0.0) as i32;
    for step in 0..=steps {
        let t = step as f32 / steps.max(1) as f32;
        for across in -half_px..=half_px {
            let (px, py) = to_px(centre, angle, from + t * length, across as f32);
            card.plot(px, py, colour);
        }
    }
}

/// A raster target: RGBA bytes, alpha strictly 0 or 255.
///
/// Hard cutout, because that is what `alpha_cutout` means and because real
/// foliage has no soft edge.  A blended leaf over a dark background is a smudge;
/// a cut one is a leaf.
struct Card {
    size: usize,
    rgba: Vec<u8>,
    rng: Rng,
}

impl Card {
    fn new(size: usize, seed: u32) -> Self {
        Self {
            size,
            rgba: vec![0; size * size * 4],
            rng: Rng::new(seed),
        }
    }

    fn centre(&self) -> f32 {
        self.size as f32 * 0.5
    }

    /// The tuft's foot: blades spring from the bottom edge of the card, not from
    /// its middle.
    fn foot(&self) -> f32 {
        self.size as f32
    }

    fn plot(&mut self, x: i32, y: i32, colour: [f32; 3]) {
        if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
            return;
        }
        let index = (y as usize * self.size + x as usize) * 4;
        self.rgba[index] = encode_srgb(colour[0]);
        self.rgba[index + 1] = encode_srgb(colour[1]);
        self.rgba[index + 2] = encode_srgb(colour[2]);
        self.rgba[index + 3] = 255;
    }

    fn coverage(&self) -> f32 {
        let opaque = self.rgba.chunks_exact(4).filter(|pixel| pixel[3] > 0).count();
        opaque as f32 / (self.size * self.size) as f32
    }

    fn into_texture(self, name: &str, tile_m: f32) -> BakedTexture {
        BakedTexture {
            name: name.to_owned(),
            width: self.size,
            height: self.size,
            tile_width_m: tile_m,
            tile_height_m: tile_m,
            has_normal_source: false,
            rgba: self.rgba,
        }
    }
}

/// Linear reflectance to sRGB8, once, on the way into the payload.
///
/// The renderer tags these textures `SRGBColorSpace`, so the bytes are an sRGB
/// encoding of an albedo.  Writing a linear value straight out — which is what a
/// 226 meant — puts the material at 0.75 reflectance when it should be at 0.15,
/// and no amount of correct lighting survives that.
fn encode_srgb(linear: f32) -> u8 {
    let value = linear.clamp(0.0, 1.0);
    let encoded = if value <= 0.003_130_8 {
        value * 12.92
    } else {
        1.055 * value.powf(1.0 / 2.4) - 0.055
    };
    (encoded * 255.0 + 0.5) as u8
}

/// Rec. 709 luma of a linear reflectance triple: the perceptual brightness a
/// reflectance triple actually is.  A green's red channel alone is meaningless.
pub fn luma(colour: [f32; 3]) -> f32 {
    0.2126 * colour[0] + 0.7152 * colour[1] + 0.0722 * colour[2]
}

/// Mean and peak **linear** reflectance of a texture's opaque pixels.
///
/// This is what an albedo test has to look at.  An sRGB byte average of a dark
/// green says almost nothing about whether the material is real — 8/255 and
/// 40/255 are 0.002 and 0.021 linear, four orders of magnitude apart on screen,
/// and 103/255 is 0.14.
pub fn opaque_albedo(texture: &BakedTexture) -> (f32, f32) {
    let mut total = 0.0_f32;
    let mut count = 0.0_f32;
    let mut peak = 0.0_f32;
    for pixel in texture.rgba.chunks_exact(4) {
        if pixel[3] == 0 {
            continue;
        }
        let channel = |value: u8| {
            let s = value as f32 / 255.0;
            if s <= 0.040_45 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        };
        let linear = luma([channel(pixel[0]), channel(pixel[1]), channel(pixel[2])]);
        total += linear;
        count += 1.0;
        peak = peak.max(linear);
    }
    if count < 1.0 {
        return (0.0, 0.0);
    }
    (total / count, peak)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::species::by_key;

    fn card_for(key: &str) -> BakedTexture {
        leaf_card(by_key(key).expect("species"), 3, 128)
    }

    fn coverage(texture: &BakedTexture) -> f32 {
        texture.rgba.chunks_exact(4).filter(|p| p[3] > 0).count() as f32
            / (texture.width * texture.height) as f32
    }

    /// The bug this file exists to fix: one generic card, tinted, for every
    /// species.  Two species' cards must not be the same image.
    #[test]
    fn every_species_gets_its_own_card_and_they_are_all_different() {
        let set = leaf_card_textures(64);
        assert_eq!(set.len(), SPECIES.len());
        let mut seen: Vec<u64> = Vec::new();
        for texture in &set {
            assert!(
                texture.name.starts_with("vegetation/leaf/"),
                "{}",
                texture.name
            );
            assert_eq!(texture.rgba.len(), texture.width * texture.height * 4);
            assert!(!texture.has_normal_source, "{} has no height field", texture.name);
            let mut digest = 0_u64;
            for (index, byte) in texture.rgba.iter().enumerate() {
                digest = digest
                    .wrapping_mul(0x0100_0000_01b3)
                    .wrapping_add(*byte as u64 ^ index as u64);
            }
            assert!(
                !seen.contains(&digest),
                "{} is a duplicate card",
                texture.name
            );
            seen.push(digest);
        }
    }

    /// A card is a crop of a canopy, so it must be mostly hole with real leaves
    /// in it.  A card that is nearly solid is a painted ball; a card that is
    /// nearly empty is noise.
    #[test]
    fn a_card_is_mostly_hole_with_real_coverage() {
        for species in SPECIES {
            let texture = card_for(species.key);
            let value = coverage(&texture);
            let target = target_coverage(species);
            assert!(
                (target * 0.70..=target * 1.45).contains(&value),
                "{}'s card covers {value:.3} of its tile against a {target:.3} target",
                species.key
            );
        }
    }

    /// The realism contract, asserted on the bake rather than on the table: the
    /// *mean* opaque reflectance of a card is the material the card depicts, and
    /// nothing in the bake pushes the peak past what a real leaf or a real petal
    /// reflects.  The previous card was a near-white blob left for a green tint
    /// to darken, which is the cartoon signature.
    #[test]
    fn the_baked_albedo_is_a_real_material_and_not_a_brightened_one() {
        for species in SPECIES {
            let texture = card_for(species.key);
            let (mean, peak) = opaque_albedo(&texture);
            let leaf = luma(species.foliage);
            let petal = species.bloom.colour.map(luma).unwrap_or(leaf);
            let density = if species.bloom.colour.is_some() {
                species.bloom.density
            } else {
                0.0
            };
            // What the card is *of*: leaves, or mostly petals in flower.  The bake
            // runs a shade under both (leaves in the shade of the canopy, petals
            // at the outside of it), so the band is generous at the bottom and
            // tight at the top — the top is where a brightened card shows.
            let expected = leaf * (1.0 - density) + petal * density;
            assert!(
                mean > expected * 0.70 && mean < expected * 1.25,
                "{}'s card averages {mean:.3}; it is a crop of {expected:.3} material \
                 (leaf {leaf:.3}, petal {petal:.3} at {density:.2} bloom)",
                species.key
            );
            // And nothing in the bake exceeds its own material.  A petal's
            // translucent edge and a leaf's midrib both read brighter than the
            // diffuse value, which is the whole allowance.
            let ceiling = if density > 0.5 {
                petal * 1.12
            } else {
                leaf * 1.25
            };
            assert!(
                peak <= ceiling,
                "{} peaks at {peak:.3} against a {ceiling:.3} ceiling; anything above \
                 that is highlighter paint",
                species.key
            );
        }
    }

    /// A `Needle` species must be visibly finer than an `Ovate` one.  Both halves
    /// matter: the card is sparser, *and* it is physically smaller, so the
    /// canopy ends up carrying more, finer elements.
    #[test]
    fn a_conifer_card_is_finer_than_a_broadleaf_one() {
        let needle = SPECIES
            .iter()
            .find(|s| s.leaf == LeafForm::Needle)
            .expect("the palette has conifers");
        let ovate = SPECIES
            .iter()
            .find(|s| s.leaf == LeafForm::Ovate)
            .expect("the palette has broadleaves");
        let needle_coverage = coverage(&card_for(needle.key));
        let ovate_coverage = coverage(&card_for(ovate.key));
        assert!(
            needle_coverage < ovate_coverage * 0.72,
            "{} covers {needle_coverage:.2} of its tile against {}'s {ovate_coverage:.2}; \
             a needle spray is mostly air",
            needle.key,
            ovate.key
        );
        // And the physical size: `leaf_scale` is a fraction of crown radius, so a
        // conifer's card is genuinely a smaller object on a smaller tree.
        assert!(
            card_tile_m(needle) < card_tile_m(ovate) * 0.80,
            "{}'s card is {} m across, {}'s is {} m",
            needle.key,
            card_tile_m(needle),
            ovate.key,
            card_tile_m(ovate)
        );
    }

    /// Opaque pixels in each row of a card.
    fn row_widths(card: &Card) -> Vec<usize> {
        (0..card.size)
            .map(|y| {
                (0..card.size)
                    .filter(|x| card.rgba[(y * card.size + x) * 4 + 3] > 0)
                    .count()
            })
            .collect()
    }

    /// Number of separate opaque runs across one row.  A blade is one run; a
    /// palmate leaf is three or more, because the sinuses go all the way in; a
    /// compound leaf is a rachis plus pairs of leaflets, which is at least three.
    fn runs_along_row(card: &Card, y: usize) -> usize {
        let mut runs = 0;
        let mut inside = false;
        for x in 0..card.size {
            let on = card.rgba[(y * card.size + x) * 4 + 3] > 0;
            if on && !inside {
                runs += 1;
            }
            inside = on;
        }
        runs
    }

    /// One leaf of `form`, drawn big and alone, standing up from a known base
    /// row.  Returns the leaf's own rows — trimmed to the blade — so a
    /// measurement at fraction `t` of the leaf's length means that fraction and
    /// not that fraction of the card.
    fn blade(form: LeafForm, size: usize, length: f32) -> (Card, Vec<usize>, usize, usize) {
        let base = size - 12;
        let mut card = Card::new(size, 0x51b0);
        draw_simple_leaf(
            &mut card,
            form,
            size as f32 * 0.5,
            base as f32,
            length,
            0.0,
            [0.18, 0.24, 0.13],
        );
        let rows = row_widths(&card);
        let first = rows.iter().position(|w| *w > 0).unwrap_or(0);
        let last = rows.iter().rposition(|w| *w > 0).unwrap_or(0);
        (card, rows[first..=last].to_vec(), first, last)
    }

    /// The `LeafForm` enum is the claim that leaf *shape* carries the species.
    /// These are the silhouettes, measured rather than asserted in a comment: a
    /// serrate elliptic margin on a `海棠`, a drip-tipped ovate blade on a
    /// `柚子`, and a `黄槿` with a heart-shaped foot.
    #[test]
    fn the_leaf_forms_have_the_silhouettes_the_table_promises() {
        let size = 200_usize;
        let length = size as f32 * 0.78;
        let mut forms = Vec::new();
        for form in [LeafForm::Ovate, LeafForm::Elliptic, LeafForm::Cordate] {
            let (_, rows, _, _) = blade(form, size, length);
            let width = |t: f32| rows[((1.0 - t) * (rows.len() - 1) as f32).round() as usize];
            let widest = rows.iter().copied().max().unwrap_or(0);
            assert!(widest > 20, "{form:?} is {widest} px wide, which is nothing");
            assert_eq!(*rows.last().unwrap(), 0, "{form:?} has no tip");
            assert!(width(0.02) < widest / 2, "{form:?} has no foot");
            forms.push((form, rows));
        }
        // Widest above the middle for an ovate blade, at the middle for an
        // elliptic one.  That is the difference between the two curves, and it
        // is the difference between a `柚子` and a `海棠`.
        for (form, rows) in &forms {
            let width = |t: f32| rows[((1.0 - t) * (rows.len() - 1) as f32).round() as usize] as f32;
            let ratio = width(0.30) / width(0.70).max(1.0);
            let expected = if *form == LeafForm::Elliptic {
                0.88..=1.14
            } else {
                1.12..=2.40
            };
            assert!(
                expected.contains(&ratio),
                "{form:?} is {ratio:.2} as wide at 30% of its length as at 70%"
            );
        }
        // The cordate's sinus: narrower just above the petiole than either below
        // it (the basal lobe) or above it (the blade).  A leaf with a lobe but no
        // notch between the lobes is an ovate leaf, and the notch is the whole
        // identification of a `黄槿`.
        let cordate = &forms[2].1;
        let width = |t: f32| cordate[((1.0 - t) * (cordate.len() - 1) as f32).round() as usize] as f32;
        let sinus = width(0.17);
        assert!(
            sinus < width(0.08) * 1.02 && sinus < width(0.34) * 0.96,
            "the cordate leaf has no basal sinus: {sinus:.0} at t=0.17 against \
             {:.0} at t=0.08 and {:.0} at t=0.34",
            width(0.08),
            width(0.34)
        );
    }

    /// A pinnate card carries a *compound* leaf; a palmate one is lobed; a
    /// conifer one is a spray.  Each is measured on a single element against its
    /// own cross-section, so a card cannot pass by being densely scattered.
    #[test]
    fn compound_lobed_and_needle_elements_are_what_the_table_claims() {
        let size = 200_usize;
        let length = size as f32 * 0.62;
        let base = size - 16;
        let centre = size as f32 * 0.5;
        let tint = [0.18, 0.24, 0.13];
        let row_at = |t: f32| base - (t * length).round() as usize;

        // A blade is one continuous run of tissue.
        let (card, _, first, last) = blade(LeafForm::Ovate, size, length);
        assert_eq!(runs_along_row(&card, (first + last) / 2), 1, "a blade is one run");

        // A palmate leaf's sinuses go in far enough to leave separate lobes.
        let mut card = Card::new(size, 0x77aa);
        draw_palmate(&mut card, centre, base as f32, length, 0.0, tint);
        let palmate = runs_along_row(&card, row_at(0.22));
        assert!(
            palmate >= 3,
            "a palmate leaf is {palmate} run(s) across at 22% of its length; the sinuses \
             have to go all the way in or it is an ovate leaf"
        );

        // A compound leaf is a rachis with paired leaflets: at any height between
        // two pairs there are at least three separate things on the cross-section.
        let mut card = Card::new(size, 0x77ab);
        draw_pinnate(&mut card, centre, base as f32, length, 0.0, tint);
        let pinnate = runs_along_row(&card, row_at(0.42));
        assert!(
            pinnate >= 3,
            "a compound leaf is {pinnate} run(s) across at 42% of its length; the leaflets \
             have to be separate from each other and from the rachis"
        );

        // And a conifer spray is a twentieth of a leaf's box.  This is the
        // assertion that a `Needle` species cannot quietly get a broadleaf card.
        let mut card = Card::new(size, 0x77ac);
        draw_needle_spray(
            &mut card,
            centre,
            centre,
            length,
            0.0,
            tint,
            size,
        );
        let spray = card.rgba.chunks_exact(4).filter(|p| p[3] > 0).count() as f32
            / (size * size) as f32;
        let (blade_card, _, _, _) = blade(LeafForm::Ovate, size, length);
        let blade_fill = blade_card.rgba.chunks_exact(4).filter(|p| p[3] > 0).count() as f32
            / (size * size) as f32;
        assert!(blade_fill > 0.04, "an ovate blade only fills {blade_fill:.3} of the card");
        assert!(
            spray < blade_fill * 0.16,
            "a needle spray fills {spray:.4} of the card against a blade's {blade_fill:.3}; \
             a conifer is mostly air"
        );
    }

    /// Blossom has to be visible, or a flowering species is just a green tree.
    #[test]
    fn blossom_takes_over_the_card_in_proportion_to_its_density() {
        for species in SPECIES {
            let (mean, _) = opaque_albedo(&card_for(species.key));
            let leaf = luma(species.foliage);
            let petal = species.bloom.colour.map(luma).unwrap_or(leaf);
            let density = if species.bloom.colour.is_some() {
                species.bloom.density
            } else {
                0.0
            };
            let expected = leaf * (1.0 - density) + petal * density;
            if density > 0.75 {
                assert!(
                    mean > leaf * 1.35,
                    "{} blooms at {density:.2} but its card only averages {mean:.3} against \
                     a {leaf:.3} leaf floor; a flowering tree is a coloured tree",
                    species.key
                );
            }
            if density < 0.05 {
                assert!(
                    mean < leaf * 1.20,
                    "{} does not flower ({density:.2}) yet its card averages {mean:.3} \
                     against its own {leaf:.3} foliage",
                    species.key
                );
            }
            if density > 0.05 && density < 0.75 {
                assert!(
                    mean > leaf * 1.12 && mean < expected * 1.30,
                    "{} at {density:.2} bloom averages {mean:.3}, outside the leaf-to-petal \
                     range it should be in",
                    species.key
                );
            }
        }
    }

    /// The card's physical size is its tile size, so one card is exactly one
    /// image and a card never samples a region of the tile that was not drawn.
    #[test]
    fn one_card_is_exactly_one_tile() {
        for species in SPECIES {
            let texture = card_for(species.key);
            let tile = card_tile_m(species);
            assert_eq!(texture.tile_width_m, tile);
            assert_eq!(texture.tile_height_m, tile);
            assert!(
                (0.30..=1.70).contains(&tile),
                "{}'s card is {tile:.2} m across, which is not a twig section",
                species.key
            );
        }
        assert!(CARD_WINDOW > 0.0 && CARD_WINDOW < 1.0);
    }

    /// The tuft used to ship as a fully transparent image, which meant the tuft
    /// prototype drew a solid green box in every park.
    #[test]
    fn the_tuft_is_a_tuft() {
        let texture = tuft_texture(96);
        let value = coverage(&texture);
        assert!(
            (0.06..=0.60).contains(&value),
            "the tuft card covers {value:.2}; it was transparent once, which drew a green box"
        );
        let (mean, peak) = opaque_albedo(&texture);
        assert!(mean < 0.20 && peak < 0.30, "grass is not a {mean:.2} material");
    }

    /// Bake cost.  Sixteen cards at the payload's real resolution, in debug,
    /// inside a test — this is the number that decides whether leaf detail is
    /// affordable at all.
    #[test]
    fn the_sixteen_card_bake_is_affordable() {
        let start = std::time::Instant::now();
        let set = leaf_card_textures(256);
        assert_eq!(set.len(), 16);
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_secs() < 20,
            "baking sixteen 256-pixel leaf cards took {elapsed:?}"
        );
        let bytes: usize = set.iter().map(|t| t.rgba.len()).sum();
        assert!(bytes < 8 * 1024 * 1024, "the leaf cards are {bytes} bytes of payload");
    }
}
