//! The street surface layer: carriageways, kerbs, sidewalks, medians, lane
//! markings, junction boxes, corner paving, bridges and signals.
//!
//! This is the module the previous port was missing.  Before it, roads were
//! independent ribbons that simply abutted at a node: no junction box, no kerb
//! return, no stop line, no crosswalk band, no lane continuity.  Everything
//! here is derived from [`crate::network::Network`], so the surface a driver
//! sees and the lane a vehicle follows come from one source.
//!
//! # What this layer has to get right to be believed
//!
//! A street is 95% surface, and the surface is read as a set of *materials at
//! known reflectance*, not as shapes.  Four things decide whether it works:
//!
//! * **Albedo.**  Asphalt is 4-12% — a near-black, very slightly cool grey.
//!   Footway concrete is 25-35%, three times lighter.  Retroreflective paint is
//!   55-70% and is the brightest thing in the scene.  The separation between
//!   those three is the reason a road reads as a road.
//! * **Texture.**  Every ground surface here carries **world-space metre UVs**, so
//!   the whole city shares one 4 m texture grid: a road drawn in several pieces
//!   has no seam, a sidewalk meets a corner without a join, and the asphalt
//!   aggregate lands at a physical 1.6 cm per texel.  (The previous version drew
//!   its ribbons without UVs at all, so the asphalt, paving and grass bakes were
//!   never sampled and every ground surface rendered as a flat colour.)
//! * **Cross-fall.**  A crowned carriageway.  A dead-flat plane reads as
//!   unbuilt however good the material is, because the eye reads drainage as
//!   evidence of engineering.
//! * **Wear.**  Manhole covers, concrete utility cuts, chipped paint, kerb ramps
//!   and tactile paving at every crossing.  None of it is expensive; all of it is
//!   what separates a road from a diagram.
//!
//! The design numbers all come from [`crate::spec::JunctionSpec`], ported from the
//! source kernel's `junction-spec.js`; nothing below hard-codes a lane width, a
//! line width or a bar pitch.
//!
//! # Layout
//!
//! The layer is split the way the work splits, so each concern stays readable:
//!
//! * [`surfaces`] — what a street is made of: the crowned carriageway, kerbs
//!   and dropped-kerb ramps, medians, junction boxes and corner paving.
//! * [`markings`] — what is painted on it: lane lines and guide zones, stop
//!   lines, crossings, waiting boxes, drive arrows and approach tapers.
//! * [`bridge`] — the structure that carries it over something else.
//! * [`signals`] — the signal heads the renderer's phase driver recolours.

mod bridge;
mod markings;
mod signals;
mod surfaces;
#[cfg(test)]
mod tests;

pub use markings::junction_lane_guides;
pub use signals::{SignalLamp, SignalRig, signal_axis};

use crate::math::{MAX_RESAMPLE_METRES, Path, Vec2, Vec3, cubic_points};
use crate::mesh::{GroupStyle, MeshBuilder};
use crate::network::{Network, Road};

/// Surface layering against the roadbed datum.  These numbers decide whether the
/// city reads as a solid built surface or as decals floating over terrain, so
/// they live in one place.
pub mod level {
    /// Carriageway gutter, the low point of the cross-fall.
    pub const ROAD: f32 = 0.0;
    /// Planted median soil.
    pub const MEDIAN: f32 = 0.06;
    /// Lane paint.
    pub const PAINT: f32 = 0.014;
    /// Crosswalk and stop-line paint, one step above lane paint so the two
    /// never z-fight where they overlap.
    pub const CROSSWALK: f32 = 0.020;
    /// Approach taper asphalt, a hair above the carriageway it widens.
    pub const TAPER: f32 = 0.008;
    /// Block ground plate, just below the sidewalk so no step shows.
    pub const BLOCK: f32 = 0.138;
    /// Paved parcel and junction corner paving.
    pub const PAVING: f32 = 0.150;
    /// Kerb top / sidewalk surface.
    pub const KERB: f32 = 0.150;
    /// Tactile paving, a raised strip on top of the footway.
    pub const TACTILE: f32 = 0.156;
    /// Manhole cover: above the paint, because a cover is a real object that
    /// interrupts the line it sits on.
    pub const COVER: f32 = 0.022;
}

const WALK_Y: f32 = level::KERB;

