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
    // whole remaining height and forking little and narrowly: everything hugs
    // the trunk and points at the sky.
    //
    // `reach` is how much of the profile a limb actually uses, and it is *not*
    // how narrow the tree looks. The narrowness is the profile's — a column
    // 0.42 of the crown's width at its foot — and the table's `crown_m`, which
    // for a fastigiate poplar is a genuinely small number for a genuinely tall
    // tree. Halving the reach on top of that would build a tree half the width
    // its own record claims, and no test would catch it if the record were
    // loose; the profile and `crown_m` are enough to make the column.
    Architecture::single(profile, 9, 1.00, 0.94, 0.92, 0.05, 2, 0.38)
}
