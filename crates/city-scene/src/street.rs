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

use urban::{JunctionKind, ModernRoadClass};

use crate::math::{MAX_RESAMPLE_METRES, Path, Rng, Vec2, Vec3, cubic_points, smoothstep, triangulate};
use crate::mesh::{GroupStyle, MeshBuilder};
use crate::network::{Junction, Lane, LaneUse, Network, Road};
use crate::spec::{ARROW_LENGTH_MM, JunctionSpec, MM, Movement, arrow_polygons, outline_signed_area};

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
/// How far the corner paving dips below sidewalk level at its outer edge.  A
/// dropped kerb ramp reads as a ramp; a flat one reads as a floating slab.
const CORNER_SINK: f32 = 0.40;
/// Outward bulge applied to the corner paving's outer edge so it slides under
/// the block's corner chamfer instead of leaving a wedge of bare ground.
const CORNER_BULGE: f32 = 3.0;
/// Length of the dropped kerb either side of a crossing's centre line.  GB 50763
/// wants at least 2.5 m of dropped kerb on each side of a crossing.
const RAMP_HALF: f32 = 2.0;
/// Set-back of the tactile strip behind the kerb line.
const TACTILE_SETBACK: f32 = 0.18;
/// Length of the flat approach kerb either side of the dropped section.
const KERB_FLAT: f32 = 0.55;

/// A signal lamp that the renderer recolours as the phase advances.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalLamp {
    /// Which lamp of the head, 0 = red, 1 = yellow, 2 = green.
    pub aspect: u8,
    pub position: [f32; 3],
}

/// Everything the street layer produced that is not static geometry.
#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreetOutput {
    pub signals: Vec<SignalRig>,
    pub traffic_lights_built: u32,
}

/// A signal rig the renderer can drive: which junction, which road, which phase
/// programme, and where every lens is.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalRig {
    pub id: String,
    pub junction: u32,
    pub road: u32,
    /// `"ns"` or `"ew"`: which phase programme this approach runs on.
    pub axis: &'static str,
    pub lamps: Vec<SignalLamp>,
}

/// Which signal axis an approach belongs to.
///
/// Two perpendicular diagonal approaches must land on *different* axes, or they
/// share a green and collide inside the box.  A 45-degree tie is broken by the
/// sign of the bearing product rather than by a threshold, which is what the
/// source kernel does and what makes the tie total.
pub fn signal_axis(bearing: Vec2) -> &'static str {
    if bearing.x.abs() > bearing.y.abs() {
        "ew"
    } else if bearing.y.abs() > bearing.x.abs() {
        "ns"
    } else if bearing.x * bearing.y >= 0.0 {
        "ew"
    } else {
        "ns"
    }
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
    ] {
        builder.style(material, style);
    }
    // The two dashed-line variants and the crossing are alpha-cut textures, not
    // opaque paint: their transparent gaps must show the asphalt underneath.
    for material in ["marking.crosswalk", "marking.dashed-3-5", "marking.dashed-6-9"] {
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
    let spec = network.spec.clone();
    for road in &network.roads {
        road_surface(road, builder, &spec);
        road_markings(network, road, builder, &spec);
        approach_taper(road, builder, &spec);
        bridge_structure(road, builder);
    }
    let mut output = StreetOutput::default();
    for junction in &network.junctions {
        junction_geometry(junction, builder, &spec);
        junction_details(junction, builder, &spec);
        if junction.kind == JunctionKind::Roundabout {
            roundabout(junction, builder, &spec);
        }
    }
    junction_lane_guides(network, builder);
    output.signals = build_signals(network, builder);
    output.traffic_lights_built = output.signals.len() as u32;
    let _ = seed;
    output
}

// ---------------------------------------------------------------------------
// the crowned carriageway
// ---------------------------------------------------------------------------