/// A crowned carriageway surface.
///
/// CJJ 37 gives a transverse cross-fall (横坡) of 1-2% so the carriageway drains.
/// A dead-flat plane is the single strongest "this is a diagram" cue a road
/// surface can have, because the eye reads drainage as evidence of engineering
/// long before it reads aggregate or paint.
///
/// Two details matter and both are usually got wrong:
///
/// * **The high point is the median edge, not the centreline.**  A divided road
///   has two independent carriageways, each sloping from its own median kerb down
///   to its own kerb; the median itself is the crest.  Crowned from the geometric
///   centre instead, the carriageway is *concave* between the lanes, which is how
///   a road ends up with standing water in the middle of each carriageway.
/// * **The fall is constant, not a parabola.**  A parabolic section has zero
///   cross-fall at the crown and double the design value at the kerb, so a
///   "1.5%" parabola gives 0.1% where the cars run.
///
/// The crown *flattens into the junction box*, so the approach and the box meet
/// without a step at the kerb.
pub struct Carriageway {
    pub path: Path,
    /// Kerb face to kerb face.
    pub half: f32,
    pub length: f32,
    /// Distance from each end over which the crown blends away.
    pub flatten: f32,
    /// Lateral offset of the crest: the median edge, or the centreline.
    pub crest: f32,
    /// Height of the crest above the gutter datum, metres.
    pub rise: f32,
}

impl Carriageway {
    pub fn for_road(road: &Road, spec: &crate::spec::JunctionSpec) -> Self {
        let length = road.carriageway.length();
        let half = road.section.half_width();
        let crest = (road.section.median_metres * 0.5).min(half * 0.5);
        // A 1.2% cross-fall across the carriageway, capped so a 36 m expressway
        // does not become a trough.
        let rise = ((half - crest) * spec.crossfall * 0.6).clamp(0.01, 0.20);
        Self {
            path: road.carriageway.clone(),
            half,
            length,
            flatten: 20.0,
            crest,
            rise,
        }
    }

    /// A crowned surface on an arbitrary path, for connectors and tests.
    pub fn on_path(path: Path, half: f32) -> Self {
        let length = path.length();
        Self {
            path,
            half,
            length,
            flatten: 0.0,
            crest: 0.0,
            rise: (half * 0.012).min(0.20),
        }
    }

