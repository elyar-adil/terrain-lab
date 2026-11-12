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
use crate::model::{bar, rounded, skin, skin_cut, skin_lifted, soft_box, torus_arc, turn};

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
const NEAR_PARTS: [&str; 9] = [
    "body",
    "glass",
    "wheel",
    "trim",
    "chrome",
    "lamp.front",
    "lamp.rear",
    "interior",
    "plate",
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
        build_shell(builder, spec, &key);
        build_cabin(builder, spec, &key);
        build_interior(builder, spec, &key);
        build_wheels(builder, spec, &key);
        build_far(builder, spec, &key);
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

/// A closed section of a car at one station: a rounded box between two heights,
/// leaning in toward the top. Point `k` is at the angle `(k + 0.5) / points`
/// of a turn from `+z`, through the top.
fn section(
    x: f32,
    y0: f32,
    y1: f32,
    half: f32,
    lean: f32,
    squareness: f32,
    points: usize,
) -> Vec<Vec3> {
    let (mid, half_h) = ((y0 + y1) * 0.5, (y1 - y0) * 0.5);
    (0..points)
        .map(|k| {
            let angle = (k as f32 + 0.5) / points as f32 * std::f32::consts::TAU;
            let (c, s) = (angle.cos(), angle.sin());
            let zz = c.signum() * c.abs().powf(2.0 / squareness);
            let yy = s.signum() * s.abs().powf(2.0 / squareness);
            Vec3::new(
                x,
                mid + yy * half_h,
                zz * half * (1.0 - lean * (yy * 0.5 + 0.5)),
            )
        })
        .collect()
}

/// Where the quad starting at point `k` of a `points`-point section lies:
/// `(across, height)`, across from -1 (left) to 1 (right) and height from 0
/// (underside) to 1 (top), and the sine and cosine of its angle.
fn quad_at(k: usize, points: usize, squareness: f32) -> (f32, f32, f32, f32) {
    let angle = (k as f32 + 1.0) / points as f32 * std::f32::consts::TAU;
    let (c, s) = (angle.cos(), angle.sin());
    let across = c.signum() * c.abs().powf(2.0 / squareness);
    let height = s.signum() * s.abs().powf(2.0 / squareness) * 0.5 + 0.5;
    (across, height, c, s)
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

const SHELL_POINTS: usize = 60;
const SHELL_SQUARENESS: f32 = 5.0;
const END_ROUND_M: f32 = 0.16;

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
        front_door: front_foot - 0.04,
        b_pillar: (rear_head + front_head) * 0.5 + 0.12,
        rear_door: (rear_head + 0.16).max(-spec.wheelbase * 0.5 + spec.wheel_r + 0.02),
        bonnet: front_foot + 0.07,
        boot: rear_foot - 0.06,
    }
}

