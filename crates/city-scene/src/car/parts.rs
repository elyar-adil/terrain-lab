//! What the car's parts are turned, swept and drawn from: a lathe whose normals
//! come from its profile (so a reflector bowl is lit from inside), a section
//! swept along a curve of the body, and the plane distance fields a lamp or a
//! window is drawn with.
use crate::math::Vec3;
use crate::mesh::MeshBuilder;

use super::surface::Vert;

/// A grid of vertices as quads, each wound to face the way its normals do.
/// `closed` joins the last column of every row back to the first.
pub(crate) fn grid(builder: &mut MeshBuilder, key: &str, rows: &[Vec<Vert>], closed: bool) {
    if rows.len() < 2 || rows[0].len() < 2 {
        return;
    }
    let around = rows[0].len();
    let mut vertices = Vec::with_capacity(rows.len() * around);
    for row in rows {
        for v in row {
            vertices.push((v.p, v.n));
        }
    }
    let mut indices = Vec::new();
    let columns = if closed { around } else { around - 1 };
    for i in 0..rows.len() - 1 {
        for k in 0..columns {
            let k1 = (k + 1) % around;
            let (a, b, c, d) = (rows[i][k], rows[i + 1][k], rows[i + 1][k1], rows[i][k1]);
            let face = (b.p - a.p).cross(c.p - a.p) + (c.p - a.p).cross(d.p - a.p);
            if face.length() < 1.0e-13 {
                continue;
            }
            let at = |i: usize, k: usize| (i * around + k) as u32;
            let (ia, ib, ic, id) = (at(i, k), at(i + 1, k), at(i + 1, k1), at(i, k1));
            if face.dot(a.n + b.n + c.n + d.n) >= 0.0 {
                indices.extend_from_slice(&[ia, ib, ic, ia, ic, id]);
            } else {
                indices.extend_from_slice(&[id, ic, ib, id, ib, ia]);
            }
        }
    }
    builder.indexed(key, &vertices, &indices);
}

/// Two unit vectors square to `axis` and to each other.
pub(crate) fn frame(axis: Vec3) -> (Vec3, Vec3) {
    let up = axis.normalized_or_up();
    let reference = if up.y.abs() < 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let a = reference.cross(up).normalized_or_up();
    (a, up.cross(a))
}

/// A profile `(radius, distance along the axis)` turned about `axis` through
/// `at`. The surface faces to the right of the profile's direction of travel:
/// drawn from the axis outward and then up, it is a solid seen from outside;
/// `inside` turns it to be seen from within (a bowl, a barrel). A point given
/// twice is a hard edge.
pub(crate) fn lathe(
    builder: &mut MeshBuilder,
    key: &str,
    at: Vec3,
    axis: Vec3,
    profile: &[(f32, f32)],
    sides: usize,
    inside: bool,
) {
    lathe_arc(
        builder,
        key,
        at,
        axis,
        profile,
        sides,
        inside,
        (0.0, 1.0),
        0.0,
    );
}

