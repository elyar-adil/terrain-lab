//! Building elevations: the facade tiles, the ground-floor variants and the roof.
//!
//! This module is the visual quality of a city, and it is the one the previous
//! port got most wrong. What it produced was a flat colour block per wall: the
//! textures were decoded from the payload and then never bound to a material, so
//! every building was lit by one pale diffuse colour. From a kilometre away that
//! reads as a white monolith, which is the artefact this module exists to
//! remove.
//!
//! # Albedo is reflectance, and reflectance is not a style knob
//!
//! Every colour in [`DESIGNS`] is a **linear reflectance**, not a screen colour.
//! That distinction is the whole difference between a render that looks
//! photographed and one that looks like a blockout, and it is easy to lose:
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
//! sky and turns a tower into a white slab. Conversely a whole city of 0.70
//! albedo walls is the single most recognisable signature of a fake render: real
//! cities are mostly mid-value, and the bright surfaces are a minority.
//!
//! # What a facade tile has to survive
//!
//! A tile is seen at three scales at once, and it has to work at all three:
//!
//! * **At a kilometre**, as one pixel per storey or less. What carries the
//!   building is the *value rhythm*: a dark inter-storey band, glazing darker
//!   than the wall, a pale pier at every structural bay. A tile whose darkest
//!   and brightest regions are close together dissolves into a silhouette.
//! * **At fifty metres**, as a wall. What carries it is that the window grid is
//!   *aligned to the geometry* — a window never straddles a corner, a floor
//!   never lands half a storey high, and the bay rhythm is a real module rather
//!   than a noise field. That is why [`STOREY_M`] is a single constant and why
//!   every wall is UV'd from the same datum: see `buildings::facade_wall`.
//! * **At two metres**, as a wall you could touch. What carries it is material:
//!   a reveal that shades, a frame, a sill that catches light, rain-washing
//!   under a window, a blind down in one opening and not the next. A facade
//!   where every window is identical reads as a texture, not a building.
//!
//! A tile is 3 m wide by 12.8 m tall — one structural bay of window columns by
//! four [`STOREY_M`] storeys. That is deliberate: the vertical size is an exact
//! multiple of the storey height, so tiling a wall of any height lands every
//! floor line exactly on a floor, with no drift and no half-storey at the
//! parapet. Every design puts a pier on every bay boundary, which means the
//! tile's own vertical seam always lands inside a pier and the 3 m repeat is
//! invisible; and a few designs put a *wider* pier on the tile boundary, so the
//! structural bay is six metres and the repeat stops reading as wallpaper.

use crate::textures::{BakedTexture, hash};

/// One storey of the tile module.  Every wall in the city is a whole number of
/// these, which is what keeps a floor line on a floor.
pub const STOREY_M: f32 = 3.2;
/// Storeys per facade tile.
pub const STOREYS_PER_TILE: u32 = 4;
/// Facade tiles cover 3 m of wall by four storeys of [`STOREY_M`].
pub const FACADE_TILE_W: f32 = 3.0;
pub const FACADE_TILE_H: f32 = STOREY_M * STOREYS_PER_TILE as f32;
/// A ground storey.  Taller than the floors above it, which is the single fact
/// that makes a facade read as a building rather than a stack of stripes — and
/// true of Chinese ground floors, which have to carry a 4.5 m shopfront.
pub const GROUND_STOREY_M: f32 = 4.5;
/// Ground-floor tiles are authored at true scale — one shop bay by one ground
/// storey, and no vertical repeat, so a shopfront's door and stall riser are the
/// right size in metres.
pub const GROUND_FLOOR_TILE_W: f32 = 4.2;
pub const GROUND_FLOOR_TILE_H: f32 = GROUND_STOREY_M;

