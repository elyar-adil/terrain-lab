//! Chinese residential elevations: one dwelling unit per tile.
//!
//! The masonry designs in [`super::designs`] are a *bay* grammar — one punched
//! window per 3.6 m 开间, repeated — which is how a western apartment block or
//! a 1980s panel slab is drawn.  A Chinese residential block of the last thirty
//! years is not drawn that way, and the difference is the whole reason a city
//! of those tiles reads as "somewhere else" in photographs:
//!
//! * the elevation repeats per **flat** (户, ~12 m), not per bay, and every flat
//!   is a sequence of *different* openings — a living-room balcony, two
//!   bedroom windows (often 飘窗 bay windows), a kitchen, an air-conditioner
//!   niche with louvres;
//! * neighbouring flats are **mirror images** of each other across the party
//!   wall, so the renderer samples these tiles with mirrored repeat;
//! * the wall is a pale painted render (cream, beige, off-white, peach) whose
//!   colour is chosen per building, with white trim and white window frames;
//! * the glazing is a large fraction of the wall and every pane is different
//!   (curtains, blinds, an empty room, a reflection of the sky).
//!
//! # The texture contract
//!
//! The RGB channels are linear reflectance encoded as sRGB, as everywhere.  The
//! **alpha channel is a surface class**, which the renderer's facade shader
//! reads (`src/city/cityScene.ts`, `patchElevationMaterial`):
//!
//! * [`ALPHA_PAINT`] — body paint.  The RGB is a near-white weathering field and
//!   the renderer multiplies it by the building's paint colour (its vertex
//!   colour), so one tile serves every colour of building.
//! * [`ALPHA_FIXED`] — trim, frames, louvres, shadows: drawn as baked.
//! * `0 ..= ALPHA_GLASS_MAX` — glazing.  The value is a per-pane id the shader
//!   hashes into a curtain, a blind or an empty room, and the pane is given a
//!   glass reflection.
//!
//! The geometry layer (`buildings::cn_residential`) projects balconies, bay
//! windows and AC platforms at exactly the spans this module paints, by reading
//! the same [`Layout`] table, so the projecting faces sample the painted opening
//! they stand in front of and the two can never drift apart.

use crate::textures::BakedTexture;
use worldgen_core::hash::avalanche01;
use worldgen_core::smooth01;

use super::tile::{srgb8, value_noise};

/// 层高 for every elevation in this module.  2.9 m is the 1990s–2010s standard
/// for 住宅 (GB 50096 allows 2.8 – 3.0).
pub const CN_STOREY_M: f32 = 2.9;
/// Storeys per tile, as for the masonry tiles: the `V` datum rule of
/// `buildings::facade_wall` holds for these tiles too.
pub const CN_STOREYS_PER_TILE: u32 = 4;

/// Alpha for body paint (tinted by the renderer).
pub const ALPHA_PAINT: u8 = 255;
/// Alpha for baked colour that must not be tinted.
pub const ALPHA_FIXED: u8 = 192;
/// Glazing carries a pane id in `0 ..= ALPHA_GLASS_MAX`.
pub const ALPHA_GLASS_MAX: u8 = 120;

/// The paint field's own reflectance.  The renderer multiplies it by the
/// building's paint colour, so a paint colour is stated as the *final*
/// reflectance divided by this.
pub const PAINT_BASE: f32 = 0.86;

/// What occupies one span of a flat's elevation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Part {
    /// Plain painted wall.
    Wall,
    /// A white vertical strip — the party-wall pier or a corner accent.
    Strip,
    /// A window: sill and head above the floor, and how many panes across.
    Window { sill: f32, head: f32, panes: u8 },
    /// A 飘窗: a window from a low sill, which the geometry boxes out.
    Bay,
    /// The living-room balcony: a glazed door wall that the geometry fronts
    /// with a slab and a parapet (and, on some buildings, glazing).
    Balcony,
    /// An air-conditioner niche closed by a louvre screen.
    AcNiche,
    /// A stair window at the half landing.
    Stair,
    /// Unitised curtain wall: a mullion every `module` metres, an opaque
    /// spandrel panel at the slab, vision glass above it.
    Curtain { module: f32 },
    /// A stone pier of a grid facade (the geometry projects it).
    Pier,
    /// Aluminium composite panel, with its joints.
    Panel,
}

