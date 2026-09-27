//! Canopy forms: the silhouette each [`Canopy`] value is grown into.
//!
//! A form is a *record*, not a code path: the profile curve that is the crown's
//! outline, and the numbers that arrange the wood inside it. One module per
//! form, one file per silhouette, so the difference between a `雪松` and a
//! `水杉` is a file you can read rather than a branch you must trace. The
//! growing machinery that interprets these records lives in
//! [`crate::trees::grow`].

mod banyan;
mod conical;
mod fan;
mod fastigiate;
mod irreg;
mod layered;
mod open;
mod oval;
mod round;
mod umbrella;
mod vase;
mod weeping;

use crate::species::Canopy;

/// Crown half-width at normalised crown height: 0 at the crown's foot, 1 at its
/// top. This *is* the silhouette, and each form's is a different curve.
pub(crate) type Profile = fn(f32) -> f32;

/// How the wood is arranged. Three arrangements cover the twelve forms: a
/// single trunk that forks into limbs, whorled tiers, or several stems from the
/// base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Layout {
    /// One trunk, then limbs that arch out and up.
    Single,
    /// Whorled tiers of nearly horizontal branches — a conifer.
    Whorled,
}

/// The numbers one canopy form needs in order to be itself. Every field is
/// architecture; none of it is colour, and none of it is a tint.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Architecture {
    pub(crate) profile: Profile,
    pub(crate) layout: Layout,
    /// Primary limbs off the trunk (or off each stem).
    pub(crate) primary: usize,
    /// Exponent on a limb's normalised attachment height. Below 1 the limbs
    /// bunch toward the top (a vase's lip, an umbrella's flat top); above 1
    /// they bunch at the foot (a low-branched banyan).
    pub(crate) attach_bias: f32,
    /// Horizontal reach of a primary limb, as a fraction of the reach the crown
    /// profile allows.
    pub(crate) reach: f32,
    /// How much of the remaining height a primary limb climbs.
    pub(crate) climb: f32,
    /// Downward bend of a branch's middle, as a fraction of its length. This is
    /// what separates a cedar's drooping tier from a metasequoia's ascending
    /// one.
    pub(crate) droop: f32,
    /// Secondary branches per primary limb.
    pub(crate) fork: usize,
    /// Splay of a secondary branch from its parent's axis.
    pub(crate) fork_reach: f32,
    /// Whorls, and branches per whorl (`Layout::Whorled` only).
    pub(crate) tiers: usize,
    pub(crate) per_tier: usize,
    /// Trunk lean, as a horizontal offset at the top in fractions of the
    /// height. 0 grows an upright trunk; the pine leans through its crown.
    pub(crate) lean: f32,
    /// How far second-order branches turn *downward* from their parent's
    /// direction, 0..1. The willow's curtain.
    pub(crate) hang: f32,
    /// How much a foliage cluster is flattened vertically, 0..1. The pine's
    /// needle plates.
    pub(crate) tuft: f32,
    /// Aerial root columns dropping from major limbs to the ground. The
    /// banyan's.
    pub(crate) aerial: usize,
}

impl Architecture {
    /// A single-trunk form: one leader, `primary` limbs off it.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn single(
        profile: Profile,
        primary: usize,
        attach_bias: f32,
        reach: f32,
        climb: f32,
        droop: f32,
        fork: usize,
        fork_reach: f32,
    ) -> Self {
        Self {
            profile,
            layout: Layout::Single,
            primary,
            attach_bias,
            reach,
            climb,
            droop,
            fork,
            fork_reach,
            tiers: 0,
            per_tier: 0,
            lean: 0.0,
            hang: 0.0,
            tuft: 0.0,
            aerial: 0,
        }
    }

    /// A whorled form: a clear leader with `tiers` whorls of `per_tier`.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn whorled(
        profile: Profile,
        tiers: usize,
        per_tier: usize,
        reach: f32,
        climb: f32,
        droop: f32,
        fork: usize,
        fork_reach: f32,
    ) -> Self {
        Self {
            profile,
            layout: Layout::Whorled,
            primary: per_tier,
            attach_bias: 1.0,
            reach,
            climb,
            droop,
            fork,
            fork_reach,
            tiers,
            per_tier,
            lean: 0.0,
            hang: 0.0,
            tuft: 0.0,
            aerial: 0,
        }
    }
}

/// The form record a canopy value grows into.
pub(crate) fn architecture(canopy: Canopy) -> Architecture {
    match canopy {
        Canopy::Rounded => round::architecture(),
        Canopy::Open => open::architecture(),
        Canopy::Oval => oval::architecture(),
        Canopy::Fan => fan::architecture(),
        Canopy::Vase => vase::architecture(),
        Canopy::Conical => conical::architecture(),
        Canopy::Layered => layered::architecture(),
        Canopy::Weeping => weeping::architecture(),
        Canopy::Fastigiate => fastigiate::architecture(),
        Canopy::Irregular => irreg::architecture(),
        Canopy::Umbrella => umbrella::architecture(),
        Canopy::Banyan => banyan::architecture(),
    }
}
