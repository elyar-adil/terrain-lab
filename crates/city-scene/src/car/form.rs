//! What a design is, and the form it is lofted from.
//!
//! A design (`design.rs` draws them without end) is a profile, a glasshouse
//! and a kit of parts: which lamps, which wheels, which mirrors. The form
//! turns the profile into the lines a car is drawn with (shoulder, sill, roof) as functions of the station along it, and
//! into the rings of the one skin everything else is cut from.
use crate::math::Vec3;

use super::lamps::{Grille, HeadStyle, TailStyle, Wing};
use super::wheels::{Rim, Tyre};

pub(super) struct Spec {
    /// Top of the body from the tail to the nose: `(x, height)`.
    pub top: Vec<(f32, f32)>,
    /// The glasshouse: `x` of the rear glass's foot and head, the windscreen's head and foot.
    pub cabin: [f32; 4],
    pub roof: f32,
    pub width: f32,
    pub sill: f32,
    pub wheelbase: f32,
    pub wheel_r: f32,
    /// Rows of seats.
    pub rows: usize,
    pub roof_rails: bool,
    /// A boot lid behind the rear glass (a saloon), or a tailgate.
    pub boot: bool,
    /// Unpainted cladding round the arches and along the sills.
    pub cladding: bool,
    /// A glass roof panel.
    pub glass_roof: bool,
    /// Bright window surrounds, or black ones.
    pub bright_trim: bool,
    /// Battery electric: no grille to speak of, no exhaust, flush handles, a green plate.
    pub electric: bool,
    pub exhausts: usize,
    pub head: Wing,
    pub head_style: HeadStyle,
    /// A light bar joining the headlamps: `(height above the face's top, depth)`.
    pub head_bar: Option<(f32, f32)>,
    pub tail: Wing,
    pub tail_style: TailStyle,
    pub tail_bar: Option<(f32, f32)>,
    pub grille: Option<Grille>,
    pub intake: Grille,
    pub tyre: Tyre,
    pub rim: Rim,
    pub mirror: Mirror,
    /// Fog lamps in the corners of the front bumper.
    pub fog: bool,
    /// How far the nose and the tail stand beyond the lofted body.
    pub face: (f32, f32),
    /// Two doors a side, or one.
    pub four_doors: bool,
    /// The roof and pillars black over the body's colour.
    pub black_roof: bool,
    pub style: Style,
}

/// The hand that drew the body: what makes two cars of one kind and one size
/// different cars. Every number is a freedom of the form, not a part.
#[derive(Clone, Copy)]
pub(super) struct Style {
    /// How far the roof stands in from the body's side: an upright cabin, or
    /// one that leans in.
    pub tumble: f32,
    /// How far the body side leans in at the shoulder.
    pub shoulder: f32,
    /// How far the flank swells over each wheel, and is drawn in between them.
    pub hip: f32,
    pub waist: f32,
    /// How much the roof arches, and where along it the arch peaks (0 over
    /// the middle, positive toward the screen).
    pub crown: f32,
    pub peak: f32,
    /// How the screens run out into the roof: near 1 flat and angular,
    /// higher bowed.
    pub bow: f32,
    /// How much the body draws in toward the nose and the tail, in plan.
    pub taper: (f32, f32),
    /// The crease along the shoulder: how far under the belt, how much it
    /// rises toward the tail, and how sharp. And the hollow in the doors.
    pub crease_drop: f32,
    pub wedge: f32,
    pub crease: f32,
    pub hollow: f32,
    /// The creases down the bonnet.
    pub bonnet: f32,
    /// How far the middle of the bonnet and the boot stands above their edges.
    pub deck_crown: f32,
    /// How far the body's side bows out between the sill and the shoulder.
    pub barrel: f32,
    /// How far the wings stand out round the wheel arches.
    pub flare: f32,
    /// The arches' shape: 2 a circle, higher squared off.
    pub arch_square: f32,
    pub nose: Face,
    pub tail: Face,
}

/// The shape of an end of the car.
#[derive(Clone, Copy)]
pub(super) struct Face {
    /// How far it sweeps back to its corners in plan.
    pub sweep: f32,
    /// How far the chin tucks under, and the panel over the bumper leans away.
    pub chin: f32,
    pub lean: f32,
    /// How proud the bumper stands.
    pub proud: f32,
    /// How round the edge is: near 1.5 soft, above 2.5 square-shouldered.
    pub square: f32,
}

