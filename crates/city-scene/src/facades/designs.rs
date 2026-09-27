//! The facade design table: one authored, dimensioned elevation per tile.
//!
//! # The design is a rule table, not a mood board
//!
//! Every design is expressed in **construction dimensions**, and the geometry
//! and the bake both derive from those numbers — nothing is eyeballed.  The
//! defaults are the Chinese residential code band, and the tests at the bottom
//! of this file hold the table to it:
//!
//! | quantity | band (GB) | where it lives |
//! |---|---|---|
//! | 住宅层高 storey height | 2.8 – 3.0 m (公建 3.2 m and up) | [`FacadeDesign::storey_m`] |
//! | 开间 structural bay | 3.3 / 3.6 / 3.9 m | [`FacadeDesign::bay_m`] (also the tile's physical width) |
//! | 窗台高 sill height | 0.9 m | [`FacadeDesign::sill_m`] |
//! | 窗高 opening height | 1.4 – 1.5 m habitable (stair/bath 1.2 – 1.3) | [`FacadeDesign::open_h`] |
//! | 窗宽 opening width | 0.5 – 1.6 m by room type | [`FacadeDesign::open_w`] |
//! | 飘窗 bay band | 2000s tile-clad stock carries one | [`FacadeDesign::bay_band`] |
//!
//! A multi-bay design states the *bay* and splits it into half- or third-bay
//! sub-modules (`bays`), which is how a 3.6 m 开间 carries a bedroom and a
//! kitchen window.  The bake jitters each opening ±50 mm around the sill and
//! shifts it a little in its clear wall — construction tolerance, not design
//! intent, and the only per-opening variation allowed to touch a dimension.
//!
//! # Albedo is reflectance, and reflectance is not a style knob
//!
//! Every colour in [`DESIGNS`] is a **linear reflectance**, not a screen colour.
//! That distinction is the whole difference between a render that looks
//! photographed and one that looks like a blockout:
//!
//! | material | linear reflectance |
//! |---|---|
//! | white cement render, new | 0.65 – 0.75 |
//! | warm beige render, weathered | 0.40 – 0.60 |
//! | grey fair-faced concrete | 0.22 – 0.35 |
//! | dark glazed wall tile | 0.10 – 0.18 |
//! | red clay brick / roof tile | 0.15 – 0.25 |
//! | dark bronze anodised aluminium | 0.08 – 0.15 |
//! | **curtain-wall glass** | **0.04 – 0.12** |
//!
//! Curtain-wall glass is the one people get wrong most often, in both
//! directions. It is *dark* in albedo — a tinted glass panel reflects 6 % of the
//! light that hits it; it looks bright only because a few per cent of the sky is
//! very bright and the surface is smooth. The renderer gets that for free from
//! low roughness plus the sky IBL, so a bright "glass" albedo double-counts the
//! sky and turns a tower into a white slab.

use crate::facades::{GROUND_FLOOR_TILE_H, GROUND_STOREY_M, STOREY_M, STOREYS_PER_TILE, FACADE_TILE_H};
use crate::facades::tile::bay_clearance;

/// What a wall is *made of*.  This, not the colour, is what makes two tiles
/// different buildings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cladding {
    /// Smooth cement render and paint: the default 板楼 wall.
    Render,
    /// Small mosaic tiles with visible grout, and the accent-tile motif that
    /// every 2000s Chinese apartment block carries.
    Mosaic,
    /// Clay brick in running bond, with mortar.
    Brick,
    /// Cast concrete panels with a recessed joint.
    Panel,
    /// Unitised curtain wall: spandrel, vision glass and a mullion grid.
    CurtainWall,
}

/// One facade tile, authored.
///
/// Every length is in **metres**, not in tile fractions, so a design reads the
/// same at any bake resolution and can be checked against a real building: a
/// window is 1.5 m wide and its sill is 0.9 m off the floor because those are
/// the numbers a window is.
#[derive(Debug, Clone, Copy)]
pub struct FacadeDesign {
    /// What this tile is, for a human reading the table.
    pub name: &'static str,
    pub cladding: Cladding,
    /// Storey height in metres — 层高.  Residential stock is 2.8 (1980s panel
    /// and brick), 2.9 (1990s onwards render and mosaic) or 3.0 (podium
    /// civic); curtain wall is office 层高 3.4.  The tile covers exactly
    /// [`STOREYS_PER_TILE`] of these, so the tile's physical height is
    /// `storey_m * STOREYS_PER_TILE` and a floor line is always a quarter tile.
    pub storey_m: f32,
    /// Structural bay in metres — 开间, one of 3.3 / 3.6 / 3.9 on masonry.
    /// This is also the tile's physical width: the baked texture covers one
    /// bay, so the renderer's horizontal repeat is the bay itself.
    pub bay_m: f32,
    /// Window columns across the bay.  A pier sits on every column boundary,
    /// so the tile's own seam is always buried in one.
    pub bays: u8,
    /// The 2000s 飘窗 band: the geometry layer projects a bay-window sill and
    /// head at [`crate::buildings::BAY_WINDOW_PROJECTION_M`] on every storey of
    /// walls clad in this design.  True for the tile-clad stock of the 2000s.
    pub bay_band: bool,
    /// Wall field reflectance.
    pub wall: [f32; 3],
    /// Reflectance jitter, so no two square metres of a wall match.
    pub grain: f32,
    /// Vertical pier: reflectance and width in metres.  On a curtain wall this
    /// is the pale stone fin instead, which is how a Chinese glass tower reads
    /// as vertical stripes from a kilometre away.
    pub pier: [f32; 3],
    pub pier_w: f32,
    /// A *wider* pier on the tile boundary only, in metres.  Zero on tiles
    /// where every bay is the same.  This is the difference between a one-bay
    /// repeat and a two-bay structural module.
    pub major_pier_w: f32,
    /// Inter-storey band: reflectance and height in metres above the floor.  The
    /// bake puts a dark slab shadow below it and a pale drip lip above it, so
    /// the storey line is a light-dark-light pair the way a real slab edge is.
    pub band: [f32; 3],
    pub band_h: f32,
    /// Curtain wall only.  Spandrel panel reflectance and height, and the pale
    /// coping at its head.
    pub spandrel: [f32; 3],
    pub spandrel_h: f32,
    pub cap: [f32; 3],
    pub cap_h: f32,
    /// Vision glass reflectance, and the frame / mullion around it.
    pub glass: [f32; 3],
    pub frame: [f32; 3],
    /// Clear opening: width, height, sill height above the floor (窗台, 0.9),
    /// and how deep the painted reveal is.
    pub open_w: f32,
    pub open_h: f32,
    pub sill_m: f32,
    pub reveal_m: f32,
    /// Painted surround above the opening: reflectance and height, zero for none.
    pub lintel: [f32; 3],
    pub lintel_h: f32,
    /// Probability an opening has a blind down, and that it carries a security
    /// grille.  Per-window variation is the difference between a building and
    /// a texture.
    pub blind: f32,
    pub grille: f32,
    /// Tile module in metres.  Read for `Mosaic`; brick sizes are fixed.
    pub module_m: f32,
    /// The accent-tile motif, `Mosaic` only.
    pub motif: bool,
}

