//! What a street is *made of*: the crowned carriageway, the kerbs and their
//! dropped-kerb ramps, the median and its hazard nose, the junction box and
//! its corner paving, and the roundabout variant of all of it.
//!
//! Markings live in [`super::markings`]; this module owns every surface a
//! driver or pedestrian actually touches.

use urban::JunctionKind;

use crate::math::{Rng, Vec2, Vec3};
use crate::mesh::MeshBuilder;
use crate::network::{Junction, Network, Road};
use crate::spec::JunctionSpec;

use super::{Carriageway, WALK_Y, ribbon, sweep};

/// Length of the dropped kerb either side of a crossing's centre line.  GB 50763
/// wants at least 2.5 m of dropped kerb on each side of a crossing.
const RAMP_HALF: f32 = 2.0;
/// Set-back of the tactile strip behind the kerb line.
const TACTILE_SETBACK: f32 = 0.18;
/// Length of the flat approach kerb either side of the dropped section.
const KERB_FLAT: f32 = 0.55;
/// Depth of a dropped-kerb ramp back from the kerb line, where it reaches the
/// footway's full height.  The block plate stops short of it
/// (`buildings::PLATE_UNDER_PAVING`), or it would show through the slope.
pub(crate) const RAMP_DEPTH: f32 = 1.35;

