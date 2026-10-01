//! The core of the procedural world system.
//!
//! The world is a pure function of (seed, address, what coarser layers decided).
//! This crate provides the pieces that make that true and keep it true:
//!
//! * [`cell`]: hierarchical addresses;
//! * [`hash`] and [`seed`]: derived, stateless randomness;
//! * [`edge`]: labels that let neighbouring cells agree on a shared side;
//! * [`engine`]: layers with declared dependencies, evaluated on demand and cached.
//!
//! It depends on nothing but the standard library, and knows nothing about
//! terrain, roads, trees or any renderer. Those are layers.

pub mod cell;
pub mod edge;
pub mod engine;
pub mod hash;
pub mod seed;

pub use cell::{Cell, Frame, Rect};
pub use edge::{edge_fraction, edge_hash, sides};
pub use engine::{Context, Dependency, Engine, EngineBuilder, Error, Layer, LayerId};
pub use seed::{Rng, Seed};

#[cfg(test)]
mod basics {
    use super::*;
    use crate::hash::{combine, mix64};

    #[test]
    fn an_edge_hash_is_the_same_from_both_sides_and_differs_between_sides() {
        let seed = Seed::new(1);
        let a = Cell::new(5, 3, -2);
        let b = a.neighbour(1, 0);
        assert_eq!(edge_hash(seed, a, b), edge_hash(seed, b, a));
        assert_ne!(edge_hash(seed, a, b), edge_hash(seed, a, a.neighbour(0, 1)));
        assert_ne!(edge_hash(seed, a, b), edge_hash(Seed::new(2), a, b));
        let f = edge_fraction(seed, a, b);
        assert!((0.0..1.0).contains(&f));
        // The four sides of a cell are the same labels its neighbours see.
        for (n, h) in sides(seed, a) {
            assert!(sides(seed, n).iter().any(|(m, g)| *m == a && *g == h));
        }
    }

    #[test]
    fn cells_nest_and_locate_points() {
        let frame = Frame::new([100.0, -50.0], 4096.0);
        let p = [1234.5, 777.7];
        let mut cell = Cell::containing(&frame, p, 6);
        assert!(cell.rect(&frame).contains(p));
        while let Some(parent) = cell.parent() {
            assert!(parent.rect(&frame).contains(p));
            assert!(parent.children().contains(&cell));
            assert_eq!(Cell::containing(&frame, p, parent.level), parent);
            cell = parent;
        }
        assert_eq!(cell.level, 0);
        // Negative coordinates divide towards minus infinity, not towards zero.
        let n = Cell::new(3, -1, -1);
        assert_eq!(n.parent(), Some(Cell::new(2, -1, -1)));
        assert_eq!(Cell::new(3, -3, 5).ancestor(1), Cell::new(1, -1, 1));
    }

    #[test]
    fn derived_seeds_differ_by_label_and_are_stable() {
        let s = Seed::new(42);
        assert_eq!(s.derive("roads"), s.derive("roads"));
        assert_ne!(s.derive("roads"), s.derive("trees"));
        assert_ne!(s.derive_index(1), s.derive_index(2));
        assert_ne!(s.derive_cell(Cell::new(1, 2, 3)), s.derive_cell(Cell::new(1, 3, 2)));
        let mut rng = s.rng();
        let a: Vec<u64> = (0..5).map(|_| rng.next_u64()).collect();
        let mut again = s.rng();
        assert_eq!(a, (0..5).map(|_| again.next_u64()).collect::<Vec<_>>());
        assert!(a.windows(2).all(|w| w[0] != w[1]));
    }

    #[test]
    fn the_hash_spreads_a_single_bit_change_across_the_output() {
        // Avalanche: flipping one input bit changes about half the output bits.
        let mut total = 0u32;
        for bit in 0..64 {
            total += (mix64(0x1234_5678_9ABC_DEF0) ^ mix64(0x1234_5678_9ABC_DEF0 ^ (1 << bit))).count_ones();
        }
        let mean = f64::from(total) / 64.0;
        assert!((26.0..38.0).contains(&mean), "mean flipped bits {mean}");
        assert_ne!(combine(1, 2), combine(2, 1), "order matters");
    }

    #[test]
    fn unit_values_are_uniform_enough() {
        let s = Seed::new(9);
        let n = 20_000;
        let mut buckets = [0u32; 10];
        for i in 0..n {
            buckets[(s.derive_index(i).unit() * 10.0) as usize] += 1;
        }
        for b in buckets {
            assert!((1700..2300).contains(&b), "bucket {b}");
        }
    }
}