/// A door mirror: its glass's width and height, and how it is carried.
#[derive(Clone, Copy)]
pub(super) struct Mirror {
    pub wide: f32,
    pub high: f32,
    /// On the sail at the front corner of the side glass, or on the door skin.
    pub on_sail: bool,
    /// Painted, or left black.
    pub painted: bool,
    /// A turn repeater in the housing (otherwise it is on the wing).
    pub repeater: bool,
}

/// A line through control points as a designer's sweep would draw it: a cubic
/// B-spline, which is fair (its curvature has no jumps) where a curve made to
/// pass through every point is lumpy. It starts and ends on the first and last
/// points and is drawn toward the ones between.
fn faired(points: &[(f32, f32)], per: usize) -> Vec<(f32, f32)> {
    let mut control = vec![points[0]; 2];
    control.extend_from_slice(points);
    control.extend_from_slice(&[points[points.len() - 1]; 2]);
    let mut out = Vec::new();
    for w in control.windows(4) {
        for step in 0..per {
            let t = step as f32 / per as f32;
            let (t2, t3) = (t * t, t * t * t);
            let b = [
                (1.0 - 3.0 * t + 3.0 * t2 - t3) / 6.0,
                (4.0 - 6.0 * t2 + 3.0 * t3) / 6.0,
                (1.0 + 3.0 * t + 3.0 * t2 - 3.0 * t3) / 6.0,
                t3 / 6.0,
            ];
            out.push((
                (0..4).map(|k| w[k].0 * b[k]).sum(),
                (0..4).map(|k| w[k].1 * b[k]).sum(),
            ));
        }
    }
    out.push(points[points.len() - 1]);
    out
}

pub(super) fn sample_line(line: &[(f32, f32)], x: f32) -> f32 {
    if x <= line[0].0 {
        return line[0].1;
    }
    for w in line.windows(2) {
        if x <= w[1].0 {
            let t = (x - w[0].0) / (w[1].0 - w[0].0).max(1.0e-5);
            return w[0].1 + (w[1].1 - w[0].1) * t;
        }
    }
    line[line.len() - 1].1
}

/// How many quads each stretch of the half-section gets, from the middle of the
/// floor round to the middle of the roof. The count is the same at every
/// station, so a feature of the section (the shoulder, the foot of the glass) is
/// a line of the mesh from one end of the car to the other.
pub(super) const STRETCHES: [usize; 10] = [2, 2, 4, 5, 5, 4, 3, 7, 4, 6];
pub(super) const SIDE_GLASS: usize = 7;
pub(super) const ROOF_RAIL: usize = 8;
pub(super) const ROOF: usize = 9;

/// Where the doors, the bonnet and the boot shut.
pub(super) struct Lines {
    pub front_door: f32,
    pub b_pillar: f32,
    pub rear_door: f32,
    pub boot: f32,
}

pub(super) fn lines_of(spec: &Spec) -> Lines {
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    let arch = spec.wheel_r + ARCH_GAP;
    Lines {
        front_door: (front_foot - 0.10).min(spec.wheelbase * 0.5 - arch - 0.07),
        // A two-door car's doors are long ones.
        b_pillar: (rear_head + front_head) * 0.5 + if spec.four_doors { 0.12 } else { -0.12 },
        rear_door: (rear_head + 0.10).max(-spec.wheelbase * 0.5 + spec.wheel_r * 0.4),
        boot: rear_foot - 0.07,
    }
}

/// The gap between a tyre and its arch.
pub(super) const ARCH_GAP: f32 = 0.042;
/// The rings a face is closed with, and how far it stands beyond the last section.
const FACE_RINGS: usize = 18;
/// Where the corner of the car is taken to be, back from its end, when a lamp
/// drawn on the face is carried round onto the wing.
const CORNER_M: f32 = 0.03;

/// The lines a car is drawn with, as functions of the station along it.
pub(super) struct Form<'a> {
    pub spec: &'a Spec,
    belt: Vec<(f32, f32)>,
    pub tail: f32,
    pub nose: f32,
    /// The lines below, tabulated every few millimetres: the fields that cut
    /// the shell ask for them millions of times.
    table: Vec<[f32; 7]>,
}

/// The spacing of a form's table, and how far it runs past the ends.
const TABLE_STEP_M: f32 = 0.004;
const TABLE_MARGIN_M: f32 = 0.3;

