//! `杨树` — the fastigiate poplar: a column whose branches sweep steeply up.

use super::Architecture;

/// Narrow at the foot and opening only slightly to its widest just below the
/// top, where the upward-swept branches fan out. A column, not a cone: the
/// width barely changes for most of the height, which is the whole difference
/// from a spire.
pub(super) fn profile(t: f32) -> f32 {
    0.42 + 0.58 * t.powf(0.65)
}

pub(super) fn architecture() -> Architecture {
    // Nine limbs attached all the way up a trunk that starts branching near
    // the ground (the table's `clear_stem` of 0.18), each climbing almost the
    // whole remaining height at a fraction of the profile's reach, forking
    // little and narrowly: everything hugs the trunk and points at the sky.
    Architecture::single(profile, 9, 1.00, 0.50, 0.92, 0.05, 2, 0.38)
}
