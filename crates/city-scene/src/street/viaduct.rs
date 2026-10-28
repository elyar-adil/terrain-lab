//! The expressway viaduct (高架) and its ramps, drawn the way the reference
//! photographs of Shenzhen, Guangzhou and Shanghai show them:
//!
//! - **Deck.** A box girder: a vertical fascia under the parapet, cantilever
//!   wings with a sloped soffit out to about a fifth of the width each side,
//!   inclined webs, and a flat bottom flange about two thirds as wide as the
//!   deck. Two metres deep on the main line, 1.6 m on a ramp. Where a ramp comes
//!   down to the ground the section closes into a retaining wall.
//! - **Parapets.** A cast F-shape barrier, 1.1 m above the deck, its outer face
//!   flush with the fascia; on the main line a double-faced barrier down the
//!   median and lamp columns on it every 40 m. A deck past homes carries a
//!   tinted glass sound barrier on its parapets (Shanghai); others hang a
//!   planted trough over the edge (Guangzhou, Shenzhen).
//! - **Piers.** Every 30 m on the main line, either twin round columns under
//!   the webs (Shenzhen) or one column with a hammerhead cap (Shanghai); every
//!   25 m under a ramp, one round column. A pier never stands in a street: it
//!   moves along the deck to clear it, or the span is made longer.
//! - **Joins.** Where a ramp leaves or joins the deck in the air, the deck
//!   widens over the taper, the gore between the two is paved and hatched up to
//!   the nose, and neither deck has a parapet on the side where they meet.

use std::collections::HashMap;

use crate::math::{Vec2, Vec3};
use crate::mesh::{GroupStyle, MeshBuilder};
use crate::network::{Network, Road};
use worldgen_core::hash::cell01;

use super::{Carriageway, level};

const CONCRETE: &str = "viaduct.concrete";
const GLASS: &str = "viaduct.glass";
const PLANTER: &str = "viaduct.planter";
/// The lowest deck a planter trough hangs on.
const PLANTER_MIN_DECK_M: f32 = 4.0;
const STEEL: &str = "rail.steel";

/// Fair-faced concrete in the light, the parapet's face, the soffit in its own
/// shade and the piers between them (linear vertex tints).
const FASCIA: [f32; 3] = [0.66, 0.65, 0.62];
const PARAPET: [f32; 3] = [0.72, 0.71, 0.68];
const SOFFIT: [f32; 3] = [0.50, 0.50, 0.48];
const PIER: [f32; 3] = [0.60, 0.59, 0.56];
const CUSHION: [f32; 3] = [0.78, 0.56, 0.05];

/// A stretch of one deck edge that has no parapet, because another deck joins it.
#[derive(Clone, Copy, Debug)]
pub(super) struct Gap {
    /// `-1` or `1`: the edge's side of the centreline.
    pub(super) side: f32,
    pub(super) from: f32,
    pub(super) to: f32,
}

/// One ramp's join with the main deck.
#[derive(Clone, Copy, Debug)]
struct Join {
    main: u32,
    ramp: u32,
    /// The side of the main deck the ramp is on.
    side: f32,
    /// The side of the ramp the main deck is on.
    inner: f32,
    /// Station on the main deck of the node, and of the ramp's deck end.
    at_node: f32,
    at_ramp: f32,
    /// Stations of the nose (where the gap between the decks has opened), on
    /// the main deck and on the ramp.
    nose_main: f32,
    nose_ramp: f32,
    /// The ramp leaves the main deck here (or joins it).
    diverge: bool,
}

/// Where the ramps meet the main decks, and the parapet gaps that follow.
pub(super) struct Joins {
    joins: Vec<Join>,
    gaps: HashMap<u32, Vec<Gap>>,
    tapers: HashMap<u32, Vec<Gap>>,
}

impl Joins {
    pub(super) fn gaps(&self, road: u32) -> &[Gap] {
        self.gaps.get(&road).map(Vec::as_slice).unwrap_or(&[])
    }

    /// The stretches of a main deck's edge where it widens into a ramp: from
    /// the node to where the ramp's own deck begins.  The deck's edge line is
    /// broken there and its shoulder gives way to the new lane.
    pub(super) fn tapers(&self, road: u32) -> &[Gap] {
        self.tapers.get(&road).map(Vec::as_slice).unwrap_or(&[])
    }
}