/// One span of a layout, in metres from the tile's left edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub x0: f32,
    pub x1: f32,
    pub part: Part,
}

const fn span(x0: f32, x1: f32, part: Part) -> Span {
    Span { x0, x1, part }
}

/// Which side of a building an elevation is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// The sunward living side: balconies and bedrooms.
    Front,
    /// The service side: kitchens, bathrooms, the stair.
    Back,
    /// The end wall.
    Gable,
}

/// Window-frame finish.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Frame {
    /// White powder-coated aluminium (1990s–2000s).
    White,
    /// Dark grey aluminium (2010s onwards).
    Dark,
}

/// One flat's elevation.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub key: &'static str,
    pub side: Side,
    /// Tile width: one flat.  Neighbouring tiles are mirrored.
    pub width: f32,
    /// 层高.  The tile is [`CN_STOREYS_PER_TILE`] of these tall.
    pub storey_m: f32,
    pub frame: Frame,
    pub spans: &'static [Span],
}

const fn win(sill: f32, head: f32, panes: u8) -> Part {
    Part::Window { sill, head, panes }
}

/// Bay window sill above the floor and head above the floor.
pub const BAY_SILL_M: f32 = 0.45;
pub const BAY_HEAD_M: f32 = 2.45;
/// Balcony door head above the floor.
pub const DOOR_HEAD_M: f32 = 2.45;
/// Louvre screen top above the floor.
pub const LOUVRE_TOP_M: f32 = 1.35;

/// The layouts.  Three fronts, two backs and a gable: a 2000s 板楼 with bay
/// windows, a 1990s tower with ribbon windows, a 2010s high-rise with dark
/// frames, and their service sides.
pub const LAYOUTS: [Layout; 6] = [
    Layout {
        key: "cnres/front-a",
        side: Side::Front,
        width: 12.0,
        storey_m: CN_STOREY_M,
        frame: Frame::White,
        spans: &[
            span(0.00, 0.30, Part::Strip),
            span(0.30, 0.60, Part::Wall),
            span(0.60, 2.40, Part::Bay),
            span(2.40, 3.10, Part::AcNiche),
            span(3.10, 3.40, Part::Wall),
            span(3.40, 7.60, Part::Balcony),
            span(7.60, 7.90, Part::Wall),
            span(7.90, 9.70, Part::Bay),
            span(9.70, 10.40, Part::AcNiche),
            span(10.40, 10.60, Part::Wall),
            span(10.60, 11.80, win(0.90, 2.40, 2)),
            span(11.80, 12.00, Part::Wall),
        ],
    },
    Layout {
        key: "cnres/front-b",
        side: Side::Front,
        width: 11.4,
        storey_m: CN_STOREY_M,
        frame: Frame::White,
        spans: &[
            span(0.00, 0.35, Part::Strip),
            span(0.35, 0.55, Part::Wall),
            span(0.55, 2.65, win(0.85, 2.45, 3)),
            span(2.65, 3.25, Part::Wall),
            span(3.25, 6.85, Part::Balcony),
            span(6.85, 7.25, Part::Wall),
            span(7.25, 9.05, win(0.85, 2.45, 2)),
            span(9.05, 9.45, Part::Wall),
            span(9.45, 11.05, win(0.85, 2.45, 2)),
            span(11.05, 11.40, Part::Wall),
        ],
    },
    Layout {
        key: "cnres/front-c",
        side: Side::Front,
        width: 12.6,
        storey_m: CN_STOREY_M,
        frame: Frame::Dark,
        spans: &[
            span(0.00, 0.30, Part::Strip),
            span(0.30, 4.80, Part::Balcony),
            span(4.80, 5.10, Part::Wall),
            span(5.10, 7.20, Part::Bay),
            span(7.20, 8.00, Part::AcNiche),
            span(8.00, 8.30, Part::Wall),
            span(8.30, 10.10, win(0.60, 2.50, 2)),
            span(10.10, 10.40, Part::Wall),
            span(10.40, 11.90, win(0.90, 2.40, 2)),
            span(11.90, 12.60, Part::AcNiche),
        ],
    },
    Layout {
        key: "cnres/back-a",
        side: Side::Back,
        width: 12.0,
        storey_m: CN_STOREY_M,
        frame: Frame::White,
        spans: &[
            span(0.00, 0.30, Part::Strip),
            span(0.30, 0.80, Part::Wall),
            span(0.80, 2.30, win(0.90, 2.40, 2)),
            span(2.30, 3.00, Part::AcNiche),
            span(3.00, 4.40, Part::Wall),
            span(4.40, 5.10, win(1.50, 2.40, 1)),
            span(5.10, 6.60, Part::Wall),
            span(6.60, 8.10, win(0.90, 2.40, 2)),
            span(8.10, 8.80, Part::AcNiche),
            span(8.80, 9.80, Part::Wall),
            span(9.80, 10.50, win(1.50, 2.40, 1)),
            span(10.50, 11.30, Part::Wall),
            span(11.30, 12.00, Part::Stair),
        ],
    },
    Layout {
        key: "cnres/back-b",
        side: Side::Back,
        width: 11.4,
        storey_m: CN_STOREY_M,
        frame: Frame::White,
        spans: &[
            span(0.00, 0.35, Part::Strip),
            span(0.35, 1.55, win(0.90, 2.40, 2)),
            span(1.55, 2.25, Part::AcNiche),
            span(2.25, 3.25, Part::Wall),
            span(3.25, 4.75, win(0.90, 2.40, 2)),
            span(4.75, 6.25, Part::Wall),
            span(6.25, 6.95, win(1.50, 2.40, 1)),
            span(6.95, 8.45, Part::Wall),
            span(8.45, 10.25, win(0.90, 2.40, 2)),
            span(10.25, 11.40, Part::Wall),
        ],
    },
    Layout {
        key: "cnres/gable",
        side: Side::Gable,
        width: 6.0,
        storey_m: CN_STOREY_M,
        frame: Frame::White,
        spans: &[
            span(0.00, 0.40, Part::Strip),
            span(0.40, 2.40, Part::Wall),
            span(2.40, 3.60, win(0.90, 2.40, 2)),
            span(3.60, 6.00, Part::Wall),
        ],
    },
];