/// `lathe` over part of the turn, `span` in turns, started `phase` turns round.
#[allow(clippy::too_many_arguments)]
pub(crate) fn lathe_arc(
    builder: &mut MeshBuilder,
    key: &str,
    at: Vec3,
    axis: Vec3,
    profile: &[(f32, f32)],
    sides: usize,
    inside: bool,
    span: (f32, f32),
    phase: f32,
) {
    let up = axis.normalized_or_up();
    let (side_a, side_b) = frame(up);
    let whole = (span.1 - span.0 - 1.0).abs() < 1.0e-4;
    // Split at doubled points: each run is smooth in itself.
    let mut run: Vec<(f32, f32)> = Vec::new();
    let flush = |run: &mut Vec<(f32, f32)>, builder: &mut MeshBuilder| {
        if run.len() >= 2 {
            let count = run.len();
            let rows: Vec<Vec<Vert>> = (0..count)
                .map(|j| {
                    let before = run[j.saturating_sub(1)];
                    let after = run[(j + 1).min(count - 1)];
                    let (dr, dh) = (after.0 - before.0, after.1 - before.1);
                    let length = (dr * dr + dh * dh).sqrt().max(1.0e-9);
                    let sign = if inside { -1.0 } else { 1.0 };
                    let (nr, nh) = (dh / length * sign, -dr / length * sign);
                    let steps = if whole { sides } else { sides + 1 };
                    (0..steps)
                        .map(|k| {
                            let turn = span.0 + (span.1 - span.0) * k as f32 / sides as f32 + phase;
                            let angle = turn * std::f32::consts::TAU;
                            let out = side_a * angle.cos() + side_b * angle.sin();
                            Vert::new(
                                at + up * run[j].1 + out * run[j].0.max(0.0),
                                (out * nr + up * nh).normalized_or_up(),
                            )
                        })
                        .collect()
                })
                .collect();
            grid(builder, key, &rows, whole);
        }
        run.clear();
    };
    for (index, point) in profile.iter().enumerate() {
        if index > 0 && *point == profile[index - 1] {
            flush(&mut run, builder);
        }
        run.push(*point);
    }
    flush(&mut run, builder);
}

/// A ball, for a bulb or a bead; `squash` flattens it along `axis`.
pub(crate) fn ball(
    builder: &mut MeshBuilder,
    key: &str,
    at: Vec3,
    axis: Vec3,
    r: f32,
    squash: f32,
    sides: usize,
) {
    let levels = (sides / 2).max(3);
    let profile: Vec<(f32, f32)> = (0..=levels)
        .map(|j| {
            let angle = (j as f32 / levels as f32 - 0.5) * std::f32::consts::PI;
            (angle.cos() * r, angle.sin() * r * squash)
        })
        .collect();
    lathe(builder, key, at, axis, &profile, sides, false);
}

/// A dome standing on a surface: the outward half of a ball.
pub(crate) fn dome(
    builder: &mut MeshBuilder,
    key: &str,
    at: Vec3,
    axis: Vec3,
    r: f32,
    height: f32,
    sides: usize,
) {
    let levels = (sides / 3).max(2);
    let profile: Vec<(f32, f32)> = (0..=levels)
        .map(|j| {
            let angle = j as f32 / levels as f32 * std::f32::consts::FRAC_PI_2;
            (angle.cos() * r, angle.sin() * height)
        })
        .collect();
    lathe(builder, key, at, axis, &profile, sides, false);
}

/// An oval section swept along a curve of a surface: `half_w` across the
/// surface, `half_h` off it, its middle `lift` above it. A chrome strip, a
/// light guide, a rubber seal. The ends close; a curve that returns to its
/// first point is a ring.
pub(crate) fn sweep(
    builder: &mut MeshBuilder,
    key: &str,
    curve: &[Vert],
    half_w: f32,
    half_h: f32,
    lift: f32,
    sides: usize,
) {
    let curve = &thinned(curve, half_w.max(half_h) * 1.6);
    if curve.len() < 2 {
        return;
    }
    let ring = (curve[0].p - curve[curve.len() - 1].p).length() < 1.0e-5 && curve.len() > 3;
    let points: &[Vert] = if ring {
        &curve[..curve.len() - 1]
    } else {
        curve
    };
    let count = points.len();
    let mut rows: Vec<Vec<Vert>> = Vec::with_capacity(count + 2);
    let section = |index: usize, scale: f32| -> Vec<Vert> {
        let (before, after) = if ring {
            (
                points[(index + count - 1) % count],
                points[(index + 1) % count],
            )
        } else {
            (
                points[index.saturating_sub(1)],
                points[(index + 1).min(count - 1)],
            )
        };
        let v = points[index];
        let along = (after.p - before.p).normalized_or_up();
        let across = along.cross(v.n).normalized_or_up();
        let middle = v.p + v.n * lift;
        (0..sides)
            .map(|k| {
                let angle = k as f32 / sides as f32 * std::f32::consts::TAU;
                let (c, s) = (angle.cos(), angle.sin());
                let normal = (across * (c / half_w.max(1.0e-5)) + v.n * (s / half_h.max(1.0e-5)))
                    .normalized_or_up();
                Vert::new(
                    middle + (across * (c * half_w) + v.n * (s * half_h)) * scale,
                    normal,
                )
            })
            .collect()
    };
    if !ring {
        let along = (points[1].p - points[0].p).normalized_or_up();
        rows.push(
            section(0, 0.0)
                .into_iter()
                .map(|v| Vert::new(v.p, -along))
                .collect(),
        );
    }
    for index in 0..count {
        rows.push(section(index, 1.0));
    }
    if ring {
        rows.push(section(0, 1.0));
    } else {
        let along = (points[count - 1].p - points[count - 2].p).normalized_or_up();
        rows.push(
            section(count - 1, 0.0)
                .into_iter()
                .map(|v| Vert::new(v.p, along))
                .collect(),
        );
    }
    grid(builder, key, &rows, true);
}

