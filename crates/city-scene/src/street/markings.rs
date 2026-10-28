//! What is painted on the street: longitudinal lane lines and their guide
//! zones, stop lines, crossings, left-turn waiting boxes, the GB 5768.3
//! drive-arrow stencils, and the approach-widening taper.
//!
//! Every dimension comes from [`crate::spec::JunctionSpec`], which is the
//! transcription of the source kernel's `junction-spec.js`; the two files
//! deliberately cannot disagree.

use urban::ModernRoadClass;

use crate::math::{Path, Vec2, Vec3};
use crate::mesh::MeshBuilder;
use crate::network::{Lane, LaneUse, Network, Road};
use crate::spec::{ARROW_LENGTH_MM, JunctionSpec, Movement};

use super::{Carriageway, Uvs, ribbon};

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
pub(super) struct Approach {
    pub(super) base: f32,
    pub(super) outward: f32,
    /// Station of the crossing's centre line.
    pub(super) crosswalk_centre: f32,
    /// Lateral span the stop line covers: from the median edge to the kerb, on
    /// the side this approach's traffic runs.
    pub(super) lateral_inner: f32,
    pub(super) lateral_outer: f32,
    /// Station of the stop line's near edge.
    pub(super) stop_near: f32,
    pub(super) stop_far: f32,
}

impl Approach {
    /// A station measured back from the junction edge, into the carriageway.
    fn back(&self, distance: f32) -> f32 {
        self.base - self.outward * distance
    }
}

/// The crossing ends of a road, nearest end first.  Empty for a road with no
/// real crossing at either end — a motorway, or a street whose ends are bends.
pub(super) fn approaches(road: &Road, spec: &JunctionSpec) -> Vec<Approach> {
    let length = road.carriageway.length();
    let half = road.half_width();
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
            crosswalk_centre: base - outward * spec.crosswalk_centre(road.class),
            lateral_inner: inner.min(outer),
            lateral_outer: inner.max(outer),
            stop_near: base
                - outward * (spec.stop_line_at(road.class) - spec.stop_line_width * 0.5),
            stop_far: base - outward * (spec.stop_line_at(road.class) + spec.stop_line_width * 0.5),
        });
    }
    result
}

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
        super::level::PAINT,
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
    straight_band(
        builder,
        material,
        surface,
        (across_from.min(across_to), across_from.max(across_to)),
        (from.min(to), from.max(to)),
        lift,
        None,
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
pub(super) fn dashed_stripe(
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
        super::level::PAINT,
        None,
        Uvs::Along(anchor),
    );
}

/// A zebra crossing: one quad with a striped alpha texture.  `U` runs the real
/// crossing width, so the 1.05 m bar pitch comes out exact at any road class.
pub(super) fn crosswalk_band(
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
    straight_band(
        builder,
        "marking.crosswalk",
        surface,
        (inner, outer),
        (centre - depth * 0.5, centre + depth * 0.5),
        super::level::CROSSWALK,
        Some(centre),
    );
}

/// A short band across the road, from lateral `across.0` to `across.1` and
/// from station `along.0` to `along.1`, laid square to the road at its middle
/// station in one straight piece: a crossing or a stop line is straight paint
/// even where the street bends under it, and following the bend would twist a
/// wide one into a wedge. `anchor` gives it `V` texture coordinates along the
/// road from that station, as [`Uvs::Along`] does.
fn straight_band(
    builder: &mut MeshBuilder,
    material: &str,
    surface: &Carriageway,
    across: (f32, f32),
    along: (f32, f32),
    lift: f32,
    anchor: Option<f32>,
) {
    let middle = ((along.0 + along.1) * 0.5).clamp(0.0, surface.length);
    let tangent = surface.path.tangent_at(middle);
    let a = surface.point(middle, across.0, lift);
    let b = surface.point(middle, across.1, lift);
    let at = |p: Vec3, station: f32| {
        let d = station - middle;
        Vec3::new(p.x + tangent.x * d, p.y, p.z + tangent.y * d)
    };
    let (pa, pb, qb, qa) = (
        at(a, along.0),
        at(b, along.0),
        at(b, along.1),
        at(a, along.1),
    );
    match anchor {
        None => builder.quad(material, pa, pb, qb, qa, None),
        Some(anchor) => {
            let (v1, v0) = (along.0 - anchor, along.1 - anchor);
            let width = across.1 - across.0;
            builder.quad_uv(
                material,
                pa,
                pb,
                qb,
                qa,
                [(0.0, v1), (width, v1), (width, v0), (0.0, v0)],
                None,
            );
        }
    }
}

