//! A stateless, seamless, hierarchical road network.
//!
//! See `docs/architecture/world-system.md`. In short: the ground is a loose grid
//! of quadrilaterals; their sides are decided once from their own address; streets
//! cross a quadrilateral between divisions of its sides; every decision is a pure
//! function of an address and of the fields the layer is given, so any window of
//! the network can be built on its own and still join its neighbours exactly.

pub mod config;
pub mod lattice;
pub mod network;
pub mod quad;
pub mod shape;
pub mod tile;
pub mod towns;

use worldgen_core::{Engine, EngineBuilder, Error, Frame, Seed};

pub use config::{Fields, LEVELS, LevelSpec, RoadsConfig};
pub use tile::ROADS;
pub use towns::HashedTowns;

/// An engine with the whole roads stack registered: ask it for [`ROADS`] tiles.
pub fn engine(seed: Seed, frame: Frame, config: RoadsConfig, fields: Fields) -> Result<Engine, Error> {
    EngineBuilder::new(seed, frame)
        .with(lattice::ChordLayer { config: config.clone(), urban: fields.urban.clone() })
        .with(quad::QuadLayer { config: config.clone(), urban: fields.urban.clone() })
        .with(network::CellLayer { config: config.clone(), fields })
        .with(tile::RoadsLayer { config })
        .build()
}
