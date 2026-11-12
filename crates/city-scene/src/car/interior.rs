//! What the windows show: floor, seats, dashboard, steering wheel, console.
use crate::math::Vec3;
use crate::mesh::MeshBuilder;
use crate::model::{bar, rounded, soft_box, torus_arc};

use super::form::{Spec, sample_line};

/// What the windows show: floor, seats, dashboard, steering wheel, console.
pub(super) fn build(builder: &mut MeshBuilder, spec: &Spec, key: &dyn Fn(&str) -> String) {
    let top: Vec<(f32, f32)> = rounded(&spec.top, 6);
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
