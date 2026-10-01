//! Growth templates: how each habit builds a crown.
//!
//! These numbers are the *architecture*, shared by every tree of a habit. What makes
//! one tree different from the next is not here: each tree draws its own limb angles,
//! lengths, bends, counts and leaves from its own seed, around these means.

use crate::species::Habit;

#[derive(Debug, Clone, Copy)]
pub struct Architecture {
    /// 0 = the trunk gives way to limbs (a broadleaf), 1 = it runs to the top (a spire).
    pub leader: f32,
    /// Trunk radius falls as `(1 - h/H)^taper`.
    pub trunk_taper: f32,
    /// Random bend of the trunk, radians per metre.
    pub trunk_wobble: f32,
    /// Largest lean of the trunk, radians.
    pub lean: f32,
    /// Limbs per metre of crown stem (a whorled conifer has several per whorl).
    pub limb_per_m: f32,
    /// Angle of a limb from the trunk axis at the crown base and at the crown top, radians.
    pub limb_down: (f32, f32),
    /// Limb length as a share of what the crown envelope allows.
    pub limb_len: f32,
    /// Droop of limbs and branches under their own weight (metres per metre of length squared).
    pub limb_sag: f32,
    /// Upward sweep of limbs (a poplar's hug the trunk).
    pub limb_up: f32,
    /// Limb radius at its base as a share of the trunk's radius there.
    pub limb_radius: f32,
    pub branch_per_m: f32,
    pub branch_down: (f32, f32),
    pub branch_len: f32,
    pub branch_sag: f32,
    pub branch_up: f32,
    pub twig_per_m: f32,
    pub twig_len: (f32, f32),
    /// Limbs per whorl; 0 means a spiral (golden-angle) arrangement.
    pub whorl: u8,
    /// How uneven the crown is from one side to the other, 0..1.
    pub lobing: f32,
    /// How far the tips of limbs and branches fall back down (a willow's shoots).
    pub tip_droop: f32,
    /// Twigs per metre carried directly on limbs: a conifer is clothed along its whole limb.
    pub limb_twigs: f32,
}

const D: f32 = std::f32::consts::PI / 180.0;

