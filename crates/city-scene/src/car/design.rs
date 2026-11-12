//! Designs without end: every car from one number.
//!
//! A street's cars are not six models repeated. They are a few *kinds* of car
//! (saloons most of all on a Chinese street, then SUVs, hatchbacks, the small
//! van that does every delivery) and within each kind every maker's own
//! proportions, lamps, grille, wheels and mirrors. So a design is drawn here
//! the way a studio arrives at one: the kind fixes what is sensible (how long
//! a bonnet, how tall a roof, how much tyre), and inside those bounds every
//! dimension and every part is chosen freely. Any seed gives a car that could
//! be parked outside; no two seeds give the same one.
use super::form::{Mirror, Spec};
use super::lamps::{Grille, GrilleKind, HeadStyle, TailStyle, Wing};
use super::wheels::{Rim, RimStyle, Tread, Tyre};

/// The kinds of car, which bound everything else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Kind {
    Saloon,
    Fastback,
    Hatchback,
    Crossover,
    Suv,
    Mpv,
    Van,
}

/// A small deterministic stream of numbers (splitmix64).
pub(super) struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed ^ 0x9e37_79b9_7f4a_7c15)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    /// In `0..1`.
    pub fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
    pub fn range(&mut self, low: f32, high: f32) -> f32 {
        low + (high - low) * self.unit()
    }
    pub fn chance(&mut self, p: f32) -> bool {
        self.unit() < p
    }
    /// One of `options`, by weight.
    pub fn pick<T: Copy>(&mut self, options: &[(T, f32)]) -> T {
        let total: f32 = options.iter().map(|o| o.1).sum();
        let mut at = self.unit() * total;
        for (option, weight) in options {
            if at < *weight {
                return *option;
            }
            at -= weight;
        }
        options[options.len() - 1].0
    }
    pub fn count(&mut self, low: usize, high: usize) -> usize {
        low + (self.next() % (high - low + 1) as u64) as usize
    }
}

/// The kind a seed draws, in the proportions of the street.
fn kind_of(rng: &mut Rng) -> Kind {
    rng.pick(&[
        (Kind::Saloon, 0.34),
        (Kind::Suv, 0.20),
        (Kind::Crossover, 0.14),
        (Kind::Hatchback, 0.11),
        (Kind::Fastback, 0.07),
        (Kind::Mpv, 0.06),
        (Kind::Van, 0.08),
    ])
}

/// The design a seed stands for.
pub(super) fn design(seed: u64) -> Spec {
    let mut rng = Rng::new(seed);
    let kind = kind_of(&mut rng);
    design_of(kind, &mut rng)
}