/// A starting point for the sixteen masonry tiles, so a design only states what
/// makes it different.
///
/// The numbers here are deliberately unremarkable: a 0.42 mid-value wall, a pale
/// concrete pier, a dark inter-storey band, and window glass a *tenth* of the
/// wall's reflectance.  Almost every way a facade tile goes wrong is a way of
/// departing from this, so the table is written as departures.  The dimensions
/// are the standard 1990s residential fit: 层高 2.9, one window per 3.6 m 开间,
/// 窗台 0.9, 窗高 1.4.
const MASONRY: FacadeDesign = FacadeDesign {
    name: "masonry",
    cladding: Cladding::Render,
    storey_m: 2.9,
    bay_m: 3.6,
    bay_band: false,
    // One window column per bay: the 板楼 structural bay.  A 1.8 m module with
    // a 1.2 m window leaves 0.6 m of wall, and wall between openings is why
    // the previous port's tiles read as black bars.
    bays: 1,
    wall: [0.420, 0.400, 0.340],
    grain: 0.020,
    pier: [0.560, 0.548, 0.520],
    pier_w: 0.56,
    major_pier_w: 0.0,
    band: [0.300, 0.285, 0.255],
    band_h: 0.34,
    spandrel: [0.300, 0.285, 0.255],
    spandrel_h: 0.90,
    cap: [0.560, 0.552, 0.530],
    cap_h: 0.20,
    glass: [0.055, 0.062, 0.072],
    /// White-painted aluminium.  A window frame is the *brightest* thing on a
    /// rendered wall, and getting that wrong is why so many procedural facades
    /// read as a grid of holes.
    frame: [0.680, 0.672, 0.650],
    open_w: 1.50,
    open_h: 1.40,
    sill_m: 0.90,
    reveal_m: 0.07,
    lintel: [0.508, 0.496, 0.472],
    lintel_h: 0.16,
    blind: 0.32,
    grille: 0.06,
    module_m: 0.095,
    motif: false,
};

/// A starting point for the eight curtain-wall tiles.
///
/// The three numbers that matter here, and that get got wrong most often:
///
/// * the **glass is 0.04-0.12** — a tinted pane reflects a few per cent and is
///   bright only because the sky it reflects is very bright;
/// * the **mullion is about half the glass** — dark bronze against a lighter
///   pane, which is what makes the grid read as a grid and not as a decal;
/// * the **band at the floor line is the darkest thing on the tile** — the
///   shadow under the spandrel, and the reason a tower's storey lines survive
///   at a kilometre even though its glazing does not.
///
/// Office 层高 3.4, on a 3.0 m grid split into two 1.5 m curtain modules.
const GLAZED: FacadeDesign = FacadeDesign {
    name: "glazed",
    cladding: Cladding::CurtainWall,
    storey_m: 3.4,
    bay_m: 3.0,
    bay_band: false,
    bays: 2,
    wall: [0.12, 0.13, 0.14],
    grain: 0.0,
    pier: [0.470, 0.472, 0.462],
    pier_w: 0.22,
    major_pier_w: 0.0,
    band: [0.024, 0.026, 0.028],
    band_h: 0.22,
    spandrel: [0.115, 0.120, 0.126],
    spandrel_h: 1.00,
    cap: [0.400, 0.400, 0.390],
    cap_h: 0.22,
    glass: [0.062, 0.070, 0.082],
    frame: [0.034, 0.036, 0.040],
    open_w: 1.0,
    open_h: 1.6,
    sill_m: 0.0,
    reveal_m: 0.0,
    lintel: [0.4, 0.4, 0.4],
    lintel_h: 0.0,
    blind: 0.28,
    grille: 0.0,
    module_m: 0.095,
    motif: false,
};