pub(super) fn road_markings(
    network: &Network,
    road: &Road,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
    tapers: &[super::Gap],
) {
    let length = road.carriageway.length();
    if length < 2.0 {
        return;
    }
    // A motorway keeps its lane markings but has no stop lines, crosswalks or
    // signals; an elevated road keeps everything except the pedestrian realm it
    // has no kerb for.  Split the three conditions apart explicitly rather than
    // short-circuiting, or a viaduct silently loses its paint.
    if road.oneway {
        ramp_markings(network, road, builder, spec);
        return;
    }
    if road.is_motorway() {
        motorway_markings(road, builder, spec, tapers);
        return;
    }
    if road.layer != 0 {
        elevated_markings(road, builder, spec);
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let half = road.half_width();
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

    // A crosswalk at every end people cross, and a stop line at every end
    // traffic stops: at an unsignalised junction the street that runs through
    // has its zebra but no stop line.
    for (base, outward, walked) in [
        (0.0_f32, -1.0_f32, road.walk_start),
        (length, 1.0_f32, road.walk_end),
    ] {
        let depth = spec.crosswalk_depth(road.class);
        let centre = base - outward * spec.crosswalk_centre(road.class);
        if !walked || centre - depth * 0.5 < -0.01 || centre + depth * 0.5 > length + 0.01 {
            continue;
        }
        crosswalk_band(builder, &surface, centre, half, depth);
    }
    for approach in approaches(road, spec) {
        let centre = approach.crosswalk_centre;
        let depth = spec.crosswalk_depth(road.class);
        if centre - depth * 0.5 < -0.01 || centre + depth * 0.5 > length + 0.01 {
            continue;
        }
        transverse_stripe(
            builder,
            "marking.white",
            &surface,
            approach.lateral_inner,
            approach.lateral_outer,
            approach.stop_near,
            approach.stop_far,
            super::level::PAINT,
        );
    }

    // Left-turn waiting box, in front of a lane that only turns left at a
    // signalised junction (see `waiting_box_lane`).
    for at_start in [true, false] {
        let node = if at_start {
            road.from_node
        } else {
            road.to_node
        };
        if let Some(lane) = waiting_box_lane(network, road, node) {
            waiting_box(network, builder, spec, lane);
        }
    }

    // Design-spec drive arrows, one per motor lane, placed clear of the
    // crosswalk band.  The arrow's allowed set comes from the connector graph,
    // so the painted marking and the legal movement are the same fact.
    // None into a roundabout: its entries all go round, there is no lane to
    // choose (`docs/city-visual-target.md` §8).
    let into_roundabout = |node: u32| network.junction(node).is_some_and(|j| j.roundabout);
    for lane in network.lanes.iter().filter(|lane| {
        lane.road == road.id
            && !into_roundabout(lane.to_node)
            && lane.path.length() > spec.arrow.tip_gap + 1.5 + 2.4
            && matches!(
                lane.use_kind,
                LaneUse::Through | LaneUse::LeftTurn | LaneUse::RightTurn
            )
    }) {
        drive_arrow(builder, lane, spec);
    }
}

/// Does any approach into `node` get a left-turn waiting box?  The box owns the
/// ground it stands on, so a junction with one never also gets a hatched grid.
pub(super) fn node_has_waiting_box(network: &Network, node: u32) -> bool {
    network
        .roads
        .iter()
        .filter(|road| road.from_node == node || road.to_node == node)
        .any(|road| waiting_box_lane(network, road, node).is_some())
}

/// The lane of `road` arriving at `node` that gets a left-turn waiting box: a
/// lane that turns left and nothing else, at a signalised junction, where the
/// left turn waits for the oncoming straight flow (GB 5768.3 §4.6). A lane
/// that may also go straight cannot wait in a box for a left turn, and a
/// roundabout or a priority junction has no left-turn phase to wait for.
pub(super) fn waiting_box_lane<'a>(
    network: &'a Network,
    road: &Road,
    node: u32,
) -> Option<&'a Lane> {
    let signalised = network
        .junctions
        .iter()
        .find(|junction| junction.node == node)
        .is_some_and(|junction| {
            junction.kind == urban::JunctionKind::Signalized && !junction.roundabout
        });
    if !signalised {
        return None;
    }
    network.lanes.iter().find(|lane| {
        lane.road == road.id
            && lane.to_node == node
            && lane.index == 0
            && lane.use_kind != LaneUse::Bike
            && lane.allowed == [Movement::Left]
    })
}

