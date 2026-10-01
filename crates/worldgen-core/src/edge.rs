//! Edge labels: how neighbouring cells agree without talking.
//!
//! Two cells that share a side must agree on whatever crosses it: where a road
//! leaves one and enters the next, which way a river runs through the border,
//! what kind of fence stands on it. Neither can wait for the other to be
//! generated first, because the order they are asked for is not theirs to
//! control.
//!
//! So the value on a shared side is a function of the *pair* of cells, and of
//! nothing else, and it is symmetric: asking from either side gives the same
//! answer. Each cell then builds its interior to meet the values on its four
//! sides, and the world is seamless by construction.

use crate::cell::Cell;
use crate::hash::{hash_words, to_unit};
use crate::seed::Seed;

/// A 64-bit label for the side shared by two adjacent cells of one level.
///
/// Symmetric: `edge_hash(s, a, b) == edge_hash(s, b, a)`.
pub fn edge_hash(seed: Seed, a: Cell, b: Cell) -> u64 {
    debug_assert_eq!(a.level, b.level, "cells on a shared side are on one level");
    debug_assert_eq!(
        (a.x - b.x).abs() + (a.y - b.y).abs(),
        1,
        "cells on a shared side are adjacent"
    );
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    hash_words(&[seed.0, u64::from(lo.level), lo.x as u64, lo.y as u64, hi.x as u64, hi.y as u64])
}

/// A position along the shared side, in `[0, 1)`: where something crosses it.
pub fn edge_fraction(seed: Seed, a: Cell, b: Cell) -> f64 {
    to_unit(edge_hash(seed, a, b))
}

/// The four sides of a cell as `(neighbour, edge hash)` in the order west, east,
/// north, south (`-x`, `+x`, `-y`, `+y`).
pub fn sides(seed: Seed, cell: Cell) -> [(Cell, u64); 4] {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].map(|(dx, dy)| {
        let n = cell.neighbour(dx, dy);
        (n, edge_hash(seed, cell, n))
    })
}
