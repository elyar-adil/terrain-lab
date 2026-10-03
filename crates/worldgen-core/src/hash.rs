//! Hashing: the only source of randomness in the system.
//!
//! Nothing here keeps state. A value is a function of the integers hashed into
//! it, so it comes out the same whenever and in whatever order it is asked for.

const GOLDEN: u64 = 0x9E37_79B9_7F4A_7C15;

/// The SplitMix64 finaliser: a bijection on `u64` with full avalanche.
#[inline]
pub fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Combine two hashes; order matters.
#[inline]
pub fn combine(a: u64, b: u64) -> u64 {
    mix64(
        a ^ mix64(b)
            .wrapping_add(GOLDEN)
            .wrapping_add(a << 6)
            .wrapping_add(a >> 2),
    )
}

/// Hash a sequence of words; order matters.
pub fn hash_words(words: &[u64]) -> u64 {
    words
        .iter()
        .fold(0x243F_6A88_85A3_08D3, |acc, w| combine(acc, *w))
}

/// Hash a string (FNV-1a, then mixed). Used for labels, which are constants.
pub fn hash_str(text: &str) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for byte in text.as_bytes() {
        h ^= u64::from(*byte);
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    mix64(h)
}

/// The top 53 bits of a hash as a number in `[0, 1)`.
#[inline]
pub fn to_unit(h: u64) -> f64 {
    (h >> 11) as f64 * (1.0 / (1_u64 << 53) as f64)
}

/// A hash reduced to `0..n` without modulo bias worth caring about (multiply-high).
#[inline]
pub fn below(h: u64, n: u64) -> u64 {
    ((u128::from(h) * u128::from(n)) >> 64) as u64
}

// -- 32-bit lattice hashes ---------------------------------------------------
//
// The legacy layers (terrain, textures, tile bakes) hash small integer lattice
// coordinates to a float in a hot loop. These are the shared 32-bit versions.

/// The 32-bit avalanche finaliser (every input bit affects every output bit).
#[inline]
pub const fn mix32(mut v: u32) -> u32 {
    v ^= v >> 16;
    v = v.wrapping_mul(0x7feb_352d);
    v ^= v >> 15;
    v = v.wrapping_mul(0x846c_a68b);
    v ^ (v >> 16)
}

/// A float in `[0, 1]` from a lattice cell: **cheap**, a two-round mix.
///
/// This is the hash the texture bakes have always used, kept bit-for-bit so
/// existing output does not move. Its weakness is real: for consecutive small
/// integers (a bay index, a tile column) the outputs are correlated, and eight
/// neighbouring cells can show only three distinct values. Use [`avalanche01`]
/// for anything that decides between a handful of discrete options.
#[inline]
pub fn cell01(seed: u32, x: i32, y: i32, salt: u32) -> f32 {
    let mut v = seed
        ^ (x as u32).wrapping_mul(0x9e37_79b9)
        ^ (y as u32).wrapping_mul(0x85eb_ca6b)
        ^ salt.wrapping_mul(0xc2b2_ae35);
    v ^= v >> 16;
    v = v.wrapping_mul(0x7feb_352d);
    v ^= v >> 15;
    v as f32 / u32::MAX as f32
}

/// A float in `[0, 1]` from a lattice cell through the full [`mix32`]: slower than
/// [`cell01`], and uncorrelated between neighbouring cells.
#[inline]
pub fn avalanche01(seed: u32, x: i32, y: i32, salt: u32) -> f32 {
    let v = mix32(
        seed ^ (x as u32).wrapping_mul(0x9e37_79b9)
            ^ (y as u32).wrapping_mul(0x85eb_ca6b)
            ^ salt.wrapping_mul(0xc2b2_ae35),
    );
    v as f32 / u32::MAX as f32
}

#[cfg(test)]
mod hash32_tests {
    use super::*;

    #[test]
    fn neighbouring_cells_are_not_correlated_under_the_avalanche_hash() {
        // The regression behind the shop-sign bug: eight consecutive cells must
        // not collapse onto a few values.
        let mut distinct: Vec<u32> = (0..8)
            .map(|bay| (avalanche01(977, bay, 0, 701) * 6.0) as u32)
            .collect();
        distinct.sort_unstable();
        distinct.dedup();
        assert!(
            distinct.len() >= 4,
            "only {} of 6 buckets hit",
            distinct.len()
        );
    }

    #[test]
    fn both_hashes_stay_in_the_unit_interval() {
        for i in -50..50 {
            for f in [cell01(7, i, 3 * i, 1), avalanche01(7, i, 3 * i, 1)] {
                assert!((0.0..=1.0).contains(&f));
            }
        }
    }

    #[test]
    fn mix32_is_a_bijection_on_a_sample() {
        let mut seen = std::collections::HashSet::new();
        assert!((0..10_000_u32).all(|i| seen.insert(mix32(i))));
    }
}
