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
    mix64(a ^ mix64(b).wrapping_add(GOLDEN).wrapping_add(a << 6).wrapping_add(a >> 2))
}

/// Hash a sequence of words; order matters.
pub fn hash_words(words: &[u64]) -> u64 {
    words.iter().fold(0x243F_6A88_85A3_08D3, |acc, w| combine(acc, *w))
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
