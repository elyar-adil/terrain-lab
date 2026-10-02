//! Building elevations: the design table, the tile bakes and the ground floors.
//!
//! This module is the visual quality of a city, and it is the one the previous
//! port got most wrong. What it produced was a flat colour block per wall: the
//! textures were decoded from the payload and then never bound to a material, so
//! every building was lit by one pale diffuse colour. From a kilometre away that
//! reads as a white monolith, which is the artefact this module exists to
//! remove.
//!
//! # The tile module and the V datum
//!
//! A tile is **one 开间 wide by four 层高 tall** — [`STOREYS_PER_TILE`] storeys
//! of the design's [`crate::facades::designs::FacadeDesign::storey_m`], so the
//! vertical size is an exact multiple of the storey height and tiling a wall of
//! any height lands every floor line exactly on a floor, with no drift and no
//! half-storey at the parapet. Every design puts a pier on every bay boundary,
//! which means the tile's own vertical seam always lands inside a pier and the
//! repeat is invisible; and a few designs put a *wider* pier on the tile
//! boundary, so the structural module is two bays and the repeat stops reading
//! as wallpaper.
//!
//! `buildings::facade_wall` measures `V` **downwards from the wall's head**, so
//! `V = 0` is the top of the last storey and every floor line lands on `V` a
//! multiple of `1 / STOREYS_PER_TILE` — on every wall of every building, at any
//! height.  The ground storey is a separate module ([`GROUND_STOREY_M`], 4.5 m)
//! with its own true-scale bake, because a 4.5 m shopfront cannot be squeezed
//! into a residential storey.
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
//!   than a noise field.  Since the geometry now derives from the same
//!   dimensioned designs (see `designs.rs`), the painted pier and the physical
//!   pier relief strip are the same width at the same pitch by construction.
//! * **At two metres**, as a wall you could touch. What carries it is material:
//!   a reveal that shades, a frame, a sill that catches light, rain-washing
//!   under a window, a blind down in one opening and not the next. A facade
//!   where every window is identical reads as a texture, not a building.

pub(crate) mod designs;
pub(crate) mod ground_floor;
pub(crate) mod tile;

pub use designs::{design, tile_height_m, Cladding, DESIGNS, FacadeDesign};
pub use ground_floor::ground_floor_textures;
pub use tile::{facade_textures, pitched_roof_texture, roof_texture, RoofCovering};

// ---------------------------------------------------------------------------
// the module constants
// ---------------------------------------------------------------------------

/// The default residential storey height — 层高 2.9 m.  Designs carry their own
/// `storey_m` (2.8 for the 1980s stock, 3.0 podium, 3.4 curtain wall); this
/// constant is the standard the rest of the crate quotes and the module of the
/// standard tile below.
pub const STOREY_M: f32 = 2.9;
/// Storeys per facade tile.
pub const STOREYS_PER_TILE: u32 = 4;
/// The standard structural bay — 开间 3.6 m — and the physical width of the
/// standard facade tile.  Individual designs bake at their own `bay_m`.
pub const FACADE_TILE_W: f32 = 3.6;
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
/// The physical size of one roof-tile texture, in metres.  A roof tile covers a
/// 3 m square of the actual tile field, so the texture's resolution is its real
/// texel density rather than an arbitrary number of pixels per roof.
pub const ROOF_TILE_M: f32 = 3.0;
