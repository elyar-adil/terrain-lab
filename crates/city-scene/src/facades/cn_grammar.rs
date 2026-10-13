//! The procedural facade grammar of the `ChineseModern` style.
//!
//! A Chinese city is not drawn from six elevations.  It is drawn from a handful
//! of *building cultures* — the 1980s walk-up, the tile-clad 1990s block, the
//! 2000s "European" compound, the white 2010s high-rise, 新中式, the urban
//! village, the office tower — and inside each one every development varies the
//! flat plan, the window, the balcony, the material and the colours.  So this
//! module does not store elevations; it *generates* them: a [`Design`] is drawn
//! from an [`Era`] and a seed (one seed per compound, so a compound is uniform
//! and the next compound is not), and each of its elevations is a sequence of
//! [`Span`]s composed flat by flat from the era's vocabulary.
//!
//! The renderer draws the elevations procedurally, from a table this module
//! packs into a small texture ([`FacadeTable::bake`], material `cnfacade`):
//! every wall carries its elevation's row in its vertex colour's alpha and its
//! position as UV (metres along the wall, storeys up from the elevation's
//! base).  The fragment shader (`src/city/facade.ts`) finds the span under the
//! fragment and draws frame, glass, room, skin and weathering analytically, at
//! any distance, with no texture to blur or alias.  The geometry layer builds
//! the balconies, bay windows and AC platforms from the same spans.

use std::collections::HashMap;

use crate::math::Rng;
use crate::textures::BakedTexture;

/// The material every procedural wall is drawn with.
pub const FACADE: &str = "cnfacade";
/// The table texture.
pub const TABLE: &str = "cnfacade/table";
/// Spans per elevation row.
pub const MAX_SPANS: usize = 20;
/// Header texels per row, then two texels per span.
pub const HEADER_TEXELS: usize = 8;
pub const ROW_TEXELS: usize = HEADER_TEXELS + 2 * MAX_SPANS;
/// Rows the vertex alpha can address.
pub const MAX_ROWS: usize = 256;

/// A building culture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Era {
    /// 1980s–90s 5–7 storey walk-up: render or brick, small windows, grilles,
    /// glazed-in balconies, solar water heaters on the roof.
    WalkUp,
    /// 1990s–2000s tile-clad (面砖) block: white, cream or pink tile, tinted
    /// glass, bay windows.
    Tile,
    /// 2000s "European" (欧式) compound: ochre and beige paint, white window
    /// surrounds, rusticated base, mansard crowns.
    Euro,
    /// 2010s high-rise: white and grey, dark frames, big windows, glass rails.
    Modern,
    /// 新中式: white walls, dark grey trim, grey-tile roofs.
    NewChinese,
    /// 城中村: dense private blocks, tile, grilles on every window.
    Village,
    /// Commercial: curtain wall, stone grid, panel ribbon.
    Office,
}

impl Era {
    const fn code(self) -> u8 {
        match self {
            Era::WalkUp => 0,
            Era::Tile => 1,
            Era::Euro => 2,
            Era::Modern => 3,
            Era::NewChinese => 4,
            Era::Village => 5,
            Era::Office => 6,
        }
    }
}

/// Which side of a building an elevation dresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Front,
    Back,
    Gable,
}

/// The wall material between the openings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skin {
    /// Smooth painted render.
    Paint,
    /// 面砖: small ceramic tile, `module` metres high.
    Tile,
    /// Dressed stone panels.
    Stone,
    /// Fair-faced brick.
    Brick,
    /// Aluminium composite panel.
    Panel,
    /// Rough, washed render of the older stock.
    Render,
}

impl Skin {
    const fn code(self) -> u8 {
        match self {
            Skin::Paint => 0,
            Skin::Tile => 1,
            Skin::Stone => 2,
            Skin::Brick => 3,
            Skin::Panel => 4,
            Skin::Render => 5,
        }
    }
}

/// What is behind the glass, and what the glass is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Glass {
    /// Clear float glass, a flat behind it.
    Residential,
    /// Blue-green tinted float glass of the 1990s, a flat behind it.
    Tinted,
    /// Coated glass, an office floor behind it.
    Office,
    /// A curtain wall: coated glass in the building's coating colour.
    Curtain,
}

impl Glass {
    const fn code(self) -> u8 {
        match self {
            Glass::Residential => 0,
            Glass::Tinted => 1,
            Glass::Office => 2,
            Glass::Curtain => 3,
        }
    }
}

/// How a window opens, which is what its frame pattern is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sash {
    /// Horizontal sliders, the Chinese default.
    Sliding,
    /// Side-hung casements with a fixed light.
    Casement,
    /// A big fixed light with a top-hung vent: the 2010s window.
    Fixed,
    /// A fixed light behind a vertical timber-look lattice: 新中式.
    Lattice,
}

impl Sash {
    const fn code(self) -> u8 {
        match self {
            Sash::Sliding => 0,
            Sash::Casement => 1,
            Sash::Fixed => 2,
            Sash::Lattice => 3,
        }
    }
}

/// A balcony's guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rail {
    /// A solid parapet in the body colour.
    Solid,
    /// Laminated glass between posts.
    Glass,
    /// Glazed in to the slab above (封阳台).
    Enclosed,
    /// White balusters or bars.
    Bars,
}

impl Rail {
    const fn code(self) -> u8 {
        match self {
            Rail::Solid => 0,
            Rail::Glass => 1,
            Rail::Enclosed => 2,
            Rail::Bars => 3,
        }
    }
}