impl Layout {
    /// The tile's physical height.
    pub fn tile_h(&self) -> f32 {
        self.storey_m * CN_STOREYS_PER_TILE as f32
    }
}

/// 层高 of the commercial elevations: 3.6 m retail and office floors, 3.9 m
/// for a curtain-walled grade-A office.
pub const OFFICE_STOREY_M: f32 = 3.6;
pub const CURTAIN_STOREY_M: f32 = 3.9;
/// Spandrel height of a curtain wall above the slab.
pub const SPANDREL_M: f32 = 0.9;

/// The commercial elevations: a unitised curtain wall, a stone grid with
/// punched windows, and the aluminium-panel ribbon-window block of a 1990s
/// to 2000s 商业楼.  Sampled with the same contract as the residential ones,
/// under a `cncom/` key, which the renderer draws with coated glass and an
/// office interior.
pub const COMMERCIAL: [Layout; 3] = [
    Layout {
        key: "cncom/curtain",
        side: Side::Front,
        width: 6.0,
        storey_m: CURTAIN_STOREY_M,
        frame: Frame::Dark,
        spans: &[span(0.0, 6.0, Part::Curtain { module: 1.5 })],
    },
    Layout {
        key: "cncom/grid",
        side: Side::Front,
        width: 4.2,
        storey_m: OFFICE_STOREY_M,
        frame: Frame::Dark,
        spans: &[
            span(0.00, 0.50, Part::Pier),
            span(0.50, 3.70, win(0.75, 3.15, 3)),
            span(3.70, 4.20, Part::Pier),
        ],
    },
    Layout {
        key: "cncom/ribbon",
        side: Side::Front,
        width: 7.8,
        storey_m: OFFICE_STOREY_M,
        frame: Frame::White,
        spans: &[
            span(0.00, 0.30, Part::Panel),
            span(0.30, 7.50, win(1.05, 3.15, 5)),
            span(7.50, 7.80, Part::Panel),
        ],
    },
];

/// A layout by key.
pub fn layout(key: &str) -> Option<&'static Layout> {
    LAYOUTS
        .iter()
        .chain(COMMERCIAL.iter())
        .find(|layout| layout.key == key)
}