/// A design of a given kind.
pub(super) fn design_of(kind: Kind, rng: &mut Rng) -> Spec {
    use Kind::*;
    let electric = rng.chance(match kind {
        Saloon => 0.30,
        Fastback => 0.60,
        Hatchback => 0.30,
        Crossover => 0.40,
        Suv => 0.30,
        Mpv => 0.25,
        Van => 0.10,
    });
    // --- proportions -------------------------------------------------------
    let length = match kind {
        Saloon => rng.range(4.50, 4.98),
        Fastback => rng.range(4.60, 5.00),
        Hatchback => rng.range(3.85, 4.35),
        Crossover => rng.range(4.25, 4.60),
        Suv => rng.range(4.55, 4.98),
        Mpv => rng.range(4.60, 5.05),
        Van => rng.range(3.85, 4.40),
    };
    let width = match kind {
        Saloon => rng.range(1.78, 1.88),
        Fastback => rng.range(1.84, 1.95),
        Hatchback => rng.range(1.68, 1.80),
        Crossover => rng.range(1.78, 1.86),
        Suv => rng.range(1.84, 1.96),
        Mpv => rng.range(1.80, 1.90),
        Van => rng.range(1.56, 1.68),
    };
    let roof = match kind {
        Saloon => rng.range(1.42, 1.49),
        Fastback => rng.range(1.38, 1.45),
        Hatchback => rng.range(1.45, 1.53),
        Crossover => rng.range(1.57, 1.65),
        Suv => rng.range(1.66, 1.78),
        Mpv => rng.range(1.70, 1.82),
        Van => rng.range(1.80, 1.92),
    };
    let sill = match kind {
        Saloon | Hatchback => rng.range(0.27, 0.31),
        Fastback => rng.range(0.25, 0.29),
        Crossover => rng.range(0.32, 0.36),
        Suv => rng.range(0.34, 0.40),
        Mpv => rng.range(0.29, 0.33),
        Van => rng.range(0.31, 0.35),
    };
    let wheel_r = match kind {
        Saloon => rng.range(0.305, 0.335),
        Fastback => rng.range(0.325, 0.350),
        Hatchback => rng.range(0.290, 0.315),
        Crossover => rng.range(0.325, 0.350),
        Suv => rng.range(0.345, 0.380),
        Mpv => rng.range(0.320, 0.345),
        Van => rng.range(0.280, 0.300),
    };
    // The faces that close the ends stand beyond the profile.
    let length = length - 0.30;
    let (tail, nose) = (-length * 0.5, length * 0.5);
    // Overhangs: an electric car's wheels sit nearer its corners.
    let front_overhang = match kind {
        Van => rng.range(0.52, 0.66),
        _ => rng.range(0.84, 0.98) - if electric { 0.06 } else { 0.0 },
    };
    let rear_overhang = match kind {
        Saloon | Fastback => rng.range(0.98, 1.12),
        Hatchback => rng.range(0.66, 0.80),
        Crossover | Suv => rng.range(0.86, 1.02),
        Mpv => rng.range(0.92, 1.04),
        Van => rng.range(0.62, 0.76),
    } - if electric { 0.05 } else { 0.0 };
    let wheelbase = length - front_overhang - rear_overhang;
    // The body is placed so the axles are either side of the origin.
    let shift = (front_overhang - rear_overhang) * 0.5;
    let (tail, nose) = (tail + shift, nose + shift);
    // Heights of the shoulder line: nose, cowl, rear deck.
    let raise = sill - 0.29 + (roof - 1.45) * 0.42;
    let cowl = match kind {
        Van => rng.range(1.00, 1.05),
        _ => rng.range(0.955, 1.0) + raise,
    };
    let nose_top = match kind {
        Van => cowl - rng.range(0.20, 0.26),
        Suv | Crossover | Mpv => cowl - rng.range(0.19, 0.25),
        _ => cowl - rng.range(0.20, 0.25),
    };
    let nose_low = nose_top - rng.range(0.13, 0.17);
    let deck = match kind {
        Saloon => cowl + rng.range(0.00, 0.04),
        Fastback => cowl + rng.range(0.02, 0.06),
        _ => cowl + rng.range(0.01, 0.04),
    };
    let tail_top = match kind {
        Saloon | Fastback => deck - rng.range(0.05, 0.08),
        _ => deck - rng.range(0.03, 0.06),
    };
    let tail_low = tail_top - rng.range(0.30, 0.36);
    // The glasshouse.
    let half_base = wheelbase * 0.5;
    let front_foot = match kind {
        Van => nose - rng.range(0.46, 0.56),
        Mpv => half_base - rng.range(-0.10, 0.06),
        _ => half_base - rng.range(0.16, 0.30) + if electric { 0.06 } else { 0.0 },
    };
    let front_head = front_foot
        - match kind {
            Van => rng.range(0.52, 0.62),
            Mpv => rng.range(0.86, 0.98),
            Fastback => rng.range(0.90, 1.00),
            Suv => rng.range(0.74, 0.84),
            _ => rng.range(0.80, 0.92),
        };
    let rear_foot = match kind {
        Saloon => tail + rng.range(0.56, 0.78),
        Fastback => tail + rng.range(0.34, 0.48),
        Hatchback => tail + rng.range(0.07, 0.12),
        Crossover => tail + rng.range(0.08, 0.14),
        Suv | Mpv => tail + rng.range(0.07, 0.11),
        Van => tail + rng.range(0.04, 0.06),
    };
    let rear_head = rear_foot
        + match kind {
            Saloon => rng.range(0.62, 0.80),
            Fastback => rng.range(0.98, 1.20),
            Hatchback => rng.range(0.36, 0.50),
            Crossover => rng.range(0.40, 0.62),
            Suv => rng.range(0.26, 0.40),
            Mpv => rng.range(0.22, 0.32),
            Van => rng.range(0.07, 0.10),
        };
    // Whatever the rakes, a roof is left between the screens.
    let rear_head = rear_head.min(front_head - 1.25);
    let rear_foot = rear_foot.min(rear_head - 0.06);
    let top = vec![
        (tail, tail_low),
        (tail + rng.range(0.035, 0.06), tail_top),
        (rear_foot.max(tail + 0.5) + 0.12, deck),
        (front_foot - 0.08, cowl),
        (
            nose - (nose - front_foot) * 0.36,
            cowl - (cowl - nose_top) * 0.38,
        ),
        (nose - rng.range(0.07, 0.10), nose_top),
        (nose, nose_low),
    ];
    let boot = matches!(kind, Saloon | Fastback);
    let tall = matches!(kind, Suv | Crossover);
    let cladding = match kind {
        Suv => rng.chance(0.65),
        Crossover => rng.chance(0.8),
        _ => false,
    };

    // --- lamps -------------------------------------------------------------
    let head_style = if kind == Van {
        HeadStyle::Halogen
    } else if electric {
        rng.pick(&[
            (HeadStyle::Slim, 0.5),
            (HeadStyle::Matrix, 0.35),
            (HeadStyle::Projector, 0.15),
        ])
    } else {
        rng.pick(&[
            (HeadStyle::Projector, 0.38),
            (HeadStyle::Matrix, 0.30),
            (
                HeadStyle::Halogen,
                if kind == Hatchback { 0.35 } else { 0.14 },
            ),
            (HeadStyle::Slim, 0.12),
        ])
    };
    let head = match head_style {
        HeadStyle::Slim => {
            let h = rng.range(0.045, 0.062);
            Wing {
                s: (rng.range(0.50, 0.60), rng.range(0.22, 0.34)),
                y: (-h * 0.5 - rng.range(0.012, 0.03), -rng.range(0.0, 0.02)),
                h: (h, h * rng.range(0.6, 0.9)),
                r: h * 0.28,
            }
        }
        HeadStyle::Halogen if kind == Van => {
            let h = rng.range(0.12, 0.15);
            Wing {
                s: (rng.range(0.46, 0.54), rng.range(0.12, 0.20)),
                y: (-h * 0.5 - 0.012, -h * 0.5 + rng.range(0.0, 0.03)),
                h: (h, h * rng.range(0.85, 1.0)),
                r: rng.range(0.03, 0.055),
            }
        }
        _ => {
            let h = rng.range(0.085, 0.125);
            Wing {
                s: (rng.range(0.46, 0.56), rng.range(0.24, 0.38)),
                y: (
                    -h * 0.5 - rng.range(0.012, 0.03),
                    -rng.range(0.0, 0.03) - 0.02,
                ),
                h: (h, h * rng.range(0.55, 0.8)),
                r: rng.range(0.014, 0.032),
            }
        }
    };
    let head_bar = (head_style == HeadStyle::Slim && rng.chance(0.6)
        || electric && rng.chance(0.25))
    .then(|| (head.y.0, rng.range(0.016, 0.024)));
    let tail_style = if kind == Van || tall && rng.chance(0.18) || kind == Mpv && rng.chance(0.4) {
        TailStyle::Stack
    } else if electric {
        rng.pick(&[(TailStyle::Bar, 0.6), (TailStyle::Guide, 0.4)])
    } else {
        rng.pick(&[
            (TailStyle::Cluster, 0.40),
            (TailStyle::Guide, 0.38),
            (TailStyle::Bar, 0.22),
        ])
    };
    let face_h = tail_top - tail_low;
    let tail_lamp = match tail_style {
        TailStyle::Stack => {
            // Upright lamps up the corners of a tailgate.
            let h = rng.range(0.26, 0.36);
            let up = if kind == Van {
                0.0
            } else {
                rng.range(0.0, 0.10)
            };
            Wing {
                s: (rng.range(0.80, 0.86), rng.range(0.07, 0.12)),
                y: (-h * 0.5 - 0.03 + up, -h * 0.5 - 0.03 + up),
                h: (h, h),
                r: rng.range(0.02, 0.035),
            }
        }
        TailStyle::Bar => {
            let h = rng.range(0.07, 0.10);
            let y = -rng.range(0.07, 0.12) - h * 0.5;
            Wing {
                s: (rng.range(0.46, 0.58), rng.range(0.20, 0.30)),
                y: (y, y + rng.range(0.0, 0.02)),
                h: (h, h * rng.range(0.6, 0.85)),
                r: rng.range(0.014, 0.02),
            }
        }
        _ => {
            let h = rng.range(0.11, 0.16).min(face_h * 0.5);
            let y = -rng.range(0.05, 0.11) - h * 0.5;
            Wing {
                s: (rng.range(0.40, 0.62), rng.range(0.18, 0.30)),
                y: (y, y + rng.range(0.0, 0.04)),
                h: (h, h * rng.range(0.7, 1.1)),
                r: rng.range(0.018, 0.032),
            }
        }
    };
    let tail_bar = (tail_style == TailStyle::Bar
        || tail_style == TailStyle::Guide && rng.chance(0.3))
    .then(|| {
        (
            tail_lamp.y.0,
            rng.range(0.028, 0.042).min(tail_lamp.h.0 * 0.6),
        )
    });

    // --- grille and intake ---------------------------------------------------
    let lamp_low = head.y.0 - head.h.0 * 0.5;
    let grille = if electric && rng.chance(0.8) {
        None
    } else {
        let (slats, bars) = (rng.count(2, 5), rng.count(7, 13));
        let kind_of = rng.pick(&[
            (GrilleKind::Slats(slats), 0.40),
            (GrilleKind::Mesh, 0.32),
            (GrilleKind::Bars(bars), 0.28),
        ]);
        // Between the lamps, or (a big-mouthed one) dropping below them.
        let deep = rng.chance(if tall { 0.6 } else { 0.35 });
        let top = -rng.range(0.022, 0.04);
        let bottom = if deep {
            lamp_low - rng.range(0.03, 0.10)
        } else {
            (lamp_low + rng.range(-0.01, 0.02)).min(top - 0.05)
        };
        let half = (head.s.0 - rng.range(0.035, 0.06)).max(0.3);
        Some(Grille {
            half: (half, half * rng.range(0.72, 1.0)),
            y: (bottom, top),
            r: rng.range(0.012, 0.035),
            kind: kind_of,
            bright: rng.chance(0.5),
        })
    };
    let intake = {
        let low = rng.range(0.06, 0.085);
        let half = rng.range(0.44, 0.62);
        let slats = rng.count(1, 3);
        Grille {
            half: (half * rng.range(0.85, 1.15), half),
            y: (low, low + rng.range(0.07, 0.11)),
            r: rng.range(0.015, 0.03),
            kind: rng.pick(&[(GrilleKind::Slats(slats), 0.6), (GrilleKind::Mesh, 0.4)]),
            bright: false,
        }
    };

    // --- wheels ------------------------------------------------------------
    // The rim fills more of a sporting car's wheel, less of a van's.
    let rim_share = match kind {
        Van => rng.range(0.60, 0.63),
        Hatchback => rng.range(0.63, 0.67),
        Fastback => rng.range(0.69, 0.73),
        _ => rng.range(0.65, 0.71),
    };
    let tyre = Tyre {
        width: (match kind {
            Van => 0.175,
            Hatchback => 0.195,
            Saloon | Mpv => 0.215,
            Crossover => 0.225,
            Fastback => 0.245,
            Suv => 0.245,
        }) + rng.range(-0.01, 0.01),
        rim_r: wheel_r * rim_share,
        tread: match kind {
            Van => Tread::Ribbed,
            Suv => rng.pick(&[(Tread::Block, 0.6), (Tread::Touring, 0.4)]),
            Fastback => rng.pick(&[(Tread::Directional, 0.6), (Tread::Touring, 0.4)]),
            _ => rng.pick(&[
                (Tread::Touring, 0.5),
                (Tread::Directional, 0.3),
                (Tread::Ribbed, 0.2),
            ]),
        },
    };
    let rim_style = if kind == Van {
        rng.pick(&[(RimStyle::Steel, 0.8), (RimStyle::Spoke, 0.2)])
    } else if electric {
        rng.pick(&[
            (RimStyle::Aero, 0.45),
            (RimStyle::Turbine, 0.2),
            (RimStyle::Split, 0.2),
            (RimStyle::Spoke, 0.15),
        ])
    } else {
        rng.pick(&[
            (RimStyle::Spoke, 0.30),
            (RimStyle::Split, 0.30),
            (RimStyle::Mesh, 0.14),
            (RimStyle::Turbine, 0.16),
            (RimStyle::Steel, if kind == Hatchback { 0.12 } else { 0.03 }),
            (RimStyle::Aero, 0.05),
        ])
    };
    let spokes = match rim_style {
        RimStyle::Spoke => rng.pick(&[(5, 0.4), (6, 0.2), (7, 0.2), (10, 0.2)]),
        RimStyle::Split => rng.pick(&[(5, 0.6), (6, 0.25), (7, 0.15)]),
        RimStyle::Mesh => rng.count(8, 12),
        RimStyle::Turbine => rng.count(8, 12),
        RimStyle::Aero => rng.count(5, 7),
        RimStyle::Steel => rng.count(6, 10),
    };
    let two_tone = rim_style != RimStyle::Steel && rng.chance(0.4);
    let tone = if rim_style == RimStyle::Steel {
        rng.range(0.30, 0.42)
    } else if two_tone {
        rng.range(0.03, 0.10)
    } else {
        let tones = (
            rng.range(0.50, 0.66),
            rng.range(0.14, 0.24),
            rng.range(0.03, 0.06),
        );
        rng.pick(&[(tones.0, 0.6), (tones.1, 0.25), (tones.2, 0.15)])
    };
    let rim = Rim {
        style: rim_style,
        spokes,
        twist: match rim_style {
            RimStyle::Turbine => rng.range(0.30, 0.55),
            RimStyle::Aero => rng.range(0.2, 0.6),
            _ => 0.0,
        },
        two_tone,
        tint: [tone, tone * 1.01, tone * 1.03],
        sport_brakes: matches!(kind, Fastback) && rng.chance(0.7) || rng.chance(0.15),
    };

    // --- mirrors and trim ----------------------------------------------------
    let mirror = if kind == Van {
        Mirror {
            wide: rng.range(0.14, 0.16),
            high: rng.range(0.18, 0.22),
            on_sail: false,
            painted: false,
            repeater: false,
        }
    } else {
        Mirror {
            wide: rng.range(0.18, 0.21) + if tall { 0.015 } else { 0.0 },
            high: rng.range(0.105, 0.125) + if tall { 0.015 } else { 0.0 },
            on_sail: rng.chance(0.4),
            painted: rng.chance(0.85),
            repeater: rng.chance(0.7),
        }
    };
    Spec {
        top,
        cabin: [rear_foot, rear_head, front_head, front_foot],
        roof,
        width,
        sill,
        wheelbase,
        wheel_r,
        rows: if matches!(kind, Mpv | Van) { 3 } else { 2 },
        roof_rails: matches!(kind, Suv | Crossover | Mpv) && rng.chance(0.6),
        boot,
        cladding,
        glass_roof: kind != Van && rng.chance(if electric { 0.7 } else { 0.3 }),
        bright_trim: kind != Van && rng.chance(0.5),
        electric,
        exhausts: if electric {
            0
        } else if matches!(kind, Van | Hatchback) {
            1
        } else {
            rng.count(1, 2)
        },
        head,
        head_style,
        head_bar,
        tail: tail_lamp,
        tail_style,
        tail_bar,
        grille,
        intake,
        tyre,
        rim,
        mirror,
    }
}
