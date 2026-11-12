//! Smooth-shaded modelling primitives: the parts every prototype is built from.
//!
//! A prototype is uploaded once and drawn by instancing, so its triangle count
//! is paid once per frame per visible instance and nothing at all in the
//! payload. What a model needs to stop reading as low-poly is not only more
//! sides but normals that belong to the surface instead of to each facet, and
//! that is what these give.
use crate::math::Vec3;
use crate::mesh::MeshBuilder;

/// A surface through a stack of closed rings, smooth-shaded: every vertex takes
/// the normal of the surface there, not of the facet it belongs to. This is the
/// difference between a lantern and a twelve-sided drum, and it costs nothing
/// per instance: a prototype is uploaded once however many sides it has.
///
/// Rings must have the same number of points. A ring collapsed to a point is a
/// pole (the tip of a bud, the end of a car) and takes the axis as its normal.
pub(crate) fn skin(builder: &mut MeshBuilder, key: &str, rings: &[Vec<Vec3>]) {
    skin_cut(builder, key, rings, |_, _| true);
}

/// `skin` with openings: `keep(ring, point)` says whether the quad that starts
/// there is part of the surface (a car's shell is its cabin with the windows out).
pub(crate) fn skin_cut(
    builder: &mut MeshBuilder,
    key: &str,
    rings: &[Vec<Vec3>],
    keep: impl Fn(usize, usize) -> bool,
) {
    skin_lifted(builder, key, rings, 0.0, keep);
}

/// Part of a skin, stood `lift` metres off it along its normals: a lamp lens, a
/// door handle, the dark line of a shut gap, all conforming exactly to the
/// surface they sit on because they are that surface, moved.
pub(crate) fn skin_lifted(
    builder: &mut MeshBuilder,
    key: &str,
    rings: &[Vec<Vec3>],
    lift: f32,
    keep: impl Fn(usize, usize) -> bool,
) {
    if rings.len() < 2 || rings[0].len() < 3 {
        return;
    }
    let around = rings[0].len();
    let normals = skin_normals(rings);
    for i in 0..rings.len() - 1 {
        for k in 0..around {
            if !keep(i, k) {
                continue;
            }
            let k1 = (k + 1) % around;
            let at = |i: usize, k: usize| (rings[i][k] + normals[i][k] * lift, normals[i][k]);
            let (a, b, c, d) = (at(i, k), at(i + 1, k), at(i + 1, k1), at(i, k1));
            let face = (b.0 - a.0).cross(c.0 - a.0) + (c.0 - a.0).cross(d.0 - a.0);
            if face.length() < 1.0e-12 {
                continue;
            }
            let mean = a.1 + b.1 + c.1 + d.1;
            if face.dot(mean) >= 0.0 {
                builder.quad_shaded(key, a, b, c, d, None);
            } else {
                builder.quad_shaded(key, d, c, b, a, None);
            }
        }
    }
}

/// The normal of a skin at every point of every ring: the surface's own, from
/// its neighbours along and around, facing away from the rings' axis.
pub(crate) fn skin_normals(rings: &[Vec<Vec3>]) -> Vec<Vec<Vec3>> {
    let around = rings[0].len();
    let centre = |ring: &Vec<Vec3>| {
        ring.iter().fold(Vec3::new(0.0, 0.0, 0.0), |a, p| a + *p) / ring.len() as f32
    };
    let centres: Vec<Vec3> = rings.iter().map(centre).collect();
    let radii: Vec<f32> = rings
        .iter()
        .zip(&centres)
        .map(|(ring, c)| {
            ring.iter()
                .map(|p| (*p - *c).length())
                .fold(0.0_f32, f32::max)
        })
        .collect();
    let mut normals: Vec<Vec<Vec3>> = Vec::with_capacity(rings.len());
    for (i, ring) in rings.iter().enumerate() {
        let before = if i == 0 { i } else { i - 1 };
        let after = if i + 1 == rings.len() { i } else { i + 1 };
        let axis = (centres[after] - centres[before]).normalized_or_up();
        let radius = radii[i];
        // The ring next to a pole takes its slope from its other neighbour: the
        // step to the pole lies in the ring's own plane and says nothing about
        // which way the side wall faces.
        let before = if before != i && radii[before] < 1.0e-4 && after != i {
            i
        } else {
            before
        };
        let after = if after != i && radii[after] < 1.0e-4 && before != i {
            i
        } else {
            after
        };
        let mut out = Vec::with_capacity(around);
        for k in 0..around {
            let p = ring[k];
            if radius < 1.0e-4 {
                // A pole faces away from the body it closes.
                out.push(if i == 0 { -axis } else { axis });
                continue;
            }
            let du = ring[(k + 1) % around] - ring[(k + around - 1) % around];
            let dv = rings[after][k] - rings[before][k];
            let mut n = du.cross(dv);
            if n.length() < 1.0e-9 {
                n = p - centres[i];
            }
            let mut n = n.normalized_or_up();
            // Outward: away from the ring's axis.
            let radial = (p - centres[i]) - axis * (p - centres[i]).dot(axis);
            let facing = if radial.length() > 1.0e-5 {
                n.dot(radial)
            } else {
                n.dot(if i * 2 < rings.len() { -axis } else { axis })
            };
            if facing < 0.0 {
                n = -n;
            }
            out.push(n);
        }
        normals.push(out);
    }
    normals
}