/// Every layout, residential then commercial.
pub fn all_layouts() -> impl Iterator<Item = &'static Layout> {
    LAYOUTS.iter().chain(COMMERCIAL.iter())
}

/// Every elevation, baked at `size` square.
pub fn cn_elevation_textures(size: usize) -> Vec<BakedTexture> {
    all_layouts()
        .enumerate()
        .map(|(index, layout)| bake(layout, index as u32, size))
        .collect()
}

/// White trim and frame reflectances.
const TRIM: [f32; 3] = [0.70, 0.70, 0.67];
const FRAME_WHITE: [f32; 3] = [0.66, 0.67, 0.66];
const FRAME_DARK: [f32; 3] = [0.075, 0.08, 0.085];
/// The louvre slats and the shadow between them.
const LOUVRE: [f32; 3] = [0.62, 0.63, 0.61];
const LOUVRE_GAP: [f32; 3] = [0.10, 0.10, 0.10];

/// A sample: reflectance and class.
#[derive(Clone, Copy)]
struct Texel {
    rgb: [f32; 3],
    alpha: u8,
}

impl Texel {
    fn paint(level: f32) -> Self {
        Self {
            rgb: [level; 3],
            alpha: ALPHA_PAINT,
        }
    }
    fn fixed(rgb: [f32; 3]) -> Self {
        Self {
            rgb,
            alpha: ALPHA_FIXED,
        }
    }
    /// A glazed texel.  The class carries the window's id; the colour channels
    /// carry where in the opening the texel is (`u` across, `v` up, both 0..1)
    /// and the opening's width over 5 m, written so that the renderer's sRGB
    /// decode returns them as plain numbers.  The renderer draws the room,
    /// the curtain and the reflection from those.
    /// An opaque spandrel of a curtain wall: glazed outside, a dark insulated
    /// panel behind.  Flagged by a full blue channel.
    fn spandrel(id: u8, u: f32) -> Self {
        Self {
            rgb: [u.clamp(0.0, 1.0), 0.0, 1.0],
            alpha: id.min(ALPHA_GLASS_MAX),
        }
    }
    fn glass(id: u8, u: f32, v: f32, width_m: f32) -> Self {
        Self {
            rgb: [
                u.clamp(0.0, 1.0),
                v.clamp(0.0, 1.0),
                (width_m / 5.0).clamp(0.0, 0.96),
            ],
            alpha: id.min(ALPHA_GLASS_MAX),
        }
    }
}

fn bake(layout: &Layout, index: u32, size: usize) -> BakedTexture {
    let n = size.max(16);
    let key = 0x5bd1_e995_u32.wrapping_mul(index + 1);
    let mut rgba = vec![0_u8; n * n * 4];
    for y in 0..n {
        for x in 0..n {
            let um = (x as f32 + 0.5) / n as f32 * layout.width;
            // Row 0 is the top of the tile.
            let m = (1.0 - (y as f32 + 0.5) / n as f32) * layout.tile_h();
            let texel = sample(layout, key, um, m);
            let offset = (y * n + x) * 4;
            rgba[offset] = srgb8(texel.rgb[0]);
            rgba[offset + 1] = srgb8(texel.rgb[1]);
            rgba[offset + 2] = srgb8(texel.rgb[2]);
            rgba[offset + 3] = texel.alpha;
        }
    }
    BakedTexture {
        name: layout.key.to_owned(),
        width: n,
        height: n,
        tile_width_m: layout.width,
        tile_height_m: layout.tile_h(),
        has_normal_source: false,
        rgba,
    }
}

