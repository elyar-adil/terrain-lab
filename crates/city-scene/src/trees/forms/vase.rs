//! `桃树` — the small spreading vase, in pink blossom every March.

use super::Architecture;

/// Narrow at the foot, widest at the lip: the one shape that gets *wider* as
/// it rises. On a 4 m tree it reads as a spreading urn of blossom.
pub(super) fn profile(t: f32) -> f32 {
    0.10 + 0.90 * t.powf(0.85)
}

pub(super) fn architecture() -> Architecture {
    // Limbs that climb hard out of a short trunk, their reach growing with
    // their height, so the widest point is the lip.
    Architecture::single(profile, 7, 0.70, 0.94, 0.82, 0.12, 4, 0.76)
}
