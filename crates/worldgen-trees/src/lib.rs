//! Trees, every one of them different.
//!
//! A tree is a pure function of its [`TreeSpec`]: which species, its own seed, its
//! height and the conditions it grew in and the day of the year it is seen on. The
//! grower turns that into a branch skeleton and into individual leaves, each with
//! its own size, outline parameters, orientation, tint and moment of turning. No
//! tree is a stamp of another and no leaf a copy of its neighbour; only the species'
//! means are shared. Nothing here knows about a renderer, a map or a road: a library
//! that only wants trees takes this crate and nothing else (and `worldgen-core`, for
//! its seeds).

pub mod grow;
pub mod math;
pub mod params;
pub mod species;

pub use grow::{LEAF_BUDGET, Leaf, Segment, Tree, TreeSpec, WOOD_LEVEL, grow, leaf_state};
pub use species::{Bark, Bloom, Habit, LeafForm, SPECIES, Species, by_key};