/// The paint field at a point: near-white, with the low-frequency blotching of
/// a rendered wall, the darker line under every slab edge, and the rain streaks
/// that run down from every sill.  Everything here is a *multiplier* on the
/// building's paint colour.
fn paint_at(layout: &Layout, key: u32, um: f32, m: f32, ml: f32) -> f32 {
    let blotch = value_noise(key, um / 2.3, m / 1.7) - 0.5;
    let fine = value_noise(key ^ 0x9e37, um / 0.35, m / 0.35) - 0.5;
    let mut level = PAINT_BASE * (1.0 + 0.045 * blotch + 0.025 * fine);
    // The slab edge: a faint horizontal line at every floor, which is what
    // gives a painted tower its storey scale from across the street.
    if ml < 0.05 || ml > layout.storey_m - 0.03 {
        level *= 0.88;
    }
    // Streaks from every sill and from the louvre platforms: water runs off the
    // ledge and carries dirt down the render beneath it.
    for span in layout.spans {
        let (sill, x0, x1) = match span.part {
            Part::Window { sill, .. } => (sill, span.x0, span.x1),
            Part::AcNiche => (0.05, span.x0, span.x1),
            _ => continue,
        };
        if um < x0 - 0.05 || um > x1 + 0.05 {
            continue;
        }
        // Distance below the sill, wrapping into the storey below.
        let below = if ml < sill {
            sill - ml
        } else {
            sill + layout.storey_m - ml
        };
        if below > 1.6 {
            continue;
        }
        let column = value_noise(key ^ 0x51ed, um / 0.07, 0.5);
        let streak = (column - 0.35).max(0.0) / 0.65;
        level *= 1.0 - 0.10 * streak * (1.0 - below / 1.6);
    }
    level
}

/// The id of one opening on one storey of the tile.
fn window_id(key: u32, span_index: usize, storey: i32) -> u8 {
    (avalanche01(key, span_index as i32, storey, 0x6a09) * ALPHA_GLASS_MAX as f32) as u8
}

/// One opening: frame, mullions, glass, and the shadowed reveal around it.
#[allow(clippy::too_many_arguments)]
fn opening(
    layout: &Layout,
    key: u32,
    span_index: usize,
    storey: i32,
    x0: f32,
    x1: f32,
    sill: f32,
    head: f32,
    panes: u8,
    um: f32,
    ml: f32,
) -> Option<Texel> {
    let frame = match layout.frame {
        Frame::White => FRAME_WHITE,
        Frame::Dark => FRAME_DARK,
    };
    let reveal = 0.11;
    // The reveal: the window sits ~150 mm back in the wall, so the head and one
    // jamb are in shadow and the sill catches light.
    if um >= x0 - reveal && um <= x1 + reveal && ml >= sill - 0.06 && ml <= head + reveal {
        let inside = um >= x0 && um <= x1 && ml >= sill && ml <= head;
        if !inside {
            if ml < sill {
                // The sill: a white projecting strip with a dark drip under it.
                return Some(if ml < sill - 0.045 {
                    Texel::fixed([0.18, 0.18, 0.17])
                } else {
                    Texel::fixed(TRIM)
                });
            }
            if ml > head {
                let depth = 1.0 - (ml - head) / reveal;
                return Some(Texel::paint(PAINT_BASE * (0.62 + 0.25 * (1.0 - depth))));
            }
            // Jambs: the left one shaded, the right one half lit.
            return Some(if um < x0 {
                Texel::paint(PAINT_BASE * 0.70)
            } else {
                Texel::paint(PAINT_BASE * 0.90)
            });
        }
        let f = 0.055;
        let u = um - x0;
        let w = x1 - x0;
        let v = ml - sill;
        let h = head - sill;
        if u < f || u > w - f || v < f || v > h - f {
            return Some(Texel::fixed(frame));
        }
        let panes = panes.max(1) as f32;
        let pane_w = w / panes;
        let pane = ((u / pane_w).floor() as i32).clamp(0, panes as i32 - 1);
        let in_pane = u - pane as f32 * pane_w;
        if pane > 0 && in_pane < 0.035 {
            return Some(Texel::fixed(frame));
        }
        // A transom: the top-hung vent of a tall window.
        if h > 1.55 && (v - (h - 0.52)).abs() < 0.03 {
            return Some(Texel::fixed(frame));
        }
        // Sliding sashes overlap: a second frame line a little off the mullion.
        if panes >= 2.0 && pane == 0 && (in_pane - (pane_w - 0.05)).abs() < 0.02 {
            return Some(Texel::fixed(frame));
        }
        return Some(Texel::glass(
            window_id(key, span_index, storey),
            u / w,
            v / h,
            w,
        ));
    }
    None
}

