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
            black_roof: rng.chance(0.14),
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

/// How big a car is on the road.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Size {
    Micro,
    Small,
    Medium,
    Large,
}

/// How high it stands and sits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Stance {
    /// A sports car's: the roof at a standing man's ribs.
    Low,
    Car,
    /// A crossover's: a car lifted a hand.
    Raised,
    /// An off-roader's, a people carrier's, a van's.
    Tall,
}

/// What is ahead of the screen.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Hood {
    /// A long bonnet, the cabin set back behind the front wheels.
    Long,
    /// A short one, the screen's foot over the front wheels.
    Short,
    /// None to speak of: one box, the screen running down to the nose.
    None,
}

/// How the roof ends.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Roof {
    /// A step down to a boot: three boxes.
    Notch,
    /// One line falling to the tail.
    Fast,
    /// Cut off short over a tailgate.
    Hatch,
    /// Carried level to an upright tail: an estate, an off-roader, a van.
    Square,
}

/// The doors down one side.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Doors {
    Two,
    Four,
    /// The rear one hinged at its back edge.
    Coach,
    /// The rear one sliding back along a rail.
    Sliding,
}

/// The architecture of a car: the decisions taken before a line of its body
/// is drawn, from which its proportions follow. Each is a choice of its own.
/// A kind of car is a habit of choosing them (a saloon is usually a long
/// bonnet, a notched roof and four doors) and no more than that: a saloon may
/// be given a fast roof, an off-roader a falling one, a people carrier no
/// bonnet at all.
#[derive(Clone, Copy, Debug)]
pub(super) struct Architecture {
    pub size: Size,
    pub stance: Stance,
    pub hood: Hood,
    pub roof: Roof,
    pub doors: Doors,
    /// No pillar between the doors' glass.
    pub pillarless: bool,
    /// The pillars blacked out, so the glass reads as one band round the car.
    pub wrapped: bool,
}

impl Architecture {
    fn of(kind: Kind, rng: &mut Rng) -> Self {
        use Kind::*;
        let (size, stance, hood, roof, doors) = match kind {
            Saloon => (
                rng.pick(&[(Size::Medium, 0.6), (Size::Large, 0.4)]),
                Stance::Car,
                Hood::Long,
                rng.pick(&[(Roof::Notch, 0.7), (Roof::Fast, 0.3)]),
                rng.pick(&[(Doors::Four, 0.96), (Doors::Coach, 0.04)]),
            ),
            Fastback => (
                Size::Large,
                Stance::Car,
                Hood::Long,
                Roof::Fast,
                Doors::Four,
            ),
            Hatchback => (
                Size::Small,
                Stance::Car,
                rng.pick(&[(Hood::Long, 0.5), (Hood::Short, 0.5)]),
                Roof::Hatch,
                rng.pick(&[(Doors::Four, 0.85), (Doors::Two, 0.15)]),
            ),
            Crossover => (
                Size::Medium,
                Stance::Raised,
                rng.pick(&[(Hood::Long, 0.6), (Hood::Short, 0.4)]),
                rng.pick(&[(Roof::Hatch, 0.6), (Roof::Fast, 0.25), (Roof::Square, 0.15)]),
                Doors::Four,
            ),
            Suv => (
                Size::Large,
                Stance::Tall,
                Hood::Long,
                rng.pick(&[(Roof::Square, 0.7), (Roof::Hatch, 0.18), (Roof::Fast, 0.12)]),
                Doors::Four,
            ),
            Mpv => (
                Size::Large,
                Stance::Tall,
                rng.pick(&[(Hood::Short, 0.55), (Hood::None, 0.45)]),
                Roof::Square,
                rng.pick(&[(Doors::Sliding, 0.75), (Doors::Four, 0.25)]),
            ),
            Van => (
                Size::Small,
                Stance::Tall,
                rng.pick(&[(Hood::None, 0.6), (Hood::Short, 0.4)]),
                Roof::Square,
                Doors::Sliding,
            ),
            Wagon => (
                rng.pick(&[(Size::Medium, 0.5), (Size::Large, 0.5)]),
                Stance::Car,
                Hood::Long,
                Roof::Square,
                Doors::Four,
            ),
            Coupe => (
                Size::Medium,
                Stance::Low,
                Hood::Long,
                rng.pick(&[(Roof::Fast, 0.65), (Roof::Notch, 0.35)]),
                Doors::Two,
            ),
            Micro => (
                Size::Micro,
                Stance::Raised,
                rng.pick(&[(Hood::None, 0.5), (Hood::Short, 0.5)]),
                rng.pick(&[(Roof::Square, 0.5), (Roof::Hatch, 0.5)]),
                rng.pick(&[(Doors::Two, 0.6), (Doors::Four, 0.4)]),
            ),
            Sports => (
                Size::Medium,
                Stance::Low,
                // Engine ahead of the cabin, or behind it.
                rng.pick(&[(Hood::Long, 0.6), (Hood::Short, 0.4)]),
                Roof::Fast,
                Doors::Two,
            ),
        };
        Self {
            size,
            stance,
            hood,
            roof,
            doors,
            pillarless: matches!(doors, Doors::Two | Doors::Coach) && rng.chance(0.6)
                || rng.chance(0.06),
            wrapped: rng.chance(match stance {
                Stance::Tall | Stance::Raised => 0.35,
                _ => 0.15,
            }),
        }
    }
}