impl<'a> Form<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        let belt = faired(&spec.top, 10);
        let (tail, nose) = (belt[0].0, belt[belt.len() - 1].0);
        let mut form = Self {
            spec,
            belt,
            tail,
            nose,
            table: Vec::new(),
        };
        let count = ((nose - tail + 2.0 * TABLE_MARGIN_M) / TABLE_STEP_M).ceil() as usize + 2;
        form.table = (0..count)
            .map(|k| {
                let x = tail - TABLE_MARGIN_M + k as f32 * TABLE_STEP_M;
                let c = form.controls(x);
                [
                    form.belt_exact(x),
                    form.floor_exact(x),
                    form.half_width_exact(x),
                    form.top_exact(x),
                    c[7].1,
                    c[8].0,
                    c[8].1,
                ]
            })
            .collect();
        form
    }
    fn look(&self, x: f32, which: usize) -> f32 {
        let at = ((x - self.tail + TABLE_MARGIN_M) / TABLE_STEP_M)
            .clamp(0.0, (self.table.len() - 2) as f32);
        let k = at as usize;
        let t = at - k as f32;
        self.table[k][which] + (self.table[k + 1][which] - self.table[k][which]) * t
    }
    pub fn belt(&self, x: f32) -> f32 {
        self.look(x, 0)
    }
    pub fn floor(&self, x: f32) -> f32 {
        self.look(x, 1)
    }
    pub fn half_width(&self, x: f32) -> f32 {
        self.look(x, 2)
    }
    pub fn top(&self, x: f32) -> f32 {
        self.look(x, 3)
    }
    /// The side glass at a station: the height of its foot, and its head as `(z, y)`.
    pub fn glass(&self, x: f32) -> (f32, (f32, f32)) {
        (self.look(x, 4), (self.look(x, 5), self.look(x, 6)))
    }
    /// Distance to the nearer end.
    fn to_end(&self, x: f32) -> f32 {
        (x - self.tail).min(self.nose - x).max(0.0)
    }
    /// The shoulder: the top of the body side, which is also the bonnet's and the boot's edge.
    fn belt_exact(&self, x: f32) -> f32 {
        sample_line(&self.belt, x.clamp(self.tail + 0.04, self.nose - 0.04))
    }
    fn floor_exact(&self, x: f32) -> f32 {
        // The underside rises a little toward each end, to clear a ramp.
        self.spec.sill + 0.03 * (1.0 - (self.to_end(x) / 0.6).min(1.0)).powi(2)
    }
    /// Half the width at the widest of the section: full between the wheels,
    /// rounding in to the corners at both ends.
    fn half_width_exact(&self, x: f32) -> f32 {
        // The nose draws in over a short length; the tail over a longer,
        // gentler one, as the air wants it.
        let front = 1.0 - ((self.nose - x).max(0.0) / 0.9).min(1.0);
        let rear = 1.0 - ((x - self.tail).max(0.0) / 1.2).min(1.0);
        let (nose, tail) = self.spec.style.taper;
        self.spec.width * 0.5 * (1.0 - nose * front.powf(2.2) - tail * rear.powf(2.0))
    }
    /// How far the flank swells here over a wheel: all of it at an axle, nothing
    /// between them.
    pub fn hip(&self, x: f32) -> f32 {
        let reach = (self.spec.wheel_r + ARCH_GAP) * 1.15;
        let half_base = self.spec.wheelbase * 0.5;
        let near = (x.abs() - half_base).abs() / reach;
        self.spec.style.hip * (-near * near).exp()
    }
    /// How much of a cabin stands on the body here: 0 over the bonnet and the
    /// boot, 1 under the roof, between along the raked glass.
    pub fn cabin(&self, x: f32) -> f32 {
        let [rear_foot, rear_head, front_head, front_foot] = self.spec.cabin;
        let rise = if x < rear_head {
            (x - rear_foot) / (rear_head - rear_foot).max(0.05)
        } else if x > front_head {
            (front_foot - x) / (front_foot - front_head).max(0.05)
        } else {
            1.0
        };
        let rise = rise.clamp(0.0, 1.0);
        // Eased at the head, where the screen runs out into the roof.
        let eased = 1.0 - (1.0 - rise).powf(self.spec.style.bow);
        eased * (0.75 + 0.25 * eased)
    }
    /// The top of the car: the deck, or the roof line over the cabin.
    fn top_exact(&self, x: f32) -> f32 {
        let deck = self.belt_exact(x) + 0.012;
        // The roof is not flat: it crowns a little between the screens.
        // One arc from the foot of one screen to the foot of the other, so
        // the roof has no corner where a screen meets it.
        let [rear_foot, _, _, front_foot] = self.spec.cabin;
        let along = ((x - rear_foot) / (front_foot - rear_foot).max(0.1)).clamp(0.0, 1.0);
        let style = &self.spec.style;
        let crown =
            style.crown * ((1.0 - (2.0 * along - 1.0 - style.peak).abs().min(1.0).powi(2)) - 0.65);
        // And the bonnet and the boot are not flat either: each rises from
        // its edges to a crown along the middle of the car.
        let g = self.cabin(x);
        deck + self.deck_crown(x) * (1.0 - g) + (self.spec.roof + crown - deck) * g
    }
    /// How far the middle of the bonnet (or the boot) stands above its edges.
    fn deck_crown(&self, x: f32) -> f32 {
        let fade = (self.to_end(x) / 0.35).min(1.0);
        self.spec.style.deck_crown * (0.35 + 0.65 * fade)
    }
    /// The distance of a point of the flank from an axle, measured the way
    /// this design's wheel arches are drawn: round, or squared off.
    pub fn to_axle(&self, x: f32, y: f32, axle: f32) -> f32 {
        let n = self.spec.style.arch_square;
        let (dx, dy) = ((x - axle).abs(), (y - self.spec.wheel_r).abs());
        (dx.powf(n) + dy.powf(n)).powf(1.0 / n)
    }
    /// The section's control points at a station, as `(z, y)` from the middle
    /// of the floor, up the side and over to the middle of the roof.
    fn controls(&self, x: f32) -> [(f32, f32); 11] {
        let (floor, belt, top, wide, g) = (
            self.floor_exact(x),
            self.belt_exact(x),
            self.top_exact(x),
            self.half_width_exact(x),
            self.cabin(x),
        );
        let style = &self.spec.style;
        let deck = belt + 0.012;
        let roof_half = self.spec.width * 0.5 - style.tumble;
        let lerp = |a: f32, b: f32| a + (b - a) * g;
        let lerp2 = |a: (f32, f32), b: (f32, f32)| (lerp(a.0, b.0), lerp(a.1, b.1));
        // The body is not a slab. Its side is a barrel: widest a little above
        // its middle, leaning in to the shoulder above and under to the sill
        // below. It swells further over each wheel and is drawn in between
        // them, most of all along the sill.
        let hip = self.hip(x);
        let waist = style.waist * (1.0 - hip / style.hip.max(1.0e-3));
        let high = belt - floor;
        let barrel = style.barrel;
        // Over the cabin the section goes up the glass to the roof; over the
        // bonnet and the boot it turns in at the shoulder and rises to the
        // crown along the middle of the car.
        let crown = top - deck;
        let glass_head = (roof_half.min(wide - 0.16), deck + crown * 0.90 + 0.004);
        let shoulder = wide - style.shoulder - barrel * 0.5;
        [
            (0.0, floor),
            (wide - 0.14 - waist, floor),
            (wide - 0.040 - waist * 1.4 - barrel, floor + 0.070),
            (
                wide - barrel * 0.45 + hip * 0.6 - waist * 0.6,
                floor + high * 0.30,
            ),
            (wide + hip, floor + high * 0.56),
            (wide - barrel * 0.30 + hip * 0.5, belt - high * 0.17),
            (shoulder, belt - 0.004),
            lerp2(
                (shoulder - 0.085, deck + crown * 0.16),
                (shoulder - 0.045, deck + 0.016),
            ),
            lerp2((wide * 0.62, deck + crown * 0.66), glass_head),
            lerp2(
                (wide * 0.30, deck + crown * 0.93),
                (glass_head.0 - 0.11, top - 0.006),
            ),
            (0.0, lerp(top, top + 0.018)),
        ]
    }
    /// The half-section at a station, as `(z, y)` from the middle of the floor,
    /// up the side and over to the middle of the roof.
    fn half_section(&self, x: f32, detail: usize) -> Vec<(f32, f32)> {
        let control = self.controls(x);
        let at = |i: isize| control[i.clamp(0, control.len() as isize - 1) as usize];
        let mut out = Vec::new();
        for (stretch, quads) in STRETCHES.iter().enumerate() {
            let quads = (quads / detail).max(1);
            let i = stretch as isize;
            let (p0, p1, p2, p3) = (at(i - 1), at(i), at(i + 1), at(i + 2));
            for step in 0..quads {
                let t = step as f32 / quads as f32;
                let (t2, t3) = (t * t, t * t * t);
                let spline = |a: f32, b: f32, c: f32, d: f32| {
                    0.5 * (2.0 * b
                        + (c - a) * t
                        + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
                        + (3.0 * b - a - 3.0 * c + d) * t3)
                };
                // Mostly the curve, held a little toward the chord: panels are
                // fair, but the shoulder and the glass foot stay edges.
                let chord = |b: f32, c: f32| b + (c - b) * t;
                let blend = 0.92;
                out.push((
                    chord(p1.0, p2.0) * (1.0 - blend) + spline(p0.0, p1.0, p2.0, p3.0) * blend,
                    chord(p1.1, p2.1) * (1.0 - blend) + spline(p0.1, p1.1, p2.1, p3.1) * blend,
                ));
            }
        }
        out.push(control[control.len() - 1]);
        out
    }

    /// The height of the top of the face at an end: what lamps and grilles are
    /// measured down from.
    pub fn face_top(&self, front: bool) -> f32 {
        self.belt(if front { self.nose } else { self.tail })
    }
    /// Half the width of the car at an end's corners.
    pub fn face_half(&self, front: bool) -> f32 {
        self.half_width(if front {
            self.nose - CORNER_M
        } else {
            self.tail + CORNER_M
        })
    }
    /// The station on the wing that lies `wrap` round the corner from an end.
    pub fn wing_x(&self, front: bool, wrap: f32) -> f32 {
        let want = self.face_half(front) + wrap;
        let toward = if front { 1.0 } else { -1.0 };
        let end = if front { self.nose } else { self.tail };
        let (mut near, mut far) = (0.0_f32, 1.6_f32);
        for _ in 0..24 {
            let mid = (near + far) * 0.5;
            let x = end - toward * mid;
            let (s, _) = self.unwrap(front, Vec3::new(x, 0.0, self.half_width(x)));
            if s < want {
                near = mid;
            } else {
                far = mid;
            }
        }
        end - toward * (near + far) * 0.5
    }
    /// A point of an end of the car on that end's drawing: `(s, y)`, with `s`
    /// measured from the centre line out across the face and on round the
    /// corner along the wing. This is how a lamp is drawn once, in elevation,
    /// and wraps the corner it sits on.
    pub fn unwrap(&self, front: bool, p: Vec3) -> (f32, f32) {
        let back = if front {
            self.nose - CORNER_M - p.x
        } else {
            p.x - (self.tail + CORNER_M)
        };
        let k = 0.03_f32;
        let soft = 0.5 * (back + (back * back + k * k).sqrt());
        (p.z.abs() + soft, p.y)
    }
}

