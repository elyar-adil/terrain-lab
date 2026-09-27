//! `梧桐` — tall, broad and *open*, branching high on a clean leg.

use super::Architecture;

/// Widest at the crown's foot — where the first branches leave the clear stem
/// and spread immediately — then a long, steep fall to a rounded top. A dome
/// with its mass *low in the crown*, which is what high branching does to a
/// silhouette, and the opposite order of a dome that rounds over.
pub(super) fn profile(t: f32) -> f32 {
    0.38 + 0.62 * (1.0 - t).max(0.0).powf(0.6)
}

pub(super) fn architecture() -> Architecture {
    // Many limbs, immediate spread, little climb: the crown is a broad,
    // thin-foliaged platform carried high. `density` in the species table does
    // the openness; the clear stem does the height.
    Architecture::single(profile, 9, 1.10, 0.98, 0.42, 0.10, 4, 0.80)
}
