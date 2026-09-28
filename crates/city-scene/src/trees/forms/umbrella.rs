//! `合欢` — the silk tree: branches rise, then flatten into a wide umbrella.

use super::Architecture;

/// A flat table that stays wide to its edge: the branches climb and then run out
/// horizontally, so the crown holds almost its full width across a broad band
/// near the top and the outline closes like an open umbrella rather than
/// rounding over like a dome.
///
/// The exponent is above 1, which is what makes the sides *convex* — a silk
/// tree's crown fills outward fast and then flattens — and it is exactly the
/// opposite curvature to [`super::vase`], which is why the two are told apart by
/// their shape rather than by a fudge factor.
pub(super) fn profile(t: f32) -> f32 {
    (1.0 - 0.94 * (1.0 - t).powf(5.0)).max(0.04)
}

pub(super) fn architecture() -> Architecture {
    // Seven limbs bunched toward the top of a clear leg (`attach_bias` 0.55),
    // each climbing nearly the whole remaining height and then forking widely
    // — the flat top is the fork spread riding on the peaked profile, and the
    // table's pinnate feather cards finish the haze.
    Architecture::single(profile, 7, 0.55, 0.96, 0.88, 0.10, 5, 0.90)
}