/// The left-turn waiting box (左转弯待转区线, GB 5768.3 §4.6, figure 9).
///
/// A lane that turns left at a signal moves into the junction while the
/// straight traffic beside it has green, and waits there for its own phase.
/// The box starts at the junction edge, just past the crossing, and follows
/// the lane's own left turn, so it curves to the left the way the cars in it
/// will; it reaches no more than two fifths of the way round the turn, short
/// of the oncoming straight flow. Its sides are white dashes (500 mm on,
/// 500 mm off, 150 mm wide), its front end a solid line, and a left-turn
/// arrow stands in it: the 2009 standard replaced the box's painted words by
/// the arrow.
pub(super) fn waiting_box(
    network: &Network,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
    lane: &Lane,
) {
    let Some(turn) = network
        .connectors
        .iter()
        .find(|c| c.from_lane == lane.id && c.movement == Movement::Left)
    else {
        return;
    };
    let depth = spec.waiting_box.depth.min(turn.path.length() * 0.4);
    if depth < 4.0 {
        return;
    }
    let mut surface = Carriageway::on_path(turn.path.clone(), lane.width * 0.5);
    // Inside the junction the crown has flattened out.
    surface.rise = 0.0;
    let half = lane.width * 0.5 - 0.1;
    const DASH: f32 = 0.5;
    const LINE: f32 = 0.15;
    for side in [-1.0_f32, 1.0] {
        let mut at = 0.0;
        while at + DASH <= depth {
            stripe(
                builder,
                "marking.white",
                &surface,
                side * half,
                at,
                at + DASH,
                LINE,
            );
            at += 2.0 * DASH;
        }
    }
    transverse_stripe(
        builder,
        "marking.white",
        &surface,
        -half - LINE * 0.5,
        half + LINE * 0.5,
        depth - spec.stop_line_width,
        depth,
        super::level::PAINT,
    );
    // The arrow, in the middle of the box, turned with it.
    let length = spec.arrow.footprint.min(depth - 1.5);
    if length < 2.4 {
        return;
    }
    let scale = length / ARROW_LENGTH_MM;
    let tail = (depth - length) * 0.5;
    for outline in crate::spec::arrow_polygons(&[Movement::Left]) {
        if outline.len() < 3 {
            continue;
        }
        let ring: Vec<Vec2> = outline
            .iter()
            .map(|(lateral, forward)| Vec2::new(lateral * scale, forward * scale))
            .collect();
        for [i, j, k] in crate::math::triangulate(&ring) {
            let point = |index: usize| {
                let (lateral, forward) = outline[index];
                surface.point(
                    tail + forward * scale,
                    lateral * scale,
                    super::level::PAINT + 0.002,
                )
            };
            builder.tri_flat("marking.white", point(i), point(j), point(k), None);
        }
    }
}

