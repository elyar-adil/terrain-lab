//! `香樟`, `榆树`, `槐树`, `刺槐` — the broad rounded dome.

use super::Architecture;

/// A dome: widest a little above the middle and rounding over above and below
/// it. The reference crown of a broad street tree.
pub(super) fn profile(t: f32) -> f32 {
    let x = (t - 0.45) / 0.70;
    (1.0 - x * x).max(0.0).powf(0.62)
}

pub(super) fn architecture() -> Architecture {
    // Eight limbs arching out of a real trunk. The scholar tree's and the
    // locust's differences from the camphor are proportions (`clear_stem`,
    // `density`, height) in the species table, not a different curve: all four
    // are honestly the same habit, and the palette already separates their
    // colours, their pinnate versus ovate cards and their sizes.
    Architecture::single(profile, 8, 1.00, 0.90, 0.52, 0.05, 5, 0.86)
}
