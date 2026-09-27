//! `桂花` — a dense evergreen oval on a small tree.

use super::Architecture;

/// An upright egg: broadest at the crown's foot, tapering throughout to a
/// narrow, rounded top. Where a dome closes over, an oval keeps drawing in —
/// the silhouette a sheared evergreen actually has.
pub(super) fn profile(t: f32) -> f32 {
    (1.0 - 0.88 * t.powf(1.6)).max(0.08)
}

pub(super) fn architecture() -> Architecture {
    // Few limbs, tight forks: a solid little mass. The table's `density` of
    // 0.90 packs it; the table's small `crown_m` against a taller `height_m`
    // is what makes it an oval rather than a ball.
    Architecture::single(profile, 6, 1.00, 0.86, 0.50, 0.05, 4, 0.76)
}