// ---------------------------------------------------------------------------
// the design table
// ---------------------------------------------------------------------------

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
/// window is 1.15 m wide and its sill is 0.9 m off the floor because those are
/// the numbers a window is.
#[derive(Debug, Clone, Copy)]
pub struct FacadeDesign {
    /// What this tile is, for a human reading the table.
    pub name: &'static str,
    pub cladding: Cladding,
    /// Window columns across the 3 m tile.  A pier sits on every column
    /// boundary, so the tile's own seam is always buried in one.
    pub bays: u8,
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
    /// where every bay is the same.  This is the difference between a 3 m
    /// repeat and a 6 m structural bay.
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
    /// Clear opening: width, height, sill height above the floor, and how deep
    /// the reveal is.
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
/// departing from this, so the table is written as departures.
const MASONRY: FacadeDesign = FacadeDesign {
    name: "masonry",
    cladding: Cladding::Render,
    // One window column per 3 m: the 板楼 structural bay.  A 1.5 m module with a
    // 1.2 m window leaves 0.3 m of wall, and 0.3 m of wall is why the previous
    // port's tiles read as black bars.
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
    open_w: 1.55,
    open_h: 1.50,
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
const GLAZED: FacadeDesign = FacadeDesign {
    name: "glazed",
    cladding: Cladding::CurtainWall,
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
        // concrete pier on every three-metre structural bay, and a single
        // punched window per bay.  Most of a Chinese residential district is
        // made of this building.
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
        band: [0.270, 0.240, 0.185],
        band_h: 0.50,
        open_w: 1.50,
        open_h: 1.45,
        sill_m: 0.88,
        blind: 0.28,
        ..MASONRY
    },
    FacadeDesign {
        name: "ban-plaster-ochre",
        // The same building forty years weathers darker and browner.  Not a
        // tint of the tile above: a third of a stop darker with the hue pushed
        // towards olive, which is what water staining does to a render.
        wall: [0.300, 0.240, 0.135],
        grain: 0.026,
        pier: [0.470, 0.455, 0.420],
        pier_w: 0.66,
        band: [0.205, 0.172, 0.115],
        band_h: 0.40,
        glass: [0.048, 0.052, 0.058],
        frame: [0.600, 0.592, 0.570],
        open_w: 1.40,
        open_h: 1.35,
        sill_m: 0.85,
        blind: 0.46,
        grille: 0.24,
        ..MASONRY
    },
    FacadeDesign {
        name: "plaster-terracotta",
        // Terracotta render: the other end of the warm family, and the colour of
        // a provincial walk-up.
        wall: [0.360, 0.175, 0.095],
        grain: 0.026,
        pier: [0.520, 0.500, 0.460],
        pier_w: 0.60,
        band: [0.245, 0.140, 0.090],
        band_h: 0.42,
        glass: [0.048, 0.050, 0.056],
        frame: [0.620, 0.610, 0.585],
        open_w: 1.45,
        open_h: 1.40,
        sill_m: 0.88,
        blind: 0.35,
        grille: 0.18,
        ..MASONRY
    },
    FacadeDesign {
        name: "plaster-khaki",
        // Olive-khaki render — the greenish cast a cement mix goes when it is
        // made with sand rather than lime.  Present on half the stock in a
        // northern city and almost absent in a southern one.
        wall: [0.375, 0.375, 0.270],
        grain: 0.022,
        pier: [0.505, 0.505, 0.440],
        pier_w: 0.34,
        major_pier_w: 0.44,
        bays: 2,
        band: [0.258, 0.258, 0.205],
        band_h: 0.38,
        glass: [0.052, 0.058, 0.060],
        frame: [0.640, 0.638, 0.620],
        open_w: 0.92,
        open_h: 1.45,
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
        bays: 2,
        band: [0.290, 0.278, 0.250],
        band_h: 0.32,
        glass: [0.054, 0.058, 0.064],
        frame: [0.660, 0.652, 0.635],
        open_w: 0.90,
        open_h: 1.40,
        blind: 0.31,
        grille: 0.16,
        ..MASONRY
    },
    // -- masonry, pale: only two, and never brighter than 0.58 --------------
    FacadeDesign {
        name: "plaster-cream",
        // Cream render: the podium and 裙房 tile, and the brightest wall in the
        // city.  0.58, not 0.75 — a bright tower is a minority in a real
        // skyline, and a bright *city* is a blockout.
        wall: [0.580, 0.556, 0.495],
        grain: 0.014,
        pier: [0.660, 0.648, 0.618],
        pier_w: 0.36,
        major_pier_w: 0.48,
        bays: 2,
        band: [0.400, 0.392, 0.372],
        band_h: 0.32,
        glass: [0.056, 0.060, 0.068],
        frame: [0.730, 0.724, 0.706],
        open_w: 0.92,
        open_h: 1.55,
        blind: 0.22,
        grille: 0.05,
        ..MASONRY
    },
    FacadeDesign {
        name: "tile-dark-warm",
        // Dark warm glazed wall tile with pale joints and a pale sill band — the
        // 深色瓷砖 slab, and the darkest masonry wall in the city.  Its piers and
        // frames are the *only* light on the tile, which is exactly the look.
        cladding: Cladding::Mosaic,
        wall: [0.108, 0.098, 0.085],
        grain: 0.014,
        pier: [0.470, 0.462, 0.448],
        pier_w: 0.28,
        major_pier_w: 0.48,
        bays: 2,
        band: [0.076, 0.072, 0.066],
        band_h: 0.34,
        glass: [0.044, 0.048, 0.052],
        frame: [0.600, 0.594, 0.580],
        open_w: 0.94,
        open_h: 1.40,
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
        spandrel: [0.110, 0.116, 0.122],
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
        // value anchor of a glass cluster.
        wall: [0.115, 0.122, 0.132],
        spandrel: [0.108, 0.115, 0.124],
        spandrel_h: 1.05,
        cap: [0.400, 0.402, 0.396],
        cap_h: 0.20,
        band: [0.020, 0.022, 0.024],
        band_h: 0.26,
        glass: [0.052, 0.060, 0.070],
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
        // the one that gives a cluster its black.
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
        // government-office tower, and the one glass tile that is *light*.
        wall: [0.390, 0.378, 0.352],
        pier: [0.420, 0.412, 0.396],
        pier_w: 0.20,
        spandrel: [0.372, 0.362, 0.340],
        spandrel_h: 0.95,
        cap: [0.450, 0.442, 0.424],
        cap_h: 0.20,
        band: [0.024, 0.024, 0.023],
        band_h: 0.28,
        glass: [0.054, 0.062, 0.072],
        frame: [0.030, 0.032, 0.034],
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
        bays: 2,
        band: [0.345, 0.352, 0.360],
        band_h: 0.30,
        glass: [0.048, 0.052, 0.058],
        frame: [0.680, 0.685, 0.692],
        open_w: 0.95,
        open_h: 1.35,
        sill_m: 0.95,
        blind: 0.20,
        grille: 0.08,
        ..MASONRY
    },
    FacadeDesign {
        name: "tile-mosaic-cream",
        // 95 mm mosaic tile in a warm cream, with the accent-tile motif every
        // Chinese apartment block of the 2000s carries.
        cladding: Cladding::Mosaic,
        wall: [0.510, 0.478, 0.415],
        grain: 0.016,
        pier: [0.610, 0.590, 0.548],
        pier_w: 0.34,
        major_pier_w: 0.46,
        bays: 2,
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
        bays: 2,
        band: [0.192, 0.208, 0.228],
        band_h: 0.36,
        glass: [0.048, 0.052, 0.060],
        frame: [0.520, 0.535, 0.552],
        lintel_h: 0.0,
        open_w: 0.98,
        open_h: 1.40,
        blind: 0.25,
        grille: 0.05,
        ..MASONRY
    },
    FacadeDesign {
        name: "panel-concrete-grey",
        // Green-grey cast concrete panel with a strong horizontal band: the
        // 1980s walk-up.
        cladding: Cladding::Panel,
        wall: [0.300, 0.325, 0.305],
        grain: 0.016,
        pier: [0.412, 0.438, 0.418],
        pier_w: 0.30,
        major_pier_w: 0.46,
        bays: 2,
        band: [0.205, 0.222, 0.212],
        band_h: 0.40,
        glass: [0.048, 0.054, 0.052],
        frame: [0.540, 0.556, 0.542],
        lintel_h: 0.0,
        open_w: 0.98,
        open_h: 1.38,
        blind: 0.24,
        grille: 0.10,
        ..MASONRY
    },
    FacadeDesign {
        name: "panel-concrete-mid",
        // Mid grey concrete, and a narrow window column: the value most of a
        // city is actually made of.
        cladding: Cladding::Panel,
        wall: [0.215, 0.226, 0.230],
        grain: 0.015,
        pier: [0.330, 0.342, 0.348],
        pier_w: 0.20,
        major_pier_w: 0.32,
        bays: 3,
        band: [0.148, 0.156, 0.160],
        band_h: 0.40,
        glass: [0.044, 0.048, 0.052],
        frame: [0.440, 0.450, 0.458],
        lintel_h: 0.0,
        open_w: 0.52,
        open_h: 1.20,
        sill_m: 0.95,
        blind: 0.20,
        grille: 0.05,
        ..MASONRY
    },
    FacadeDesign {
        name: "tile-mosaic-grey",
        // The greenish-grey mosaic of the older stock.  A different *material*
        // from the cream above, at a value a fifth of a stop lower.
        cladding: Cladding::Mosaic,
        wall: [0.350, 0.368, 0.338],
        grain: 0.018,
        pier: [0.452, 0.468, 0.442],
        pier_w: 0.30,
        major_pier_w: 0.46,
        bays: 2,
        band: [0.240, 0.252, 0.240],
        band_h: 0.38,
        glass: [0.048, 0.052, 0.052],
        frame: [0.580, 0.590, 0.578],
        lintel_h: 0.0,
        open_w: 0.98,
        open_h: 1.38,
        blind: 0.34,
        grille: 0.22,
        motif: true,
        ..MASONRY
    },
    // -- masonry, brick ------------------------------------------------------
    FacadeDesign {
        name: "brick-clay",
        // Warm clay brick in running bond, with a rendered pier and a limestone
        // sill: the six-storey walk-up of every provincial city.
        cladding: Cladding::Brick,
        wall: [0.205, 0.098, 0.062],
        grain: 0.030,
        pier: [0.470, 0.442, 0.396],
        pier_w: 0.64,
        band: [0.142, 0.078, 0.056],
        band_h: 0.40,
        glass: [0.044, 0.044, 0.048],
        frame: [0.610, 0.600, 0.575],
        open_w: 1.35,
        open_h: 1.30,
        sill_m: 0.88,
        blind: 0.40,
        grille: 0.30,
        ..MASONRY
    },
    FacadeDesign {
        name: "brick-maroon",
        // Dark maroon engineering brick.  Two bricks, a third of a stop apart in
        // value and a hue apart — which is the point of having both.
        cladding: Cladding::Brick,
        wall: [0.155, 0.055, 0.048],
        grain: 0.032,
        pier: [0.430, 0.398, 0.358],
        pier_w: 0.70,
        band: [0.108, 0.046, 0.040],
        band_h: 0.42,
        glass: [0.042, 0.042, 0.046],
        frame: [0.580, 0.570, 0.548],
        lintel_h: 0.0,
        open_w: 1.30,
        open_h: 1.28,
        sill_m: 0.88,
        blind: 0.44,
        grille: 0.36,
        ..MASONRY
    },
];

/// The design for tile `index`, panicking only on a table bug.
pub fn design(index: usize) -> &'static FacadeDesign {
    &DESIGNS[index.min(DESIGNS.len() - 1)]
}