/// What occupies one span of an elevation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Part {
    Wall,
    /// A vertical strip in the trim colour.
    Strip,
    Window {
        sill: f32,
        head: f32,
        panes: u8,
        sash: Sash,
    },
    /// A 飘窗: the geometry boxes it out `Design::bay_depth`.
    Bay {
        panes: u8,
        sash: Sash,
    },
    /// A projecting balcony in front of a glazed door wall.
    Balcony {
        rail: Rail,
    },
    /// A recessed balcony (凹阳台): the recess is drawn, nothing projects.
    Loggia {
        rail: Rail,
    },
    /// An AC niche behind a louvre screen; the geometry projects the platform.
    AcNiche,
    /// A stair window at the half landing.
    Stair,
    /// Unitised curtain wall with a mullion every `module` metres.
    Curtain {
        module: f32,
    },
    /// A stone pier of a grid facade; the geometry projects it.
    Pier,
    /// Aluminium panel.
    Panel,
}

impl Part {
    /// `(kind, p0, p1, packed)`: the table encoding of a part.
    fn encode(self, storey: f32) -> (u8, f32, f32, u8) {
        let nib = |panes: u8, style: u8| (panes.min(15)) | (style.min(15) << 4);
        match self {
            Part::Wall => (0, 0.0, 0.0, 0),
            Part::Strip => (1, 0.0, 0.0, 0),
            Part::Window {
                sill,
                head,
                panes,
                sash,
            } => (2, sill, head, nib(panes, sash.code())),
            Part::Bay { panes, sash } => (3, BAY_SILL_M, bay_head(storey), nib(panes, sash.code())),
            Part::Balcony { rail } => (4, 0.0, door_head(storey), nib(0, rail.code())),
            Part::AcNiche => (5, 0.08, LOUVRE_TOP_M, 0),
            Part::Stair => (6, 1.35, (storey - 0.25).min(2.65), 0),
            Part::Curtain { module } => (7, module, SPANDREL_M, 0),
            Part::Pier => (8, 0.0, 0.0, 0),
            Part::Panel => (9, 0.0, 0.0, 0),
            Part::Loggia { rail } => (10, 0.0, door_head(storey), nib(0, rail.code())),
        }
    }
}

/// Bay-window sill above the floor.
pub const BAY_SILL_M: f32 = 0.45;
/// The bay-window head: half a metre under the slab above.
pub fn bay_head(storey: f32) -> f32 {
    storey - 0.45
}
/// The head of a balcony door.
pub fn door_head(storey: f32) -> f32 {
    storey - 0.45
}
/// Louvre screen top above the floor.
pub const LOUVRE_TOP_M: f32 = 1.35;
/// Spandrel height of a curtain wall.
pub const SPANDREL_M: f32 = 0.9;

/// One span of an elevation, in metres from the tile's left edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub x0: f32,
    pub x1: f32,
    pub part: Part,
}

/// One elevation: a tile one flat wide (mirrored at every tile edge), one
/// storey tall, repeated up the wall.
#[derive(Debug, Clone, PartialEq)]
pub struct Elevation {
    pub side: Side,
    pub era: Era,
    pub width: f32,
    pub storey_m: f32,
    pub skin: Skin,
    /// Tile or stone module, metres.
    pub module_m: f32,
    pub glass: Glass,
    /// The lowest this many storeys carry security grilles; 255 is all.
    pub grille_storeys: u8,
    pub frame: [f32; 3],
    pub trim: [f32; 3],
    /// Glass tint (tinted and curtain glass).
    pub tint: [f32; 3],
    /// How weathered the wall is, 0..1.
    pub weather: f32,
    /// Bit 0: moulded window surrounds; bit 1: a band at every slab;
    /// bit 2: deep reveals.
    pub flags: u8,
    pub spans: Vec<Span>,
}

pub const FLAG_SURROUND: u8 = 1;
pub const FLAG_SLAB_BAND: u8 = 2;
pub const FLAG_DEEP_REVEAL: u8 = 4;

impl Elevation {
    /// Pack into one table row.
    fn pack(&self, row: &mut [u8]) {
        debug_assert_eq!(row.len(), ROW_TEXELS * 4);
        let u16be = |v: f32| {
            let mm = (v * 1000.0).round().clamp(0.0, 65_535.0) as u16;
            [(mm >> 8) as u8, (mm & 0xff) as u8]
        };
        let byte = |v: f32| (v * 255.0).round().clamp(0.0, 255.0) as u8;
        let spans = &self.spans[..self.spans.len().min(MAX_SPANS)];
        let w = u16be(self.width);
        row[0..4].copy_from_slice(&[
            w[0],
            w[1],
            (self.storey_m * 50.0).round().clamp(0.0, 255.0) as u8,
            spans.len() as u8,
        ]);
        row[4..8].copy_from_slice(&[
            self.skin.code(),
            (self.module_m * 100.0).round().clamp(0.0, 255.0) as u8,
            self.glass.code(),
            self.grille_storeys,
        ]);
        row[8..12].copy_from_slice(&[
            byte(self.frame[0]),
            byte(self.frame[1]),
            byte(self.frame[2]),
            self.era.code(),
        ]);
        row[12..16].copy_from_slice(&[
            byte(self.trim[0]),
            byte(self.trim[1]),
            byte(self.trim[2]),
            self.flags,
        ]);
        row[16..20].copy_from_slice(&[
            byte(self.tint[0]),
            byte(self.tint[1]),
            byte(self.tint[2]),
            byte(self.weather),
        ]);
        for (index, span) in spans.iter().enumerate() {
            let at = (HEADER_TEXELS + index * 2) * 4;
            let (kind, p0, p1, packed) = span.part.encode(self.storey_m);
            let x0 = u16be(span.x0);
            let x1 = u16be(span.x1);
            row[at..at + 4].copy_from_slice(&[x0[0], x0[1], x1[0], x1[1]]);
            row[at + 4..at + 8].copy_from_slice(&[
                kind,
                (p0 * 100.0).round().clamp(0.0, 255.0) as u8,
                (p1 * 50.0).round().clamp(0.0, 255.0) as u8,
                packed,
            ]);
        }
    }
}

/// How a block's roof ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Crown {
    /// A parapet and the roof plant.
    Flat,
    /// A white beam frame (构架) round the roof.
    Frame,
    /// A hipped clay-tile roof.
    Hip,
    /// A mansard with dormers: the 欧式 crown.
    Mansard,
    /// Grey-tile roofs with upturned eaves over the core: 新中式.
    NewChinese,
}

