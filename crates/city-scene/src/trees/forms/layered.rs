//! `雪松` — the deodar cedar: conical, in whorled horizontal tiers that droop
//! at the tips.

use super::Architecture;

/// A cone with convex sides: widest at the foot, holding its width low down
/// and tapering in a bulge rather than on a straight line. That convexity is
/// the difference between a cedar's tiers and a metasequoia's straight taper
/// — measured, not asserted: the two profiles differ by more than a quarter
/// of the crown's reach in every height band.
pub(super) fn profile(t: f32) -> f32 {
    (1.0 - 0.94 * t * t).max(0.05).powf(0.62)
}

pub(super) fn architecture() -> Architecture {
    // Nine whorls of five long, nearly horizontal branches — the tiers — each
    // dropping at the tip (`droop` bows every branch's middle down hard), with
    // the table's low `clear_stem` letting the lowest tier hang near the lawn.
    Architecture::whorled(profile, 9, 5, 0.97, 0.10, 0.42, 2, 0.52)
}
