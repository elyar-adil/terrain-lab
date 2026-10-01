//! Contracts: the small, stable types and interfaces procedural-world layers
//! exchange.
//!
//! A layer depends on a *contract*, not on another layer. The roads layer asks a
//! [`HeightField`] how steep it is and a [`WaterField`] where the rivers are; it
//! does not know whether they come from this project's eroded terrain, a painted
//! map or a real elevation model. What it produces is a [`RoadTile`]; whatever
//! draws, simulates or exports roads reads that. Replacing a layer means
//! satisfying the same contract.
//!
//! Depends only on `worldgen-core` and the standard library.

pub mod field;
pub mod geom;
pub mod network;
pub mod road;

pub use field::{ConstantUrban, DryLand, FlatGround, HeightField, PolylineRiver, UrbanField, WaterField, WaterHit};
pub use geom::{Polyline, V2, closest_on_segment, segment_intersection, v2};
pub use network::{Conflict, EdgeId, NodeId, NodeKind, RoadEdge, RoadNetwork, RoadNode, RoadTile, Span, SpanKind};
pub use road::{CrossSection, RoadClass, Setting, cross_section, junction_trim_m};