/// A profile `(radius, distance along the axis)` turned about `axis` through
/// `at`, smooth-shaded.
pub(crate) fn turn(
    builder: &mut MeshBuilder,
    key: &str,
    at: Vec3,
    axis: Vec3,
    profile: &[(f32, f32)],
    sides: usize,
) {
    let up = axis.normalized_or_up();
    let reference = if up.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    };
    let side_a = up.cross(reference).normalized_or_up();
    let side_b = up.cross(side_a);
    let rings: Vec<Vec<Vec3>> = profile
        .iter()
        .map(|(r, along)| {
            (0..sides)
                .map(|k| {
                    let angle = k as f32 / sides as f32 * std::f32::consts::TAU;
                    at + up * *along + (side_a * angle.cos() + side_b * angle.sin()) * r.max(0.0)
                })
                .collect()
        })
        .collect();
    skin(builder, key, &rings);
}

/// A polyline of `(a, b)` pairs passed through a Catmull-Rom spline, `per`
/// points to the span: a hand-set profile of six points becomes a curve.
pub(crate) fn rounded(points: &[(f32, f32)], per: usize) -> Vec<(f32, f32)> {
    if points.len() < 3 {
        return points.to_vec();
    }
    let at = |i: isize| points[i.clamp(0, points.len() as isize - 1) as usize];
    let mut out = Vec::with_capacity((points.len() - 1) * per + 1);
    for i in 0..points.len() as isize - 1 {
        let (p0, p1, p2, p3) = (at(i - 1), at(i), at(i + 1), at(i + 2));
        for step in 0..per {
            let t = step as f32 / per as f32;
            let (t2, t3) = (t * t, t * t * t);
            let blend = |a: f32, b: f32, c: f32, d: f32| {
                0.5 * (2.0 * b
                    + (c - a) * t
                    + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
                    + (3.0 * b - a - 3.0 * c + d) * t3)
            };
            out.push((blend(p0.0, p1.0, p2.0, p3.0), blend(p0.1, p1.1, p2.1, p3.1)));
        }
    }
    out.push(points[points.len() - 1]);
    out
}

/// A round bar between two points, smooth-shaded, for every pole, arm and rail
/// of the street furniture.
pub(crate) fn bar(
    builder: &mut MeshBuilder,
    key: &str,
    from: Vec3,
    to: Vec3,
    r0: f32,
    r1: f32,
    sides: u8,
) {
    let axis = to - from;
    if !axis.is_finite() || axis.length() <= 1.0e-5 {
        return;
    }
    // Thick members get enough sides to be round at arm's length; thin ones
    // (a bracket, a tie) enough not to be a prism.
    let sides = if sides >= 8 { 28 } else { 10 };
    turn(
        builder,
        key,
        from,
        axis,
        &[(r0, 0.0), (r1, axis.length())],
        sides,
    );
}

