//! `榕树` — the banyan: a very wide dense dome and its aerial roots.

use super::Architecture;

/// Broad and nearly cylindrical: the crown holds almost its full width from
/// foot to top and closes only at the very top. A banyan's dome is wider than
/// it is tall in spirit — the table's `crown_m` carries that — and the aerial
/// roots (`aerial`) hang the silhouette to the ground.
pub(super) fn profile(t: f32) -> f32 {
    (1.0 - 0.28 * t).max(0.15)
}

pub(super) fn architecture() -> Architecture {
    // Nine limbs spread wide off a short, thick trunk, each forking five ways
    // into a solid mass (`density` 0.86 in the table); five aerial roots drop
    // from the major limbs to the ground as thin flared columns.
    let mut a = Architecture::single(profile, 9, 1.00, 0.98, 0.50, 0.06, 5, 0.88);
    a.aerial = 5;
    a
}