/// The painted shell below the waist and everything laid on it.
fn build_shell(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let top: Vec<(f32, f32)> = rounded(spec.top, 6);
    let (tail, nose) = (top[0].0, top[top.len() - 1].0);
    let half_base = spec.wheelbase * 0.5;
    let arch = spec.wheel_r + 0.055;
    let lines = lines_of(spec);
    let xs = stations(
        tail,
        nose,
        72,
        &[
            lines.front_door,
            lines.b_pillar,
            lines.rear_door,
            lines.bonnet,
            lines.boot,
        ],
        END_ROUND_M,
    );
    let shape = |x: f32| {
        let end = ((x - (tail + nose) * 0.5).abs() / ((nose - tail) * 0.5)).clamp(0.0, 1.0);
        let y1 = sample_line(&top, x);
        // The bumpers hang a little below the sill at both ends.
        let mut y0 = spec.sill - 0.06 * end.powf(5.0);
        for axle in [-half_base, half_base] {
            let d = (x - axle).abs();
            if d < arch {
                y0 = y0.max(spec.wheel_r + (arch * arch - d * d).sqrt() - 0.02);
            }
        }
        let y0 = y0.min(y1 - 0.12);
        let plan = 1.0 - 0.13 * end.powf(5.0) - 0.05 * end.powf(14.0);
        // Both ends close over the last hand's breadth, as a bumper rounds.
        let d = (x - tail).min(nose - x).max(0.0);
        let f = if d >= END_ROUND_M {
            1.0
        } else {
            (1.0 - (1.0 - d / END_ROUND_M).powf(2.4)).powf(1.0 / 2.4)
        };
        let (mid, half_h) = ((y0 + y1) * 0.5, (y1 - y0) * 0.5 * (0.40 + 0.60 * f));
        (
            mid - half_h,
            mid + half_h,
            spec.width * 0.5 * plan * (0.88 + 0.12 * f),
        )
    };
    let rings: Vec<Vec<Vec3>> = xs
        .iter()
        .map(|x| {
            let (y0, y1, half) = shape(*x);
            section(*x, y0, y1, half, 0.07, SHELL_SQUARENESS, SHELL_POINTS)
        })
        .collect();
    let mut closed = Vec::with_capacity(rings.len() + 2);
    closed.push(pole_of(&rings[0]));
    closed.extend(rings.iter().cloned());
    closed.push(pole_of(&rings[rings.len() - 1]));
    skin(builder, &key("body"), &closed);

    let mid_x = |i: usize| (xs[i] + xs[i + 1]) * 0.5;
    let near = |x: f32, line: f32| (x - line).abs() < 0.0075;
    let between_axles = |x: f32| x > -half_base + arch + 0.03 && x < half_base - arch - 0.03;
    let in_arch = |x: f32| (x + half_base).abs() < arch || (x - half_base).abs() < arch;
    let patch = |builder: &mut MeshBuilder,
                 part: &str,
                 lift: f32,
                 keep: &dyn Fn(f32, f32, f32, f32, f32) -> bool| {
        skin_lifted(builder, &key(part), &rings, lift, |i, k| {
            let (across, height, c, s) = quad_at(k, SHELL_POINTS, SHELL_SQUARENESS);
            keep(mid_x(i), across, height, c, s)
        });
    };
    // The two ends are faces, not sides: on the rings that round a bumper over,
    // how far a quad is from the tip (`d`) is how high up the face it is, above
    // the bumper bar for the upper half of a ring and below it for the lower.
    let front = |x: f32| nose - x;
    let back = |x: f32| x - tail;
    // Black plastic: the floor and the wheel wells, the sills between the
    // wheels, the grille above the front bumper and the intakes under both.
    patch(builder, "trim", 0.002, &|x, across, height, c, s| {
        let side = c.abs() > 0.72;
        let under = s < -0.80;
        (under && front(x) > 0.12 && back(x) > 0.12)
            || (side && in_arch(x) && height < 0.30)
            || (side && between_axles(x) && height < 0.11)
            || (s > 0.30 && across.abs() < 0.50 && (0.012..0.085).contains(&front(x)))
            || (s < -0.30 && across.abs() < 0.66 && (0.016..0.10).contains(&front(x)))
            || (s < -0.30 && across.abs() < 0.72 && (0.016..0.10).contains(&back(x)))
    });
    // The shut lines of the doors, the bonnet and the boot: gaps, drawn dark.
    patch(builder, "trim", 0.0012, &|x, _, height, c, s| {
        let side = c.abs() > 0.72;
        let over = s > 0.25;
        (side
            && height > 0.11
            && (near(x, lines.front_door) || near(x, lines.b_pillar) || near(x, lines.rear_door)))
            || ((over || (side && height > 0.55)) && near(x, lines.bonnet))
            || (spec.boot && (over || (side && height > 0.55)) && near(x, lines.boot))
    });
    // Bright work: the door handles and the bar across the grille.
    patch(builder, "chrome", 0.013, &|x, across, height, c, s| {
        let side = c.abs() > 0.72;
        let handle = |door_rear: f32| x > door_rear - 0.22 && x < door_rear - 0.08;
        (side
            && (0.765..0.825).contains(&height)
            && (handle(lines.b_pillar) || handle(lines.rear_door)))
            || (s > 0.30 && across.abs() < 0.46 && (0.030..0.050).contains(&front(x)))
    });
    // Lamps fill the corners of each end and wrap a hand's width down the side:
    // clear at the front, red at the back.
    patch(builder, "lamp.front", 0.004, &|x, across, height, c, s| {
        let side = c.abs() > 0.72;
        (s > 0.12 && across.abs() > 0.54 && (0.012..0.13).contains(&front(x)))
            || (side && (0.66..0.90).contains(&height) && (0.10..0.30).contains(&front(x)))
    });
    patch(builder, "lamp.rear", 0.004, &|x, across, height, c, s| {
        let side = c.abs() > 0.72;
        (s > 0.10 && across.abs() > 0.42 && (0.012..0.12).contains(&back(x)))
            || (side && (0.62..0.88).contains(&height) && (0.10..0.24).contains(&back(x)))
    });

    // A lip round each wheel arch, a mirror on each front door, and the plates.
    let body_half = spec.width * 0.5;
    for axle in [-half_base, half_base] {
        for side in [-1.0_f32, 1.0] {
            torus_arc(
                builder,
                &key("trim"),
                Vec3::new(axle, spec.wheel_r - 0.02, side * (body_half - 0.012)),
                Vec3::new(0.0, 0.0, side),
                Vec3::new(side, 0.0, 0.0),
                arch + 0.004,
                0.02,
                (0.03, 0.47),
            );
        }
    }
    let waist = sample_line(&top, spec.cabin[3] - 0.25);
    for side in [-1.0_f32, 1.0] {
        let at = Vec3::new(
            spec.cabin[3] - 0.22,
            waist + 0.07,
            side * (body_half + 0.01),
        );
        turn(
            builder,
            &key("body"),
            at,
            Vec3::new(0.25, 0.0, side),
            &rounded(
                &[
                    (0.0, 0.0),
                    (0.05, 0.02),
                    (0.07, 0.06),
                    (0.045, 0.10),
                    (0.0, 0.115),
                ],
                4,
            ),
            14,
        );
    }
    for (x, low) in [(nose + 0.004, 0.10_f32), (tail - 0.004, 0.16)] {
        let (y0, y1, _) = shape(x.clamp(tail, nose));
        soft_box(
            builder,
            &key("plate"),
            Vec3::new(x, y0 + (y1 - y0) * 0.5 - low * 0.2, 0.0),
            Vec3::new(0.008, 0.07, 0.22),
            0.0,
            9.0,
        );
    }
    // The exhaust's tail pipe.
    bar(
        builder,
        &key("chrome"),
        Vec3::new(tail + 0.16, spec.sill - 0.02, body_half - 0.42),
        Vec3::new(tail - 0.03, spec.sill - 0.02, body_half - 0.42),
        0.028,
        0.03,
        12,
    );
    if spec.roof_rails {
        let [_, rear_head, front_head, _] = spec.cabin;
        for side in [-1.0_f32, 1.0] {
            let z = side * (body_half - 0.34);
            bar(
                builder,
                &key("chrome"),
                Vec3::new(rear_head + 0.05, spec.roof + 0.05, z),
                Vec3::new(front_head - 0.05, spec.roof + 0.05, z),
                0.016,
                0.016,
                8,
            );
            for x in [rear_head + 0.12, front_head - 0.12] {
                bar(
                    builder,
                    &key("chrome"),
                    Vec3::new(x, spec.roof, z),
                    Vec3::new(x, spec.roof + 0.05, z),
                    0.014,
                    0.014,
                    6,
                );
            }
        }
    }
}