/// Station along `road`'s carriageway of the nearest point to `p`, and the
/// signed lateral offset (positive on the left normal, the kerb side of travel).
fn project(road: &Road, p: Vec2) -> (f32, f32) {
    let points = road.carriageway.points();
    let stations = road.carriageway.stations();
    let mut best = (f32::MAX, 0.0, 0.0);
    for i in 0..points.len().saturating_sub(1) {
        let a = Vec2::new(points[i].x, points[i].z);
        let b = Vec2::new(points[i + 1].x, points[i + 1].z);
        let d = b - a;
        let l2 = d.length_squared().max(1.0e-9);
        let t = ((p - a).dot(d) / l2).clamp(0.0, 1.0);
        let q = a + d * t;
        let gap = p.distance(q);
        if gap < best.0 {
            let lateral = (p - q).dot(d.normalize().left_normal());
            best = (
                gap,
                stations[i] + t * (stations[i + 1] - stations[i]),
                lateral,
            );
        }
    }
    (best.1, best.2)
}

fn plan(p: Vec3) -> Vec2 {
    Vec2::new(p.x, p.z)
}

/// Find every join of a ramp and a main deck in the air.
pub(super) fn joins(network: &Network) -> Joins {
    let mut joins = Vec::new();
    for node in 0..network.air.len() as u32 {
        if !network.is_air(node) {
            continue;
        }
        let roads: Vec<&Road> = network
            .incident_roads(node)
            .iter()
            .filter_map(|id| network.road(*id))
            .collect();
        let mains: Vec<&Road> = roads.iter().copied().filter(|r| !r.oneway).collect();
        for ramp in roads.iter().copied().filter(|r| r.oneway) {
            let diverge = ramp.from_node == node;
            let length = ramp.carriageway.length();
            let end = if diverge { 0.0 } else { length };
            let p = plan(ramp.carriageway.sample(end).0);
            // The main deck the ramp lies beside.
            // By true distance: the lateral offset alone picks the piece on the
            // far side of the node, onto whose end the ramp projects.
            let distance = |m: &Road| {
                let plan: Vec<Vec2> = m.carriageway.points().iter().map(|q| plan(*q)).collect();
                crate::math::distance_to_polyline(p, &plan)
            };
            let Some((main, (at_ramp, lateral))) = mains
                .iter()
                .min_by(|a, b| distance(a).total_cmp(&distance(b)))
                .map(|m| (*m, project(m, p)))
            else {
                continue;
            };
            let side = lateral.signum();
            let at_node = if main.from_node == node {
                0.0
            } else {
                main.carriageway.length()
            };
            let half_main = main.half_width();
            let half_ramp = ramp.half_width();
            // The main deck seen from the ramp, to know the ramp's inner side.
            let toward = plan(main.carriageway.sample(at_ramp).0);
            let inner = project(ramp, toward).1.signum();
            // Walk the ramp away from its deck end until the gap between the
            // two edges is a metre: that is the nose.
            let mut nose_ramp = if diverge { 0.0 } else { length };
            let mut nose_main = at_ramp;
            let step = 2.0;
            let mut walked = 0.0;
            while walked < length.min(120.0) {
                let u = if diverge { walked } else { length - walked };
                let edge = ramp.carriageway.offset_at(u, inner * half_ramp, 0.0);
                let (s, l) = project(main, plan(edge));
                nose_ramp = u;
                nose_main = s;
                if l.abs() - half_main > 1.0 {
                    break;
                }
                walked += step;
            }
            joins.push(Join {
                main: main.id,
                ramp: ramp.id,
                side,
                inner,
                at_node,
                at_ramp,
                nose_main,
                nose_ramp,
                diverge,
            });
        }
    }
    let mut gaps: HashMap<u32, Vec<Gap>> = HashMap::new();
    let mut tapers: HashMap<u32, Vec<Gap>> = HashMap::new();
    for join in &joins {
        tapers.entry(join.main).or_default().push(Gap {
            side: join.side,
            from: join.at_node.min(join.at_ramp),
            to: join.at_node.max(join.at_ramp),
        });
        let (a, b) = (join.at_node, join.nose_main);
        gaps.entry(join.main).or_default().push(Gap {
            side: join.side,
            from: a.min(b),
            to: a.max(b),
        });
        let ramp_length = network
            .road(join.ramp)
            .map(|r| r.carriageway.length())
            .unwrap_or(0.0);
        let (from, to) = if join.diverge {
            (0.0, join.nose_ramp)
        } else {
            (join.nose_ramp, ramp_length)
        };
        gaps.entry(join.ramp).or_default().push(Gap {
            side: join.inner,
            from,
            to,
        });
    }
    Joins {
        joins,
        gaps,
        tapers,
    }
}