/// A cushion: a box with every edge and corner rounded, `half` its half-sizes,
/// leaned back by `lean` radians about the cross axis (z). Seats, head
/// restraints, a dashboard, a bumper insert.
pub(crate) fn soft_box(
    builder: &mut MeshBuilder,
    key: &str,
    centre: Vec3,
    half: Vec3,
    lean: f32,
    squareness: f32,
) {
    let levels = 9;
    let around = 20;
    let (cos, sin) = (lean.cos(), lean.sin());
    let place =
        |x: f32, y: f32, z: f32| centre + Vec3::new(x * cos - y * sin, x * sin + y * cos, z);
    let mut rings: Vec<Vec<Vec3>> = Vec::with_capacity(levels + 3);
    rings.push(vec![place(0.0, -half.y, 0.0); around]);
    for level in 0..=levels {
        let t = level as f32 / levels as f32 * 2.0 - 1.0;
        // Ends close in like the sides do.
        let inset = (1.0 - t.abs().powf(squareness))
            .max(0.0)
            .powf(1.0 / squareness)
            .max(0.18);
        let ring = (0..around)
            .map(|k| {
                let angle = (k as f32 + 0.5) / around as f32 * std::f32::consts::TAU;
                let (c, s) = (angle.cos(), angle.sin());
                let x = c.signum() * c.abs().powf(2.0 / squareness) * half.x * inset;
                let z = s.signum() * s.abs().powf(2.0 / squareness) * half.z * inset;
                place(x, t * half.y, z)
            })
            .collect();
        rings.push(ring);
    }
    rings.push(vec![place(0.0, half.y, 0.0); around]);
    skin(builder, key, &rings);
}

/// A ring of round section: a steering wheel, a wheel-arch lip. `from..to` in
/// turns of the circle (0..1 is the whole ring).
pub(crate) fn torus_arc(
    builder: &mut MeshBuilder,
    key: &str,
    centre: Vec3,
    axis: Vec3,
    start: Vec3,
    radius: f32,
    tube: f32,
    span: (f32, f32),
) {
    let n = axis.normalized_or_up();
    let u = (start - n * start.dot(n)).normalized_or_up();
    let v = n.cross(u);
    let steps = (((span.1 - span.0).abs() * 48.0).ceil() as usize).max(6);
    let around = 10;
    let mut rings: Vec<Vec<Vec3>> = Vec::with_capacity(steps + 3);
    let ring_at = |turn: f32, r: f32| -> Vec<Vec3> {
        let angle = turn * std::f32::consts::TAU;
        let out = u * angle.cos() + v * angle.sin();
        let mid = centre + out * radius;
        (0..around)
            .map(|k| {
                let a = k as f32 / around as f32 * std::f32::consts::TAU;
                mid + (out * a.cos() + n * a.sin()) * r
            })
            .collect()
    };
    let whole = (span.1 - span.0 - 1.0).abs() < 1.0e-3;
    if !whole {
        rings.push(ring_at(span.0, 0.0));
    }
    for step in 0..=steps {
        rings.push(ring_at(
            span.0 + (span.1 - span.0) * step as f32 / steps as f32,
            tube,
        ));
    }
    if !whole {
        rings.push(ring_at(span.1, 0.0));
    }
    skin(builder, key, &rings);
}

/// `soft_box` with its three half-axes given outright, for a part that sits at
/// an angle: a lamp round a corner, a mirror on its stalk.
pub(crate) fn soft_block(
    builder: &mut MeshBuilder,
    key: &str,
    centre: Vec3,
    along: Vec3,
    up: Vec3,
    across: Vec3,
    squareness: f32,
) {
    let levels = 9;
    let around = 20;
    let mut rings: Vec<Vec<Vec3>> = Vec::with_capacity(levels + 3);
    rings.push(vec![centre - up; around]);
    for level in 0..=levels {
        let t = level as f32 / levels as f32 * 2.0 - 1.0;
        let inset = (1.0 - t.abs().powf(squareness))
            .max(0.0)
            .powf(1.0 / squareness)
            .max(0.18);
        let ring = (0..around)
            .map(|k| {
                let angle = (k as f32 + 0.5) / around as f32 * std::f32::consts::TAU;
                let (c, s) = (angle.cos(), angle.sin());
                let a = c.signum() * c.abs().powf(2.0 / squareness) * inset;
                let b = s.signum() * s.abs().powf(2.0 / squareness) * inset;
                centre + along * a + across * b + up * t
            })
            .collect();
        rings.push(ring);
    }
    rings.push(vec![centre + up; around]);
    skin(builder, key, &rings);
}
