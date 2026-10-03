//! A plain C interface, for hosts that are not Rust: a browser loading this as
//! WebAssembly, or anything with a foreign-function interface.
//!
//! Call [`tree_generate`] with a tree's spec; it returns the length of a byte
//! buffer that [`tree_buffer_ptr`] points at, valid until the next call. The buffer
//! is little-endian:
//!
//! | what | layout |
//! | --- | --- |
//! | header | 32 words of 4 bytes (see [`HEADER_WORDS`]) |
//! | segments | `n_segments` × 8 f32: `a.xyz, ra, b.xyz, rb` |
//! | leaves, placement | `n_leaves` × 4 f32: `pos.xyz, length` |
//! | leaves, direction | `n_leaves` × 4 i8: unit blade axis ×127 (4th byte unused) |
//! | leaves, normal | `n_leaves` × 4 i8 |
//! | leaves, shape | `n_leaves` × 4 u8: lobing/teeth, tip sharpness, asymmetry, curl |
//! | leaves, tint | `n_leaves` × 4 u8: hue, shade, age, when it turns |

use std::cell::RefCell;

use crate::grow::{Tree, TreeSpec, grow};
use crate::species::SPECIES;

pub const MAGIC: u32 = 0x4545_5254; // "TREE"
pub const VERSION: u32 = 1;
pub const HEADER_WORDS: usize = 32;

thread_local! {
    static BUFFER: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn f(buffer: &mut Vec<u8>, x: f32) {
    buffer.extend_from_slice(&x.to_le_bytes());
}

fn u(buffer: &mut Vec<u8>, x: u32) {
    buffer.extend_from_slice(&x.to_le_bytes());
}

fn q(x: f32) -> u8 {
    ((x.clamp(-1.0, 1.0) * 127.0).round() as i8) as u8
}

/// Serialise a tree into the interface's buffer layout.
pub fn encode(tree: &Tree) -> Vec<u8> {
    let sp = &SPECIES[tree.spec.species];
    let mut b =
        Vec::with_capacity(HEADER_WORDS * 4 + tree.segments.len() * 32 + tree.leaves.len() * 20);
    u(&mut b, MAGIC);
    u(&mut b, VERSION);
    u(&mut b, tree.segments.len() as u32);
    u(&mut b, tree.leaves.len() as u32);
    for x in [
        tree.height,
        tree.crown_radius,
        tree.crown_base,
        tree.trunk_radius,
    ] {
        f(&mut b, x);
    }
    for x in tree.bark {
        f(&mut b, x);
    }
    for x in [tree.cover, tree.autumn, tree.flush, tree.bloom] {
        f(&mut b, x);
    }
    u(&mut b, tree.spec.species as u32);
    u(&mut b, u32::from(sp.leaf.code()));
    let autumn = sp.autumn.unwrap_or(sp.foliage);
    let bloom = sp.bloom.map_or([1.0, 1.0, 1.0], |x| x.colour);
    for x in sp.foliage.into_iter().chain(autumn).chain(bloom) {
        f(&mut b, x);
    }
    f(&mut b, sp.leaf_aspect);
    f(&mut b, sp.bark.fissure);
    u(&mut b, u32::from(tree.lod));
    while b.len() < HEADER_WORDS * 4 {
        u(&mut b, 0);
    }
    for s in &tree.segments {
        for x in [s.a.x, s.a.y, s.a.z, s.ra, s.b.x, s.b.y, s.b.z, s.rb] {
            f(&mut b, x);
        }
    }
    for l in &tree.leaves {
        for x in [l.pos.x, l.pos.y, l.pos.z, l.length] {
            f(&mut b, x);
        }
    }
    for l in &tree.leaves {
        b.extend_from_slice(&[q(l.dir.x), q(l.dir.y), q(l.dir.z), 0]);
    }
    for l in &tree.leaves {
        b.extend_from_slice(&[q(l.normal.x), q(l.normal.y), q(l.normal.z), 0]);
    }
    for l in &tree.leaves {
        b.extend_from_slice(&l.shape);
    }
    for l in &tree.leaves {
        b.extend_from_slice(&l.tint);
    }
    b
}

#[unsafe(no_mangle)]
pub extern "C" fn tree_species_count() -> u32 {
    SPECIES.len() as u32
}

/// Write species `index`'s key into the buffer; returns its length (0 if no such species).
#[unsafe(no_mangle)]
pub extern "C" fn tree_species_key(index: u32) -> u32 {
    BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        buffer.clear();
        if let Some(s) = SPECIES.get(index as usize) {
            buffer.extend_from_slice(s.key.as_bytes());
        }
        buffer.len() as u32
    })
}

/// Grow a tree and put it in the buffer; returns the buffer's length.
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn tree_generate(
    species: u32,
    seed_lo: u32,
    seed_hi: u32,
    height_m: f32,
    openness: f32,
    age: f32,
    season: f32,
    health: f32,
    lift_m: f32,
    lod: u32,
) -> u32 {
    let species = (species as usize).min(SPECIES.len() - 1);
    let spec = TreeSpec {
        species,
        seed: u64::from(seed_hi) << 32 | u64::from(seed_lo),
        height_m,
        openness,
        age,
        season,
        health,
        lift_m,
    };
    let encoded = encode(&grow(&spec, lod.min(3) as u8));
    BUFFER.with(|buffer| {
        *buffer.borrow_mut() = encoded;
        buffer.borrow().len() as u32
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn tree_buffer_ptr() -> *const u8 {
    BUFFER.with(|buffer| buffer.borrow().as_ptr())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_buffer_has_the_documented_layout_and_round_trips() {
        let n = tree_generate(0, 7, 0, 12.0, 0.7, 0.7, 0.5, 1.0, 0.0, 2);
        let bytes = BUFFER.with(|b| b.borrow().clone());
        assert_eq!(bytes.len() as u32, n);
        let word = |i: usize| u32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap());
        assert_eq!(word(0), MAGIC);
        let (segments, leaves) = (word(2) as usize, word(3) as usize);
        assert!(segments > 100 && leaves > 100);
        assert_eq!(
            bytes.len(),
            HEADER_WORDS * 4 + segments * 32 + leaves * (16 + 4 * 4)
        );
        // The first leaf's direction decodes to a unit vector.
        let at = HEADER_WORDS * 4 + segments * 32 + leaves * 16;
        let d: Vec<f32> = bytes[at..at + 3]
            .iter()
            .map(|&x| f32::from(x as i8) / 127.0)
            .collect();
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((len - 1.0).abs() < 0.05, "{len}");
        assert_eq!(tree_species_key(0), "xiang-zhang".len() as u32);
        assert_eq!(tree_species_count() as usize, SPECIES.len());
    }
}