fn declare(builder: &mut MeshBuilder) {
    let solid = GroupStyle {
        cast_shadow: true,
        receive_shadow: true,
        alpha_cutout: false,
        dynamic: false,
    };
    builder.style(CONCRETE, solid);
    builder.style(PLANTER, solid);
    builder.style(
        GLASS,
        GroupStyle {
            cast_shadow: false,
            ..solid
        },
    );
}

/// A quad wound to face `out`.
fn quad_out(builder: &mut MeshBuilder, material: &str, q: [Vec3; 4], out: Vec3, colour: [f32; 3]) {
    let n = crate::mesh::face_normal(q[0], q[1], q[2]);
    if n.dot(out) >= 0.0 {
        builder.quad(material, q[0], q[1], q[2], q[3], Some(colour));
    } else {
        builder.quad(material, q[3], q[2], q[1], q[0], Some(colour));
    }
}

/// Asphalt on the shared world-metre grid (the group carries UVs on every
/// vertex), wound to face up.
fn asphalt_quad(builder: &mut MeshBuilder, q: [Vec3; 4]) {
    let q = if crate::mesh::face_normal(q[0], q[1], q[2]).y >= 0.0 {
        q
    } else {
        [q[3], q[2], q[1], q[0]]
    };
    builder.quad_uv(
        "asphalt",
        q[0],
        q[1],
        q[2],
        q[3],
        [
            (q[0].x, q[0].z),
            (q[1].x, q[1].z),
            (q[2].x, q[2].z),
            (q[3].x, q[3].z),
        ],
        None,
    );
}

/// Paint, wound to face up.
fn paint_quad(builder: &mut MeshBuilder, material: &str, q: [Vec3; 4]) {
    if crate::mesh::face_normal(q[0], q[1], q[2]).y >= 0.0 {
        builder.quad(material, q[0], q[1], q[2], q[3], None);
    } else {
        builder.quad(material, q[3], q[2], q[1], q[0], None);
    }
}

/// The box girder's outline below the deck surface at one station: lateral
/// offset and depth below the top, from the left fascia round to the right.
fn girder_outline(half: f32, depth: f32) -> [(f32, f32, [f32; 3]); 6] {
    let wing = (half * 0.42).max(1.2);
    let bottom = (half * 0.66).max(1.0);
    [
        (-half, -0.45, FASCIA),
        (-(half - wing), -0.85, SOFFIT),
        (-bottom * 0.5 - 0.6, -depth, SOFFIT),
        (bottom * 0.5 + 0.6, -depth, SOFFIT),
        (half - wing, -0.85, SOFFIT),
        (half, -0.45, FASCIA),
    ]
}

