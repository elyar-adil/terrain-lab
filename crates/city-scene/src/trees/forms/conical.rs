//! `水杉`, `杉树` — the narrow conical spire.

use super::Architecture;

/// Widest just above the crown's foot, then a straight taper to a leader that
/// keeps going. The `0.94` leaves a spike so the spire is a spire.
pub(super) fn profile(t: f32) -> f32 {
    (1.0 - t * 0.94).max(0.05).powf(0.72)
}

pub(super) fn architecture() -> Architecture {
    // Ten whorls of four *short* branches, each a little shorter than the one
    // below, lifting slightly and drooping only a little: a tight, feathery
    // cone with a straight leader, not a stack of plates. The dawn redwood's
    // clear stem in the species table keeps the crown off the ground; the fir's
    // shorter one and denser packing make the same cone read solid.
    Architecture::whorled(profile, 10, 4, 0.85, 0.14, 0.20, 2, 0.50)
}