/// Everything one development's buildings share.
#[derive(Debug, Clone)]
pub struct Design {
    pub era: Era,
    pub front: Elevation,
    pub back: Elevation,
    pub gable: Elevation,
    /// Body colour, final linear reflectance.
    pub paint: [f32; 3],
    /// A darker base of `.1` storeys, a lighter or contrasting top of `.3`.
    pub base: ([f32; 3], u32),
    pub top: ([f32; 3], u32),
    pub balcony_depth: f32,
    pub bay_depth: f32,
    /// Balcony parapets in the trim colour rather than the body colour.
    pub trim_parapet: bool,
    /// Condensers hung on the wall, probability per eligible wall span.
    pub hung_ac: f32,
    /// Security cages over the windows of the grilled storeys.
    pub cages: bool,
    /// Solar water heaters on the roof.
    pub solar: bool,
    pub crown: Crown,
    /// Clay roof colour for a pitched crown.
    pub terracotta: bool,
}

fn pick<T: Copy>(rng: &mut Rng, table: &[(T, f32)]) -> T {
    let total: f32 = table.iter().map(|(_, w)| w).sum();
    let mut left = rng.unit() * total;
    for (value, weight) in table {
        if left < *weight {
            return *value;
        }
        left -= weight;
    }
    table[table.len() - 1].0
}

fn jitter(rng: &mut Rng, colour: [f32; 3], amount: f32) -> [f32; 3] {
    let k = 1.0 + rng.range(-amount, amount);
    [colour[0] * k, colour[1] * k, colour[2] * k]
}

/// Round to 5 cm, which is how elevations are dimensioned.
fn r5(v: f32) -> f32 {
    (v * 20.0).round() / 20.0
}

/// The era of a residential development: by height band, with every culture
/// present in every band where it exists in reality.
pub fn residential_era(seed: u32, floors: u16) -> Era {
    let mut rng = Rng::new(seed ^ 0x51ed_270b);
    if floors <= 7 {
        pick(
            &mut rng,
            &[
                (Era::WalkUp, 4.0),
                (Era::Euro, 2.0),
                (Era::NewChinese, 1.5),
                (Era::Village, 1.5),
                (Era::Modern, 1.0),
                (Era::Tile, 1.0),
            ],
        )
    } else if floors <= 18 {
        pick(
            &mut rng,
            &[
                (Era::Tile, 3.5),
                (Era::Euro, 2.5),
                (Era::Modern, 3.0),
                (Era::NewChinese, 1.0),
                (Era::WalkUp, 0.5),
            ],
        )
    } else {
        pick(
            &mut rng,
            &[
                (Era::Modern, 4.5),
                (Era::Tile, 2.5),
                (Era::Euro, 2.5),
                (Era::NewChinese, 0.5),
            ],
        )
    }
}

/// The parameters an era draws a design from.
struct Vocabulary {
    storey: (f32, f32),
    skins: &'static [(Skin, f32)],
    paints: &'static [([f32; 3], f32)],
    frames: &'static [([f32; 3], f32)],
    trim: &'static [([f32; 3], f32)],
    sashes: &'static [(Sash, f32)],
    rails: &'static [(Rail, f32)],
    bay: f32,
    loggia: f32,
    ac_niche: f32,
    hung_ac: f32,
    window_w: (f32, f32),
    sill: (f32, f32),
    glass: &'static [(Glass, f32)],
    grilles: (u8, u8),
    weather: (f32, f32),
    flags: &'static [(u8, f32)],
    crowns: &'static [(Crown, f32)],
}

const WHITE_FRAME: [f32; 3] = [0.66, 0.67, 0.66];
const SILVER_FRAME: [f32; 3] = [0.48, 0.50, 0.51];
const DARK_FRAME: [f32; 3] = [0.075, 0.08, 0.085];
const GREEN_FRAME: [f32; 3] = [0.20, 0.36, 0.30];
const BLUE_FRAME: [f32; 3] = [0.18, 0.28, 0.42];
const WOOD_FRAME: [f32; 3] = [0.22, 0.13, 0.08];
const WHITE_TRIM: [f32; 3] = [0.72, 0.72, 0.69];
const GREY_TRIM: [f32; 3] = [0.14, 0.145, 0.15];
const STONE_TRIM: [f32; 3] = [0.55, 0.50, 0.42];