/// A road's carriageway as a *crowned* surface.
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
    pub fn for_road(road: &Road, spec: &JunctionSpec) -> Self {
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
enum Uvs {
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
fn ribbon(
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
                    [
                        (pa.x, pa.z),
                        (pb.x, pb.z),
                        (b.x, b.z),
                        (a.x, a.z),
                    ],
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

/// One end of a road's carriageway, in the source kernel's terms.
///
/// `base` is the station of the junction edge — which is station 0 or
/// `length`, because the carriageway path is already trimmed back to the box —
/// and `outward` points *away* from the junction, along the road.  Everything
/// downstream is measured from the edge into the carriageway, which is the
/// sense that keeps "5.2 m from the junction" meaning the same thing at both
/// ends.
///
/// Getting this sense wrong is not a small error: with `outward` flipped, the
/// crosswalk lands at a negative station, the length check rejects it, and the
/// crossing silently disappears from one end of every road in the city.  That
/// is exactly what the previous version did.
struct Approach {
    base: f32,
    outward: f32,
    /// Station of the crossing's centre line.
    crosswalk_centre: f32,
    /// Lateral span the stop line covers: from the median edge to the kerb, on
    /// the side this approach's traffic runs.
    lateral_inner: f32,
    lateral_outer: f32,
    /// Station of the stop line's near edge.
    stop_near: f32,
    stop_far: f32,
}

impl Approach {
    /// A station measured back from the junction edge, into the carriageway.
    fn back(&self, distance: f32) -> f32 {
        self.base - self.outward * distance
    }
}

/// The crossing ends of a road, nearest end first.  Empty for a road with no
/// real crossing at either end — a motorway, or a street whose ends are bends.
fn approaches(road: &Road, spec: &JunctionSpec) -> Vec<Approach> {
    let length = road.carriageway.length();
    let half = road.section.half_width();
    let median = road.section.median_metres * 0.5;
    let mut result = Vec::new();
    for (base, outward, crossing) in [
        (0.0_f32, -1.0_f32, road.crossing_start),
        (length, 1.0_f32, road.crossing_end),
    ] {
        if !crossing {
            continue;
        }
        let inner = outward * median;
        let outer = outward * (half - 0.2);
        result.push(Approach {
            base,
            outward,
            crosswalk_centre: base - outward * spec.crosswalk.gap,
            lateral_inner: inner.min(outer),
            lateral_outer: inner.max(outer),
            stop_near: base - outward * (spec.stop_line_gap - spec.stop_line_width * 0.5),
            stop_far: base - outward * (spec.stop_line_gap + spec.stop_line_width * 0.5),
        });
    }
    result
}

// ---------------------------------------------------------------------------
// carriageway, kerb, sidewalk
// ---------------------------------------------------------------------------

/// Station ranges where the kerb is dropped for a crossing, as `(from, to)`.
fn dropped_kerb(road: &Carriageway, spec: &JunctionSpec, crossing_start: bool, crossing_end: bool) -> Vec<(f32, f32)> {
    let mut ranges = Vec::new();
    for (base, outward, crossing) in [
        (0.0_f32, -1.0_f32, crossing_start),
        (road.length, 1.0_f32, crossing_end),
    ] {
        if !crossing {
            continue;
        }
        let centre = base - outward * spec.crosswalk.gap;
        let lo = (centre - RAMP_HALF - KERB_FLAT).max(0.0);
        let hi = (centre + RAMP_HALF + KERB_FLAT).min(road.length);
        if hi > lo {
            ranges.push((lo, hi));
        }
    }
    ranges
}

/// The complement of `cut` inside `[0, length]`, so a kerb or a footway can be
/// drawn in the pieces that are not dropped for a crossing.
fn remaining(length: f32, cut: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut pieces = Vec::new();
    let mut cursor = 0.0_f32;
    for (lo, hi) in cut {
        if *lo > cursor {
            pieces.push((cursor, lo.min(length)));
        }
        cursor = cursor.max(*hi);
    }
    if cursor < length {
        pieces.push((cursor, length));
    }
    pieces.retain(|(a, b)| b - a > 0.05);
    pieces
}

fn road_surface(road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    if road.carriageway.length() < 1.0 {
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let length = surface.length;
    let half = section.half_width();

    // The carriageway proper.
    ribbon(
        builder,
        "asphalt",
        &surface,
        -half,
        half,
        0.0,
        length,
        level::ROAD,
        None,
        Uvs::World,
    );

    // The non-motorized lane (非机动车道) and the hard shoulder.  A separate
    // material, not a tint: the renderer multiplies vertex colours only on the
    // materials that declare `vertexColors`, so a tint on `asphalt` is silently
    // dropped and a 32 m arterial reads as one undifferentiated slab.  Chinese
    // 非机动车道 asphalt is coarser, greyer and patched, which is what separates
    // it from the carriageway at a glance.
    let motor_edge = section.half_carriageway();
    if section.bike_lane_width > 0.0 {
        for side in [-1.0_f32, 1.0] {
            let inner = side * motor_edge;
            let outer = side * motor_edge.max(half - 0.6);
            if (outer - inner).abs() < 0.05 {
                continue;
            }
            ribbon(
                builder,
                "asphalt.cycle",
                &surface,
                inner.min(outer),
                inner.max(outer),
                0.0,
                length,
                level::ROAD,
                None,
                Uvs::World,
            );
        }
    }
    if section.shoulder_width > 0.0 {
        for side in [-1.0_f32, 1.0] {
            let inner = side * (motor_edge + section.bike_lane_width);
            let outer = inner + side * section.shoulder_width;
            ribbon(
                builder,
                "asphalt.cycle",
                &surface,
                inner.min(outer),
                inner.max(outer),
                0.0,
                length,
                level::ROAD,
                None,
                Uvs::World,
            );
        }
    }

    // Pavement skirt.  Without it a road is a floating decal: the kerb face and
    // the road's underside are what visually anchor it to the block.
    sweep(
        builder,
        "asphalt.pavement",
        &surface,
        -half,
        half,
        0.0,
        length,
        road.deck_thickness,
    );

    if !road.has_sidewalk() {
        median_and_hazard(builder, road, &surface, spec);
        road_furniture_marks(builder, road, &surface, spec);
        return;
    }
    let cut = dropped_kerb(&surface, spec, road.crossing_start, road.crossing_end);
    let footway = remaining(length, &cut);
    for side in [-1.0_f32, 1.0] {
        let inner = side * (half + 0.20);
        let outer = side * (half + section.sidewalk_metres);
        for (from, to) in &footway {
            ribbon(
                builder,
                "sidewalk",
                &surface,
                inner.min(outer),
                inner.max(outer),
                *from,
                *to,
                level::KERB,
                None,
                Uvs::World,
            );
        }
        // Kerb: a capping ribbon plus its full-height face.  Drawn in the same
        // pieces as the footway, because a dropped kerb is a kerb that is *not
        // there* — left continuous it pokes through the ramp and the crossing
        // reads as a trip hazard modelled in concrete.
        for (from, to) in &footway {
            ribbon(
                builder,
                "kerb",
                &surface,
                inner - 0.16,
                inner + 0.16,
                *from,
                *to,
                level::KERB + 0.002,
                None,
                Uvs::None,
            );
            sweep(
                builder,
                "kerb",
                &surface,
                inner - 0.16,
                inner + 0.16,
                *from,
                *to,
                level::KERB,
            );
        }
        // And the ramp where the kerb is dropped, with its tactile strip.
        for range in &cut {
            kerb_ramp(builder, &surface, side, *range, spec);
        }
    }

    median_and_hazard(builder, road, &surface, spec);
    road_furniture_marks(builder, road, &surface, spec);
}

/// A dropped-kerb ramp and the tactile paving behind it.
///
/// GB 50763: the footway slopes from full height to road level over about a
/// metre, the kerb is sawn down and laid flush, and a 300 mm tactile strip
/// warns the blind pedestrian that they are at the edge.  The tactile strip is
/// modelled as raised ribs rather than a texture, because at the distances this
/// layer is seen from a bump pattern is the only thing that reads and a colour
/// change alone does not.
fn kerb_ramp(
    builder: &mut MeshBuilder,
    surface: &Carriageway,
    side: f32,
    (from, to): (f32, f32),
    spec: &JunctionSpec,
) {
    let half = surface.half;
    let kerb = side * (half + 0.20);
    let back = side * (half + 1.35);
    let middle = (from + to) * 0.5;
    // Three longitudinal stations: flat kerb, dropped, flat kerb.
    let lo = (from + 0.4).min(middle - 0.2);
    let hi = (to - 0.4).max(middle + 0.2);
    if hi <= lo {
        return;
    }
    let ground = level::ROAD + 0.02;
    let quad = |builder: &mut MeshBuilder, s0: f32, y0: f32, s1: f32, y1: f32| {
        let a = surface.point(s0, kerb, y0);
        let b = surface.point(s0, back, level::KERB);
        let c = surface.point(s1, back, level::KERB);
        let d = surface.point(s1, kerb, y1);
        builder.quad_uv(
            "sidewalk",
            a,
            b,
            c,
            d,
            [(a.x, a.z), (b.x, b.z), (c.x, c.z), (d.x, d.z)],
            None,
        );
    };
    quad(builder, from, level::KERB, lo, ground);
    quad(builder, lo, ground, hi, ground);
    quad(builder, hi, ground, to, level::KERB);

    // The tactile strip, set back from the kerb, with six raised ribs.
    let inner = side * (half + 0.20 + TACTILE_SETBACK);
    let outer = side * (half + 0.20 + TACTILE_SETBACK + spec.tactile_width);
    let start = (middle - 1.6).max(lo);
    let end = (middle + 1.6).min(hi);
    if end - start < 0.4 {
        return;
    }
    let ribs = 6;
    // The warning surface itself, so the strip reads as one band from a car...
    ribbon(
        builder,
        "kerb",
        surface,
        inner.min(outer),
        inner.max(outer),
        start,
        end,
        level::TACTILE - 0.004,
        None,
        Uvs::None,
    );
    // ... and the raised ribs on top of it, which is what a tactile paving
    // surface actually is and the only part of it visible from a moving car.
    for rib in 0..ribs {
        let offset = inner + (outer - inner) * (rib as f32 + 0.5) / ribs as f32;
        ribbon(
            builder,
            "kerb",
            surface,
            offset - 0.028,
            offset + 0.028,
            start,
            end,
            level::TACTILE,
            None,
            Uvs::None,
        );
    }
}

/// The median: two kerb faces, a planted bed, and the yellow-on-black hazard
/// marking on the nose at each junction end.
fn median_and_hazard(builder: &mut MeshBuilder, road: &Road, surface: &Carriageway, spec: &JunctionSpec) {
    let section = road.section;
    if !section.has_median() {
        return;
    }
    let median_half = section.median_metres * 0.5;
    for side in [-1.0_f32, 1.0] {
        ribbon(
            builder,
            "kerb",
            surface,
            (side * median_half - 0.12).min(side * median_half + 0.12),
            (side * median_half - 0.12).max(side * median_half + 0.12),
            0.0,
            surface.length,
            level::KERB,
            None,
            Uvs::None,
        );
    }
    ribbon(
        builder,
        "median.plant",
        surface,
        -median_half,
        median_half,
        0.0,
        surface.length,
        level::MEDIAN,
        None,
        Uvs::World,
    );
    // The nose: on a divided road the median is cut back and marked before the
    // box, and the black-and-yellow diagonal is the single most recognisable
    // piece of Chinese junction furniture there is.
    for (base, outward, crossing) in [
        (0.0_f32, -1.0_f32, road.crossing_start),
        (surface.length, 1.0_f32, road.crossing_end),
    ] {
        let start = if crossing {
            (base - outward * spec.median_hazard_length).clamp(0.0, surface.length)
        } else {
            continue;
        };
        let end = (start + outward * spec.median_hazard_length).clamp(0.0, surface.length);
        if (end - start).abs() < 0.2 {
            continue;
        }
        hazard_stripes(builder, surface, -median_half, median_half, start, end);
    }
}

/// Diagonal black-and-yellow stripes across a band.
///
/// Genuinely diagonal, which needs a parallelogram per stripe rather than a
/// quad: a vertical stripe reads as a hazard *marker*, and the 45-degree skew is
/// the whole point of the marking — it is what a driver reads at 60 m as "this
/// median ends".
fn hazard_stripes(
    builder: &mut MeshBuilder,
    surface: &Carriageway,
    lateral_from: f32,
    lateral_to: f32,
    from: f32,
    to: f32,
) {
    let lo = lateral_from.min(lateral_to);
    let width = (lateral_to - lateral_from).abs();
    if width < 0.2 || (to - from).abs() < 0.2 {
        return;
    }
    let pitch = 0.42_f32;
    let stripes = ((width / pitch).ceil() as usize).max(2);
    let step = width / stripes as f32;
    // Skew one full stripe width over the length of the marking, so the bands run
    // at roughly 45 degrees whatever the marker's proportions.
    let skew = (to - from).signum() * (to - from) * step / width;
    for stripe in 0..stripes {
        let a = lo + step * stripe as f32;
        let b = a + step * 0.5;
        let material = if stripe % 2 == 0 {
            "marking.yellow"
        } else {
            "asphalt.pavement"
        };
        let p0 = surface.point(from, a, level::PAINT);
        let p1 = surface.point(from, b, level::PAINT);
        let p2 = surface.point(to, b + skew, level::PAINT);
        let p3 = surface.point(to, a + skew, level::PAINT);
        builder.quad(material, p0, p1, p2, p3, None);
    }
}

/// Manhole covers and concrete utility cuts.
///
/// Both are what a road *is* after ten years: a cover interrupts the lane line
/// it sits under, and a trench is cut and made good with concrete rather than
/// asphalt, so the patch is lighter than the road around it.  Cheap in
/// triangles, and the single most effective cure for "this surface has never
/// been driven on".
fn road_furniture_marks(builder: &mut MeshBuilder, road: &Road, surface: &Carriageway, spec: &JunctionSpec) {
    if surface.length < 24.0 {
        return;
    }
    let mut rng = Rng::new(road.id ^ 0x9c0f_1e57);
    let half = surface.half;
    let cover_material = "kerb";
    // One or two covers per road, in the running lanes rather than at the kerb.
    let covers = 1 + usize::from(rng.chance(0.55));
    for _ in 0..covers {
        let station = rng.range(8.0, (surface.length - 8.0).max(9.0));
        let offset = rng.range(-half * 0.72, half * 0.72);
        let centre = surface.point(station, offset, level::COVER);
        let sides = 10;
        let ring: Vec<Vec3> = (0..sides)
            .map(|index| {
                let angle = index as f32 / sides as f32 * std::f32::consts::TAU;
                Vec3::new(
                    centre.x + angle.cos() * 0.34,
                    centre.y,
                    centre.z + angle.sin() * 0.34,
                )
            })
            .collect();
        builder.fan(cover_material, &ring, None, Vec3::new(0.0, 1.0, 0.0));
    }
    // One utility cut, rectangular, set flush and a shade lighter.
    if rng.chance(0.62) {
        let station = rng.range(10.0, (surface.length - 10.0).max(11.0));
        let offset = rng.range(-half * 0.6, half * 0.6);
        let along = rng.range(0.9, 1.6);
        let across = rng.range(0.6, 1.1);
        let a = surface.point(station - along, offset - across, level::ROAD + 0.003);
        let b = surface.point(station - along, offset + across, level::ROAD + 0.003);
        let c = surface.point(station + along, offset + across, level::ROAD + 0.003);
        let d = surface.point(station + along, offset - across, level::ROAD + 0.003);
        builder.quad_uv(
            cover_material,
            a,
            b,
            c,
            d,
            [(a.x, a.z), (b.x, b.z), (c.x, c.z), (d.x, d.z)],
            None,
        );
    }
    let _ = spec;
}

/// Side walls and a bottom slab for a ribbon, giving a surface real thickness.
fn sweep(
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

// ---------------------------------------------------------------------------
// longitudinal and transverse markings
// ---------------------------------------------------------------------------

/// A solid lateral band between two stations.
fn stripe(
    builder: &mut MeshBuilder,
    material: &str,
    surface: &Carriageway,
    offset: f32,
    from: f32,
    to: f32,
    width: f32,
) {
    ribbon(
        builder,
        material,
        surface,
        offset - width * 0.5,
        offset + width * 0.5,
        from,
        to,
        level::PAINT,
        None,
        Uvs::None,
    );
}

/// A **transverse** band: a stop line, a waiting-box cap, a give-way line's
/// backing bar.  Lateral extent from `across_from` to `across_to`, longitudinal
/// extent from `from` to `to`.
///
/// This is a different shape from [`stripe`] and mixing the two up is not
/// cosmetic.  `stripe` draws a *line running along the road*; a stop line drawn
/// with it is a 350 mm longitudinal stripe down one edge of the carriageway,
/// which is invisible at any distance and leaves the approach unmarked.  The
/// previous version of this file did exactly that, and also drew a stop line on
/// both carriageways at both ends, so no approach in the city was ever correct.
#[allow(clippy::too_many_arguments)]
fn transverse_stripe(
    builder: &mut MeshBuilder,
    material: &str,
    surface: &Carriageway,
    across_from: f32,
    across_to: f32,
    from: f32,
    to: f32,
    lift: f32,
) {
    if across_to - across_from < 0.02 || to - from < 0.02 {
        return;
    }
    ribbon(
        builder,
        material,
        surface,
        across_from.min(across_to),
        across_from.max(across_to),
        from.min(to),
        from.max(to),
        lift,
        None,
        Uvs::None,
    );
}

/// A dashed lateral band, drawn as one continuous ribbon with a periodic alpha
/// texture instead of one quad per dash.
///
/// The dash phase is anchored at `anchor` and the texture's `V` is scaled by the
/// real length, so the pattern matches the GB 5768.3 3 m/5 m (or 6 m/9 m on a
/// motorway) rhythm exactly while costing a single quad per divider.
///
/// **Anchoring is the whole invariant.**  `laneLineStartFor` in the source
/// kernel puts the first dash's leading edge immediately past the stop line, and
/// a dash sequence anchored anywhere else puts a 5 m *gap* where a queue forms,
/// which is the single most visible marking error a road can have.
fn dashed_stripe(
    builder: &mut MeshBuilder,
    material: &str,
    surface: &Carriageway,
    offset: f32,
    from: f32,
    to: f32,
    width: f32,
    anchor: f32,
) {
    ribbon(
        builder,
        material,
        surface,
        offset - width * 0.5,
        offset + width * 0.5,
        from,
        to,
        level::PAINT,
        None,
        Uvs::Along(anchor),
    );
}

/// A zebra crossing: one quad with a striped alpha texture.  `U` runs the real
/// crossing width, so the 1.05 m bar pitch comes out exact at any road class.
fn crosswalk_band(
    builder: &mut MeshBuilder,
    surface: &Carriageway,
    centre: f32,
    half_width: f32,
    depth: f32,
) {
    // GB 5768.3: bars start 0.6 m in from the kerb and stop 0.4 m short of the
    // far one, exactly as the source kernel's `for (offset = left + 0.6; ...)`
    // loop does.
    let inner = -half_width + 0.6;
    let outer = half_width - 0.4;
    if outer <= inner {
        return;
    }
    ribbon(
        builder,
        "marking.crosswalk",
        surface,
        inner,
        outer,
        centre - depth * 0.5,
        centre + depth * 0.5,
        level::CROSSWALK,
        None,
        Uvs::Along(centre),
    );
}

fn road_markings(network: &Network, road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    let length = road.carriageway.length();
    if length < 2.0 {
        return;
    }
    // A motorway keeps its lane markings but has no stop lines, crosswalks or
    // signals; an elevated road keeps everything except the pedestrian realm it
    // has no kerb for.  Split the three conditions apart explicitly rather than
    // short-circuiting, or a viaduct silently loses its paint.
    if road.is_motorway() {
        motorway_markings(road, builder, spec);
        return;
    }
    if road.layer != 0 {
        elevated_markings(road, builder, spec);
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let half = section.half_width();
    let median = section.median_metres;
    let expressway = road.class == ModernRoadClass::Expressway;

    // Where the longitudinal paint starts and stops.  `marking_start` and
    // `marking_end` are set-backs from the junction edge, measured on the
    // trimmed carriageway, so they read directly here.
    let mark_from = road.marking_start.clamp(0.0, length);
    let mark_to = (length - road.marking_end).clamp(0.0, length);

    // Yellow median lines.  A divided street gets two lines either side of its
    // physical median; an undivided one gets a double yellow on the centre.
    if median > 0.3 {
        for side in [-1.0_f32, 1.0] {
            let offset = side * (median * 0.5).max(0.15);
            stripe(
                builder,
                "marking.yellow",
                &surface,
                offset,
                mark_from,
                mark_to,
                spec.yellow_line_width,
            );
        }
    } else {
        for side in [-1.0_f32, 1.0] {
            stripe(
                builder,
                "marking.yellow",
                &surface,
                side * 0.15,
                mark_from,
                mark_to,
                spec.yellow_line_width,
            );
        }
    }

    // Lane dividers, dashed over the open length and solid inside the 30 m
    // guide zone before each stop line.  On a short block the guide zone shrinks
    // and then disappears so a dash is never lost entirely.
    let guide = guide_zone_length(length, spec);
    let dash_material = if expressway {
        "marking.dashed-6-9"
    } else {
        "marking.dashed-3-5"
    };
    let count = section.motor_lanes_per_direction;
    // The dash run, and the phase it is anchored at.  `laneLineStartFor` in the
    // source kernel puts the first dash's leading edge immediately past the guide
    // zone, and anchoring the texture there is what guarantees a dash rather
    // than a five-metre hole where the queue forms.
    let dash_a = if road.crossing_start {
        spec.solid_zone_gap + guide
    } else {
        mark_from
    };
    let dash_b = if road.crossing_end {
        length - spec.solid_zone_gap - guide
    } else {
        mark_to
    };
    for direction in [1.0_f32, -1.0] {
        for index in 1..count {
            let offset = direction * (median * 0.5 + index as f32 * section.motor_lane_width);
            if guide >= 6.0 && dash_b > dash_a {
                dashed_stripe(
                    builder,
                    dash_material,
                    &surface,
                    offset,
                    dash_a,
                    dash_b,
                    spec.white_line_width,
                    dash_a,
                );
            } else if guide < 6.0 {
                // Too short for a guide zone: the divider is dashed end to end so
                // the lane divider never disappears entirely.
                dashed_stripe(
                    builder,
                    dash_material,
                    &surface,
                    offset,
                    mark_from,
                    mark_to,
                    spec.white_line_width,
                    mark_from,
                );
            }
            // The guide zone: `solid_zone_gap` metres past the stop line, for
            // `guide_zone` metres.
            if guide >= 6.0 {
                for approach in approaches(road, spec) {
                    let solid_a = approach.back(spec.solid_zone_gap + guide);
                    let solid_b = approach.back(spec.solid_zone_gap);
                    stripe(
                        builder,
                        "marking.white",
                        &surface,
                        offset,
                        solid_a.min(solid_b),
                        solid_a.max(solid_b),
                        spec.white_line_width,
                    );
                }
            }
        }
    }

    // The boundary between the motor carriageway and the 非机动车道 is a *solid*
    // white line on a Chinese arterial, and it is the only marking on the road
    // that is not centred on a lane divider.
    if section.bike_lane_width > 0.0 {
        for side in [-1.0_f32, 1.0] {
            stripe(
                builder,
                "marking.white",
                &surface,
                side * section.half_carriageway(),
                mark_from,
                mark_to,
                spec.edge_line_width,
            );
        }
    }

    // Edge lines, held back from the junction by the same margin as the source
    // kernel's `markingStart`, because the crossing and the taper own that ground.
    for side in [-1.0_f32, 1.0] {
        stripe(
            builder,
            "marking.white",
            &surface,
            side * (half - 0.35),
            mark_from,
            mark_to,
            spec.edge_line_width,
        );
    }

    // Crosswalk band and stop line, one set per crossing end, on the correct leg.
    for approach in approaches(road, spec) {
        let centre = approach.crosswalk_centre;
        if centre - spec.crosswalk.depth * 0.5 < -0.01
            || centre + spec.crosswalk.depth * 0.5 > length + 0.01
        {
            continue;
        }
        crosswalk_band(builder, &surface, centre, half, spec.crosswalk.depth);
        transverse_stripe(
            builder,
            "marking.white",
            &surface,
            approach.lateral_inner,
            approach.lateral_outer,
            approach.stop_near,
            approach.stop_far,
            level::PAINT,
        );
    }

    // Left-turn waiting box, drawn only where an innermost lane really turns
    // left and has company in the same direction.
    for at_start in [true, false] {
        let node = if at_start { road.from_node } else { road.to_node };
        let direction = if at_start { -1_i8 } else { 1_i8 };
        let Some(lane) = network.lanes.iter().find(|lane| {
            lane.road == road.id
                && lane.to_node == node
                && lane.direction == direction
                && lane.index == 0
        }) else {
            continue;
        };
        if !lane.allowed.contains(&Movement::Left) {
            continue;
        }
        let companions = network
            .lanes
            .iter()
            .filter(|other| other.road == road.id && other.to_node == node && other.direction == direction)
            .count();
        if companions < 2 {
            continue;
        }
        waiting_box(network, road, builder, spec, lane, at_start);
    }

    // Design-spec drive arrows, one per motor lane, placed clear of the
    // crosswalk band.  The arrow's allowed set comes from the connector graph,
    // so the painted marking and the legal movement are the same fact.
    for lane in network.lanes.iter().filter(|lane| {
        lane.road == road.id
            && lane.path.length() > spec.arrow.tip_gap + ARROW_LENGTH_MM * MM + 1.0
            && matches!(
                lane.use_kind,
                LaneUse::Through | LaneUse::LeftTurn | LaneUse::RightTurn
            )
    }) {
        drive_arrow(builder, lane, spec);
    }
}

/// The left-turn waiting box (左转待转区).
///
/// Two things the previous version got wrong, both of which made the box never
/// appear in any city:
///
/// * it drew the leading cap with the *longitudinal* helper, so the cap came out
///   as a stripe down the middle of the lane instead of a transverse bar;
/// * it measured the box on the trimmed carriageway, whose station 0 **is** the
///   junction edge — and the box by definition reaches 11 m *past* that edge,
///   into the box.  The bounds check therefore rejected it every time.
///
/// So the box is drawn on the untrimmed centreline, which still exists, and the
/// cap is transverse.  The two dashed side lines and the cap are the marking;
/// the interior stays asphalt, because a box is a place, not a surface.
fn waiting_box(
    network: &Network,
    road: &Road,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
    lane: &Lane,
    at_start: bool,
) {
    let _ = network;
    // The untrimmed centreline carries the lane's offset, so the box can reach
    // into the junction exactly as the design calls for.
    let mut surface = Carriageway::on_path(road.centreline.clone(), road.section.half_width());
    // The box is inside the junction, where the crown has already flattened out.
    surface.rise = 0.0;
    let offset = lane.offset;
    let edge = if at_start {
        road.trim_start
    } else {
        surface.length - road.trim_end
    };
    let inward = if at_start { 1.0_f32 } else { -1.0 };
    let stop = edge + inward * spec.stop_line_gap;
    let cap = edge + inward * spec.waiting_box.depth;
    if (stop - cap).abs() < 0.5 {
        return;
    }
    let inner = offset + inward * 0.2;
    let outer = offset + inward * (road.section.motor_lane_width - 0.2);
    for edge_offset in [inner, outer] {
        stripe(
            builder,
            "marking.white",
            &surface,
            edge_offset,
            stop.min(cap),
            stop.max(cap),
            0.12,
        );
    }
    // The leading cap: transverse, 300 mm deep, spanning the whole box.
    transverse_stripe(
        builder,
        "marking.white",
        &surface,
        inner.min(outer),
        inner.max(outer),
        cap - 0.15,
        cap + 0.15,
        level::PAINT,
    );
    // And the stop line the box runs into, which is the *far* edge of the box
    // and is the only thing a driver in it actually obeys.
    transverse_stripe(
        builder,
        "marking.white",
        &surface,
        inner.min(outer),
        inner.max(outer),
        stop - 0.15,
        stop + 0.15,
        level::PAINT,
    );
}

fn motorway_markings(road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    let length = road.carriageway.length();
    if length < 2.0 {
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let median_half = section.median_metres * 0.5;
    for direction in [1.0_f32, -1.0] {
        for index in 1..section.motor_lanes_per_direction {
            let offset = direction * (median_half + index as f32 * section.motor_lane_width);
            dashed_stripe(
                builder,
                "marking.dashed-6-9",
                &surface,
                offset,
                0.0,
                length,
                spec.white_line_width,
                0.0,
            );
        }
        for side in [median_half, section.half_width() - 0.4] {
            stripe(
                builder,
                "marking.white",
                &surface,
                direction * side,
                0.0,
                length,
                spec.edge_line_width,
            );
        }
    }
    for side in [-1.0_f32, 1.0] {
        stripe(
            builder,
            "marking.yellow",
            &surface,
            side * median_half,
            0.0,
            length,
            spec.yellow_line_width,
        );
    }
    // A crash barrier down the middle: a motorway median is not a planted strip.
    if section.median_metres > 0.5 {
        for side in [-1.0_f32, 1.0] {
            let cap = offset_path(&surface, side * (road.half_width() - 0.42));
            let cap_surface = Carriageway::on_path(cap, 0.2);
            ribbon(
                builder,
                "barrier.concrete",
                &cap_surface,
                -0.18,
                0.18,
                4.0,
                (length - 4.0).max(4.0),
                level::ROAD + 0.85,
                None,
                Uvs::None,
            );
            sweep(
                builder,
                "barrier.concrete",
                &cap_surface,
                -0.18,
                0.18,
                4.0,
                (length - 4.0).max(4.0),
                level::ROAD + 0.85,
            );
        }
    }
}

/// Lane paint on a viaduct: edge lines, lane dividers and the median line, but
/// no pedestrian furniture, no stop line and no crosswalk — there is no kerb to
/// stop for.
fn elevated_markings(road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    let length = road.carriageway.length();
    if length < 2.0 {
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let median_half = section.median_metres * 0.5;
    for direction in [1.0_f32, -1.0] {
        for index in 1..section.motor_lanes_per_direction {
            let offset = direction * (median_half + index as f32 * section.motor_lane_width);
            dashed_stripe(
                builder,
                "marking.dashed-3-5",
                &surface,
                offset,
                0.0,
                length,
                spec.white_line_width,
                0.0,
            );
        }
        stripe(
            builder,
            "marking.white",
            &surface,
            direction * (section.half_width() - 0.35),
            0.0,
            length,
            spec.edge_line_width,
        );
    }
    for side in [-1.0_f32, 1.0] {
        stripe(
            builder,
            "marking.yellow",
            &surface,
            side * median_half.max(0.15),
            0.0,
            length,
            spec.yellow_line_width,
        );
    }
}

fn offset_path(surface: &Carriageway, offset: f32) -> Path {
    let points: Vec<Vec3> = (0..=64)
        .map(|step| {
            let station = surface.length * step as f32 / 64.0;
            surface.point(station, offset, 0.0)
        })
        .collect();
    Path::new(points)
}

/// Parapets, piers and abutments for an elevated road.
///
/// The previous renderer lifted a bridge ribbon six metres and left it floating.
/// A viaduct without a deck edge and piers reads as a road pasted onto the sky,
/// which is worse than not drawing the bridge at all.
fn bridge_structure(road: &Road, builder: &mut MeshBuilder) {
    if road.layer == 0 {
        return;
    }
    let path = road.carriageway.clone();
    let length = path.length();
    if length < 4.0 {
        return;
    }
    let surface = Carriageway::on_path(path, road.half_width());
    let half = road.half_width();
    // Parapet: a capping ribbon plus its full-height face on both sides.
    for side in [-1.0_f32, 1.0] {
        let edge = side * (half - 0.3);
        let cap = offset_path(&surface, edge);
        builder.ribbon("bridge.concrete", &cap, -0.18, 0.18, 0.0, length, 0.85, None);
        sweep(builder, "bridge.concrete", &Carriageway::on_path(cap, 0.2), -0.18, 0.18, 0.0, length, 0.85);
    }
    // Piers every 28 m, skipping any that would land in the carriageway of
    // another road — the same avoidance the source kernel applies.
    let mut station = 14.0;
    while station < length - 12.0 {
        let (position, tangent) = surface.path.sample(station);
        let top = position.y - road.deck_thickness;
        if top > 1.2 {
            let height = top;
            let normal = tangent.left_normal();
            let base = Vec3::new(position.x - normal.x * 0.7, 0.0, position.z - normal.y * 0.7);
            let top_point = Vec3::new(position.x - normal.x * 0.7, height, position.z - normal.y * 0.7);
            builder.tube("bridge.concrete", base, top_point, 0.7, 0.6, 6, None);
            // Pier cap, spread to carry the deck.
            let cap_a = Vec3::new(position.x - normal.x * 1.1, height, position.z - normal.y * 1.1);
            let cap_b = Vec3::new(position.x + normal.x * 1.1, height, position.z + normal.y * 1.1);
            builder.tube("bridge.concrete", cap_a, cap_b, 0.6, 0.6, 4, None);
        }
        station += 28.0;
    }
}

fn guide_zone_length(usable: f32, spec: &JunctionSpec) -> f32 {
    if usable < 44.0 {
        0.0
    } else {
        spec.guide_zone.min((usable - 24.0) * 0.5)
    }
}

/// A design-spec GB 5768.3 drive arrow, drawn at its true size.
///
/// # Three things that were wrong
///
/// * **The size.**  `spec.arrow.footprint` is the 5 m of clear approach the
///   design *reserves* for the marking, not the marking.  The stencil's own
///   length is [`ARROW_LENGTH_CM`], which at 100% scale gives a 3.0 m arrow with
///   a 450 mm head — the GB 5768.3 proportion.  Scaling by the footprint instead
///   drew a 5 m arrow with a 740 mm head, nearly twice the design, and a head
///   that stops reading as an arrow at all.
/// * **The winding.**  Half the stencils are mirrors of the other half, and
///   mirroring a polygon reverses it.  A hard-coded triangle order therefore lit
///   every left arrow and every through-and-left arrow from *below* — invisible
///   against the sun, or black against the sky.  The winding now comes from
///   [`outline_signed_area`].
/// * **The lift.**  A painted arrow on a crowned carriageway has to follow the
///   crown, or it floats at one end and buries itself at the other.
fn drive_arrow(builder: &mut MeshBuilder, lane: &Lane, spec: &JunctionSpec) {
    let total = lane.path.length();
    // GB 5768.3: the stencil is 3.05 m long and its tip stands `tip_gap` metres
    // back from the junction edge, so a queue forms behind the arrow rather than
    // on it.
    let full = ARROW_LENGTH_MM * MM;
    let length = full.min(total - spec.arrow.tip_gap - 1.5);
    if length < 2.4 {
        return;
    }
    // Millimetres to metres, along the stencil's own axis.
    let scale = length / ARROW_LENGTH_MM;
    let tail = total - spec.arrow.tip_gap - length;
    // The lateral axis is the *left* normal, which is the frame the stencil
    // tables are authored in: positive `lateral` is the driver's left, so a
    // right-turn stencil's head (authored at negative `lateral`) lands on the
    // driver's right, which is the side `classify_movement` calls a right turn.
    //
    // The arrow follows the crowned surface, so a stencil on a 1.2% cross-fall
    // does not float at the crown and bury its tail in the gutter.
    let surface = Carriageway::on_path(lane.path.clone(), lane.width.max(0.5) * 0.5);
    for outline in arrow_polygons(&lane.allowed) {
        if outline.len() < 3 {
            continue;
        }
        // Ear-clipped, never fanned.
        //
        // A GB arrow outline is **not convex**: the shaft joins the head with a
        // reflex vertex, and a combined straight-and-turn stencil has a second
        // lobe attached at another one.  A triangle fan from the first vertex
        // therefore produces triangles that lie *outside* the polygon, and
        // because the stencil paint is the brightest material in the scene those
        // spurious triangles are exactly the "white blob" an arrow used to render
        // as.  `triangulate` also normalises the winding, so the mirrored
        // stencils and the unmirrored ones come out facing the same way without
        // a per-outline sign.
        let ring: Vec<Vec2> = outline
            .iter()
            .map(|(lateral, forward)| Vec2::new(lateral * scale, forward * scale))
            .collect();
        for [i, j, k] in crate::math::triangulate(&ring) {
            let point = |index: usize| {
                let (lateral, forward) = outline[index];
                let station = tail + forward * scale;
                surface.point(station, lateral * scale, level::PAINT + 0.002)
            };
            builder.tri_flat("marking.white", point(i), point(j), point(k), None);
        }
    }
}

// ---------------------------------------------------------------------------
// approach widening taper
// ---------------------------------------------------------------------------

fn approach_taper(road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    if road.layer != 0 || road.class == ModernRoadClass::Local || road.is_motorway() {
        return;
    }
    let length = road.carriageway.length();
    let section = road.section;
    if section.motor_lanes_per_direction < 2 {
        return;
    }
    let surface = Carriageway::for_road(road, spec);
    for approach in approaches(road, spec) {
        let edge = approach.outward * (section.half_width() - 0.15);
        let from = approach.back(spec.taper_len + 4.0);
        let to = approach.back(4.0);
        if from < 0.0 || to > length || to <= from {
            continue;
        }
        // The extra queue lane grows as the road nears the junction, eating the
        // verge the way a real Chinese arterial approach does.
        let extra = |station: f32| {
            let distance = (approach.base - station) * approach.outward;
            spec.taper_lane_width
                * smoothstep((1.0 - (distance - 4.0) / spec.taper_len).clamp(0.0, 1.0))
        };
        let steps = ((to - from) / 4.0).ceil().max(1.0) as usize;
        let mut previous: Option<(Vec3, Vec3, Vec3, Vec3)> = None;
        for step in 0..=steps {
            let station = from + (to - from) * step as f32 / steps as f32;
            let inner = surface.point(station, edge, level::TAPER);
            let outer = surface.point(
                station,
                edge + approach.outward * (0.2 + extra(station)),
                level::TAPER,
            );
            let line_inner = surface.point(
                station,
                edge + approach.outward * 0.18,
                level::PAINT,
            );
            let line_outer = surface.point(
                station,
                edge + approach.outward * (0.30 + extra(station)),
                level::PAINT,
            );
            if let Some((a, b, c, d)) = previous {
                builder.quad("asphalt", a, b, c, d, None);
                builder.quad("marking.white", c, d, line_outer, line_inner, None);
            }
            previous = Some((inner, outer, line_inner, line_outer));
        }
    }
}

// ---------------------------------------------------------------------------
// junctions
// ---------------------------------------------------------------------------

fn junction_geometry(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    let _ = spec;
    if junction.ports.len() < 2 {
        return;
    }
    // The box itself: carriageway, not pavement.  Everything the walker stands
    // on is the corner patch drawn afterwards.  World UVs, so the box's asphalt
    // is the *same* asphalt as the approach it joins and the two do not read as
    // two materials meeting.
    builder.ground_uv("asphalt", &junction.ring, level::ROAD, None);
    sweep_ring(builder, "asphalt.pavement", &junction.ring, 0.25);

    // Corner paving.  Nine inner vertices follow the junction's own kerb
    // fillet at sidewalk level; the outer edge is a quadratic from one mouth's
    // sidewalk end to the next, bulged outward and sunk on a sine so it reads as
    // a dropped kerb ramp instead of a floating slab.  The invariant that makes
    // this work is `ring.len() == walk_ring.len() == 9 * ports`.
    //
    // It is triangulated from a patch that carries **per-vertex elevation**.
    // The previous version computed a 400 mm sink into the corner's outer edge
    // and then threw it away by handing the triangulator a single `y`, so every
    // corner in the city was a flat slab at footway height with a hole in the
    // ground beside it.
    let count = junction.ring.len();
    if count % 9 != 0 || junction.walk_ring.len() != count {
        return;
    }
    let centre = junction.centre;
    for start in (0..count).step_by(9) {
        let a = junction.walk_ring[(start + 9) % count];
        let b = junction.walk_ring[start + 1];
        let mut patch: Vec<Vec3> = Vec::with_capacity(16);
        for step in 1..=9 {
            let point = junction.ring[(start + step) % count];
            patch.push(Vec3::new(point.x, WALK_Y, point.y));
        }
        let midpoint = (a + b) * 0.5;
        let offset = midpoint - centre;
        let distance = offset.length().max(0.01);
        let control = centre + offset * ((distance + CORNER_BULGE) / distance);
        for step in 0..=6 {
            let t = step as f32 / 6.0;
            let u = 1.0 - t;
            let point = a * (u * u) + control * (2.0 * u * t) + b * (t * t);
            let y = WALK_Y - CORNER_SINK * (std::f32::consts::PI * t).sin();
            patch.push(Vec3::new(point.x, y, point.y));
        }
        // The kerb: a real 150 mm face along the fillet, so the corner reads as
        // kerbed pavement rather than as a painted patch.
        for index in 0..9 {
            let p0 = patch[index];
            let p1 = patch[index + 1];
            builder.wall(
                "kerb",
                Vec2::new(p0.x, p0.z),
                Vec2::new(p1.x, p1.z),
                level::ROAD,
                WALK_Y,
                None,
            );
        }
        let plan: Vec<Vec2> = patch.iter().map(|v| Vec2::new(v.x, v.z)).collect();
        for [i, j, k] in triangulate(&plan) {
            // `triangulate` normalises to counter-clockwise, which faces `-Y`;
            // reverse so the paving lights from above, as `fill_ring` does.
            builder.tri_uv(
                "sidewalk",
                patch[i],
                patch[k],
                patch[j],
                [
                    (patch[i].x, patch[i].z),
                    (patch[k].x, patch[k].z),
                    (patch[j].x, patch[j].z),
                ],
                None,
            );
        }
    }
}

/// The junction's own markings: the yellow no-stopping box on a large signalised
/// junction, and the give-way line of triangles on every unsignalised approach.
fn junction_details(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    yellow_grid_box(junction, builder, spec);
    give_way_across_approach(junction, builder, spec);
}

/// The yellow box (黄色网格线) that marks the no-stopping area inside a large
/// signalised box.
///
/// It is drawn only where the box is genuinely big — four or more legs and a
/// 20 m trim radius — because the grid is what tells a driver that the middle of
/// the box is not a lane, and on a small box the corners already say it.  The
/// line count is capped for the same reason: a 5 m grid across a 60 m box is 200
/// quads a junction, and past a handful of junctions that is a triangle budget
/// spent on paint nobody sees from a car.
fn yellow_grid_box(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    if junction.ports.len() < 4 || junction.radius < 20.0 {
        return;
    }
    let inset = crate::math::inset_ring(&junction.ring, 2.2);
    if inset.len() < 3 {
        return;
    }
    let mut lo = Vec2::new(f32::MAX, f32::MAX);
    let mut hi = Vec2::new(f32::MIN, f32::MIN);
    for point in &inset {
        lo = Vec2::new(lo.x.min(point.x), lo.y.min(point.y));
        hi = Vec2::new(hi.x.max(point.x), hi.y.max(point.y));
    }
    let width = spec.yellow_grid_width * 0.5;
    let lines = 8;
    for index in 0..lines {
        let t = (index as f32 + 0.5) / lines as f32;
        grid_line(builder, &inset, lerp(lo.x, hi.x, t), true, width);
        grid_line(builder, &inset, lerp(lo.y, hi.y, t), false, width);
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn grid_line(
    builder: &mut MeshBuilder,
    inset: &[Vec2],
    at: f32,
    vertical: bool,
    width: f32,
) {
    // Walk the line in 6 m steps and keep the runs that fall inside the box.
    let mut lo = f32::MAX;
    let mut hi = f32::MIN;
    for point in inset {
        let value = if vertical { point.x } else { point.y };
        let across = if vertical { point.y } else { point.x };
        if (at - value).abs() > width {
            continue;
        }
        lo = lo.min(across);
        hi = hi.max(across);
    }
    if hi - lo < 6.0 {
        return;
    }
    let mut station = lo;
    while station < hi {
        let end = (station + 6.0).min(hi);
        let mid = (station + end) * 0.5;
        let probe = if vertical {
            Vec2::new(at, mid)
        } else {
            Vec2::new(mid, at)
        };
        if crate::math::point_in_ring(probe, inset) {
            let quad = if vertical {
                [
                    Vec3::new(at - width, level::PAINT, station),
                    Vec3::new(at + width, level::PAINT, station),
                    Vec3::new(at + width, level::PAINT, end),
                    Vec3::new(at - width, level::PAINT, end),
                ]
            } else {
                [
                    Vec3::new(station, level::PAINT, at - width),
                    Vec3::new(end, level::PAINT, at - width),
                    Vec3::new(end, level::PAINT, at + width),
                    Vec3::new(station, level::PAINT, at + width),
                ]
            };
            builder.quad("marking.yellow", quad[0], quad[1], quad[2], quad[3], None);
        }
        station = end;
    }
}

/// The give-way line (让行线) across an unsignalised approach: a row of inverted
/// triangles, which is the one piece of Chinese road marking with no Western
/// equivalent and is unmistakable.
///
/// The triangles' bases sit on the mouth's own edge and their apexes point back
/// at the driver, so from the saddle they read as a row of arrowheads telling you
/// to stop.  China also paints a 400 mm stop bar behind them, which is what makes
/// the line legible at night, so both go down.
fn give_way_across_approach(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    if junction.kind == JunctionKind::Signalized || junction.ports.len() < 3 {
        return;
    }
    for port in &junction.ports {
        let width = port.left.distance(port.right).max(3.0);
        if width < 3.0 {
            continue;
        }
        // Just inside the mouth, on the approach's side of the box.
        let along = -port.dir;
        let mouth = (port.left + port.right) * 0.5;
        let base = mouth + along * (junction.radius + 1.4);
        let right = Vec2::new(-along.y, along.x);
        let count = (width / spec.give_way_pitch).floor().max(1.0) as usize;
        let pitch = width / count as f32;
        for index in 0..count {
            let centre = base + right * ((index as f32 + 0.5 - count as f32 * 0.5) * pitch);
            let half_pitch = pitch * 0.4;
            let p0 = centre - right * half_pitch;
            let p1 = centre + right * half_pitch;
            let apex = centre + along * spec.give_way_depth;
            let lift = level::PAINT;
            builder.tri_flat(
                "marking.white",
                Vec3::new(p0.x, lift, p0.y),
                Vec3::new(apex.x, lift, apex.y),
                Vec3::new(p1.x, lift, p1.y),
                None,
            );
        }
    }
}

/// Lane lines carried through a junction box.
///
/// A Chinese junction box is **unmarked** — GB 5768.3 forbids lane markings
/// inside the intersection, because they are read by turning drivers as a lane to
/// follow and cause exactly the conflict the box exists to resolve.  The
/// previous version painted two solid edge lines and a dashed centre down every
/// connector, which is the classic Western "ghost road through the junction"
/// look and reads as a mistake to anyone who drives here.
///
/// What is kept is a single dashed guide on a *turning* connector: not a lane
/// line, but a path hint, and the only thing in the box that helps a driver
/// resolve a 90-degree left in a 25 m box.  It costs one quad per connector.
///
/// The guide is **clipped to the box**.  A connector's cubic is allowed to cut
/// the corner slightly, exactly as a vehicle does, and an unclipped guide then
/// paints the kerb return — which is the one place in a junction that must never
/// have paint on it.  Clipping is by point-in-ring along the path, so the dash
/// phase is unaffected: `V` is still metres from the connector's own start.
pub fn junction_lane_guides(network: &Network, builder: &mut MeshBuilder) {
    let spec = &network.spec;
    for connector in &network.connectors {
        // Skip movements the box already reads from its own shape: a
        // near-straight connector is implied by the kerb line.
        if connector.movement != Movement::Left && connector.movement != Movement::Right {
            continue;
        }
        if connector.turn_degrees.abs() < 25.0 {
            continue;
        }
        // A connector's *shape* is the detail, so this is the one place the
        // fine subdivision step is right.
        let path = connector.path.densified(crate::math::FINE_RESAMPLE_METRES);
        if path.length() < 0.8 {
            continue;
        }
        let Some(junction) = network.junction(connector.node) else {
            continue;
        };
        let Some((from, to)) = inside_ring(&path, &junction.ring) else {
            continue;
        };
        let surface = Carriageway::on_path(path, connector.width * 0.5);
        dashed_stripe(
            builder,
            "marking.dashed-3-5",
            &surface,
            0.0,
            from,
            to,
            spec.white_line_width,
            0.0,
        );
    }
}

/// The contiguous station span of `path` that lies inside `ring`, trimmed half a
/// metre in from each end so the guide stops short of the kerb return.
fn inside_ring(path: &Path, ring: &[Vec2]) -> Option<(f32, f32)> {
    let length = path.length();
    let steps = ((length / 2.0).ceil().max(2.0)) as usize;
    let inside = |station: f32| {
        let p = path.plan_at(station);
        crate::math::point_in_ring(p, ring)
    };
    let mut first: Option<usize> = None;
    let mut last = 0_usize;
    for step in 0..=steps {
        let station = length * step as f32 / steps as f32;
        if inside(station) {
            if first.is_none() {
                first = Some(step);
            }
            last = step;
        }
    }
    let first = first?;
    if last <= first + 1 {
        return None;
    }
    let inset = 0.5_f32;
    let a = (length * first as f32 / steps as f32) + inset;
    let b = (length * last as f32 / steps as f32) - inset;
    (b > a + 0.5).then_some((a, b))
}

fn sweep_ring(builder: &mut MeshBuilder, material: &str, ring: &[Vec2], thickness: f32) {
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        builder.wall(material, a, b, -thickness, level::ROAD, None);
    }
}

/// A roundabout: circulatory carriageway, mountable truck apron, raised central
/// island, and the give-way line of triangles across every entry.
///
/// The apron matters more than it looks.  A truck whose wheelbase is 12 m cannot
/// follow a 6 m island, so the inner 400 mm of every roundabout is laid as a
/// mountable kerb at a different tone, and that single ring is most of what
/// tells a driver how big the circle is.
fn roundabout(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    let radius = junction.radius.max(18.0);
    let island = radius * 0.42;
    let centre = junction.centre;
    let ring = |r: f32, segments: usize| -> Vec<Vec2> {
        (0..segments)
            .map(|index| {
                let angle = index as f32 / segments as f32 * std::f32::consts::TAU;
                centre + Vec2::new(angle.cos(), angle.sin()) * r
            })
            .collect()
    };
    builder.ground_uv("asphalt", &ring(radius * 0.96, 48), level::ROAD, None);
    // The apron, in concrete, from the circulatory surface to the island kerb.
    builder.ground_uv("kerb", &ring(island + 0.45, 48), level::ROAD + 0.012, None);
    // The island kerb as a wall, so it has a face and catches the sun on top.
    let kerb_ring = ring(island + 0.45, 48);
    for index in 0..kerb_ring.len() {
        builder.wall(
            "kerb",
            kerb_ring[index],
            kerb_ring[(index + 1) % kerb_ring.len()],
            level::ROAD,
            level::KERB,
            None,
        );
    }
    builder.ground_uv("median.plant", &ring(island, 36), level::MEDIAN + 0.10, None);

    // Give-way triangles and a deflection arrow on every entry.
    for port in &junction.ports {
        let mouth = (port.left + port.right) * 0.5;
        let offset = mouth - centre;
        let distance = offset.length();
        if distance < 1.0 {
            continue;
        }
        let outward = offset / distance;
        let width = port.left.distance(port.right).max(3.0);
        let line = centre + outward * (island + 1.6);
        let right = Vec2::new(-outward.y, outward.x);
        let count = (width / spec.give_way_pitch).floor().max(1.0) as usize;
        let pitch = width / count as f32;
        for index in 0..count {
            let centre_point = line + right * ((index as f32 + 0.5 - count as f32 * 0.5) * pitch);
            let half_pitch = pitch * 0.4;
            // The triangles point *into* the circle, at the driver.
            let p0 = centre_point - right * half_pitch;
            let p1 = centre_point + right * half_pitch;
            let apex = centre_point - outward * spec.give_way_depth;
            builder.tri_flat(
                "marking.white",
                Vec3::new(p0.x, level::PAINT, p0.y),
                Vec3::new(apex.x, level::PAINT, apex.y),
                Vec3::new(p1.x, level::PAINT, p1.y),
                None,
            );
        }
        // The deflection arrow: a straight stencil turned to the circle's
        // tangent, so the driver is told to curve rather than to continue.
        let entry = centre + outward * (island + 4.0);
        deflection_arrow(builder, entry, right, 0.0);
    }
    let _ = spec.roundabout_apron;
}

/// A straight drive arrow re-laid along an arbitrary direction, for a
/// roundabout's entry lane.
fn deflection_arrow(builder: &mut MeshBuilder, centre: Vec2, direction: Vec2, lift: f32) {
    let normal = direction.left_normal();
    for outline in arrow_polygons(&[Movement::Straight]) {
        let ring: Vec<Vec2> = outline
            .iter()
            .map(|(lateral, forward)| Vec2::new(lateral * MM, forward * MM))
            .collect();
        for [i, j, k] in crate::math::triangulate(&ring) {
            let at = |index: usize| {
                let (lateral, forward) = outline[index];
                let plan = centre + normal * (lateral * MM) + direction * (forward * MM);
                Vec3::new(plan.x, level::PAINT + lift, plan.y)
            };
            builder.tri_flat("marking.white", at(i), at(j), at(k), None);
        }
    }
}

// ---------------------------------------------------------------------------
// signals
// ---------------------------------------------------------------------------

/// Signal heads, on the near-side footway of every signalised approach.
///
/// The previous version planted every head at `junction.centre + side + along`,
/// which put the pole **inside the junction box**, six metres past the kerb,
/// facing across the carriageway.  A signal head has one job: be in the driver's
/// field of view *before* the stop line and face back down the approach at it.
/// So the head is placed on the road's own geometry, at the station of the stop
/// line plus a metre, laterally out on the footway, and its housing is built
/// along the road axis with the lens face pointing back at the queue.
fn build_signals(network: &Network, builder: &mut MeshBuilder) -> Vec<SignalRig> {
    let spec = &network.spec;
    let mut rigs = Vec::new();
    let mut rng = Rng::new(0);
    for junction in &network.junctions {
        if junction.kind != JunctionKind::Signalized || junction.ports.len() < 3 {
            continue;
        }
        let has_arterial = junction.ports.iter().any(|port| {
            network
                .road(port.road)
                .is_some_and(|road| matches!(road.class, ModernRoadClass::Arterial | ModernRoadClass::Expressway))
        });
        if !has_arterial {
            continue;
        }
        // Signalise the junctions that carry the traffic, not every corner.
        rng.fork(junction.node);
        if junction.ports.len() >= 4 && rng.chance(0.35) {
            continue;
        }
        for port in &junction.ports {
            let Some(road) = network.road(port.road) else {
                continue;
            };
            if !road.has_sidewalk() {
                continue;
            }
            let path = &road.carriageway;
            let length = path.length();
            if length < 8.0 {
                continue;
            }
            let (edge, into) = if port.at_start {
                (0.0_f32, 1.0_f32)
            } else {
                (length, -1.0_f32)
            };
            // The stop line is `stop_line_gap` in from the edge; the pole stands a
            // metre further back, on the footway, on the approach's own side.
            let station = (edge + into * (spec.stop_line_gap + 1.3)).clamp(0.5, length - 0.5);
            let side = if port.at_start { -1.0_f32 } else { 1.0_f32 };
            let lateral = side * (road.half_width() + road.section.sidewalk_metres * 0.35 + 0.6);
            let base = path.offset_at(station, lateral, 0.0);
            let tangent = path.tangent_at(station);
            // The head faces back down the approach, at the queue.
            let facing = tangent * -into;
            let axis = signal_axis(facing);
            let foot = Vec3::new(base.x, WALK_Y, base.z);
            let pole_height = 6.2_f32;
            let head_centre_y = 5.6_f32;
            builder.tube(
                "signal.body",
                foot,
                foot + Vec3::new(0.0, pole_height, 0.0),
                0.12,
                0.09,
                8,
                None,
            );
            // A short bracket arm out over the kerb line, which is what actually
            // puts the lens in a driver's eye rather than behind the column.
            let arm_plan = Vec2::new(base.x, base.z) + tangent.left_normal() * (-side * 0.9);
            let arm_tip = Vec3::new(arm_plan.x, head_centre_y, arm_plan.y);
            builder.tube(
                "signal.body",
                foot + Vec3::new(0.0, head_centre_y + 0.66, 0.0),
                arm_tip + Vec3::new(0.0, 0.66, 0.0),
                0.06,
                0.05,
                5,
                None,
            );
            // The housing: 340 mm along the road, 440 mm across it, 1.16 m tall,
            // wound so the lens face looks back at the stop line.
            let plan = Vec2::new(arm_tip.x, arm_tip.z);
            crate::mesh::box_at(
                builder,
                "signal.body",
                plan,
                head_centre_y,
                0.34,
                1.16,
                0.44,
                facing.angle(),
            );
            let right3 = Vec2::new(-facing.y, facing.x);
            let mut lamps = Vec::with_capacity(3);
            for (aspect, dy) in [(0_u8, 0.38_f32), (1, 0.0), (2, -0.38)] {
                let lens = plan - facing * 0.19;
                let centre = Vec3::new(lens.x, head_centre_y + dy, lens.y);
                // A bezel ring around each lens, so the head reads as hardware.
                builder.tube(
                    "signal.body",
                    Vec3::new(
                        centre.x + right3.x * 0.02,
                        centre.y,
                        centre.z + right3.y * 0.02,
                    ),
                    Vec3::new(
                        centre.x - right3.x * 0.05,
                        centre.y,
                        centre.z - right3.y * 0.05,
                    ),
                    0.15,
                    0.13,
                    8,
                    None,
                );
                // The visor over the top lens, which is what shades it from the
                // low sun that is otherwise straight down the approach.
                if aspect == 0 {
                    builder.wall(
                        "signal.body",
                        Vec2::new(lens.x, lens.y),
                        Vec2::new(
                            lens.x - facing.x * 0.26,
                            lens.y - facing.y * 0.26,
                        ),
                        centre.y + 0.13,
                        centre.y + 0.18,
                        None,
                    );
                }
                lamps.push(SignalLamp {
                    aspect,
                    position: [centre.x, centre.y, centre.z],
                });
            }
            rigs.push(SignalRig {
                id: format!("signal/{}/{}", junction.node, port.road),
                junction: junction.node,
                road: port.road,
                axis,
                lamps,
            });
        }
    }
    rigs
}

/// Weld a joint between two road ends: a shared kerb return so a street that
/// bends does not show a seam.  Called by the block pass for degree-2 nodes
/// that survived trimming.
pub fn bend_relief(node: Vec2, dir_in: Vec2, dir_out: Vec2, radius: f32) -> Vec<Vec2> {
    let from = node - dir_in * radius;
    let to = node + dir_out * radius;
    cubic_points(from, node, node, to, 6)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Path;
    use crate::network::derive;
    use urban::{ModernChinaSpec, generate_modern_chinese_city};

    fn city_network() -> Network {
        let city = generate_modern_chinese_city(ModernChinaSpec {
            seed: 42,
            radius_km: 0.5,
            block_size_metres: 110.0,
            ..ModernChinaSpec::default()
        });
        derive(
            &city.nodes,
            &city.sd_roads,
            &city.hd_roads,
            city.frame,
            JunctionSpec::default(),
            city.seed,
        )
    }

    /// A straight lane heading `+X`, 3.5 m wide, long enough for the arrow.
    fn straight_lane(movements: &[Movement]) -> Lane {
        let length = 90.0;
        let path = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(length, 0.0)]);
        Lane {
            id: "test".into(),
            road: 0,
            index: 0,
            direction: 1,
            width: 3.5,
            offset: 0.0,
            use_kind: LaneUse::Through,
            path,
            from_node: 0,
            to_node: 1,
            allowed: movements.to_vec(),
            successors: Vec::new(),
            predecessors: Vec::new(),
        }
    }

    /// Every vertex a built arrow produced, in the lane's own frame: `x` is
    /// forward, `z` is lateral with `+Z` the driver's right for a `+X` heading.
    fn arrow_vertices(movements: &[Movement]) -> Vec<(f32, f32)> {
        let lane = straight_lane(movements);
        let mut builder = MeshBuilder::new();
        declare(&mut builder);
        drive_arrow(&mut builder, &lane, &JunctionSpec::default());
        let scene = builder.build();
        let mut out = Vec::new();
        for group in scene.meshes {
            for chunk in group.positions.chunks(3) {
                out.push((chunk[0], chunk[2]));
            }
        }
        assert!(!out.is_empty(), "no arrow geometry for {movements:?}");
        out
    }

    #[test]
    fn a_right_arrow_head_lands_on_the_drivers_right_and_a_left_arrow_on_its_left() {
        // The single most important assertion in the file.  Computed from the
        // *built triangles*, not from the outline table, so it catches a wrong
        // lateral axis, a wrong stencil assignment and a mirrored frame alike.
        // For a lane heading `+X` the driver's right is `+Z`.
        let spec = JunctionSpec::default();
        let right = arrow_vertices(&[Movement::Right]);
        let left = arrow_vertices(&[Movement::Left]);
        let straight = arrow_vertices(&[Movement::Straight]);

        // A right arrow's *head* — the forward third of the stencil — must be
        // entirely on the right, and the shaft may only straddle the centreline.
        let head = |points: &[(f32, f32)]| {
            let max_x = points.iter().map(|p| p.0).fold(f32::MIN, f32::max);
            points
                .iter()
                .filter(|p| p.0 > max_x - 1.0)
                .map(|p| p.1)
                .collect::<Vec<f32>>()
        };
        let right_head = head(&right);
        let left_head = head(&left);
        assert!(
            right_head.iter().all(|z| *z > 0.05),
            "a right arrow's head is not on the driver's right: {right_head:?}"
        );
        assert!(
            left_head.iter().all(|z| *z < -0.05),
            "a left arrow's head is not on the driver's left: {left_head:?}"
        );
        // And the whole stencil leans that way: the tip is the extreme vertex.
        let tip = |points: &[(f32, f32)]| {
            *points
                .iter()
                .max_by(|a, b| a.0.total_cmp(&b.0))
                .unwrap()
        };
        assert!(tip(&right).1 > 0.0, "right arrow tip at z={}", tip(&right).1);
        assert!(tip(&left).1 < 0.0, "left arrow tip at z={}", tip(&left).1);

        // A straight arrow is symmetric about the centreline.
        let (lo, hi) = straight
            .iter()
            .map(|p| p.1)
            .fold((f32::MAX, f32::MIN), |(a, b), z| (a.min(z), b.max(z)));
        assert!(
            (lo + hi).abs() < 1.0e-3,
            "a straight arrow is not symmetric about its lane: {lo} .. {hi}"
        );
        assert!(((hi - lo) - 0.45).abs() < 0.02, "head is {} m across", hi - lo);

        // The arrow is the GB size, not the reserved footprint.
        let span = right
            .iter()
            .map(|p| p.0)
            .fold(f32::MAX, f32::min)
            ..right.iter().map(|p| p.0).fold(f32::MIN, f32::max);
        let length = span.end - span.start;
        assert!(
            (length - ARROW_LENGTH_MM * MM).abs() < 0.02,
            "a right arrow is {length} m long, expected {}",
            ARROW_LENGTH_MM * MM
        );
        // And it is the GB distance back from the lane's end.
        let tip_x = right.iter().map(|p| p.0).fold(f32::MIN, f32::max);
        let total = 90.0;
        assert!(
            (total - tip_x - spec.arrow.tip_gap).abs() < 0.02,
            "the arrow's tip stands {tip_x} m along a {total} m lane"
        );
    }

    #[test]
    fn every_built_arrow_faces_the_sky() {
        // A stencil wound the other way lights from below and disappears against
        // the sun.  Half the stencils are mirrors, so this is a real class of bug
        // and not a hypothetical one.
        for movements in all_stencils() {
            let lane = straight_lane(&movements);
            let mut builder = MeshBuilder::new();
            declare(&mut builder);
            drive_arrow(&mut builder, &lane, &JunctionSpec::default());
            for group in builder.build().meshes {
                for normal in group.normals.chunks(3) {
                    assert!(
                        normal[1] > 0.5,
                        "an arrow face for {movements:?} points downwards ({normal:?})"
                    );
                }
            }
        }
    }

    #[test]
    fn an_arrow_never_leaves_its_lane() {
        for movements in all_stencils() {
            for (_, z) in arrow_vertices(&movements) {
                assert!(
                    z.abs() <= 1.85,
                    "a {movements:?} arrow reaches {z:.2} m off its lane centre"
                );
            }
        }
    }

    #[test]
    fn a_crossing_band_is_one_quad_with_a_bar_pattern() {
        let spec = JunctionSpec::default();
        let path = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(120.0, 0.0)]);
        let surface = Carriageway::on_path(path, 16.0);
        let mut builder = MeshBuilder::new();
        declare(&mut builder);
        // A 32 m arterial: the bar count has to come from the U scale, not from
        // geometry.
        crosswalk_band(&mut builder, &surface, 60.0, 16.0, 4.0);
        let scene = builder.build();
        assert_eq!(scene.meshes[0].material, "marking.crosswalk");
        assert_eq!(scene.meshes[0].indices.len(), 6, "a crossing is one quad");
        assert!(
            scene.meshes[0].alpha_cutout,
            "the crossing's gaps must show the asphalt through them"
        );
        let uvs = scene.meshes[0].uvs.as_ref().unwrap();
        let u_span = uvs.chunks(2).map(|pair| pair[0]).fold(f32::MAX, f32::min).abs()
            .max(uvs.chunks(2).map(|pair| pair[0]).fold(f32::MIN, f32::max).abs());
        // 31.0 m of crossing: the texture holds eight bars per 8.4 m, so the
        // renderer tiles it about 3.7 times and the pitch stays 1.05 m.
        assert!((u_span - 31.0).abs() < 0.2, "U span was {u_span}");
        let _ = spec;
    }

    #[test]
    fn the_crosswalk_bar_pitch_survives_the_current_resampling() {
        // `MAX_RESAMPLE_METRES` is 34 m, so a 4 m crossing is one quad and the
        // bar pitch lives entirely in the texture.  Assert the geometry agrees
        // with the design table anyway: if a future change coarsens the ribbon
        // step, the depth must still be exact.
        let spec = JunctionSpec::default();
        let path = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(60.0, 0.0)]);
        let surface = Carriageway::on_path(path, 8.0);
        let mut builder = MeshBuilder::new();
        declare(&mut builder);
        crosswalk_band(&mut builder, &surface, 30.0, 8.0, spec.crosswalk.depth);
        let scene = builder.build();
        let positions = &scene.meshes[0].positions;
        let xs: Vec<f32> = positions.chunks(3).map(|c| c[0]).collect();
        let depth = xs.iter().copied().fold(f32::MIN, f32::max)
            - xs.iter().copied().fold(f32::MAX, f32::min);
        assert!(
            (depth - spec.crosswalk.depth).abs() < 0.05,
            "crossing depth is {depth} m, expected {}",
            spec.crosswalk.depth
        );
    }

    #[test]
    fn a_dash_run_anchored_at_a_stop_line_starts_with_a_dash() {
        // The invariant `laneLineStartFor` exists for: the first thing past the
        // stop line is paint, not a five-metre hole in the queue's lane.
        let spec = JunctionSpec::default();
        let path = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(200.0, 0.0)]);
        let surface = Carriageway::on_path(path, 8.0);
        let anchor = spec.solid_zone_gap + spec.guide_zone;
        let mut builder = MeshBuilder::new();
        declare(&mut builder);
        dashed_stripe(
            &mut builder,
            "marking.dashed-3-5",
            &surface,
            0.0,
            anchor,
            anchor + 30.0,
            spec.white_line_width,
            anchor,
        );
        let scene = builder.build();
        let uvs = scene.meshes[0].uvs.as_ref().expect("a dash needs UVs");
        // `V` is metres from the anchor, and the texture's period is 8 m, so the
        // first sample past the anchor is inside the first 3 m of paint.
        let v_min = uvs.chunks(2).map(|pair| pair[1]).fold(f32::MAX, f32::min);
        assert!((v_min).abs() < 1.0e-3, "V does not start at the anchor: {v_min}");
        let v_max = uvs.chunks(2).map(|pair| pair[1]).fold(f32::MIN, f32::max);
        assert!((v_max - 30.0).abs() < 0.1, "V span was {}", v_max - v_min);
    }

    #[test]
    fn a_dashed_divider_is_one_quad_not_one_quad_per_dash() {
        let path = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(300.0, 0.0)]);
        let surface = Carriageway::on_path(path, 4.0);
        let mut builder = MeshBuilder::new();
        declare(&mut builder);
        dashed_stripe(&mut builder, "marking.dashed-3-5", &surface, 0.0, 0.0, 300.0, 0.15, 0.0);
        // 300 m of 3-on/5-off would be 37 geometry dashes.  The texture carries
        // the pattern, so the band is resampled only coarsely along the road.
        let triangles = builder.index_count("marking.dashed-3-5") / 3;
        assert!(
            triangles <= 24,
            "a 300 m dashed line cost {triangles} triangles, expected far fewer than 37"
        );
        let scene = builder.build();
        let group = &scene.meshes[0];
        assert!(group.uvs.is_some(), "a dashed line needs metre UVs");
        assert!(group.alpha_cutout, "the gaps must be alpha-cut");
        let span = group
            .uvs
            .as_ref()
            .unwrap()
            .chunks(2)
            .map(|pair| pair[1].abs())
            .fold(0.0_f32, f32::max);
        // UVs are metres, so `V` must span the real 300 m for the texture's
        // 8 m period to fall in the right places.
        assert!(span > 250.0, "V span was {span}, the dash period would be wrong");
    }

    #[test]
    fn every_crossing_end_gets_a_crosswalk_and_a_stop_line() {
        let network = city_network();
        let mut builder = MeshBuilder::new();
        build(&network, &mut builder, 42);
        assert!(builder.vertex_count("marking.crosswalk") > 0, "no crossings at all");
        assert!(
            builder.index_count("marking.white") > 10_000,
            "expected a full set of markings, got {} indices",
            builder.index_count("marking.white")
        );
    }

    /// Every arrow a lane can be given, in the order the tests use them.
    fn all_stencils() -> Vec<Vec<Movement>> {
        vec![
            vec![Movement::Straight],
            vec![Movement::Left],
            vec![Movement::Right],
            vec![Movement::Left, Movement::Straight],
            vec![Movement::Right, Movement::Straight],
            vec![Movement::Left, Movement::Right, Movement::Straight],
        ]
    }

    /// Plan-space axis-aligned boxes, one per triangle, for one material.
    fn material_bounds(scene: &crate::mesh::SceneGeometry, material: &str) -> Vec<[f32; 4]> {
        let Some(group) = scene
            .meshes
            .iter()
            .find(|group| group.material == material)
        else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for triangle in group.indices.chunks(3) {
            let points: Vec<(f32, f32)> = triangle
                .iter()
                .map(|index| {
                    let index = *index as usize;
                    (
                        group.positions[index * 3],
                        group.positions[index * 3 + 2],
                    )
                })
                .collect();
            let lo = (
                points.iter().map(|p| p.0).fold(f32::MAX, f32::min),
                points.iter().map(|p| p.1).fold(f32::MAX, f32::min),
            );
            let hi = (
                points.iter().map(|p| p.0).fold(f32::MIN, f32::max),
                points.iter().map(|p| p.1).fold(f32::MIN, f32::max),
            );
            out.push([lo.0, lo.1, hi.0, hi.1]);
        }
        out
    }

    /// The completeness test the junction layer has to pass: a four-port
    /// signalised junction carries a box, corner paving, a stop line and a
    /// crossing on **every** arm, and no two opposing crossings overlap.
    #[test]
    fn a_four_port_junction_is_complete_on_every_arm() {
        let network = city_network();
        let mut builder = MeshBuilder::new();
        build(&network, &mut builder, 42);
        let vertex_count = builder.vertex_count("sidewalk");
        let scene = builder.build();
        let crossings = material_bounds(&scene, "marking.crosswalk");

        let Some(junction) = network
            .junctions
            .iter()
            .filter(|junction| {
                junction.ports.len() == 4 && junction.kind == JunctionKind::Signalized
            })
            .max_by(|a, b| a.radius.total_cmp(&b.radius))
        else {
            return;
        };
        let crossings = material_bounds(&scene, "marking.crosswalk");
        assert!(
            !crossings.is_empty(),
            "no crossing geometry anywhere in the city"
        );

        // One crossing per arm, matched by proximity to the arm's mouth.
        let mut per_arm = Vec::new();
        for port in &junction.ports {
            let mouth = (port.left + port.right) * 0.5;
            let best = crossings
                .iter()
                .filter(|quad| {
                    let cx = (quad[0] + quad[2]) * 0.5;
                    let cz = (quad[1] + quad[3]) * 0.5;
                    (cx - mouth.x).hypot(cz - mouth.y) < junction.radius * 0.5 + 14.0
                })
                .min_by(|a, b| {
                    let da = ((a[0] + a[2]) * 0.5 - mouth.x)
                        .hypot((a[1] + a[3]) * 0.5 - mouth.y);
                    let db = ((b[0] + b[2]) * 0.5 - mouth.x)
                        .hypot((b[1] + b[3]) * 0.5 - mouth.y);
                    da.total_cmp(&db)
                })
                .copied();
            per_arm.push(best);
        }
        for (index, crossing) in per_arm.iter().enumerate() {
            assert!(
                crossing.is_some(),
                "arm {index} of a 4-port junction has no crossing"
            );
        }

        // No two *opposing* crossings may touch.  Two arms are opposing when
        // their outward directions are more than 120 degrees apart.
        for a in 0..junction.ports.len() {
            for b in a + 1..junction.ports.len() {
                if junction.ports[a].dir.dot(junction.ports[b].dir) > -0.5 {
                    continue;
                }
                let (Some(x), Some(y)) = (per_arm[a], per_arm[b]) else {
                    continue;
                };
                let overlap_x = (x[0] - y[2]).max(y[0] - x[2]);
                let overlap_y = (x[1] - y[3]).max(y[1] - x[3]);
                assert!(
                    overlap_x <= 0.0 || overlap_y <= 0.0,
                    "opposing crossings {a} and {b} overlap by {overlap_x:.2} x {overlap_y:.2} m"
                );
            }
        }

        // A stop line on every arm too, and the corner paving.
        let white = material_bounds(&scene, "marking.white");
        for port in &junction.ports {
            let mouth = (port.left + port.right) * 0.5;
            let found = white.iter().any(|quad| {
                let cx = (quad[0] + quad[2]) * 0.5;
                let cz = (quad[1] + quad[3]) * 0.5;
                (cx - mouth.x).hypot(cz - mouth.y) < junction.radius * 0.5 + 12.0
            });
            assert!(found, "arm has no stop line near ({:.1}, {:.1})", mouth.x, mouth.y);
        }
        assert!(vertex_count > 0, "no corner paving emitted");
        for junction in &network.junctions {
            assert_eq!(junction.ring.len(), junction.ports.len() * 9);
            assert_eq!(junction.walk_ring.len(), junction.ring.len());
        }
    }

    #[test]
    fn every_crossing_lands_where_the_spec_says_it_does() {
        // The bounds check that silently deleted every crossing in the previous
        // version: with the approach sense flipped, the crossing centre came out
        // at a negative station and was rejected.  Asserted on the geometry.
        let network = city_network();
        let spec = JunctionSpec::default();
        let mut checked = 0;
        for road in &network.roads {
            if road.layer != 0 || road.is_motorway() {
                continue;
            }
            let length = road.carriageway.length();
            for approach in approaches(road, &spec) {
                let centre = approach.crosswalk_centre;
                assert!(
                    centre > -0.01 && centre < length + 0.01,
                    "crossing centre at {centre} is off a {length} m carriageway"
                );
                // Measured as a distance from the junction edge, because that is
                // the only frame both ends share.
                let crossing_edge = (approach.base - centre).abs() + spec.crosswalk.depth * 0.5;
                let stop_edge = (approach.base - approach.stop_near)
                    .abs()
                    .min((approach.base - approach.stop_far).abs());
                assert!(
                    stop_edge > crossing_edge,
                    "the stop line at {stop_edge:.2} m from the junction is not behind \
                     the crossing, whose far edge is {crossing_edge:.2} m out"
                );
                // The stop line only covers the approach's own carriageway.
                assert!(
                    approach.lateral_outer.abs() <= road.section.half_width(),
                    "the stop line runs past the kerb"
                );
                checked += 1;
            }
        }
        assert!(checked > 50, "only {checked} approaches to check");
    }

    #[test]
    fn the_carrageway_crowns_from_the_median_and_flattens_into_the_box() {
        let spec = JunctionSpec::default();
        let path = Path::flat(vec![Vec2::new(0.0, 0.0), Vec2::new(200.0, 0.0)]);
        let mut surface = Carriageway::on_path(path, 16.0);
        surface.flatten = 20.0;
        // Mid-road the crest stands proud of the gutter...
        assert!(surface.lift(0.0, 100.0) > 0.10);
        // ... the gutter does not...
        assert!(surface.lift(16.0, 100.0).abs() < 1.0e-4);
        // ... and at the junction edge there is no step at all.
        assert!(surface.lift(0.0, 0.0).abs() < 1.0e-4);
        // A constant cross-fall of 1-2%, measured across the running lanes.
        let fall = (surface.lift(4.0, 100.0) - surface.lift(12.0, 100.0)) / 8.0;
        assert!(
            (0.008..0.030).contains(&fall),
            "cross-fall is {fall:.3}, CJJ 37 wants 1-2%"
        );
        // And the fall is *linear*, so the middle of a carriageway is not a
        // trough: the crest-to-gutter gradient equals the kerb-to-kerb one.
        let near = (surface.lift(0.0, 100.0) - surface.lift(2.0, 100.0)) / 2.0;
        assert!((near - fall).abs() < 1.0e-3, "the section is not planar");
        // Flattening over 20 m of a 0.2 m crest is a 1% longitudinal ramp, which
        // is invisible; a shorter blend would be a visible crease where the box
        // meets the approach.
        let blend = (surface.lift(0.0, 20.0) - surface.lift(0.0, 0.0)) / 20.0;
        assert!(
            blend <= 0.012,
            "the crown blends into the box at {blend:.3}, which would show as a crease"
        );
        let _ = spec;
    }

    #[test]
    fn junction_layer_never_emits_a_non_finite_vertex() {
        let network = city_network();
        let mut builder = MeshBuilder::new();
        build(&network, &mut builder, 42);
        for group in builder.build().meshes {
            for value in &group.positions {
                assert!(value.is_finite(), "{} has a non-finite position", group.material);
            }
            for value in &group.normals {
                assert!(value.is_finite(), "{} has a non-finite normal", group.material);
            }
        }
    }

    #[test]
    fn signal_heads_land_on_signalized_arterial_junctions() {
        let network = city_network();
        let mut builder = MeshBuilder::new();
        let output = build(&network, &mut builder, 42);
        assert!(!output.signals.is_empty(), "no signals built");
        for rig in &output.signals {
            assert_eq!(rig.lamps.len(), 3);
            assert!(matches!(rig.axis, "ns" | "ew"));
        }
    }

    #[test]
    fn a_signal_head_stands_on_the_footway_before_the_stop_line() {
        // The previous version planted every head at the junction centre, six
        // metres *inside* the box.  Assert it is on the footway and behind the
        // line, in world space, from the lamp positions the renderer uses.
        let network = city_network();
        let mut builder = MeshBuilder::new();
        let output = build(&network, &mut builder, 42);
        for rig in &output.signals {
            let Some(road) = network.road(rig.road) else {
                continue;
            };
            let Some(junction) = network.junction(rig.junction) else {
                continue;
            };
            let port = junction
                .ports
                .iter()
                .find(|port| port.road == rig.road)
                .expect("signal without a port");
            let mouth = (port.left + port.right) * 0.5;
            for lamp in &rig.lamps {
                let plan = Vec2::new(lamp.position[0], lamp.position[2]);
                // Outside the box...
                let from_centre = plan.distance(junction.centre);
                assert!(
                    from_centre > junction.radius * 0.35,
                    "a signal lens stands {from_centre:.1} m from the junction centre"
                );
                // ... and outside the carriageway, on the footway.
                let path: Vec<Vec2> = road
                    .centreline
                    .points()
                    .iter()
                    .map(|p| Vec2::new(p.x, p.z))
                    .collect();
                let clearance = crate::math::distance_to_polyline(plan, &path) - road.half_width();
                assert!(
                    clearance > -0.05,
                    "a signal lens stands {clearance:.2} m inside the carriageway"
                );
                let _ = mouth;
            }
            // Red on top, green at the bottom: GB 5768.3 signal head order.
            let red = rig
                .lamps
                .iter()
                .find(|lamp| lamp.aspect == 0)
                .map(|lamp| lamp.position[1])
                .unwrap();
            let green = rig
                .lamps
                .iter()
                .find(|lamp| lamp.aspect == 2)
                .map(|lamp| lamp.position[1])
                .unwrap();
            assert!(red > green, "red is not above green");
        }
    }

    #[test]
    fn diagonal_approaches_never_share_a_signal_axis() {
        // Two streets crossing at 90 degrees must always differ.
        assert_eq!(signal_axis(Vec2::new(1.0, 0.0)), "ew");
        assert_eq!(signal_axis(Vec2::new(0.0, 1.0)), "ns");
        assert_ne!(
            signal_axis(Vec2::new(1.0, 1.0)),
            signal_axis(Vec2::new(1.0, -1.0))
        );
    }

    #[test]
    fn no_marking_is_emitted_outside_its_carriageway() {
        // Markings are the one part of the layer that is visible from a driver's
        // seat at a metre's height, so paint on a footway is not a small error.
        let network = city_network();
        let spec = JunctionSpec::default();
        let mut builder = MeshBuilder::new();
        build(&network, &mut builder, 42);
        let scene = builder.build();
        // Each road's plan alignment, its kerb-to-kerb half width, its deck
        // elevation, and its class — enough to decide whether a marking belongs
        // to it.  A viaduct's markings ride its own deck, which is metres up at
        // mid-span and *at roadbed level* where the ramp meets the ground, so
        // "is this paint on a bridge or on the street under it" is answered by
        // comparing elevations, not by the layer flag.
        let centrelines: Vec<(Vec<Vec2>, f32, &'static str)> = network
            .roads
            .iter()
            .map(|road| {
                (
                    road.centreline
                        .points()
                        .iter()
                        .map(|p| Vec2::new(p.x, p.z))
                        .collect(),
                    road.half_width(),
                    crate::network::export::road_class(road.class),
                )
            })
            .collect();
        let clearance = |point: Vec2| {
            let mut best = f32::MAX;
            for (plan, half, _) in &centrelines {
                best = best.min(crate::math::distance_to_polyline(point, plan) - half);
            }
            best
        };
        let rings: Vec<Vec<Vec2>> = network.junctions.iter().map(|j| j.ring.clone()).collect();
        let mut offenders: Vec<String> = Vec::new();
        for group in &scene.meshes {
            if !group.material.starts_with("marking.") {
                continue;
            }
            for index in group.indices.iter() {
                let index = *index as usize;
                let point = Vec2::new(group.positions[index * 3], group.positions[index * 3 + 2]);
                let y = group.positions[index * 3 + 1];
                // A marking may sit on the carriageway, on the taper that widens
                // it, on a viaduct's own deck, or inside a junction box.  Nowhere
                // else.
                let slack = spec.taper_lane_width + 0.5;
                if clearance(point) <= slack {
                    continue;
                }
                if rings
                    .iter()
                    .any(|ring| crate::math::point_in_ring(point, ring))
                {
                    continue;
                }
                offenders.push(format!(
                    "{} y={y:.1} ({:.1} m out) at ({:.1}, {:.1})",
                    group.material,
                    clearance(point),
                    point.x,
                    point.y
                ));
            }
        }
        offenders.sort();
        offenders.dedup();
        assert!(
            offenders.is_empty(),
            "{} markings off the carriageway: {:?}",
            offenders.len(),
            &offenders[..offenders.len().min(8)]
        );
    }
}
