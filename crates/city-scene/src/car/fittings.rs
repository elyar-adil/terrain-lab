//! What is fitted to the shell as a part of its own: wheel wells, mirrors,
//! door handles, wipers, roof rails, the aerial, the exhaust.
use crate::math::Vec3;
use crate::mesh::MeshBuilder;
use crate::model::{bar, soft_block};

use super::form::{ARCH_GAP, Form};
use super::label;
use super::parts::{cap, grid, lathe, round_and, sd_box, slab, wall};
use super::surface::{Soup, Vert};

/// A door mirror, built in its own frame (x forward, y up, z out from the
/// car) and stood on the door: a housing lofted from the door outward, its
/// glass cut into the face that looks back and set in a little, a black foot,
/// a repeater let into the leading edge.
fn mirror(builder: &mut MeshBuilder, form: &Form, key: &dyn Fn(&str) -> String, side: f32) {
    let spec = form.spec;
    let m = spec.mirror;
    let [_, _, front_head, front_foot] = spec.cabin;
    let side_front = front_head + (front_foot - front_head) * 0.80;
    let x = if m.on_sail {
        side_front - 0.065
    } else {
        side_front + 0.03
    };
    let (foot, _) = form.glass(x);
    let wide = form.half_width(x);
    let root = if m.on_sail {
        Vec3::new(x, foot + 0.055, side * (wide - 0.085))
    } else {
        Vec3::new(x, form.belt(x) - 0.012, side * (wide - 0.045))
    };
    let deep = (m.wide * 0.42).clamp(0.065, 0.09);
    // The housing's frame: swept back a little at its outer end, and tipped up.
    let (sweep, tip) = (0.14_f32, 0.05_f32);
    let out = Vec3::new(-sweep.sin(), tip, side * sweep.cos()).normalized_or_up();
    let up = Vec3::new(0.0, 1.0, 0.0);
    let ahead = up.cross(out).normalized_or_up() * side;
    let up = out.cross(ahead).normalized_or_up() * side;
    let stand_off = if m.on_sail { 0.045 } else { 0.035 };
    let origin = root
        + Vec3::new(
            -0.01,
            m.high * 0.5 + if m.on_sail { -0.01 } else { 0.035 },
            side * stand_off,
        );
    let place = |l: Vec3| origin + ahead * l.x + up * l.y + out * l.z;
    let local = |p: Vec3| {
        let d = p - origin;
        Vec3::new(d.dot(ahead), d.dot(up), d.dot(out))
    };
    let (levels, around) = (16, 32);
    let mut rings: Vec<Vec<Vec3>> = Vec::with_capacity(levels + 1);
    for level in 0..=levels {
        let t = level as f32 / levels as f32;
        let ends = (1.0 - (2.0 * t - 1.0).abs().powf(4.0)).max(0.0).powf(0.25);
        let scale = if level == 0 || level == levels {
            0.0
        } else {
            ends
        };
        let depth = deep * (1.0 - 0.22 * t);
        rings.push(
            (0..around)
                .map(|k| {
                    let angle = (k as f32 + 0.5) / around as f32 * std::f32::consts::TAU;
                    let (c, s) = (angle.cos(), angle.sin());
                    // Round ahead, flat behind where the glass is.
                    let fore = if c > 0.0 {
                        c.powf(0.85) * 0.62
                    } else {
                        -c.abs().powf(0.22) * 0.38
                    };
                    let rise = s.signum() * s.abs().powf(0.55) * 0.5;
                    place(Vec3::new(
                        fore * depth * scale,
                        rise * m.high * scale,
                        t * m.wide,
                    ))
                })
                .collect(),
        );
    }
    let mut soup = Soup::from_rings(&rings, label::BODY);
    let shell = |l: u8| l == label::BODY;
    let glass = |p: Vec3| {
        let l = local(p);
        sd_box(
            (m.wide * 0.5, 0.0),
            (m.wide * 0.5 - 0.016, m.high * 0.5 - 0.013),
            (l.z, l.y),
            0.022,
        )
        .max(l.x + deep * 0.20)
    };
    soup.cut(&glass, &shell, label::GLASS);
    if m.painted {
        // The lower shell and the edge round the glass are left black.
        soup.cut(&|p| local(p).y + m.high * 0.24, &shell, label::BLACK);
        soup.cut(&|p| local(p).x + deep * 0.20, &shell, label::BLACK);
    }
    if m.repeater {
        let strip = |p: Vec3| {
            let l = local(p);
            round_and(
                &[
                    (l.y - m.high * 0.10).abs() - 0.0065,
                    m.wide * 0.34 - l.z,
                    -l.x,
                ],
                0.004,
            )
        };
        soup.cut(&strip, &|l| l != label::GLASS, label::AMBER);
    }
    let paint = if m.painted { key("body") } else { key("trim") };
    soup.emit(builder, &paint, 0.0, &|t| t.label == label::BODY);
    soup.emit(builder, &key("trim"), 0.0, &|t| t.label == label::BLACK);
    soup.emit(builder, &key("lamp.amber"), 0.0, &|t| {
        t.label == label::AMBER
    });
    // The glass, set in: a mirror is the brightest thing on the car.
    let pane = soup.take(&|t| t.label == label::GLASS);
    pane.sunk(0.005)
        .emit(builder, &key("chrome"), 0.0, &|_| true);
    for edge in pane.boundary(&|_| true) {
        wall(builder, &key("trim"), &edge, &|v| v.n * -0.005, true);
    }
    // The foot.
    let neck = place(Vec3::new(0.0, -m.high * 0.30, 0.012));
    soft_block(
        builder,
        &key("trim"),
        (root + neck) * 0.5,
        ahead * (deep * 0.36),
        (neck - root) * 0.62 + up * 0.012,
        out * 0.024,
        2.6,
    );
    soft_block(
        builder,
        &key("trim"),
        root,
        Vec3::new(0.062, 0.0, 0.0),
        Vec3::new(0.0, 0.020, 0.0),
        Vec3::new(0.0, 0.0, 0.016),
        3.0,
    );
}