fn vocabulary(era: Era) -> Vocabulary {
    match era {
        Era::WalkUp => Vocabulary {
            storey: (2.8, 2.8),
            skins: &[(Skin::Render, 4.0), (Skin::Brick, 3.0), (Skin::Tile, 3.0)],
            paints: &[
                ([0.56, 0.55, 0.51], 3.0),
                ([0.62, 0.58, 0.48], 2.0),
                ([0.60, 0.52, 0.42], 1.5),
                ([0.66, 0.64, 0.60], 1.5),
                ([0.52, 0.46, 0.42], 1.0),
            ],
            frames: &[
                (WHITE_FRAME, 4.0),
                (SILVER_FRAME, 2.0),
                (GREEN_FRAME, 1.5),
                (BLUE_FRAME, 1.5),
            ],
            trim: &[(WHITE_TRIM, 2.0), ([0.50, 0.49, 0.46], 1.0)],
            sashes: &[(Sash::Sliding, 3.0), (Sash::Casement, 2.0)],
            rails: &[(Rail::Enclosed, 5.0), (Rail::Solid, 3.0), (Rail::Bars, 1.0)],
            bay: 0.0,
            loggia: 0.15,
            ac_niche: 0.0,
            hung_ac: 0.75,
            window_w: (1.2, 1.8),
            sill: (0.9, 0.9),
            glass: &[(Glass::Residential, 3.0), (Glass::Tinted, 1.0)],
            grilles: (2, 7),
            weather: (0.55, 0.95),
            flags: &[(0, 3.0), (FLAG_SLAB_BAND, 1.0)],
            crowns: &[(Crown::Flat, 6.0), (Crown::Hip, 4.0)],
        },
        Era::Tile => Vocabulary {
            storey: (2.9, 2.9),
            skins: &[(Skin::Tile, 7.0), (Skin::Paint, 3.0)],
            paints: &[
                ([0.72, 0.72, 0.70], 3.0),
                ([0.70, 0.64, 0.52], 3.0),
                ([0.68, 0.53, 0.48], 1.5),
                ([0.60, 0.61, 0.62], 1.5),
                ([0.72, 0.66, 0.46], 1.0),
            ],
            frames: &[(WHITE_FRAME, 4.0), (SILVER_FRAME, 2.0)],
            trim: &[
                (WHITE_TRIM, 3.0),
                ([0.40, 0.42, 0.48], 1.0),
                ([0.55, 0.35, 0.28], 1.0),
            ],
            sashes: &[(Sash::Sliding, 4.0), (Sash::Casement, 1.0)],
            rails: &[(Rail::Enclosed, 4.0), (Rail::Solid, 3.0), (Rail::Bars, 1.0)],
            bay: 0.55,
            loggia: 0.1,
            ac_niche: 0.25,
            hung_ac: 0.55,
            window_w: (1.5, 2.1),
            sill: (0.85, 0.9),
            glass: &[(Glass::Tinted, 3.0), (Glass::Residential, 2.0)],
            grilles: (0, 2),
            weather: (0.35, 0.7),
            flags: &[(0, 2.0), (FLAG_SLAB_BAND, 1.0)],
            crowns: &[(Crown::Frame, 4.0), (Crown::Flat, 3.0), (Crown::Hip, 3.0)],
        },
        Era::Euro => Vocabulary {
            storey: (3.0, 3.0),
            skins: &[(Skin::Paint, 8.0), (Skin::Stone, 2.0)],
            paints: &[
                ([0.66, 0.55, 0.40], 3.0),
                ([0.70, 0.52, 0.30], 2.0),
                ([0.68, 0.48, 0.38], 2.0),
                ([0.74, 0.64, 0.42], 2.0),
                ([0.56, 0.34, 0.24], 1.0),
            ],
            frames: &[(WHITE_FRAME, 3.0), (SILVER_FRAME, 1.0), (WOOD_FRAME, 1.0)],
            trim: &[(WHITE_TRIM, 5.0), ([0.70, 0.66, 0.56], 2.0)],
            sashes: &[(Sash::Casement, 4.0), (Sash::Sliding, 1.0)],
            rails: &[(Rail::Bars, 4.0), (Rail::Solid, 3.0), (Rail::Glass, 1.0)],
            bay: 0.3,
            loggia: 0.15,
            ac_niche: 0.6,
            hung_ac: 0.2,
            window_w: (1.5, 2.1),
            sill: (0.6, 0.9),
            glass: &[(Glass::Residential, 3.0), (Glass::Tinted, 1.0)],
            grilles: (0, 1),
            weather: (0.2, 0.55),
            flags: &[
                (FLAG_SURROUND, 3.0),
                (FLAG_SURROUND | FLAG_SLAB_BAND, 2.0),
                (FLAG_SURROUND | FLAG_DEEP_REVEAL, 1.0),
            ],
            crowns: &[
                (Crown::Mansard, 6.0),
                (Crown::Hip, 3.0),
                (Crown::Frame, 1.0),
            ],
        },
        Era::Modern => Vocabulary {
            storey: (2.95, 3.0),
            skins: &[(Skin::Paint, 6.0), (Skin::Stone, 3.0), (Skin::Tile, 1.0)],
            paints: &[
                ([0.72, 0.72, 0.70], 4.0),
                ([0.58, 0.59, 0.60], 2.5),
                ([0.56, 0.52, 0.47], 2.0),
                ([0.64, 0.58, 0.48], 1.5),
                ([0.66, 0.62, 0.56], 1.5),
            ],
            frames: &[(DARK_FRAME, 6.0), (SILVER_FRAME, 2.0), (WHITE_FRAME, 1.0)],
            trim: &[
                (WHITE_TRIM, 3.0),
                ([0.36, 0.37, 0.38], 2.0),
                (STONE_TRIM, 1.0),
            ],
            sashes: &[(Sash::Fixed, 5.0), (Sash::Sliding, 1.0)],
            rails: &[
                (Rail::Glass, 6.0),
                (Rail::Solid, 2.0),
                (Rail::Enclosed, 1.0),
            ],
            bay: 0.3,
            loggia: 0.25,
            ac_niche: 0.85,
            hung_ac: 0.05,
            window_w: (1.8, 2.7),
            sill: (0.45, 0.6),
            glass: &[(Glass::Residential, 4.0), (Glass::Tinted, 1.0)],
            grilles: (0, 0),
            weather: (0.05, 0.3),
            flags: &[
                (FLAG_SLAB_BAND, 3.0),
                (0, 2.0),
                (FLAG_SLAB_BAND | FLAG_DEEP_REVEAL, 1.0),
            ],
            crowns: &[(Crown::Frame, 6.0), (Crown::Flat, 3.0)],
        },
        Era::NewChinese => Vocabulary {
            storey: (3.0, 3.0),
            skins: &[(Skin::Paint, 6.0), (Skin::Stone, 2.0)],
            paints: &[([0.74, 0.73, 0.70], 5.0), ([0.62, 0.60, 0.56], 2.0)],
            frames: &[(DARK_FRAME, 6.0), ([0.22, 0.22, 0.22], 2.0)],
            trim: &[(GREY_TRIM, 5.0), ([0.30, 0.30, 0.30], 2.0)],
            sashes: &[(Sash::Lattice, 3.0), (Sash::Fixed, 3.0)],
            rails: &[(Rail::Glass, 4.0), (Rail::Solid, 3.0)],
            bay: 0.15,
            loggia: 0.3,
            ac_niche: 0.85,
            hung_ac: 0.05,
            window_w: (1.5, 2.4),
            sill: (0.45, 0.9),
            glass: &[(Glass::Residential, 1.0)],
            grilles: (0, 0),
            weather: (0.05, 0.3),
            flags: &[
                (FLAG_SLAB_BAND, 3.0),
                (FLAG_SLAB_BAND | FLAG_DEEP_REVEAL, 1.0),
            ],
            crowns: &[(Crown::NewChinese, 8.0), (Crown::Flat, 2.0)],
        },
        Era::Village => Vocabulary {
            storey: (3.0, 3.1),
            skins: &[(Skin::Tile, 7.0), (Skin::Render, 3.0)],
            paints: &[
                ([0.72, 0.72, 0.70], 3.0),
                ([0.70, 0.64, 0.52], 2.0),
                ([0.68, 0.53, 0.50], 2.0),
                ([0.56, 0.62, 0.68], 1.0),
                ([0.58, 0.66, 0.58], 1.0),
            ],
            frames: &[(WHITE_FRAME, 3.0), (SILVER_FRAME, 3.0), (BLUE_FRAME, 1.0)],
            trim: &[(WHITE_TRIM, 2.0), ([0.52, 0.52, 0.50], 1.0)],
            sashes: &[(Sash::Sliding, 5.0)],
            rails: &[(Rail::Bars, 4.0), (Rail::Enclosed, 3.0)],
            bay: 0.0,
            loggia: 0.0,
            ac_niche: 0.0,
            hung_ac: 0.9,
            window_w: (1.0, 1.8),
            sill: (0.9, 1.0),
            glass: &[(Glass::Tinted, 2.0), (Glass::Residential, 2.0)],
            grilles: (255, 255),
            weather: (0.5, 0.9),
            flags: &[(0, 1.0)],
            crowns: &[(Crown::Flat, 1.0)],
        },
        Era::Office => Vocabulary {
            storey: (3.6, 3.9),
            skins: &[(Skin::Stone, 1.0)],
            paints: &[([0.56, 0.51, 0.43], 1.0)],
            frames: &[(DARK_FRAME, 1.0)],
            trim: &[(WHITE_TRIM, 1.0)],
            sashes: &[(Sash::Fixed, 1.0)],
            rails: &[(Rail::Glass, 1.0)],
            bay: 0.0,
            loggia: 0.0,
            ac_niche: 0.0,
            hung_ac: 0.0,
            window_w: (1.8, 3.0),
            sill: (0.75, 0.9),
            glass: &[(Glass::Office, 1.0)],
            grilles: (0, 0),
            weather: (0.1, 0.3),
            flags: &[(0, 1.0)],
            crowns: &[(Crown::Flat, 1.0)],
        },
    }
}