// ---------------------------------------------------------------------------
// the bake
// ---------------------------------------------------------------------------

/// The 24 facade tiles, one texture each so a renderer can bind one and never
/// branch.  The material key is `facade/NN` and it is the only contract between
/// this module and the renderer.
pub fn facade_textures(size: usize) -> Vec<BakedTexture> {
    (0..DESIGNS.len()).map(|index| facade_tile(index, size)).collect()
}

/// Encode a **linear** reflectance as an sRGB byte, which is what the renderer
/// decodes back through an sRGB texture.  Getting this wrong — writing a linear
/// value straight into an sRGB texture — darkens every wall by a factor of
/// about 2.2 at the top of the range and is the other half of "why is my city
/// black".
fn srgb8(linear: f32) -> u8 {
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
fn value_noise(seed: u32, x: f32, y: f32) -> f32 {
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
fn shade(colour: [f32; 3], factor: f32) -> [f32; 3] {
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
/// stops a 3 m tile from reading as wallpaper — a single pier width repeated
/// three times a tile is a stripe, and a stripe is what the previous port had.
fn pier_half_at(design: &FacadeDesign, bays: i32, k: i32, bay_w: f32) -> f32 {
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
fn bay_clearance(design: &FacadeDesign, bays: i32, bay: i32, bay_w: f32) -> (f32, f32, f32) {
    let left = pier_half_at(design, bays, bay, bay_w);
    // The boundary at the far side of the last bay is the *next tile's* first
    // boundary, so the pattern stays exactly periodic over the 3 m repeat.
    let right_k = if (bay + 1) % bays == 0 { 0 } else { bay + 1 };
    let right = pier_half_at(design, bays, right_k, bay_w);
    let clear = (bay_w - left - right - 0.12).max(0.34);
    (left, right, clear)
}

fn opening_for(design: &FacadeDesign, key: u32, bay: i32, storey: i32, left: f32, clear: f32) -> Opening {
    let a = hash(key.wrapping_add(11), bay, storey);
    let b = hash(key.wrapping_add(29), bay, storey);
    let c = hash(key.wrapping_add(47), bay, storey);
    let d = hash(key.wrapping_add(71), bay, storey);
    let e = hash(key.wrapping_add(97), bay, storey);
    let w = design.open_w.min(clear);
    // Centred in the clear wall, with a small per-opening shift.  The shift is
    // the *only* thing that varies horizontally, and it is kept well inside the
    // clear zone so a window never touches a pier.
    let slack = (clear - 0.12 - w).max(0.0);
    let cx = left + 0.06 + w * 0.5 + (a - 0.5) * slack * 0.6;
    let y0 = design.sill_m + (b - 0.5) * 0.10;
    Opening {
        cx,
        w,
        y0,
        y1: y0 + design.open_h,
        blind: c < design.blind,
        tone: 0.70 + d * 0.70,
        curtain: if e < 0.34 { 0.35 + hash(key, storey, bay) * 0.55 } else { 0.0 },
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
    let bays = design.bays.max(1) as f32;
    let bay_w = FACADE_TILE_W / bays;
    // Pixels per metre, so a joint or a bar is a fixed physical width instead of
    // a fixed number of texels — a fixed pixel width swallows a 45 mm tile whole
    // at low bake sizes and vanishes at high ones.
    let px_per_m = n as f32 / FACADE_TILE_W;
    let hairline = 1.0 / px_per_m;

    for y in 0..n {
        for x in 0..n {
            // `um` runs along the wall in metres, `m` up it in metres.
            let um = (x as f32 + 0.5) / px_per_m;
            let m = (1.0 - (y as f32 + 0.5) / n as f32) * FACADE_TILE_H;
            let storey = ((m / STOREY_M).floor() as i32).clamp(0, STOREYS_PER_TILE as i32 - 1);
            let ml = m - storey as f32 * STOREY_M;
            let bays_i = design.bays.max(1) as i32;
            let bay = ((um / bay_w).floor() as i32).clamp(0, bays_i - 1);
            let bx = um - bay as f32 * bay_w;
            let rgb = if design.cladding == Cladding::CurtainWall {
                curtain_sample(design, key, um, ml, bay, bx, bay_w, storey, (x as i32, y as i32))
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
        tile_width_m: FACADE_TILE_W,
        tile_height_m: FACADE_TILE_H,
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
    let opening = opening_for(design, key, bay, storey, left, clear);
    let frame_w = 0.055_f32.min((clear - opening.w) * 0.35).max(0.02);
    let x0 = opening.cx - opening.w * 0.5;
    let x1 = opening.cx + opening.w * 0.5;

    // --- 1. the opening, from the inside out -------------------------------
    let in_frame = bx > x0 - frame_w && bx < x1 + frame_w && ml > opening.y0 - frame_w && ml < opening.y1 + frame_w;
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
                    return [
                        linen * 1.03,
                        linen,
                        linen * 0.94,
                    ];
                }
            }
            // A faint interior falloff, the horizontal banding a reflection off a
            // neighbouring building leaves in a pane, and a cross-pane gradient:
            // a sash window is two lights, and the light nearer the room's
            // ceiling shows more of the ceiling.
            let interior = 0.86 + 0.22 * (1.0 - v);
            let band = if ((v * 6.0).fract() - 0.5).abs() < 0.10 { 1.10 } else { 1.0 };
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
        return shade(design.pier, 0.78 + 0.22 * edge + if wide { 0.04 } else { 0.0 });
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
        let t = (ml - (opening.y0 - 0.15 - 0.85 * opening.wash)) / (0.85 * opening.wash).max(1.0e-3);
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
            let stagger = if course.rem_euclid(2) == 0 { 0.0 } else { 0.120 };
            let bx = um + stagger;
            let along = bx / 0.240;
            let face = along - along.floor();
            let up = (m / 0.072) - (m / 0.072).floor();
            // 10 mm of joint on a 65 mm course is 15% of the face, not 25%:
            // a brick wall whose mortar is a quarter of its area reads as a
            // grid of pale lines, not as brick.
            let joint = face < 0.045 || face > 0.975 || up < 0.13 || up > 0.93;
            if joint {
                let mortar = 0.330 + 0.045 * hash(key.wrapping_add(401), along.floor() as i32, course);
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
                    let grout = 0.40 + 0.06 * hash(key.wrapping_add(433), tx.floor() as i32, ty.floor() as i32);
                    return [grout, grout * 0.99, grout * 0.96];
                }
                // Fired tile, so each one is its own colour.
                let fire = (hash(key.wrapping_add(449), tx.floor() as i32, ty.floor() as i32) - 0.5) * 0.040;
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
                    let checker = ((um / 0.047).floor() as i32 + (m / 0.047).floor() as i32) % 2 == 0;
                    let motif = if checker { [0.075, 0.105, 0.115] } else { [0.150, 0.190, 0.195] };
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
            [
                colour[0] + trowel,
                colour[1] + trowel,
                colour[2] + trowel,
            ]
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
    let glass_top = STOREY_M - mullion_w;
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
    let sheen = if ((um * 2.2) - (um * 2.2).floor()) < 0.14 { 1.06 } else { 1.0 };
    let _ = px;
    [
        glass[0] * depth * sheen,
        glass[1] * depth * sheen,
        glass[2] * depth * sheen,
    ]
}

// ---------------------------------------------------------------------------
// ground floors
// ---------------------------------------------------------------------------

/// The three ground-floor variants: `ground/shop`, `ground/lobby`,
/// `ground/home`.  Each is one shop bay by one ground storey at true scale, so
/// a door is 2.1 m tall on the wall rather than a stretched repeat.
pub fn ground_floor_textures(size: usize) -> Vec<BakedTexture> {
    ["shop", "lobby", "home"]
        .iter()
        .map(|kind| ground_floor(kind, size))
        .collect()
}

/// Chinese ground floors are a *continuous* band, not a row of doors: a stall
/// riser, a run of piers, glazed shopfronts, a roller shutter somewhere in
/// every four bays, a fascia, and the projecting sign boxes the geometry adds in
/// front of it.  That rhythm — not the door — is what makes a street read as
/// retail.
fn ground_floor(kind: &str, size: usize) -> BakedTexture {
    let n = size.max(16);
    let mut rgba = vec![0_u8; n * n * 4];
    let key = match kind {
        "shop" => 977_u32,
        "lobby" => 1481,
        _ => 2003,
    };
    let px_per_m = n as f32 / GROUND_FLOOR_TILE_W;
    let px_per_m = n as f32 / GROUND_FLOOR_TILE_W;
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
        for x in 0..n {
            let u = (x as f32 + 0.5) / px_per_m;
            // `h` is metres above the shop floor; the tile is one storey tall.
            let h = (1.0 - (y as f32 + 0.5) / n as f32) * GROUND_FLOOR_TILE_H;
            // Per-pixel grain only.  Anything that *decides* something — whether
            // a shop is lit, whether a shutter is down — is keyed on a 1.05 m
            // block, because a decision that changes every pixel is not a
            // decision, it is noise, and noise at this contrast reads as dirt on
            // the lens.
            let block = (u / 1.05).floor() as i32;
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

            let mut colour = match kind {
                "shop" => {
                    if h < 0.42 {
                        // Stall riser: a dark stone kick plate, grooved, scuffed
                        // by a thousand trolleys.
                        let groove = (u / 0.42) - (u / 0.42).floor() < 0.06;
                        shade(RISER, if groove { 0.72 } else { 1.0 })
                    } else if h > 3.55 {
                        // Fascia: the sign band, with a soffit shadow under the
                        // canopy lip and a pale top rail.  The geometry adds the
                        // projecting sign boxes in front of this.
                        if h > 4.32 {
                            FASCIA_TOP
                        } else if h > 4.20 {
                            [0.055, 0.058, 0.060]
                        } else if h < 3.68 {
                            shade(FASCIA, 0.60)
                        } else {
                            FASCIA
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
            let offset = (y * n + x) * 4;
            for channel in 0..3 {
                let value = colour[channel] - grime * (1.0 - h / GROUND_FLOOR_TILE_H).min(1.0);
                rgba[offset + channel] = srgb8(value);
            }
            rgba[offset + 3] = 255;
        }
    }
    BakedTexture {
        name: format!("ground/{kind}"),
        width: n,
        height: n,
        tile_width_m: GROUND_FLOOR_TILE_W,
        tile_height_m: GROUND_FLOOR_TILE_H,
        has_normal_source: false,
        rgba,
    }
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
    let px_per_m = n as f32 / ROOF_TILE_M;
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
            let pebble = if hash(43, x as i32 / 2, y as i32 / 2) < 0.10 { 0.030 } else { 0.0 };
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
        tile_width_m: ROOF_TILE_M,
        tile_height_m: ROOF_TILE_M,
        has_normal_source: true,
        rgba,
    }
}

/// One roof tile, in metres.  A roof is seen from above at every scale, so it
/// tiles at a size that puts about four membrane sheets in a tile — fine enough
/// to read as a surface, coarse enough not to alias at distance.
pub const ROOF_TILE_M: f32 = 3.0;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::FACADE_TILES;


    fn luma(linear: [f32; 3]) -> f32 {
        0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
    }

    /// The **median linear albedo** of a baked tile, decoded back out of sRGB.
    /// This is what a viewer reads a building as: one number per material, taken
    /// from the pixels a renderer will actually sample.
    fn median_albedo(texture: &BakedTexture) -> [f32; 3] {
        let n = texture.width;
        let mut values: Vec<[f32; 3]> = (0..n * n)
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

    /// Scale-invariant colour space: chromaticity plus where a tile falls on the
    /// palette's own light-to-dark axis.
    ///
    /// The chromaticity axes are the ones `species.rs` uses.  Its fourth axis,
    /// luma over peak, is deliberately *not* reused: for near-neutral building
    /// materials it is 1.0 for every tile, so it cannot tell a 0.29 grey slab
    /// from a 0.60 grey slab — which in a skyline is the largest difference
    /// there is.  A city reads as a set of values far more than as a set of
    /// hues, so the value axis is normalised against the palette's own spread.
    fn separable(texture: &BakedTexture, lo: f32, hi: f32) -> [f32; 4] {
        let colour = median_albedo(texture);
        let total = (colour[0] + colour[1] + colour[2]).max(1.0e-4);
        [
            colour[0] / total,
            colour[1] / total,
            colour[2] / total,
            ((luma(colour) - lo) / (hi - lo).max(1.0e-4)).clamp(0.0, 1.0),
        ]
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
                // opaque panel occupies — and it has to be either clearly darker
                // or clearly lighter than the glass, never within a few per cent,
                // or the storey line stops reading.
                let spandrel = luma(design.spandrel);
                assert!(
                    (0.06..=0.55).contains(&spandrel),
                    "{index} {}: spandrel at {spandrel:.3}, which is not an opaque panel",
                    design.name
                );
                assert!(
                    spandrel < luma(design.glass) * 0.80 || spandrel > luma(design.glass) * 1.9,
                    "{index} {}: spandrel {spandrel:.3} is too close to its glass {:.3}",
                    design.name,
                    luma(design.glass)
                );
            } else {
                // Render, mosaic, brick, panel: nothing darker than dark tile and
                // nothing brighter than a fresh white coat.
                assert!(
                    (0.09..=0.75).contains(&wall),
                    "{index} {}: wall at {wall:.3}; masonry runs 0.09 (dark glazed \
                     tile) to 0.75 (new white render)",
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
            // highlight, and none of them is a mirror.
            for (what, value) in [("pier", design.pier), ("frame", design.frame)] {
                let v = luma(value);
                assert!(
                    (0.25..=0.80).contains(&v),
                    "{index} {}: {what} at {v:.3}, which no real trim material occupies",
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
            // What a facade needs is a *dark* rhythm to read at a kilometre; a
            // bright element only has to exist at all, and demanding 1.3x here
            // would force every sill and every mullion to be blown out.
            assert!(
                bright > median * 1.12,
                "{}: brightest tenth {bright:.3} is not lighter than its median {median:.3}",
                texture.name
            );
        }
    }

    /// Storey alignment.  A tile's height has to be an exact whole number of
    /// storeys, and a wall of any height has to land its floor lines on floors.
    #[test]
    fn the_tile_is_an_exact_whole_number_of_storeys() {
        let storeys = FACADE_TILE_H / STOREY_M;
        assert!(
            (storeys - storeys.round()).abs() < 1.0e-6,
            "the tile is {storeys} storeys; a fractional storey drifts"
        );
        assert_eq!(storeys.round() as u32, STOREYS_PER_TILE);
        assert!((FACADE_TILE_H - 12.8).abs() < 1.0e-6);
        // The ground storey is its own module, because a 4.5 m shopfront cannot
        // be squeezed into a 3.2 m tile.
        assert_eq!(GROUND_FLOOR_TILE_H, GROUND_STOREY_M);
        // A wall of any whole number of storeys above the ground floor ends
        // exactly on a storey line, for every count there is.
        for storeys in 1..=60_u32 {
            let wall = GROUND_STOREY_M + storeys as f32 * STOREY_M;
            let above = wall - GROUND_STOREY_M;
            let exact = above / STOREY_M;
            assert!(
                (exact - exact.round()).abs() < 1.0e-4,
                "{storeys} storeys drifts by {}",
                exact - exact.round()
            );
        }
        // The tile's own floor lines are on its edge, so tiling never doubles or
        // splits a band.
        assert!((FACADE_TILE_H % STOREY_M).abs() < 1.0e-6);
    }

    /// Distinctness.  Twenty-four tints of one wall is precisely what the
    /// previous port shipped, and it is invisible in any single screenshot and
    /// fatal in a whole district.
    #[test]
    fn the_twenty_four_tiles_are_measurably_different_from_each_other() {
        let textures = facade_textures(64);
        let mut values: Vec<f32> = textures.iter().map(median_albedo).map(luma).collect();
        values.sort_by(|a, b| a.total_cmp(b));
        let (lo, hi) = (values[0], *values.last().unwrap());
        let points: Vec<[f32; 4]> = textures.iter().map(|t| separable(t, lo, hi)).collect();
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
        assert!(hi - lo > 0.35, "the palette spans only {:.3} of value", hi - lo);
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

    /// The window module has to be a real structural module.  The spec table's
    /// legacy `cols` implies modules from 0.5 m to 1.5 m; half a metre of glass
    /// is not a thing anybody builds, so the bake uses its own bay count and
    /// this test is what holds it to a physical size.
    ///
    /// A *masonry* bay is allowed to be the full 3 m, because that is the 板楼
    /// structural bay: one punched window per 3 m of wall with 1.2 m of pier and
    /// render around it.  A curtain-wall module is not, because 3 m of glass
    /// between mullions is a swimming pool.
    #[test]
    fn every_bay_module_is_a_size_a_builder_could_order() {
        for (index, design) in DESIGNS.iter().enumerate() {
            let module = FACADE_TILE_W / design.bays.max(1) as f32;
            let (lo, hi) = if design.cladding == Cladding::CurtainWall {
                (0.70_f32, 1.60_f32)
            } else {
                (0.90, 3.05)
            };
            assert!(
                (lo..=hi).contains(&module),
                "{index} {}: a {module:.2} m bay module is not a real module",
                design.name
            );
            // A pier on every bay boundary, or the tile's own seam shows as a
            // hard vertical line down a wall.
            assert!(
                design.pier_w > 0.0 || design.major_pier_w > 0.0,
                "{index} {} has no pier, so the 3 m repeat will be visible",
                design.name
            );
            // And there has to be *wall* left.  This is the assertion that would
            // have caught the previous port's tiles, which put a 1.8 m window in
            // a 3 m bay and turned every wall into a black bar.
            let pane = design.open_w / module;
            assert!(
                pane <= 0.68,
                "{index} {}: the opening is {pane:.2} of its bay, which leaves no wall",
                design.name
            );
            // And it has to fit *between* the piers.  An opening that runs under
            // a pier is an opening cut by a column, and it is the most obvious
            // way a procedural facade gives itself away.
            let bays = design.bays.max(1) as i32;
            let (_, _, clear) = bay_clearance(design, bays, 0, module);
            assert!(
                design.open_w <= clear + 1.0e-4,
                "{index} {}: a {:.2} m opening in a {:.2} m clear wall",
                design.name,
                design.open_w,
                clear
            );
            assert!(
                design.open_w > clear * 0.55,
                "{index} {}: a {:.2} m opening in a {:.2} m clear wall is a slot, \
                 not a window",
                design.name,
                design.open_w,
                clear
            );
        }
    }

    /// A window is a window: it has a sill, a head, a reveal, and it fits inside
    /// its storey with the band to spare.
    #[test]
    fn every_opening_fits_inside_its_storey() {
        for (index, design) in DESIGNS.iter().enumerate() {
            if design.cladding == Cladding::CurtainWall {
                let glazed = STOREY_M - design.band_h - design.spandrel_h - design.cap_h;
                assert!(
                    (1.2..=2.4).contains(&glazed),
                    "{index} {}: a {glazed:.2} m vision panel in a {STOREY_M} m storey",
                    design.name
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
                head < STOREY_M - design.band_h - 0.2,
                "{index} {}: the window head at {head:.2} m runs into the storey band",
                design.name
            );
            assert!(
                (0.75..=1.05).contains(&design.sill_m),
                "{index} {}: a {:.2} m sill is not a sill",
                design.name,
                design.sill_m
            );
            assert!(
                (0.50..=2.0).contains(&design.open_w),
                "{index} {}: a {:.2} m wide window",
                design.name,
                design.open_w
            );
        }
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
            assert!((texture.tile_width_m - FACADE_TILE_W).abs() < 1.0e-6);
            assert!((texture.tile_height_m - FACADE_TILE_H).abs() < 1.0e-6);
        }
    }

    /// The three ground floors are a retail band, a portal and an entrance, and
    /// they are authored at true size: one 4.2 m bay by one 4.5 m storey, with
    /// no vertical repeat, so a 2.1 m door is 2.1 m on the wall.
    #[test]
    fn the_ground_floor_is_authored_at_true_scale() {
        let textures = ground_floor_textures(128);
        let names: Vec<&str> = textures.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, vec!["ground/shop", "ground/lobby", "ground/home"]);
        for texture in &textures {
            assert_eq!(texture.rgba.len(), 128 * 128 * 4);
            assert!((texture.tile_width_m - GROUND_FLOOR_TILE_W).abs() < 1.0e-6);
            assert!((texture.tile_height_m - GROUND_STOREY_M).abs() < 1.0e-6);
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
        assert!(hi > lo + 12, "the roof has no relief to differentiate ({}..{lo})", hi);
        let values: Vec<f32> = texture
            .rgba
            .chunks(4)
            .map(|pixel| luma([srgb_to_linear(pixel[0] as f32 / 255.0), srgb_to_linear(pixel[1] as f32 / 255.0), srgb_to_linear(pixel[2] as f32 / 255.0)]))
            .collect();
        let median = {
            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.total_cmp(b));
            sorted[sorted.len() / 2]
        };
        // Bitumen and gravel ballast, not a pale grey lid.
        assert!(
            (0.12..=0.30).contains(&median),
            "the roof's median reflectance is {median:.3}"
        );
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
}