/// A design of a given kind.
pub(super) fn design_of(kind: Kind, rng: &mut Rng, marque: &Marque) -> Spec {
    use Kind::*;
    let arch = Architecture::of(kind, rng);
    // What is left of the kind once the architecture is drawn is its trade:
    // a van is fitted out as cheaply as a van is, whatever its shape.
    let shape = kind;
    let kind = match shape {
        Wagon => Saloon,
        Coupe | Sports => Fastback,
        Micro => Hatchback,
        other => other,
    };
    let electric = rng.chance(match (shape, arch.hood) {
        (Micro, _) => 0.85,
        (Van, _) => 0.10,
        // With no engine to house, a bonnet may be as short as it likes.
        (_, Hood::None) => 0.6,
        (Fastback | Sports, _) => 0.5,
        _ => 0.32,
    });

    // --- proportions, from the architecture ---------------------------------
    let length = match arch.size {
        Size::Micro => rng.range(2.95, 3.55),
        Size::Small => rng.range(3.85, 4.35),
        Size::Medium => rng.range(4.35, 4.75),
        Size::Large => rng.range(4.72, 5.05),
    };
    let narrow = shape == Van;
    let width = match arch.size {
        Size::Micro => rng.range(1.50, 1.62),
        Size::Small if narrow => rng.range(1.56, 1.68),
        Size::Small => rng.range(1.70, 1.80),
        Size::Medium => rng.range(1.78, 1.88),
        Size::Large => rng.range(1.85, 1.96),
    } + if arch.stance == Stance::Low {
        0.05
    } else {
        0.0
    };
    let roof = match arch.stance {
        Stance::Low => rng.range(1.25, 1.38),
        Stance::Car => rng.range(1.42, 1.50),
        Stance::Raised => rng.range(1.55, 1.66),
        Stance::Tall if shape == Van => rng.range(1.80, 1.92),
        Stance::Tall => rng.range(1.68, 1.82),
    };
    let wheel_r = match arch.size {
        Size::Micro => 0.275,
        Size::Small => 0.315,
        Size::Medium => 0.340,
        Size::Large => 0.352,
    } + match arch.stance {
        Stance::Low => 0.008,
        Stance::Car => 0.0,
        Stance::Raised => 0.012,
        Stance::Tall if shape == Van => -0.028,
        Stance::Tall if shape == Mpv => -0.006,
        Stance::Tall => 0.026,
    } + rng.range(-0.012, 0.012);
    // The underside of the body stands a quarter of the way up the wheel: a
    // little more for what is meant to leave the road.
    let sill = wheel_r
        * match arch.stance {
            Stance::Low => rng.range(0.42, 0.48),
            Stance::Car => rng.range(0.47, 0.53),
            Stance::Raised => rng.range(0.54, 0.60),
            Stance::Tall if shape == Suv => rng.range(0.60, 0.68),
            Stance::Tall => rng.range(0.50, 0.57),
        };
    // Overhangs. With no bonnet the front wheels are at the corner; an
    // electric car's sit nearer its corners anyway.
    let small = arch.size == Size::Micro;
    let front_overhang = match arch.hood {
        _ if small => rng.range(0.50, 0.58),
        Hood::Long => rng.range(0.84, 0.98),
        Hood::Short => rng.range(0.76, 0.90),
        Hood::None => rng.range(0.56, 0.72),
    } - if electric && !small { 0.05 } else { 0.0 };
    let rear_overhang = match arch.roof {
        _ if small => rng.range(0.46, 0.54),
        Roof::Notch => rng.range(0.98, 1.12),
        Roof::Fast => rng.range(0.90, 1.06),
        Roof::Hatch => rng.range(0.66, 0.86),
        Roof::Square if shape == Van => rng.range(0.62, 0.76),
        Roof::Square => rng.range(0.84, 1.04),
    } - if electric && !small { 0.04 } else { 0.0 };
    let wheelbase = length - front_overhang - rear_overhang;
    // The nose and the tail are modelled as faces standing beyond the lofted
    // body: the loft stops this far short of each end (never so far that it
    // would end inside a wheel arch).
    let room = wheel_r + 0.20;
    let blunt = small || arch.hood == Hood::None;
    let face = (
        (if blunt { 0.16_f32 } else { 0.30 }).min(front_overhang - room),
        (if small || shape == Van {
            0.13_f32
        } else {
            0.22
        })
        .min(rear_overhang - room),
    );
    let nose = wheelbase * 0.5 + front_overhang - face.0;
    let tail = -wheelbase * 0.5 - rear_overhang + face.1;
    // Heights of the shoulder line: cowl, nose, rear deck. A taller car's
    // shoulder is higher; a low one's lower.
    let cowl = rng.range(0.955, 1.0) + (roof - 1.45) * 0.50;
    let nose_top = cowl
        - match arch.hood {
            Hood::Long => rng.range(0.19, 0.26),
            Hood::Short => rng.range(0.17, 0.23),
            // One box: the nose is hardly below the screen's foot.
            Hood::None => rng.range(0.10, 0.17),
        };
    let deck = cowl
        + match arch.roof {
            Roof::Notch => rng.range(0.00, 0.04),
            Roof::Fast => rng.range(0.02, 0.07),
            _ => rng.range(0.01, 0.04),
        };
    let tail_top = deck
        - match arch.roof {
            Roof::Notch | Roof::Fast => rng.range(0.05, 0.08),
            _ => rng.range(0.03, 0.06),
        };
    // The glasshouse: where the screen stands follows from the bonnet.
    let half_base = wheelbase * 0.5;
    let front_foot = match arch.hood {
        Hood::Long => half_base - rng.range(0.14, 0.32) + if electric { 0.06 } else { 0.0 },
        Hood::Short => half_base + rng.range(-0.04, 0.14),
        Hood::None => nose - rng.range(0.30, 0.46),
    }
    .min(nose - 0.30);
    let front_head = front_foot
        - match (arch.stance, arch.hood) {
            (_, Hood::None) if shape == Van => rng.range(0.50, 0.62),
            (_, Hood::None) if small => rng.range(0.58, 0.72),
            // One box: the screen is long and laid back, running out of the nose.
            (_, Hood::None) => rng.range(0.95, 1.15),
            (Stance::Low, _) => rng.range(1.00, 1.15),
            (Stance::Car, _) => rng.range(0.90, 1.04),
            (Stance::Raised, _) => rng.range(0.84, 0.96),
            (Stance::Tall, _) => rng.range(0.76, 0.90),
        } * if small { 0.72 } else { 1.0 };
    // And where the roof ends follows from the tail.
    let rear_foot = tail
        + match arch.roof {
            Roof::Notch => rng.range(0.38, 0.60),
            Roof::Fast => rng.range(0.10, 0.34),
            Roof::Hatch => rng.range(0.07, 0.13),
            Roof::Square => rng.range(0.04, 0.10),
        };
    let rear_head = rear_foot
        + match arch.roof {
            Roof::Notch => rng.range(0.78, 1.00),
            Roof::Fast if arch.stance == Stance::Low => rng.range(1.05, 1.35),
            Roof::Fast => rng.range(0.95, 1.25),
            Roof::Hatch => rng.range(0.36, 0.62),
            Roof::Square if shape == Van => rng.range(0.07, 0.10),
            Roof::Square if arch.stance == Stance::Car => rng.range(0.32, 0.56),
            Roof::Square => rng.range(0.18, 0.38),
        } * if small { 0.6 } else { 1.0 };
    // Whatever the rakes, a roof is left between the screens.
    let rear_head = rear_head.min(
        front_head
            - match (arch.size, arch.stance) {
                (Size::Micro, _) => 0.75,
                (_, Stance::Low) => 0.55,
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
    // A boot lid where there is a deck behind the glass to lift.
    let boot = arch.roof == Roof::Notch || arch.roof == Roof::Fast && rear_foot - tail > 0.26;
    let tall =
        matches!(arch.stance, Stance::Raised | Stance::Tall) && !matches!(shape, Van | Mpv | Micro);
    let cladding = tall
        && rng.chance(if arch.stance == Stance::Tall {
            0.65
        } else {
            0.8
        });

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
    let sporty = arch.stance == Stance::Low || arch.roof == Roof::Fast || rng.chance(0.25);
    let boxy = arch.roof == Roof::Square && arch.stance != Stance::Car || arch.size == Size::Micro;
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
        deck_crown: rng.range(0.035, 0.075),
        barrel: rng.range(0.020, 0.055),
        flare: if sporty {
            rng.range(0.018, 0.034)
        } else {
            rng.range(0.008, 0.024)
        },
        arch_square: if boxy && rng.chance(0.6) {
            rng.range(2.8, 4.2)
        } else {
            2.0
        },
        nose: face_of(rng, true),
        tail: face_of(rng, false),
    };
    let four_doors = arch.doors != Doors::Two;
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
        coach: arch.doors == Doors::Coach,
        sliding: arch.doors == Doors::Sliding,
        pillarless: arch.pillarless,
        wrapped: arch.wrapped,
        style,
        black_roof: marque.black_roof && !matches!(shape, Van | Micro) || tall && rng.chance(0.1),
        fog: kind != Van && rng.chance(0.6),
    }
}
