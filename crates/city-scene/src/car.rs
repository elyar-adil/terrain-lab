//! Cars, generated: every design from a handful of numbers, inside and out.
//!
//! A car is the object in a street that people know best, so it is where a
//! model is judged. Each design here is lofted from its own profile and then
//! detailed the way a car is: the shell with its wheel arches, bumpers that
//! round the corners, lamps, grille, shut lines and handles laid on the shell as
//! lifted patches of the same surface; a cabin of pillars and roof with the
//! glazing set in the openings; and behind the glass the seats, dashboard,
//! steering wheel and console that are what a window shows. Wheels have a
//! treaded tyre, a spoked rim and a brake disc.
//!
//! All of that is for a car within a street's width. Past [`CAR_NEAR_M`] the
//! renderer draws the `car/far.*` prototypes instead: the same outline in a few
//! hundred triangles.
use crate::math::Vec3;
use crate::mesh::{GroupStyle, Instance, InstanceGroup, MeshBuilder};
use crate::model::{
    bar, rounded, skin, skin_cut, skin_lifted, soft_block, soft_box, torus_arc, turn,
};

/// Within this distance a car is drawn in full; beyond it, by its far model.
/// The renderer's `cityLod.ts` holds the same number.
pub const CAR_NEAR_M: f32 = 45.0;

struct Spec {
    /// Top of the body from the tail to the nose: `(x, height)`.
    top: &'static [(f32, f32)],
    /// The glasshouse: `x` of the rear glass's foot and head, the windscreen's head and foot.
    cabin: [f32; 4],
    roof: f32,
    width: f32,
    sill: f32,
    wheelbase: f32,
    wheel_r: f32,
    spokes: usize,
    /// Rows of seats.
    rows: usize,
    roof_rails: bool,
    /// A boot lid behind the rear glass (a saloon), or a tailgate.
    boot: bool,
}

/// The cars of a Chinese street: saloons most of all, then hatchbacks, SUVs
/// and the small van (面包车) that does every delivery. Index 0 is the plain
/// saloon the unsuffixed prototype keys draw.
const SPECS: [Spec; 6] = [
    Spec {
        top: &[
            (-2.32, 0.60),
            (-2.26, 0.94),
            (-1.45, 1.00),
            (1.05, 0.98),
            (1.85, 0.90),
            (2.26, 0.76),
            (2.34, 0.58),
        ],
        cabin: [-1.58, -0.86, 0.30, 1.14],
        roof: 1.45,
        width: 1.81,
        sill: 0.30,
        wheelbase: 2.69,
        wheel_r: 0.315,
        spokes: 5,
        rows: 2,
        roof_rails: false,
        boot: true,
    },
    Spec {
        top: &[
            (-2.03, 0.62),
            (-1.99, 0.98),
            (-1.25, 1.01),
            (0.95, 0.99),
            (1.68, 0.91),
            (2.02, 0.76),
            (2.09, 0.58),
        ],
        cabin: [-1.96, -1.52, 0.24, 1.02],
        roof: 1.47,
        width: 1.74,
        sill: 0.30,
        wheelbase: 2.52,
        wheel_r: 0.30,
        spokes: 7,
        rows: 2,
        roof_rails: false,
        boot: false,
    },
    Spec {
        top: &[
            (-2.30, 0.74),
            (-2.26, 1.10),
            (-1.30, 1.15),
            (1.00, 1.13),
            (1.88, 1.05),
            (2.27, 0.90),
            (2.35, 0.70),
        ],
        cabin: [-2.22, -1.90, 0.28, 1.08],
        roof: 1.70,
        width: 1.88,
        sill: 0.37,
        wheelbase: 2.72,
        wheel_r: 0.36,
        spokes: 6,
        rows: 2,
        roof_rails: true,
        boot: false,
    },
    Spec {
        top: &[
            (-1.98, 0.68),
            (-1.95, 1.04),
            (-1.00, 1.06),
            (1.22, 1.04),
            (1.72, 0.98),
            (1.97, 0.80),
            (2.02, 0.62),
        ],
        cabin: [-1.94, -1.86, 0.92, 1.52],
        roof: 1.84,
        width: 1.62,
        sill: 0.33,
        wheelbase: 2.50,
        wheel_r: 0.29,
        spokes: 4,
        rows: 3,
        roof_rails: false,
        boot: false,
    },
    Spec {
        top: &[
            (-2.42, 0.60),
            (-2.36, 0.93),
            (-1.55, 0.99),
            (1.10, 0.97),
            (1.95, 0.88),
            (2.36, 0.74),
            (2.44, 0.57),
        ],
        cabin: [-1.70, -1.02, 0.28, 1.20],
        roof: 1.43,
        width: 1.85,
        sill: 0.29,
        wheelbase: 2.80,
        wheel_r: 0.325,
        spokes: 10,
        rows: 2,
        roof_rails: false,
        boot: true,
    },
    Spec {
        top: &[
            (-2.22, 0.72),
            (-2.18, 1.06),
            (-1.28, 1.10),
            (0.98, 1.08),
            (1.80, 1.00),
            (2.18, 0.86),
            (2.26, 0.68),
        ],
        cabin: [-2.14, -1.74, 0.20, 1.04],
        roof: 1.62,
        width: 1.80,
        sill: 0.35,
        wheelbase: 2.62,
        wheel_r: 0.34,
        spokes: 5,
        rows: 2,
        roof_rails: true,
        boot: false,
    },
];

/// How many designs there are; variant `n > 0` is keyed `car/<part>~n`.
pub const CAR_VARIANTS: usize = SPECS.len();

/// The parts of the full model, and of the far one.
const NEAR_PARTS: [&str; 10] = [
    "body",
    "glass",
    "wheel",
    "trim",
    "chrome",
    "lamp.front",
    "lamp.rear",
    "interior",
    "plate",
    "well",
];
const FAR_PARTS: [&str; 3] = ["far.body", "far.glass", "far.wheel"];