pub fn architecture(habit: Habit) -> Architecture {
    let base = Architecture {
        leader: 0.30,
        trunk_taper: 0.85,
        trunk_wobble: 0.020,
        lean: 4.0 * D,
        limb_per_m: 1.05,
        limb_down: (62.0 * D, 38.0 * D),
        limb_len: 1.0,
        limb_sag: 0.010,
        limb_up: 0.25,
        limb_radius: 0.50,
        branch_per_m: 3.0,
        branch_down: (58.0 * D, 38.0 * D),
        branch_len: 0.62,
        branch_sag: 0.020,
        branch_up: 0.20,
        twig_per_m: 7.0,
        twig_len: (0.22, 0.60),
        whorl: 0,
        lobing: 0.20,
        tip_droop: 0.0,
        limb_twigs: 0.0,
    };
    match habit {
        Habit::Rounded => base,
        Habit::Open => Architecture {
            limb_per_m: 0.85,
            limb_down: (58.0 * D, 34.0 * D),
            limb_len: 1.05,
            branch_per_m: 1.7,
            twig_per_m: 3.0,
            lobing: 0.28,
            ..base
        },
        Habit::Oval => Architecture {
            leader: 0.5,
            limb_per_m: 1.5,
            limb_down: (52.0 * D, 30.0 * D),
            branch_per_m: 2.6,
            twig_per_m: 4.6,
            ..base
        },
        Habit::Fan => Architecture {
            leader: 0.35,
            limb_per_m: 0.95,
            limb_down: (64.0 * D, 30.0 * D),
            limb_up: 0.35,
            branch_per_m: 1.9,
            twig_per_m: 2.8,
            twig_len: (0.10, 0.32),
            ..base
        },
        Habit::Vase => Architecture {
            leader: 0.10,
            trunk_wobble: 0.035,
            lean: 7.0 * D,
            limb_per_m: 1.1,
            limb_down: (38.0 * D, 28.0 * D),
            limb_up: 0.45,
            limb_radius: 0.62,
            branch_per_m: 2.4,
            lobing: 0.30,
            ..base
        },
        Habit::Conical => Architecture {
            leader: 1.0,
            trunk_taper: 1.05,
            trunk_wobble: 0.006,
            lean: 1.5 * D,
            limb_per_m: 1.6,
            limb_down: (84.0 * D, 62.0 * D),
            limb_len: 1.0,
            limb_sag: 0.006,
            limb_up: 0.05,
            limb_radius: 0.30,
            branch_per_m: 3.4,
            branch_down: (68.0 * D, 52.0 * D),
            branch_len: 0.42,
            twig_per_m: 7.0,
            twig_len: (0.14, 0.34),
            lobing: 0.08,
            limb_twigs: 3.5,
            ..base
        },
        Habit::Layered => Architecture {
            leader: 1.0,
            trunk_taper: 1.0,
            trunk_wobble: 0.008,
            lean: 2.0 * D,
            limb_per_m: 4.0,
            limb_down: (88.0 * D, 74.0 * D),
            limb_sag: 0.020,
            limb_up: 0.0,
            limb_radius: 0.28,
            branch_per_m: 3.0,
            branch_down: (74.0 * D, 62.0 * D),
            branch_len: 0.40,
            branch_sag: 0.035,
            twig_per_m: 6.0,
            twig_len: (0.18, 0.42),
            whorl: 6,
            lobing: 0.10,
            tip_droop: 0.35,
            limb_twigs: 3.5,
            ..base
        },
        Habit::Weeping => Architecture {
            leader: 0.05,
            limb_per_m: 1.0,
            limb_down: (56.0 * D, 40.0 * D),
            limb_len: 1.05,
            limb_up: 0.55,
            branch_per_m: 2.6,
            branch_sag: 0.060,
            branch_len: 0.55,
            twig_per_m: 5.0,
            twig_len: (0.9, 2.4),
            lobing: 0.15,
            tip_droop: 1.0,
            limb_twigs: 1.6,
            ..base
        },
        Habit::Fastigiate => Architecture {
            leader: 0.85,
            trunk_taper: 1.0,
            limb_per_m: 1.7,
            limb_down: (34.0 * D, 18.0 * D),
            limb_len: 0.95,
            limb_sag: 0.0,
            limb_up: 0.70,
            limb_radius: 0.38,
            branch_per_m: 2.4,
            branch_down: (38.0 * D, 26.0 * D),
            branch_len: 0.46,
            branch_up: 0.4,
            twig_per_m: 4.4,
            lobing: 0.10,
            ..base
        },
        Habit::Irregular => Architecture {
            leader: 0.3,
            trunk_wobble: 0.06,
            lean: 12.0 * D,
            limb_per_m: 0.95,
            limb_down: (78.0 * D, 54.0 * D),
            limb_len: 1.15,
            limb_sag: 0.008,
            branch_per_m: 2.4,
            branch_len: 0.5,
            twig_per_m: 6.0,
            twig_len: (0.18, 0.40),
            lobing: 0.45,
            limb_twigs: 1.2,
            ..base
        },
        Habit::Umbrella => Architecture {
            leader: 0.15,
            limb_per_m: 0.80,
            limb_down: (72.0 * D, 46.0 * D),
            limb_len: 1.1,
            limb_up: 0.55,
            branch_per_m: 1.5,
            branch_down: (66.0 * D, 40.0 * D),
            twig_per_m: 2.6,
            twig_len: (0.20, 0.55),
            lobing: 0.20,
            ..base
        },
        Habit::Banyan => Architecture {
            leader: 0.05,
            trunk_taper: 0.55,
            lean: 2.0 * D,
            limb_per_m: 0.9,
            limb_down: (74.0 * D, 56.0 * D),
            limb_len: 1.15,
            limb_radius: 0.62,
            limb_sag: 0.004,
            branch_per_m: 2.2,
            twig_per_m: 4.4,
            lobing: 0.12,
            ..base
        },
    }
}