/// The 24 facade tiles.
///
/// **Sixteen are masonry** — render, mosaic tile, brick and cast panel — and the
/// family is led by **warm beige and buff**, the way a Chinese residential
/// district actually is, with grey and tile-clad slabs behind it and exactly two
/// near-white tiles so the city never goes pale.  Nothing here is brighter than
/// 0.58: a Chinese skyline is mostly mid-value, and a city of white monoliths is
/// the most recognisable signature of a fake render.
///
/// **Eight are curtain wall**, and they are separated from each other by their
/// *non-glass* elements — the spandrel, the coping and the fin — because eight
/// dark tinted glasses are indistinguishable from one another by albedo alone,
/// and the non-glass elements are what a viewer actually reads.
pub static DESIGNS: [FacadeDesign; 24] = [

    // -- masonry, warm: the sound of a residential district -----------------
    FacadeDesign {
        name: "ban-plaster-tan",
        // The signature 板楼 wall: warm beige cement render, a 560 mm pale
        // concrete pier on every 3.6 m structural bay, and a single punched
        // window per bay.  Most of a Chinese residential district is made of
        // this building.  层高 2.9, 窗台 0.9, 窗高 1.5.
        wall: [0.480, 0.400, 0.290],
        pier: [0.575, 0.562, 0.530],
        band: [0.330, 0.292, 0.230],
        glass: [0.052, 0.058, 0.068],
        frame: [0.690, 0.680, 0.655],
        open_w: 1.55,
        open_h: 1.50,
        blind: 0.34,
        grille: 0.05,
        ..MASONRY
    },
    FacadeDesign {
        name: "plaster-sand",
        // Warm sand render with a heavier string course: the 1990s block.
        wall: [0.395, 0.335, 0.230],
        grain: 0.022,
        pier: [0.520, 0.508, 0.478],
        pier_w: 0.62,
        bay_m: 3.3,
        band: [0.270, 0.240, 0.185],
        band_h: 0.46,
        open_w: 1.45,
        open_h: 1.40,
        blind: 0.28,
        ..MASONRY
    },
    FacadeDesign {
        name: "ban-plaster-ochre",
        // The same building forty years weathers darker and browner.  Not a
        // tint of the tile above: a third of a stop darker with the hue pushed
        // towards olive, which is what water staining does to a render.
        // 1980s stock: 层高 2.8, 3.3 m 开间.
        wall: [0.300, 0.240, 0.135],
        grain: 0.026,
        pier: [0.470, 0.455, 0.420],
        pier_w: 0.66,
        storey_m: 2.8,
        bay_m: 3.3,
        band: [0.205, 0.172, 0.115],
        band_h: 0.38,
        glass: [0.048, 0.052, 0.058],
        frame: [0.600, 0.592, 0.570],
        open_w: 1.40,
        open_h: 1.40,
        blind: 0.46,
        grille: 0.24,
        ..MASONRY
    },
    FacadeDesign {
        name: "plaster-terracotta",
        // Terracotta render: the other end of the warm family, and the colour of
        // a provincial walk-up.  2.8 m 层高, 3.3 m 开间.
        wall: [0.360, 0.175, 0.095],
        grain: 0.026,
        pier: [0.520, 0.500, 0.460],
        pier_w: 0.60,
        storey_m: 2.8,
        bay_m: 3.3,
        band: [0.245, 0.140, 0.090],
        band_h: 0.38,
        glass: [0.048, 0.050, 0.056],
        frame: [0.620, 0.610, 0.585],
        open_w: 1.45,
        open_h: 1.40,
        blind: 0.35,
        grille: 0.18,
        ..MASONRY
    },
    FacadeDesign {
        name: "plaster-khaki",
        // Olive-khaki render — the greenish cast a cement mix goes when it is
        // made with sand rather than lime.  Present on half the stock in a
        // northern city and almost absent in a southern one.  A 3.9 m 开间
        // split into two 1.95 m sub-bays.
        wall: [0.375, 0.375, 0.270],
        grain: 0.022,
        pier: [0.505, 0.505, 0.440],
        pier_w: 0.34,
        major_pier_w: 0.44,
        storey_m: 2.8,
        bay_m: 3.9,
        bays: 2,
        band: [0.258, 0.258, 0.205],
        band_h: 0.38,
        glass: [0.052, 0.058, 0.060],
        frame: [0.640, 0.638, 0.620],
        open_w: 0.92,
        open_h: 1.40,
        blind: 0.33,
        ..MASONRY
    },
    FacadeDesign {
        name: "plaster-taupe",
        // Weathered sand render with almost no chroma left: a 板楼 that has been
        // repainted twice and washed twice.  Close to the tile above in hue and
        // a fifth of a stop lighter, which is a different building.
        wall: [0.420, 0.390, 0.320],
        grain: 0.018,
        pier: [0.540, 0.525, 0.480],
        pier_w: 0.36,
        major_pier_w: 0.48,
        bay_m: 3.6,
        bays: 2,
        band: [0.290, 0.278, 0.250],
        band_h: 0.32,
        glass: [0.054, 0.058, 0.064],
        frame: [0.660, 0.652, 0.635],
        open_w: 0.90,
        open_h: 1.45,
        blind: 0.31,
        grille: 0.16,
        ..MASONRY
    },
    // -- masonry, pale: only two, and never brighter than 0.58 --------------
    FacadeDesign {
        name: "plaster-cream",
        // Cream render: the podium and 裙房 tile, and the brightest wall in the
        // city.  0.58, not 0.75 — a bright tower is a minority in a real
        // skyline, and a bright *city* is a blockout.  公建 storey height 3.0.
        wall: [0.580, 0.556, 0.495],
        grain: 0.014,
        pier: [0.660, 0.648, 0.618],
        pier_w: 0.36,
        major_pier_w: 0.48,
        storey_m: 3.0,
        bay_m: 3.6,
        bays: 2,
        band: [0.400, 0.392, 0.372],
        band_h: 0.32,
        glass: [0.056, 0.060, 0.068],
        frame: [0.730, 0.724, 0.706],
        open_w: 0.92,
        open_h: 1.50,
        blind: 0.22,
        grille: 0.05,
        ..MASONRY
    },
    FacadeDesign {
        name: "tile-dark-warm",
        // Dark warm glazed wall tile with pale joints and a pale sill band — the
        // 深色瓷砖 slab, and the darkest masonry wall in the city.  Its piers and
        // frames are the *only* light on the tile, which is exactly the look.
        // A 2000s tile-clad block, so it carries the 飘窗 band.
        cladding: Cladding::Mosaic,
        wall: [0.108, 0.098, 0.085],
        grain: 0.014,
        pier: [0.470, 0.462, 0.448],
        pier_w: 0.28,
        major_pier_w: 0.48,
        bay_m: 3.6,
        bays: 2,
        bay_band: true,
        band: [0.076, 0.072, 0.066],
        band_h: 0.34,
        glass: [0.044, 0.048, 0.052],
        frame: [0.600, 0.594, 0.580],
        open_w: 0.94,
        open_h: 1.45,
        blind: 0.30,
        grille: 0.16,
        module_m: 0.050,
        ..MASONRY
    },
    // -- curtain wall --------------------------------------------------------
    FacadeDesign {
        name: "curtain-teal-band",
        // Tinted aqua vision glass with a *pale horizontal band* at every
        // storey.  The band, not the glass, is what a viewer reads, which is
        // why a dark glass tower is given one.
        wall: [0.400, 0.415, 0.420],
        spandrel: [0.370, 0.386, 0.392],
        spandrel_h: 0.56,
        cap: [0.440, 0.452, 0.458],
        cap_h: 0.16,
        band: [0.024, 0.030, 0.034],
        band_h: 0.22,
        glass: [0.058, 0.084, 0.094],
        frame: [0.030, 0.042, 0.048],
        blind: 0.26,
        ..GLAZED
    },
    FacadeDesign {
        name: "curtain-pale-fins",
        // Wide pale stone fins over a dark grid.  This is the tower that reads as
        // a bundle of vertical stripes from a kilometre out, and it is the
        // commonest glass elevation in a Chinese CBD.
        wall: [0.470, 0.472, 0.462],
        pier: [0.470, 0.472, 0.462],
        pier_w: 0.26,
        spandrel: [0.128, 0.134, 0.140],
        spandrel_h: 1.00,
        cap: [0.430, 0.430, 0.420],
        cap_h: 0.22,
        band: [0.022, 0.024, 0.026],
        band_h: 0.22,
        glass: [0.058, 0.064, 0.072],
        frame: [0.030, 0.032, 0.036],
        blind: 0.32,
        ..GLAZED
    },
    FacadeDesign {
        name: "curtain-dark-grid",
        // The dark grid: smoke-blue glass, a dark spandrel and a thin pale
        // coping.  Almost all of this tile is dark, which is what makes it the
        // value anchor of a glass cluster — and, because its median lands on
        // glass rather than on stone, it is the tile `curtain-stone-band` has to
        // be told apart from.
        wall: [0.098, 0.106, 0.115],
        spandrel: [0.092, 0.100, 0.110],
        spandrel_h: 0.95,
        cap: [0.400, 0.402, 0.396],
        cap_h: 0.20,
        band: [0.018, 0.020, 0.022],
        band_h: 0.26,
        glass: [0.062, 0.076, 0.092],
        frame: [0.028, 0.030, 0.032],
        blind: 0.24,
        ..GLAZED
    },
    FacadeDesign {
        name: "curtain-bronze",
        // Bronze-tinted glass with a bronze spandrel: the warm office tower.
        wall: [0.215, 0.165, 0.115],
        spandrel: [0.205, 0.158, 0.112],
        spandrel_h: 1.05,
        cap: [0.400, 0.360, 0.300],
        cap_h: 0.22,
        band: [0.030, 0.022, 0.014],
        band_h: 0.22,
        glass: [0.080, 0.060, 0.038],
        frame: [0.044, 0.032, 0.020],
        blind: 0.30,
        ..GLAZED
    },
    FacadeDesign {
        name: "curtain-narrow-dark",
        // Narrow modules and a deep spandrel: the darkest tile in the set, and
        // the one that gives a cluster its black.  A 3.0 m grid split into four
        // 0.75 m leaves.
        bays: 4,
        wall: [0.092, 0.098, 0.104],
        pier: [0.400, 0.402, 0.396],
        pier_w: 0.10,
        spandrel: [0.082, 0.088, 0.094],
        spandrel_h: 1.20,
        cap: [0.360, 0.362, 0.356],
        cap_h: 0.18,
        band: [0.016, 0.017, 0.018],
        band_h: 0.24,
        glass: [0.042, 0.046, 0.052],
        frame: [0.022, 0.024, 0.026],
        blind: 0.18,
        ..GLAZED
    },
    FacadeDesign {
        name: "curtain-stone-band",
        // A pale warm stone spandrel under dark glass: the institutional
        // government-office tower, and the one glass tile that is *light*.  The
        // spandrel is deliberately deep and the fin wide, so the tile's median
        // lands on stone rather than on glass — otherwise this tile and
        // `curtain-dark-grid` are the same colour with the same glass, and no
        // amount of hue tuning will separate them.
        wall: [0.390, 0.378, 0.352],
        pier: [0.430, 0.420, 0.402],
        pier_w: 0.30,
        spandrel: [0.372, 0.362, 0.340],
        spandrel_h: 1.20,
        cap: [0.450, 0.442, 0.424],
        cap_h: 0.22,
        band: [0.024, 0.024, 0.023],
        band_h: 0.28,
        glass: [0.048, 0.052, 0.058],
        frame: [0.028, 0.030, 0.032],
        blind: 0.28,
        ..GLAZED
    },
    FacadeDesign {
        name: "curtain-seafoam",
        // Seafoam green tint with a mid green spandrel: the mint-green crown
        // that tops half the residential towers of the 2010s.
        wall: [0.165, 0.215, 0.192],
        spandrel: [0.158, 0.206, 0.184],
        spandrel_h: 0.90,
        cap: [0.390, 0.400, 0.390],
        cap_h: 0.24,
        band: [0.022, 0.030, 0.026],
        band_h: 0.22,
        glass: [0.068, 0.098, 0.086],
        frame: [0.034, 0.046, 0.042],
        blind: 0.30,
        ..GLAZED
    },
    FacadeDesign {
        name: "curtain-warm-fins",
        // Pale warm fins over warm grey glass: the other vertical-stripe tower,
        // and the warm half of a mixed cluster.
        bays: 4,
        wall: [0.480, 0.452, 0.400],
        pier: [0.480, 0.452, 0.400],
        pier_w: 0.24,
        spandrel: [0.150, 0.140, 0.124],
        spandrel_h: 0.85,
        cap: [0.420, 0.395, 0.355],
        cap_h: 0.22,
        band: [0.026, 0.024, 0.021],
        band_h: 0.22,
        glass: [0.074, 0.070, 0.064],
        frame: [0.038, 0.036, 0.032],
        blind: 0.34,
        ..GLAZED
    },
    FacadeDesign {
        name: "panel-concrete-pale",
        // Fair-faced concrete panel, cool and pale: the institutional slab and
        // the white residential tower that lines every Chinese ring road.
        cladding: Cladding::Panel,
        wall: [0.505, 0.512, 0.520],
        grain: 0.013,
        pier: [0.620, 0.628, 0.638],
        pier_w: 0.30,
        major_pier_w: 0.52,
        bay_m: 3.9,
        bays: 2,
        band: [0.345, 0.352, 0.360],
        band_h: 0.30,
        glass: [0.048, 0.052, 0.058],
        frame: [0.680, 0.685, 0.692],
        open_w: 0.95,
        open_h: 1.40,
        blind: 0.20,
        grille: 0.08,
        ..MASONRY
    },
    FacadeDesign {
        name: "tile-mosaic-cream",
        // 95 mm mosaic tile in a warm cream, with the accent-tile motif every
        // Chinese apartment block of the 2000s carries — and the 飘窗 band to
        // match, because this is *the* 2000s slab.
        cladding: Cladding::Mosaic,
        wall: [0.510, 0.478, 0.415],
        grain: 0.016,
        pier: [0.610, 0.590, 0.548],
        pier_w: 0.34,
        major_pier_w: 0.46,
        bay_m: 3.6,
        bays: 2,
        bay_band: true,
        band: [0.352, 0.340, 0.315],
        band_h: 0.36,
        glass: [0.052, 0.058, 0.066],
        frame: [0.700, 0.692, 0.672],
        open_w: 0.90,
        open_h: 1.45,
        blind: 0.30,
        grille: 0.06,
        motif: true,
        ..MASONRY
    },
    // -- masonry, grey and tile ---------------------------------------------
    FacadeDesign {
        name: "plaster-cool-grey",
        // Cool blue-grey render: a northern city, or a block next to a viaduct.
        wall: [0.280, 0.305, 0.335],
        grain: 0.018,
        pier: [0.390, 0.415, 0.442],
        pier_w: 0.32,
        major_pier_w: 0.50,
        bay_m: 3.6,
        bays: 2,
        band: [0.192, 0.208, 0.228],
        band_h: 0.36,
        glass: [0.048, 0.052, 0.060],
        frame: [0.520, 0.535, 0.552],
        lintel_h: 0.0,
        open_w: 0.90,
        open_h: 1.40,
        blind: 0.25,
        grille: 0.05,
        ..MASONRY
    },
    FacadeDesign {
        name: "panel-concrete-grey",
        // Green-grey cast concrete panel with a strong horizontal band: the
        // 1980s walk-up.  2.8 m 层高 on a 3.3 m 开间.
        cladding: Cladding::Panel,
        storey_m: 2.8,
        wall: [0.300, 0.325, 0.305],
        grain: 0.016,
        pier: [0.412, 0.438, 0.418],
        pier_w: 0.30,
        major_pier_w: 0.46,
        bay_m: 3.3,
        bays: 2,
        band: [0.205, 0.222, 0.212],
        band_h: 0.38,
        glass: [0.048, 0.054, 0.052],
        frame: [0.540, 0.556, 0.542],
        lintel_h: 0.0,
        open_w: 0.98,
        open_h: 1.40,
        blind: 0.24,
        grille: 0.10,
        ..MASONRY
    },
    FacadeDesign {
        name: "panel-concrete-mid",
        // Mid grey concrete, and a narrow window column: the value most of a
        // city is actually made of.  The small openings are stair and bathroom
        // windows (楼梯间/卫生间), which is why they sit below the habitable
        // 窗高 band.
        cladding: Cladding::Panel,
        storey_m: 2.8,
        wall: [0.215, 0.226, 0.230],
        grain: 0.015,
        pier: [0.330, 0.342, 0.348],
        pier_w: 0.20,
        major_pier_w: 0.32,
        bay_m: 3.3,
        bays: 3,
        band: [0.148, 0.156, 0.160],
        band_h: 0.38,
        glass: [0.044, 0.048, 0.052],
        frame: [0.440, 0.450, 0.458],
        lintel_h: 0.0,
        open_w: 0.52,
        open_h: 1.30,
        blind: 0.20,
        grille: 0.05,
        ..MASONRY
    },
    FacadeDesign {
        name: "tile-mosaic-grey",
        // The greenish-grey mosaic of the older stock.  A different *material*
        // from the cream above, at a value a fifth of a stop lower.
        cladding: Cladding::Mosaic,
        storey_m: 2.8,
        wall: [0.350, 0.368, 0.338],
        grain: 0.018,
        pier: [0.452, 0.468, 0.442],
        pier_w: 0.30,
        major_pier_w: 0.46,
        bay_m: 3.3,
        bays: 2,
        bay_band: true,
        band: [0.240, 0.252, 0.240],
        band_h: 0.38,
        glass: [0.048, 0.052, 0.052],
        frame: [0.580, 0.590, 0.578],
        lintel_h: 0.0,
        open_w: 0.98,
        open_h: 1.40,
        blind: 0.34,
        grille: 0.22,
        motif: true,
        ..MASONRY
    },
    // -- masonry, brick ------------------------------------------------------
    FacadeDesign {
        name: "brick-clay",
        // Warm clay brick in running bond, with a rendered pier and a limestone
        // sill: the six-storey walk-up of every provincial city.  2.8 m 层高.
        cladding: Cladding::Brick,
        storey_m: 2.8,
        wall: [0.205, 0.098, 0.062],
        grain: 0.030,
        pier: [0.470, 0.442, 0.396],
        pier_w: 0.64,
        bay_m: 3.3,
        band: [0.142, 0.078, 0.056],
        band_h: 0.38,
        glass: [0.044, 0.044, 0.048],
        frame: [0.610, 0.600, 0.575],
        open_w: 1.35,
        open_h: 1.35,
        blind: 0.40,
        grille: 0.30,
        ..MASONRY
    },
    FacadeDesign {
        name: "brick-maroon",
        // Dark maroon engineering brick.  Two bricks, a third of a stop apart in
        // value and a hue apart — which is the point of having both.
        cladding: Cladding::Brick,
        storey_m: 2.8,
        wall: [0.155, 0.055, 0.048],
        grain: 0.032,
        pier: [0.430, 0.398, 0.358],
        pier_w: 0.70,
        bay_m: 3.3,
        band: [0.108, 0.046, 0.040],
        band_h: 0.38,
        glass: [0.042, 0.042, 0.046],
        frame: [0.580, 0.570, 0.548],
        lintel_h: 0.0,
        open_w: 1.30,
        open_h: 1.35,
        blind: 0.44,
        grille: 0.36,
        ..MASONRY
    },
];