fn sample(layout: &Layout, key: u32, um: f32, m: f32) -> Texel {
    let storey = ((m / layout.storey_m).floor() as i32).clamp(0, CN_STOREYS_PER_TILE as i32 - 1);
    let ml = m - storey as f32 * layout.storey_m;
    let paint = paint_at(layout, key, um, m, ml);
    let Some((span_index, span)) = layout
        .spans
        .iter()
        .enumerate()
        .find(|(_, span)| um >= span.x0 && um < span.x1)
        .or_else(|| layout.spans.iter().enumerate().next_back())
    else {
        return Texel::paint(paint);
    };
    // Openings may draw their reveal and sill a little outside their own span,
    // so the neighbours are asked first.
    for (other_index, other) in layout.spans.iter().enumerate() {
        if other_index == span_index {
            continue;
        }
        if let Part::Window { sill, head, panes } = other.part
            && let Some(texel) = opening(
                layout,
                key,
                other_index,
                storey,
                other.x0,
                other.x1,
                sill,
                head,
                panes,
                um,
                ml,
            )
        {
            return texel;
        }
    }
    match span.part {
        Part::Wall => Texel::paint(paint),
        Part::Strip => {
            // A white strip, with a faint shadow line at each edge so it reads as
            // a projecting pier rather than a stripe of paint.
            let edge = (um - span.x0).min(span.x1 - um);
            if edge < 0.03 {
                Texel::fixed(towards_dark(TRIM, 0.75))
            } else {
                Texel::fixed(TRIM)
            }
        }
        Part::Window { sill, head, panes } => opening(
            layout, key, span_index, storey, span.x0, span.x1, sill, head, panes, um, ml,
        )
        .unwrap_or(Texel::paint(paint)),
        Part::Bay => opening(
            layout,
            key,
            span_index,
            storey,
            span.x0 + 0.06,
            span.x1 - 0.06,
            BAY_SILL_M,
            BAY_HEAD_M,
            3,
            um,
            ml,
        )
        .unwrap_or(Texel::paint(paint)),
        Part::Balcony => {
            // The door wall behind the balcony: a glazed door and its side light
            // to 2.45 m, the soffit shadow above.
            if ml > DOOR_HEAD_M + 0.1 {
                // Under the balcony above: the soffit's shadow on the wall.
                let t = smooth01(((ml - DOOR_HEAD_M - 0.1) / 0.3).min(1.0));
                return Texel::paint(paint * (0.72 + 0.10 * t));
            }
            let x0 = span.x0 + 0.30;
            let x1 = span.x1 - 0.30;
            let panes = ((x1 - x0) / 0.9).round().clamp(2.0, 6.0) as u8;
            opening(
                layout,
                key,
                span_index,
                storey,
                x0,
                x1,
                0.02,
                DOOR_HEAD_M,
                panes,
                um,
                ml,
            )
            .unwrap_or(Texel::paint(paint * 0.84))
        }
        Part::AcNiche => {
            if ml < LOUVRE_TOP_M && ml > 0.08 {
                let slat = ((ml - 0.08) / 0.09).fract();
                let frame = (um - span.x0).min(span.x1 - um) < 0.04
                    || !(0.12..=LOUVRE_TOP_M - 0.04).contains(&ml);
                if frame {
                    Texel::fixed(LOUVRE)
                } else if slat < 0.45 {
                    Texel::fixed(LOUVRE_GAP)
                } else {
                    Texel::fixed(towards_dark(LOUVRE, 0.9 + 0.1 * slat))
                }
            } else {
                Texel::paint(paint)
            }
        }
        Part::Curtain { module } => {
            let k = (um / module).floor();
            let mu = um - k * module;
            let mullion = 0.035;
            if mu < mullion || mu > module - mullion || ml < 0.05 || (ml - SPANDREL_M).abs() < 0.035
            {
                return Texel::fixed(FRAME_DARK);
            }
            let id = window_id(key, span_index * 16 + k as usize, storey);
            if ml < SPANDREL_M {
                Texel::spandrel(id, mu / module)
            } else {
                Texel::glass(
                    id,
                    mu / module,
                    (ml - SPANDREL_M) / (layout.storey_m - SPANDREL_M),
                    module,
                )
            }
        }
        Part::Pier => {
            // Dressed stone: the panel joints every 600 mm up and a vertical
            // joint on the pier's centre line.
            let joint =
                (ml / 0.6).fract() < 0.02 || ((um - (span.x0 + span.x1) * 0.5).abs() < 0.008);
            Texel::paint(if joint { paint * 0.82 } else { paint })
        }
        Part::Panel => {
            let joint = (ml / 1.2).fract() < 0.012;
            Texel::paint(if joint { paint * 0.78 } else { paint })
        }
        Part::Stair => {
            // Half-landing window: lower than the flat windows, and half of a
            // pair, because the tile is mirrored at its edge.
            opening(
                layout,
                key,
                span_index,
                storey,
                span.x0,
                span.x1 + 0.05,
                1.35,
                2.65,
                1,
                um,
                ml,
            )
            .unwrap_or(Texel::paint(paint))
        }
    }
}

