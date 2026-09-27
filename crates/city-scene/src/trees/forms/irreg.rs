//! `松树` — the pine: a leaning, tortuous trunk, irregular flat needle
//! clusters, gaps in the outline.

use super::Architecture;

/// A lumpy, wide-shouldered outline: broad and irregular through the middle,
/// where the needle plates are, thinning both ways. The lumpiness is real
/// profile, not noise on the cards — a pine's outline *is* irregular, and the
/// `sin` term makes every tier's reach differ, which is what reads as gaps.
pub(super) fn profile(t: f32) -> f32 {
    (1.0 - 0.42 * t).max(0.22) * (1.0 + 0.14 * (t * 9.0).sin())
}

pub(super) fn architecture() -> Architecture {
    // Five heavy limbs, bunched low, each forking widely into clusters; the
    // trunk leans 5-15 degrees (`lean`), the foliage clusters flatten into
    // needle plates (`tuft`), and the table's low `density` leaves the gaps.
    let mut a = Architecture::single(profile, 5, 1.25, 0.92, 0.45, 0.16, 4, 0.95);
    a.lean = 0.19;
    a.tuft = 0.55;
    a
}