/// Stations from the tail to the nose: close enough everywhere for the cuts
/// that will be made, closer where the profile turns down into the ends, and
/// with one exactly on each of `lines`.
pub(super) fn stations(form: &Form, step: f32, lines: &[f32]) -> Vec<f32> {
    let mut marks = vec![form.tail, form.nose];
    marks.extend(lines.iter().filter(|x| **x > form.tail && **x < form.nose));
    marks.sort_by(|a, b| a.total_cmp(b));
    marks.dedup_by(|a, b| (*a - *b).abs() < 5.0e-3);
    let mut xs = Vec::new();
    for w in marks.windows(2) {
        let count = ((w[1] - w[0]) / step).ceil().max(1.0) as usize;
        for k in 0..count {
            xs.push(w[0] + (w[1] - w[0]) * k as f32 / count as f32);
        }
    }
    xs.push(form.nose);
    // Halve any step over which the shoulder or the roof moves far.
    let mut fine = Vec::with_capacity(xs.len() * 2);
    for w in xs.windows(2) {
        let moved = (form.belt(w[1]) - form.belt(w[0]))
            .abs()
            .max((form.top(w[1]) - form.top(w[0])).abs())
            .max((form.half_width(w[1]) - form.half_width(w[0])).abs() * 2.0);
        let parts = ((moved / (step * 0.45)).ceil() as usize).clamp(1, 12);
        for k in 0..parts {
            fine.push(w[0] + (w[1] - w[0]) * k as f32 / parts as f32);
        }
    }
    fine.push(form.nose);
    fine
}