/// How far the crown reaches at relative height `h` (0 at the crown's foot, 1 at its
/// top), as a share of the crown's greatest radius.
pub fn profile(habit: Habit, h: f32) -> f32 {
    let h = h.clamp(0.0, 1.0);
    let dome = |peak: f32, round: f32| {
        // A smooth bump with its widest point at `peak`, falling to a rounded top and foot.
        let x = if h < peak { h / peak } else { (1.0 - h) / (1.0 - peak) };
        (1.0 - (1.0 - x).powf(round)).max(0.0).powf(0.55)
    };
    match habit {
        Habit::Rounded => 0.18 + 0.82 * dome(0.46, 1.9),
        Habit::Open => 0.20 + 0.80 * dome(0.58, 1.6),
        Habit::Oval => 0.15 + 0.85 * dome(0.48, 1.7),
        Habit::Fan => 0.12 + 0.88 * dome(0.58, 1.5),
        Habit::Vase => 0.30 + 0.70 * h.powf(0.7) * (1.0 - 0.18 * h.powi(4)),
        Habit::Conical => (1.0 - h).powf(0.92) * 0.97 + 0.03,
        Habit::Layered => {
            let tiers = 6.0;
            let step = 0.84 + 0.16 * (h * tiers * std::f32::consts::TAU).cos().abs();
            ((1.0 - h).powf(0.85) * 0.97 + 0.03) * step
        }
        Habit::Weeping => 0.25 + 0.75 * dome(0.42, 1.6),
        Habit::Fastigiate => 0.40 + 0.60 * dome(0.50, 1.4),
        Habit::Irregular => 0.20 + 0.80 * dome(0.50, 1.5),
        Habit::Umbrella => 0.18 + 0.82 * (h.powf(0.75)).min(1.0) * (1.0 - 0.6 * (h - 0.88).max(0.0).powi(2) * 20.0).max(0.4),
        Habit::Banyan => 0.22 + 0.78 * dome(0.50, 1.35),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Habit; 12] = [
        Habit::Rounded, Habit::Open, Habit::Oval, Habit::Fan, Habit::Vase, Habit::Conical, Habit::Layered,
        Habit::Weeping, Habit::Fastigiate, Habit::Irregular, Habit::Umbrella, Habit::Banyan,
    ];

    #[test]
    fn every_profile_is_a_sensible_crown_outline() {
        for habit in ALL {
            let values: Vec<f32> = (0..=20).map(|k| profile(habit, k as f32 / 20.0)).collect();
            assert!(values.iter().all(|v| v.is_finite() && (0.0..=1.05).contains(v)), "{habit:?}: {values:?}");
            let widest = values.iter().copied().fold(0.0, f32::max);
            assert!(widest > 0.85, "{habit:?} never reaches its crown radius ({widest})");
        }
        // A cone is widest at the bottom and narrows to a point; a dome is widest in the middle.
        assert!(profile(Habit::Conical, 0.0) > 0.9 && profile(Habit::Conical, 1.0) < 0.1);
        assert!(profile(Habit::Rounded, 0.45) > profile(Habit::Rounded, 0.05));
        assert!(profile(Habit::Rounded, 0.45) > profile(Habit::Rounded, 0.98));
        // An umbrella is widest at the top, a vase wider at the lip than the foot.
        assert!(profile(Habit::Umbrella, 0.85) > profile(Habit::Umbrella, 0.2));
        assert!(profile(Habit::Vase, 0.95) > profile(Habit::Vase, 0.1));
    }

    #[test]
    fn architecture_numbers_are_in_range_for_every_habit() {
        for habit in ALL {
            let a = architecture(habit);
            assert!((0.0..=1.0).contains(&a.leader));
            assert!(a.limb_down.0 > 0.2 && a.limb_down.0 < 1.7 && a.limb_down.1 > 0.1 && a.limb_down.1 < 1.7);
            assert!(a.limb_per_m > 0.3 && a.branch_per_m > 1.0 && a.twig_per_m > 2.0);
            assert!(a.twig_len.0 < a.twig_len.1);
        }
    }
}
