//! Deterministic procedural primitives shared by the world layers.  The
//! L-System engine and the vegetation prototypes it feeds live here so every
//! consumer — the urban street trees, the regional forests, future crops —
//! interprets the same grammar with the same turtle, instead of each module
//! growing its own ad-hoc generator.

pub mod architecture;
pub mod lsystem;
pub mod materials;
pub mod noise;
pub mod vegetation;
pub mod weathering;

pub use architecture::{FacadeKind, FacadeMetrics, facade_metrics};
pub use lsystem::{LSystem, TurtleParams, interpret};
pub use materials::{BakedTexture, MaterialName, bake, standard_texture_set};
pub use noise::{fbm, hash01, value_noise};
use serde::{Deserialize, Serialize};
pub use vegetation::{
    STANDARD_SPECIES, build_prototype, build_prototype_pair, species_from_name,
    standard_prototype_set,
};

/// Small deterministic RNG (SplitMix64).  Every stochastic procedural system
/// in the project draws from this stream so results stay reproducible on any
/// platform and every caller shares one implementation.
#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed.wrapping_add(0x9e37_79b9_7f4a_7c15),
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    pub fn next_f64(&mut self) -> f64 {
        self.next_u64() as f64 / u64::MAX as f64
    }

    pub fn next_range(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.next_f64()
    }
}

/// A straight, tapered branch segment in local tree space (metres, origin at
/// the root collar, +Y up).  Renderers build their own tessellation from it.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub start: [f32; 3],
    pub end: [f32; 3],
    /// Radius at the start of the segment, metres.
    pub radius_start: f32,
    /// Radius at the end of the segment, metres.
    pub radius_end: f32,
}

/// One foliage cluster: a canopy volume the renderer fills with leaf cards or
/// a blobby canopy mesh depending on LOD.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoliageBlob {
    pub centre: [f32; 3],
    pub radius: f32,
    /// 0 = sparse interior twig, 1 = dense sunlit outer crown.
    pub density: f32,
}

/// A growth rule annotation interpreted between symbols: how strongly the
/// turtle pulls toward light (up) or droops under gravity (down).
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tropism {
    pub up: f32,
}

/// A tree geometry prototype: shared branch/foliage data that instances scale,
/// rotate and tint.  Flattened floats keep the payload compact for IPC.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeGeometry {
    pub segments: Vec<Segment>,
    pub foliage: Vec<FoliageBlob>,
    /// Total height of the prototype in metres; instances divide their target
    /// height by this to obtain the uniform scale.
    pub height_metres: f32,
    /// Radius of the crown bounding sphere, metres (LOD culling on the client).
    pub crown_radius_metres: f32,
}

impl TreeGeometry {
    /// Concise binary-ish encoding used inside JSON payloads: each segment as
    /// seven floats and each blob as five.  Kept here so every emitter and the
    /// renderer agree on the layout in one place.
    pub fn segments_flat(&self) -> Vec<f32> {
        let mut out = Vec::with_capacity(self.segments.len() * 8);
        for segment in &self.segments {
            out.extend_from_slice(&segment.start);
            out.extend_from_slice(&segment.end);
            out.push(segment.radius_start);
            out.push(segment.radius_end);
        }
        out
    }

    pub fn foliage_flat(&self) -> Vec<f32> {
        let mut out = Vec::with_capacity(self.foliage.len() * 5);
        for blob in &self.foliage {
            out.extend_from_slice(&blob.centre);
            out.push(blob.radius);
            out.push(blob.density);
        }
        out
    }
}

/// Instancing-friendly prototype ready for serialisation: the renderer builds
/// one merged mesh per (species, LOD) and instances it per tree.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreePrototype {
    pub species: Species,
    pub lod: Lod,
    pub segments: Vec<f32>,
    pub foliage: Vec<f32>,
    pub height_metres: f32,
    pub crown_radius_metres: f32,
}

/// Level of detail for a prototype.  `Near` carries the full L-System with
/// secondary branches; `Far` keeps the primary skeleton and large blobs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Lod {
    Near,
    Far,
}

/// The tree species the world layers plant, with the growth parameters that
/// make each read correctly at street scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Species {
    /// 樟树 camphor — the workhorse street tree of southern Chinese cities.
    Camphor,
    /// 银杏 ginkgo — avenue tree with a narrow young crown.
    Ginkgo,
    /// 垂柳 weeping willow — riverbank species with drooping shoots.
    Willow,
    /// 雪松 deodar cedar — conifer used in parks and civic grounds.
    Cedar,
    /// 竹 bamboo — courtyard groves.
    Bamboo,
    /// 法国梧桐 london plane — the classic 上海 avenue tree.
    LondonPlane,
}

impl Species {
    pub fn from_name(name: &str) -> Option<Self> {
        species_from_name(name)
    }
}
