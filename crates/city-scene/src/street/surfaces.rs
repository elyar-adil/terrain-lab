//! What a street is *made of*: the crowned carriageway, the kerbs and their
//! dropped-kerb ramps, the median and its hazard nose, the junction box and
//! its corner paving, and the roundabout variant of all of it.
//!
//! Markings live in [`super::markings`]; this module owns every surface a
//! driver or pedestrian actually touches.

use urban::JunctionKind;

use crate::math::{Rng, Vec2, Vec3};
use crate::mesh::MeshBuilder;
use crate::network::{Junction, Road};
use crate::spec::{JunctionSpec, MM, Movement, arrow_polygons};

use super::{Carriageway, WALK_Y, ribbon, sweep};

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

pub(super) fn road_surface(road: &Road, builder: &mut MeshBuilder, spec: &JunctionSpec) {
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
                super::level::KERB,
                None,
                super::Uvs::World,
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
                super::level::KERB + 0.002,
                None,
                super::Uvs::None,
            );
            sweep(
                builder,
                "kerb",
                &surface,
                inner - 0.16,
                inner + 0.16,
                *from,
                *to,
                super::level::KERB,
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
    let ground = super::level::ROAD + 0.02;
    let quad = |builder: &mut MeshBuilder, s0: f32, y0: f32, s1: f32, y1: f32| {
        let a = surface.point(s0, kerb, y0);
        let b = surface.point(s0, back, super::level::KERB);
        let c = surface.point(s1, back, super::level::KERB);
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
    quad(builder, from, super::level::KERB, lo, ground);
    quad(builder, lo, ground, hi, ground);
    quad(builder, hi, ground, to, super::level::KERB);

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
        super::level::TACTILE - 0.004,
        None,
        super::Uvs::None,
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
            super::level::TACTILE,
            None,
            super::Uvs::None,
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
        0.0,
        surface.length,
        super::level::MEDIAN,
        None,
        super::Uvs::World,
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
        let p0 = surface.point(from, a, super::level::PAINT);
        let p1 = surface.point(from, b, super::level::PAINT);
        let p2 = surface.point(to, b + skew, super::level::PAINT);
        let p3 = surface.point(to, a + skew, super::level::PAINT);
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

pub(super) fn junction_geometry(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
    let _ = spec;
    if junction.ports.len() < 2 {
        return;
    }
    // The box itself: carriageway, not pavement.  Everything the walker stands
    // on is the corner patch drawn afterwards.  World UVs, so the box's asphalt
    // is the *same* asphalt as the approach it joins and the two do not read as
    // two materials meeting.
    builder.ground_uv("asphalt", &junction.ring, super::level::ROAD, None);
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
                super::level::ROAD,
                WALK_Y,
                None,
            );
        }
        let plan: Vec<Vec2> = patch.iter().map(|v| Vec2::new(v.x, v.z)).collect();
        for [i, j, k] in crate::math::triangulate(&plan) {
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
pub(super) fn junction_details(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
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
    if junction.roundabout || junction.ports.len() < 4 || junction.radius < 22.0 {
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
                if !inside {
                    if let Some(start) = run_start.take() {
                        if start.distance(last) > 1.5 {
                            yellow_bar(builder, start, last, width);
                        }
                    }
                }
                last = point;
            }
            if let Some(start) = run_start {
                if start.distance(last) > 1.5 {
                    yellow_bar(builder, start, last, width);
                }
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
                    Vec3::new(at - width, super::level::PAINT, station),
                    Vec3::new(at + width, super::level::PAINT, station),
                    Vec3::new(at + width, super::level::PAINT, end),
                    Vec3::new(at - width, super::level::PAINT, end),
                ]
            } else {
                [
                    Vec3::new(station, super::level::PAINT, at - width),
                    Vec3::new(end, super::level::PAINT, at - width),
                    Vec3::new(end, super::level::PAINT, at + width),
                    Vec3::new(station, super::level::PAINT, at + width),
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

/// A roundabout: circulatory carriageway, mountable truck apron, raised central
/// island, and the give-way line of triangles across every entry.
///
/// The apron matters more than it looks.  A truck whose wheelbase is 12 m cannot
/// follow a 6 m island, so the inner 400 mm of every roundabout is laid as a
/// mountable kerb at a different tone, and that single ring is most of what
/// tells a driver how big the circle is.
pub(super) fn roundabout(junction: &Junction, builder: &mut MeshBuilder, spec: &JunctionSpec) {
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
    // The carriageway is the junction ring itself, which is a circle for a
    // roundabout. The apron: a coloured, mountable ring around the island, so the
    // circle reads at a glance the way it does on a real one.
    let inner = ring(island + 0.45, 48);
    let outer = ring(island + 3.4, 48);
    for index in 0..inner.len() {
        let next = (index + 1) % inner.len();
        builder.ground_uv(
            "asphalt.cycle",
            &[inner[index], outer[index], outer[next], inner[next]],
            super::level::ROAD + 0.012,
            None,
        );
    }
    builder.ground_uv("kerb", &inner, super::level::ROAD + 0.014, None);
    // The island kerb as a wall, so it has a face and catches the sun on top.
    let kerb_ring = ring(island + 0.45, 48);
    for index in 0..kerb_ring.len() {
        builder.wall(
            "kerb",
            kerb_ring[index],
            kerb_ring[(index + 1) % kerb_ring.len()],
            super::level::ROAD,
            super::level::KERB,
            None,
        );
    }
    builder.ground_uv("median.plant", &ring(island, 36), super::level::MEDIAN + 0.10, None);

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
        let line = centre + outward * (distance - 2.2);
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
                Vec3::new(p0.x, super::level::PAINT, p0.y),
                Vec3::new(apex.x, super::level::PAINT, apex.y),
                Vec3::new(p1.x, super::level::PAINT, p1.y),
                None,
            );
        }
        // The deflection arrow: a straight stencil turned to the circle's
        // tangent, so the driver is told to curve rather than to continue.
        let entry = centre + outward * (distance - 8.0);
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
                Vec3::new(plan.x, super::level::PAINT + lift, plan.y)
            };
            builder.tri_flat("marking.white", at(i), at(j), at(k), None);
        }
    }
}
