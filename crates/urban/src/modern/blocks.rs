//! Planar-face extraction: city blocks are the bounded faces of the road graph,
//! not cells of an assumed grid.  Whatever the road generator produces —
//! diagonals, a ring road, dropped local streets, a river gap — the blocks
//! follow it, so lots can never be laid across a street.

use super::geom::{V, signed_area};
use crate::ModernRoadClass;

pub(super) struct Face {
    /// Counter-clockwise ring in local metres.
    pub ring: Vec<V>,
    /// Class of the road along edge `i` (`ring[i] -> ring[i+1]`).
    pub classes: Vec<ModernRoadClass>,
    pub edge_len: Vec<f32>,
    /// True where edge `i` is a cut made when a concave face was split into
    /// convex pieces, not a street: it carries no right-of-way.
    pub open: Vec<bool>,
}

/// Faces plus the dangling streets that bound no block.  A cul-de-sac still
/// occupies a right-of-way, so the lot layout must keep clear of it.
pub(super) struct Extraction {
    pub faces: Vec<Face>,
    pub spurs: Vec<(V, V, ModernRoadClass)>,
}

pub(super) fn extract_faces(pts: &[V], edges: &[(usize, usize, ModernRoadClass)]) -> Extraction {
    // Deduplicate parallel edges, keeping the widest class.
    let mut unique: Vec<(usize, usize, ModernRoadClass)> = Vec::new();
    for &(a, b, c) in edges {
        if a == b {
            continue;
        }
        let key = (a.min(b), a.max(b));
        if let Some(e) = unique
            .iter_mut()
            .find(|e| (e.0.min(e.1), e.0.max(e.1)) == key)
        {
            if (c as i32) > (e.2 as i32) {
                e.2 = c;
            }
        } else {
            unique.push((a, b, c));
        }
    }
    let mut spurs: Vec<(V, V, ModernRoadClass)> = Vec::new();
    // Prune dangling streets: a cul-de-sac bounds no block and would make a
    // face traverse the spur twice.
    loop {
        let mut degree = vec![0_u32; pts.len()];
        for e in &unique {
            degree[e.0] += 1;
            degree[e.1] += 1;
        }
        let before = unique.len();
        for e in unique
            .iter()
            .filter(|e| degree[e.0] <= 1 || degree[e.1] <= 1)
        {
            spurs.push((pts[e.0], pts[e.1], e.2));
        }
        unique.retain(|e| degree[e.0] > 1 && degree[e.1] > 1);
        if unique.len() == before {
            break;
        }
    }
    let half_count = unique.len() * 2;
    // half-edge h: even = a->b, odd = b->a
    let origin = |h: usize| {
        if h.is_multiple_of(2) {
            unique[h / 2].0
        } else {
            unique[h / 2].1
        }
    };
    let target = |h: usize| {
        if h.is_multiple_of(2) {
            unique[h / 2].1
        } else {
            unique[h / 2].0
        }
    };
    let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); pts.len()];
    for h in 0..half_count {
        outgoing[origin(h)].push(h);
    }
    let angle = |h: usize| {
        let (a, b) = (pts[origin(h)], pts[target(h)]);
        (b.1 - a.1).atan2(b.0 - a.0)
    };
    for list in outgoing.iter_mut() {
        list.sort_by(|x, y| angle(*x).total_cmp(&angle(*y)));
    }
    let mut slot = vec![0_usize; half_count];
    for list in &outgoing {
        for (i, h) in list.iter().enumerate() {
            slot[*h] = i;
        }
    }
    let next = |h: usize| {
        let twin = h ^ 1;
        let list = &outgoing[origin(twin)];
        list[(slot[twin] + list.len() - 1) % list.len()]
    };
    let mut seen = vec![false; half_count];
    let mut faces = Vec::new();
    for start in 0..half_count {
        if seen[start] {
            continue;
        }
        let mut ring = Vec::new();
        let mut classes = Vec::new();
        let mut h = start;
        let mut guard = 0;
        loop {
            seen[h] = true;
            ring.push(pts[origin(h)]);
            classes.push(unique[h / 2].2);
            h = next(h);
            guard += 1;
            if h == start || guard > half_count {
                break;
            }
        }
        if h != start || ring.len() < 3 {
            continue;
        }
        if signed_area(&ring) <= 1.0 {
            continue; // outer face or a sliver
        }
        let n = ring.len();
        let edge_len = (0..n)
            .map(|i| {
                let (a, b) = (ring[i], ring[(i + 1) % n]);
                (b.0 - a.0).hypot(b.1 - a.1)
            })
            .collect();
        let open = vec![false; ring.len()];
        faces.push(Face {
            ring,
            classes,
            edge_len,
            open,
        });
    }
    Extraction { faces, spurs }
}