/// A builder that lays spans left to right and keeps them on the 5 cm grid.
struct Lay {
    x: f32,
    spans: Vec<Span>,
}

impl Lay {
    fn new() -> Self {
        Self {
            x: 0.0,
            spans: Vec::new(),
        }
    }
    fn push(&mut self, width: f32, part: Part) {
        let w = r5(width).max(0.05);
        if self.spans.len() >= MAX_SPANS {
            return;
        }
        // Merge consecutive walls, so the shader scans fewer spans.
        if part == Part::Wall
            && let Some(last) = self.spans.last_mut()
            && last.part == Part::Wall
        {
            last.x1 = r5(last.x1 + w);
            self.x = last.x1;
            return;
        }
        let x0 = self.x;
        self.x = r5(x0 + w);
        self.spans.push(Span {
            x0,
            x1: self.x,
            part,
        });
    }
}

/// A window of the era at a drawn width.
fn window(rng: &mut Rng, voc: &Vocabulary, storey: f32, width: f32) -> Part {
    let sash = pick(rng, voc.sashes);
    let sill = r5(rng.range(voc.sill.0, voc.sill.1));
    let head = r5((storey - rng.range(0.40, 0.55)).max(sill + 1.1));
    let panes = match sash {
        Sash::Sliding => {
            if width > 1.9 {
                3
            } else {
                2
            }
        }
        Sash::Casement => (width / 0.6).round().clamp(2.0, 4.0) as u8,
        Sash::Fixed => (width / 0.9).round().clamp(1.0, 3.0) as u8,
        Sash::Lattice => (width / 0.75).round().clamp(2.0, 4.0) as u8,
    };
    Part::Window {
        sill,
        head,
        panes,
        sash,
    }
}

/// The living side of one flat: a sequence of rooms, each a window, a bay
/// window or a balcony, with AC niches and piers between them.
fn front_spans(rng: &mut Rng, voc: &Vocabulary, storey: f32, village: bool) -> Vec<Span> {
    let mut lay = Lay::new();
    if !village {
        lay.push(rng.range(0.2, 0.4), Part::Strip);
    }
    lay.push(rng.range(0.15, 0.45), Part::Wall);
    // The room sequence of the flat: bedrooms either side of the living room
    // in most plans, two bedrooms on one side in some.
    let plans: &[&[u8]] = &[b"BLM", b"BLM", b"LBM", b"BLBM", b"MLB", b"BL"];
    let plan = plans[rng.int(plans.len() as u32) as usize];
    let rail = pick(rng, voc.rails);
    let loggia = rng.chance(voc.loggia);
    let bay = rng.chance(voc.bay);
    let ac = rng.chance(voc.ac_niche);
    for (index, room) in plan.iter().enumerate() {
        if index > 0 {
            lay.push(rng.range(0.25, 0.6), Part::Wall);
        }
        match room {
            b'L' => {
                let w = rng.range(3.0, 4.8);
                if loggia {
                    lay.push(w, Part::Loggia { rail });
                } else {
                    lay.push(w, Part::Balcony { rail });
                }
            }
            _ => {
                let w = rng.range(voc.window_w.0, voc.window_w.1)
                    + if *room == b'M' { 0.3 } else { 0.0 };
                if bay && *room == b'M' || bay && rng.chance(0.5) {
                    let sash = pick(rng, voc.sashes);
                    lay.push(w, Part::Bay { panes: 3, sash });
                } else {
                    let part = window(rng, voc, storey, w);
                    lay.push(w, part);
                }
                if ac {
                    lay.push(rng.range(0.6, 0.8), Part::AcNiche);
                }
            }
        }
    }
    lay.push(rng.range(0.15, 0.45), Part::Wall);
    lay.spans
}