/// One skin for the whole car: body, pillars and roof.
pub(super) struct Shell {
    pub rings: Vec<Vec<Vec3>>,
    pub xs: Vec<f32>,
    /// Quads in half a ring.
    half: usize,
    /// The stretch each quad of the half-ring belongs to, and its place within it.
    stretch: Vec<(usize, usize)>,
}

impl Shell {
    pub fn new(form: &Form, xs: Vec<f32>, detail: usize) -> Self {
        let mut stretch = Vec::new();
        for (index, quads) in STRETCHES.iter().enumerate() {
            for place in 0..(quads / detail).max(1) {
                stretch.push((index, place));
            }
        }
        let half = stretch.len();
        let rings = xs
            .iter()
            .map(|x| {
                let section = form.half_section(*x, detail);
                let mut ring: Vec<Vec3> =
                    section.iter().map(|(z, y)| Vec3::new(*x, *y, *z)).collect();
                for k in (1..half).rev() {
                    ring.push(Vec3::new(*x, section[k].1, -section[k].0));
                }
                ring
            })
            .collect();
        Self {
            rings,
            xs,
            half,
            stretch,
        }
    }
    /// The stretch of the quad starting at ring point `k`, and its place in it.
    pub fn quad(&self, k: usize) -> (usize, usize) {
        if k < self.half {
            self.stretch[k]
        } else {
            self.stretch[2 * self.half - 1 - k]
        }
    }
    pub fn mid_x(&self, i: usize) -> f32 {
        (self.xs[i] + self.xs[i + 1]) * 0.5
    }
    /// How many rings close each end.
    pub fn face_rings(&self) -> usize {
        FACE_RINGS
    }
    /// Closed at both ends by a face: rings drawing in to a point, turning the
    /// corner tightly and then nearly flat across, as a bumper is.
    pub fn closed(&self, form: &Form) -> Vec<Vec<Vec3>> {
        let face = |ring: &Vec<Vec3>, toward: f32| -> Vec<Vec<Vec3>> {
            let x = ring[0].x;
            let (floor, belt) = (form.floor(x), form.belt(x));
            let mid = (floor + belt) * 0.5;
            let half = form.half_width(x);
            (1..=FACE_RINGS)
                .map(|j| {
                    let angle = j as f32 / FACE_RINGS as f32 * std::f32::consts::FRAC_PI_2;
                    let scale = if j == FACE_RINGS { 0.0 } else { angle.cos() };
                    let shape_of = if toward > 0.0 {
                        form.spec.style.nose
                    } else {
                        form.spec.style.tail
                    };
                    let out = (1.0 - scale.powf(shape_of.square)).powf(1.0 / shape_of.square);
                    ring.iter()
                        .map(|p| {
                            // The rings draw in across the car faster than
                            // down it, so the top edge (the bonnet's lip, the
                            // boot lid's) turns over more tightly than the
                            // corners do in plan.
                            let (y, z) = (mid + (p.y - mid) * scale.powf(0.72), p.z * scale);
                            let v = ((y - floor) / (belt - floor)).clamp(0.0, 1.0);
                            let across = (z / half).abs().min(1.0);
                            let step = |a: f32, b: f32| {
                                let t = ((v - a) / (b - a)).clamp(0.0, 1.0);
                                t * t * (3.0 - 2.0 * t)
                            };
                            // The end of a car is not a wall. In plan it
                            // sweeps back to its corners. In elevation, from
                            // the road up: the chin tucks under, the bumper
                            // stands proud, and above it the panel (lamps and
                            // grille, or the boot's back) leans away to the
                            // edge of the bonnet or the lid.
                            let depth = if toward > 0.0 {
                                form.spec.face.0
                            } else {
                                form.spec.face.1
                            };
                            let shape = 1.0
                                - shape_of.sweep * across.powf(2.4)
                                - shape_of.chin * (1.0 - step(0.0, 0.22))
                                + shape_of.proud * step(0.10, 0.26) * (1.0 - step(0.36, 0.52))
                                - shape_of.lean * step(0.38, 1.0);
                            Vec3::new(x + toward * out * depth * shape.max(0.05), y, z)
                        })
                        .collect()
                })
                .collect()
        };
        let mut out: Vec<Vec<Vec3>> = face(&self.rings[0], -1.0).into_iter().rev().collect();
        out.extend(self.rings.iter().cloned());
        out.extend(face(&self.rings[self.rings.len() - 1], 1.0));
        out
    }
}
