//! Plane geometry shared by railway and access-road alignment: simplification,
//! circular fillets with a radius floor, and resampling.

pub type P = [f32; 2];

pub fn dist(a: P, b: P) -> f32 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

pub fn sub(a: P, b: P) -> P {
    [a[0] - b[0], a[1] - b[1]]
}

pub fn unit(a: P) -> P {
    let l = a[0].hypot(a[1]).max(1.0e-9);
    [a[0] / l, a[1] / l]
}

pub fn add_scaled(a: P, d: P, s: f32) -> P {
    [a[0] + d[0] * s, a[1] + d[1] * s]
}

fn point_line(p: P, a: P, b: P) -> f32 {
    let v = sub(b, a);
    let l2 = v[0] * v[0] + v[1] * v[1];
    if l2 < 1.0e-9 {
        return dist(p, a);
    }
    let t = (((p[0] - a[0]) * v[0] + (p[1] - a[1]) * v[1]) / l2).clamp(0.0, 1.0);
    dist(p, [a[0] + v[0] * t, a[1] + v[1] * t])
}

/// Douglas-Peucker simplification keeping both ends.
pub fn simplify(points: &[P], tolerance: f32) -> Vec<P> {
    if points.len() < 3 {
        return points.to_vec();
    }
    let mut keep = vec![false; points.len()];
    keep[0] = true;
    keep[points.len() - 1] = true;
    let mut stack = vec![(0usize, points.len() - 1)];
    while let Some((a, b)) = stack.pop() {
        let mut worst = (0.0, a);
        for i in a + 1..b {
            let d = point_line(points[i], points[a], points[b]);
            if d > worst.0 {
                worst = (d, i);
            }
        }
        if worst.0 > tolerance {
            keep[worst.1] = true;
            stack.push((a, worst.1));
            stack.push((worst.1, b));
        }
    }
    points
        .iter()
        .zip(keep)
        .filter_map(|(p, k)| k.then_some(*p))
        .collect()
}

pub fn length(points: &[P]) -> f32 {
    points.windows(2).map(|w| dist(w[0], w[1])).sum()
}

