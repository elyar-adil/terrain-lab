//! Wheels: a tyre with its tread, a rim with its spokes, and the brake behind.
//!
//! A tyre is a carcass turned from its section (bead, sidewall with its rim
//! protector, shoulder, the floor of the grooves) and, standing on that, the
//! tread: every block its own solid with a top and four walls, laid out in
//! ribs the way the pattern is. A rim's face is a turned disc with the
//! windows between its spokes cut out of it, so any design is one field in
//! polar coordinates: plain spokes, split spokes, a mesh, a turbine, the slots
//! of an aero cover, the holes of a steel wheel. The edges of the windows are
//! walled to give the spokes their depth. Behind them the barrel, the brake
//! disc and its caliper.
use crate::math::Vec3;
use crate::mesh::MeshBuilder;
use crate::model::rounded;

use super::form::Spec;
use super::parts::{cap, grid, lathe, lathe_arc, round_and, wall};
use super::surface::{Soup, Vert};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Tread {
    /// Straight ribs: a van's, a cheap car's.
    Ribbed,
    /// Ribs cut by angled sipes, slotted shoulders.
    Touring,
    /// Chevrons that point the way it rolls.
    Directional,
    /// Staggered blocks with wide grooves.
    Block,
}

#[derive(Clone, Copy)]
pub(super) struct Tyre {
    /// Section width.
    pub width: f32,
    /// The radius of the rim it is on.
    pub rim_r: f32,
    pub tread: Tread,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum RimStyle {
    Spoke,
    /// Each spoke split in two toward the rim.
    Split,
    /// Two sets of thin spokes crossing.
    Mesh,
    /// Spokes swept round like blades.
    Turbine,
    /// A nearly closed cover with slots.
    Aero,
    /// Pressed steel with round holes.
    Steel,
}

#[derive(Clone, Copy)]
pub(super) struct Rim {
    pub style: RimStyle,
    pub spokes: usize,
    /// How far the spokes sweep round from hub to rim, in radians.
    pub twist: f32,
    /// A machined bright face on painted spokes.
    pub two_tone: bool,
    /// The paint's tint.
    pub tint: [f32; 3],
    /// Big discs and painted calipers.
    pub sport_brakes: bool,
}

/// A wheel's place: its middle, and its axle pointing out of the car.
struct Hub {
    centre: Vec3,
    axis: Vec3,
    /// 1 on the right of the car, -1 on the left: patterns are mirrored.
    side: f32,
}

impl Hub {
    /// A point by its angle round the axle, its radius and its distance out
    /// along the axle; and the directions out from the axle and round it there.
    fn at(&self, angle: f32, r: f32, out: f32) -> Vec3 {
        self.centre + self.radial(angle) * r + self.axis * out
    }
    fn radial(&self, angle: f32) -> Vec3 {
        Vec3::new(angle.cos() * self.side, angle.sin(), 0.0)
    }
    fn around(&self, angle: f32) -> Vec3 {
        Vec3::new(-angle.sin() * self.side, angle.cos(), 0.0)
    }
}

/// The outside of the tread across the tyre, and the floor of its grooves:
/// `(top, floor)` radii at a distance `w` from the tyre's middle.
fn crown(tyre: &Tyre, r: f32, depth: f32, w: f32) -> (f32, f32) {
    let half = tyre.width * 0.5;
    let t = (w.abs() / half).min(1.0);
    // Nearly flat across the tread, rolling over into the shoulder.
    let top = r - 0.004 * t * t - 0.030 * ((t - 0.70) / 0.30).max(0.0).powf(2.2);
    let fade = 1.0 - ((t - 0.78) / 0.20).clamp(0.0, 1.0);
    (top, top - depth * fade)
}

fn tyre(builder: &mut MeshBuilder, key: &str, hub: &Hub, tyre: &Tyre, r: f32) {
    let half = tyre.width * 0.5;
    let depth = match tyre.tread {
        Tread::Block => 0.010,
        _ => 0.0075,
    };
    let height = r - tyre.rim_r;
    // The carcass, from the inner bead round to the outer.
    let mut section: Vec<(f32, f32)> = Vec::new();
    let wall = |side: f32| -> Vec<(f32, f32)> {
        vec![
            (tyre.rim_r - 0.006, side * half * 0.80),
            (tyre.rim_r + 0.004, side * half * 0.90),
            (tyre.rim_r + 0.016, side * (half * 0.90 + 0.007)),
            (tyre.rim_r + 0.030, side * half * 0.955),
            (tyre.rim_r + height * 0.50, side * half * 1.0),
            (tyre.rim_r + height * 0.78, side * half * 0.985),
        ]
    };
    section.extend(wall(-1.0));
    let across = 8;
    for k in 0..=across {
        let w = half * 0.97 * (k as f32 / across as f32 * 2.0 - 1.0);
        section.push((crown(tyre, r, depth, w).1, w));
    }
    section.extend(wall(1.0).into_iter().rev());
    lathe(
        builder,
        key,
        hub.centre,
        hub.axis,
        &rounded(&section, 2),
        48,
        false,
    );

    // The tread: ribs of blocks. Each rib is `(from, to)` across the tyre as
    // shares of the half-width, how many blocks go round, the share of each
    // pitch that is groove, how far the block leans round across its width
    // (radians), and its phase.
    struct Rib {
        w: (f32, f32),
        count: usize,
        gap: f32,
        lean: f32,
        phase: f32,
        steps: usize,
    }
    let rib = |w: (f32, f32), count: usize, gap: f32, lean: f32, phase: f32| Rib {
        w,
        count,
        gap,
        lean,
        phase,
        steps: if w.0.abs().max(w.1.abs()) > 0.75 {
            2
        } else {
            1
        },
    };
    let ribs: Vec<Rib> = match tyre.tread {
        Tread::Ribbed => vec![
            rib((-0.96, -0.62), 64, 0.22, 0.0, 0.0),
            rib((-0.54, -0.22), 1, 0.0, 0.0, 0.0),
            rib((-0.15, 0.15), 1, 0.0, 0.0, 0.0),
            rib((0.22, 0.54), 1, 0.0, 0.0, 0.0),
            rib((0.62, 0.96), 64, 0.22, 0.0, 0.5),
        ],
        Tread::Touring => vec![
            rib((-0.97, -0.60), 72, 0.20, 0.010, 0.0),
            rib((-0.52, -0.19), 60, 0.10, 0.040, 0.3),
            rib((-0.12, 0.12), 1, 0.0, 0.0, 0.0),
            rib((0.19, 0.52), 60, 0.10, 0.040, 0.8),
            rib((0.60, 0.97), 72, 0.20, 0.010, 0.5),
        ],
        Tread::Directional => vec![
            rib((-0.97, -0.56), 56, 0.24, -0.050, 0.0),
            rib((-0.50, -0.10), 56, 0.20, -0.085, 0.45),
            rib((-0.055, 0.055), 1, 0.0, 0.0, 0.0),
            rib((0.10, 0.50), 56, 0.20, 0.085, 0.45),
            rib((0.56, 0.97), 56, 0.24, 0.050, 0.0),
        ],
        Tread::Block => vec![
            rib((-0.97, -0.58), 40, 0.30, 0.012, 0.0),
            rib((-0.48, -0.06), 44, 0.26, -0.030, 0.5),
            rib((0.06, 0.48), 44, 0.26, 0.030, 0.0),
            rib((0.58, 0.97), 40, 0.30, -0.012, 0.5),
        ],
    };
    let tau = std::f32::consts::TAU;
    for rib in &ribs {
        let (w0, w1) = (rib.w.0 * half, rib.w.1 * half);
        if rib.count == 1 {
            // A rib that runs unbroken round the tyre: turned.
            let (top0, floor0) = crown(tyre, r, depth, w0);
            let (top1, floor1) = crown(tyre, r, depth, w1);
            lathe(
                builder,
                key,
                hub.centre,
                hub.axis,
                &[
                    (floor0 - 0.001, w0),
                    (top0, w0),
                    (top0, w0),
                    (top1, w1),
                    (top1, w1),
                    (floor1 - 0.001, w1),
                ],
                56,
                false,
            );
            continue;
        }
        let pitch = tau / rib.count as f32;
        let wide = pitch * (1.0 - rib.gap);
        for block in 0..rib.count {
            let middle = (block as f32 + rib.phase) * pitch;
            // The block's four corners' columns across the tyre.
            let column = |edge: f32, top: bool| -> Vec<Vert> {
                (0..=rib.steps)
                    .map(|k| {
                        let t = k as f32 / rib.steps as f32;
                        let w = w0 + (w1 - w0) * t;
                        let angle = middle + edge * wide * 0.5 + rib.lean * (t - 0.5) * 2.0;
                        let (up, floor) = crown(tyre, r, depth, w);
                        let radius = if top { up } else { floor - 0.001 };
                        Vert::new(hub.at(angle, radius, w), hub.radial(angle))
                    })
                    .collect()
            };
            let with = |column: Vec<Vert>, normal: &dyn Fn(usize) -> Vec3| -> Vec<Vert> {
                column
                    .into_iter()
                    .enumerate()
                    .map(|(k, v)| Vert::new(v.p, normal(k)))
                    .collect()
            };
            let (lo_top, hi_top) = (column(-1.0, true), column(1.0, true));
            let (lo_foot, hi_foot) = (column(-1.0, false), column(1.0, false));
            // Top: lit as the tyre's own round surface.
            grid(builder, key, &[lo_top.clone(), hi_top.clone()], false);
            // The walls across the tyre, leading and trailing.
            let back = -hub.around(middle);
            let ahead = hub.around(middle);
            grid(
                builder,
                key,
                &[
                    with(lo_top.clone(), &|_| back),
                    with(lo_foot.clone(), &|_| back),
                ],
                false,
            );
            grid(
                builder,
                key,
                &[
                    with(hi_top.clone(), &|_| ahead),
                    with(hi_foot.clone(), &|_| ahead),
                ],
                false,
            );
            // The walls along the grooves.
            for (k, out) in [(0, -1.0_f32), (rib.steps, 1.0)] {
                if (w0 + (w1 - w0) * k as f32 / rib.steps as f32).abs() > half * 0.9 {
                    continue;
                }
                let normal = hub.axis * out;
                grid(
                    builder,
                    key,
                    &[
                        vec![
                            Vert::new(lo_top[k].p, normal),
                            Vert::new(hi_top[k].p, normal),
                        ],
                        vec![
                            Vert::new(lo_foot[k].p, normal),
                            Vert::new(hi_foot[k].p, normal),
                        ],
                    ],
                    false,
                );
            }
        }
    }
}

/// The windows of a rim's face: negative where there is no metal. `rho` and
/// `phi` are a point's radius and angle; `inner..outer` the band the windows
/// may occupy.
fn windows(rim: &Rim, rho: f32, phi: f32, inner: f32, outer: f32) -> f32 {
    let sector = std::f32::consts::TAU / rim.spokes as f32;
    let fold = |angle: f32| -> f32 { (angle.rem_euclid(sector)) - sector * 0.5 };
    let span = (rho - inner) / (outer - inner);
    // The distance round to the nearest spoke's middle line, in metres.
    let to_spoke = |angle: f32| (sector * 0.5 - fold(angle).abs()) * rho;
    let band = [inner - rho, rho - outer];
    let between = |half_in: f32, half_out: f32, angle: f32, corner: f32| -> f32 {
        let half = half_in + (half_out - half_in) * span.clamp(0.0, 1.0);
        round_and(&[half - to_spoke(angle), band[0], band[1]], corner)
    };
    match rim.style {
        RimStyle::Spoke => {
            let thick = (0.26 / rim.spokes as f32).clamp(0.011, 0.034);
            between(thick * 1.15, thick * 0.80, phi, 0.014)
        }
        RimStyle::Split => {
            let thick = (0.34 / rim.spokes as f32).clamp(0.030, 0.050);
            let main = between(thick, thick * 1.25, phi, 0.014);
            // The slot down the middle of each spoke.
            let slot = round_and(
                &[
                    to_spoke(phi) - thick * (0.18 + 0.50 * span),
                    inner + (outer - inner) * 0.30 - rho,
                    rho - outer,
                ],
                0.006,
            );
            main.min(slot)
        }
        RimStyle::Mesh => {
            let lean = 0.55 * span;
            between(0.0065, 0.0055, phi + lean, 0.006).max(between(
                0.0065,
                0.0055,
                phi - lean,
                0.006,
            ))
        }
        RimStyle::Turbine => between(0.014, 0.011, phi - rim.twist * span, 0.010),
        RimStyle::Aero => {
            let angle = fold(phi - rim.twist * span + sector * 0.5);
            round_and(
                &[
                    angle.abs() * rho - (0.010 + 0.022 * span),
                    inner + (outer - inner) * 0.34 - rho,
                    rho - (outer - 0.008),
                ],
                0.008,
            )
        }
        RimStyle::Steel => {
            let at = inner + (outer - inner) * 0.70;
            let (dx, dy) = (fold(phi + sector * 0.5) * rho * 0.72, rho - at);
            (dx * dx + dy * dy).sqrt() - (outer - inner) * 0.15
        }
    }
}

const FACE: u8 = 0;
const WINDOW: u8 = 1;

fn rim(builder: &mut MeshBuilder, key: &dyn Fn(&str) -> String, hub: &Hub, spec: &Spec, r: f32) {
    let (tyre, rim) = (&spec.tyre, &spec.rim);
    let rr = tyre.rim_r;
    let half = tyre.width * 0.5;
    let paint = key("rim");
    let steel = rim.style == RimStyle::Steel;
    // The barrel and its two flanges: seen from outside through the windows.
    lathe(
        builder,
        &paint,
        hub.centre,
        hub.axis,
        &rounded(
            &[
                (rr - 0.004, half * 0.80),
                (rr + 0.006, half * 0.86),
                (rr + 0.002, half * 0.90),
                (rr - 0.012, half * 0.86),
                (rr - 0.020, half * 0.70),
                (rr - 0.030, half * 0.30),
                (rr - 0.030, -half * 0.60),
                (rr - 0.014, -half * 0.84),
                (rr + 0.004, -half * 0.88),
            ],
            2,
        ),
        44,
        true,
    );
    // The face: a dished disc from the hub out to the barrel.
    let outer = rr - 0.014;
    let face_out = half * if steel { 0.52 } else { 0.72 };
    let dish = if steel { -0.018 } else { r * 0.085 };
    let hub_r = r * 0.185;
    let profile = |rho: f32| -> f32 {
        let t = ((rho - hub_r) / (outer - hub_r)).clamp(0.0, 1.0);
        // Out from the hub the spokes fall back, then rise to the rim.
        face_out - dish * (1.0 - t).powf(1.6)
            + if steel {
                0.018 * (t * std::f32::consts::PI).sin()
            } else {
                0.0
            }
    };
    let (rings, around) = (14, 144);
    let rows: Vec<Vec<Vert>> = (0..=rings)
        .map(|j| {
            let rho = outer * (j as f32 / rings as f32).powf(0.9).max(1.0e-4);
            let slope = (profile(rho + 0.002) - profile(rho - 0.002)) / 0.004;
            (0..around)
                .map(|k| {
                    let angle = k as f32 / around as f32 * std::f32::consts::TAU;
                    Vert::new(
                        hub.at(angle, rho, profile(rho)),
                        (hub.axis - hub.radial(angle) * slope).normalized_or_up(),
                    )
                })
                .collect()
        })
        .collect();
    let mut face = Soup::from_grid(&rows, true, FACE);
    let inner = hub_r + 0.012;
    let field = |p: Vec3| {
        let d = p - hub.centre;
        let rho = (d.x * d.x + d.y * d.y).sqrt();
        let phi = d.y.atan2(d.x * hub.side);
        windows(rim, rho, phi, inner, outer - 0.004)
    };
    face.cut(&field, &|l| l == FACE, WINDOW);
    let bright = if rim.two_tone {
        key("chrome")
    } else {
        paint.clone()
    };
    face.emit(builder, &bright, 0.0, &|t| t.label == FACE);
    // The depth of the spokes: the windows' edges, walled back.
    let thick = if steel { 0.004 } else { 0.030 };
    for edge in face.boundary(&|l| l == WINDOW) {
        wall(builder, &paint, &edge, &|_| hub.axis * -thick, true);
    }
    // Centre cap and wheel nuts.
    let seat = profile(0.0);
    lathe(
        builder,
        &key("chrome"),
        hub.centre + hub.axis * seat,
        hub.axis,
        &[
            (0.0, 0.010),
            (r * 0.060, 0.009),
            (r * 0.082, 0.003),
            (r * 0.088, -0.004),
        ],
        24,
        true,
    );
    let nuts = if steel { 4 } else { 5 };
    for k in 0..nuts {
        let angle = (k as f32 + 0.5) / nuts as f32 * std::f32::consts::TAU;
        let at = hub.at(angle, r * 0.135, profile(r * 0.135) - 0.004);
        lathe(
            builder,
            &key("chrome"),
            at,
            hub.axis,
            &[
                (0.0, 0.011),
                (0.0062, 0.010),
                (0.0085, 0.006),
                (0.0085, 0.006),
                (0.0085, 0.0),
            ],
            6,
            true,
        );
    }
    // The valve.
    let valve = hub.at(0.3, rr - 0.030, half * 0.42);
    lathe(
        builder,
        &key("trim"),
        valve,
        (hub.axis * 0.9 - hub.radial(0.3) * 0.4).normalized_or_up(),
        &[
            (0.0, 0.034),
            (0.0035, 0.033),
            (0.0035, 0.033),
            (0.0045, 0.0),
        ],
        8,
        true,
    );

    // The brake: a disc on its bell, and the caliper over it.
    let disc_r = rr * if rim.sport_brakes { 0.90 } else { 0.76 };
    let plane = face_out - dish.max(0.0) - 0.045;
    lathe(
        builder,
        &key("chrome"),
        hub.centre,
        hub.axis,
        &[
            (0.0, plane + 0.030),
            (disc_r * 0.50, plane + 0.030),
            (disc_r * 0.50, plane + 0.030),
            (disc_r * 0.54, plane + 0.010),
            (disc_r * 0.54, plane + 0.010),
            (disc_r, plane + 0.010),
            (disc_r, plane + 0.010),
            (disc_r, plane - 0.012),
        ],
        48,
        true,
    );
    let grip = if rim.sport_brakes {
        key("lamp.rear")
    } else {
        key("trim")
    };
    let (low, high) = (disc_r * 0.62, (disc_r + 0.014).min(rr - 0.034));
    let (a, b) = (plane - 0.02, plane + 0.034);
    let span = if rim.sport_brakes {
        (0.02, 0.21)
    } else {
        (0.05, 0.17)
    };
    let section = rounded(
        &[
            (low, a),
            (low, b),
            (low + (high - low) * 0.5, b + 0.004),
            (high, b),
            (high, a),
        ],
        3,
    );
    lathe_arc(
        builder, &grip, hub.centre, hub.axis, &section, 14, true, span, 0.0,
    );
    for (turn, toward) in [(span.0, -1.0_f32), (span.1, 1.0)] {
        // The caliper's two ends. `lathe_arc` measures its turn in its own
        // frame; `frame` is the same one.
        let (side_a, side_b) = super::parts::frame(hub.axis);
        let angle = turn * std::f32::consts::TAU;
        let out = side_a * angle.cos() + side_b * angle.sin();
        let round = (side_b * angle.cos() - side_a * angle.sin()) * toward;
        let edge: Vec<Vec3> = section
            .iter()
            .map(|(rho, along)| hub.centre + out * *rho + hub.axis * *along)
            .collect();
        cap(builder, &grip, &edge, round);
    }
}

/// Four wheels.
pub(super) fn build(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let r = spec.wheel_r;
    let track = spec.width * 0.5 - spec.tyre.width * 0.5 - 0.016;
    for x in [-spec.wheelbase * 0.5, spec.wheelbase * 0.5] {
        for side in [-1.0_f32, 1.0] {
            let hub = Hub {
                centre: Vec3::new(x, r, side * track),
                axis: Vec3::new(0.0, 0.0, side),
                side,
            };
            tyre(builder, &key("wheel"), &hub, &spec.tyre, r);
            rim(builder, key, &hub, spec, r);
        }
    }
}

/// The far model's wheels: four drums.
pub(super) fn build_far(builder: &mut MeshBuilder, spec: &Spec, key: &str) {
    let r = spec.wheel_r;
    let half = spec.tyre.width * 0.5;
    let track = spec.width * 0.5 - half - 0.016;
    for x in [-spec.wheelbase * 0.5, spec.wheelbase * 0.5] {
        for side in [-1.0_f32, 1.0] {
            lathe(
                builder,
                key,
                Vec3::new(x, r, side * track),
                Vec3::new(0.0, 0.0, side),
                &[
                    (0.0, -half),
                    (r, -half),
                    (r, -half),
                    (r, half),
                    (r, half),
                    (0.0, half),
                ],
                12,
                false,
            );
        }
    }
}