/// The structure of one expressway deck or ramp.
pub(super) fn structure(network: &Network, road: &Road, joins: &Joins, builder: &mut MeshBuilder) {
    if road.layer == 0 || road.bridge {
        return;
    }
    declare(builder);
    let spec = network.spec;
    let surface = Carriageway::for_road(road, &spec);
    let length = surface.length;
    if length < 2.0 {
        return;
    }
    let half = road.half_width();
    let depth = if road.oneway { 1.6 } else { 2.1 };
    let ground = -0.2_f32;
    let roll = |salt: u32| cell01(salt, road.id as i32, road.from_node as i32, road.to_node);

    // --- the girder --------------------------------------------------------
    let outline = girder_outline(half, depth);
    let steps = ((length / 5.0).ceil() as usize).max(1);
    let at = |s: f32, u: f32, v: f32| {
        let p = surface.point(s, u, 0.0);
        Vec3::new(p.x, (p.y + v).max(ground), p.z)
    };
    for k in 0..steps {
        let (s0, s1) = (
            length * k as f32 / steps as f32,
            length * (k + 1) as f32 / steps as f32,
        );
        let centre = surface.point((s0 + s1) * 0.5, 0.0, -depth * 0.5);
        // The fascia, from the deck edge down to the wing.
        for side in [-1.0_f32, 1.0] {
            let q = [
                at(s0, side * half, 0.0),
                at(s1, side * half, 0.0),
                at(s1, side * half, -0.45),
                at(s0, side * half, -0.45),
            ];
            let mid = surface.point((s0 + s1) * 0.5, side * (half + 1.0), 0.0);
            quad_out(builder, CONCRETE, q, mid - centre, FASCIA);
        }
        for w in outline.windows(2) {
            let (a, b) = (w[0], w[1]);
            let q = [
                at(s0, a.0, a.1),
                at(s1, a.0, a.1),
                at(s1, b.0, b.1),
                at(s0, b.0, b.1),
            ];
            // Every face of the underside looks down and out of the box.
            let mid_u = (a.0 + b.0) * 0.5;
            let out = Vec3::new(0.0, -1.0, 0.0)
                + (surface.point((s0 + s1) * 0.5, mid_u * 2.0, 0.0)
                    - surface.point((s0 + s1) * 0.5, 0.0, 0.0))
                    * 0.05;
            quad_out(builder, CONCRETE, q, out, b.2);
        }
    }
    // End faces where the deck stops: a ramp's foot is closed by its fill.
    for (s, sign) in [(0.0_f32, -1.0_f32), (length, 1.0)] {
        let node = if sign < 0.0 {
            road.from_node
        } else {
            road.to_node
        };
        if network.is_air(node) {
            continue;
        }
        let tangent = surface.path.sample(s).1;
        let out = Vec3::new(tangent.x * sign, 0.0, tangent.y * sign);
        let mut ring = vec![at(s, -half, 0.0)];
        for (u, v, _) in outline {
            ring.push(at(s, u, v));
        }
        ring.push(at(s, half, 0.0));
        for i in 1..ring.len() - 2 {
            let q = [ring[0], ring[i], ring[i + 1], ring[i + 1]];
            quad_out(builder, CONCRETE, q, out, SOFFIT);
        }
    }

    // --- parapets ----------------------------------------------------------
    let gaps = joins.gaps(road.id);
    let barrier = roll(0x51D) < 0.4 && !road.oneway;
    let planted = !barrier && roll(0x9A7) < 0.5;
    for side in [-1.0_f32, 1.0] {
        let mut runs: Vec<(f32, f32)> = vec![(0.0, length)];
        for gap in gaps.iter().filter(|g| g.side == side) {
            runs = runs
                .into_iter()
                .flat_map(|(a, b)| {
                    let mut out = Vec::new();
                    if gap.from > a {
                        out.push((a, gap.from.min(b)));
                    }
                    if gap.to < b {
                        out.push((gap.to.max(a), b));
                    }
                    out
                })
                .filter(|(a, b)| b - a > 0.5)
                .collect();
        }
        for (a, b) in runs {
            parapet(builder, &surface, side, half, a, b);
            if barrier {
                sound_barrier(builder, &surface, side, half, a, b);
            } else if planted {
                planter(builder, &surface, side, half, a, b, road.id);
            }
        }
    }

    // --- the median and its lamps -----------------------------------------
    if !road.oneway && road.section.median_metres > 0.3 {
        median_barrier(builder, &surface, length);
        let mut s = 20.0;
        while s < length - 10.0 {
            lamp_column(builder, &surface, s, road.section.median_metres * 0.5 + 3.0);
            s += 40.0;
        }
    }

    // --- piers -------------------------------------------------------------
    piers(network, road, &surface, depth, builder);

    // --- the joins this deck carries ----------------------------------------
    for join in joins.joins.iter().filter(|j| j.main == road.id) {
        if let Some(ramp) = network.road(join.ramp) {
            draw_join(builder, network, road, ramp, join);
        }
    }
}

/// A cast F-shape parapet along one edge, from station `a` to `b`.
fn parapet(builder: &mut MeshBuilder, surface: &Carriageway, side: f32, half: f32, a: f32, b: f32) {
    let steps = (((b - a) / 5.0).ceil() as usize).max(1);
    // Across the barrier: (offset in from the edge, height above the deck).
    let profile = [(0.0_f32, 1.1_f32), (0.18, 1.1), (0.28, 0.25), (0.45, 0.0)];
    let point = |s: f32, inset: f32, up: f32| {
        let p = surface.point(s, side * (half - inset), 0.0);
        Vec3::new(p.x, p.y + up, p.z)
    };
    for k in 0..steps {
        let (s0, s1) = (
            a + (b - a) * k as f32 / steps as f32,
            a + (b - a) * (k + 1) as f32 / steps as f32,
        );
        let mid = (s0 + s1) * 0.5;
        let inward =
            surface.point(mid, side * (half - 2.0), 0.6) - surface.point(mid, side * half, 0.6);
        // Outer face, flush with the fascia.
        let outer = [
            point(s0, 0.0, 0.0),
            point(s1, 0.0, 0.0),
            point(s1, 0.0, 1.1),
            point(s0, 0.0, 1.1),
        ];
        quad_out(builder, CONCRETE, outer, inward * -1.0, FASCIA);
        for w in profile.windows(2) {
            let ((i0, h0), (i1, h1)) = (w[0], w[1]);
            let q = [
                point(s0, i0, h0),
                point(s1, i0, h0),
                point(s1, i1, h1),
                point(s0, i1, h1),
            ];
            quad_out(
                builder,
                CONCRETE,
                q,
                inward + Vec3::new(0.0, 0.6, 0.0),
                PARAPET,
            );
        }
    }
    // Ends.
    for (s, sign) in [(a, -1.0_f32), (b, 1.0)] {
        let t = surface.path.sample(s).1;
        let out = Vec3::new(t.x * sign, 0.0, t.y * sign);
        let q = [
            point(s, 0.0, 0.0),
            point(s, 0.45, 0.0),
            point(s, 0.18, 1.1),
            point(s, 0.0, 1.1),
        ];
        quad_out(builder, CONCRETE, q, out, PARAPET);
    }
}

