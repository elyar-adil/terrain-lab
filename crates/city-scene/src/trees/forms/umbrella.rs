//! `合欢` — the silk tree: branches rise, then flatten into a wide umbrella.

use super::Architecture;

/// A parabola peaked *at the crown's top*: the branches climb and then run
/// out flat, so the widest point is the top and the outline closes like an
/// open umbrella rather than rounding over like a dome.
pub(super) fn profile(t: f32) -> f32 {
    (1.0 - 0.85 * (1.0 - t) * (1.0 - t)).max(0.05)
}

pub(super) fn architecture() -> Architecture {
    // Seven limbs bunched toward the top of a clear leg (`attach_bias` 0.55),
    // each climbing nearly the whole remaining height and then forking widely
    // — the flat top is the fork spread riding on the peaked profile, and the
    // table's pinnate feather cards finish the haze.
    Architecture::single(profile, 7, 0.55, 0.96, 0.88, 0.10, 5, 0.90)
}