/// A wall raised from a loop of a surface: from each vertex to the same vertex
/// moved by `to`, facing the side `inward` says (the inside of the loop for a
/// pocket's wall, the outside for a raised part's).
pub(crate) fn wall(
    builder: &mut MeshBuilder,
    key: &str,
    edge: &[Vert],
    to: &dyn Fn(Vert) -> Vec3,
    inward: bool,
) {
    if edge.len() < 2 {
        return;
    }
    let count = edge.len();
    let ring = (edge[0].p - edge[count - 1].p).length() < 1.0e-5;
    let mut top = Vec::with_capacity(count);
    let mut foot = Vec::with_capacity(count);
    for index in 0..count {
        let before = if index == 0 {
            if ring { edge[count - 2] } else { edge[0] }
        } else {
            edge[index - 1]
        };
        let after = if index + 1 == count {
            if ring { edge[1] } else { edge[index] }
        } else {
            edge[index + 1]
        };
        let v = edge[index];
        let down = to(v);
        let along = after.p - before.p;
        // The region lies to the left of its edge, seen from outside.
        let mut normal = v.n.cross(along).normalized_or_up();
        normal = (normal - down.normalized_or_up() * normal.dot(down.normalized_or_up()))
            .normalized_or_up();
        if !inward {
            normal = -normal;
        }
        top.push(Vert::new(v.p, normal));
        foot.push(Vert::new(v.p + down, normal));
    }
    grid(builder, key, &[top, foot], false);
}

/// A flat cap over a loop: a fan from its middle, facing `normal`.
pub(crate) fn cap(builder: &mut MeshBuilder, key: &str, edge: &[Vec3], normal: Vec3) {
    if edge.len() < 3 {
        return;
    }
    let middle = edge.iter().fold(Vec3::new(0.0, 0.0, 0.0), |a, p| a + *p) / edge.len() as f32;
    let mut vertices = vec![(middle, normal)];
    vertices.extend(edge.iter().map(|p| (*p, normal)));
    let mut indices = Vec::new();
    for k in 0..edge.len() {
        let (a, b) = (k as u32 + 1, ((k + 1) % edge.len()) as u32 + 1);
        let face = (edge[k] - middle).cross(edge[(k + 1) % edge.len()] - middle);
        if face.dot(normal) >= 0.0 {
            indices.extend_from_slice(&[0, a, b]);
        } else {
            indices.extend_from_slice(&[0, b, a]);
        }
    }
    builder.indexed(key, &vertices, &indices);
}