/// Tinted glass panels in steel posts on top of the parapet.
fn sound_barrier(
    builder: &mut MeshBuilder,
    surface: &Carriageway,
    side: f32,
    half: f32,
    a: f32,
    b: f32,
) {
    let height = 3.0;
    let at = |s: f32, up: f32| {
        let p = surface.point(s, side * (half - 0.09), 0.0);
        Vec3::new(p.x, p.y + 1.1 + up, p.z)
    };
    let mut s = a;
    while s < b - 0.5 {
        let next = (s + 2.5).min(b);
        builder.quad(
            GLASS,
            at(s, 0.0),
            at(next, 0.0),
            at(next, height),
            at(s, height),
            None,
        );
        builder.tube(STEEL, at(s, 0.0), at(s, height + 0.1), 0.07, 0.07, 4, None);
        s = next;
    }
    builder.tube(STEEL, at(b, 0.0), at(b, height + 0.1), 0.07, 0.07, 4, None);
}

/// A planted trough hung on the outside of the parapet, bougainvillea spilling
/// over the edge.
#[allow(clippy::too_many_arguments)]
fn planter(
    builder: &mut MeshBuilder,
    surface: &Carriageway,
    side: f32,
    half: f32,
    a: f32,
    b: f32,
    seed: u32,
) {
    let steps = (((b - a) / 3.0).ceil() as usize).max(1);
    let at = |s: f32, out: f32, up: f32| {
        let p = surface.point(s, side * (half + out), 0.0);
        Vec3::new(p.x, p.y + up, p.z)
    };
    for k in 0..steps {
        let (s0, s1) = (
            a + (b - a) * k as f32 / steps as f32,
            a + (b - a) * (k + 1) as f32 / steps as f32,
        );
        // Hung only where the deck is up in the air: on a ramp's low end the
        // foliage spilling down the fascia lay on the embankment beside it.
        if at(s0, 0.0, 0.0).y.min(at(s1, 0.0, 0.0).y) < PLANTER_MIN_DECK_M {
            continue;
        }
        // Foliage: mostly green, a run of magenta here and there, a little
        // thinner where it has not taken.
        let r = cell01(0x5EED, seed as i32, k as i32, 7);
        let leaf = if r < 0.35 {
            [0.42, 0.06, 0.20]
        } else {
            [0.05 + 0.03 * r, 0.12 + 0.05 * r, 0.04]
        };
        let hang = 0.6 + 0.7 * cell01(0x5EED, seed as i32, k as i32, 9);
        let out = Vec3::new(0.0, 0.0, 0.0)
            + (at((s0 + s1) * 0.5, 2.0, 0.0) - at((s0 + s1) * 0.5, 0.0, 0.0));
        // The face that spills down the fascia, and the top of the trough.
        quad_out(
            builder,
            PLANTER,
            [
                at(s0, 0.55, 1.05),
                at(s1, 0.55, 1.05),
                at(s1, 0.55, 1.05 - hang),
                at(s0, 0.55, 1.05 - hang),
            ],
            out,
            leaf,
        );
        quad_out(
            builder,
            PLANTER,
            [
                at(s0, 0.0, 1.15),
                at(s1, 0.0, 1.15),
                at(s1, 0.55, 1.05),
                at(s0, 0.55, 1.05),
            ],
            Vec3::new(0.0, 1.0, 0.0),
            leaf,
        );
    }
}