/// The service side of one flat: kitchen, bathroom, a bedroom, and the stair
/// at the tile edge, which mirrors into a full stair window.
fn back_spans(rng: &mut Rng, voc: &Vocabulary, storey: f32, village: bool) -> Vec<Span> {
    let mut lay = Lay::new();
    if !village {
        lay.push(rng.range(0.2, 0.35), Part::Strip);
    }
    lay.push(rng.range(0.3, 0.8), Part::Wall);
    let kitchen_w = rng.range(1.2, 1.8);
    let kitchen = window(rng, voc, storey, kitchen_w);
    lay.push(kitchen_w, kitchen);
    if rng.chance(voc.ac_niche) {
        lay.push(rng.range(0.6, 0.8), Part::AcNiche);
    }
    lay.push(rng.range(0.8, 1.6), Part::Wall);
    let bath_head = r5(storey - 0.45);
    lay.push(
        rng.range(0.6, 0.9),
        Part::Window {
            sill: r5((bath_head - 0.9).max(1.2)),
            head: bath_head,
            panes: 1,
            sash: Sash::Casement,
        },
    );
    lay.push(rng.range(0.9, 1.8), Part::Wall);
    let bedroom_w = rng.range(1.2, 1.8);
    let bedroom = window(rng, voc, storey, bedroom_w);
    lay.push(bedroom_w, bedroom);
    if rng.chance(voc.ac_niche) {
        lay.push(rng.range(0.6, 0.8), Part::AcNiche);
    }
    lay.push(rng.range(0.4, 1.2), Part::Wall);
    if village {
        let part = window(rng, voc, storey, 1.2);
        lay.push(1.2, part);
        lay.push(0.3, Part::Wall);
    } else {
        lay.push(rng.range(0.6, 0.75), Part::Stair);
    }
    lay.spans
}

/// The end wall: mostly blank, one column of small windows, a strip.
fn gable_spans(rng: &mut Rng, voc: &Vocabulary, storey: f32) -> Vec<Span> {
    let mut lay = Lay::new();
    lay.push(rng.range(0.3, 0.5), Part::Strip);
    lay.push(rng.range(1.6, 2.4), Part::Wall);
    let w = rng.range(0.9, 1.5);
    let part = window(rng, voc, storey, w);
    lay.push(w, part);
    lay.push(rng.range(1.6, 2.6), Part::Wall);
    lay.spans
}

fn width_of(spans: &[Span]) -> f32 {
    spans.last().map_or(1.0, |span| span.x1)
}

/// Draw one residential development.  `seed` is the development's (one per
/// compound), so its buildings match each other; `floors` and `tower` pick
/// the crown and the colour bands.
pub fn residential_design(seed: u32, era: Era, floors: u16, tower: bool) -> Design {
    let mut rng = Rng::new(seed ^ 0x2545_f491);
    let voc = vocabulary(era);
    let village = era == Era::Village;
    let storey = r5(rng.range(voc.storey.0, voc.storey.1));
    let skin = pick(&mut rng, voc.skins);
    let paint = pick(&mut rng, voc.paints);
    let paint = jitter(&mut rng, paint, 0.05);
    let paint = if skin == Skin::Brick {
        if rng.chance(0.7) {
            [0.34, 0.15, 0.10]
        } else {
            [0.33, 0.33, 0.32]
        }
    } else {
        paint
    };
    let frame = pick(&mut rng, voc.frames);
    let trim = pick(&mut rng, voc.trim);
    let glass = pick(&mut rng, voc.glass);
    let tint = if glass == Glass::Tinted {
        pick(
            &mut rng,
            &[
                ([0.30, 0.55, 0.52], 3.0),
                ([0.28, 0.42, 0.58], 2.0),
                ([0.40, 0.52, 0.40], 1.0),
            ],
        )
    } else {
        [0.6, 0.66, 0.70]
    };
    let grilles = if voc.grilles.0 == voc.grilles.1 {
        voc.grilles.0
    } else {
        voc.grilles.0 + rng.int((voc.grilles.1 - voc.grilles.0) as u32 + 1) as u8
    };
    let weather = rng.range(voc.weather.0, voc.weather.1);
    let flags = pick(&mut rng, voc.flags);
    let module = match skin {
        Skin::Tile => rng.range(0.05, 0.10),
        Skin::Stone => rng.range(0.6, 1.2),
        Skin::Brick => 0.06,
        Skin::Panel => 1.2,
        _ => 0.0,
    };
    let elevation = |side: Side, spans: Vec<Span>| Elevation {
        side,
        era,
        width: width_of(&spans),
        storey_m: storey,
        skin,
        module_m: module,
        glass,
        grille_storeys: grilles,
        frame,
        trim,
        tint,
        weather,
        flags,
        spans,
    };
    let front = elevation(Side::Front, front_spans(&mut rng, &voc, storey, village));
    let back = elevation(Side::Back, back_spans(&mut rng, &voc, storey, village));
    let gable = elevation(Side::Gable, gable_spans(&mut rng, &voc, storey));

    let shade = |c: [f32; 3], k: f32| [c[0] * k, c[1] * k, c[2] * k];
    let base_storeys = match era {
        Era::Euro => 2,
        Era::NewChinese | Era::Modern => {
            if tower {
                2
            } else {
                1
            }
        }
        Era::Village | Era::WalkUp => 0,
        _ => {
            if floors >= 8 {
                1 + rng.int(2)
            } else {
                rng.int(2)
            }
        }
    };
    let base_colour = match era {
        Era::Euro => pick(
            &mut rng,
            &[([0.48, 0.40, 0.31], 2.0), ([0.56, 0.50, 0.42], 2.0)],
        ),
        Era::NewChinese => [0.30, 0.30, 0.30],
        _ => pick(
            &mut rng,
            &[
                ([0.40, 0.34, 0.28], 1.0),
                ([0.46, 0.44, 0.41], 1.0),
                (shade(paint, 0.72), 2.0),
            ],
        ),
    };
    let top_storeys = if floors >= 6 && rng.chance(0.5) && era != Era::Village {
        1
    } else {
        0
    };
    let top_colour = if rng.chance(0.4) {
        trim
    } else {
        shade(paint, 0.86)
    };
    let crown = pick(&mut rng, voc.crowns);
    let crown = match crown {
        // A hipped or mansard roof on a thirty-storey slab is not built; the
        // tall stock of those cultures crowns itself with a frame instead.
        Crown::Hip if floors > 11 && !tower => Crown::Frame,
        other => other,
    };
    Design {
        era,
        front,
        back,
        gable,
        paint,
        base: (base_colour, base_storeys),
        top: (top_colour, top_storeys),
        balcony_depth: r5(rng.range(1.2, 1.6)),
        bay_depth: r5(rng.range(0.4, 0.6)),
        trim_parapet: matches!(era, Era::Euro | Era::Modern | Era::NewChinese) || rng.chance(0.3),
        hung_ac: voc.hung_ac,
        cages: matches!(era, Era::WalkUp | Era::Village) && rng.chance(0.7),
        solar: era == Era::WalkUp && rng.chance(0.75),
        crown,
        terracotta: rng.chance(0.55),
    }
}