    /// How much of the crown survives at `station`, blending to zero at each end.
    pub fn crown_factor(&self, station: f32) -> f32 {
        if self.flatten <= 0.0 {
            return 1.0;
        }
        let t = (station / self.flatten)
            .min((self.length - station) / self.flatten)
            .clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Extra lift a point at `offset` gets from the cross-fall.
    pub fn lift(&self, offset: f32, station: f32) -> f32 {
        if self.rise <= 0.0 {
            return 0.0;
        }
        let span = (self.half - self.crest).max(0.5);
        let across = (offset.abs() - self.crest).max(0.0) / span;
        (self.rise * (1.0 - across.min(1.0))) * self.crown_factor(station)
    }

    /// A point on the crowned surface, `lift` metres above the gutter datum.
    pub fn point(&self, station: f32, offset: f32, lift: f32) -> Vec3 {
        let (position, tangent) = self.path.sample(station);
        let lateral = tangent.left_normal() * offset;
        Vec3::new(
            position.x + lateral.x,
            position.y + lift + self.lift(offset, station),
            position.z + lateral.y,
        )
    }
}

/// Texture coordinates for a ribbon.
#[derive(Clone, Copy)]
pub(super) enum Uvs {
    /// None.  For a material with no map, a UV attribute is eight wasted bytes
    /// per vertex — and the paint layers are the biggest groups in the payload.
    None,
    /// **World-space metres.**  Every ground surface in the city shares one 4 m
    /// texture grid, so a road drawn in several pieces has no seam, a sidewalk
    /// runs into a corner paving without a join, and the aggregate lands at a
    /// physical 1.6 cm per texel.  This is the single highest-value change in
    /// the layer and it costs nothing but the UV attribute.
    World,
    /// `U` across the band in metres, `V` along it in metres from `anchor`.  For
    /// the alpha-cut patterns — a dash rhythm, a crossing's bar pitch — this is
    /// what makes the pattern physically exact at any road width, and anchoring
    /// `V` is what keeps a road drawn in two pieces in phase.
    Along(f32),
}

/// A ribbon on a crowned surface.
#[allow(clippy::too_many_arguments)]
pub(super) fn ribbon(
    builder: &mut MeshBuilder,
    material: &str,
    road: &Carriageway,
    inner: f32,
    outer: f32,
    from: f32,
    to: f32,
    lift: f32,
    color: Option<[f32; 3]>,
    uv: Uvs,
) {
    if road.length <= 1.0e-4 || to <= from {
        return;
    }
    let steps = (((to - from) / MAX_RESAMPLE_METRES).ceil().max(1.0)) as usize;
    let mut previous: Option<(Vec3, Vec3, f32)> = None;
    for step in 0..=steps {
        let station = from + (to - from) * step as f32 / steps as f32;
        let a = road.point(station, inner, lift);
        let b = road.point(station, outer, lift);
        if let Some((pa, pb, previous_station)) = previous {
            match uv {
                Uvs::None => builder.quad(material, pa, pb, b, a, color),
                Uvs::World => builder.quad_uv(
                    material,
                    pa,
                    pb,
                    b,
                    a,
                    [(pa.x, pa.z), (pb.x, pb.z), (b.x, b.z), (a.x, a.z)],
                    color,
                ),
                Uvs::Along(anchor) => {
                    let v0 = station - anchor;
                    let v1 = previous_station - anchor;
                    builder.quad_uv(
                        material,
                        pa,
                        pb,
                        b,
                        a,
                        [
                            (0.0, v1),
                            (outer - inner, v1),
                            (outer - inner, v0),
                            (0.0, v0),
                        ],
                        color,
                    );
                }
            }
        }
        previous = Some((a, b, station));
    }
}

/// Side walls and a bottom slab for a ribbon, giving a surface real thickness.
pub(super) fn sweep(
    builder: &mut MeshBuilder,
    material: &str,
    surface: &Carriageway,
    inner: f32,
    outer: f32,
    from: f32,
    to: f32,
    thickness: f32,
) {
    if thickness <= 0.0 {
        return;
    }
    let steps = (((to - from) / 14.0).ceil().max(1.0)) as usize;
    let mut previous: Option<(Vec3, Vec3)> = None;
    for step in 0..=steps {
        let station = from + (to - from) * step as f32 / steps as f32;
        let (a, b) = (
            surface.point(station, inner, 0.0),
            surface.point(station, outer, 0.0),
        );
        if let Some((pa, pb)) = previous {
            // Two side walls, wound outward on both sides.
            builder.wall(
                material,
                Vec2::new(pa.x, pa.z),
                Vec2::new(pb.x, pb.z),
                pa.y - thickness,
                pa.y,
                None,
            );
            builder.wall(
                material,
                Vec2::new(pb.x, pb.z),
                Vec2::new(pa.x, pa.z),
                pb.y - thickness,
                pb.y,
                None,
            );
        }
        previous = Some((a, b));
    }
}

/// A path parallel to `surface` at a constant lateral offset — the centreline
/// of a crash barrier, a parapet or any other narrow object that rides the
/// carriageway.
pub(super) fn offset_path(surface: &Carriageway, offset: f32) -> Path {
    let points: Vec<Vec3> = (0..=64)
        .map(|step| {
            let station = surface.length * step as f32 / 64.0;
            surface.point(station, offset, 0.0)
        })
        .collect();
    Path::new(points)
}

fn declare(builder: &mut MeshBuilder) {
    let style = GroupStyle {
        cast_shadow: false,
        receive_shadow: true,
        alpha_cutout: false,
        dynamic: false,
    };
    for material in [
        "asphalt",
        "asphalt.cycle",
        "asphalt.pavement",
        "sidewalk",
        "kerb",
        "median.plant",
        "barrier.concrete",
        "rail.steel",
        "marking.white",
        "marking.yellow",
        "marking.crosswalk",
        "marking.dashed-3-5",
        "marking.dashed-6-9",
        "bridge.concrete",
        "bridge.steel",
    ] {
        builder.style(material, style);
    }
    // The two dashed-line variants and the crossing are alpha-cut textures, not
    // opaque paint: their transparent gaps must show the asphalt underneath.
    for material in [
        "marking.crosswalk",
        "marking.dashed-3-5",
        "marking.dashed-6-9",
    ] {
        builder.style(
            material,
            GroupStyle {
                alpha_cutout: true,
                ..style
            },
        );
    }
    builder.style(
        "signal.body",
        GroupStyle {
            cast_shadow: true,
            ..style
        },
    );
}

/// Build the whole street layer into `builder`.
pub fn build(network: &Network, builder: &mut MeshBuilder, seed: u32) -> StreetOutput {
    declare(builder);
    let spec = network.spec;
    for road in &network.roads {
        surfaces::road_surface(road, builder, &spec);
        markings::road_markings(network, road, builder, &spec);
        markings::approach_taper(road, builder, &spec);
        bridge::bridge_structure(road, builder);
    }
    let mut output = StreetOutput::default();
    for junction in &network.junctions {
        surfaces::junction_geometry(junction, builder, &spec);
        surfaces::junction_details(
            junction,
            builder,
            &spec,
            !markings::node_has_waiting_box(network, junction.node),
        );
        if junction.kind == urban::JunctionKind::Roundabout {
            surfaces::roundabout(junction, builder, &spec);
        }
    }
    markings::junction_lane_guides(network, builder);
    output.signals = signals::build_signals(network, builder);
    output.traffic_lights_built = output.signals.len() as u32;
    let _ = seed;
    output
}

/// Everything the street layer produced that is not static geometry.
#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreetOutput {
    pub signals: Vec<SignalRig>,
    pub traffic_lights_built: u32,
}

/// Weld a joint between two road ends: a shared kerb return so a street that
/// bends does not show a seam.  Called by the block pass for degree-2 nodes
/// that survived trimming.
pub fn bend_relief(node: Vec2, dir_in: Vec2, dir_out: Vec2, radius: f32) -> Vec<Vec2> {
    let from = node - dir_in * radius;
    let to = node + dir_out * radius;
    cubic_points(from, node, node, to, 6)
}