fn motorway_markings(
    road: &Road,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
    tapers: &[super::Gap],
) {
    let length = road.carriageway.length();
    if length < 2.0 {
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let median_half = section.median_metres * 0.5;
    // The lanes start past the marginal strip by the median; the lines sit on
    // the lane boundaries the lanes themselves are laid on.
    let first_lane = median_half + section.inner_shoulder;
    for direction in [1.0_f32, -1.0] {
        for index in 1..section.motor_lanes_per_direction {
            let offset = direction * (first_lane + index as f32 * section.motor_lane_width);
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
        // The left edge by the median is yellow on a Chinese expressway (GB
        // 5768.3), the right edge white, at the start of the hard shoulder.
        stripe(
            builder,
            "marking.yellow",
            &surface,
            direction * (first_lane - 0.15),
            0.0,
            length,
            spec.yellow_line_width,
        );
        let kerb_lane =
            first_lane + section.motor_lanes_per_direction as f32 * section.motor_lane_width;
        // Solid, except where the deck widens into a ramp: there the edge line
        // is the broken line between the through lanes and the lane that
        // leaves (or joins), as GB 5768.3 marks a deceleration lane.
        let (solid, broken) = split_by(length, tapers, direction);
        for (from, to) in solid {
            stripe(
                builder,
                "marking.white",
                &surface,
                direction * (kerb_lane + 0.1),
                from,
                to,
                spec.edge_line_width,
            );
        }
        for (from, to) in broken {
            dashed_stripe(
                builder,
                "marking.dashed-3-5",
                &surface,
                direction * (kerb_lane + 0.1),
                from,
                to,
                spec.edge_line_width * 2.0,
                from,
            );
        }
    }
    // Crash barriers on the verges of an expressway at grade; on a viaduct the
    // parapets and the median barrier are the structure's (`viaduct`).
    if section.median_metres > 0.5 && road.layer == 0 {
        for side in [-1.0_f32, 1.0] {
            let cap = super::offset_path(&surface, side * (road.half_width() - 0.42));
            let cap_surface = Carriageway::on_path(cap, 0.2);
            ribbon(
                builder,
                "barrier.concrete",
                &cap_surface,
                -0.18,
                0.18,
                4.0,
                (length - 4.0).max(4.0),
                super::level::ROAD + 0.85,
                None,
                Uvs::None,
            );
            super::sweep(
                builder,
                "barrier.concrete",
                &cap_surface,
                -0.18,
                0.18,
                4.0,
                (length - 4.0).max(4.0),
                super::level::ROAD + 0.85,
            );
        }
    }
}

/// A stretch of a road, `(from, to)` stations.
pub(super) type Span = (f32, f32);

/// `[0, length]` split into the pieces outside and inside the `tapers` on the
/// `side` of the centreline.
pub(super) fn split_by(length: f32, tapers: &[super::Gap], side: f32) -> (Vec<Span>, Vec<Span>) {
    let mut inside: Vec<Span> = tapers
        .iter()
        .filter(|gap| gap.side == side)
        .map(|gap| (gap.from.clamp(0.0, length), gap.to.clamp(0.0, length)))
        .filter(|(a, b)| b - a > 0.5)
        .collect();
    inside.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut outside = Vec::new();
    let mut cursor = 0.0_f32;
    for (a, b) in &inside {
        if *a > cursor + 0.05 {
            outside.push((cursor, *a));
        }
        cursor = cursor.max(*b);
    }
    if cursor < length - 0.05 {
        outside.push((cursor, length));
    }
    (outside, inside)
}

/// Lane paint on a one-way ramp: white edge lines either side of its lane(s),
/// and where it meets a signalised terminal at grade, the stop line across
/// the whole carriageway, the crossing, and the arrows of the turns it may
/// make there.
fn ramp_markings(network: &Network, road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    let length = road.carriageway.length();
    if length < 2.0 {
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let count = section.motor_lanes_per_direction;
    let lanes_half = count as f32 * section.motor_lane_width * 0.5;
    let mark_from = road.marking_start.clamp(0.0, length);
    let mark_to = (length - road.marking_end).clamp(0.0, length);
    for side in [-1.0_f32, 1.0] {
        stripe(
            builder,
            "marking.white",
            &surface,
            side * (lanes_half + 0.1),
            mark_from,
            mark_to,
            spec.edge_line_width,
        );
    }
    for index in 1..count {
        let offset = -lanes_half + index as f32 * section.motor_lane_width;
        dashed_stripe(
            builder,
            "marking.dashed-6-9",
            &surface,
            offset,
            mark_from,
            mark_to,
            spec.white_line_width,
            mark_from,
        );
    }
    let half = road.half_width();
    for approach in approaches(road, spec) {
        let centre = approach.crosswalk_centre;
        let depth = spec.crosswalk_depth(road.class);
        if centre - depth * 0.5 < -0.01 || centre + depth * 0.5 > length + 0.01 {
            continue;
        }
        crosswalk_band(builder, &surface, centre, half, depth);
        // Traffic arrives only at the terminal end of an off-ramp; an on-ramp's
        // terminal end is where it is entered, and has no stop line.
        let arriving = if approach.outward > 0.0 {
            network
                .lanes
                .iter()
                .any(|l| l.road == road.id && l.to_node == road.to_node)
        } else {
            network
                .lanes
                .iter()
                .any(|l| l.road == road.id && l.to_node == road.from_node)
        };
        if arriving {
            transverse_stripe(
                builder,
                "marking.white",
                &surface,
                -(half - 0.2),
                half - 0.2,
                approach.stop_near,
                approach.stop_far,
                super::level::PAINT,
            );
        }
    }
    // None into a roundabout: its entries all go round, there is no lane to
    // choose (`docs/city-visual-target.md` §8).
    let into_roundabout = |node: u32| network.junction(node).is_some_and(|j| j.roundabout);
    for lane in network.lanes.iter().filter(|lane| {
        lane.road == road.id
            && !into_roundabout(lane.to_node)
            && lane.path.length() > spec.arrow.tip_gap + 1.5 + 2.4
            && network.junction(lane.to_node).is_some()
    }) {
        drive_arrow(builder, lane, spec);
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
            direction * (road.half_width() - 0.35),
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

pub(super) fn guide_zone_length(usable: f32, spec: &JunctionSpec) -> f32 {
    if usable < 44.0 {
        0.0
    } else {
        spec.guide_zone.min((usable - 24.0) * 0.5)
    }
}

/// A design-spec GB 5768.3 drive arrow.
///
/// # Two things the previous version got wrong
///
/// * **The size.**  The city kernel draws the stencil at the approach's
///   *reserved* footprint: `length = min(footprint, total - tipGap - 1.5)` with
///   `footprint = 5` and `tipGap = 13`.  The millimetre stencil table is the GB
///   *shape* — shaft, head, barbs — and the whole stencil scales by
///   `length / 3.05`, so at the full footprint the arrow is 5 m long with a
///   740 mm head.  That is what the reference city renders and what reads at
///   the distance a queueing driver sees it; a strict 3.05 m arrow is the
///   *editor's* minimum and disappears from a moving car.  A road too short
///   for the full 5 m shortens the stencil (never below 2.4 m) instead of
///   dropping it.
/// * **The placement.**  The tip stands `tip_gap` (13 m) back from the lane's
///   end — the junction edge — so a queue forms behind the arrow rather than on
///   it, and the arrow sits in front of the crosswalk band, never on it.
///
/// A lane that carries no legal movement gets no arrow at all: painting a
/// phantom straight on an approach that leads nowhere is exactly the lie a
/// marking must never tell.
///
/// # The frame and the fill
///
/// The lateral axis is the *left* normal, which is the frame the stencil
/// tables are authored against: positive `lateral` is the driver's left, so a
/// right-turn stencil's head (authored at negative `lateral`) lands on the
/// driver's right, the side [`crate::spec::classify_movement`] calls a right
/// turn.  The outline is **ear-clipped**, never fanned: a GB arrow is concave
/// at the shaft-head junction and a fan emits triangles outside the polygon —
/// the "white blob" failure.  [`crate::math::triangulate`] also normalises the
/// winding, so mirrored and unmirrored stencils face the same way.  The
/// stencil follows the crowned surface, so it neither floats at the crown nor
/// buries its tail in the gutter.
pub(super) fn drive_arrow(builder: &mut MeshBuilder, lane: &Lane, spec: &JunctionSpec) {
    if lane.allowed.is_empty() {
        return;
    }
    let total = lane.path.length();
    let length = spec.arrow.footprint.min(total - spec.arrow.tip_gap - 1.5);
    if length < 2.4 {
        return;
    }
    // Millimetres to metres, along the stencil's own axis, at the reserved
    // footprint's scale.
    let scale = length / ARROW_LENGTH_MM;
    let tail = total - spec.arrow.tip_gap - length;
    let surface = Carriageway::on_path(lane.path.clone(), lane.width.max(0.5) * 0.5);
    for outline in crate::spec::arrow_polygons(&lane.allowed) {
        if outline.len() < 3 {
            continue;
        }
        let ring: Vec<Vec2> = outline
            .iter()
            .map(|(lateral, forward)| Vec2::new(lateral * scale, forward * scale))
            .collect();
        for [i, j, k] in crate::math::triangulate(&ring) {
            let point = |index: usize| {
                let (lateral, forward) = outline[index];
                let station = tail + forward * scale;
                surface.point(station, lateral * scale, super::level::PAINT + 0.002)
            };
            builder.tri_flat("marking.white", point(i), point(j), point(k), None);
        }
    }
}

/// The approach widening taper (进口道拓宽渐变).
///
/// The lane line that parts an approach widening's queue lane from the lane it
/// grew beside: solid over the widened length, the way a lane is marked inside
/// the guide zone before a stop line (`docs/city-visual-target.md` §8). The
/// taper itself needs no paint of its own; the edge line follows the kerb out.
pub(super) fn widening_markings(road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    if road.layer != 0 || !road.widen.iter().any(|w| w.is_some()) {
        return;
    }
    let length = road.carriageway.length();
    // The line stands where the motor lanes ended before the widening, so it is
    // drawn on the carriageway as it was, not moved out with the kerb.
    let mut fixed = Carriageway::for_road(road, spec);
    fixed.widen = Default::default();
    let edge = road.section.half_carriageway();
    for approach in approaches(road, spec) {
        let w = road.widen[usize::from(approach.outward > 0.0)];
        if !w.is_some() {
            continue;
        }
        let a = approach.back(spec.stop_line_at(road.class) + spec.stop_line_width);
        let b = approach.back(w.hold);
        let (from, to) = (a.min(b).clamp(0.0, length), a.max(b).clamp(0.0, length));
        stripe(
            builder,
            "marking.white",
            &fixed,
            approach.outward * edge,
            from,
            to,
            spec.white_line_width,
        );
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
    // One guide per arm pair: several lanes feed the same turn, and drawing each
    // one produced a fan of crossing ticks. Only left turns cross the box, so only
    // they are guided.
    let mut drawn: std::collections::HashSet<(u32, i32, i32)> = std::collections::HashSet::new();
    // Guides are painted only through a signalised box.
    let signalised: std::collections::HashSet<u32> = network
        .junctions
        .iter()
        .filter(|j| j.kind == urban::JunctionKind::Signalized)
        .map(|j| j.node)
        .collect();
    for connector in &network.connectors {
        if connector.movement != Movement::Left || !signalised.contains(&connector.node) {
            continue;
        }
        if connector.turn_degrees.abs() < 25.0 {
            continue;
        }
        if connector.from_lane.ends_with("bike") || connector.to_lane.ends_with("bike") {
            continue;
        }
        {
            let road_of = |lane: &str| {
                lane.split('/')
                    .nth(1)
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(-1)
            };
            let key = (
                connector.node,
                road_of(&connector.from_lane),
                road_of(&connector.to_lane),
            );
            if !drawn.insert(key) {
                continue;
            }
        }
        // A connector's *shape* is the detail, so this is the one place the
        // fine subdivision step is right.
        let path = connector.path.densified(crate::math::FINE_RESAMPLE_METRES);
        if path.length() < 8.0 {
            continue;
        }
        let Some(junction) = network.junction(connector.node) else {
            continue;
        };
        // A hatched no-stopping box owns the middle of the junction.
        if junction.ports.len() >= 4 && junction.radius >= 16.0 {
            continue;
        }
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
            spec.white_line_width * 1.5,
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
