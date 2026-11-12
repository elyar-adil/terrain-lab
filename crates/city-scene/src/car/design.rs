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
use super::form::{Face, Mirror, Spec, Style};
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
    /// An estate: a saloon whose roof runs to the tail.
    Wagon,
    /// Two long doors under a low roof.
    Coupe,
    /// The city runabout, barely three metres long.
    Micro,
    /// Low, wide and two-seated, its roof falling in one line to the tail.
    Sports,
}

/// A maker: what every car of one make has in common whatever its kind. A
/// make is known by its face before its badge is read: the shape of its
/// grille, the look of its lamps, the signature of its tail.
#[derive(Clone, Copy)]
pub(super) struct Marque {
    grille: Mouth,
    grille_kind: GrilleKind,
    bright: bool,
    head: HeadStyle,
    tail: TailStyle,
    /// Round lamps, where its cars are low enough to carry them.
    round_lamps: bool,
    /// A roof and pillars in black over the body's colour.
    black_roof: bool,
    /// A light bar across the nose.
    head_bar: bool,
}

/// The shape of a make's grille.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mouth {
    /// A band between the lamps.
    Band,
    /// One frame from the bonnet's edge down to the bumper.
    Frame,
    /// Two openings side by side.
    Twin,
    /// None: a smooth nose.
    Closed,
}

impl Marque {
    pub fn new(seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x6d61_7271_7565);
        let (slats, bars) = (rng.count(2, 5), rng.count(7, 13));
        Self {
            grille: rng.pick(&[
                (Mouth::Frame, 0.36),
                (Mouth::Band, 0.24),
                (Mouth::Twin, 0.20),
                (Mouth::Closed, 0.20),
            ]),
            grille_kind: rng.pick(&[
                (GrilleKind::Slats(slats), 0.40),
                (GrilleKind::Mesh, 0.32),
                (GrilleKind::Bars(bars), 0.28),
            ]),
            bright: rng.chance(0.5),
            head: rng.pick(&[
                (HeadStyle::Projector, 0.36),
                (HeadStyle::Matrix, 0.34),
                (HeadStyle::Slim, 0.22),
                (HeadStyle::Halogen, 0.08),
            ]),
            tail: rng.pick(&[
                (TailStyle::Cluster, 0.30),
                (TailStyle::Guide, 0.38),
                (TailStyle::Bar, 0.32),
            ]),
            round_lamps: rng.chance(0.2),
            black_roof: rng.chance(0.3),
            head_bar: rng.chance(0.25),
        }
    }
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
        (Kind::Saloon, 0.28),
        (Kind::Suv, 0.18),
        (Kind::Crossover, 0.13),
        (Kind::Hatchback, 0.10),
        (Kind::Fastback, 0.06),
        (Kind::Mpv, 0.05),
        (Kind::Van, 0.06),
        (Kind::Wagon, 0.04),
        (Kind::Coupe, 0.03),
        (Kind::Micro, 0.06),
        (Kind::Sports, 0.03),
    ])
}

/// The design a seed stands for.
pub(super) fn design(seed: u64, marque: &Marque) -> Spec {
    let mut rng = Rng::new(seed);
    let kind = kind_of(&mut rng);
    design_of(kind, &mut rng, marque)
}

