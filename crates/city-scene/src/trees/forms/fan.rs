//! `银杏` — the broad fan on a clean leg.

use super::Architecture;

/// Narrow at the crown's foot, spreading fast to its widest in the upper
/// middle, drawn in again toward the top: a fan opened from a point. The
/// ginkgo's high, elegant branching does the rest.
///
/// The dome is centred above the crown's middle and the exponent is low, so the
/// sides come out *concave* — a fan's sides sweep out from a point — and the
/// curve peaks at exactly 1.0. That low exponent is also what tells a fan from
/// a dome: `round` falls away above its own middle, a fan *holds* its width to
/// near the top and only then draws in, and the two are then unmistakably
/// different shapes at a glance.
pub(super) fn profile(t: f32) -> f32 {
    let x = (t - 0.58) / 0.66;
    (1.0 - x * x).max(0.0).powf(0.30)
}

pub(super) fn architecture() -> Architecture {
    // Seven limbs from mid-height, arching out and only gently up, each
    // forking twice: the sparse, spreading frame a ginkgo's foliage sits in.
    // The table's `density` of 0.62 keeps it see-through.
    Architecture::single(profile, 7, 0.85, 0.99, 0.55, 0.08, 4, 0.82)
}