/// The prototype key of a car part for a variant.
pub fn car_key(part: &str, variant: usize) -> String {
    if variant == 0 {
        format!("car/{part}")
    } else {
        format!("car/{part}~{variant}")
    }
}

/// Every prototype key this module binds.
pub fn prototype_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for variant in 0..CAR_VARIANTS {
        for part in NEAR_PARTS.iter().chain(FAR_PARTS.iter()) {
            keys.push(car_key(part, variant));
        }
    }
    keys
}

/// Which design stands at a place: by where it is, so the choice draws nothing
/// from the placement's random stream.
pub fn car_variant_at(x: f32, z: f32) -> usize {
    let cell = worldgen_core::hash::avalanche01(
        0x00ca_7a11,
        (x * 7.0).round() as i32,
        (z * 7.0).round() as i32,
        3,
    );
    // Half the street is saloons (two designs); the rest share the other half.
    let pick = (cell * 10.0) as usize;
    [0, 0, 0, 4, 4, 1, 1, 2, 5, 3][pick.min(9)]
}

/// Upholstery, by place: cloth greys most often, then beige, black and tan.
fn interior_tone(x: f32, z: f32) -> [f32; 3] {
    let cell = worldgen_core::hash::avalanche01(
        0x1a7e_0105,
        (x * 5.0).round() as i32,
        (z * 5.0).round() as i32,
        9,
    );
    const TONES: [[f32; 3]; 5] = [
        [0.115, 0.115, 0.12],
        [0.20, 0.20, 0.205],
        [0.40, 0.35, 0.28],
        [0.045, 0.045, 0.05],
        [0.30, 0.19, 0.12],
    ];
    TONES[((cell * TONES.len() as f32) as usize).min(TONES.len() - 1)]
}

fn instances_of(x: f32, y: f32, z: f32, yaw: f32, paint: [f32; 3]) -> Vec<(String, Instance)> {
    let variant = car_variant_at(x, z);
    let tone = interior_tone(x, z);
    NEAR_PARTS
        .iter()
        .chain(FAR_PARTS.iter())
        .map(|part| {
            let tint = match *part {
                "body" | "far.body" => paint,
                "glass" | "far.glass" => [0.6, 0.7, 0.78],
                "wheel" | "far.wheel" => [0.15, 0.15, 0.16],
                "interior" => tone,
                _ => [1.0, 1.0, 1.0],
            };
            (
                car_key(part, variant),
                Instance::new(x, y, z, yaw, 1.0, tint),
            )
        })
        .collect()
}

/// One car standing at a place, nose along local `+X`: every part of both models.
pub fn add_car(builder: &mut MeshBuilder, x: f32, y: f32, z: f32, yaw: f32, paint: [f32; 3]) {
    for (key, instance) in instances_of(x, y, z, yaw, paint) {
        builder.add_instance(&key, instance);
    }
}

/// The same, into instance lists that have already been built (the traffic's
/// vehicles are known only after the geometry is).
pub fn add_car_to(
    groups: &mut Vec<InstanceGroup>,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    paint: [f32; 3],
) {
    for (key, instance) in instances_of(x, y, z, yaw, paint) {
        match groups.iter_mut().find(|group| group.key == key) {
            Some(group) => group.instances.push(instance),
            None => groups.push(InstanceGroup {
                key,
                instances: vec![instance],
            }),
        }
    }
}

/// Paint for a moving vehicle, by the traffic model's colour index: the whites,
/// blacks and silvers that are most of a Chinese street, then the rest.
pub fn fleet_paint(index: u8) -> [f32; 3] {
    const PAINTS: [[f32; 3]; 12] = [
        [0.87, 0.87, 0.85],
        [0.87, 0.87, 0.85],
        [0.87, 0.87, 0.85],
        [0.009, 0.009, 0.011],
        [0.009, 0.009, 0.011],
        [0.009, 0.009, 0.011],
        [0.34, 0.37, 0.39],
        [0.34, 0.37, 0.39],
        [0.105, 0.112, 0.125],
        [0.262, 0.012, 0.012],
        [0.018, 0.051, 0.158],
        [0.147, 0.102, 0.058],
    ];
    PAINTS[index as usize % PAINTS.len()]
}

pub fn build_prototypes(builder: &mut MeshBuilder) {
    for (variant, spec) in SPECS.iter().enumerate() {
        let key = |part: &str| car_key(part, variant);
        build_body(builder, spec, &key, false);
        build_fittings(builder, spec, &key);
        build_interior(builder, spec, &key);
        build_wheels(builder, spec, &key);
        build_body(builder, spec, &key, true);
        build_far_wheels(builder, spec, &key);
        for part in NEAR_PARTS.iter().chain(FAR_PARTS.iter()) {
            let key = key(part);
            // Glass casts no shadow (it would black out the cabin it glazes),
            // and nothing inside the car needs to cast one.
            let solid = matches!(
                *part,
                "body" | "wheel" | "far.body" | "far.glass" | "far.wheel"
            );
            builder.style(
                &key,
                GroupStyle {
                    cast_shadow: solid,
                    receive_shadow: *part != "glass",
                    ..GroupStyle::default()
                },
            );
            builder.bind(&key, &key);
        }
    }
}

