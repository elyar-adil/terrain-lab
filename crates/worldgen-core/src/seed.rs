//! Seeds: derived, never consumed.
//!
//! A [`Seed`] is a number you derive other seeds from by naming what they are
//! for. There is no stream to draw from in order, so two pieces of code that ask
//! for "the seed of building 7 of parcel 3" get the same one without having
//! agreed on anything else. [`Rng`] exists for the places that need many numbers
//! from one key; it is keyed, so it too is the same wherever it is started.

use crate::cell::Cell;
use crate::hash::{below, combine, hash_str, hash_words, mix64, to_unit};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Seed(pub u64);

impl Seed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// A seed for a named purpose ("trees", "roads/arterial").
    pub fn derive(self, label: &str) -> Seed {
        Seed(combine(self.0, hash_str(label)))
    }

    pub fn derive_u64(self, key: u64) -> Seed {
        Seed(combine(self.0, key))
    }

    /// A seed for a numbered thing (the i-th lot, the j-th floor).
    pub fn derive_index(self, index: i64) -> Seed {
        self.derive_u64(index as u64)
    }

    pub fn derive_cell(self, cell: Cell) -> Seed {
        Seed(hash_words(&[self.0, u64::from(cell.level), cell.x as u64, cell.y as u64]))
    }

    /// A number in `[0, 1)`.
    pub fn unit(self) -> f64 {
        to_unit(mix64(self.0))
    }

    pub fn range(self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }

    /// A number in `0..n`.
    pub fn below(self, n: u64) -> u64 {
        below(mix64(self.0), n)
    }

    pub fn chance(self, p: f64) -> bool {
        self.unit() < p
    }

    pub fn rng(self) -> Rng {
        Rng { key: mix64(self.0 ^ 0xA076_1D64_78BD_642F), counter: 0 }
    }
}

/// Many numbers from one key. Deterministic from the seed it started with.
#[derive(Clone, Debug)]
pub struct Rng {
    key: u64,
    counter: u64,
}

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.counter = self.counter.wrapping_add(1);
        mix64(self.key ^ mix64(self.counter.wrapping_mul(0x9E37_79B9_7F4A_7C15)))
    }
    pub fn unit(&mut self) -> f64 {
        to_unit(self.next_u64())
    }
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
    pub fn below(&mut self, n: u64) -> u64 {
        below(self.next_u64(), n)
    }
    pub fn chance(&mut self, p: f64) -> bool {
        self.unit() < p
    }
}