/// A design of a given kind.
pub(super) fn design_of(kind: Kind, rng: &mut Rng, marque: &Marque) -> Spec {
    use Kind::*;
    // An estate, a coupe and a runabout are each drawn from the kind nearest
    // to them and then changed where they differ.
    let shape = kind;
    let kind = match shape {
        Wagon => Saloon,
        Coupe | Sports => Fastback,
        Micro => Hatchback,
        other => other,
    };
    let electric = rng.chance(match kind {
        Saloon => 0.30,
        Fastback => 0.60,
        Hatchback => 0.30,
        Crossover => 0.40,
        Suv => 0.30,
        Mpv => 0.25,
        Van => 0.10,
        Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
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
        Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
    };
    let length = match shape {
        Micro => rng.range(2.95, 3.55),
        Coupe => length - rng.range(0.15, 0.40),
        Sports => rng.range(4.30, 4.62),
        _ => length,
    };
    let width = match kind {
        Saloon => rng.range(1.78, 1.88),
        Fastback => rng.range(1.84, 1.95),
        Hatchback => rng.range(1.68, 1.80),
        Crossover => rng.range(1.78, 1.86),
        Suv => rng.range(1.84, 1.96),
        Mpv => rng.range(1.80, 1.90),
        Van => rng.range(1.56, 1.68),
        Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
    };
    let width = if shape == Micro {
        rng.range(1.50, 1.62)
    } else {
        width
    };
    let roof = match kind {
        Saloon => rng.range(1.42, 1.49),
        Fastback => rng.range(1.38, 1.45),
        Hatchback => rng.range(1.45, 1.53),
        Crossover => rng.range(1.57, 1.65),
        Suv => rng.range(1.66, 1.78),
        Mpv => rng.range(1.70, 1.82),
        Van => rng.range(1.80, 1.92),
        Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
    };
    let roof = match shape {
        Micro => rng.range(1.54, 1.66),
        Coupe => roof - rng.range(0.05, 0.10),
        Sports => rng.range(1.25, 1.33),
        _ => roof,
    };
    let wheel_r = match kind {
        Saloon => rng.range(0.325, 0.350),
        Fastback => rng.range(0.340, 0.365),
        Hatchback => rng.range(0.305, 0.330),
        Crossover => rng.range(0.345, 0.370),
        Suv => rng.range(0.360, 0.395),
        Mpv => rng.range(0.330, 0.355),
        Van => rng.range(0.280, 0.300),
        Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
    };
    let wheel_r = if shape == Micro {
        rng.range(0.262, 0.290)
    } else {
        wheel_r
    };
    // The underside of the body stands a quarter of the way up the wheel: a
    // little more for what is meant to leave the road.
    let sill = wheel_r
        * match kind {
            Suv => rng.range(0.60, 0.68),
            Crossover => rng.range(0.54, 0.60),
            Van => rng.range(0.52, 0.58),
            _ => rng.range(0.47, 0.53),
        };
    // Overhangs: an electric car's wheels sit nearer its corners.
    let front_overhang = match kind {
        Van => rng.range(0.52, 0.66),
        _ => rng.range(0.84, 0.98) - if electric { 0.06 } else { 0.0 },
    };
    let front_overhang = if shape == Micro {
        rng.range(0.50, 0.58)
    } else {
        front_overhang
    };
    let rear_overhang = match kind {
        Saloon | Fastback => rng.range(0.98, 1.12),
        Hatchback => rng.range(0.66, 0.80),
        Crossover | Suv => rng.range(0.86, 1.02),
        Mpv => rng.range(0.92, 1.04),
        Van => rng.range(0.62, 0.76),
        Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
    } - if electric { 0.05 } else { 0.0 };
    let rear_overhang = if shape == Micro {
        rng.range(0.46, 0.54)
    } else {
        rear_overhang
    };
    let wheelbase = length - front_overhang - rear_overhang;
    // The nose and the tail are modelled as faces standing beyond the lofted
    // body: the loft stops this far short of each end.
    // (Never so far that the loft would end inside a wheel arch.)
    let room = wheel_r + 0.20;
    let face = if kind == Van || shape == Micro {
        (
            0.15_f32.min(front_overhang - room),
            0.13_f32.min(rear_overhang - room),
        )
    } else {
        (
            0.30_f32.min(front_overhang - room),
            0.22_f32.min(rear_overhang - room),
        )
    };
    let nose = wheelbase * 0.5 + front_overhang - face.0;
    let tail = -wheelbase * 0.5 - rear_overhang + face.1;
    // Heights of the shoulder line: nose, cowl, rear deck.
    let raise = (roof - 1.45) * 0.50;
    let cowl = match kind {
        Van => rng.range(1.00, 1.05),
        _ => rng.range(0.955, 1.0) + raise,
    };
    let nose_top = match kind {
        Van => cowl - rng.range(0.20, 0.26),
        Suv | Crossover | Mpv => cowl - rng.range(0.19, 0.25),
        _ => cowl - rng.range(0.19, 0.25),
    };
    let deck = match kind {
        Saloon => cowl + rng.range(0.00, 0.04),
        Fastback => cowl + rng.range(0.02, 0.06),
        _ => cowl + rng.range(0.01, 0.04),
    };
    let tail_top = match kind {
        Saloon | Fastback => deck - rng.range(0.05, 0.08),
        _ => deck - rng.range(0.03, 0.06),
    };
    // The glasshouse.
    let half_base = wheelbase * 0.5;
    let front_foot = match kind {
        Van => nose - rng.range(0.46, 0.56),
        Mpv => half_base - rng.range(-0.10, 0.06),
        _ => half_base - rng.range(0.16, 0.30) + if electric { 0.06 } else { 0.0 },
    };
    let front_foot = if shape == Micro {
        nose - rng.range(0.36, 0.48)
    } else {
        front_foot
    };
    let front_head = front_foot
        - match kind {
            Van => rng.range(0.52, 0.62),
            Mpv => rng.range(0.86, 0.98),
            Fastback => rng.range(1.00, 1.12),
            Suv => rng.range(0.80, 0.92),
            _ => rng.range(0.90, 1.04),
        };
    let rear_foot = match kind {
        Saloon => tail + rng.range(0.38, 0.60),
        Fastback => tail + rng.range(0.34, 0.48),
        Hatchback => tail + rng.range(0.07, 0.12),
        Crossover => tail + rng.range(0.08, 0.14),
        Suv | Mpv => tail + rng.range(0.07, 0.11),
        Van => tail + rng.range(0.04, 0.06),
        Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
    };
    let rear_head = rear_foot
        + match kind {
            Saloon => rng.range(0.78, 1.00),
            Fastback => rng.range(0.98, 1.20),
            Hatchback => rng.range(0.36, 0.50),
            Crossover => rng.range(0.40, 0.62),
            Suv => rng.range(0.26, 0.40),
            Mpv => rng.range(0.22, 0.32),
            Van => rng.range(0.07, 0.10),
            Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
        };
    // An estate's roof runs to an upright tailgate; a runabout's cabin is
    // most of the car.
    let (front_head, rear_foot, rear_head) = match shape {
        Wagon => {
            let foot = tail + rng.range(0.07, 0.12);
            (front_head, foot, foot + rng.range(0.32, 0.58))
        }
        Sports => {
            // The roof falls in one line from over the seats to the tail.
            let foot = tail + rng.range(0.10, 0.20);
            (front_head, foot, foot + rng.range(1.10, 1.35))
        }
        Micro => {
            let foot = tail + rng.range(0.05, 0.09);
            (
                front_foot - rng.range(0.58, 0.72),
                foot,
                foot + rng.range(0.16, 0.30),
            )
        }
        _ => (front_head, rear_foot, rear_head),
    };
    // Whatever the rakes, a roof is left between the screens.
    let rear_head = rear_head.min(
        front_head
            - match shape {
                Micro => 0.75,
                Sports => 0.55,
                _ => 1.25,
            },
    );
    let rear_foot = rear_foot.min(rear_head - 0.06);
    let top = vec![
        (tail, tail_top - 0.03),
        (tail + 0.24, tail_top),
        (rear_foot.max(tail + 0.45) + 0.12, deck),
        (front_foot - 0.08, cowl),
        (
            nose - (nose - front_foot) * 0.42,
            cowl - (cowl - nose_top) * 0.36,
        ),
        (nose - 0.20, nose_top + 0.035),
        (nose, nose_top),
    ];
    let boot = matches!(kind, Saloon | Fastback) && !matches!(shape, Wagon | Sports);
    let tall = matches!(kind, Suv | Crossover);
    let cladding = match kind {
        Suv => rng.chance(0.65),
        Crossover => rng.chance(0.8),
        _ => false,
    };

    // --- lamps -------------------------------------------------------------
    // A make's lamps are its own; a van's are what is cheapest.
    let head_style = if kind == Van {
        HeadStyle::Halogen
    } else if electric && marque.head == HeadStyle::Halogen {
        HeadStyle::Slim
    } else {
        marque.head
    };
    let round = marque.round_lamps
        && matches!(shape, Sports | Coupe | Micro | Hatchback | Fastback)
        || shape == Sports && rng.chance(0.5);
    let head = match head_style {
        _ if round => {
            // An oval lamp set into the wing, as a sports car carries them.
            let h = rng.range(0.13, 0.17);
            let y = -h * 0.5 - rng.range(0.0, 0.02);
            Wing {
                s: (rng.range(0.56, 0.66), rng.range(0.0, 0.05)),
                y: (y, y + 0.01),
                h: (h, h * 0.92),
                r: h * 0.46,
            }
        }
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
    let head_bar = (!round
        && (marque.head_bar && head_style != HeadStyle::Halogen || electric && rng.chance(0.25)))
    .then(|| (head.y.0, rng.range(0.016, 0.024)));
    let tail_style = if kind == Van || tall && rng.chance(0.18) || kind == Mpv && rng.chance(0.4) {
        TailStyle::Stack
    } else if shape == Sports {
        TailStyle::Bar
    } else {
        marque.tail
    };
    let face_h = tail_top - sill;
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
    let mouth = if electric && rng.chance(0.8) || shape == Sports {
        Mouth::Closed
    } else {
        marque.grille
    };
    let grille = if mouth == Mouth::Closed {
        None
    } else {
        let kind_of = marque.grille_kind;
        let deep = mouth != Mouth::Band;
        // Its top edge is the lamps' top edge: one line across the face.
        let top = head.y.0 + head.h.0 * 0.5 - rng.range(0.0, 0.012);
        let bottom = if deep {
            lamp_low - rng.range(0.12, 0.24)
        } else {
            (lamp_low + rng.range(-0.01, 0.02)).min(top - 0.05)
        };
        let half = (head.s.0 - rng.range(0.025, 0.04)).max(0.3);
        Some(Grille {
            half: if deep {
                (half, half * rng.range(1.05, 1.30))
            } else {
                (half, half * rng.range(0.72, 1.0))
            },
            y: (bottom, top),
            r: rng.range(0.012, 0.035),
            kind: kind_of,
            bright: marque.bright,
            twin: mouth == Mouth::Twin,
        })
    };
    let intake = {
        let low = rng.range(0.045, 0.06);
        let half = rng.range(0.44, 0.62);
        let slats = rng.count(1, 3);
        Grille {
            half: (half * rng.range(0.85, 1.15), half),
            y: (low, low + rng.range(0.055, 0.085)),
            r: rng.range(0.015, 0.03),
            kind: rng.pick(&[(GrilleKind::Slats(slats), 0.6), (GrilleKind::Mesh, 0.4)]),
            bright: false,
            twin: false,
        }
    };

    // --- wheels ------------------------------------------------------------
    // The rim fills more of a sporting car's wheel, less of a van's.
    let rim_share = match kind {
        Van => rng.range(0.60, 0.63),
        Hatchback => rng.range(0.63, 0.67),
        Fastback => rng.range(0.72, 0.76),
        _ => rng.range(0.68, 0.74),
    };
    let tyre = Tyre {
        width: (match kind {
            Van => 0.175,
            Hatchback => 0.195,
            Saloon | Mpv => 0.215,
            Crossover => 0.225,
            Fastback => 0.245,
            Suv => 0.245,
            Wagon | Coupe | Micro | Sports => unreachable!("drawn from another kind"),
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
    // --- the hand that drew it ---------------------------------------------
    // Sharp-edged or soft, slab-sided or hipped, wedge or level: the same
    // freedoms whatever the kind, with a few leanings.
    let soft = rng.unit();
    let sporty = matches!(shape, Fastback | Coupe | Sports) || rng.chance(0.25);
    let boxy = matches!(shape, Van | Micro | Suv | Mpv);
    let face_of = |rng: &mut Rng, front: bool| Face {
        sweep: rng.range(0.30, 0.72),
        chin: rng.range(0.22, 0.50),
        lean: if front {
            rng.range(0.20, 0.62)
        } else {
            rng.range(0.08, 0.40)
        } * if boxy { 0.6 } else { 1.0 },
        proud: rng.range(0.02, 0.11),
        square: 1.55 + (1.0 - soft) * rng.range(0.2, 1.3),
    };
    let style = Style {
        tumble: if boxy {
            rng.range(0.20, 0.28)
        } else {
            rng.range(0.26, 0.37)
        },
        shoulder: rng.range(0.030, 0.075),
        hip: if sporty {
            rng.range(0.035, 0.058)
        } else {
            rng.range(0.012, 0.042)
        },
        waist: rng.range(0.0, 0.045),
        crown: if boxy {
            rng.range(0.03, 0.06)
        } else {
            rng.range(0.05, 0.11)
        },
        peak: rng.range(-0.15, 0.30),
        bow: rng.range(1.25, 2.3),
        taper: (rng.range(0.05, 0.16), rng.range(0.05, 0.17)),
        crease_drop: rng.range(0.09, 0.24),
        wedge: rng.range(-0.03, 0.07),
        crease: (1.0 - soft) * rng.range(0.006, 0.016) + 0.002,
        hollow: rng.range(0.0, 0.014),
        bonnet: rng.range(0.0, 0.010),
        nose: face_of(rng, true),
        tail: face_of(rng, false),
    };
    let four_doors = match shape {
        Coupe | Sports => false,
        Micro => rng.chance(0.4),
        _ => true,
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
        roof_rails: (matches!(kind, Suv | Crossover | Mpv) || shape == Wagon) && rng.chance(0.6),
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
        face,
        four_doors,
        style,
        black_roof: marque.black_roof && !matches!(shape, Van | Micro) || tall && rng.chance(0.2),
        fog: kind != Van && rng.chance(0.6),
    }
}