fn sample_line(line: &[(f32, f32)], x: f32) -> f32 {
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

fn pole_of(ring: &[Vec3]) -> Vec<Vec3> {
    let centre = ring.iter().fold(Vec3::new(0.0, 0.0, 0.0), |a, p| a + *p) / ring.len() as f32;
    vec![centre; ring.len()]
}

/// Stations from `from` to `to`: `even` of them evenly, plus a pair either side
/// of every `line` so that a thin feature there has quads of its own.
fn stations(from: f32, to: f32, even: usize, lines: &[f32], ends: f32) -> Vec<f32> {
    let mut xs: Vec<f32> = (0..=even)
        .map(|k| from + (to - from) * k as f32 / even as f32)
        .collect();
    for line in lines {
        xs.push(line - 0.007);
        xs.push(line + 0.007);
    }
    // The ends round over a short distance and need rings to do it with.
    if ends > 0.0 {
        for k in 1..10 {
            let d = ends * (k as f32 / 10.0).powi(2);
            xs.push(from + d);
            xs.push(to - d);
        }
    }
    xs.retain(|x| *x >= from && *x <= to);
    xs.sort_by(|a, b| a.total_cmp(b));
    xs.dedup_by(|a, b| (*a - *b).abs() < 2.0e-3);
    xs
}

/// How many quads each stretch of the half-section gets, from the middle of the
/// floor round to the middle of the roof. The count is the same at every
/// station, so a feature of the section (the shoulder, the foot of the glass) is
/// a line of the mesh from one end of the car to the other.
const STRETCHES: [usize; 9] = [5, 3, 5, 5, 3, 2, 6, 3, 5];
const FLOOR: usize = 0;
const ROCKER: usize = 1;
const SHOULDER: usize = 5;
const SIDE_GLASS: usize = 6;
const ROOF_RAIL: usize = 7;
const ROOF: usize = 8;

struct Lines {
    front_door: f32,
    b_pillar: f32,
    rear_door: f32,
    bonnet: f32,
    boot: f32,
}

fn lines_of(spec: &Spec) -> Lines {
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    Lines {
        front_door: front_foot - 0.10,
        b_pillar: (rear_head + front_head) * 0.5 + 0.12,
        rear_door: (rear_head + 0.10).max(-spec.wheelbase * 0.5 + spec.wheel_r + 0.10),
        bonnet: front_foot + 0.10,
        boot: rear_foot - 0.07,
    }
}

/// The lines a car is drawn with, as functions of the station along it.
struct Form<'a> {
    spec: &'a Spec,
    belt: Vec<(f32, f32)>,
    tail: f32,
    nose: f32,
}

impl<'a> Form<'a> {
    fn new(spec: &'a Spec) -> Self {
        let belt = rounded(spec.top, 6);
        let (tail, nose) = (belt[0].0, belt[belt.len() - 1].0);
        Self {
            spec,
            belt,
            tail,
            nose,
        }
    }
    /// Distance to the nearer end.
    fn from_end(&self, x: f32) -> f32 {
        (x - self.tail).min(self.nose - x).max(0.0)
    }
    /// The shoulder: the top of the body side, which is also the bonnet's and the boot's edge.
    fn belt(&self, x: f32) -> f32 {
        sample_line(&self.belt, x.clamp(self.tail + 0.05, self.nose - 0.05))
    }
    fn floor(&self, x: f32) -> f32 {
        // The bumpers hang a little below the sill.
        self.spec.sill - 0.05 * (1.0 - (self.from_end(x) / 0.5).min(1.0)).powi(2)
    }
    /// Half the width at the widest of the section: full between the wheels,
    /// rounding in to the corners at both ends.
    fn half_width(&self, x: f32) -> f32 {
        let t = 1.0 - (self.from_end(x) / 0.45).min(1.0);
        self.spec.width * 0.5 * (1.0 - 0.17 * t.powf(2.3))
    }
    /// How much of a cabin stands on the body here: 0 over the bonnet and the
    /// boot, 1 under the roof, between along the raked glass.
    fn cabin(&self, x: f32) -> f32 {
        let [rear_foot, rear_head, front_head, front_foot] = self.spec.cabin;
        let rise = if x < rear_head {
            (x - rear_foot) / (rear_head - rear_foot).max(0.05)
        } else if x > front_head {
            (front_foot - x) / (front_foot - front_head).max(0.05)
        } else {
            1.0
        };
        let rise = rise.clamp(0.0, 1.0);
        1.0 - (1.0 - rise).powf(1.25)
    }
    /// The top of the car: the deck, or the roof line over the cabin.
    fn top(&self, x: f32) -> f32 {
        let deck = self.belt(x) + 0.012;
        deck + (self.spec.roof - deck) * self.cabin(x)
    }
    /// The half-section at a station, as `(z, y)` from the middle of the floor,
    /// up the side and over to the middle of the roof.
    fn half_section(&self, x: f32, detail: usize) -> Vec<(f32, f32)> {
        let (floor, belt, top, wide, g) = (
            self.floor(x),
            self.belt(x),
            self.top(x),
            self.half_width(x),
            self.cabin(x),
        );
        let deck = belt + 0.012;
        let roof_half = self.spec.width * 0.5 - 0.25;
        let lerp = |a: f32, b: f32| a + (b - a) * g;
        let glass_head = (
            lerp(wide - 0.22, roof_half.min(wide - 0.16)),
            deck + (top - deck) * 0.90 + 0.004,
        );
        let control = [
            (0.0, floor),
            (wide - 0.11, floor),
            (wide - 0.014, floor + 0.075),
            (wide, floor + (belt - floor) * 0.46),
            (wide - 0.010, belt - 0.11),
            (wide - 0.036, belt - 0.004),
            (lerp(wide - 0.10, wide - 0.066), deck + 0.004 + 0.012 * g),
            glass_head,
            (glass_head.0 - 0.10, top + 0.004),
            (0.0, top + 0.022),
        ];
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
                let blend = 0.7;
                out.push((
                    chord(p1.0, p2.0) * (1.0 - blend) + spline(p0.0, p1.0, p2.0, p3.0) * blend,
                    chord(p1.1, p2.1) * (1.0 - blend) + spline(p0.1, p1.1, p2.1, p3.1) * blend,
                ));
            }
        }
        out.push(control[control.len() - 1]);
        out
    }
}