/// The skin of a commercial building.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfficeSkin {
    Curtain,
    Grid,
    Ribbon,
}

/// Draw one commercial elevation: a curtain wall, a stone grid or a ribbon
/// block, with its module, proportions and colours from the seed.
pub fn office_elevation(seed: u32, skin: OfficeSkin, cladding: [f32; 3]) -> Elevation {
    let mut rng = Rng::new(seed ^ 0x7f4a_7c15);
    let storey = r5(rng.range(3.6, 4.2));
    let coating = pick(
        &mut rng,
        &[
            ([0.42, 0.66, 0.66], 3.0),
            ([0.38, 0.52, 0.74], 2.5),
            ([0.66, 0.70, 0.72], 2.0),
            ([0.30, 0.33, 0.36], 1.5),
            ([0.44, 0.62, 0.48], 1.0),
        ],
    );
    let frame = pick(&mut rng, &[(DARK_FRAME, 2.0), (SILVER_FRAME, 2.0)]);
    let mut lay = Lay::new();
    let (glass, skin_kind) = match skin {
        OfficeSkin::Curtain => {
            let module = r5(rng.range(1.2, 1.8));
            let modules = (rng.range(3.0, 5.0)).round();
            lay.push(module * modules, Part::Curtain { module });
            (Glass::Curtain, Skin::Panel)
        }
        OfficeSkin::Grid => {
            let pier = rng.range(0.4, 0.9);
            let w = rng.range(1.8, 3.6);
            let panes = (w / 1.1).round().clamp(2.0, 4.0) as u8;
            lay.push(pier * 0.5, Part::Pier);
            lay.push(
                w,
                Part::Window {
                    sill: r5(rng.range(0.6, 0.9)),
                    head: r5(storey - rng.range(0.4, 0.7)),
                    panes,
                    sash: Sash::Fixed,
                },
            );
            lay.push(pier * 0.5, Part::Pier);
            (Glass::Office, Skin::Stone)
        }
        OfficeSkin::Ribbon => {
            let w = rng.range(5.0, 9.0);
            lay.push(0.3, Part::Panel);
            lay.push(
                w,
                Part::Window {
                    sill: r5(rng.range(0.9, 1.1)),
                    head: r5(storey - rng.range(0.35, 0.5)),
                    panes: (w / 1.4).round().clamp(3.0, 8.0) as u8,
                    sash: Sash::Fixed,
                },
            );
            lay.push(0.3, Part::Panel);
            (Glass::Office, Skin::Panel)
        }
    };
    let spans = lay.spans;
    Elevation {
        side: Side::Front,
        era: Era::Office,
        width: width_of(&spans),
        storey_m: storey,
        skin: skin_kind,
        module_m: if skin_kind == Skin::Stone {
            rng.range(0.6, 1.2)
        } else {
            1.2
        },
        glass,
        grille_storeys: 0,
        frame,
        trim: cladding,
        tint: coating,
        weather: rng.range(0.05, 0.3),
        flags: 0,
        spans,
    }
}

/// The table of every elevation a city uses.  Designs are cached by key, so a
/// development's buildings share their rows, and a city that needs more rows
/// than the vertex alpha can address reuses the earliest ones.
#[derive(Debug, Default)]
pub struct FacadeTable {
    rows: Vec<Elevation>,
    designs: HashMap<u64, Design>,
    ids: HashMap<u64, [u8; 3]>,
    offices: HashMap<u64, u8>,
}