/// The design for tile `index`, panicking only on a table bug.
pub fn design(index: usize) -> &'static FacadeDesign {
    &DESIGNS[index.min(DESIGNS.len() - 1)]
}

/// The physical height of one tile of `design`: an exact whole number of
/// storeys, so tiling a wall of any storey count lands every floor line.
pub fn tile_height_m(design: &FacadeDesign) -> f32 {
    design.storey_m * STOREYS_PER_TILE as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::FACADE_TILES;

    fn luma(linear: [f32; 3]) -> f32 {
        0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
    }

    // -- the dimension rules (the table is held to the GB band) --------------

    /// 层高: residential stock sits in 2.8 – 3.0, 公建 and curtain wall at 3.2
    /// and above.  A design outside its family's band is a building nobody
    /// could get approved.
    #[test]
    fn storey_heights_sit_in_the_code_band() {
        for (index, design) in DESIGNS.iter().enumerate() {
            if design.cladding == Cladding::CurtainWall {
                assert!(
                    design.storey_m >= 3.2,
                    "{index} {}: an office storey of {} m is below the 公建 band",
                    design.name,
                    design.storey_m
                );
            } else {
                assert!(
                    (2.8..=3.0).contains(&design.storey_m),
                    "{index} {}: a residential storey of {} m is outside 2.8-3.0",
                    design.name,
                    design.storey_m
                );
            }
        }
    }

    /// 开间: a single-bay masonry tile is one of the three real structural
    /// bays; a multi-bay tile splits a real bay into halves or thirds; a
    /// curtain-wall module is a fabricator's 0.7 – 1.6 m leaf.
    #[test]
    fn bay_modules_are_structural_bays_or_their_subdivisions() {
        for (index, design) in DESIGNS.iter().enumerate() {
            let module = design.bay_m / design.bays.max(1) as f32;
            if design.cladding == Cladding::CurtainWall {
                assert!(
                    (0.70..=1.60).contains(&module),
                    "{index} {}: a {module:.2} m curtain module is not a leaf a fabricator builds",
                    design.name
                );
            } else if design.bays == 1 {
                assert!(
                    matches!(design.bay_m, 3.3 | 3.6 | 3.9),
                    "{index} {}: a {} m 开间 is not a structural bay (3.3/3.6/3.9)",
                    design.name,
                    design.bay_m
                );
            } else {
                let parts = design.bays.max(1) as f32;
                let base = design.bay_m / parts;
                // The sub-module must be an exact division of one of the three
                // bays, so the wall still reads as built on the 开间 grid.
                let on_grid = [3.3_f32, 3.6, 3.9]
                    .iter()
                    .any(|bay| ((bay / parts) - base).abs() < 1.0e-4);
                assert!(
                    on_grid && (1.0..=2.0).contains(&module),
                    "{index} {}: a {module:.2} m sub-module is not a division of a real bay",
                    design.name
                );
            }
            // A pier on every bay boundary, or the tile's own seam shows as a
            // hard vertical line down a wall.
            assert!(
                design.pier_w > 0.0 || design.major_pier_w > 0.0,
                "{index} {} has no pier, so the tile repeat will be visible",
                design.name
            );
        }
    }

    /// 窗台 0.9, exactly: the bake's per-opening jitter is construction
    /// tolerance on top of this, not an excuse for the design to miss it.
    #[test]
    fn every_sill_is_the_code_sill_height() {
        for (index, design) in DESIGNS.iter().enumerate() {
            if design.cladding == Cladding::CurtainWall {
                continue;
            }
            assert_eq!(
                design.sill_m, 0.90,
                "{index} {}: the sill is {} m; 住宅窗台 is 0.9",
                design.name, design.sill_m
            );
        }
    }

    /// 窗高 1.4 – 1.5 on habitable windows; the deliberately smaller stair and
    /// bathroom openings sit in their own band and say so by being narrow.
    #[test]
    fn opening_heights_are_habitable_or_admit_to_being_service() {
        for (index, design) in DESIGNS.iter().enumerate() {
            if design.cladding == Cladding::CurtainWall {
                continue;
            }
            if design.open_w >= 0.85 {
                assert!(
                    (1.35..=1.55).contains(&design.open_h),
                    "{index} {}: a habitable window of {:.2} x {:.2} m is outside 窗高 1.4-1.5",
                    design.name,
                    design.open_w,
                    design.open_h
                );
            } else {
                assert!(
                    (1.2..=1.35).contains(&design.open_h),
                    "{index} {}: a service opening of {:.2} m high is out of band",
                    design.name,
                    design.open_h
                );
            }
            assert!(
                (0.45..=1.65).contains(&design.open_w),
                "{index} {}: a {:.2} m wide window",
                design.name,
                design.open_w
            );
        }
    }

    /// The 飘窗 flag belongs to the 2000s tile-clad stock, and a bay band
    /// without a genuine tile cladding would be a western relief band.
    #[test]
    fn the_bay_window_band_belongs_to_tile_clad_stock() {
        for design in DESIGNS.iter() {
            if design.bay_band {
                assert_eq!(
                    design.cladding,
                    Cladding::Mosaic,
                    "{}: the 飘窗 band is a 2000s tile-clad feature",
                    design.name
                );
            }
        }
        assert!(
            DESIGNS.iter().filter(|d| d.bay_band).count() >= 3,
            "the 2000s stock is a real share of the city"
        );
    }

    /// A window is a window: it has a sill, a head, and it fits inside its
    /// storey with the inter-storey band to spare.
    #[test]
    fn every_opening_fits_inside_its_storey() {
        for (index, design) in DESIGNS.iter().enumerate() {
            if design.cladding == Cladding::CurtainWall {
                let glazed = design.storey_m - design.band_h - design.spandrel_h - design.cap_h;
                assert!(
                    (1.2..=2.4).contains(&glazed),
                    "{index} {}: a {glazed:.2} m vision panel in a {} m storey",
                    design.name,
                    design.storey_m
                );
                assert!(
                    design.spandrel_h > 0.4,
                    "{index} {}: a {:.0} mm spandrel hides the floor",
                    design.name,
                    design.spandrel_h * 1000.0
                );
                continue;
            }
            let head = design.sill_m + design.open_h;
            assert!(
                head < design.storey_m - design.band_h - 0.10,
                "{index} {}: the window head at {head:.2} m runs into the storey band",
                design.name
            );
            assert!(
                (0.50..=2.0).contains(&design.open_w),
                "{index} {}: a {:.2} m wide window",
                design.name,
                design.open_w
            );
        }
    }

    /// An opening has to fit *between* the piers.  An opening that runs under a
    /// pier is an opening cut by a column, and it is the most obvious way a
    /// procedural facade gives itself away.  A single-bay wall is also required
    /// to keep real 窗间墙: a 3.6 m bay with a 1.5 m window leaves 0.9 m of
    /// wall each side, and that wall is what makes it a bay rather than a ribbon.
    #[test]
    fn every_opening_fits_between_its_piers_with_wall_to_spare() {
        for (index, design) in DESIGNS.iter().enumerate() {
            if design.cladding == Cladding::CurtainWall {
                continue;
            }
            let bays = design.bays.max(1) as i32;
            let module = design.bay_m / bays as f32;
            let (_, _, clear) = bay_clearance(design, bays, 0, module);
            assert!(
                design.open_w <= clear + 1.0e-4,
                "{index} {}: a {:.2} m opening in a {:.2} m clear wall",
                design.name,
                design.open_w,
                clear
            );
            let (lo, hi) = if bays == 1 { (0.42, 0.80) } else { (0.45, 0.95) };
            let pane = design.open_w / clear;
            assert!(
                (lo..=hi).contains(&pane),
                "{index} {}: the opening is {pane:.2} of its clear wall (band {lo}-{hi})",
                design.name
            );
        }
    }

    // -- the albedo contract -------------------------------------------------

    /// The one test that would have caught the previous port.
    ///
    /// Every wall reflectance must sit in the range the *material* occupies, and
    /// glass must sit in the range glass occupies.  A curtain wall at 0.5 albedo
    /// is not a bright tower, it is a double-counted sky; a whole palette above
    /// 0.6 is a blockout.
    #[test]
    fn every_wall_reflectance_is_one_a_real_material_occupies() {
        for (index, design) in DESIGNS.iter().enumerate() {
            let wall = luma(design.wall);
            if design.cladding == Cladding::CurtainWall {
                // Glass is dark.  It is bright only because it reflects a very
                // bright sky, and the renderer does that with roughness and the
                // IBL, not with albedo.  Note what is *not* asserted here: the
                // tile's `wall` is its signature colour, which for a curtain wall
                // is the spandrel or the fin, and those are opaque panels allowed
                // to be pale.
                assert!(
                    (0.030..=0.16).contains(&luma(design.glass)),
                    "{index} {}: vision glass at {:.3}; a tinted pane reflects 4-16% \
                     and any more is a lit surface pretending to be glass",
                    design.name,
                    luma(design.glass)
                );
                // And the frame has to be darker than the glass it holds, or
                // the mullion grid disappears and the tower reads as a decal.
                assert!(
                    luma(design.frame) < luma(design.glass),
                    "{index} {}: frame {:.3} is not darker than glass {:.3}",
                    design.name,
                    luma(design.frame),
                    luma(design.glass)
                );
                // The spandrel is an opaque panel, so it lives in the range an
                // opaque panel occupies, and the pale coping at its head is the
                // horizontal graphic every glass tower has.  The coping has to be
                // a real step: a spandrel and a cap within a few per cent of each
                // other means the storey line stops reading, and the storey line
                // is the only thing that makes a tower legible at a kilometre.
                let spandrel = luma(design.spandrel);
                assert!(
                    (0.06..=0.55).contains(&spandrel),
                    "{index} {}: spandrel at {spandrel:.3}, which is not an opaque panel",
                    design.name
                );
                let cap = luma(design.cap);
                assert!(
                    (0.25..=0.60).contains(&cap),
                    "{index} {}: coping at {cap:.3}, which is no material's coping",
                    design.name
                );
                assert!(
                    cap > spandrel * 1.10,
                    "{index} {}: coping {cap:.3} is not a step above its spandrel {spandrel:.3}",
                    design.name
                );
                // And the coping is the *horizontal* graphic, so it has to be a
                // real step against the dark glass it caps.  This is the
                // assertion that keeps a glass tower from dissolving into a
                // silhouette: the vertical mullion grid is too fine to survive
                // a kilometre, and the pale band at every floor is not.
                assert!(
                    cap > luma(design.glass) * 2.5,
                    "{index} {}: coping {cap:.3} is not a graphic against its glass {:.3}",
                    design.name,
                    luma(design.glass)
                );
            } else {
                // Render, mosaic, brick, panel: nothing darker than a dark
                // engineering brick and nothing brighter than a fresh white coat.
                assert!(
                    (0.07..=0.75).contains(&wall),
                    "{index} {}: wall at {wall:.3}; masonry runs 0.07 (dark maroon \
                     engineering brick) to 0.75 (new white render)",
                    design.name
                );
                // Masonry glazing is *dark in absolute terms* — a window is a
                // hole in a lit wall — and darker than the wall it sits in.  Both
                // bounds matter: the first is what stops a bright "window", the
                // second is what stops a dark tile's window from looking lighter
                // than its own wall.
                assert!(
                    luma(design.glass) < 0.12,
                    "{index} {}: glass at {:.3}; a window reflects a few per cent and \
                     is always one of the darkest things on a rendered wall",
                    design.name,
                    luma(design.glass)
                );
                assert!(
                    luma(design.glass) < wall * 0.75,
                    "{index} {}: glass {:.3} against wall {wall:.3}",
                    design.name,
                    luma(design.glass)
                );
            }
            // Trim, piers, frames and copings are all light enough to catch a
            // highlight — except a curtain wall's mullion, which is the darkest
            // thing on the tile and has to be, or the grid stops reading.
            for (what, value) in [("pier", design.pier), ("frame", design.frame)] {
                let v = luma(value);
                let range = if design.cladding == Cladding::CurtainWall && what == "frame" {
                    0.020..=0.12
                } else {
                    0.25..=0.80
                };
                assert!(
                    range.contains(&v),
                    "{index} {}: {what} at {v:.3}, outside {range:?}",
                    design.name
                );
            }
        }
    }

    /// The palette is not 24 tints of one wall, and the way to prove that is to
    /// name the families and count them.
    #[test]
    fn the_palette_spans_the_families_a_chinese_skyline_is_made_of() {
        let masonry: Vec<_> = DESIGNS.iter().filter(|d| d.cladding != Cladding::CurtainWall).collect();
        let glazed = DESIGNS.len() - masonry.len();
        assert_eq!(glazed, 8, "eight curtain-wall tiles is the mix a skyline needs");
        // Warm beige leads, the way it does in every photograph of a Chinese
        // residential district.  The point of this assertion is that a future
        // edit cannot quietly rebalance the city towards grey.
        let warm = masonry
            .iter()
            .filter(|d| d.wall[0] - d.wall[2] > 0.09 && d.wall[0] > 0.30)
            .count();
        assert!(warm >= 4, "only {warm} warm beige masonry tiles");
        let cool_grey = masonry.iter().filter(|d| d.wall[2] >= d.wall[0]).count();
        assert!(cool_grey >= 2, "only {cool_grey} grey masonry tiles");
        // Tile-clad, brick and panel are different materials, not tints.
        for cladding in [Cladding::Mosaic, Cladding::Brick, Cladding::Panel] {
            assert!(
                masonry.iter().any(|d| d.cladding == cladding),
                "no {cladding:?} tile"
            );
        }
        // Nothing may be white, and nothing may be black.  A city that is too
        // white is the most recognisable signature of a fake render.
        let mut values: Vec<f32> = DESIGNS
            .iter()
            .map(|d| luma(if d.cladding == Cladding::CurtainWall { d.glass } else { d.wall }))
            .collect();
        values.sort_by(|a, b| a.total_cmp(b));
        assert!(
            values[0] >= 0.035,
            "the darkest wall is at {:.3}, which is a hole rather than a material",
            values[0]
        );
        assert!(
            values[values.len() - 1] <= 0.74,
            "the brightest wall is at {:.3}; a city of 0.75 walls is a blockout",
            values[values.len() - 1]
        );
    }

    /// Storey alignment.  A tile's height has to be an exact whole number of
    /// storeys *of its own design*, and a wall of any height has to land its
    /// floor lines on floors.
    #[test]
    fn the_tile_is_an_exact_whole_number_of_storeys() {
        for (index, design) in DESIGNS.iter().enumerate() {
            let tile_h = tile_height_m(design);
            let storeys = tile_h / design.storey_m;
            assert!(
                (storeys - storeys.round()).abs() < 1.0e-6,
                "{index} {}: the tile is {storeys} storeys; a fractional storey drifts",
                design.name
            );
            assert_eq!(storeys.round() as u32, STOREYS_PER_TILE);
            // The tile's own floor lines are on its edge, so tiling never
            // doubles or splits a band.
            assert!((tile_h % design.storey_m).abs() < 1.0e-6);
        }
        // The standard constants the rest of the crate quotes are the standard
        // residential module: 层高 2.9, four storeys to the tile.
        assert!((STOREY_M - 2.9).abs() < 1.0e-6);
        assert!((FACADE_TILE_H - STOREY_M * 4.0).abs() < 1.0e-6);
        // The ground storey is its own module, because a 4.5 m shopfront cannot
        // be squeezed into a residential storey.
        assert_eq!(GROUND_FLOOR_TILE_H, GROUND_STOREY_M);
        // A wall of any whole number of storeys above the ground floor ends
        // exactly on a storey line, for every count there is, in every family.
        for design in DESIGNS.iter() {
            for storeys in 1..=60_u32 {
                let above = storeys as f32 * design.storey_m;
                let exact = above / design.storey_m;
                assert!(
                    (exact - exact.round()).abs() < 1.0e-4,
                    "{}: {storeys} storeys drifts by {}",
                    design.name,
                    exact - exact.round()
                );
            }
        }
    }

    /// The spec table owns which eight tiles are curtain wall, because the
    /// renderer's roughness and metalness switch on that index.  If the two ever
    /// disagree, a masonry tile gets a mirror finish.
    #[test]
    fn the_glazing_flags_agree_with_the_spec_table() {
        for (index, design) in DESIGNS.iter().enumerate() {
            assert_eq!(
                design.cladding == Cladding::CurtainWall,
                FACADE_TILES[index].glass,
                "{index} {} disagrees with spec::FACADE_TILES on being glazed",
                design.name
            );
        }
    }
}