/// One skin for the whole car: body, pillars and roof.
struct Shell {
    rings: Vec<Vec<Vec3>>,
    xs: Vec<f32>,
    /// Quads in half a ring.
    half: usize,
    /// The stretch each quad of the half-ring belongs to, and its place within it.
    stretch: Vec<(usize, usize)>,
}

impl Shell {
    fn new(form: &Form, xs: Vec<f32>, detail: usize) -> Self {
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
    /// The stretch of the quad starting at ring point `k`, its place in the
    /// stretch, and which side of the car it is on.
    fn quad(&self, k: usize) -> (usize, usize, f32) {
        if k < self.half {
            let (s, place) = self.stretch[k];
            (s, place, 1.0)
        } else {
            let (s, place) = self.stretch[2 * self.half - 1 - k];
            (s, place, -1.0)
        }
    }
    fn mid_x(&self, i: usize) -> f32 {
        (self.xs[i] + self.xs[i + 1]) * 0.5
    }
    /// Closed at both ends by rings drawing in to a point, bowed a little: a
    /// bumper's face.
    fn closed(&self, form: &Form) -> Vec<Vec<Vec3>> {
        let face = |ring: &Vec<Vec3>, toward: f32| -> Vec<Vec<Vec3>> {
            let x = ring[0].x;
            let mid = (form.floor(x) + form.belt(x)) * 0.5;
            (1..=FACE_RINGS)
                .map(|j| {
                    let angle = j as f32 / FACE_RINGS as f32 * std::f32::consts::FRAC_PI_2;
                    let scale = if j == FACE_RINGS { 0.0 } else { angle.cos() };
                    ring.iter()
                        .map(|p| {
                            Vec3::new(
                                x + toward * FACE_DEPTH_M * angle.sin(),
                                mid + (p.y - mid) * scale,
                                p.z * scale,
                            )
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

const FACE_RINGS: usize = 8;
/// How far a bumper's face bows out beyond the last section.
const FACE_DEPTH_M: f32 = 0.09;

/// Is this quad of the shell glazing? Side glass between the pillars, a
/// windscreen and a rear glass between the roof rails.
fn glazed(spec: &Spec, form: &Form, shell: &Shell, i: usize, k: usize) -> bool {
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    let lines = lines_of(spec);
    let x = shell.mid_x(i);
    let (stretch, place, _) = shell.quad(k);
    let pillar = |line: f32| (x - line).abs() < 0.042;
    let side = stretch == SIDE_GLASS
        && place > 0
        && form.cabin(x) > 0.42
        && !pillar(lines.b_pillar)
        && !(spec.rows > 2 && pillar((rear_head + lines.b_pillar) * 0.5 - 0.1))
        && x < front_head + (front_foot - front_head) * 0.62
        && x > rear_head - (rear_head - rear_foot) * 0.62;
    let over = stretch == ROOF || (stretch == ROOF_RAIL && place > 0);
    let screen = over && x > front_head + 0.035 && x < front_foot - 0.045;
    let rear = over && x < rear_head - 0.035 && x > rear_foot + 0.045;
    side || screen || rear
}

/// The shell and what is cut into it: wheel arches, glazing, the lining seen
/// through the glass, the dark of the floor and the shut lines.
fn build_body(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String, far: bool) {
    let form = Form::new(spec);
    let half_base = spec.wheelbase * 0.5;
    let arch = spec.wheel_r + 0.06;
    let lines = lines_of(spec);
    let part = |name: &str| {
        if far {
            key(&format!("far.{name}"))
        } else {
            key(name)
        }
    };
    if far {
        let shell = Shell::new(&form, stations(form.tail, form.nose, 18, &[], 0.0), 3);
        let closed = shell.closed(&form);
        let ends = FACE_RINGS;
        let glass = |i: usize, k: usize| {
            i >= ends && i - ends < shell.xs.len() - 1 && glazed(spec, &form, &shell, i - ends, k)
        };
        skin_cut(builder, &part("body"), &closed, |i, k| !glass(i, k));
        skin_lifted(builder, &part("glass"), &closed, -0.004, glass);
        return;
    }
    // Stations: even, a pair either side of each shut line, and close-set across
    // the wheel arches, whose edge is cut out of the mesh.
    let mut xs = stations(
        form.tail,
        form.nose,
        64,
        &[
            lines.front_door,
            lines.b_pillar,
            lines.rear_door,
            lines.bonnet,
            lines.boot,
        ],
        0.0,
    );
    for axle in [-half_base, half_base] {
        for step in 0..=26 {
            xs.push(axle - arch - 0.02 + (2.0 * arch + 0.04) * step as f32 / 26.0);
        }
    }
    xs.retain(|x| *x >= form.tail && *x <= form.nose);
    xs.sort_by(|a, b| a.total_cmp(b));
    xs.dedup_by(|a, b| (*a - *b).abs() < 4.0e-3);
    let mut shell = Shell::new(&form, xs, 1);
    // Bring the vertices nearest each arch onto its circle, so the opening is a
    // true arc and not the staircase of the grid.
    let centre_y = spec.wheel_r;
    let radial = |p: Vec3, axle: f32| ((p.x - axle).powi(2) + (p.y - centre_y).powi(2)).sqrt();
    for ring in &mut shell.rings {
        for p in ring.iter_mut() {
            if p.z.abs() < spec.width * 0.5 - 0.30 {
                continue;
            }
            for axle in [-half_base, half_base] {
                let r = radial(*p, axle);
                if (r - arch).abs() < 0.024 && r > 1.0e-3 {
                    let pull = arch / r;
                    p.x = axle + (p.x - axle) * pull;
                    p.y = centre_y + (p.y - centre_y) * pull;
                }
            }
        }
    }
    let in_arch = |i: usize, k: usize| -> bool {
        let (stretch, _, _) = shell.quad(k);
        if stretch > 4 {
            return false;
        }
        let around = shell.rings[0].len();
        let corners = [
            shell.rings[i][k],
            shell.rings[i + 1][k],
            shell.rings[i + 1][(k + 1) % around],
            shell.rings[i][(k + 1) % around],
        ];
        let outer = corners.iter().all(|p| p.z.abs() > spec.width * 0.5 - 0.42);
        outer
            && [-half_base, half_base]
                .iter()
                .any(|axle| corners.iter().all(|p| radial(*p, *axle) <= arch + 2.0e-3))
    };
    let closed = shell.closed(&form);
    let ends = FACE_RINGS;
    let on_shell = |i: usize| i >= ends && i - ends < shell.xs.len() - 1;
    let glass = |i: usize, k: usize| on_shell(i) && glazed(spec, &form, &shell, i - ends, k);
    skin_cut(builder, &part("body"), &closed, |i, k| {
        !glass(i, k) && !(on_shell(i) && in_arch(i - ends, k))
    });
    skin_lifted(builder, &part("glass"), &shell.rings, -0.006, |i, k| {
        glazed(spec, &form, &shell, i, k)
    });
    // The headlining and the pillars' inner faces.
    skin_lifted(builder, &key("interior"), &shell.rings, -0.022, |i, k| {
        let (stretch, _, _) = shell.quad(k);
        stretch >= SIDE_GLASS
            && form.cabin(shell.mid_x(i)) > 0.05
            && !glazed(spec, &form, &shell, i, k)
    });
    // Black: the floor, the sills, and the gaps of the doors, bonnet and boot.
    let near = |x: f32, line: f32| (x - line).abs() < 0.0075;
    skin_lifted(builder, &key("trim"), &shell.rings, 0.0015, |i, k| {
        if in_arch(i, k) {
            return false;
        }
        let x = shell.mid_x(i);
        let (stretch, place, _) = shell.quad(k);
        let side = (2..=SHOULDER).contains(&stretch);
        let deck = stretch >= SIDE_GLASS && form.cabin(x) < 0.02;
        stretch == FLOOR
            || stretch == ROCKER
            || (stretch == 2 && place < 2 && x > -half_base + arch && x < half_base - arch)
            || (side
                && (near(x, lines.front_door)
                    || near(x, lines.b_pillar)
                    || near(x, lines.rear_door)))
            || ((deck || stretch == SHOULDER || stretch == 4) && near(x, lines.bonnet))
            || (spec.boot && (deck || stretch == SHOULDER || stretch == 4) && near(x, lines.boot))
    });
    // The cowl the wipers park on, at the foot of the windscreen.
    let [_, _, _, front_foot] = spec.cabin;
    skin_lifted(builder, &key("trim"), &shell.rings, 0.003, |i, k| {
        let (stretch, place, _) = shell.quad(k);
        let x = shell.mid_x(i);
        (stretch == ROOF || (stretch == ROOF_RAIL && place > 0))
            && x > front_foot - 0.045
            && x < front_foot + 0.035
    });
}

/// Everything fitted to the shell as a part of its own: wheel wells and arch
/// lips, lamps, grille, intakes, plates, handles, mirrors, wipers, exhaust.
fn build_fittings(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let form = Form::new(spec);
    let half_base = spec.wheelbase * 0.5;
    let arch = spec.wheel_r + 0.06;
    let lines = lines_of(spec);
    let x_axis = Vec3::new(1.0, 0.0, 0.0);
    let y_axis = Vec3::new(0.0, 1.0, 0.0);
    let z_axis = Vec3::new(0.0, 0.0, 1.0);
    for axle in [-half_base, half_base] {
        let wide = form.half_width(axle);
        for side in [-1.0_f32, 1.0] {
            // The well: a dark drum over the wheel, open to the road.
            let centre = Vec3::new(axle, spec.wheel_r, 0.0);
            let rings: Vec<Vec<Vec3>> = [wide - 0.44, wide - 0.012]
                .iter()
                .map(|z| {
                    (0..=24)
                        .map(|k| {
                            let angle = (-0.10 + 1.20 * k as f32 / 24.0) * std::f32::consts::PI;
                            centre
                                + Vec3::new(
                                    angle.cos() * (arch + 0.012),
                                    angle.sin() * (arch + 0.012),
                                    side * z,
                                )
                        })
                        .collect()
                })
                .collect();
            skin(builder, &key("well"), &rings);
            let back: Vec<Vec3> = rings[0].clone();
            skin(builder, &key("well"), &[pole_of(&back), back]);
            torus_arc(
                builder,
                &key("trim"),
                Vec3::new(axle, spec.wheel_r, side * (wide - 0.006)),
                Vec3::new(0.0, 0.0, side),
                Vec3::new(side, 0.0, 0.0),
                arch + 0.004,
                0.019,
                (-0.035, 0.535),
            );
        }
    }
    // The two faces. Each has lamps at its corners; the front has a grille slot
    // with a bright bar and an intake under the bumper, the back a valance and
    // the exhaust; both carry a plate. Every part is placed on the bowed face
    // itself, so it stands a centimetre proud of the paint and no more.
    for (front, x_end) in [(true, form.nose), (false, form.tail)] {
        let toward = if front { 1.0_f32 } else { -1.0 };
        let (floor, belt) = (form.floor(x_end), form.belt(x_end));
        let (mid, half_h) = ((floor + belt) * 0.5, (belt - floor) * 0.5);
        let wide = form.half_width(x_end);
        // A point on the face, by fractions of its half-width and half-height.
        let on_face = |across: f32, up: f32| {
            let s = across.abs().max(up.abs()).min(1.0);
            Vec3::new(
                x_end + toward * (FACE_DEPTH_M * (1.0 - s * s).sqrt() - 0.018),
                mid + up * half_h,
                across * wide,
            )
        };
        let lamp = if front {
            key("lamp.front")
        } else {
            key("lamp.rear")
        };
        for side in [-1.0_f32, 1.0] {
            let yaw = 0.42_f32;
            let along = Vec3::new(toward * yaw.cos(), 0.0, side * yaw.sin());
            let across = Vec3::new(-toward * yaw.sin(), 0.0, side * yaw.cos()) * toward;
            soft_block(
                builder,
                &lamp,
                on_face(side * 0.66, if front { 0.50 } else { 0.42 }),
                along * 0.026,
                y_axis * (half_h * if front { 0.27 } else { 0.34 }),
                across * (wide * 0.25),
                3.4,
            );
        }
        if front {
            let grille = on_face(0.0, 0.50);
            soft_block(
                builder,
                &key("trim"),
                grille,
                x_axis * 0.022,
                y_axis * (half_h * 0.24),
                z_axis * (wide * 0.36),
                5.0,
            );
            for rung in [-0.5_f32, 0.5] {
                let at = grille + y_axis * (half_h * 0.24 * rung);
                bar(
                    builder,
                    &key("chrome"),
                    at + Vec3::new(0.022, 0.0, -wide * 0.33),
                    at + Vec3::new(0.022, 0.0, wide * 0.33),
                    0.006,
                    0.006,
                    6,
                );
            }
            bar(
                builder,
                &key("chrome"),
                grille + Vec3::new(0.024, 0.0, -wide * 0.34),
                grille + Vec3::new(0.024, 0.0, wide * 0.34),
                0.009,
                0.009,
                8,
            );
            soft_block(
                builder,
                &key("trim"),
                on_face(0.0, -0.58),
                x_axis * 0.02,
                y_axis * (half_h * 0.18),
                z_axis * (wide * 0.46),
                5.0,
            );
            for side in [-1.0_f32, 1.0] {
                // Fog lamps in the corners of the intake.
                turn(
                    builder,
                    &key("lamp.front"),
                    on_face(side * 0.66, -0.58),
                    x_axis,
                    &rounded(&[(0.0, 0.026), (0.030, 0.022), (0.038, 0.004)], 3),
                    16,
                );
            }
        } else {
            soft_block(
                builder,
                &key("trim"),
                on_face(0.0, -0.62),
                x_axis * 0.02,
                y_axis * (half_h * 0.16),
                z_axis * (wide * 0.55),
                5.0,
            );
            bar(
                builder,
                &key("chrome"),
                Vec3::new(x_end + 0.22, floor + 0.035, wide * 0.52),
                Vec3::new(x_end - 0.07, floor + 0.035, wide * 0.52),
                0.027,
                0.030,
                12,
            );
        }
        soft_block(
            builder,
            &key("plate"),
            on_face(0.0, if front { -0.10 } else { 0.10 }) + x_axis * (toward * 0.016),
            x_axis * 0.006,
            y_axis * 0.07,
            z_axis * 0.22,
            9.0,
        );
    }
    // Door handles, a mirror on each front door.
    let handle_y = |x: f32| form.belt(x) - 0.105;
    for door_rear in [lines.b_pillar, lines.rear_door] {
        let x = door_rear - 0.16;
        for side in [-1.0_f32, 1.0] {
            soft_block(
                builder,
                &key("chrome"),
                Vec3::new(x, handle_y(x), side * (form.half_width(x) - 0.004)),
                x_axis * 0.075,
                y_axis * 0.013,
                z_axis * 0.014,
                3.0,
            );
        }
    }
    let mirror_x = spec.cabin[3] - 0.30;
    for side in [-1.0_f32, 1.0] {
        let root = Vec3::new(
            mirror_x,
            form.belt(mirror_x) + 0.03,
            side * (form.half_width(mirror_x) - 0.06),
        );
        let head = root + Vec3::new(-0.03, 0.055, side * 0.085);
        bar(builder, &key("trim"), root, head, 0.016, 0.014, 6);
        let facing = Vec3::new(-0.96, 0.0, side * 0.28);
        let sideways = Vec3::new(side * 0.28, 0.0, 0.96) * side;
        soft_block(
            builder,
            &key("body"),
            head,
            facing * 0.035,
            y_axis * 0.058,
            sideways * 0.08,
            3.0,
        );
        soft_block(
            builder,
            &key("chrome"),
            head + facing * 0.034,
            facing * 0.004,
            y_axis * 0.046,
            sideways * 0.066,
            6.0,
        );
    }
    // Two wiper arms, parked along the foot of the screen.
    let front_foot = spec.cabin[3];
    let cowl = form.belt(front_foot) + 0.03;
    for (z0, z1) in [(-0.60_f32, -0.10_f32), (-0.02, 0.48)] {
        bar(
            builder,
            &key("trim"),
            Vec3::new(front_foot + 0.01, cowl, z0),
            Vec3::new(front_foot - 0.035, cowl + 0.018, z1),
            0.008,
            0.006,
            6,
        );
    }
    if spec.roof_rails {
        let [_, rear_head, front_head, _] = spec.cabin;
        for side in [-1.0_f32, 1.0] {
            let z = side * (spec.width * 0.5 - 0.37);
            bar(
                builder,
                &key("chrome"),
                Vec3::new(rear_head + 0.05, spec.roof + 0.055, z),
                Vec3::new(front_head - 0.05, spec.roof + 0.055, z),
                0.015,
                0.015,
                8,
            );
            for x in [rear_head + 0.12, front_head - 0.12] {
                bar(
                    builder,
                    &key("chrome"),
                    Vec3::new(x, spec.roof, z),
                    Vec3::new(x, spec.roof + 0.055, z),
                    0.013,
                    0.013,
                    6,
                );
            }
        }
    }
}

/// What the windows show: floor, seats, dashboard, steering wheel, console.
fn build_interior(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let top: Vec<(f32, f32)> = rounded(spec.top, 6);
    let [rear_foot, rear_head, _, front_foot] = spec.cabin;
    let floor = spec.sill + 0.12;
    let half = spec.width * 0.5;
    let cloth = key("interior");
    let trim = key("trim");
    // Seat heights scale to the headroom the design has.
    let k = ((spec.roof - floor - 0.07) / 1.11).min(1.0);
    soft_box(
        builder,
        &trim,
        Vec3::new((rear_foot + front_foot) * 0.5, floor - 0.03, 0.0),
        Vec3::new((front_foot - rear_foot) * 0.5 + 0.1, 0.035, half - 0.10),
        0.0,
        8.0,
    );
    let seat = |builder: &mut MeshBuilder, x: f32, z: f32, wide: f32| {
        soft_box(
            builder,
            &cloth,
            Vec3::new(x + 0.05, floor + 0.21 * k, z),
            Vec3::new(0.25, 0.075, wide),
            0.06,
            3.2,
        );
        soft_box(
            builder,
            &cloth,
            Vec3::new(x - 0.25, floor + 0.60 * k, z),
            Vec3::new(0.065, 0.34 * k, wide - 0.01),
            0.36,
            3.2,
        );
    };
    let headrest = |builder: &mut MeshBuilder, x: f32, z: f32| {
        soft_box(
            builder,
            &cloth,
            Vec3::new(x - 0.40, floor + 1.02 * k, z),
            Vec3::new(0.05, 0.085, 0.12),
            0.25,
            2.6,
        );
        for dz in [-0.05_f32, 0.05] {
            bar(
                builder,
                &key("chrome"),
                Vec3::new(x - 0.375, floor + 0.88 * k, z + dz),
                Vec3::new(x - 0.395, floor + 0.96 * k, z + dz),
                0.006,
                0.006,
                6,
            );
        }
    };
    // Front seats either side of the console; the driver sits on the left (-z).
    let front = front_foot - 0.98;
    for z in [-0.37_f32, 0.37] {
        seat(builder, front, z, 0.235);
        headrest(builder, front, z);
    }
    // The rows behind are benches.
    let last = (rear_head + 0.42).max(rear_foot + 0.50);
    for row in 1..spec.rows {
        let x = if spec.rows == 2 {
            last
        } else {
            front + (last - front) * row as f32 / (spec.rows - 1) as f32
        };
        seat(builder, x, 0.0, half - 0.20);
        for z in [-0.40_f32, 0.40] {
            headrest(builder, x, z);
        }
    }
    // Dashboard across the car under the windscreen, with the binnacle in front of the driver.
    let waist = sample_line(&top, front_foot - 0.3) - 0.04;
    soft_box(
        builder,
        &trim,
        Vec3::new(front_foot - 0.30, waist - 0.11, 0.0),
        Vec3::new(0.21, 0.12, half - 0.11),
        0.0,
        3.0,
    );
    soft_box(
        builder,
        &trim,
        Vec3::new(front_foot - 0.40, waist + 0.005, -0.37),
        Vec3::new(0.10, 0.045, 0.17),
        0.0,
        2.6,
    );
    // The wheel, on its column.
    let hub = Vec3::new(front_foot - 0.62, waist - 0.03, -0.37);
    let facing = Vec3::new(-0.92, 0.39, 0.0);
    torus_arc(
        builder,
        &trim,
        hub,
        facing,
        Vec3::new(0.0, 0.0, 1.0),
        0.18,
        0.016,
        (0.0, 1.0),
    );
    bar(
        builder,
        &trim,
        hub,
        Vec3::new(front_foot - 0.42, waist - 0.11, -0.37),
        0.03,
        0.035,
        8,
    );
    for angle in [0.0_f32, 2.1, 4.2] {
        let out = Vec3::new(0.39 * angle.sin(), 0.92 * angle.sin(), angle.cos());
        bar(builder, &trim, hub, hub + out * 0.18, 0.014, 0.011, 6);
    }
    // Console between the seats, with the gear lever.
    soft_box(
        builder,
        &trim,
        Vec3::new(front + 0.12, floor + 0.15 * k, 0.0),
        Vec3::new(0.42, 0.11, 0.085),
        0.0,
        3.5,
    );
    bar(
        builder,
        &key("chrome"),
        Vec3::new(front + 0.36, floor + 0.24 * k, 0.0),
        Vec3::new(front + 0.33, floor + 0.36 * k, 0.0),
        0.008,
        0.008,
        6,
    );
    soft_box(
        builder,
        &trim,
        Vec3::new(front + 0.33, floor + 0.38 * k, 0.0),
        Vec3::new(0.022, 0.028, 0.022),
        0.0,
        2.2,
    );
    if spec.boot {
        // The parcel shelf under the rear glass.
        soft_box(
            builder,
            &trim,
            Vec3::new(
                (rear_foot + rear_head) * 0.5,
                sample_line(&top, rear_foot) - 0.07,
                0.0,
            ),
            Vec3::new((rear_head - rear_foot) * 0.5, 0.02, half - 0.14),
            0.0,
            6.0,
        );
    }
}

/// Four wheels: a tyre with a rounded shoulder and grooved tread, a spoked rim
/// with its lip, and the brake disc behind the spokes.
fn build_wheels(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let half_base = spec.wheelbase * 0.5;
    let r = spec.wheel_r;
    let track = spec.width * 0.5 - 0.125;
    let rim = r * 0.66;
    for x in [-half_base, half_base] {
        for side in [-1.0_f32, 1.0] {
            let centre = Vec3::new(x, r, side * track);
            let axis = Vec3::new(0.0, 0.0, side);
            // From the inner bead out and round to the outer bead: sidewall,
            // shoulder, and a tread with three grooves cut round it.
            let groove = r - 0.007;
            let tyre = [
                (rim, -0.098),
                (r * 0.80, -0.108),
                (r * 0.95, -0.100),
                (r, -0.082),
                (r, -0.050),
                (groove, -0.046),
                (groove, -0.038),
                (r, -0.034),
                (r, -0.006),
                (groove, -0.004),
                (groove, 0.004),
                (r, 0.006),
                (r, 0.034),
                (groove, 0.038),
                (groove, 0.046),
                (r, 0.050),
                (r, 0.082),
                (r * 0.95, 0.100),
                (r * 0.80, 0.108),
                (rim, 0.098),
            ];
            for w in tyre.windows(2) {
                turn(builder, &key("wheel"), centre, axis, &[w[0], w[1]], 32);
            }
            // The rim: its lip outside, the barrel running in, a hub.
            let alloy = key("chrome");
            turn(
                builder,
                &alloy,
                centre,
                axis,
                &rounded(
                    &[
                        (rim, 0.098),
                        (rim * 0.97, 0.104),
                        (rim * 0.93, 0.094),
                        (rim * 0.90, 0.02),
                        (rim * 0.90, -0.09),
                    ],
                    3,
                ),
                40,
            );
            turn(
                builder,
                &alloy,
                centre,
                axis,
                &rounded(
                    &[
                        (0.0, 0.092),
                        (r * 0.10, 0.090),
                        (r * 0.17, 0.078),
                        (r * 0.19, 0.05),
                        (r * 0.19, 0.0),
                    ],
                    3,
                ),
                20,
            );
            for spoke in 0..spec.spokes {
                let angle = spoke as f32 / spec.spokes as f32 * std::f32::consts::TAU;
                let out = Vec3::new(angle.cos(), angle.sin(), 0.0);
                let wide = if spec.spokes > 7 { 0.011 } else { 0.019 };
                bar(
                    builder,
                    &alloy,
                    centre + out * (r * 0.15) + axis * 0.072,
                    centre + out * (rim * 0.93) + axis * 0.080,
                    wide * 1.25,
                    wide,
                    6,
                );
            }
            // The brake disc and its caliper, seen between the spokes.
            turn(
                builder,
                &key("trim"),
                centre,
                axis,
                &[
                    (0.0, 0.03),
                    (rim * 0.80, 0.03),
                    (rim * 0.80, 0.012),
                    (0.0, 0.012),
                ],
                28,
            );
            soft_box(
                builder,
                &key("lamp.rear"),
                centre + Vec3::new(rim * 0.62, rim * 0.30, side * 0.035),
                Vec3::new(0.035, 0.07, 0.03),
                -0.45,
                3.0,
            );
        }
    }
}

/// The far model's wheels: four drums.
fn build_far_wheels(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let half_base = spec.wheelbase * 0.5;
    let r = spec.wheel_r;
    for x in [-half_base, half_base] {
        for side in [-1.0_f32, 1.0] {
            turn(
                builder,
                &key("far.wheel"),
                Vec3::new(x, r, side * (spec.width * 0.5 - 0.125)),
                Vec3::new(0.0, 0.0, side),
                &[(0.0, -0.10), (r, -0.10), (r, 0.10), (0.0, 0.10)],
                10,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_design_is_a_car_sized_car_standing_on_its_wheels() {
        let mut builder = MeshBuilder::new();
        build_prototypes(&mut builder);
        let scene = builder.build();
        for variant in 0..CAR_VARIANTS {
            let extent = |part: &str| {
                let key = car_key(part, variant);
                let group = scene
                    .meshes
                    .iter()
                    .find(|g| g.material == key)
                    .unwrap_or_else(|| panic!("no {key}"));
                let mut lo = [f32::MAX; 3];
                let mut hi = [f32::MIN; 3];
                for v in group.positions.chunks(3) {
                    for k in 0..3 {
                        lo[k] = lo[k].min(v[k]);
                        hi[k] = hi[k].max(v[k]);
                    }
                }
                (lo, hi)
            };
            // Length along +X (placed with `yaw_along_x`), width over the mirrors.
            let (lo, hi) = extent("body");
            let (length, width, height) = (hi[0] - lo[0], hi[2] - lo[2], hi[1]);
            assert!(
                (3.9..5.1).contains(&length),
                "design {variant} is {length:.2} m long"
            );
            assert!(
                (1.6..2.15).contains(&width),
                "design {variant} is {width:.2} m wide"
            );
            assert!(
                (1.35..1.95).contains(&height),
                "design {variant} is {height:.2} m high"
            );
            let (wlo, whi) = extent("wheel");
            assert!(
                wlo[1].abs() < 0.01,
                "design {variant}'s wheels are {:.2} m off the road",
                wlo[1]
            );
            assert!(
                whi[0] - wlo[0] > 2.8 && whi[2] - wlo[2] < 2.0,
                "axles across the car"
            );
            // The seats are inside the roof, and the far model is the same size.
            let (_, ihi) = extent("interior");
            assert!(
                ihi[1] < height,
                "design {variant}'s interior comes through its roof"
            );
            let (flo, fhi) = extent("far.body");
            assert!((fhi[0] - flo[0] - length).abs() < 0.1, "far model length");
        }
    }

    #[test]
    fn a_placed_car_is_one_design_in_every_part() {
        let parts = instances_of(12.0, 0.0, -40.0, 0.3, [0.5, 0.1, 0.1]);
        assert_eq!(parts.len(), NEAR_PARTS.len() + FAR_PARTS.len());
        let variant = car_variant_at(12.0, -40.0);
        for (key, _) in &parts {
            assert_eq!(key.contains('~'), variant > 0, "{key}");
        }
    }
}
