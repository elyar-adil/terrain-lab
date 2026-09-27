//! `银杏` — the broad fan on a clean leg.

use super::Architecture;

/// Narrow at the crown's foot, spreading fast to its widest in the middle,
/// drawn in again toward the top: a fan opened from a point. The ginkgo's
/// high, elegant branching does the rest.
pub(super) fn profile(t: f32) -> f32 {
    (t.powf(0.55) * (1.0 - 0.80 * t * t)).max(0.06)
}

pub(super) fn architecture() -> Architecture {
    // Seven limbs from mid-height, arching out and only gently up, each
    // forking twice: the sparse, spreading frame a ginkgo's foliage sits in.
    // The table's `density` of 0.62 keeps it see-through.
    Architecture::single(profile, 7, 0.85, 0.94, 0.55, 0.08, 4, 0.82)
}
