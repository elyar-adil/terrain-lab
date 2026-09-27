//! `柳树` — the willow, and the one unmistakably *weeping* silhouette.

use super::Architecture;

/// A shoulder and a fall: narrow at the crown's foot — the hanging skirt pulls
/// the lower crown in — rising to its widest at the arch's shoulder, then
/// closing gently over the top. The profile bounds the envelope; the curtain
/// itself is `hang`, which turns every second-order branch downward so the
/// shoots pour off the limbs.
pub(super) fn profile(t: f32) -> f32 {
    let rise = 0.42 + 0.58 * (t * 2.0).min(1.0).powf(0.8);
    let fall = (1.0 - 0.55 * ((t - 0.55).max(0.0) * 2.22).powf(1.9)).max(0.05);
    rise * fall
}

pub(super) fn architecture() -> Architecture {
    // Six limbs arch out early and fall (the strong `droop` bows each limb
    // below its attach point), then each forks into six shoots that turn
    // hard *downward* — the curtain. The table's narrow lanceolate cards hang
    // along those shoots.
    let mut a = Architecture::single(profile, 6, 1.15, 0.88, 0.24, 0.82, 6, 1.05);
    a.hang = 0.85;
    a
}