/// A flat slab: a rectangle of half-sizes `across` and `up` (vectors), `thick`
/// deep behind its face, which looks along `normal`. Square edges: a number
/// plate, a badge.
pub(crate) fn slab(
    builder: &mut MeshBuilder,
    key: &str,
    centre: Vec3,
    normal: Vec3,
    up: Vec3,
    across: Vec3,
    thick: f32,
) {
    let corners = [
        centre - across - up,
        centre + across - up,
        centre + across + up,
        centre - across + up,
    ];
    let face: Vec<Vert> = corners.iter().map(|p| Vert::new(*p, normal)).collect();
    grid(
        builder,
        key,
        &[vec![face[0], face[1]], vec![face[3], face[2]]],
        false,
    );
    for k in 0..4 {
        let (a, b) = (corners[k], corners[(k + 1) % 4]);
        let out = (b - a).cross(normal).normalized_or_up();
        let out = if out.dot(a - centre) < 0.0 { -out } else { out };
        let back = normal * -thick;
        grid(
            builder,
            key,
            &[
                vec![Vert::new(a, out), Vert::new(b, out)],
                vec![Vert::new(a + back, out), Vert::new(b + back, out)],
            ],
            false,
        );
    }
}

/// A curve thinned so that no two of its points are nearer than `step`: what
/// a contour of a fine mesh needs before something is swept along it.
pub(crate) fn thinned(curve: &[Vert], step: f32) -> Vec<Vert> {
    if curve.len() < 3 {
        return curve.to_vec();
    }
    let mut out = vec![curve[0]];
    for v in &curve[1..curve.len() - 1] {
        if (v.p - out[out.len() - 1].p).length() >= step {
            out.push(*v);
        }
    }
    let last = curve[curve.len() - 1];
    if out.len() > 1 && (last.p - out[out.len() - 1].p).length() < step * 0.5 {
        out.pop();
    }
    out.push(last);
    out
}

// ---------------------------------------------------------------------------
// Distance fields in the plane: negative inside, in metres.

/// A convex polygon (anticlockwise) with corners of radius `r`.
pub(crate) fn sd_convex(points: &[(f32, f32)], p: (f32, f32), r: f32) -> f32 {
    let mut deepest = f32::MIN;
    let mut outside = 0.0_f32;
    let count = points.len();
    // Either winding: the inside is where the polygon's own middle is.
    let middle = points.iter().fold((0.0, 0.0), |a, q| {
        (a.0 + q.0 / count as f32, a.1 + q.1 / count as f32)
    });
    for k in 0..count {
        let (a, b) = (points[k], points[(k + 1) % count]);
        let (ex, ey) = (b.0 - a.0, b.1 - a.1);
        let length = (ex * ex + ey * ey).sqrt().max(1.0e-9);
        let (mut nx, mut ny) = (ey / length, -ex / length);
        if (middle.0 - a.0) * nx + (middle.1 - a.1) * ny > 0.0 {
            nx = -nx;
            ny = -ny;
        }
        let d = (p.0 - a.0) * nx + (p.1 - a.1) * ny + r;
        deepest = deepest.max(d);
        outside += d.max(0.0).powi(2);
    }
    deepest.min(0.0) + outside.sqrt() - r
}

/// A box of half-sizes `half` about `centre` with corners of radius `r`.
pub(crate) fn sd_box(centre: (f32, f32), half: (f32, f32), p: (f32, f32), r: f32) -> f32 {
    let (dx, dy) = (
        (p.0 - centre.0).abs() - half.0 + r,
        (p.1 - centre.1).abs() - half.1 + r,
    );
    dx.max(dy).min(0.0) + (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt() - r
}

/// The intersection of regions, its corners rounded to radius `r`.
pub(crate) fn round_and(fields: &[f32], r: f32) -> f32 {
    let mut deepest = f32::MIN;
    let mut outside = 0.0_f32;
    for f in fields {
        let d = f + r;
        deepest = deepest.max(d);
        outside += d.max(0.0).powi(2);
    }
    deepest.min(0.0) + outside.sqrt() - r
}

pub(crate) fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
