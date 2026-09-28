//! `桃树` — the small spreading vase, in pink blossom every March.

use super::Architecture;

/// Narrow at the foot, widest at the lip: the one shape that gets *wider* as
/// it rises. On a 4 m tree it reads as a spreading urn of blossom.
/// A trumpet: almost nothing at the foot, opening steadily all the way up, and
/// then **turning back in at the lip** — the last term closes the crown over in
/// its top tenth. That closure is the whole difference between a vase and an
/// umbrella. An umbrella's crown is a flat table that stays wide to its edge; a
/// vase's is a vessel whose branches converge as they meet at the top, so its
/// widest point sits below the crown's top and the top itself is narrower again.
pub(super) fn profile(t: f32) -> f32 {
    (0.08 + 0.92 * t.powf(0.75)) * (1.0 - 0.52 * t.powf(9.0))
}

pub(super) fn architecture() -> Architecture {
    // Limbs that climb hard out of a short trunk, their reach growing with
    // their height, so the widest point is the lip.
    Architecture::single(profile, 7, 0.70, 0.94, 0.82, 0.12, 4, 0.76)
}