/// Resample at uniform arc length, always keeping the last point.
/// Returns the points and each point's chainage.
pub fn resample(points: &[P], step: f32) -> (Vec<P>, Vec<f32>) {
    let mut out = vec![points[0]];
    let mut s_out = vec![0.0];
    let mut carried = 0.0_f32;
    let mut total = 0.0_f32;
    for w in points.windows(2) {
        let seg = dist(w[0], w[1]);
        if seg < 1.0e-6 {
            continue;
        }
        let d = unit(sub(w[1], w[0]));
        let mut at = step - carried;
        while at <= seg {
            out.push(add_scaled(w[0], d, at));
            s_out.push(total + at);
            at += step;
        }
        carried = seg - (at - step);
        total += seg;
    }
    let last = *points.last().unwrap();
    if dist(*out.last().unwrap(), last) > 0.5 {
        out.push(last);
        s_out.push(total);
    }
    (out, s_out)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pin {
    /// May be dropped to make the curves fit.
    Free,
    /// Must stay (a station's straight, a junction).
    Fixed,
}

/// Round every corner of a polyline with a circular arc of at least `r_min`
/// (preferably `r_pref`). A corner too tight for that is removed when it is
/// `Free`; segments flagged `straight` give up at most half their length to
/// the arcs at their ends. Returns `None` if a `Fixed` corner cannot be
/// rounded to `r_min`.
pub fn fillet(vertices: &[(P, Pin)], straight: &[bool], r_min: f32, r_pref: f32) -> Option<Vec<P>> {
    let mut v: Vec<(P, Pin, bool)> = Vec::new(); // vertex, pin, segment-after-is-straight
    for (i, (p, pin)) in vertices.iter().enumerate() {
        v.push((*p, *pin, straight.get(i).copied().unwrap_or(false)));
    }
    // Remove duplicates.
    v.dedup_by(|b, a| dist(a.0, b.0) < 1.0);
    let radii = loop {
        let n = v.len();
        if n < 3 {
            break vec![0.0; n];
        }
        let seg: Vec<f32> = (0..n - 1).map(|i| dist(v[i].0, v[i + 1].0)).collect();
        let allow = |i: usize| seg[i] * if v[i].2 { 0.7 } else { 0.9 };
        let mut radii = vec![0.0_f32; n];
        let mut failing: Option<usize> = None;
        for i in 1..n - 1 {
            let d0 = unit(sub(v[i].0, v[i - 1].0));
            let d1 = unit(sub(v[i + 1].0, v[i].0));
            let cos = (d0[0] * d1[0] + d0[1] * d1[1]).clamp(-1.0, 1.0);
            let theta = cos.acos();
            if theta < 0.5_f32.to_radians() {
                continue;
            }
            let tan_half = (theta * 0.5).tan();
            let t_cap = (0.5 * allow(i - 1)).min(0.5 * allow(i));
            let r = (t_cap / tan_half).min(r_pref);
            if r < r_min {
                if std::env::var("RAIL_DEBUG").is_ok() {
                    eprintln!(
                        "  corner {i}/{n} {:?} theta {:.1} deg, tcap {:.0} m, r {:.0} m < {:.0}; seg {:.0}/{:.0}",
                        v[i].1,
                        theta.to_degrees(),
                        t_cap,
                        r,
                        r_min,
                        seg[i - 1],
                        seg[i]
                    );
                }
                failing = Some(i);
                break;
            }
            radii[i] = r;
        }
        match failing {
            None => break radii,
            Some(i) => {
                if v[i].1 == Pin::Free {
                    v.remove(i);
                } else if i > 1 && v[i - 1].1 == Pin::Free {
                    v.remove(i - 1);
                } else if i + 2 < v.len() && v[i + 1].1 == Pin::Free {
                    v.remove(i + 1);
                } else {
                    return None;
                }
            }
        }
    };
    // Build the curve.
    let n = v.len();
    let mut out = vec![v[0].0];
    for i in 1..n.saturating_sub(1) {
        let r = radii[i];
        if r <= 0.0 {
            out.push(v[i].0);
            continue;
        }
        let d0 = unit(sub(v[i].0, v[i - 1].0));
        let d1 = unit(sub(v[i + 1].0, v[i].0));
        let cos = (d0[0] * d1[0] + d0[1] * d1[1]).clamp(-1.0, 1.0);
        let theta = cos.acos();
        let t = r * (theta * 0.5).tan();
        let a = add_scaled(v[i].0, d0, -t);
        let cross = d0[0] * d1[1] - d0[1] * d1[0];
        let sign = if cross >= 0.0 { 1.0 } else { -1.0 };
        let normal = [-d0[1] * sign, d0[0] * sign];
        let centre = add_scaled(a, normal, r);
        let a0 = (a[1] - centre[1]).atan2(a[0] - centre[0]);
        let steps = ((r * theta / 10.0).ceil() as usize).max(2);
        for k in 0..=steps {
            let ang = a0 + sign * theta * k as f32 / steps as f32;
            out.push([centre[0] + r * ang.cos(), centre[1] + r * ang.sin()]);
        }
    }
    out.push(v[n - 1].0);
    Some(out)
}

/// Smallest radius of curvature along a polyline sampled at roughly uniform
/// spacing, from circles through points `span` metres apart.
pub fn min_radius(points: &[P], s: &[f32], span: f32) -> f32 {
    let mut best = f32::INFINITY;
    let n = points.len();
    let mut j = 0;
    let mut k = 0;
    for i in 0..n {
        while j < i && s[i] - s[j] > span {
            j += 1;
        }
        while k + 1 < n && s[k] - s[i] < span {
            k += 1;
        }
        if s[i] - s[j] < span * 0.9 || s[k] - s[i] < span * 0.9 {
            continue;
        }
        let (a, b, c) = (points[j], points[i], points[k]);
        let ab = dist(a, b);
        let bc = dist(b, c);
        let ca = dist(c, a);
        let area2 = ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs();
        if area2 < 1.0e-3 {
            continue;
        }
        best = best.min(ab * bc * ca / (2.0 * area2));
    }
    best
}