/// Station ranges where the kerb is dropped for a crossing, as `(from, to)`.
fn dropped_kerb(
    road: &Carriageway,
    spec: &JunctionSpec,
    crossing_start: bool,
    crossing_end: bool,
) -> Vec<(f32, f32)> {
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

pub(super) fn road_surface(
    network: &Network,
    road: &Road,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
    tapers: &[super::Gap],
) {
    if road.carriageway.length() < 1.0 {
        return;
    }
    let section = road.section;
    let surface = Carriageway::for_road(road, spec);
    let length = surface.length;
    let half = road.half_width();

    // The carriageway proper.
    ribbon(
        builder,
        "asphalt",
        &surface,
        -half,
        half,
        0.0,
        length,
        super::level::ROAD,
        None,
        super::Uvs::World,
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
                super::level::ROAD,
                None,
                super::Uvs::World,
            );
        }
    }
    if section.shoulder_width > 0.0 {
        for side in [-1.0_f32, 1.0] {
            let inner = side * (motor_edge + section.bike_lane_width);
            let outer = inner + side * section.shoulder_width;
            // Where the deck widens into a ramp the shoulder is lane.
            let (pieces, _) = super::markings::split_by(length, tapers, side);
            for (from, to) in pieces {
                ribbon(
                    builder,
                    "asphalt.cycle",
                    &surface,
                    inner.min(outer),
                    inner.max(outer),
                    from,
                    to,
                    super::level::ROAD,
                    None,
                    super::Uvs::World,
                );
            }
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
    let cut = dropped_kerb(&surface, spec, road.walk_start, road.walk_end);
    // Near each junction the footway is laid round the junction's other
    // carriageways, never over them: where two streets meet too acutely for
    // their footways to part before the box, one footway would otherwise run
    // out across the other street.
    // Each end's zone reaches as far as its footways still lie over the
    // junction's other carriageways: further than `JUNCTION_ZONE_M` where two
    // streets part at a narrow angle.
    let reach = half + section.sidewalk_metres;
    let zone_at = |at_start: bool| -> (f32, Vec<Vec<Vec2>>) {
        let mut zone = super::JUNCTION_ZONE_M;
        loop {
            let rings = super::road_near_within(network, road, at_start, zone + 20.0);
            let edge = zone.min(length);
            let station = if at_start { edge } else { length - edge };
            let clear = [-reach, reach].iter().all(|lateral| {
                let p = surface.point(station, *lateral, 0.0);
                let p = Vec2::new(p.x, p.z);
                !rings.iter().any(|ring| crate::math::point_in_ring(p, ring))
            });
            if clear || zone >= length * 0.5 {
                return (zone, rings);
            }
            zone = (zone * 1.5).min(length * 0.5);
        }
    };
    let ((zone_start, mut near_start), (zone_end, mut near_end)) = (zone_at(true), zone_at(false));
    // A street shorter than its two junction zones is near both junctions
    // all along.
    if length < zone_start + zone_end {
        let both: Vec<Vec<Vec2>> = near_start.iter().chain(&near_end).cloned().collect();
        near_start = both.clone();
        near_end = both;
    }
    let (zone_start, zone_end) = (zone_start.min(length * 0.5), zone_end.min(length * 0.5));
    // Round a bend tighter than the footway's own reach, the footway on the
    // inside folds back over the carriageway: there the street's own
    // carriageway is cut out of it too, the carriageway winning as everywhere.
    // And wherever another street runs beside this one, its carriageway too.
    let mut own: Vec<Vec<Vec2>> = super::road_beside(network, road);
    near_start.extend(own.iter().cloned());
    near_end.extend(own.iter().cloned());
    let tight = surface.stations(0.0, length, 2.0).windows(2).any(|w| {
        let turn = surface
            .path
            .tangent_at(w[0])
            .dot(surface.path.tangent_at(w[1]))
            .clamp(-1.0, 1.0)
            .acos();
        turn > 1e-4 && (w[1] - w[0]) / turn < reach + 1.0
    });
    if tight {
        let run = surface.stations(0.0, length, 2.0);
        let mut ring: Vec<Vec2> = run
            .iter()
            .map(|s| {
                let p = surface.point(*s, half - 0.05, 0.0);
                Vec2::new(p.x, p.z)
            })
            .collect();
        ring.extend(run.iter().rev().map(|s| {
            let p = surface.point(*s, -(half - 0.05), 0.0);
            Vec2::new(p.x, p.z)
        }));
        own.push(ring);
        near_start.extend(own.iter().cloned());
        near_end.extend(own.iter().cloned());
    }
    let mut footway: Vec<(f32, f32, &[Vec<Vec2>])> = Vec::new();
    // Each piece reaches 5 cm under the ramp beside it, so a clipped footway
    // and a clipped ramp overlap instead of meeting edge to edge in a crack.
    let overlap = |(from, to): (f32, f32)| {
        let lo = if from > 0.0 {
            (from - 0.05).max(0.0)
        } else {
            from
        };
        let hi = if to < length {
            (to + 0.05).min(length)
        } else {
            to
        };
        (lo, hi)
    };
    for (from, to) in remaining(length, &cut).into_iter().map(overlap) {
        let edges = [
            from,
            zone_start.clamp(from, to),
            (length - zone_end).clamp(from, to),
            to,
        ];
        let clips: [&[Vec<Vec2>]; 3] = [&near_start, &own, &near_end];
        for k in 0..3 {
            if edges[k + 1] - edges[k] > 0.05 {
                footway.push((edges[k], edges[k + 1], clips[k]));
            }
        }
    }
    for side in [-1.0_f32, 1.0] {
        // The footway runs from the back of the kerb top; the kerb top from the
        // carriageway edge, exactly as round a junction corner.
        let kerb_from = side * half;
        let kerb_to = side * (half + KERB_TOP);
        let inner = kerb_to;
        let outer = side * (half + section.sidewalk_metres);
        for (from, to, clips) in &footway {
            super::ribbon_clipped(
                builder,
                "sidewalk",
                &surface,
                inner.min(outer),
                inner.max(outer),
                *from,
                *to,
                super::level::KERB,
                super::Uvs::World,
                clips,
            );
        }
        // Kerb: a capping ribbon plus its full-height face.  Drawn in the same
        // pieces as the footway, because a dropped kerb is a kerb that is *not
        // there* — left continuous it pokes through the ramp and the crossing
        // reads as a trip hazard modelled in concrete.
        for (from, to, clips) in &footway {
            super::ribbon_clipped(
                builder,
                "kerb",
                &surface,
                kerb_from.min(kerb_to),
                kerb_from.max(kerb_to),
                *from,
                *to,
                super::level::KERB + 0.002,
                super::Uvs::None,
                clips,
            );
            super::sweep_clipped(
                builder,
                "kerb",
                &surface,
                kerb_from.min(kerb_to),
                kerb_from.max(kerb_to),
                *from,
                *to,
                super::level::KERB,
                clips,
            );
        }
        // And the ramp where the kerb is dropped, with its tactile strip, and
        // the footway behind the ramp at full height out to the back edge.
        for range in &cut {
            // A crossing is always next to a junction: the nearer one's road.
            let clips: &[Vec<Vec2>] = if range.0 < length - range.1 {
                &near_start
            } else {
                &near_end
            };
            kerb_ramp(builder, &surface, side, *range, spec, clips);
            let back = side * (half + RAMP_DEPTH);
            if (outer - back) * side > 0.05 {
                super::ribbon_clipped(
                    builder,
                    "sidewalk",
                    &surface,
                    back.min(outer),
                    back.max(outer),
                    range.0,
                    range.1,
                    super::level::KERB,
                    super::Uvs::World,
                    clips,
                );
            }
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
    clips: &[Vec<Vec2>],
) {
    let half = surface.half;
    let kerb = side * half;
    let back = side * (half + RAMP_DEPTH);
    let middle = (from + to) * 0.5;
    // Three longitudinal stations: flat kerb, dropped, flat kerb.
    let lo = (from + 0.4).min(middle - 0.2);
    let hi = (to - 0.4).max(middle + 0.2);
    if hi <= lo {
        return;
    }
    let ground = super::level::ROAD + 0.02;
    // A sloped quad that reaches another street's carriageway is cut back
    // off it: that ground is the other street's.
    let on_road = |points: &[Vec3]| {
        let plan: Vec<Vec2> = points.iter().map(|p| Vec2::new(p.x, p.z)).collect();
        clips.iter().any(|ring| {
            plan.iter().any(|p| crate::math::point_in_ring(*p, ring))
                || ring.iter().any(|p| crate::math::point_in_ring(*p, &plan))
        })
    };
    let quad = |builder: &mut MeshBuilder, s0: f32, y0: f32, s1: f32, y1: f32| {
        let a = surface.point(s0, kerb, y0);
        let b = surface.point(s0, back, super::level::KERB);
        let c = surface.point(s1, back, super::level::KERB);
        let d = surface.point(s1, kerb, y1);
        if on_road(&[a, b, c, d]) {
            // Cut, keeping the slope: each piece's height is read off the
            // quad it came from.
            super::quad_clipped(builder, "sidewalk", [a, b, c, d], clips);
            return;
        }
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
    quad(builder, from, super::level::KERB, lo, ground);
    quad(builder, lo, ground, hi, ground);
    quad(builder, hi, ground, to, super::level::KERB);

    // The tactile strip, set back from the kerb, with six raised ribs.
    let inner = side * (half + KERB_TOP + TACTILE_SETBACK);
    let outer = side * (half + KERB_TOP + TACTILE_SETBACK + spec.tactile_width);
    let start = (middle - 1.6).max(lo);
    let end = (middle + 1.6).min(hi);
    if end - start < 0.4 {
        return;
    }
    let ribs = 6;
    // The warning surface itself, so the strip reads as one band from a car...
    super::ribbon_clipped(
        builder,
        "kerb",
        surface,
        inner.min(outer),
        inner.max(outer),
        start,
        end,
        super::level::TACTILE - 0.004,
        super::Uvs::None,
        clips,
    );
    // ... and the raised ribs on top of it, which is what a tactile paving
    // surface actually is and the only part of it visible from a moving car.
    for rib in 0..ribs {
        let offset = inner + (outer - inner) * (rib as f32 + 0.5) / ribs as f32;
        super::ribbon_clipped(
            builder,
            "kerb",
            surface,
            offset - 0.028,
            offset + 0.028,
            start,
            end,
            super::level::TACTILE,
            super::Uvs::None,
            clips,
        );
    }
}

/// A roundabout entry's splitter island (导流岛, CJJ 152 §4.6; the Xinjiang
/// town-centre aerial): a raised, kerbed, planted wedge on the arm's axis from
/// just inside the arm's end to just outside the give-way line, parting the
/// entering lanes from the leaving ones. It starts as wide as the arm's median
/// (a point on an undivided arm) and widens toward the ring; its nose is
/// painted in the yellow and black hazard bands. Returns the island's
/// half-width where the give-way triangle stands, 6 m short of the ring.
fn splitter_island(
    builder: &mut MeshBuilder,
    mid: Vec2,
    dir: Vec2,
    median: f32,
    ring: Option<Vec2>,
) -> f32 {
    let Some(ring) = ring else {
        return 0.0;
    };
    let (start, end) = (0.6, (ring - mid).dot(dir));
    if end - start < 5.0 {
        return 0.0;
    }
    let side = Vec2::new(-dir.y, dir.x);
    let (nose, base) = (median.max(0.35), median.max(2.2));
    let half = |s: f32| nose + (base - nose) * ((s - start) / (end - start)).clamp(0.0, 1.0);
    // The outline, inset by `by`: up one side, across the base, down the
    // other, round the nose on a half circle.
    let outline = |by: f32| -> Vec<Vec2> {
        let (s0, s1) = (start + nose, end - by);
        let mut ring = Vec::new();
        for k in 0..=6 {
            let s = s0 + (s1 - s0) * k as f32 / 6.0;
            ring.push(mid + dir * s + side * (half(s) - by));
        }
        for k in (0..=6).rev() {
            let s = s0 + (s1 - s0) * k as f32 / 6.0;
            ring.push(mid + dir * s - side * (half(s) - by));
        }
        let r = (nose - by).max(0.05);
        for k in 1..8 {
            let angle = std::f32::consts::PI * k as f32 / 8.0;
            ring.push(mid + dir * (s0 - angle.sin() * r) - side * (angle.cos() * r));
        }
        ring
    };
    let edge = outline(0.0);
    let nose_from = edge.len() - 8;
    for index in 0..edge.len() {
        let next = (index + 1) % edge.len();
        // The nose, its last metre of sides included, banded for hazard.
        let near_nose = index >= nose_from - 1 || index == 0 || next == 0;
        let material = match (near_nose, index % 2) {
            (true, 0) => "marking.yellow",
            (true, _) => "asphalt.pavement",
            _ => "kerb",
        };
        builder.wall(
            material,
            edge[index],
            edge[next],
            super::level::ROAD,
            super::level::KERB,
            None,
        );
    }
    // The kerb's top a band round the bed, not a plate under it: one surface
    // over each point, or the two trade places in the depth buffer.
    let bed = outline(0.25);
    for piece in crate::buildings::cut_away(edge.clone(), std::slice::from_ref(&bed)) {
        builder.ground_uv("kerb", &piece, super::level::KERB, None);
    }
    builder.ground_uv("median.plant", &bed, super::level::MEDIAN + 0.10, None);
    half(end - 6.0)
}

/// The median: two kerb faces, a planted bed, and the yellow-on-black hazard
/// marking on the nose at each junction end.
fn median_and_hazard(
    builder: &mut MeshBuilder,
    road: &Road,
    surface: &Carriageway,
    spec: &JunctionSpec,
) {
    let section = road.section;
    // A viaduct's median is a concrete barrier (`viaduct`), not a planted strip.
    if !section.has_median() || (road.is_motorway() && road.layer != 0) {
        return;
    }
    let median_half = section.median_metres * 0.5;
    // At a crossing the median stops behind the crosswalk, its nose level with
    // the stop line: the crossing runs across the carriageway unbroken and the
    // box is clear.  Run on into the box, the median of a divided street
    // pokes into the turning paths of every left turn.
    let nose = spec.crosswalk.gap + spec.crosswalk.depth * 0.5 + 0.4;
    let from = if road.crossing_start || road.walk_start {
        nose
    } else {
        0.0
    };
    let to = surface.length
        - if road.crossing_end || road.walk_end {
            nose
        } else {
            0.0
        };
    if to - from < 1.0 {
        return;
    }
    for side in [-1.0_f32, 1.0] {
        ribbon(
            builder,
            "kerb",
            surface,
            (side * median_half - 0.12).min(side * median_half + 0.12),
            (side * median_half - 0.12).max(side * median_half + 0.12),
            from,
            to,
            super::level::KERB,
            None,
            super::Uvs::None,
        );
    }
    ribbon(
        builder,
        "median.plant",
        surface,
        -median_half,
        median_half,
        from,
        to,
        super::level::MEDIAN,
        None,
        super::Uvs::World,
    );
    median_planting(builder, road, surface, median_half, from, to);
    // The nose: the black-and-yellow diagonal on the median's end, the single
    // most recognisable piece of Chinese junction furniture there is.
    for (start, outward, crossing) in [
        (from, 1.0_f32, road.crossing_start || road.walk_start),
        (to, -1.0_f32, road.crossing_end || road.walk_end),
    ] {
        if !crossing {
            continue;
        }
        let end = (start + outward * spec.median_hazard_length).clamp(from, to);
        if (end - start).abs() < 0.2 {
            continue;
        }
        hazard_stripes(builder, surface, -median_half, median_half, start, end);
    }
}

/// What a Chinese median is planted with (the Shanghai, Shenzhen, Urumqi and
/// Kuitun photographs): a clipped hedge (小叶女贞, 黄杨) along its middle, cut
/// square and about 0.9 m high, and on a median two metres or more a band of
/// bedding flowers along each edge, one colour to a street. The hedge stops a
/// metre and a half short of each nose, where it would hide a pedestrian.
fn median_planting(
    builder: &mut MeshBuilder,
    road: &Road,
    surface: &Carriageway,
    median_half: f32,
    from: f32,
    to: f32,
) {
    let (from, to) = (from + 1.5, to - 1.5);
    if median_half < 0.5 || to - from < 3.0 {
        return;
    }
    let bed = super::level::MEDIAN;
    let flowers = median_half >= 1.0;
    if flowers {
        let colour = ["flowers/red", "flowers/pink", "flowers/yellow"][road.id as usize % 3];
        for side in [-1.0_f32, 1.0] {
            let (a, b) = (side * (median_half - 0.62), side * (median_half - 0.16));
            ribbon(
                builder,
                colour,
                surface,
                a.min(b),
                a.max(b),
                from,
                to,
                bed + 0.08,
                None,
                super::Uvs::World,
            );
        }
    }
    let half = if flowers {
        (median_half - 0.75).clamp(0.25, 0.55)
    } else {
        (median_half - 0.2).clamp(0.2, 0.45)
    };
    let height = 0.9;
    // A face, wound to look along `out`, with metre UVs (along, up).
    let face = |builder: &mut MeshBuilder, q: [Vec3; 4], uv: [(f32, f32); 4], out: Vec3| {
        let n = crate::mesh::face_normal(q[0], q[1], q[2]);
        if n.dot(out) >= 0.0 {
            builder.quad_uv("hedge.clipped", q[0], q[1], q[2], q[3], uv, None);
        } else {
            builder.quad_uv(
                "hedge.clipped",
                q[3],
                q[2],
                q[1],
                q[0],
                [uv[3], uv[2], uv[1], uv[0]],
                None,
            );
        }
    };
    let at = |s: f32, lateral: f32, up: f32| {
        let p = surface.point(s, lateral, 0.0);
        Vec3::new(p.x, bed + up, p.z)
    };
    let stations = surface.stations(from, to, 2.0);
    for w in stations.windows(2) {
        let (s0, s1) = (w[0], w[1]);
        let up = Vec3::new(0.0, 1.0, 0.0);
        face(
            builder,
            [
                at(s0, -half, height),
                at(s1, -half, height),
                at(s1, half, height),
                at(s0, half, height),
            ],
            [(s0, -half), (s1, -half), (s1, half), (s0, half)],
            up,
        );
        for side in [-1.0_f32, 1.0] {
            let out = at(s0, side, 0.0) - at(s0, 0.0, 0.0);
            face(
                builder,
                [
                    at(s0, side * half, 0.0),
                    at(s1, side * half, 0.0),
                    at(s1, side * half, height),
                    at(s0, side * half, height),
                ],
                [(s0, 0.0), (s1, 0.0), (s1, height), (s0, height)],
                out,
            );
        }
    }
    for (s, sign) in [(from, -1.0_f32), (to, 1.0)] {
        let along = (at(s + sign * 0.1, 0.0, 0.0) - at(s, 0.0, 0.0)) * sign;
        face(
            builder,
            [
                at(s, -half, 0.0),
                at(s, half, 0.0),
                at(s, half, height),
                at(s, -half, height),
            ],
            [(-half, 0.0), (half, 0.0), (half, height), (-half, height)],
            along,
        );
    }
}

/// The arterial's median carried straight across a right-in, right-out
/// junction: the side street meets an unbroken kerb, so it can only turn right.
pub(super) fn median_across(network: &Network, junction: &Junction, builder: &mut MeshBuilder) {
    let rank = |road: u32| {
        network
            .road(road)
            .map(|r| crate::network::class_rank(r.class))
            .unwrap_or(3)
    };
    let major = junction
        .ports
        .iter()
        .map(|p| rank(p.road))
        .min()
        .unwrap_or(3);
    let arms: Vec<_> = junction
        .ports
        .iter()
        .filter(|p| rank(p.road) == major)
        .collect();
    let [a, b] = arms.as_slice() else {
        return;
    };
    let Some(road) = network.road(a.road) else {
        return;
    };
    if !road.section.has_median() {
        return;
    }
    let half = road.section.median_metres * 0.5;
    let (ma, mb) = ((a.left + a.right) * 0.5, (b.left + b.right) * 0.5);
    let length = ma.distance(mb);
    if length < 1.0 {
        return;
    }
    let path = crate::math::Path::flat(vec![ma, mb]);
    for side in [-1.0_f32, 1.0] {
        builder.ribbon(
            "kerb",
            &path,
            (side * half - 0.12).min(side * half + 0.12),
            (side * half - 0.12).max(side * half + 0.12),
            0.0,
            length,
            super::level::KERB,
            None,
        );
    }
    builder.ribbon(
        "median.plant",
        &path,
        -half,
        half,
        0.0,
        length,
        super::level::MEDIAN,
        None,
    );
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
        let p0 = surface.point(from, a, super::level::KERB + 0.004);
        let p1 = surface.point(from, b, super::level::KERB + 0.004);
        let p2 = surface.point(to, b + skew, super::level::KERB + 0.004);
        let p3 = surface.point(to, a + skew, super::level::KERB + 0.004);
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
fn road_furniture_marks(
    builder: &mut MeshBuilder,
    road: &Road,
    surface: &Carriageway,
    spec: &JunctionSpec,
) {
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
        let centre = surface.point(station, offset, super::level::COVER);
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
        let a = surface.point(station - along, offset - across, super::level::ROAD + 0.003);
        let b = surface.point(station - along, offset + across, super::level::ROAD + 0.003);
        let c = surface.point(station + along, offset + across, super::level::ROAD + 0.003);
        let d = surface.point(station + along, offset - across, super::level::ROAD + 0.003);
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

pub(super) fn junction_geometry(
    network: &Network,
    junction: &Junction,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
) {
    if junction.ports.len() == 1 {
        dead_end(builder, &junction.ports[0]);
        return;
    }
    if junction.ports.len() < 2 {
        return;
    }
    // The box itself: carriageway, not pavement.  Everything the walker stands
    // on is the corner patch drawn afterwards.  World UVs, so the box's asphalt
    // is the *same* asphalt as the approach it joins and the two do not read as
    // two materials meeting.
    // The region the ring encloses, every lobe of it: where two streets meet
    // so acutely that the ring crosses itself, this is exactly the ground the
    // footways are cut away from, so the two can never leave a gap between
    // them.
    for piece in crate::buildings::normalise(&junction.ring) {
        builder.ground_uv("asphalt", &piece, super::level::ROAD, None);
    }
    sweep_ring(builder, "asphalt.pavement", &junction.ring, 0.25);

    // Corner paving: everything between the kerb return and the back of the
    // footway round the corner, flat at footway level like the footways it
    // joins, with the same kerb as theirs along its edge.  Built from the
    // streets' own kerb and footway lines (`network::Corner`), so it meets each
    // footway along its end and the block behind along its back edge.
    // Each arm's carriageway over its first 60 m, as it is drawn, bends and
    // approach widenings included: the corner paving is never laid on it.
    let approaches: Vec<Vec<Vec2>> = junction
        .ports
        .iter()
        .map(|port| match network.road(port.road) {
            Some(road) if road.carriageway.length() > 0.5 => {
                let surface = Carriageway::for_road(road, spec);
                let length = surface.length;
                let (from, to) = if port.at_start {
                    (0.0, length.min(60.0))
                } else {
                    ((length - 60.0).max(0.0), length)
                };
                let half = road.half_width();
                let run = surface.stations(from, to, 2.0);
                let edge = |s: f32, lateral: f32| {
                    let p = surface.point(s, lateral, 0.0);
                    Vec2::new(p.x, p.z)
                };
                let mut ring: Vec<Vec2> = run.iter().map(|s| edge(*s, half)).collect();
                ring.extend(run.iter().rev().map(|s| edge(*s, -half)));
                ring
            }
            _ => {
                let back = -port.dir * 60.0;
                vec![port.left, port.right, port.right + back, port.left + back]
            }
        })
        .collect();
    for corner in &junction.corners {
        corner_paving(builder, junction, corner, &approaches);
    }
}

/// Width of the kerb's top, from the carriageway edge back onto the footway.
pub(super) const KERB_TOP: f32 = 0.25;

fn corner_paving(
    builder: &mut MeshBuilder,
    junction: &Junction,
    corner: &crate::network::Corner,
    approaches: &[Vec<Vec2>],
) {
    let kerb = &corner.kerb;
    if kerb.len() < 2 {
        return;
    }
    // Which side of the kerb the footway is: away from the box.
    let outward = |a: Vec2, b: Vec2| -> Vec2 {
        let along = (b - a).normalize();
        let n = Vec2::new(along.y, -along.x);
        let mid = (a + b) * 0.5;
        if crate::math::point_in_ring(mid + n * 0.05, &junction.ring) {
            -n
        } else {
            n
        }
    };
    // Reaching 10 cm back up each footway it joins, so the two overlap and no
    // crack opens along the mouth where they meet; what that puts on a
    // carriageway is cut away below.
    let back = |line: &[Vec2], first: bool| -> Option<Vec2> {
        let (a, b) = if first {
            (*line.first()?, *line.get(1)?)
        } else {
            (*line.last()?, *line.get(line.len().checked_sub(2)?)?)
        };
        let d = (a - b).normalize();
        d.is_finite().then(|| a + d * 0.1)
    };
    let mut plan: Vec<Vec2> = Vec::new();
    plan.extend(back(kerb, true));
    plan.extend(kerb.iter().copied());
    plan.extend(back(kerb, false));
    plan.extend(back(&corner.walk, false));
    plan.extend(corner.walk.iter().rev().copied());
    plan.extend(back(&corner.walk, true));
    // Never over the road: where two streets meet so acutely that no kerb
    // return fits between them, the paving's outline crosses itself and
    // would spill into the box and up the approaches, so both are cut out
    // of it.  Everywhere else this takes nothing.
    let mut road: Vec<Vec<Vec2>> = vec![junction.ring.clone()];
    road.extend(approaches.iter().cloned());
    for piece in crate::buildings::cut_away(plan, &road) {
        if crate::math::signed_area(&piece).abs() <= 0.05 {
            continue;
        }
        for [i, j, k] in crate::math::triangulate(&piece) {
            let (a, b, c) = (piece[i], piece[j], piece[k]);
            let (a3, b3, c3) = (
                Vec3::from_plan(a, WALK_Y),
                Vec3::from_plan(b, WALK_Y),
                Vec3::from_plan(c, WALK_Y),
            );
            // Lit from above, whichever way the piece winds.
            if crate::mesh::face_normal(a3, c3, b3).y >= 0.0 {
                builder.tri_uv(
                    "sidewalk",
                    a3,
                    c3,
                    b3,
                    [(a.x, a.y), (c.x, c.y), (b.x, b.y)],
                    None,
                );
            } else {
                builder.tri_uv(
                    "sidewalk",
                    a3,
                    b3,
                    c3,
                    [(a.x, a.y), (b.x, b.y), (c.x, c.y)],
                    None,
                );
            }
        }
    }
    for pair in kerb.windows(2) {
        let (p0, p1) = (pair[0], pair[1]);
        if p0.distance(p1) < 1.0e-3 {
            continue;
        }
        // Where two streets meet too acutely for a kerb return, the kerb runs
        // straight across the corner and over one street: none is laid there.
        let probe = (p0 + p1) * 0.5 + outward(p0, p1) * 0.3;
        if road[1..]
            .iter()
            .any(|ring| crate::math::point_in_ring(probe, ring))
        {
            continue;
        }
        // The kerb face, toward the box...
        builder.wall("kerb", p0, p1, super::level::ROAD, WALK_Y, None);
        // ...and its top, mitred at each point by the neighbouring segments.
        let n = outward(p0, p1) * KERB_TOP;
        let y = super::level::KERB + 0.002;
        let q = [
            Vec3::from_plan(p0, y),
            Vec3::from_plan(p1, y),
            Vec3::from_plan(p1 + n, y),
            Vec3::from_plan(p0 + n, y),
        ];
        if crate::mesh::face_normal(q[0], q[1], q[2]).y >= 0.0 {
            builder.quad("kerb", q[0], q[1], q[2], q[3], None);
        } else {
            builder.quad("kerb", q[3], q[2], q[1], q[0], None);
        }
    }
}

/// The footway across the end of a dead-end street, and the kerb in front of
/// it: the street ends at a kerb, and the footways either side run round the
/// end and meet.
pub(crate) fn dead_end_footway(port: &crate::network::Port) -> Option<[Vec2; 4]> {
    let width = port.outer_left.distance(port.left);
    if width < 0.5 {
        return None;
    }
    let ahead = port.dir * width;
    Some([
        port.outer_left,
        port.outer_right,
        port.outer_right + ahead,
        port.outer_left + ahead,
    ])
}

fn dead_end(builder: &mut MeshBuilder, port: &crate::network::Port) {
    let Some(band) = dead_end_footway(port) else {
        return;
    };
    builder.ground_uv("sidewalk", &band, WALK_Y, None);
    // The kerb along the street's end, facing back down the street, and its
    // top; the footway band covers the two corners.
    let ahead = port.dir * KERB_TOP;
    for (a, b) in [(port.left, port.right), (port.right, port.left)] {
        builder.wall("kerb", a, b, super::level::ROAD, WALK_Y, None);
    }
    let y = super::level::KERB + 0.002;
    builder.ground_uv(
        "kerb",
        &[port.left, port.right, port.right + ahead, port.left + ahead],
        y,
        None,
    );
}

/// The junction's own markings: the yellow no-stopping box on a large signalised
/// junction, and the give-way line of triangles on every unsignalised approach.
pub(super) fn junction_details(
    network: &Network,
    junction: &Junction,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
    grid: bool,
) {
    if grid {
        yellow_grid_box(junction, builder, spec);
    }
    give_way_across_approach(network, junction, builder, spec);
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
fn yellow_grid_box(junction: &Junction, builder: &mut MeshBuilder, _spec: &JunctionSpec) {
    if junction.kind != JunctionKind::Signalized
        || junction.ports.len() < 4
        || junction.radius < 22.0
    {
        return;
    }
    let inset = crate::math::inset_ring(&junction.ring, 2.6);
    if inset.len() < 3 {
        return;
    }
    let width = 0.085_f32;
    // The outline: a continuous yellow border around the no-stopping box.
    for index in 0..inset.len() {
        let a = inset[index];
        let b = inset[(index + 1) % inset.len()];
        yellow_bar(builder, a, b, width * 1.5);
    }
    // The hatching: diagonal lines at +-45 degrees, 4.4 m apart, clipped to the
    // outline. Each line is sampled every 0.7 m and drawn as one quad per run
    // that lies inside the box.
    let mut lo = Vec2::new(f32::MAX, f32::MAX);
    let mut hi = Vec2::new(f32::MIN, f32::MIN);
    for point in &inset {
        lo = Vec2::new(lo.x.min(point.x), lo.y.min(point.y));
        hi = Vec2::new(hi.x.max(point.x), hi.y.max(point.y));
    }
    let spacing = 4.4_f32;
    let reach = (hi.x - lo.x) + (hi.y - lo.y);
    for sign in [1.0_f32, -1.0] {
        // Lines x + sign*z = c.
        let (c_lo, c_hi) = if sign > 0.0 {
            (lo.x + lo.y, hi.x + hi.y)
        } else {
            (lo.x - hi.y, hi.x - lo.y)
        };
        let mut c = c_lo + spacing * 0.5;
        while c < c_hi {
            // The line is x = c - sign*z: start at the box's low edge and walk up z.
            let dir = Vec2::new(-sign, 1.0) * std::f32::consts::FRAC_1_SQRT_2;
            let origin = Vec2::new(c - sign * lo.y, lo.y);
            let steps = (reach / 0.7) as usize + 1;
            let mut run_start: Option<Vec2> = None;
            let mut last = origin;
            for step in 0..=steps {
                let point = origin + dir * (step as f32 * 0.7);
                let inside = crate::math::point_in_ring(point, &inset);
                if inside && run_start.is_none() {
                    run_start = Some(point);
                }
                if !inside
                    && let Some(start) = run_start.take()
                    && start.distance(last) > 1.5
                {
                    yellow_bar(builder, start, last, width);
                }
                last = point;
            }
            if let Some(start) = run_start
                && start.distance(last) > 1.5
            {
                yellow_bar(builder, start, last, width);
            }
            c += spacing;
        }
    }
}

/// A flat yellow bar of half-width `half` from `a` to `b`, painted on the road.
fn yellow_bar(builder: &mut MeshBuilder, a: Vec2, b: Vec2, half: f32) {
    let along = b - a;
    let length = along.x.hypot(along.y);
    if length < 0.05 {
        return;
    }
    let (nx, nz) = (-along.y / length * half, along.x / length * half);
    let y = super::level::PAINT;
    builder.quad(
        "marking.yellow",
        Vec3::new(a.x + nx, y, a.y + nz),
        Vec3::new(b.x + nx, y, b.y + nz),
        Vec3::new(b.x - nx, y, b.y - nz),
        Vec3::new(a.x - nx, y, a.y - nz),
        None,
    );
}

/// The give-way line (让行线) across an unsignalised approach: a row of inverted
/// triangles, which is the one piece of Chinese road marking with no Western
/// equivalent and is unmistakable.
///
/// The triangles' bases sit on the mouth's own edge and their apexes point back
/// at the driver, so from the saddle they read as a row of arrowheads telling you
/// to stop.  China also paints a 400 mm stop bar behind them, which is what makes
/// the line legible at night, so both go down.
fn give_way_across_approach(
    network: &Network,
    junction: &Junction,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
) {
    if !matches!(
        junction.kind,
        JunctionKind::Priority | JunctionKind::RightInRightOut | JunctionKind::Channelized
    ) || junction.ports.len() < 3
    {
        return;
    }
    // Only the minor arms give way; the major street runs through unmarked.
    let rank = |road: u32| {
        network
            .road(road)
            .map(|r| crate::network::class_rank(r.class))
            .unwrap_or(3)
    };
    let major = junction
        .ports
        .iter()
        .map(|p| rank(p.road))
        .min()
        .unwrap_or(3);
    for port in junction.ports.iter().filter(|p| rank(p.road) > major) {
        let width = port.left.distance(port.right).max(3.0);
        if width < 3.0 {
            continue;
        }
        // Just up the approach from the mouth (the box's kerb face), apex toward
        // the driver coming in: `dir` points into the box.
        let along = -port.dir;
        let mouth = (port.left + port.right) * 0.5;
        let base = mouth + along * 1.4;
        let right = Vec2::new(-along.y, along.x);
        let count = (width / spec.give_way_pitch).floor().max(1.0) as usize;
        let pitch = width / count as f32;
        for index in 0..count {
            let centre = base + right * ((index as f32 + 0.5 - count as f32 * 0.5) * pitch);
            let half_pitch = pitch * 0.4;
            let p0 = centre - right * half_pitch;
            let p1 = centre + right * half_pitch;
            let apex = centre + along * spec.give_way_depth;
            let lift = super::level::PAINT;
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

fn sweep_ring(builder: &mut MeshBuilder, material: &str, ring: &[Vec2], thickness: f32) {
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        builder.wall(material, a, b, -thickness, super::level::ROAD, None);
    }
}

/// A roundabout's own ground (`docs/city-visual-target.md` §8): the central
/// island and the give-way at every entry. The circulating road is the
/// junction's own asphalt, unmarked, as the photographs show it.
///
/// The island stands on a stone kerb, a paved band inside it and planting
/// within, a paved ring through the bed, as a town-centre roundabout's flower
/// pattern has; there is no European truck apron. Each entry gives
/// way at the circle with the 减速让行 marking, two dashed lines along the
/// circle across the lanes coming in and an inverted triangle on them a few
/// metres back; no stop line, waiting box or lane arrow leads up to it.
pub(super) fn roundabout(
    network: &Network,
    junction: &Junction,
    builder: &mut MeshBuilder,
    spec: &JunctionSpec,
) {
    let centre = junction.centre;
    let island = junction.island;
    if island < 3.0 {
        return;
    }
    let ring = |r: f32, segments: usize| -> Vec<Vec2> {
        (0..segments)
            .map(|index| {
                let angle = index as f32 / segments as f32 * std::f32::consts::TAU;
                centre + Vec2::new(angle.cos(), angle.sin()) * r
            })
            .collect()
    };
    let segments = ((island * 3.0) as usize).clamp(48, 160);
    let edge = ring(island, segments);
    for index in 0..edge.len() {
        builder.wall(
            "kerb",
            edge[index],
            edge[(index + 1) % edge.len()],
            super::level::ROAD,
            super::level::KERB,
            None,
        );
    }
    let band = |builder: &mut MeshBuilder, material: &str, outer: f32, inner: f32, y: f32| {
        let (o, i) = (ring(outer, segments), ring(inner, segments));
        for index in 0..segments {
            let next = (index + 1) % segments;
            builder.ground_uv(material, &[o[index], o[next], i[next], i[index]], y, None);
        }
    };
    band(
        builder,
        "kerb",
        island,
        island - 0.3,
        super::level::KERB + 0.002,
    );
    band(
        builder,
        "sidewalk",
        island - 0.3,
        island - 1.8,
        super::level::KERB,
    );
    // The planting, and a paved ring through it (the bed's pattern), side by
    // side rather than one over the other.
    let bed = super::level::MEDIAN + 0.10;
    band(builder, "median.plant", island - 1.8, island * 0.5, bed);
    band(builder, "sidewalk", island * 0.5, island * 0.5 - 0.8, bed);
    builder.ground_uv(
        "median.plant",
        &ring(island * 0.5 - 0.8, segments),
        bed,
        None,
    );

    // Each entry's lanes reach the circle between the arm's centreline and its
    // arriving kerb: where those two lines, run on along the arm, meet it.
    let meet = |from: Vec2, dir: Vec2, radius: f32| -> Option<Vec2> {
        let q = from - centre;
        let b = q.dot(dir);
        let disc = b * b - (q.length_squared() - radius * radius);
        (disc >= 0.0).then(|| from + dir * (-b - disc.sqrt()).max(0.0))
    };
    let angle = |p: Vec2| (p - centre).y.atan2((p - centre).x);
    let circle = junction.circle;
    for port in &junction.ports {
        let mid = (port.left + port.right) * 0.5;
        let (Some(a), Some(b)) = (
            meet(mid, port.dir, circle),
            meet(port.left, port.dir, circle),
        ) else {
            continue;
        };
        let (a0, mut sweep) = (angle(a), angle(b) - angle(a));
        while sweep > std::f32::consts::PI {
            sweep -= std::f32::consts::TAU;
        }
        while sweep < -std::f32::consts::PI {
            sweep += std::f32::consts::TAU;
        }
        // GB 5768.3 §5.6: two dashed lines 200 mm wide and 200 mm apart,
        // dashes 600 mm long with 300 mm gaps.
        for radius in [circle + 0.3, circle + 0.7] {
            let span = sweep.abs() * radius;
            let mut along = 0.15;
            while along + 0.6 <= span - 0.15 {
                let (t0, t1) = (along / span, (along + 0.6) / span);
                let at = |t: f32, r: f32| {
                    let theta = a0 + sweep * t;
                    centre + Vec2::new(theta.cos(), theta.sin()) * r
                };
                let quad = [
                    at(t0, radius - 0.1),
                    at(t1, radius - 0.1),
                    at(t1, radius + 0.1),
                    at(t0, radius + 0.1),
                ];
                builder.ground_uv("marking.white", &quad, super::level::PAINT, None);
                along += 0.9;
            }
        }
        let median = network
            .road(port.road)
            .map_or(0.0, |road| road.section.median_metres * 0.5);
        let splitter = splitter_island(
            builder,
            mid,
            port.dir,
            median,
            meet(mid, port.dir, circle + 1.2),
        );
        // The inverted triangle, 6 m up from the circle, centred on the entry
        // lanes between the splitter island and the kerb.
        let across_entry = port.left - mid;
        let entry_half = across_entry.length();
        let inner = splitter.min(entry_half - 1.0).max(0.0);
        let lanes_mid = mid + across_entry / entry_half.max(0.01) * (inner + entry_half) * 0.5;
        let back = -port.dir;
        let Some(on_circle) = meet(lanes_mid, port.dir, circle) else {
            continue;
        };
        let base = on_circle + back * 6.0;
        let across = Vec2::new(-back.y, back.x) * (spec.give_way_width * 1.5);
        let apex = base - back * (spec.give_way_depth * 2.5);
        let (p0, p1) = (base - across, base + across);
        let lift = super::level::PAINT;
        builder.tri_flat(
            "marking.white",
            Vec3::new(p0.x, lift, p0.y),
            Vec3::new(apex.x, lift, apex.y),
            Vec3::new(p1.x, lift, p1.y),
            None,
        );
    }
}