pub(super) fn build(
    builder: &mut MeshBuilder,
    soup: &Soup,
    form: &Form,
    key: &dyn Fn(&str) -> String,
) {
    let spec = form.spec;
    let half_base = spec.wheelbase * 0.5;
    let arch = spec.wheel_r + ARCH_GAP;
    let [_, rear_head, front_head, front_foot] = spec.cabin;
    // The wells: a dark drum over each wheel, open to the road.
    for axle in [-half_base, half_base] {
        let wide = form.half_width(axle);
        for side in [-1.0_f32, 1.0] {
            let centre = Vec3::new(axle, spec.wheel_r, 0.0);
            let rings: Vec<Vec<Vec3>> = [wide - 0.44, wide + spec.style.hip - 0.006]
                .iter()
                .map(|z| {
                    (0..=32)
                        .map(|k| {
                            let angle = (-0.03 + 1.06 * k as f32 / 32.0) * std::f32::consts::PI;
                            // The well follows the arch, round or squared off.
                            let n = spec.style.arch_square;
                            let reach = (arch + 0.004)
                                / (angle.cos().abs().powf(n) + angle.sin().abs().powf(n))
                                    .powf(1.0 / n);
                            centre + Vec3::new(angle.cos() * reach, angle.sin() * reach, side * z)
                        })
                        .collect()
                })
                .collect();
            // Open below: an arc, not a ring, so nothing spans the wheel.
            let inward = |ring: &Vec<Vec3>| -> Vec<Vert> {
                ring.iter()
                    .map(|p| {
                        let d = *p - centre;
                        Vert::new(*p, Vec3::new(-d.x, -d.y, 0.0).normalized_or_up())
                    })
                    .collect()
            };
            grid(
                builder,
                &key("well"),
                &[inward(&rings[0]), inward(&rings[1])],
                false,
            );
            cap(builder, &key("well"), &rings[0], Vec3::new(0.0, 0.0, side));
        }
    }
    for side in [-1.0_f32, 1.0] {
        mirror(builder, form, key, side);
        // Door handles: the bar of a pull handle across its dish.
        if !spec.electric {
            for (x, _) in super::body::handle_places(form) {
                let y = form.belt(x) - 0.105;
                let here = soup.take(&|t| t.centre().z * side > 0.0 && t.label == label::BODY);
                // Its ends rest on the paint either side of the dish.
                let ends: Vec<Vec3> = [-0.108_f32, 0.108]
                    .iter()
                    .filter_map(|dx| here.locate(&|_| true, &|p| (p.x, p.y), (x + dx, y + 0.004)))
                    .map(|v| v.p)
                    .collect();
                if ends.len() == 2 {
                    let middle = (ends[0] + ends[1]) * 0.5 + Vec3::new(0.0, 0.0, side * 0.003);
                    soft_block(
                        builder,
                        &key(if spec.bright_trim { "chrome" } else { "body" }),
                        middle,
                        (ends[1] - ends[0]) * 0.5,
                        Vec3::new(0.0, 0.0125, 0.0),
                        Vec3::new(0.0, 0.0, 0.0085),
                        3.2,
                    );
                }
            }
        }
    }
    // Two wipers parked along the foot of the screen: an arm and its blade.
    let cowl = form.belt(front_foot) + 0.034;
    let (_, (screen_half, _)) = form.glass(front_foot - 0.05);
    for (z0, z1) in [(-screen_half + 0.12, -0.06_f32), (0.02, screen_half - 0.16)] {
        let pivot = Vec3::new(front_foot + 0.012, cowl - 0.004, z0);
        let tip = Vec3::new(front_foot - 0.045, cowl + 0.020, z1);
        bar(builder, &key("trim"), pivot, tip, 0.007, 0.004, 6);
        soft_block(
            builder,
            &key("trim"),
            (pivot + tip) * 0.5 + Vec3::new(-0.022, 0.002, (z1 - z0) * 0.08),
            Vec3::new(0.004, 0.0, 0.0),
            Vec3::new(0.0, 0.009, 0.0),
            Vec3::new(-0.03, 0.012, (z1 - z0) * 0.52),
            4.0,
        );
        lathe(
            builder,
            &key("trim"),
            pivot - Vec3::new(0.0, 0.012, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            &[(0.0, 0.020), (0.011, 0.018), (0.013, 0.0)],
            12,
            true,
        );
    }
    if spec.roof_rails {
        for side in [-1.0_f32, 1.0] {
            let z = side * (spec.width * 0.5 - 0.33);
            let top = |x: f32| form.top(x) + 0.004;
            let (from, to) = (rear_head + 0.08, front_head - 0.06);
            let mut previous = Vec3::new(from, top(from) + 0.01, z);
            for k in 1..=12 {
                let t = k as f32 / 12.0;
                let x = from + (to - from) * t;
                // The rail stands off the roof between its feet.
                let off = 0.012 + 0.040 * (1.0 - (2.0 * t - 1.0).powi(6));
                let next = Vec3::new(x, top(x) + off, z);
                bar(
                    builder,
                    &key(if spec.bright_trim { "chrome" } else { "trim" }),
                    previous,
                    next,
                    0.014,
                    0.014,
                    8,
                );
                previous = next;
            }
        }
    }
    // The aerial: a fin at the back of the roof.
    let fin = rear_head + 0.14;
    soft_block(
        builder,
        &key("body"),
        Vec3::new(fin, form.top(fin) + 0.040, 0.0),
        Vec3::new(0.078, 0.014, 0.0),
        Vec3::new(-0.018, 0.028, 0.0),
        Vec3::new(0.0, 0.0, 0.021),
        2.2,
    );
    // The third brake light, inside the head of the rear glass.
    let lamp = rear_head + 0.03;
    soft_block(
        builder,
        &key("lamp.rear"),
        Vec3::new(lamp, form.top(lamp) - 0.045, 0.0),
        Vec3::new(0.012, 0.0, 0.0),
        Vec3::new(0.0, 0.007, 0.0),
        Vec3::new(0.0, 0.0, 0.13),
        4.0,
    );
    // A tailgate has a wiper of its own, parked along the foot of its glass.
    if !spec.boot {
        let low = form.belt(spec.cabin[0]) + 0.058;
        let rear = soup.take(&|t| t.centre().x < 0.0 && t.centre().z >= 0.0);
        let on = |s: f32, y: f32| rear.locate(&|_| true, &|p| form.unwrap(false, p), (s, y));
        if let (Some(pivot), Some(tip)) = (on(0.004, low - 0.02), on(0.30, low + 0.035)) {
            let (a, b) = (pivot.p + pivot.n * 0.016, tip.p + tip.n * 0.012);
            let (a, b) = (Vec3::new(a.x, a.y, 0.0), b);
            bar(builder, &key("trim"), a, b, 0.008, 0.005, 6);
            lathe(
                builder,
                &key("trim"),
                a - pivot.n * 0.016,
                pivot.n,
                &[(0.0, 0.024), (0.014, 0.022), (0.017, 0.0)],
                12,
                true,
            );
        }
    }
    // Where the exhaust comes out: a bright-framed outlet let into the
    // valance each side, or on a van just the pipe.
    let places: &[f32] = match spec.exhausts {
        0 => &[],
        1 => &[1.0],
        _ => &[-1.0, 1.0],
    };
    let valance = soup.take(&|t| t.centre().x < 0.0);
    for side in places {
        let place = (form.face_half(false) * 0.56, form.floor(form.tail) + 0.105);
        let here = valance.take(&|t| t.centre().z * side >= 0.0);
        let found = here.locate(&|_| true, &|p| form.unwrap(false, p), place);
        if let (Some(at), true) = (found, spec.rows < 3) {
            let normal = Vec3::new(at.n.x, at.n.y * 0.3, 0.0).normalized_or_up();
            let up = Vec3::new(-normal.y, normal.x, 0.0) * if normal.x >= 0.0 { 1.0 } else { -1.0 };
            let across = Vec3::new(0.0, 0.0, 1.0);
            slab(
                builder,
                &key("chrome"),
                at.p + normal * 0.010,
                normal,
                up * 0.034,
                across * 0.085,
                0.03,
            );
            slab(
                builder,
                &key("well"),
                at.p + normal * 0.0105,
                normal,
                up * 0.024,
                across * 0.075,
                0.002,
            );
            continue;
        }
        let at = Vec3::new(
            form.tail + 0.045,
            form.floor(form.tail) + 0.045,
            side * form.face_half(false) * 0.62,
        );
        let back = Vec3::new(-1.0, 0.0, 0.0);
        lathe(
            builder,
            &key("chrome"),
            at,
            back,
            &[
                (0.030, -0.16),
                (0.030, 0.0),
                (0.034, 0.004),
                (0.037, 0.0),
                (0.037, -0.16),
            ],
            20,
            true,
        );
        lathe(
            builder,
            &key("well"),
            at,
            back,
            &[(0.030, -0.10), (0.0, -0.10)],
            12,
            false,
        );
    }
}
