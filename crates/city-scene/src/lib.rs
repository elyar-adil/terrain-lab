//! Renderer-facing city scene derived entirely in Rust.
//!
//! `urban` decides *what* a city is: a road graph, blocks, parcels, buildings,
//! tree seeds.  This crate turns that plan into everything a renderer needs —
//! finished vertex buffers, instanced prototypes, signal states and traffic
//! agents — so the browser side never re-derives geometry and cannot disagree
//! with the simulation about where anything is.

pub mod bake;
pub mod buildings;
pub mod facades;
pub mod furniture;
pub mod leaf_cards;
pub mod math;
pub mod mesh;
pub mod network;
pub mod scene;
pub mod species;
pub mod spec;
pub mod street;
pub mod textures;
pub mod traffic;
pub mod trees;

pub use buildings::build as build_building_layer;
pub use mesh::{GroupStyle, Instance, MeshBuilder, MeshGroup};
pub use network::{Network, derive};
pub use scene::{CityScene, SceneBudget, build_city_scene};
pub use spec::JunctionSpec;
pub use street::{SignalRig, StreetOutput, build as build_street_layer};
pub use textures::BakedTexture;
pub use bake::standard_set as standard_texture_set;