/// A double-faced concrete barrier down the median.
fn median_barrier(builder: &mut MeshBuilder, surface: &Carriageway, length: f32) {
    let steps = ((length / 5.0).ceil() as usize).max(1);
    let point = |s: f32, u: f32, up: f32| {
        let p = surface.point(s, u, 0.0);
        Vec3::new(p.x, p.y + up, p.z)
    };
    let profile = [
        (-0.3_f32, 0.0_f32),
        (-0.18, 0.25),
        (-0.08, 1.0),
        (0.08, 1.0),
        (0.18, 0.25),
        (0.3, 0.0),
    ];
    for k in 0..steps {
        let (s0, s1) = (
            length * k as f32 / steps as f32,
            length * (k + 1) as f32 / steps as f32,
        );
        for w in profile.windows(2) {
            let ((u0, h0), (u1, h1)) = (w[0], w[1]);
            let q = [
                point(s0, u0, h0),
                point(s1, u0, h0),
                point(s1, u1, h1),
                point(s0, u1, h1),
            ];
            let mid = (u0 + u1) * 0.5;
            let out = point(s0, mid * 4.0, 0.5) - point(s0, 0.0, 0.5) + Vec3::new(0.0, 0.4, 0.0);
            quad_out(builder, CONCRETE, q, out, PARAPET);
        }
    }
}

/// A twin-arm lamp column on the median barrier.
fn lamp_column(builder: &mut MeshBuilder, surface: &Carriageway, s: f32, reach: f32) {
    let base = surface.point(s, 0.0, 1.0);
    let top = Vec3::new(base.x, base.y + 11.0, base.z);
    builder.tube(STEEL, base, top, 0.14, 0.09, 6, None);
    for side in [-1.0_f32, 1.0] {
        let p = surface.point(s, side * reach, 0.0);
        let head = Vec3::new(p.x, top.y + 0.4, p.z);
        builder.tube(STEEL, top, head, 0.06, 0.05, 4, None);
        builder.tube(
            STEEL,
            head,
            Vec3::new(head.x, head.y - 0.25, head.z),
            0.22,
            0.18,
            6,
            None,
        );
    }
}

/// Is a pier at `p` with footprint radius `r` clear of every street at grade?
fn clear_of_streets(network: &Network, road: &Road, p: Vec2, r: f32) -> bool {
    network.roads.iter().all(|other| {
        if other.id == road.id || other.layer != 0 {
            return true;
        }
        let line: Vec<Vec2> = other
            .centreline
            .points()
            .iter()
            .map(|q| Vec2::new(q.x, q.z))
            .collect();
        crate::math::distance_to_polyline(p, &line) > other.half_width() + r + 0.8
    }) && network
        .junctions
        .iter()
        .all(|junction| !crate::math::point_in_ring(p, &junction.ring))
}

fn piers(
    network: &Network,
    road: &Road,
    surface: &Carriageway,
    depth: f32,
    builder: &mut MeshBuilder,
) {
    let length = surface.length;
    let half = road.half_width();
    let span = if road.oneway { 25.0 } else { 30.0 };
    // Twin round columns under the webs, or one column under a hammerhead.
    let twin = !road.oneway && cell01(0x7A1, road.id as i32, 3, 11) < 0.5;
    let bottom_half = (half * 0.66).max(1.0) * 0.5 + 0.6;
    let mut s = span * 0.5;
    while s < length - 4.0 {
        for shift in [0.0_f32, 5.0, -5.0, 10.0, -10.0] {
            let at = (s + shift).clamp(2.0, length - 2.0);
            let (position, _) = surface.path.sample(at);
            let soffit = position.y - depth;
            if soffit < 1.6 {
                break;
            }
            let columns: Vec<f32> = if twin {
                vec![-bottom_half * 0.55, bottom_half * 0.55]
            } else {
                vec![0.0]
            };
            let points: Vec<Vec2> = columns
                .iter()
                .map(|u| plan(surface.point(at, *u, 0.0)))
                .collect();
            let radius = if road.oneway {
                0.75
            } else if twin {
                0.85
            } else {
                1.15
            };
            if !points
                .iter()
                .all(|p| clear_of_streets(network, road, *p, radius))
            {
                continue;
            }
            let cap = if twin || road.oneway { 0.0 } else { 1.7 };
            for p in &points {
                let foot = Vec3::new(p.x, -0.3, p.y);
                let head = Vec3::new(p.x, soffit - cap, p.y);
                builder.tube(CONCRETE, foot, head, radius, radius, 10, Some(PIER));
                // The drain down the column from the deck's gully.
                let t = surface.path.sample(at).1;
                let pipe = Vec3::new(
                    p.x + t.x * (radius + 0.12),
                    0.0,
                    p.y + t.y * (radius + 0.12),
                );
                builder.tube(
                    STEEL,
                    Vec3::new(pipe.x, -0.2, pipe.z),
                    Vec3::new(pipe.x, soffit, pipe.z),
                    0.08,
                    0.08,
                    4,
                    None,
                );
            }
            if cap > 0.0 {
                // The hammerhead: wide at the top under the box, narrowing down.
                let c = points[0];
                let t = surface.path.sample(at).1;
                let across = t.left_normal();
                let along = t * 1.2;
                let width = bottom_half + 1.2;
                let top = soffit;
                let low = soffit - cap;
                let corner = |u: f32, l: f32, y: f32| {
                    let q = c + across * u + along * l;
                    Vec3::new(q.x, y, q.y)
                };
                let narrow = radius + 0.2;
                for sa in [-1.0_f32, 1.0] {
                    // The two long faces.
                    let q = [
                        corner(-width, sa, top),
                        corner(width, sa, top),
                        corner(narrow, sa, low),
                        corner(-narrow, sa, low),
                    ];
                    let out = Vec3::new(along.x * sa, 0.0, along.y * sa);
                    quad_out(builder, CONCRETE, q, out, PIER);
                }
                for su in [-1.0_f32, 1.0] {
                    // The sloped undersides and the ends.
                    let q = [
                        corner(su * width, -1.0, top),
                        corner(su * width, 1.0, top),
                        corner(su * narrow, 1.0, low),
                        corner(su * narrow, -1.0, low),
                    ];
                    let out = Vec3::new(across.x * su, -0.6, across.y * su);
                    quad_out(builder, CONCRETE, q, out, PIER);
                }
            }
            break;
        }
        s += span;
    }
}