const CABIN_POINTS: usize = 56;

/// The cabin as closed sections from the rear glass's foot to the windscreen's.
fn cabin_rings(spec: &Spec, top: &[(f32, f32)], xs: &[f32], points: usize) -> Vec<Vec<Vec3>> {
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    xs.iter()
        .map(|x| {
            let x = *x;
            let waist = sample_line(top, x) - 0.04;
            let rise = if x < rear_head {
                (x - rear_foot) / (rear_head - rear_foot).max(0.05)
            } else if x > front_head {
                (front_foot - x) / (front_foot - front_head).max(0.05)
            } else {
                1.0
            };
            // Glass is flat between its corners: a raked plane, eased only where it meets the roof.
            let rise = rise.clamp(0.0, 1.0);
            let eased = 1.0 - (1.0 - rise).powf(1.35);
            let y1 = waist + 0.02 + (spec.roof - waist - 0.02) * eased;
            let half = spec.width * 0.5 - 0.06 - 0.02 * (1.0 - rise);
            let lean = 0.26 * (0.25 + 0.75 * eased);
            section(
                x,
                waist,
                y1.max(waist + 0.02),
                half,
                lean,
                SHELL_SQUARENESS,
                points,
            )
        })
        .collect()
}

/// Is the quad at `(x, k)` of the cabin glazing? Side windows between the
/// pillars, the windscreen and the rear glass between their frames.
fn glazed(spec: &Spec, x: f32, k: usize) -> bool {
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    let lines = lines_of(spec);
    let (across, height, c, s) = quad_at(k, CABIN_POINTS, SHELL_SQUARENESS);
    let side = c.abs() > 0.74;
    let pillar = |line: f32| (x - line).abs() < 0.045;
    let side_window = side
        && (0.16..0.86).contains(&height)
        && x > rear_head - 0.14
        && x < front_head + 0.32
        && !pillar(lines.b_pillar)
        && !(spec.rows > 2 && pillar((rear_head + lines.b_pillar) * 0.5));
    let over = across.abs() < 0.86 && s > 0.28;
    let screen = over && x > front_head + 0.05 && x < front_foot - 0.07;
    let rear = over && x < rear_head - 0.05 && x > rear_foot + 0.07;
    side_window || screen || rear
}