impl FacadeTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, elevation: Elevation, salt: u64) -> u8 {
        if self.rows.len() < MAX_ROWS {
            self.rows.push(elevation);
            (self.rows.len() - 1) as u8
        } else {
            (salt % MAX_ROWS as u64) as u8
        }
    }

    /// The design of a development and the rows of its front, back and gable.
    pub fn residential(&mut self, seed: u32, floors: u16, tower: bool) -> (Design, [u8; 3]) {
        let era = residential_era(seed, floors);
        // Height band and tower-ness change the crown and bands, so they key
        // the design along with the seed.
        let key = ((seed as u64) << 16) | ((floors.min(255) as u64) << 1) | tower as u64;
        if let (Some(design), Some(ids)) = (self.designs.get(&key), self.ids.get(&key)) {
            return (design.clone(), *ids);
        }
        let design = residential_design(seed, era, floors, tower);
        let ids = [
            self.add(design.front.clone(), key),
            self.add(design.back.clone(), key ^ 1),
            self.add(design.gable.clone(), key ^ 2),
        ];
        self.designs.insert(key, design.clone());
        self.ids.insert(key, ids);
        (design, ids)
    }

    /// The row of one commercial elevation.
    pub fn office(&mut self, seed: u32, skin: OfficeSkin, cladding: [f32; 3]) -> (Elevation, u8) {
        let key = ((seed as u64) << 2)
            | match skin {
                OfficeSkin::Curtain => 0,
                OfficeSkin::Grid => 1,
                OfficeSkin::Ribbon => 2,
            };
        let elevation = office_elevation(seed, skin, cladding);
        if let Some(id) = self.offices.get(&key) {
            return (elevation, *id);
        }
        let id = self.add(elevation.clone(), key);
        self.offices.insert(key, id);
        (elevation, id)
    }

    /// How many rows are curtain walls, and how many are not.
    pub fn glass_census(&self) -> (usize, usize) {
        let curtain = self
            .rows
            .iter()
            .filter(|row| row.glass == Glass::Curtain)
            .count();
        (curtain, self.rows.len() - curtain)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// The table texture: one row per elevation, linear bytes, read with
    /// `texelFetch` and no filtering.
    pub fn bake(&self) -> BakedTexture {
        let rows = self.rows.len().max(1);
        let mut rgba = vec![0_u8; ROW_TEXELS * rows * 4];
        for (index, row) in self.rows.iter().enumerate() {
            let at = index * ROW_TEXELS * 4;
            row.pack(&mut rgba[at..at + ROW_TEXELS * 4]);
        }
        BakedTexture {
            name: TABLE.to_owned(),
            width: ROW_TEXELS,
            height: rows,
            tile_width_m: 1.0,
            tile_height_m: 1.0,
            has_normal_source: false,
            rgba,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERAS: [Era; 6] = [
        Era::WalkUp,
        Era::Tile,
        Era::Euro,
        Era::Modern,
        Era::NewChinese,
        Era::Village,
    ];

    #[test]
    fn spans_tile_every_generated_elevation() {
        for era in ERAS {
            for seed in 0..60 {
                let d = residential_design(seed, era, 18, seed % 2 == 0);
                for e in [&d.front, &d.back, &d.gable] {
                    let mut x = 0.0_f32;
                    assert!(!e.spans.is_empty() && e.spans.len() <= MAX_SPANS);
                    for s in &e.spans {
                        assert!((s.x0 - x).abs() < 1e-3, "{era:?} gap at {x}");
                        assert!(s.x1 > s.x0);
                        x = s.x1;
                    }
                    assert!((x - e.width).abs() < 1e-3);
                    assert!(
                        (5.0..=20.0).contains(&e.width) || e.side == Side::Gable,
                        "{era:?} {:?} width {}",
                        e.side,
                        e.width
                    );
                }
            }
        }
    }

    /// The point of a grammar: designs differ.  Sixty seeds of one era must
    /// not collapse to a few elevations.
    #[test]
    fn a_culture_yields_many_distinct_elevations() {
        for era in ERAS {
            let mut seen = std::collections::HashSet::new();
            for seed in 0..60 {
                let d = residential_design(seed, era, 12, false);
                let signature: Vec<(i32, i32, u8)> = d
                    .front
                    .spans
                    .iter()
                    .map(|s| {
                        (
                            (s.x0 * 20.0) as i32,
                            (s.x1 * 20.0) as i32,
                            s.part.encode(3.0).0,
                        )
                    })
                    .collect();
                seen.insert(signature);
            }
            assert!(
                seen.len() >= 40,
                "{era:?}: only {} distinct fronts",
                seen.len()
            );
        }
    }

    #[test]
    fn openings_fit_in_their_storey() {
        for era in ERAS {
            for seed in 0..40 {
                let d = residential_design(seed, era, 12, false);
                for e in [&d.front, &d.back, &d.gable] {
                    for s in &e.spans {
                        if let Part::Window { sill, head, .. } = s.part {
                            assert!(sill >= 0.3 && head > sill + 0.5 && head <= e.storey_m - 0.2);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_table_round_trips_its_widths_and_spans() {
        let mut table = FacadeTable::new();
        let (design, ids) = table.residential(7, 18, true);
        let texture = table.bake();
        assert_eq!(texture.width, ROW_TEXELS);
        let row = &texture.rgba[ids[0] as usize * ROW_TEXELS * 4..];
        let width = ((row[0] as u32) << 8 | row[1] as u32) as f32 / 1000.0;
        assert!((width - design.front.width).abs() < 0.002);
        assert_eq!(row[3] as usize, design.front.spans.len());
        let span = &row[HEADER_TEXELS * 4..];
        let x1 = ((span[2] as u32) << 8 | span[3] as u32) as f32 / 1000.0;
        assert!((x1 - design.front.spans[0].x1).abs() < 0.002);
    }

    #[test]
    fn a_development_shares_its_rows() {
        let mut table = FacadeTable::new();
        let (_, a) = table.residential(3, 12, false);
        let (_, b) = table.residential(3, 12, false);
        assert_eq!(a, b);
        assert_eq!(table.len(), 3);
    }

    #[test]
    fn every_culture_appears_in_the_height_bands_it_exists_in() {
        let mut low = std::collections::HashSet::new();
        let mut tall = std::collections::HashSet::new();
        for seed in 0..400 {
            low.insert(residential_era(seed, 6));
            tall.insert(residential_era(seed, 26));
        }
        assert!(low.contains(&Era::WalkUp) && low.contains(&Era::Village));
        assert!(tall.contains(&Era::Modern) && tall.contains(&Era::Euro));
        assert!(!tall.contains(&Era::Village));
    }
}