/// Turn at vertex `b` of `a -> b -> c`, positive for a left (convex, for a
/// counter-clockwise ring) turn, in radians.
fn turn(a: V, b: V, c: V) -> f32 {
    let (u, v) = ((b.0 - a.0, b.1 - a.1), (c.0 - b.0, c.1 - b.1));
    (u.0 * v.1 - u.1 * v.0).atan2(u.0 * v.0 + u.1 * v.1)
}

/// A boundary bend sharper than this (radians, towards the inside of the face)
/// makes the face concave enough that half-plane setbacks would eat the block.
const REFLEX_TOL: f32 = 0.21;

fn point_in_tri(p: V, a: V, b: V, c: V) -> bool {
    let s = |p: V, q: V, r: V| (q.0 - p.0) * (r.1 - p.1) - (q.1 - p.1) * (r.0 - p.0);
    let (d1, d2, d3) = (s(a, b, p), s(b, c, p), s(c, a, p));
    d1 > 1.0e-4 && d2 > 1.0e-4 && d3 > 1.0e-4
}

/// Ear-clip a simple counter-clockwise ring into triangles of ring indices.
fn triangulate(ring: &[V]) -> Vec<[usize; 3]> {
    let mut idx: Vec<usize> = (0..ring.len()).collect();
    let mut tris = Vec::new();
    while idx.len() > 3 {
        let m = idx.len();
        let mut pick = None;
        for k in 0..m {
            let (ia, ib, ic) = (idx[(k + m - 1) % m], idx[k], idx[(k + 1) % m]);
            let (a, b, c) = (ring[ia], ring[ib], ring[ic]);
            if (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0) <= 1.0e-4 {
                continue;
            }
            if idx
                .iter()
                .any(|&o| o != ia && o != ib && o != ic && point_in_tri(ring[o], a, b, c))
            {
                continue;
            }
            pick = Some(k);
            break;
        }
        let k = pick.unwrap_or(0);
        let m = idx.len();
        tris.push([idx[(k + m - 1) % m], idx[k], idx[(k + 1) % m]]);
        idx.remove(k);
    }
    if idx.len() == 3 {
        tris.push([idx[0], idx[1], idx[2]]);
    }
    tris
}

/// Split a concave face into near-convex pieces (Hertel-Mehlhorn merge of an
/// ear-clipped triangulation).  Edges along the original boundary keep their
/// street class; diagonals become `open`.
pub(super) fn split_concave(face: Face) -> Vec<Face> {
    let n = face.ring.len();
    let ring = &face.ring;
    let convexish =
        (0..n).all(|i| turn(ring[(i + n - 1) % n], ring[i], ring[(i + 1) % n]) > -REFLEX_TOL);
    if convexish || n < 4 {
        return vec![face];
    }
    let mut polys: Vec<Vec<usize>> = triangulate(ring).into_iter().map(|t| t.to_vec()).collect();
    'merge: loop {
        for pi in 0..polys.len() {
            for qi in (pi + 1)..polys.len() {
                let (p, q) = (&polys[pi], &polys[qi]);
                for x in 0..p.len() {
                    let (u, v) = (p[x], p[(x + 1) % p.len()]);
                    let Some(y) = (0..q.len()).find(|&y| q[y] == v && q[(y + 1) % q.len()] == u)
                    else {
                        continue;
                    };
                    // p rotated to start at v and end at u; q rotated to start at u and end at v.
                    let mut merged: Vec<usize> =
                        (0..p.len()).map(|k| p[(x + 1 + k) % p.len()]).collect();
                    let qrot: Vec<usize> = (0..q.len()).map(|k| q[(y + 1 + k) % q.len()]).collect();
                    merged.extend_from_slice(&qrot[1..qrot.len() - 1]);
                    let m = merged.len();
                    let ok = (0..m).all(|k| {
                        turn(
                            ring[merged[(k + m - 1) % m]],
                            ring[merged[k]],
                            ring[merged[(k + 1) % m]],
                        ) > -REFLEX_TOL
                    });
                    if ok {
                        polys[pi] = merged;
                        polys.remove(qi);
                        continue 'merge;
                    }
                }
            }
        }
        break;
    }
    polys
        .into_iter()
        .map(|poly| {
            let m = poly.len();
            let pts: Vec<V> = poly.iter().map(|&i| ring[i]).collect();
            let mut classes = Vec::with_capacity(m);
            let mut open = Vec::with_capacity(m);
            let mut edge_len = Vec::with_capacity(m);
            for k in 0..m {
                let (a, b) = (poly[k], poly[(k + 1) % m]);
                let original = b == (a + 1) % n;
                classes.push(face.classes[a]);
                open.push(!original);
                let (pa, pb) = (ring[a], ring[b]);
                edge_len.push((pb.0 - pa.0).hypot(pb.1 - pa.1));
            }
            Face {
                ring: pts,
                classes,
                edge_len,
                open,
            }
        })
        .collect()
}