/// Roof and pillars in the body's paint, the glazing in its openings, the
/// headlining inside, and the dark cowl the wipers park on.
fn build_cabin(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let top: Vec<(f32, f32)> = rounded(spec.top, 6);
    let [rear_foot, rear_head, front_head, front_foot] = spec.cabin;
    let lines = lines_of(spec);
    let xs = stations(
        rear_foot,
        front_foot,
        44,
        &[
            lines.b_pillar - 0.038,
            lines.b_pillar + 0.038,
            rear_head - 0.05,
            front_head + 0.05,
        ],
        0.0,
    );
    let rings = cabin_rings(spec, &top, &xs, CABIN_POINTS);
    let mut closed = Vec::with_capacity(rings.len() + 2);
    closed.push(pole_of(&rings[0]));
    closed.extend(rings.iter().cloned());
    closed.push(pole_of(&rings[rings.len() - 1]));
    let mid_x = |i: usize| (xs[i] + xs[i + 1]) * 0.5;
    // `closed` has one more ring at each end than `xs` has stations.
    skin_cut(builder, &key("body"), &closed, |i, k| {
        i == 0 || i > xs.len() - 1 || !glazed(spec, mid_x(i - 1), k)
    });
    skin_lifted(builder, &key("glass"), &rings, -0.008, |i, k| {
        glazed(spec, mid_x(i), k)
    });
    skin_lifted(builder, &key("interior"), &rings, -0.022, |i, k| {
        !glazed(spec, mid_x(i), k)
    });
    skin_lifted(builder, &key("trim"), &rings, 0.004, |i, k| {
        let (across, _, _, s) = quad_at(k, CABIN_POINTS, SHELL_SQUARENESS);
        let x = mid_x(i);
        s > 0.28 && across.abs() < 0.88 && x > front_foot - 0.075 && x < front_foot - 0.02
    });
    // Two wiper arms, parked.
    let waist = sample_line(&top, front_foot - 0.06) - 0.02;
    for (z0, z1) in [(-0.62_f32, -0.12_f32), (-0.02, 0.50)] {
        bar(
            builder,
            &key("trim"),
            Vec3::new(front_foot - 0.045, waist + 0.035, z0),
            Vec3::new(front_foot - 0.075, waist + 0.050, z1),
            0.008,
            0.006,
            6,
        );
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

/// The far model: the same outline in a few hundred triangles, glass as a dark solid.
fn build_far(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let top: Vec<(f32, f32)> = rounded(spec.top, 2);
    let (tail, nose) = (top[0].0, top[top.len() - 1].0);
    let half_base = spec.wheelbase * 0.5;
    let arch = spec.wheel_r + 0.055;
    let xs = stations(tail, nose, 22, &[], 0.0);
    let mut rings: Vec<Vec<Vec3>> = xs
        .iter()
        .map(|x| {
            let end = ((x - (tail + nose) * 0.5).abs() / ((nose - tail) * 0.5)).clamp(0.0, 1.0);
            let y1 = sample_line(&top, *x);
            let mut y0 = spec.sill;
            for axle in [-half_base, half_base] {
                let d = (x - axle).abs();
                if d < arch {
                    y0 = y0.max(spec.wheel_r + (arch * arch - d * d).sqrt() - 0.02);
                }
            }
            section(
                *x,
                y0.min(y1 - 0.12),
                y1,
                spec.width * 0.5 * (1.0 - 0.14 * end.powf(5.0)),
                0.07,
                SHELL_SQUARENESS,
                14,
            )
        })
        .collect();
    rings.insert(0, pole_of(&rings[0]));
    let closing = pole_of(&rings[rings.len() - 1]);
    rings.push(closing);
    skin(builder, &key("far.body"), &rings);
    let [rear_foot, _, _, front_foot] = spec.cabin;
    let cabin_xs = stations(rear_foot, front_foot, 10, &[], 0.0);
    let mut cabin = cabin_rings(spec, &top, &cabin_xs, 12);
    cabin.insert(0, pole_of(&cabin[0]));
    let closing = pole_of(&cabin[cabin.len() - 1]);
    cabin.push(closing);
    skin(builder, &key("far.glass"), &cabin);
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
                (3.9..5.0).contains(&length),
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