fn towards_dark(colour: [f32; 3], factor: f32) -> [f32; 3] {
    [colour[0] * factor, colour[1] * factor, colour[2] * factor]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_tile_each_layout_without_gaps_or_overlaps() {
        for layout in all_layouts() {
            let mut x = 0.0_f32;
            for span in layout.spans {
                assert!(
                    (span.x0 - x).abs() < 1.0e-4,
                    "{}: gap or overlap at {x}",
                    layout.key
                );
                assert!(span.x1 > span.x0, "{}: empty span", layout.key);
                x = span.x1;
            }
            assert!(
                (x - layout.width).abs() < 1.0e-4,
                "{}: spans end at {x}, not {}",
                layout.key,
                layout.width
            );
        }
    }

    /// The glazing share is what makes a Chinese residential elevation read as
    /// one: measured on the photographs, a front elevation is about a third
    /// glass and a back one about a sixth.  A punched-window tile is under a
    /// tenth.
    #[test]
    fn glazing_share_matches_the_reference_band() {
        for (index, layout) in LAYOUTS.iter().enumerate() {
            let texture = bake(layout, index as u32, 128);
            let total = texture.rgba.len() / 4;
            let glass = texture
                .rgba
                .chunks_exact(4)
                .filter(|px| px[3] <= ALPHA_GLASS_MAX)
                .count();
            let share = glass as f32 / total as f32;
            let (low, high) = match layout.side {
                Side::Front => (0.22, 0.50),
                Side::Back => (0.10, 0.30),
                Side::Gable => (0.04, 0.20),
            };
            assert!(
                (low..=high).contains(&share),
                "{}: glazing share {share:.3} outside {low}..{high}",
                layout.key
            );
        }
    }

    /// Commercial glazing: a curtain wall is nearly all glass (vision and
    /// spandrel), a stone grid a little over half, a ribbon block in between.
    #[test]
    fn commercial_glazing_share_matches_its_type() {
        for (index, layout) in COMMERCIAL.iter().enumerate() {
            let texture = bake(layout, index as u32, 128);
            let total = texture.rgba.len() / 4;
            let glass = texture
                .rgba
                .chunks_exact(4)
                .filter(|px| px[3] <= ALPHA_GLASS_MAX)
                .count();
            let share = glass as f32 / total as f32;
            let (low, high) = match layout.key {
                "cncom/curtain" => (0.85, 1.0),
                "cncom/grid" => (0.40, 0.65),
                _ => (0.45, 0.72),
            };
            assert!(
                (low..=high).contains(&share),
                "{}: glazing share {share:.3} outside {low}..{high}",
                layout.key
            );
        }
    }

    #[test]
    fn window_ids_vary_and_stay_in_the_glass_band() {
        let mut seen = std::collections::HashSet::new();
        for storey in 0..4 {
            for span in 0..3 {
                let id = window_id(7, span, storey);
                assert!(id <= ALPHA_GLASS_MAX);
                seen.insert(id);
            }
        }
        assert!(seen.len() >= 9, "pane ids repeat: {seen:?}");
    }

    #[test]
    fn every_class_is_present_and_nothing_else() {
        for (index, layout) in all_layouts().enumerate() {
            let texture = bake(layout, index as u32, 96);
            for px in texture.rgba.chunks_exact(4) {
                assert!(
                    px[3] == ALPHA_PAINT || px[3] == ALPHA_FIXED || px[3] <= ALPHA_GLASS_MAX,
                    "{}: alpha {} is not a class",
                    layout.key,
                    px[3]
                );
            }
        }
    }
}
