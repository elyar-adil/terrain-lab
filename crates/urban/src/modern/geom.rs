//! Small planar-geometry kit in local metres, used by block extraction and lot
//! layout.  Everything works on `(x, z)` tuples with counter-clockwise rings
//! having positive signed area.

pub(super) type V = (f32, f32);

pub(super) fn signed_area(ring: &[V]) -> f32 {
    let n = ring.len();
    let mut s = 0.0;
    for i in 0..n {
        let a = ring[i];
        let b = ring[(i + 1) % n];
        s += a.0 * b.1 - b.0 * a.1;
    }
    s * 0.5
}

pub(super) fn centroid(ring: &[V]) -> V {
    let a = signed_area(ring);
    if a.abs() < 1.0e-6 {
        let n = ring.len().max(1) as f32;
        return (
            ring.iter().map(|p| p.0).sum::<f32>() / n,
            ring.iter().map(|p| p.1).sum::<f32>() / n,
        );
    }
    let (mut cx, mut cz) = (0.0, 0.0);
    for i in 0..ring.len() {
        let p = ring[i];
        let q = ring[(i + 1) % ring.len()];
        let w = p.0 * q.1 - q.0 * p.1;
        cx += (p.0 + q.0) * w;
        cz += (p.1 + q.1) * w;
    }
    (cx / (6.0 * a), cz / (6.0 * a))
}

pub(super) fn point_in(ring: &[V], q: V) -> bool {
    let mut inside = false;
    let n = ring.len();
    let mut j = n - 1;
    for i in 0..n {
        let (pi, pj) = (ring[i], ring[j]);
        if (pi.1 > q.1) != (pj.1 > q.1) && q.0 < (pj.0 - pi.0) * (q.1 - pi.1) / (pj.1 - pi.1) + pi.0
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

pub(super) fn point_seg_dist(p: V, a: V, b: V) -> f32 {
    let (dx, dz) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dz * dz;
    let t = if len2 < 1.0e-9 {
        0.0
    } else {
        (((p.0 - a.0) * dx + (p.1 - a.1) * dz) / len2).clamp(0.0, 1.0)
    };
    (p.0 - (a.0 + t * dx)).hypot(p.1 - (a.1 + t * dz))
}

fn segments_cross(a: V, b: V, c: V, d: V) -> bool {
    let o = |p: V, q: V, r: V| (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0);
    let (d1, d2, d3, d4) = (o(c, d, a), o(c, d, b), o(a, b, c), o(a, b, d));
    ((d1 > 0.0) != (d2 > 0.0)) && ((d3 > 0.0) != (d4 > 0.0))
}

pub(super) fn seg_seg_dist(a: V, b: V, c: V, d: V) -> f32 {
    if segments_cross(a, b, c, d) {
        return 0.0;
    }
    point_seg_dist(a, c, d)
        .min(point_seg_dist(b, c, d))
        .min(point_seg_dist(c, a, b))
        .min(point_seg_dist(d, a, b))
}

/// Minimum distance between a closed ring and an open polyline (0 if crossing
/// or if the polyline starts inside the ring).
pub(super) fn ring_polyline_dist(ring: &[V], line: &[V]) -> f32 {
    if line.iter().any(|p| point_in(ring, *p)) {
        return 0.0;
    }
    let mut best = f32::MAX;
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        for w in line.windows(2) {
            best = best.min(seg_seg_dist(a, b, w[0], w[1]));
        }
    }
    best
}

/// Keep the part of `ring` where `n · p >= c` (Sutherland–Hodgman).
pub(super) fn clip_half(ring: &[V], n: V, c: f32) -> Vec<V> {
    let mut out = Vec::with_capacity(ring.len() + 2);
    let count = ring.len();
    for i in 0..count {
        let cur = ring[i];
        let nxt = ring[(i + 1) % count];
        let dc = n.0 * cur.0 + n.1 * cur.1 - c;
        let dn = n.0 * nxt.0 + n.1 * nxt.1 - c;
        if dc >= 0.0 {
            out.push(cur);
        }
        if (dc >= 0.0) != (dn >= 0.0) {
            let t = dc / (dc - dn);
            out.push((cur.0 + (nxt.0 - cur.0) * t, cur.1 + (nxt.1 - cur.1) * t));
        }
    }
    dedupe(out)
}

pub(super) fn dedupe(mut ring: Vec<V>) -> Vec<V> {
    ring.dedup_by(|a, b| (a.0 - b.0).abs() < 1.0e-3 && (a.1 - b.1).abs() < 1.0e-3);
    while ring.len() > 1 {
        let (f, l) = (ring[0], ring[ring.len() - 1]);
        if (f.0 - l.0).abs() < 1.0e-3 && (f.1 - l.1).abs() < 1.0e-3 {
            ring.pop();
        } else {
            break;
        }
    }
    ring
}

/// Minimum-area oriented bounding box over the ring's own edge directions.
pub(super) struct Obb {
    pub u: V,
    pub v: V,
    pub min_u: f32,
    pub max_u: f32,
    pub min_v: f32,
    pub max_v: f32,
}

impl Obb {
    pub fn width(&self) -> f32 {
        self.max_u - self.min_u
    }
    pub fn depth(&self) -> f32 {
        self.max_v - self.min_v
    }
    pub fn to_world(&self, u: f32, v: f32) -> V {
        (self.u.0 * u + self.v.0 * v, self.u.1 * u + self.v.1 * v)
    }
}

pub(super) fn obb(ring: &[V]) -> Obb {
    let mut best: Option<(f32, Obb)> = None;
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        let len = (b.0 - a.0).hypot(b.1 - a.1);
        if len < 1.0e-3 {
            continue;
        }
        let u = ((b.0 - a.0) / len, (b.1 - a.1) / len);
        let v = (-u.1, u.0);
        let (mut u0, mut u1, mut v0, mut v1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for p in ring {
            let pu = p.0 * u.0 + p.1 * u.1;
            let pv = p.0 * v.0 + p.1 * v.1;
            u0 = u0.min(pu);
            u1 = u1.max(pu);
            v0 = v0.min(pv);
            v1 = v1.max(pv);
        }
        let area = (u1 - u0) * (v1 - v0);
        if best.as_ref().map(|(a, _)| area < *a).unwrap_or(true) {
            best = Some((
                area,
                Obb {
                    u,
                    v,
                    min_u: u0,
                    max_u: u1,
                    min_v: v0,
                    max_v: v1,
                },
            ));
        }
    }
    best.map(|(_, o)| o).unwrap_or(Obb {
        u: (1.0, 0.0),
        v: (0.0, 1.0),
        min_u: 0.0,
        max_u: 0.0,
        min_v: 0.0,
        max_v: 0.0,
    })
}