/// Lateral offset of a ramp's outer edge line from its centreline, as
/// `ramp_markings` paints it.
fn main_line_offset(ramp: &Road) -> f32 {
    ramp.section.motor_lanes_per_direction as f32 * ramp.section.motor_lane_width * 0.5 + 0.1
}

/// The taper where a ramp's deck meets the main deck, the hatched gore to the
/// nose, and the crash cushion on the nose.
fn draw_join(builder: &mut MeshBuilder, network: &Network, main: &Road, ramp: &Road, join: &Join) {
    let spec = network.spec;
    let surface = Carriageway::for_road(main, &spec);
    let ramp_surface = Carriageway::for_road(ramp, &spec);
    let half = main.half_width();
    let width = ramp.half_width() * 2.0;
    let (a, b) = (join.at_node, join.at_ramp);
    let steps = (((b - a).abs() / 3.0).ceil() as usize).max(1);
    let lift = level::ROAD;
    // The taper: the deck widens from nothing at the node to the ramp's width
    // where the ramp's own deck begins.
    let outward = |s: f32| {
        let t = ((s - a) / (b - a)).clamp(0.0, 1.0);
        width * t * t * (3.0 - 2.0 * t)
    };
    for k in 0..steps {
        let s0 = a + (b - a) * k as f32 / steps as f32;
        let s1 = a + (b - a) * (k + 1) as f32 / steps as f32;
        let e0 = join.side * half;
        let q = [
            surface.point(s0, e0, lift),
            surface.point(s1, e0, lift),
            surface.point(s1, join.side * (half + outward(s1)), lift),
            surface.point(s0, join.side * (half + outward(s0)), lift),
        ];
        asphalt_quad(builder, q);
        // Its fascia and soffit, so it is a deck and not a sheet.
        let lo = |p: Vec3| Vec3::new(p.x, p.y - 0.85, p.z);
        let outer = [q[3], q[2], lo(q[2]), lo(q[3])];
        let mid = surface.point((s0 + s1) * 0.5, join.side * (half + 8.0), 0.0)
            - surface.point((s0 + s1) * 0.5, join.side * half, 0.0);
        quad_out(builder, CONCRETE, outer, mid, FASCIA);
        let under = [lo(q[0]), lo(q[1]), lo(q[2]), lo(q[3])];
        quad_out(builder, CONCRETE, under, Vec3::new(0.0, -1.0, 0.0), SOFFIT);
    }
    // The taper's outer edge carries on the ramp's outer parapet.
    let outer_side = join.side;
    let edge_parapet_steps = steps;
    for k in 0..edge_parapet_steps {
        let s0 = a + (b - a) * k as f32 / steps as f32;
        let s1 = a + (b - a) * (k + 1) as f32 / steps as f32;
        let p = |s: f32, up: f32| {
            let q = surface.point(s, outer_side * (half + outward(s)), 0.0);
            Vec3::new(q.x, q.y + up, q.z)
        };
        let out = surface.point(s0, outer_side * (half + 10.0), 0.0)
            - surface.point(s0, outer_side * half, 0.0);
        quad_out(
            builder,
            CONCRETE,
            [p(s0, 0.0), p(s1, 0.0), p(s1, 1.1), p(s0, 1.1)],
            out,
            PARAPET,
        );
        quad_out(
            builder,
            CONCRETE,
            [p(s0, 0.0), p(s1, 0.0), p(s1, 1.1), p(s0, 1.1)],
            out * -1.0,
            PARAPET,
        );
        quad_out(
            builder,
            CONCRETE,
            [p(s0, 1.1), p(s1, 1.1), p(s1, 1.1), p(s0, 1.1)],
            Vec3::new(0.0, 1.0, 0.0),
            PARAPET,
        );
    }
    // The new lane's edge line, running out from the deck's own edge line to
    // the ramp's along the taper, so the lane the ramp becomes is marked from
    // the moment it opens.
    let main_line = main.section.half_carriageway() + 0.1;
    let ramp_line = half + width * 0.5 + main_line_offset(ramp);
    let line_at = |s: f32| {
        let t = ((s - a) / (b - a)).clamp(0.0, 1.0);
        main_line + (ramp_line - main_line) * t * t * (3.0 - 2.0 * t)
    };
    for k in 0..steps {
        let s0 = a + (b - a) * k as f32 / steps as f32;
        let s1 = a + (b - a) * (k + 1) as f32 / steps as f32;
        let w = network.spec.edge_line_width * 0.5;
        let p = |s: f32, d: f32| surface.point(s, join.side * (line_at(s) + d), level::PAINT);
        paint_quad(
            builder,
            "marking.white",
            [p(s0, -w), p(s1, -w), p(s1, w), p(s0, w)],
        );
    }
    // The gore: paved and hatched between the two deck edges up to the nose.
    let ramp_length = ramp_surface.length;
    let (u0, u1) = if join.diverge {
        (0.0, join.nose_ramp)
    } else {
        (join.nose_ramp, ramp_length)
    };
    let gore_steps = (((u1 - u0).abs() / 2.0).ceil() as usize).max(1);
    for k in 0..gore_steps {
        let v0 = u0 + (u1 - u0) * k as f32 / gore_steps as f32;
        let v1 = u0 + (u1 - u0) * (k + 1) as f32 / gore_steps as f32;
        let r0 = ramp_surface.point(v0, join.inner * ramp.half_width(), lift);
        let r1 = ramp_surface.point(v1, join.inner * ramp.half_width(), lift);
        let m0 = surface.point(project(main, plan(r0)).0, join.side * half, lift);
        let m1 = surface.point(project(main, plan(r1)).0, join.side * half, lift);
        asphalt_quad(builder, [m0, m1, r1, r0]);
        let lo = |p: Vec3| Vec3::new(p.x, p.y - 0.85, p.z);
        quad_out(
            builder,
            CONCRETE,
            [lo(m0), lo(m1), lo(r1), lo(r0)],
            Vec3::new(0.0, -1.0, 0.0),
            SOFFIT,
        );
        // A white chevron every 4 m, as GB 5768 marks a gore.
        if k % 2 == 0 {
            let paint = level::PAINT;
            let up = |p: Vec3| Vec3::new(p.x, p.y + paint, p.z);
            let mid0 = m0.lerp(r0, 0.5);
            let mid1 = m1.lerp(r1, 0.5);
            let stripe = [
                up(m0.lerp(mid0, 0.1)),
                up(mid1),
                up(mid1.lerp(r1, 0.15)),
                up(mid0.lerp(r0, 0.9)),
            ];
            paint_quad(builder, "marking.white", stripe);
        }
    }
    // The nose: a crash cushion where the two parapets meet.
    let nose = ramp_surface.point(join.nose_ramp, join.inner * (ramp.half_width() + 0.5), 0.0);
    let t = ramp_surface.path.sample(join.nose_ramp).1;
    let yaw = t.y.atan2(t.x);
    let (sin, cos) = yaw.sin_cos();
    let corner = |x: f32, z: f32| Vec2::new(nose.x + x * cos - z * sin, nose.z + x * sin + z * cos);
    let k = [
        corner(-1.0, -0.5),
        corner(1.0, -0.5),
        corner(1.0, 0.5),
        corner(-1.0, 0.5),
    ];
    for i in 0..4 {
        let (p, q) = (k[i], k[(i + 1) % 4]);
        let out = (p + q) * 0.5 - Vec2::new(nose.x, nose.z);
        quad_out(
            builder,
            CONCRETE,
            [
                Vec3::from_plan(p, nose.y),
                Vec3::from_plan(q, nose.y),
                Vec3::from_plan(q, nose.y + 0.9),
                Vec3::from_plan(p, nose.y + 0.9),
            ],
            Vec3::new(out.x, 0.0, out.y),
            CUSHION,
        );
    }
}
