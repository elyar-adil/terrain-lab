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
mod viaduct;

use viaduct::Gap;

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
    /// Approach widenings at the start and the end (see [`crate::network::Widening`]).
    pub widen: [crate::network::Widening; 2],
    /// Offsets at least this far out move out with a widening: the outer edge
    /// of the motor lanes and everything beyond it.
    pub widen_from: f32,
}

impl Carriageway {
    pub fn for_road(road: &Road, spec: &crate::spec::JunctionSpec) -> Self {
        let length = road.carriageway.length();
        let half = road.half_width();
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
            widen: road.widen,
            widen_from: road.section.half_carriageway() - 0.45,
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
            widen: Default::default(),
            widen_from: f32::INFINITY,
        }
    }

    /// How far a point at `offset` moves out at `station`, for the widening on
    /// its side.
    pub fn widening(&self, station: f32, offset: f32) -> f32 {
        if offset.abs() < self.widen_from {
            return 0.0;
        }
        if offset < 0.0 {
            -self.widen[0].at(station)
        } else {
            self.widen[1].at(self.length - station)
        }
    }

    /// Stations from `from` to `to` at most `step` apart, at every vertex, and
    /// at most two metres apart along a widening's taper so its curve is smooth.
    pub fn stations(&self, from: f32, to: f32, step: f32) -> Vec<f32> {
        let mut out = stations(&self.path, from, to, step);
        for (k, w) in self.widen.iter().enumerate() {
            if !w.is_some() {
                continue;
            }
            let (a, b) = if k == 0 {
                (w.hold, w.hold + w.taper)
            } else {
                (self.length - w.hold - w.taper, self.length - w.hold)
            };
            let (a, b) = (a.max(from), b.min(to));
            if b > a {
                let n = ((b - a) / 2.0).ceil() as usize;
                out.extend((0..=n).map(|i| a + (b - a) * i as f32 / n as f32));
            }
        }
        out.sort_by(f32::total_cmp);
        out.dedup_by(|a, b| (*a - *b).abs() < 0.05);
        out
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
        let crown = self.lift(offset, station);
        let offset = offset + self.widening(station, offset);
        let (position, tangent) = self.path.sample(station);
        // At a vertex the offset is mitred, so the edges of the two segments
        // meet there instead of leaving a notch on the outside of the bend.
        let before = self.path.tangent_at(station - 0.01);
        let after = self.path.tangent_at(station + 0.01);
        let lateral =
            if before.dot(after) < 0.9999 && station > 0.01 && station < self.length - 0.01 {
                let bisector = (before + after).normalize();
                let cos = bisector.dot(after).max(0.3);
                bisector.left_normal() * (offset / cos)
            } else {
                tangent.left_normal() * offset
            };
        Vec3::new(
            position.x + lateral.x,
            position.y + lift + crown,
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
    let mut previous: Option<(Vec3, Vec3, f32)> = None;
    for station in road.stations(from, to, MAX_RESAMPLE_METRES) {
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

/// Stations from `from` to `to` at most `step` apart, and at every vertex of
/// `path` between them.  A ribbon that skipped a vertex would cut the corner
/// of every bend in its street, and the footway's back edge would pull off the
/// block beside it.
pub(crate) fn stations(path: &Path, from: f32, to: f32, step: f32) -> Vec<f32> {
    let steps = (((to - from) / step).ceil().max(1.0)) as usize;
    let mut out: Vec<f32> = (0..=steps)
        .map(|k| from + (to - from) * k as f32 / steps as f32)
        .collect();
    out.extend(
        path.stations()
            .iter()
            .copied()
            .filter(|s| *s > from + 0.05 && *s < to - 0.05),
    );
    out.sort_by(f32::total_cmp);
    out.dedup_by(|a, b| (*a - *b).abs() < 0.05);
    out
}

/// The stretches of other streets' carriageways that come within reach of
/// `road`'s footways anywhere along it, not only at its junctions: two streets
/// the plan ran side by side, their footways on each other's road. Empty for
/// almost every street.
pub(super) fn road_beside(network: &Network, road: &Road) -> Vec<Vec<Vec2>> {
    let own: Vec<Vec2> = road
        .carriageway
        .points()
        .iter()
        .map(|p| Vec2::new(p.x, p.z))
        .collect();
    if own.len() < 2 {
        return Vec::new();
    }
    let reach = road.half_width() + road.section.sidewalk_metres;
    let bounds = |points: &[Vec2], grow: f32| {
        let (x0, z0, x1, z1) = points.iter().fold(
            (f32::MAX, f32::MAX, f32::MIN, f32::MIN),
            |(x0, z0, x1, z1), p| (x0.min(p.x), z0.min(p.y), x1.max(p.x), z1.max(p.y)),
        );
        (x0 - grow, z0 - grow, x1 + grow, z1 + grow)
    };
    let mine = bounds(&own, reach);
    let mut out = Vec::new();
    for other in network
        .roads
        .iter()
        .filter(|r| r.id != road.id && r.layer == 0)
    {
        let surface = Carriageway::for_road(other, &network.spec);
        if surface.length < 1.0 {
            continue;
        }
        let half = other.half_width();
        let line: Vec<Vec2> = other
            .carriageway
            .points()
            .iter()
            .map(|p| Vec2::new(p.x, p.z))
            .collect();
        let theirs = bounds(&line, half);
        if theirs.0 > mine.2 || theirs.2 < mine.0 || theirs.1 > mine.3 || theirs.3 < mine.1 {
            continue;
        }
        let run = surface.stations(0.0, surface.length, 3.0);
        let close: Vec<bool> = run
            .iter()
            .map(|s| {
                let p = surface.path.plan_at(*s);
                crate::math::distance_to_polyline(p, &own) < reach + half + ROAD_WIDEN_SLACK
            })
            .collect();
        let mut k = 0;
        while k < run.len() {
            if !close[k] {
                k += 1;
                continue;
            }
            let first = k.saturating_sub(1);
            while k < run.len() && close[k] {
                k += 1;
            }
            let last = k.min(run.len() - 1);
            let stretch = &run[first..=last];
            if stretch.len() < 2 {
                continue;
            }
            let edge = |s: f32, lateral: f32| {
                let p = surface.point(s, lateral, 0.0);
                Vec2::new(p.x, p.z)
            };
            let mut ring: Vec<Vec2> = stretch.iter().map(|s| edge(*s, half)).collect();
            ring.extend(stretch.iter().rev().map(|s| edge(*s, -half)));
            out.push(ring);
        }
    }
    out
}

/// A widened approach stands up to a lane further out than its section, and
/// a stretch is taken with a few metres to spare.
const ROAD_WIDEN_SLACK: f32 = 6.0;

/// Within this distance of a junction a road's footway and kerb are cut
/// against the junction's other carriageways ([`ribbon_clipped`]).
pub(super) const JUNCTION_ZONE_M: f32 = 70.0;

/// The road near a junction a footway must never be laid on: the junction
/// box, and every other street's carriageway that comes within
/// `JUNCTION_ZONE_M` of the junction, whether it meets there or at a
/// junction a few metres off.  Empty where the road's end is not a junction.
///
/// Within `zone` metres of the junction, `JUNCTION_ZONE_M` or more: two
/// streets meeting at a narrow angle run side by side further than that
/// before their footways part.
pub(super) fn road_near_within(
    network: &Network,
    road: &Road,
    at_start: bool,
    zone: f32,
) -> Vec<Vec<Vec2>> {
    let node = if at_start {
        road.from_node
    } else {
        road.to_node
    };
    let Some(junction) = network.junction(node) else {
        return Vec::new();
    };
    if junction.ports.len() < 2 {
        return Vec::new();
    }
    let centre = junction.centre;
    let mut out = Vec::new();
    if junction.ring.len() >= 3 {
        out.push(junction.ring.clone());
    }
    for other in network
        .roads
        .iter()
        .filter(|r| r.id != road.id && r.layer == 0)
    {
        // Cheaply out of reach: the junction is nowhere near the road's box.
        let (lo, hi) = other.carriageway.points().iter().fold(
            (Vec2::new(f32::MAX, f32::MAX), Vec2::new(f32::MIN, f32::MIN)),
            |(lo, hi), p| {
                (
                    Vec2::new(lo.x.min(p.x), lo.y.min(p.z)),
                    Vec2::new(hi.x.max(p.x), hi.y.max(p.z)),
                )
            },
        );
        let reach = zone + other.half_width();
        if centre.x < lo.x - reach
            || centre.x > hi.x + reach
            || centre.y < lo.y - reach
            || centre.y > hi.y + reach
        {
            continue;
        }
        let surface = Carriageway::for_road(other, &network.spec);
        let length = surface.length;
        if length < 1.0 {
            continue;
        }
        // The stretch of the other road near this junction, if any.
        let near: Vec<f32> = surface
            .stations(0.0, length, 4.0)
            .into_iter()
            .filter(|s| {
                let p = surface.path.plan_at(*s);
                p.distance(centre) < zone
            })
            .collect();
        let (Some(&from), Some(&to)) = (near.first(), near.last()) else {
            continue;
        };
        if to - from < 0.5 {
            continue;
        }
        let half = other.half_width();
        let run = surface.stations(from, to, 4.0);
        let mut ring: Vec<Vec2> = run
            .iter()
            .map(|s| {
                let p = surface.point(*s, half, 0.0);
                Vec2::new(p.x, p.z)
            })
            .collect();
        ring.extend(run.iter().rev().map(|s| {
            let p = surface.point(*s, -half, 0.0);
            Vec2::new(p.x, p.z)
        }));
        if ring.len() >= 3 {
            out.push(ring);
        }
    }
    // Junction boxes a few metres off, too.
    for other in &network.junctions {
        if other.node != node
            && other.ports.len() >= 2
            && other.centre.distance(centre) < zone
            && other.ring.len() >= 3
        {
            out.push(other.ring.clone());
        }
    }
    out
}

/// [`ribbon`] on a flat surface (a footway, a kerb top), with `clips` cut out
/// of every quad that reaches one, so it is never laid over a carriageway.
#[allow(clippy::too_many_arguments)]
pub(super) fn ribbon_clipped(
    builder: &mut MeshBuilder,
    material: &str,
    road: &Carriageway,
    inner: f32,
    outer: f32,
    from: f32,
    to: f32,
    lift: f32,
    uv: Uvs,
    clips: &[Vec<Vec2>],
) {
    if clips.is_empty() {
        ribbon(
            builder, material, road, inner, outer, from, to, lift, None, uv,
        );
        return;
    }
    if road.length <= 1.0e-4 || to <= from {
        return;
    }
    let run = road.stations(from, to, 2.0);
    for pair in run.windows(2) {
        let quad = [
            road.point(pair[0], inner, lift),
            road.point(pair[0], outer, lift),
            road.point(pair[1], outer, lift),
            road.point(pair[1], inner, lift),
        ];
        let plan: Vec<Vec2> = quad.iter().map(|p| Vec2::new(p.x, p.z)).collect();
        let y = quad.iter().map(|p| p.y).sum::<f32>() * 0.25;
        for piece in crate::buildings::cut_away(plan, clips) {
            if crate::math::signed_area(&piece).abs() < 1.0e-3 {
                continue;
            }
            for [i, j, k] in crate::math::triangulate(&piece) {
                let (a, b, c) = (
                    Vec3::from_plan(piece[i], y),
                    Vec3::from_plan(piece[j], y),
                    Vec3::from_plan(piece[k], y),
                );
                let (a, b, c) = if crate::mesh::face_normal(a, c, b).y >= 0.0 {
                    (a, c, b)
                } else {
                    (a, b, c)
                };
                match uv {
                    Uvs::None => builder.tri_flat(material, a, b, c, None),
                    _ => builder.tri_uv(
                        material,
                        a,
                        b,
                        c,
                        [(a.x, a.z), (b.x, b.z), (c.x, c.z)],
                        None,
                    ),
                }
            }
        }
    }
}

/// One quad (`a b c d` round its edge, possibly sloped) with `clips` cut out,
/// each remaining point at the height the quad has there.
pub(super) fn quad_clipped(
    builder: &mut MeshBuilder,
    material: &str,
    quad: [Vec3; 4],
    clips: &[Vec<Vec2>],
) {
    let plan: Vec<Vec2> = quad.iter().map(|p| Vec2::new(p.x, p.z)).collect();
    // Height on the quad's two triangles (a b c) and (a c d), by barycentric.
    let height = |p: Vec2| -> f32 {
        for [i, j, k] in [[0, 1, 2], [0, 2, 3]] {
            let (a, b, c) = (plan[i], plan[j], plan[k]);
            let area = (b - a).cross(c - a);
            if area.abs() < 1.0e-6 {
                continue;
            }
            let w1 = (p - a).cross(c - a) / area;
            let w2 = (b - a).cross(p - a) / area;
            let w0 = 1.0 - w1 - w2;
            if w0 >= -1.0e-3 && w1 >= -1.0e-3 && w2 >= -1.0e-3 {
                return w0 * quad[i].y + w1 * quad[j].y + w2 * quad[k].y;
            }
        }
        quad.iter().map(|q| q.y).sum::<f32>() * 0.25
    };
    for piece in crate::buildings::cut_away(plan.clone(), clips) {
        for [i, j, k] in crate::math::triangulate(&piece) {
            let (a, b, c) = (
                Vec3::from_plan(piece[i], height(piece[i])),
                Vec3::from_plan(piece[j], height(piece[j])),
                Vec3::from_plan(piece[k], height(piece[k])),
            );
            let (a, b, c) = if crate::mesh::face_normal(a, c, b).y >= 0.0 {
                (a, c, b)
            } else {
                (a, b, c)
            };
            builder.tri_uv(
                material,
                a,
                b,
                c,
                [(a.x, a.z), (b.x, b.z), (c.x, c.z)],
                None,
            );
        }
    }
}

/// [`sweep`]'s two walls, leaving out every piece that would stand on a
/// carriageway in `clips`.
#[allow(clippy::too_many_arguments)]
pub(super) fn sweep_clipped(
    builder: &mut MeshBuilder,
    material: &str,
    surface: &Carriageway,
    inner: f32,
    outer: f32,
    from: f32,
    to: f32,
    thickness: f32,
    clips: &[Vec<Vec2>],
) {
    if clips.is_empty() {
        sweep(
            builder, material, surface, inner, outer, from, to, thickness,
        );
        return;
    }
    let run = surface.stations(from, to, 2.0);
    for pair in run.windows(2) {
        for offset in [inner, outer] {
            let (p, q) = (
                surface.point(pair[0], offset, 0.0),
                surface.point(pair[1], offset, 0.0),
            );
            let mid = Vec2::new((p.x + q.x) * 0.5, (p.z + q.z) * 0.5);
            if clips
                .iter()
                .any(|ring| crate::math::point_in_ring(mid, ring))
            {
                continue;
            }
            let (a, b) = if offset == inner {
                (Vec2::new(p.x, p.z), Vec2::new(q.x, q.z))
            } else {
                (Vec2::new(q.x, q.z), Vec2::new(p.x, p.z))
            };
            builder.wall(material, a, b, p.y - thickness, p.y, None);
        }
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
    let mut previous: Option<(Vec3, Vec3)> = None;
    for station in surface.stations(from, to, 14.0) {
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
    let joins = viaduct::joins(network);
    for road in &network.roads {
        surfaces::road_surface(network, road, builder, &spec, joins.tapers(road.id));
        markings::road_markings(network, road, builder, &spec, joins.tapers(road.id));
        markings::widening_markings(road, builder, &spec);
        // An expressway's deck and its ramps are a viaduct; anything else on
        // a structure is a bridge.
        if road.is_motorway() && road.layer != 0 {
            viaduct::structure(network, road, &joins, builder);
        } else {
            bridge::bridge_structure(road, builder);
        }
    }
    let mut output = StreetOutput::default();
    for junction in &network.junctions {
        surfaces::junction_geometry(network, junction, builder, &spec);
        surfaces::junction_details(
            network,
            junction,
            builder,
            &spec,
            !markings::node_has_waiting_box(network, junction.node),
        );
        if junction.kind == urban::JunctionKind::Roundabout {
            surfaces::roundabout(junction, builder, &spec);
        }
        if junction.kind == urban::JunctionKind::RightInRightOut {
            surfaces::median_across(network, junction, builder);
        }
    }
    markings::junction_lane_guides(network, builder);
    output.signals = signals::build_signals(network, builder);
    output.traffic_lights_built = output.signals.len() as u32;
    let _ = seed;
    output
}

/// The ground the streets take, as closed rings: every road from the back of
/// one footway to the back of the other, and every junction to the back of
/// its corner footways, each pulled in by `under_footway` so whatever is laid
/// beside the streets runs on that far under the paving.  An elevated road
/// takes only the ground where it is down at street level.
///
/// The block plates are cut out of this, so a plate can never be drawn over a
/// street: not at a junction corner the kerb rounds, not over a dead-end
/// street running into the block, not where a street bows off the line the
/// block was cut along.
pub fn footprints(network: &Network, under_footway: f32) -> Vec<Vec<Vec2>> {
    let mut rings = Vec::new();
    for road in &network.roads {
        if road.carriageway.length() < 1.0 {
            continue;
        }
        let surface = Carriageway::for_road(road, &network.spec);
        let half = road.half_width();
        let reach = if road.has_sidewalk() {
            (half + road.section.sidewalk_metres - under_footway).max(half)
        } else {
            half + 0.3
        };
        let mut run: Vec<f32> = Vec::new();
        // A metre on past either end of the road, straight on, so the road's
        // ground overlaps its junction's rather than leaving a seam across
        // the mouth where neither reaches.
        let at = |station: f32, offset: f32| -> Vec2 {
            let (from, past) = if station <= 0.0 {
                (0.0, station)
            } else if station >= surface.length {
                (surface.length, station - surface.length)
            } else {
                (station, 0.0)
            };
            let p = surface.point(from, offset, 0.0);
            let along = surface.path.tangent_at(from) * past;
            Vec2::new(p.x + along.x, p.z + along.y)
        };
        let flush = |run: &mut Vec<f32>, rings: &mut Vec<Vec<Vec2>>| {
            if run.len() >= 2 {
                if run[0] <= 0.0 {
                    run.insert(0, -1.0);
                }
                if run[run.len() - 1] >= surface.length {
                    run.push(surface.length + 1.0);
                }
                let mut ring: Vec<Vec2> = run.iter().map(|s| at(*s, reach)).collect();
                ring.extend(run.iter().rev().map(|s| at(*s, -reach)));
                rings.push(ring);
            }
            run.clear();
        };
        for station in surface.stations(0.0, surface.length, 6.0) {
            if surface.path.sample(station).0.y < 1.0 {
                run.push(station);
            } else {
                flush(&mut run, &mut rings);
            }
        }
        flush(&mut run, &mut rings);
    }
    for junction in &network.junctions {
        if junction.ports.len() == 1 {
            // A dead end: the footway across its end (`dead_end_footway`).
            let port = &junction.ports[0];
            if let Some(band) = surfaces::dead_end_footway(port) {
                let back = port.dir * under_footway;
                rings.push(vec![band[0], band[1], band[2] - back, band[3] - back]);
            }
            continue;
        }
        if junction.walk_ring.len() >= 3 {
            let inset = crate::math::inset_ring(&junction.walk_ring, under_footway);
            rings.push(if inset.len() >= 3 {
                inset
            } else {
                junction.walk_ring.clone()
            });
        }
        if junction.ring.len() >= 3 {
            rings.push(junction.ring.clone());
        }
    }
    rings
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
