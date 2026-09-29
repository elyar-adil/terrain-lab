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
        if let Some(e) = unique.iter_mut().find(|e| (e.0.min(e.1), e.0.max(e.1)) == key) {
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
        for e in unique.iter().filter(|e| degree[e.0] <= 1 || degree[e.1] <= 1) {
            spurs.push((pts[e.0], pts[e.1], e.2));
        }
        unique.retain(|e| degree[e.0] > 1 && degree[e.1] > 1);
        if unique.len() == before {
            break;
        }
    }
    let half_count = unique.len() * 2;
    // half-edge h: even = a->b, odd = b->a
    let origin = |h: usize| if h % 2 == 0 { unique[h / 2].0 } else { unique[h / 2].1 };
    let target = |h: usize| if h % 2 == 0 { unique[h / 2].1 } else { unique[h / 2].0 };
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
        faces.push(Face { ring, classes, edge_len });
    }
    Extraction { faces, spurs }
}
